//! build-english — the English words a language writes in its own script,
//! as its transliteration table offers them (`translit/<lang>.english.txt`,
//! `osk_data::translit::Scheme::with_english`).
//!
//! People typing their language in Latin letters type the English words it
//! has taken as English spells them (`computer`, `college`), which a table
//! of how the language's own sounds are typed does not read as the word
//! (कंप्यूटर, कॉलेज). How English says each word, CMUdict's pronunciation
//! (ARPAbet), is spelled in Latin letters as the language's sounds are
//! typed (`kampyootar`), a few ways, and each read by the table: the
//! likeliest reading that is a word of the lexicon is how the language
//! writes the English word. An English word the table reads as that word
//! already is left out, and one whose reading cost more than `--max-cost`.
//!
//!   build-english --table translit/hi.txt --lexicon predict/hi.fst \
//!       --cmudict cmudict.dict --out translit/hi.english.txt
//!
//! The Latin spellings of English's sounds below are this repository's;
//! CMUdict is Carnegie Mellon University's (BSD-2-Clause-style notice in
//! NOTICES.md), the words the language's lexicon's.

use std::collections::{BTreeMap, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::Parser;
use osk_data::translit::{Lexicon, Scheme, UNKNOWN_BELOW_RAREST};
use osk_data_tools::lexicon::FstLexicon;

#[derive(Parser)]
#[command(
    about = "The English words a language writes in its script, from CMUdict's pronunciations"
)]
struct Args {
    /// The language's table, `translit/<lang>.txt`.
    #[arg(long)]
    table: PathBuf,
    /// Its lexicon, `predict/<lang>.fst`.
    #[arg(long)]
    lexicon: PathBuf,
    /// CMUdict (`cmudict.dict`): a word and its sounds a line.
    #[arg(long)]
    cmudict: PathBuf,
    /// Where to write the words.
    #[arg(long)]
    out: PathBuf,
    /// The costliest reading kept, in nats: the Latin spelling's and the
    /// table's reading of it.
    #[arg(long, default_value_t = 6.0)]
    max_cost: f32,
    /// How much a reading's cost counts against its word's likelihood, in
    /// choosing among the readings: above 1, a word that fits the sounds
    /// better wins over a commoner one (case: केस, not किस).
    #[arg(long, default_value_t = 1.0)]
    fit: f32,
    /// How many of a word's readings are kept, the likeliest first: those
    /// within `--second` nats of the likeliest.
    #[arg(long, default_value_t = 2)]
    readings: usize,
    #[arg(long, default_value_t = 3.0)]
    second: f32,
    /// How many Latin spellings of each pronunciation are read.
    #[arg(long, default_value_t = 12)]
    spellings: usize,
    /// Only the words of this lexicon (`predict/en.fst`): English's
    /// words, not the names CMUdict says too.
    #[arg(long)]
    only: Option<PathBuf>,
    /// English's words (`predict/en.fst`): a word of CMUdict this lexicon
    /// lacks, a name or a word of another language it says too (hain,
    /// yah), is left out where the table reads its letters as a word of
    /// the language already.
    #[arg(long)]
    english_words: Option<PathBuf>,
    /// Only the commonest this many of `--english-words` count as English's:
    /// a lexicon learned from the web's text holds more of the rare words
    /// that cost a language's own readings.
    #[arg(long)]
    english_top: Option<usize>,
    /// The fewest letters of a word kept: the table names single letters
    /// (`[letters]`), and two are mostly words of the language's own.
    #[arg(long, default_value_t = 3)]
    min_letters: usize,
    /// Threads (all the machine's by default).
    #[arg(long)]
    threads: Option<usize>,
    /// What the file's first lines say of where it came from.
    #[arg(long, default_value = "")]
    source: String,
}

