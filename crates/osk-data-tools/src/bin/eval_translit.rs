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
//! With `--known` only the pairs whose word the lexicon has are measured:
//! a related language's romanizations then measure a table on the words
//! the two languages share (Bhojpuri's on Dakshina's Hindi).
//!
//! A word is the same word as the lexicon keys it: its identity
//! (`osk_data::words::Splitter::normalize`, of the table's language, by
//! its file name or `--lang`), so Sindhi's ھ and ه, which its texts write
//! for one h, are one, as are Marathi's two ways of writing an eyelash ra.

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, bail};
use clap::Parser;
use osk_data::ngram::NgramModel;
use osk_data::translit::{CONTEXT_WEIGHT, Lexicon, Scheme, UNKNOWN_BELOW_RAREST};
use osk_data::words::Splitter;
use osk_data_tools::lexicon::FstLexicon;

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
    /// Measure only the pairs whose word the lexicon has: a related
    /// language's romanizations, of the words the two share.
    #[arg(long)]
    known: bool,
    /// The language's n-grams (`predict/<lang>.ngram.bin`): each word's
    /// readings ranked after the words before it in its sentence, as the
    /// keyboard ranks them after the words typed
    /// (`osk_data::translit::CONTEXT_WEIGHT`).
    #[arg(long)]
    ngrams: Option<PathBuf>,
    /// The weight of the words before, to try another than the keyboard's.
    #[arg(long, default_value_t = CONTEXT_WEIGHT)]
    context_weight: f32,
    /// Write every pair's readings here, a line each (its line, the word,
    /// the romanization, then each reading's text, likelihood and cost),
    /// and `</s>` where the pairs have it: for trying other rankings.
    #[arg(long)]
    dump: Option<PathBuf>,
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

/// A lexicon that knows the words before the one typed: how likely each
/// word is after them, from the n-grams, the weight scaled to
/// `--context-weight`.
struct InContext<'a> {
    inner: &'a dyn Lexicon,
    model: &'a NgramModel,
    before: &'a [String],
    splitter: &'a Splitter,
    scale: f32,
}

impl Lexicon for InContext<'_> {
    fn has_prefix(&self, prefix: &str) -> bool {
        self.inner.has_prefix(prefix)
    }
    fn log_prob(&self, word: &str) -> Option<f32> {
        self.inner.log_prob(word)
    }
    fn unknown_log_prob(&self) -> f32 {
        self.inner.unknown_log_prob()
    }
    fn completions(&self, prefix: &str, limit: usize) -> Vec<(String, f32)> {
        self.inner.completions(prefix, limit)
    }
    fn context_log_prob(&self, word: &str) -> Option<f32> {
        let prev = self.before.last()?;
        let alone = self.inner.log_prob(word)?;
        let prev_alone = self.inner.log_prob(prev)?;
        let after = osk_data::translit::log_prob_after(
            self.model,
            prev,
            &self.splitter.normalize(word),
            prev_alone,
            alone,
        );
        Some(alone + (after - alone) * self.scale)
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
    let model = match &args.ngrams {
        Some(path) => Some(
            NgramModel::from_bytes(
                &fs::read(path).with_context(|| format!("reading {}", path.display()))?,
            )
            .with_context(|| format!("{}", path.display()))?,
        ),
        None => None,
    };
    // The sentence's last two words before the one typed.
    let mut before: Vec<String> = Vec::new();
    let mut t = Tally::default();
    let mut shown = 0;
    let mut dump = match &args.dump {
        Some(path) => Some(std::io::BufWriter::new(
            fs::File::create(path).with_context(|| format!("writing {}", path.display()))?,
        )),
        None => None,
    };
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
        let context = before.clone();
        if word == "</s>" {
            before.clear();
        } else {
            before.push(splitter.normalize(word));
            if before.len() > 2 {
                before.remove(0);
            }
        }
        if word == "</s>" || word == latin || latin.is_empty() {
            if let Some(d) = dump.as_mut() {
                use std::io::Write as _;
                writeln!(d, "{}\t{word}\t{latin}", i + 1)?;
            }
            continue;
        }
        let word = splitter.normalize(word);
        let known = lexicon.log_prob(&word).is_some();
        if args.known && !known {
            continue;
        }
        if let Some(d) = dump.as_mut() {
            use std::io::Write as _;
            let unknown = lexicon.unknown_log_prob();
            write!(d, "{}\t{word}\t{latin}", i + 1)?;
            for c in scheme.candidates(latin, lexicon, 6) {
                write!(
                    d,
                    "\t{}|{:.4}|{:.4}|{}",
                    splitter.normalize(&c.text),
                    c.log_prob.unwrap_or(unknown),
                    c.cost,
                    u8::from(c.completes)
                )?;
            }
            writeln!(d)?;
        }
        let in_context = model.as_ref().map(|m| InContext {
            inner: lexicon,
            model: m,
            before: &context,
            splitter: &splitter,
            scale: args.context_weight / CONTEXT_WEIGHT,
        });
        let lexicon: &dyn Lexicon = match &in_context {
            Some(c) => c,
            None => lexicon,
        };
        let got = scheme.candidates(latin, lexicon, 2);
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
