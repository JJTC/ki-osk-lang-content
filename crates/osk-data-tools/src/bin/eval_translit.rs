//! eval-translit — how often a transliteration table (`translit/<lang>.txt`)
//! and a lexicon (`predict/<lang>.fst`) put the word meant first, as the
//! keyboard's composer offers them.
//!
//! The words meant come from a list of romanizations: one per line, the
//! word in the language's script, a romanization of it as people typed it,
//! and how many typed it so, tab-separated, as the Dakshina dataset's
//! lexicons are (`hi.translit.sampled.test.tsv`):
//!
//!   eval-translit --table translit/hi.txt --lexicon predict/hi.fst \
//!       --pairs eval/translit/hi.tsv [--show 20]
//!
//! Dakshina's romanized sentences, a word and its romanization a line and
//! `</s>` between sentences (`hi.romanized.rejoined.aligned.cased_nopunct.tsv`),
//! measure running text the same way, each word once; a word typed as it
//! is written (a number) is skipped.
//!
//! With `--keystrokes` it also types each romanization a letter at a time,
//! as the composer is typed: a word costs the letters typed until it is
//! one of the strip's two readings, and a tap (or Space) to take it, or
//! all its letters and Space where it never is; keystroke savings is one
//! less what the words cost over what typing them out would.
//!
//! It reports, each pair weighted by how many typed it: the share whose
//! word the lexicon has, the share whose word comes first (top-1, what
//! Space takes) and among the first two (top-2: the strip shows two
//! readings, then the letters as typed), over all pairs and over those the
//! lexicon has, and the share the cheapest reading alone writes right (the
//! table without a lexicon).
//!
//! A word is the same word as the lexicon keys it: its identity
//! (`osk_data::words::Splitter::normalize`, of the table's language, by
//! its file name or `--lang`), so Sindhi's ھ and ه, which its texts write
//! for one h, are one, as are Marathi's two ways of writing an eyelash ra.

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, bail};
use clap::Parser;
use fst::raw::Fst;
use osk_data::translit::{Lexicon, Scheme, UNKNOWN_BELOW_RAREST};
use osk_data::words::Splitter;

#[derive(Parser)]
#[command(about = "Measure a transliteration table against romanizations people typed")]
struct Args {
    /// The table, `translit/<lang>.txt`.
    #[arg(long)]
    table: PathBuf,
    /// The lexicon, `predict/<lang>.fst`; without it, the cheapest reading
    /// alone is measured.
    #[arg(long)]
    lexicon: Option<PathBuf>,
    /// Word, romanization and count, tab-separated, one pair a line.
    #[arg(long)]
    pairs: PathBuf,
    /// Print this many pairs whose word did not come first.
    #[arg(long, default_value_t = 0)]
    show: usize,
    /// How far below the rarest word a reading that is no word is put
    /// (the keyboard's is `UNKNOWN_BELOW_RAREST`): to try another.
    #[arg(long, default_value_t = UNKNOWN_BELOW_RAREST)]
    unknown_below: f32,
    /// Measure keystroke savings too (slower: every prefix is read).
    #[arg(long)]
    keystrokes: bool,
    /// Measure at most this many pairs.
    #[arg(long)]
    limit: Option<usize>,
    /// Offer no word the letters only begin: to compare.
    #[arg(long)]
    no_completions: bool,
    /// The language whose words' identities are compared; the table's file
    /// name (`hi` of `translit/hi.txt`) by default.
    #[arg(long)]
    lang: Option<String>,
}

/// A lexicon that gives no completions.
struct Whole<'a>(&'a dyn Lexicon);

impl Lexicon for Whole<'_> {
    fn has_prefix(&self, prefix: &str) -> bool {
        self.0.has_prefix(prefix)
    }
    fn log_prob(&self, word: &str) -> Option<f32> {
        self.0.log_prob(word)
    }
    fn unknown_log_prob(&self) -> f32 {
        self.0.unknown_log_prob()
    }
}

/// A lexicon file, read as the keyboard reads it: a word's likelihood its
/// count's share of all the counts.
struct FstLexicon {
    map: fst::Map<Vec<u8>>,
    log_total: f32,
    unknown: f32,
}

impl FstLexicon {
    fn open(bytes: Vec<u8>, below: f32) -> anyhow::Result<Self> {
        let map = fst::Map::new(bytes)?;
        let mut total: u64 = 0;
        let mut rarest = u64::MAX;
        let mut stream = map.stream();
        while let Some((_, v)) = fst::Streamer::next(&mut stream) {
            total = total.saturating_add(v);
            rarest = rarest.min(v.max(1));
        }
        #[allow(clippy::cast_precision_loss)]
        let log_total = (total.max(1) as f32).ln();
        #[allow(clippy::cast_precision_loss)]
        let unknown = (rarest as f32).ln() - log_total - below;
        Ok(Self {
            map,
            log_total,
            unknown,
        })
    }
}

impl Lexicon for FstLexicon {
    fn has_prefix(&self, prefix: &str) -> bool {
        let fst: &Fst<Vec<u8>> = self.map.as_fst();
        let mut node = fst.root();
        for b in prefix.bytes() {
            let Some(i) = node.find_input(b) else {
                return false;
            };
            node = fst.node(node.transition_addr(i));
        }
        node.is_final() || !node.is_empty()
    }

    fn log_prob(&self, word: &str) -> Option<f32> {
        #[allow(clippy::cast_precision_loss)]
        self.map
            .get(word.as_bytes())
            .map(|c| (c.max(1) as f32).ln() - self.log_total)
    }

    fn unknown_log_prob(&self) -> f32 {
        self.unknown
    }