/// How each of English's sounds (ARPAbet, stress aside) is typed in Latin
/// letters, as Indian languages, Urdu and the others are typed: each way
/// and its cost, in nats. A vowel's stress picks among the schwa's.
fn spellings(phone: &str) -> &'static [(&'static str, f32)] {
    match phone {
        "AA" => &[("aa", 0.0), ("o", 0.5), ("a", 0.7)],
        "AE" => &[("ai", 0.3), ("e", 0.5), ("a", 0.7), ("aa", 0.8)],
        "AH1" | "AH2" => &[("a", 0.0), ("u", 1.0)],
        "AH" | "AH0" => &[
            ("a", 0.0),
            ("", 0.8),
            ("o", 0.8),
            ("e", 1.0),
            ("i", 1.0),
            ("u", 1.0),
        ],
        "AO" => &[("o", 0.0), ("aa", 0.5), ("au", 1.0)],
        "AW" => &[("au", 0.0), ("aau", 0.5)],
        "AY" => &[("aai", 0.0), ("ai", 0.3), ("aay", 1.0)],
        "EH" => &[("e", 0.0), ("ai", 1.0)],
        "ER" => &[("ar", 0.0), ("er", 0.5)],
        "EY" => &[("e", 0.0), ("ey", 0.7)],
        "IH" => &[("i", 0.0), ("e", 1.0)],
        "IY" => &[("ee", 0.0), ("i", 0.3)],
        "OW" => &[("o", 0.0)],
        "OY" => &[("oy", 0.0), ("oi", 0.5)],
        "UH" => &[("u", 0.0), ("oo", 0.5)],
        "UW" => &[("oo", 0.0), ("u", 0.3)],
        "B" => &[("b", 0.0)],
        "CH" => &[("ch", 0.0)],
        "D" => &[("d", 0.0)],
        "DH" => &[("d", 0.0), ("th", 0.5)],
        "F" => &[("f", 0.0), ("ph", 1.0)],
        "G" => &[("g", 0.0)],
        "HH" => &[("h", 0.0)],
        "JH" => &[("j", 0.0)],
        "K" => &[("k", 0.0)],
        "L" => &[("l", 0.0)],
        "M" => &[("m", 0.0)],
        "N" => &[("n", 0.0)],
        "NG" => &[("ng", 0.0), ("n", 1.0)],
        "P" => &[("p", 0.0)],
        "R" => &[("r", 0.0)],
        "S" => &[("s", 0.0)],
        "SH" => &[("sh", 0.0)],
        "T" => &[("t", 0.0)],
        "TH" => &[("th", 0.0), ("t", 0.5)],
        "V" => &[("v", 0.0), ("w", 0.5)],
        "W" => &[("w", 0.0), ("v", 0.3)],
        "Y" => &[("y", 0.0)],
        "Z" => &[("z", 0.0), ("j", 0.5), ("s", 1.0)],
        "ZH" => &[("zh", 0.0), ("j", 0.5), ("z", 0.5)],
        _ => &[],
    }
}

fn is_vowel(phone: &str) -> bool {
    phone.ends_with(['0', '1', '2'])
}

/// The ways of typing `phones[i]` where it is: its own, and the ways the
/// English the languages took from (British, as it was taught) says it
/// beside its neighbours. A vowel before another is glided into it with
/// `y` or `w` (media: meediya, मीडिया); `oo` after n, t or d is `yoo`
/// (new: nyoo, न्यू); and `e` before r is `eya` (share: sheyar, शेयर). A
/// word that ends in a consonant is spelled with a u after it too.
fn ways_at(phones: &[&str], i: usize) -> Vec<(String, f32)> {
    let phone = phones[i];
    let base = phone.trim_end_matches(['0', '1', '2']);
    let own = match spellings(phone) {
        [] => spellings(base),
        ways => ways,
    };
    let mut ways: Vec<(String, f32)> = own.iter().map(|(w, c)| ((*w).to_string(), *c)).collect();
    let before = i
        .checked_sub(1)
        .map(|j| phones[j].trim_end_matches(['0', '1', '2']));
    let after = phones.get(i + 1).copied();
    if matches!(base, "UW" | "UH") && matches!(before, Some("N" | "T" | "D")) {
        ways.extend(own.iter().map(|(w, c)| (format!("y{w}"), c + 0.4)));
    }
    if matches!(base, "EH" | "EY") && after.is_some_and(|a| a == "R") {
        ways.push(("eya".to_string(), 0.3));
    }
    if matches!(base, "IH" | "IY") && after.is_some_and(|a| a == "R") {
        ways.push(("iya".to_string(), 0.5));
    }
    if after.is_some_and(is_vowel) && is_vowel(phone) {
        let glide = if matches!(base, "UW" | "UH" | "OW" | "AW") {
            "w"
        } else {
            "y"
        };
        ways = ways
            .into_iter()
            .flat_map(|(w, c)| [(format!("{w}{glide}"), c), (w, c + 0.5)])
            .collect();
    }
    ways
}

