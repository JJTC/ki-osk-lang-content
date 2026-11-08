//! `build-pinyin` — ingest CC-CEDICT (CC BY-SA 4.0) into a runtime
//! Pinyin → Hanzi dictionary bundle.
//!
//! Input: a file in CC-CEDICT format, one entry per line:
//!
//! ```text
//! # comment
//! 你好 你好 [ni3 hao3] /hello/hi/
//! 中國 中国 [Zhong1 guo2] /China/
//! ```
//!
//! Output: `<out>/cedict.bin`, an `asset_codec`-encoded `BTreeMap<String,
//! Vec<String>>` mapping tone-stripped lowercased Pinyin (no spaces)
//! to an ordered list of simplified-Hanzi candidates. The runtime
//! `CedictPinyinDict` mmap-friendly? Not really — we just load the
//! whole blob into a `BTreeMap` at startup; CC-CEDICT is only ~120 k
//! entries and a few MB serialised.
//!
//! Candidate ordering:
//!   1. **By corpus frequency, descending** (`--freq`), which is the
//!      half of CLAUDE.md §4.10.1 — "Pinyin → Hanzi *with frequency*" —
//!      that was specified and never built. Without it the order was
//!      "shortest first, ties by input order", and CC-CEDICT's input
//!      order is essentially Unicode codepoint order, so the archaic
//!      variants sort ahead of the characters people actually type:
//!      `ni` gave 伱 伲 伲 (你 fourth), `hao` gave 嗥 嗥 嚆 (好 sixth),
//!      and `de` did not reach 的 — the single most common character in
//!      the language — inside the top eight. The panel shows THREE
//!      candidates, so those keys were simply unusable.
//!   2. Then shorter hanzi sequences first, then input order, as
//!      tie-breaks for everything the frequency list has never seen.
//!   3. Duplicate surfaces are dropped — CC-CEDICT lists a form once per
//!      sense, and repeats were consuming the visible slots (好 twice,
//!      中 three times).
//!   4. Cap at `--cap` candidates per Pinyin key (default 9, matching
//!      CLAUDE.md §4.10.1's candidate bar).
//!
//! Usage:
//!   # real dataset
//!   build-pinyin --cedict cc-cedict.u8 --out pinyin/
//!   # bundled seed (no CC-CEDICT file needed)
//!   build-pinyin --seed --out pinyin/
//!
//! The `--seed` mode emits a small curated dictionary of ~200 common
//! HSK-1/2 entries so the OSK ships with usable Pinyin input even
//! when no licensed CC-CEDICT copy is available.

use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::Context as _;
use clap::Parser;

#[derive(Parser)]
#[command(name = "build-pinyin")]
struct Args {
    /// Path to the CC-CEDICT source file. Ignored when `--seed` is set.
    #[arg(long)]
    cedict: Option<PathBuf>,
    /// Use the bundled curated seed list instead of an external
    /// CC-CEDICT file.
    #[arg(long, default_value_t = false)]
    seed: bool,
    /// Output directory. File written as `<out>/cedict.bin`.
    #[arg(long)]
    out: PathBuf,
    /// Max candidates kept per Pinyin key.
    #[arg(long, default_value_t = 9)]
    cap: usize,
    /// Word-frequency list used to rank candidates: `<word> <count>`
    /// per line, descending. Optional — without it the ordering falls
    /// back to length-then-input-order, which is what shipped before
    /// and is bad enough to make single-syllable input unusable (see
    /// the module docs).
    #[arg(long)]
    freq: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    let lines: Vec<String> = if args.seed {
        log::info!("build-pinyin: using bundled curated seed");
        SEED_CEDICT.lines().map(str::to_string).collect()
    } else {
        let Some(path) = &args.cedict else {
            anyhow::bail!("need --cedict <path> or --seed");
        };
        let f = fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
        BufReader::new(f).lines().collect::<Result<_, _>>()?
    };

