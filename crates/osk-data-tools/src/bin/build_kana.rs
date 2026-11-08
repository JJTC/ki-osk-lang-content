//! `build-kana` — ingest mozc-style kana→kanji TSV into a runtime
//! dictionary bundle. Same general shape as `build-pinyin`: parse,
//! aggregate, sort, cap, serialise via `osk_data::codec`.
//!
//! Two input formats are supported, picked by which flag is given:
//!
//! 1. **mozc system-dict TSV** (`--mozc <path>`). One row per
//!    surface/reading pair:
//!
//!    ```text
//!    reading\tlid\trid\tcost\tword
//!    ```
//!
//!    Mozc's `dictionary_oss/dictionary{N}.txt` shards use this
//!    format. Lower `cost` == more common; we sort by cost ascending
//!    so the top-N candidates per reading are the frequent ones.
//!
//! 2. **Curated seed** (`--seed`). Same format as the embedded
//!    `assets/kana/seed.mozc.txt` — `reading\tword` (cost optional;
//!    defaults to insertion order). Lets us ship a usable dict
//!    without a licensed mozc dump.
//!
//! Output: `<out>/mozc-kanji.bin`, an `osk_data::codec`-encoded
//! `BTreeMap<String, Vec<String>>` keyed by reading.

use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::Context as _;
use clap::Parser;
use osk_data::kana::KanjiEntry;

#[derive(Parser)]
#[command(name = "build-kana")]
struct Args {
    /// Path to a mozc-format dictionary TSV. Ignored when `--seed`
    /// is set.
    #[arg(long)]
    mozc: Option<PathBuf>,
    /// Use the bundled curated seed list instead of an external
    /// mozc dump.
    #[arg(long, default_value_t = false)]
    seed: bool,
    /// Output directory. File written as `<out>/mozc-kanji.bin`.
    #[arg(long)]
    out: PathBuf,
    /// Max candidates kept per reading.
    #[arg(long, default_value_t = 9)]
    cap: usize,
    /// Drop entries with a mozc cost above this (mozc costs rank by
    /// frequency: lower = more common; rare compounds and most proper
    /// nouns sit above ~7000). No effect on the 2-column seed format,
    /// whose entries all carry cost 0.
    #[arg(long)]
    max_cost: Option<i32>,
    /// After grouping, keep only the N most frequent readings (ranked
    /// by their best candidate's cost, ties by insertion order). The
    /// full mozc dump yields ~750 k readings ≈ 50 MB encoded, which
    /// a <30 MB self-contained binary can't carry; this puts the
    /// bundle on a deterministic size budget instead.
    #[arg(long)]
    max_readings: Option<usize>,
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    let lines: Vec<String> = if args.seed {
        log::info!("build-kana: using bundled curated seed");
        SEED_MOZC.lines().map(str::to_string).collect()
    } else {
        let Some(path) = &args.mozc else {
            anyhow::bail!("need --mozc <path> or --seed");
        };
        let f = fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
        BufReader::new(f).lines().collect::<Result<_, _>>()?
    };

    // One candidate before final packing: (cost, insertion_order, word, lid, rid).
    type RawEntry = (i32, usize, String, u16, u16);
    // reading → its candidates
    let mut accum: BTreeMap<String, Vec<RawEntry>> = BTreeMap::new();
    let mut order = 0usize;
    let mut dropped_cost = 0usize;
    for line in &lines {
        let Some((reading, lid, rid, cost, word)) = parse_mozc_line(line) else {
            continue;
        };
        if reading.is_empty() || word.is_empty() {
            continue;
        }
        if let Some(max) = args.max_cost
            && cost > max
        {
            dropped_cost += 1;
            continue;
        }
        accum.entry(reading.to_string()).or_default().push((
            cost,
            order,
            word.to_string(),
            lid,
            rid,
        ));
        order += 1;
    }

    if dropped_cost > 0 {
        log::info!("build-kana: dropped {dropped_cost} entries above --max-cost");
    }