/// The `limit` cheapest Latin spellings of `phones`, each with its cost;
/// none where a sound is not ARPAbet's.
fn latin(phones: &[&str], limit: usize) -> Vec<(String, f32)> {
    let mut at: Vec<(String, f32)> = vec![(String::new(), 0.0)];
    for i in 0..phones.len() {
        let ways = ways_at(phones, i);
        if ways.is_empty() {
            return Vec::new();
        }
        let mut next: Vec<(String, f32)> = Vec::with_capacity(at.len() * ways.len());
        for (text, cost) in &at {
            for (way, c) in &ways {
                next.push((format!("{text}{way}"), cost + c));
            }
        }
        next.sort_by(|a, b| a.1.total_cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
        next.dedup_by(|a, b| a.0 == b.0);
        next.truncate(limit.max(1) * 4);
        at = next;
    }
    at.truncate(limit);
    // A last consonant with a u after it, as the Dravidian languages write
    // the English words they take (court: కోర్టు, college: ಕಾಲೇಜು).
    if phones.last().is_some_and(|p| !is_vowel(p)) {
        let with_u: Vec<(String, f32)> = at
            .iter()
            .map(|(t, c)| (format!("{t}u"), c + FINAL_U))
            .collect();
        at.extend(with_u);
    }
    at
}

/// What a u typed after an English word's last consonant costs.
const FINAL_U: f32 = 0.6;

/// A lexicon that gives no completions: only the readings of all the
/// letters count.
struct Whole<'a>(&'a FstLexicon);

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

/// English's words: those of the lexicon at `path`, or its `top` commonest.
fn english_words(path: &Path, top: Option<usize>) -> anyhow::Result<HashSet<String>> {
    let map =
        fst::Map::new(fs::read(path).with_context(|| format!("reading {}", path.display()))?)?;
    let mut words: Vec<(u64, String)> = Vec::new();
    let mut stream = map.stream();
    while let Some((word, count)) = fst::Streamer::next(&mut stream) {
        if let Ok(word) = std::str::from_utf8(word) {
            words.push((count, word.to_string()));
        }
    }
    if let Some(top) = top {
        words.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
        words.truncate(top);
    }
    Ok(words.into_iter().map(|(_, word)| word).collect())
}