    let freq = match &args.freq {
        Some(p) => {
            let f = osk_data_tools::load_freq_list(p)
                .with_context(|| format!("reading {}", p.display()))?;
            log::info!("build-pinyin: frequency entries = {}", f.len());
            f
        }
        None => {
            log::warn!(
                "build-pinyin: no --freq list; candidates will be ordered by length \
                 and input order, which puts archaic variants first (see module docs)"
            );
            std::collections::HashMap::new()
        }
    };

    // (pinyin_key, hanzi_char_count, insertion_order) → hanzi
    let mut accum: BTreeMap<String, Vec<(usize, usize, String)>> = BTreeMap::new();
    let mut order = 0usize;
    for line in &lines {
        let Some((simp, pinyin)) = parse_cedict_line(line) else {
            continue;
        };
        let key = normalize_pinyin(pinyin);
        if key.is_empty() {
            continue;
        }
        let char_count = simp.chars().count();
        accum
            .entry(key)
            .or_default()
            .push((char_count, order, simp.to_string()));
        order += 1;
    }

    let mut table: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut ranked_keys = 0usize;
    for (key, entries) in accum {
        // Frequency first (descending), then shorter-first, then input
        // order. Unknown surfaces score 0 and so keep the old relative
        // order among themselves.
        let cands = osk_data_tools::order_candidates(entries, &freq, args.cap);
        if cands.first().is_some_and(|c| freq.contains_key(c)) {
            ranked_keys += 1;
        }
        table.insert(key, cands);
    }
    log::info!(
        "build-pinyin: {ranked_keys} of {} keys lead with a frequency-ranked candidate",
        table.len()
    );

    log::info!("build-pinyin: pinyin keys = {}", table.len());

    fs::create_dir_all(&args.out).with_context(|| format!("mkdir -p {}", args.out.display()))?;
    let out_path = args.out.join("cedict.bin");
    let bytes = osk_data::codec::encode(&table).context("encode cedict")?;
    fs::write(&out_path, &bytes).with_context(|| format!("writing {}", out_path.display()))?;
    log::info!("wrote {} ({} bytes)", out_path.display(), bytes.len());
    Ok(())
}

/// Parse one CC-CEDICT line. Returns `(simplified_hanzi, pinyin_raw)`
/// or `None` if the line is a comment / blank / malformed.
///
/// Format: `Trad Simp [pin1 yin1] /def/def/`
fn parse_cedict_line(line: &str) -> Option<(&str, &str)> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    // Skip Traditional (first whitespace-delimited token).
    let rest = trimmed.split_once(char::is_whitespace)?.1.trim_start();
    let (simp, rest) = rest.split_once(char::is_whitespace)?;
    let rest = rest.trim_start();
    // Pinyin is bracketed in `[...]`.
    let after_open = rest.strip_prefix('[')?;
    let close = after_open.find(']')?;
    let pinyin = &after_open[..close];
    Some((simp, pinyin))
}

/// Normalize raw CC-CEDICT pinyin into the exact-match lookup key used
/// by the composer: lowercase, drop tone digits, drop spaces, drop the
/// apostrophes that CC-CEDICT sometimes uses to disambiguate syllable
/// boundaries (e.g. `Xi'an` → `xian`).
fn normalize_pinyin(raw: &str) -> String {
    raw.chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(char::to_lowercase)
        .collect()
}

/// Curated seed list when no external CC-CEDICT file is available.
/// Covers HSK-1 + commonly-typed greetings. Used by `--seed`.
static SEED_CEDICT: &str = include_str!("../../../../seeds/pinyin/seed.cedict.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_nihao_line() {
        let (simp, pin) = parse_cedict_line("你好 你好 [ni3 hao3] /hello/hi/").unwrap();
        assert_eq!(simp, "你好");
        assert_eq!(pin, "ni3 hao3");
    }
    #[test]
    fn skips_comments() {
        assert!(parse_cedict_line("# this is a comment").is_none());
        assert!(parse_cedict_line("").is_none());
    }
    #[test]
    fn normalizes_tones_and_spaces() {
        assert_eq!(normalize_pinyin("ni3 hao3"), "nihao");
        assert_eq!(normalize_pinyin("Zhong1 guo2"), "zhongguo");
        assert_eq!(normalize_pinyin("Xi1'an1"), "xian");
    }
}