    // Rank readings by their best (lowest-cost) candidate so an
    // optional --max-readings keeps the most frequent ones.
    // (best_cost, first_seen, reading, its candidates)
    type RankedReading = (i32, usize, String, Vec<RawEntry>);
    let mut ranked: Vec<RankedReading> = accum
        .into_iter()
        .map(|(reading, entries)| {
            let best = entries
                .iter()
                .map(|(c, o, _, _, _)| (*c, *o))
                .min()
                .expect("groups are never empty");
            (best.0, best.1, reading, entries)
        })
        .collect();
    ranked.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    let total_readings = ranked.len();
    if let Some(max) = args.max_readings {
        ranked.truncate(max);
        if total_readings > max {
            log::info!("build-kana: kept the {max} most frequent readings of {total_readings}");
        }
    }

    let mut table: BTreeMap<String, Vec<KanjiEntry>> = BTreeMap::new();
    for (_, _, reading, mut entries) in ranked {
        // Lower cost first (more common); ties broken by insertion order.
        entries.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        let cands: Vec<KanjiEntry> = entries
            .into_iter()
            .map(|(cost, _, w, lid, rid)| KanjiEntry {
                surface: w,
                cost: i16::try_from(cost).unwrap_or(i16::MAX),
                lid,
                rid,
            })
            .take(args.cap)
            .collect();
        table.insert(reading, cands);
    }

    log::info!("build-kana: readings = {}", table.len());

    fs::create_dir_all(&args.out).with_context(|| format!("mkdir -p {}", args.out.display()))?;
    let out_path = args.out.join("mozc-kanji.bin");
    let bytes = osk_data::codec::encode(&table).context("encode mozc-kanji")?;
    fs::write(&out_path, &bytes).with_context(|| format!("writing {}", out_path.display()))?;
    log::info!("wrote {} ({} bytes)", out_path.display(), bytes.len());
    Ok(())
}

/// Parse one mozc / seed line. Returns `(reading, lid, rid, cost, word)` or
/// `None` for comments / blank / malformed.
///
/// Accepts both the 5-column mozc system-dict format
/// (`reading\tlid\trid\tcost\tword`, carrying the POS ids the runtime
/// connection-cost lattice needs) and the 2-column seed format
/// (`reading\tword`, POS ids default to 0 / BOS-EOS, i.e. no context
/// signal). Anything in between is a 3-column case (`reading\tcost\tword`).
fn parse_mozc_line(line: &str) -> Option<(&str, u16, u16, i32, &str)> {
    let trimmed = line.trim_end_matches(['\r', '\n']);
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    let cols: Vec<&str> = trimmed.split('\t').collect();
    match cols.as_slice() {
        // Two columns: seed format. lid/rid/cost default to 0.
        [reading, word] => Some((*reading, 0, 0, 0, *word)),
        // Three columns: reading, cost, word (no POS).
        [reading, cost, word] => Some((*reading, 0, 0, cost.parse().unwrap_or(0), *word)),
        // Five+ columns: mozc format. lid=1, rid=2, cost=3, word=4.
        cols if cols.len() >= 5 => Some((
            cols[0],
            cols[1].parse().unwrap_or(0),
            cols[2].parse().unwrap_or(0),
            cols[3].parse().unwrap_or(0),
            cols[4],
        )),
        _ => None,
    }
}

/// Curated seed list when no external mozc dump is available.
/// Covers JLPT N5/N4 + greetings. Used by `--seed`.
static SEED_MOZC: &str = include_str!("../../../../seeds/kana/seed.mozc.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_seed_format() {
        let r = parse_mozc_line("にほん\t日本").unwrap();
        assert_eq!(r, ("にほん", 0, 0, 0, "日本"));
    }

    #[test]
    fn parses_three_column_format() {
        let r = parse_mozc_line("にほん\t100\t日本").unwrap();
        assert_eq!(r, ("にほん", 0, 0, 100, "日本"));
    }

    #[test]
    fn parses_mozc_system_dict_format() {
        // Synthetic but follows the real shape: reading lid rid cost word.
        let r = parse_mozc_line("にほん\t1851\t1852\t7236\t日本").unwrap();
        assert_eq!(r, ("にほん", 1851, 1852, 7236, "日本"));
    }

    #[test]
    fn skips_comments_and_blanks() {
        assert!(parse_mozc_line("# comment").is_none());
        assert!(parse_mozc_line("").is_none());
        assert!(parse_mozc_line("   ").is_none());
    }
}
