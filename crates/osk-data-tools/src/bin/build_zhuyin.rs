//! `build-zhuyin` — ingest CC-CEDICT (CC BY-SA 4.0) into a runtime
//! Zhuyin (Bopomofo) → **Traditional** Hanzi dictionary bundle.
//!
//! The Traditional-Chinese twin of `build-pinyin`. Two differences:
//!   1. Candidates come from CC-CEDICT's **Traditional** column (first
//!      token), not the Simplified one — this is the Taiwan/HK market.
//!   2. The lookup key is the toneless **Zhuyin** spelling, produced by
//!      converting each toneless Pinyin syllable to Bopomofo (see
//!      [`syllable_to_zhuyin`]) and concatenating — so `ni3 hao3` →
//!      `ㄋㄧㄏㄠ`, matching what the Zhuyin keyboard's keys emit.
//!
//! Output: `<out>/cedict.bin`, an `osk_data::codec` `BTreeMap<String, Vec<String>>`
//! keyed by toneless Zhuyin, same format the runtime `CedictPinyinDict`
//! decodes (the map is script-agnostic).
//!
//! Usage:
//!   build-zhuyin --seed --out zhuyin/
//!   build-zhuyin --cedict cc-cedict.u8 --out zhuyin/

use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::Context as _;
use clap::Parser;

#[derive(Parser)]
#[command(name = "build-zhuyin")]
struct Args {
    /// CC-CEDICT source file. Ignored when `--seed` is set.
    #[arg(long)]
    cedict: Option<PathBuf>,
    /// Use the bundled curated seed list instead of an external file.
    #[arg(long, default_value_t = false)]
    seed: bool,
    /// Output directory. File written as `<out>/cedict.bin`.
    #[arg(long)]
    out: PathBuf,
    /// Word-frequency list (`<word> <count>` per line) used to rank
    /// candidates. Optional; without it the order is length-then-input,
    /// which puts archaic variants first — see `osk_data_tools::order_candidates`.
    #[arg(long)]
    freq: Option<PathBuf>,
    /// Max candidates kept per Zhuyin key.
    #[arg(long, default_value_t = 9)]
    cap: usize,
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    let lines: Vec<String> = if args.seed {
        log::info!("build-zhuyin: using the bundled curated seed");
        SEED_CEDICT.lines().map(str::to_string).collect()
    } else {
        let Some(path) = &args.cedict else {
            anyhow::bail!("need --cedict <path> or --seed");
        };
        let f = fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
        BufReader::new(f).lines().collect::<Result<_, _>>()?
    };

    let freq = match &args.freq {
        Some(p) => osk_data_tools::load_freq_list(p)?,
        None => std::collections::HashMap::new(),
    };

    // (zhuyin_key, hanzi_char_count, insertion_order) → traditional hanzi
    let mut accum: BTreeMap<String, Vec<(usize, usize, String)>> = BTreeMap::new();
    let mut order = 0usize;
    let mut skipped = 0usize;
    for line in &lines {
        let Some((trad, pinyin)) = parse_cedict_line(line) else {
            continue;
        };
        let Some(key) = pinyin_to_zhuyin_key(pinyin) else {
            skipped += 1;
            continue;
        };
        let char_count = trad.chars().count();
        accum
            .entry(key)
            .or_default()
            .push((char_count, order, trad.to_string()));
        order += 1;
    }

    let mut table: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (key, entries) in accum {
        // Shorter (single-char) candidates first; stable within length.
        let cands = osk_data_tools::order_candidates(entries, &freq, args.cap);
        table.insert(key, cands);
    }

    log::info!(
        "build-zhuyin: zhuyin keys = {} ({skipped} entries skipped — unconvertible pinyin)",
        table.len()
    );

    fs::create_dir_all(&args.out).with_context(|| format!("mkdir -p {}", args.out.display()))?;
    let out_path = args.out.join("cedict.bin");
    let bytes = osk_data::codec::encode(&table).context("encode cedict")?;
    fs::write(&out_path, &bytes).with_context(|| format!("writing {}", out_path.display()))?;
    log::info!("wrote {} ({} bytes)", out_path.display(), bytes.len());
    Ok(())
}

/// Parse one CC-CEDICT line → `(traditional_hanzi, pinyin_raw)`. Returns
/// `None` for comments / blank / malformed. Format:
/// `Trad Simp [pin1 yin1] /def/def/`.
fn parse_cedict_line(line: &str) -> Option<(&str, &str)> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    let (trad, rest) = trimmed.split_once(char::is_whitespace)?;
    // Skip Simplified (second token).
    let rest = rest
        .trim_start()
        .split_once(char::is_whitespace)?
        .1
        .trim_start();
    let after_open = rest.strip_prefix('[')?;
    let close = after_open.find(']')?;
    Some((trad, &after_open[..close]))
}