/// How `word` (its pronunciations `sounds`) is written: the likeliest
/// reading of its Latin spellings that is a word, unless the table reads
/// `word` as it first already, or it costs more than `max_cost`.
fn written(
    scheme: &Scheme,
    lexicon: &Whole<'_>,
    english: Option<&HashSet<String>>,
    word: &str,
    sounds: &[Vec<&str>],
    args: &Args,
) -> Option<String> {
    // Each reading that is a word, at its best score.
    let mut found: Vec<(f32, String)> = Vec::new();
    for phones in sounds {
        for (spelled, spelling_cost) in latin(phones, args.spellings) {
            for c in scheme.candidates(&spelled, lexicon, 4) {
                let (Some(log_prob), false) = (c.log_prob, c.completes) else {
                    continue;
                };
                let cost = spelling_cost + c.cost;
                if cost > args.max_cost {
                    continue;
                }
                let score = log_prob - args.fit * cost;
                match found.iter_mut().find(|(_, t)| *t == c.text) {
                    Some(slot) => slot.0 = slot.0.max(score),
                    None => found.push((score, c.text)),
                }
            }
        }
    }
    found.sort_by(|a, b| b.0.total_cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    let top = found.first()?.0;
    let first = scheme.candidates(word, lexicon, 1);
    let first = first.first();
    let not_english = english.is_some_and(|e| !e.contains(word));
    if not_english && first.is_some_and(|c| c.log_prob.is_some()) {
        return None;
    }
    // The likeliest, and those within `--second` nats of it, up to
    // `--readings`, but for the one the table reads the word as first.
    let kept: Vec<String> = found
        .into_iter()
        .take_while(|(s, _)| top - s <= args.second)
        .take(args.readings)
        .map(|(_, t)| t)
        .filter(|t| first.is_none_or(|c| c.text != *t))
        .collect();
    (!kept.is_empty()).then(|| kept.join(" "))
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let table = fs::read_to_string(&args.table)
        .with_context(|| format!("reading {}", args.table.display()))?;
    let scheme = Scheme::parse(&table).with_context(|| format!("{}", args.table.display()))?;
    let lexicon = FstLexicon::open(
        fs::read(&args.lexicon).with_context(|| format!("reading {}", args.lexicon.display()))?,
        UNKNOWN_BELOW_RAREST,
    )?;
    let english = match &args.english_words {
        Some(path) => Some(english_words(path, args.english_top)?),
        None => None,
    };
    let only = match &args.only {
        Some(path) => Some(FstLexicon::open(
            fs::read(path).with_context(|| format!("reading {}", path.display()))?,
            UNKNOWN_BELOW_RAREST,
        )?),
        None => None,
    };
    let dict = fs::read_to_string(&args.cmudict)
        .with_context(|| format!("reading {}", args.cmudict.display()))?;
    // Each word's pronunciations: `word`, `word(2)`…
    let mut sounds: BTreeMap<&str, Vec<Vec<&str>>> = BTreeMap::new();
    for line in dict.lines() {
        let line = line.split('#').next().unwrap_or("");
        let mut fields = line.split_whitespace();
        let Some(head) = fields.next() else { continue };
        let word = head.split('(').next().unwrap_or(head);
        if word.len() < args.min_letters
            || !word.bytes().all(|b| b.is_ascii_lowercase())
            || only.as_ref().is_some_and(|o| o.log_prob(word).is_none())
        {
            continue;
        }
        sounds.entry(word).or_default().push(fields.collect());
    }
    let words: Vec<(&str, Vec<Vec<&str>>)> = sounds.into_iter().collect();
    let threads = args
        .threads
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, usize::from))
        .max(1);
    let chunk = words.len().div_ceil(threads).max(1);
    let whole = Whole(&lexicon);
    let found: Vec<(&str, String)> = std::thread::scope(|scope| {
        let handles: Vec<_> = words
            .chunks(chunk)
            .map(|part| {
                let (scheme, whole, args, english) = (&scheme, &whole, &args, english.as_ref());
                scope.spawn(move || {
                    part.iter()
                        .filter_map(|(word, s)| {
                            written(scheme, whole, english, word, s, args).map(|t| (*word, t))
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().expect("a thread of build-english panicked"))
            .collect()
    });
    let mut out = String::new();
    for line in args.source.lines() {
        writeln!(out, "# {line}").expect("a String takes it");
    }
    for (word, text) in &found {
        writeln!(out, "{word} {text}").expect("a String takes it");
    }
    fs::write(&args.out, out).with_context(|| format!("writing {}", args.out.display()))?;
    eprintln!(
        "{}: {} English words of {} written as the lexicon has them",
        args.out.display(),
        found.len(),
        words.len()
    );
    Ok(())
}