    fn completions(&self, prefix: &str, limit: usize) -> Vec<(String, f32)> {
        use fst::{IntoStreamer, Streamer, automaton::Str};
        let aut = fst::Automaton::starts_with(Str::new(prefix));
        let mut stream = self.map.search(aut).into_stream();
        let mut found: Vec<(String, f32)> = Vec::new();
        while let Some((k, v)) = stream.next() {
            if k.len() > prefix.len()
                && let Ok(w) = std::str::from_utf8(k)
            {
                #[allow(clippy::cast_precision_loss)]
                found.push((w.to_string(), (v.max(1) as f32).ln() - self.log_total));
            }
        }
        found.sort_by(|a, b| b.1.total_cmp(&a.1));
        found.truncate(limit);
        found
    }
}

/// No lexicon: every reading is unknown, so the cheapest comes first.
struct NoLexicon;

impl Lexicon for NoLexicon {
    fn has_prefix(&self, _: &str) -> bool {
        false
    }
    fn log_prob(&self, _: &str) -> Option<f32> {
        None
    }
    fn unknown_log_prob(&self) -> f32 {
        0.0
    }
}

#[derive(Default)]
struct Tally {
    pairs: f64,
    known: f64,
    top1: f64,
    top3: f64,
    known_top1: f64,
    known_top3: f64,
    literal: f64,
    /// Keystrokes the strip saved: what typing took, what typing out takes.
    took: f64,
    out: f64,
    early: f64,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let table = fs::read_to_string(&args.table)
        .with_context(|| format!("reading {}", args.table.display()))?;
    let scheme = Scheme::parse(&table).with_context(|| format!("{}", args.table.display()))?;
    let lexicon: Box<dyn Lexicon> = match &args.lexicon {
        Some(path) => Box::new(FstLexicon::open(
            fs::read(path).with_context(|| format!("reading {}", path.display()))?,
            args.unknown_below,
        )?),
        None => Box::new(NoLexicon),
    };
    let whole = Whole(lexicon.as_ref());
    let lexicon: &dyn Lexicon = if args.no_completions {
        &whole
    } else {
        lexicon.as_ref()
    };
    let lang = match &args.lang {
        Some(lang) => lang.clone(),
        None => args
            .table
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .with_context(|| format!("{}: no language in its name", args.table.display()))?,
    };
    let splitter = Splitter::new(&lang);
    let same = |text: &str, word: &str| splitter.normalize(text) == word;
    let pairs = fs::read_to_string(&args.pairs)
        .with_context(|| format!("reading {}", args.pairs.display()))?;
    let mut t = Tally::default();
    let mut shown = 0;
    for (i, line) in pairs
        .lines()
        .enumerate()
        .take(args.limit.unwrap_or(usize::MAX))
    {
        let mut fields = line.split('\t');
        let (Some(word), Some(latin)) = (fields.next(), fields.next()) else {
            bail!(
                "{}:{}: not word<TAB>romanization",
                args.pairs.display(),
                i + 1
            );
        };
        let n: f64 = fields.next().and_then(|c| c.parse().ok()).unwrap_or(1.0);
        if word == "</s>" || word == latin || latin.is_empty() {
            continue;
        }
        let word = splitter.normalize(word);
        let got = scheme.candidates(latin, lexicon, 2);
        let known = lexicon.log_prob(&word).is_some();
        let first = got.first().is_some_and(|c| same(&c.text, &word));
        let three = got.iter().any(|c| same(&c.text, &word));
        t.pairs += n;
        if known {
            t.known += n;
        }
        if first {
            t.top1 += n;
            if known {
                t.known_top1 += n;
            }
        }
        if three {
            t.top3 += n;
            if known {
                t.known_top3 += n;
            }
        }
        if same(&scheme.literal(latin), &word) {
            t.literal += n;
        }
        if args.keystrokes {
            let letters: Vec<(usize, char)> = latin.char_indices().collect();
            #[allow(clippy::cast_precision_loss)]
            let full = (letters.len() + 1) as f64;
            let mut cost = full;
            for (k, _) in letters.iter().enumerate() {
                let end = letters.get(k + 1).map_or(latin.len(), |(at, _)| *at);
                let strip = scheme.candidates(&latin[..end], lexicon, 2);
                if strip.iter().any(|c| same(&c.text, &word)) {
                    #[allow(clippy::cast_precision_loss)]
                    {
                        cost = (k + 2) as f64;
                    }
                    if k + 1 < letters.len() {
                        t.early += n;
                    }
                    break;
                }
            }
            t.took += n * cost;
            t.out += n * full;
        }
        if !first && shown < args.show {
            shown += 1;
            let offered: Vec<&str> = got.iter().map(|c| c.text.as_str()).collect();
            println!(
                "{latin}\t{word}{}\t{}",
                if known { "" } else { " (not a word)" },
                offered.join(" ")
            );
        }
    }
    let pct = |a: f64, b: f64| if b > 0.0 { 100.0 * a / b } else { 0.0 };
    println!(
        "{}: {:.0} typed (weighted), the lexicon has {:.1}%",
        args.table.display(),
        t.pairs,
        pct(t.known, t.pairs)
    );
    println!(
        "  top-1 {:.1}%  top-2 {:.1}%   of words the lexicon has: top-1 {:.1}%  top-2 {:.1}%   cheapest reading right: {:.1}%",
        pct(t.top1, t.pairs),
        pct(t.top3, t.pairs),
        pct(t.known_top1, t.known),
        pct(t.known_top3, t.known),
        pct(t.literal, t.pairs)
    );
    if args.keystrokes {
        println!(
            "  keystroke savings {:.1}%   offered before the last letter: {:.1}%",
            pct(t.out - t.took, t.out),
            pct(t.early, t.pairs)
        );
    }
    Ok(())
}
