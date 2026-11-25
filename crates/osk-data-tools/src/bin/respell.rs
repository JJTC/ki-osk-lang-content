//! respell: a text in Latin letters spelled in a language's own script by
//! its transliteration table, as the keyboard reads what is typed: each
//! word its likeliest reading that is a word of a word list, or its
//! cheapest reading where none is. For a language written in two scripts
//! whose text is in one of them only (Garo: its Latin-letter text, and the
//! words its Bengali-script writing has): the respelled text is what a
//! model of the other script learns from, and how many of its words the
//! word list has says how near the table's spelling is to the script's.
//!
//! ```text
//! respell --table translit/grt-Beng.txt --corpus bengali.txt --join-dots \
//!     --text in.txt --out out.txt
//! ```
//!
//! The words are counted in `--corpus`, a text in the script, or read from
//! `--words`, a word and its count a line, tab-separated. A word in the
//! text is a run of letters, the marks among them and the apostrophe and
//! `·` within it; with `--join-dots`, a full stop between two letters too,
//! read as `·` (Garo's raka, which some write so: ong.a); the rest is
//! copied as it is. The report says how many of
//! the text's words were read as words of the list, and the commonest that
//! were not.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, bail};
use clap::Parser;
use osk_data::translit::{Scheme, UNKNOWN_BELOW_RAREST};
use osk_data_tools::lexicon::FstLexicon;

#[derive(Parser)]
#[command(about = "Spell a Latin-letter text in a language's script by its transliteration table")]
struct Args {
    /// The table, `translit/<lang>.txt`.
    #[arg(long)]
    table: PathBuf,
    /// The words its readings are chosen among: a word and its count a
    /// line, tab-separated.
    #[arg(long, required_unless_present = "corpus", conflicts_with = "corpus")]
    words: Option<PathBuf>,
    /// The words its readings are chosen among, counted in this text.
    #[arg(long)]
    corpus: Option<PathBuf>,
    /// A full stop between two letters is part of the word, read as `·`.
    #[arg(long)]
    join_dots: bool,
    /// The text, in Latin letters.
    #[arg(long)]
    text: PathBuf,
    /// Where the respelled text goes; without it, only the report.
    #[arg(long)]
    out: Option<PathBuf>,
    /// How many of the commonest words read as no word of the list to show.
    #[arg(long, default_value_t = 30)]
    show: usize,
}

/// The words of `path`, a word and its count a line, tab-separated.
fn read_counts(path: &PathBuf) -> anyhow::Result<Vec<(String, u64)>> {
    let list = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut counts = Vec::new();
    for (i, line) in list.lines().enumerate() {
        let Some((word, count)) = line.split_once('\t') else {
            bail!("{}:{}: no tab", path.display(), i + 1);
        };
        let count: u64 = count
            .trim()
            .parse()
            .with_context(|| format!("{}:{}: no count", path.display(), i + 1))?;
        counts.push((word.to_string(), count));
    }
    Ok(counts)
}

/// The words of the text at `path`, each with how often it is written.
fn count_words(path: &PathBuf) -> anyhow::Result<Vec<(String, u64)>> {
    let text = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut counts: HashMap<String, u64> = HashMap::new();
    for w in text.split(|c: char| !in_word(c)) {
        let w = w.trim_matches(|c| c == '\'' || c == '·');
        if !w.is_empty() {
            *counts.entry(w.to_string()).or_default() += 1;
        }
    }
    Ok(counts.into_iter().collect())
}

/// `text` with a full stop between two letters written `·`.
fn join_dots(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    chars
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let between = i > 0
                && chars[i - 1].is_alphabetic()
                && chars.get(i + 1).is_some_and(|n| n.is_alphabetic());
            if c == '.' && between { '·' } else { c }
        })
        .collect()
}

/// Whether `c` belongs to a word: a letter, a mark, `·`, or an apostrophe
/// (within one).
fn in_word(c: char) -> bool {
    c.is_alphabetic() || unicode_normalization::char::is_combining_mark(c) || c == '·' || c == '\''
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let table = fs::read_to_string(&args.table)
        .with_context(|| format!("reading {}", args.table.display()))?;
    let scheme =
        Scheme::parse(&table).map_err(|e| anyhow::anyhow!("{}: {e}", args.table.display()))?;
    let mut counts: Vec<(String, u64)> = match (&args.words, &args.corpus) {
        (Some(path), _) => read_counts(path)?,
        (None, Some(path)) => count_words(path)?,
        (None, None) => bail!("--words or --corpus"),
    };
    counts.sort();
    counts.dedup_by(|a, b| a.0 == b.0);
    let source = args
        .words
        .as_ref()
        .or(args.corpus.as_ref())
        .map_or_else(String::new, |p| p.display().to_string());
    let mut built = fst::MapBuilder::memory();
    for (word, count) in &counts {
        built.insert(word, *count)?;
    }
    let lexicon = FstLexicon::open(built.into_inner()?, UNKNOWN_BELOW_RAREST)?;

    let mut text = fs::read_to_string(&args.text)
        .with_context(|| format!("reading {}", args.text.display()))?;
    if args.join_dots {
        text = join_dots(&text);
    }
    let mut seen: HashMap<String, (String, bool)> = HashMap::new();
    let mut missed: HashMap<String, u64> = HashMap::new();
    let (mut words, mut found) = (0u64, 0u64);
    let mut out = String::with_capacity(text.len() * 2);
    let mut word = String::new();
    let mut flush = |word: &mut String, out: &mut String| {
        if word.is_empty() {
            return;
        }
        // An apostrophe or `·` at a word's edge is no part of it.
        let core = word.trim_matches(|c| c == '\'' || c == '·');
        let lead = &word[..word.find(core).unwrap_or(0)];
        let tail = &word[lead.len() + core.len()..];
        if core.is_empty() {
            out.push_str(word);
            word.clear();
            return;
        }
        let (spelled, known) = seen
            .entry(core.to_string())
            .or_insert_with(|| {
                match scheme
                    .candidates(core, &lexicon, 1)
                    .into_iter()
                    .find(|c| c.log_prob.is_some())
                {
                    Some(c) => (c.text, true),
                    None => (scheme.literal(core), false),
                }
            })
            .clone();
        words += 1;
        if known {
            found += 1;
        } else {
            *missed.entry(core.to_lowercase()).or_default() += 1;
        }
        out.push_str(lead);
        out.push_str(&spelled);
        out.push_str(tail);
        word.clear();
    };
    for c in text.chars() {
        if in_word(c) {
            word.push(c);
        } else {
            flush(&mut word, &mut out);
            out.push(c);
        }
    }
    flush(&mut word, &mut out);

    #[allow(clippy::cast_precision_loss)]
    let share = 100.0 * found as f64 / words.max(1) as f64;
    println!(
        "{}: {found} of {words} words ({share:.1}%) read as words of {source}",
        args.text.display(),
    );
    let mut missed: Vec<(String, u64)> = missed.into_iter().collect();
    missed.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    for (w, n) in missed.iter().take(args.show) {
        let spelled = seen
            .get(w)
            .map_or_else(|| scheme.literal(w), |s| s.0.clone());
        println!("  {n:>6}  {w}  {spelled}");
    }
    if let Some(path) = &args.out {
        fs::write(path, out).with_context(|| format!("writing {}", path.display()))?;
    }
    Ok(())
}