/// Convert a whole CC-CEDICT pinyin field (`"ni3 hao3"`) to the toneless
/// Zhuyin lookup key (`"ㄋㄧㄏㄠ"`). Returns `None` if any syllable can't
/// be converted (the entry is then skipped).
fn pinyin_to_zhuyin_key(raw: &str) -> Option<String> {
    let mut key = String::new();
    for syl in raw.split_whitespace() {
        // CC-CEDICT writes ü as `u:`; drop tone digits and apostrophes.
        let s = syl.to_lowercase().replace("u:", "ü");
        let toneless: String = s.chars().filter(|c| c.is_alphabetic()).collect();
        if toneless.is_empty() {
            continue;
        }
        key.push_str(&syllable_to_zhuyin(&toneless)?);
    }
    (!key.is_empty()).then_some(key)
}

/// Convert one toneless, lowercase Pinyin syllable to Bopomofo. `None`
/// for an unrecognised syllable.
fn syllable_to_zhuyin(py: &str) -> Option<String> {
    // Whole-syllable table: the empty-rime sibilants/retroflexes (`shi`
    // is ㄕ, not ㄕㄧ), the y-/w-glide syllables, the initial-less rimes,
    // and erhua `r`.
    let whole = match py {
        "zhi" => "ㄓ",
        "chi" => "ㄔ",
        "shi" => "ㄕ",
        "ri" => "ㄖ",
        "zi" => "ㄗ",
        "ci" => "ㄘ",
        "si" => "ㄙ",
        "yi" => "ㄧ",
        "ya" => "ㄧㄚ",
        "yao" => "ㄧㄠ",
        "ye" => "ㄧㄝ",
        "you" => "ㄧㄡ",
        "yan" => "ㄧㄢ",
        "yang" => "ㄧㄤ",
        "yin" => "ㄧㄣ",
        "ying" => "ㄧㄥ",
        "yong" => "ㄩㄥ",
        "yu" => "ㄩ",
        "yue" => "ㄩㄝ",
        "yuan" => "ㄩㄢ",
        "yun" => "ㄩㄣ",
        "wu" => "ㄨ",
        "wa" => "ㄨㄚ",
        "wai" => "ㄨㄞ",
        "wan" => "ㄨㄢ",
        "wang" => "ㄨㄤ",
        "wei" => "ㄨㄟ",
        "wen" => "ㄨㄣ",
        "weng" => "ㄨㄥ",
        "wo" => "ㄨㄛ",
        "a" => "ㄚ",
        "o" => "ㄛ",
        "e" => "ㄜ",
        "ê" => "ㄝ",
        "ai" => "ㄞ",
        "ei" => "ㄟ",
        "ao" => "ㄠ",
        "ou" => "ㄡ",
        "an" => "ㄢ",
        "en" => "ㄣ",
        "ang" => "ㄤ",
        "eng" => "ㄥ",
        "er" => "ㄦ",
        "r" => "ㄦ",
        _ => "",
    };
    if !whole.is_empty() {
        return Some(whole.to_string());
    }

    // initial + rime decomposition.
    const INITIALS: &[(&str, &str)] = &[
        ("zh", "ㄓ"),
        ("ch", "ㄔ"),
        ("sh", "ㄕ"),
        ("b", "ㄅ"),
        ("p", "ㄆ"),
        ("m", "ㄇ"),
        ("f", "ㄈ"),
        ("d", "ㄉ"),
        ("t", "ㄊ"),
        ("n", "ㄋ"),
        ("l", "ㄌ"),
        ("g", "ㄍ"),
        ("k", "ㄎ"),
        ("h", "ㄏ"),
        ("j", "ㄐ"),
        ("q", "ㄑ"),
        ("x", "ㄒ"),
        ("r", "ㄖ"),
        ("z", "ㄗ"),
        ("c", "ㄘ"),
        ("s", "ㄙ"),
    ];
    let (init_p, init_z) = INITIALS
        .iter()
        .find(|(p, _)| py.starts_with(p))
        .map(|(p, z)| (*p, *z))?;
    let rime = &py[init_p.len()..];
    // j/q/x + u… spells ü (ju = ㄐㄩ, juan = ㄐㄩㄢ).
    let rime_norm = if matches!(init_p, "j" | "q" | "x") && rime.starts_with('u') {
        format!("ü{}", &rime[1..])
    } else {
        rime.to_string()
    };
    Some(format!("{init_z}{}", rime_to_zhuyin(&rime_norm)?))
}

/// A Pinyin rime (medial + final, after the initial) → Bopomofo.
fn rime_to_zhuyin(rime: &str) -> Option<&'static str> {
    Some(match rime {
        "a" => "ㄚ",
        "o" => "ㄛ",
        "e" => "ㄜ",
        "ê" => "ㄝ",
        "ai" => "ㄞ",
        "ei" => "ㄟ",
        "ao" => "ㄠ",
        "ou" => "ㄡ",
        "an" => "ㄢ",
        "en" => "ㄣ",
        "ang" => "ㄤ",
        "eng" => "ㄥ",
        "ong" => "ㄨㄥ",
        "i" => "ㄧ",
        "ia" => "ㄧㄚ",
        "ie" => "ㄧㄝ",
        "iao" => "ㄧㄠ",
        "iu" => "ㄧㄡ",
        "ian" => "ㄧㄢ",
        "in" => "ㄧㄣ",
        "iang" => "ㄧㄤ",
        "ing" => "ㄧㄥ",
        "iong" => "ㄩㄥ",
        "u" => "ㄨ",
        "ua" => "ㄨㄚ",
        "uo" => "ㄨㄛ",
        "uai" => "ㄨㄞ",
        "ui" => "ㄨㄟ",
        "uan" => "ㄨㄢ",
        "un" => "ㄨㄣ",
        "uang" => "ㄨㄤ",
        "ueng" => "ㄨㄥ",
        "ü" => "ㄩ",
        "üe" => "ㄩㄝ",
        "üan" => "ㄩㄢ",
        "ün" => "ㄩㄣ",
        "v" => "ㄩ",
        "ve" => "ㄩㄝ",
        "van" => "ㄩㄢ",
        "vn" => "ㄩㄣ",
        "er" => "ㄦ",
        _ => return None,
    })
}

/// Curated seed shared with `build-pinyin` (it carries the Traditional
/// column too). `--seed` uses this when no CC-CEDICT copy is on hand.
static SEED_CEDICT: &str = include_str!("../../../../seeds/pinyin/seed.cedict.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syllables_convert() {
        assert_eq!(syllable_to_zhuyin("ni").as_deref(), Some("ㄋㄧ"));
        assert_eq!(syllable_to_zhuyin("hao").as_deref(), Some("ㄏㄠ"));
        assert_eq!(syllable_to_zhuyin("zhong").as_deref(), Some("ㄓㄨㄥ"));
        assert_eq!(syllable_to_zhuyin("guo").as_deref(), Some("ㄍㄨㄛ"));
        assert_eq!(syllable_to_zhuyin("shi").as_deref(), Some("ㄕ")); // empty rime
        assert_eq!(syllable_to_zhuyin("zi").as_deref(), Some("ㄗ"));
        assert_eq!(syllable_to_zhuyin("xie").as_deref(), Some("ㄒㄧㄝ"));
        assert_eq!(syllable_to_zhuyin("ju").as_deref(), Some("ㄐㄩ")); // jqx + u = ü
        assert_eq!(syllable_to_zhuyin("juan").as_deref(), Some("ㄐㄩㄢ"));
        assert_eq!(syllable_to_zhuyin("yuan").as_deref(), Some("ㄩㄢ"));
        assert_eq!(syllable_to_zhuyin("er").as_deref(), Some("ㄦ"));
        assert_eq!(syllable_to_zhuyin("wo").as_deref(), Some("ㄨㄛ"));
        assert_eq!(syllable_to_zhuyin("lü").as_deref(), Some("ㄌㄩ")); // 綠
    }

    #[test]
    fn whole_pinyin_field_to_key() {
        assert_eq!(
            pinyin_to_zhuyin_key("ni3 hao3").as_deref(),
            Some("ㄋㄧㄏㄠ")
        );
        assert_eq!(
            pinyin_to_zhuyin_key("Zhong1 guo2").as_deref(),
            Some("ㄓㄨㄥㄍㄨㄛ")
        );
        // ü written as `u:` in CC-CEDICT.
        assert_eq!(pinyin_to_zhuyin_key("nu:3").as_deref(), Some("ㄋㄩ"));
    }

    #[test]
    fn parses_traditional_column() {
        let (trad, pin) = parse_cedict_line("中國 中国 [Zhong1 guo2] /China/").unwrap();
        assert_eq!(trad, "中國"); // Traditional, not 中国
        assert_eq!(pin, "Zhong1 guo2");
    }

    #[test]
    fn unconvertible_is_skipped() {
        // A stray non-Mandarin token yields no key rather than panicking.
        assert_eq!(pinyin_to_zhuyin_key("xyz1"), None);
    }
}
