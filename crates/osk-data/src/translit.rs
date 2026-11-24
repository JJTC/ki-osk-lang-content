//! Transliteration tables (`translit/<lang>.txt`): how the Latin letters a
//! keyboard types become a language's own script, as people write their
//! language in Latin letters where its script is not at hand: `namaste`
//! becomes नमस्ते, `kalimera` καλημέρα.
//!
//! Such Latin spelling is loose. `t` is त or ट, a long vowel is written
//! short, Greek `i` stands for ι, η, υ, ει and οι, and Urdu's short vowels
//! go unwritten in its script. So a table gives every way of writing each
//! Latin letter or group, each at a cost, and [`Scheme::candidates`] reads
//! the letters typed every way the table allows, keeping the readings that
//! are words of the language's lexicon, the likeliest first, and the
//! cheapest reading, a word or not, among them, for the names and words a
//! lexicon lacks. The words the likeliest readings begin come in too, at
//! [`COMPLETION_COST`]: `namas` offers नमस्ते before it is typed out.
//!
//! # The format
//!
//! A table is a text file in UTF-8, where `#` starts a comment. First its
//! settings, one per line:
//!
//! - `script abugida`, `script alphabet` or `script abjad`: how the script
//!   writes. Only an abugida's consonants are joined (below).
//! - `virama ्`: an abugida's sign that takes a consonant's vowel away and
//!   joins it to the next; `virama -` for a script that writes consonants
//!   side by side with none (Lao).
//! - `cluster 0` and `schwa 1.5`: an abugida's costs of the two ways of
//!   writing two consonants typed with no vowel between them: joined by the
//!   virama (`namaste`, नमस्ते), or each with its own vowel, the one
//!   between them unwritten in Latin as it is unspoken (`kamla`, कमला).
//! - `final 0` and `final-virama 4`: an abugida's costs of a word's last
//!   consonant written as it is (`kamal`, कमल) and with the virama
//!   (`avan`, அவன்).
//! - `vowels aeiou`: the Latin letters a rule's `>v` means; `>c` means
//!   every other letter.
//! - `unspaced`: the script writes no space between its words (Nuosu's
//!   Yi, as Chinese): Space, and a word taken from the strip, end the word
//!   without one ([`Scheme::unspaced`]).
//! - `tones ່ ້` and `tone 1`: marks an abugida writes on a syllable that
//!   nothing typed says (Lao's tone marks), and what writing one costs
//!   ([`TONE_COST`] without it): each is read after the syllable's
//!   consonant and the signs above or below it, where its vowel is read,
//!   only as far as the lexicon has a word so written (ເກົ້າ from `kao`).
//! - `carrier ອ`: the letter that carries a vowel no consonant does, in
//!   the vowels' letters (Lao's ອ: ອ້າຍ), after which a tone is read too.
//!
//! Then the rules, in the sections `[consonants]`, `[vowels]` and
//! `[other]`. A rule is the Latin it reads, then each way of writing it,
//! separated by spaces:
//!
//! ```text
//! t    त  ट:0.5
//! a    अ/-  आ/ा:1
//! a$   आ/ा  अ/-:0.7
//! n>c  ं:0.8
//! ```
//!
//! A way's cost follows a colon, 0 without one, in nats: a way of cost 1
//! is taken to be e (2.7) times rarer than a way of cost 0 of the same
//! letters. `-` writes nothing. An abugida's vowel is written as
//! `letter/sign`: the letter at a word's start and after a vowel, the sign
//! after a consonant (`-` for the vowel a consonant carries unwritten). A
//! sign with `◌` (U+25CC) in it is written around the consonant: what is
//! before the `◌` before it, what is after it after (Lao's `ເ◌ົາ`: ເກົາ),
//! the consonant being what the last rule of `[consonants]` wrote (so a
//! cluster written after a vowel's first part is one rule: `khw ຂວ`).
//! The Latin may start with `^`, read only at a word's start, and end with
//! `$`, read only at its end, or with `>c` or `>v`, read only before a
//! consonant letter or a vowel letter, which it leaves to the next rule. A
//! rule in lower case reads its letters in either case; one with a capital
//! reads them only as typed (Tamil's `L` is ள, its `l` ல). Of the rules
//! of the same letters and section that read at a place, those asking most
//! of it apply: at a word's end `a$`, not `a`; typed `L`, the rule `L`,
//! not `l`. Rules alike in that add their ways together.
//!
//! The section a rule is in matters to an abugida only: its consonants
//! are joined as `cluster` and `schwa` say, its vowels take their sign
//! after a consonant, and `[other]` (a nasal sign, a mark) follows a
//! consonant as it is.
//!
//! A last section, `[letters]`, may name the Latin letters as the script
//! writes them, a letter and its name a line (`v वी`), for the
//! abbreviations people type letter by letter (`tv`: टीवी). Two to five
//! letters typed are also read as their names run together, at the
//! setting `acronym`'s cost (1 without it), where that is a word of the
//! lexicon; and where it is none, the letters hold no vowel and their
//! cheapest reading is no word either, in that reading's place.
//!
//! Two consonants are also joined with the virama and a zero-width
//! non-joiner, and a consonant after a sign (a nasal sign, a chillu) with
//! a non-joiner, at a cost, where the lexicon has a word written so: the
//! joiner keeps apart letters the script would join, as some words are
//! written (ಗುಜರಾತ್‌ನ, മുൻ‌നിര).
//!
//! A table may be given English words besides ([`Scheme::with_english`],
//! `translit/<lang>.english.txt`): each English word a line, then the
//! words of the language that write it (`computer कंप्यूटर`), as the
//! language writes the English words it has taken, which people type as
//! English spells them, not as they sound. Typed whole, such a word is
//! read as each of them too, at the setting `english`'s cost
//! ([`ENGLISH_COST`] without it), and ranked with the other readings; one
//! whose first letters are such a word reads the rest on from each word
//! that writes it, as the language adds to it (collegenalli: ಕಾಲೇಜಿನಲ್ಲಿ),
//! and from three letters the words of the English words begun are
//! offered as completions.
//!
//! A tool fitting a table's costs to romanizations people typed reads
//! every cost each reading was charged ([`Scheme::readings`]) and tries
//! others ([`Scheme::costs`], [`Scheme::set_costs`]).

use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::ops::Bound;

use unicode_normalization::UnicodeNormalization;

/// What each rule read costs on top of its way's own, so that of two
/// readings otherwise as cheap, the one of fewer, longer rules wins: `kh`
/// read as one letter (ख) rather than two (क्ह).
const STEP: f32 = 0.05;

/// What a character no rule reads costs: it is written as typed, so that
/// every input has a reading.
const UNREAD: f32 = 8.0;

/// How many readings of a word's first letters are carried on to the next
/// letter, the cheapest first.
const BEAM: usize = 256;

/// The longest input read every way; a longer one gets its cheapest
/// reading alone.
const MAX_INPUT: usize = 40;

/// What offering a word the letters typed only begin costs, beyond the
/// reading of those letters: it is taken to be this many nats less likely
/// to be meant than as common a word they spell out. Offered from the
/// first letter, such words save 20% of Hindi's keystrokes on Dakshina's
/// romanized test sentences, 19% of Urdu's, 17% of Tamil's
/// (`eval-translit --keystrokes`), the reading Space takes unchanged;
/// between 0.5 and 4 nats, savings move by under a point.
pub const COMPLETION_COST: f32 = 2.0;

/// How many of the cheapest readings of the letters typed are completed,
/// and with how many words each.
const COMPLETED: usize = 4;
const COMPLETIONS: usize = 3;

/// How far below its rarest word's likelihood a lexicon puts a reading
/// that is no word ([`Lexicon::unknown_log_prob`]), so that a word read at
/// some cost can still come before it. Six puts the word meant first most
/// often in Dakshina's Hindi romanizations (`eval-translit`): at two,
/// rare words lost to cheaper readings that were none.
pub const UNKNOWN_BELOW_RAREST: f32 = 6.0;

/// How much a word's likelihood after the words before it
/// ([`Lexicon::context_log_prob`]) counts beyond its likelihood alone, in
/// the readings' ranking: the homophones of Hindi and Urdu (है and हैं,
/// کی and کے) are told apart by the word before them.
pub const CONTEXT_WEIGHT: f32 = 0.8;

/// How many things a [`Ranker`] reads of a reading ([`Ranker::features`]).
pub const RANK_FEATURES: usize = 7;

/// How a table's readings are ranked when it has learned to (its
/// `[ranker]` section, `make fit-ranker`): a score of what each reading
/// is, the likeliest first. Of each reading it reads, in order: the word's
/// likelihood (or the unknown one, [`Lexicon::unknown_log_prob`]), its
/// cost, what the words before it add to its likelihood
/// ([`Lexicon::context_log_prob`]), whether it is a word, whether the
/// letters only begin it, and its length and the letters typed's, in tens
/// of characters. The score is a weighed sum of them (`linear`), and of
/// a few units that each weigh them and bend the sum (`tanh`), so that,
/// say, a word's likelihood can count for more when the letters typed are
/// many. A table without it ranks by [`Ranker::standard`]'s sum: the
/// likelihood less the cost, and [`CONTEXT_WEIGHT`] times the context.
#[derive(Debug, Clone, PartialEq)]
pub struct Ranker {
    /// The weight of each feature, summed.
    pub linear: [f32; RANK_FEATURES],
    /// The units, each `out * tanh(bias + weights · features)`.
    pub units: Vec<RankUnit>,
}

/// One unit of a [`Ranker`].
#[derive(Debug, Clone, PartialEq)]
pub struct RankUnit {
    /// Its bias.
    pub bias: f32,
    /// What it weighs each feature by.
    pub weights: [f32; RANK_FEATURES],
    /// What it adds to the score, times its value.
    pub out: f32,
}

impl Ranker {
    /// The ranking of a table that has not learned one: the likelihood
    /// less the cost, and [`CONTEXT_WEIGHT`] times what the words before
    /// add.
    #[must_use]
    pub fn standard() -> Self {
        Self {
            linear: [1.0, -1.0, CONTEXT_WEIGHT, 0.0, 0.0, 0.0, 0.0],
            units: Vec::new(),
        }
    }

    /// What the ranker reads of `c`: `unknown` the likelihood of a reading
    /// that is no word, `context` what the words before add to its
    /// likelihood (0 where none is known), `typed` the letters typed.
    #[must_use]
    pub fn features(
        c: &Candidate,
        unknown: f32,
        context: f32,
        typed: usize,
    ) -> [f32; RANK_FEATURES] {
        #[allow(clippy::cast_precision_loss)]
        let tens = |n: usize| n as f32 / 10.0;
        [
            c.log_prob.unwrap_or(unknown),
            c.cost,
            context,
            if c.log_prob.is_some() { 1.0 } else { 0.0 },
            if c.completes { 1.0 } else { 0.0 },
            tens(c.text.chars().count()),
            tens(typed),
        ]
    }

    /// The score of a reading of `features`.
    #[must_use]
    pub fn score(&self, features: &[f32; RANK_FEATURES]) -> f32 {
        let dot =
            |w: &[f32; RANK_FEATURES]| -> f32 { w.iter().zip(features).map(|(w, x)| w * x).sum() };
        dot(&self.linear)
            + self
                .units
                .iter()
                .map(|u| u.out * (u.bias + dot(&u.weights)).tanh())
                .sum::<f32>()
    }

    /// The `[ranker]` section's lines: `linear` and its weights, and a
    /// `unit` line for each unit, its bias, weights and out.
    #[must_use]
    pub fn to_table(&self) -> String {
        use std::fmt::Write as _;
        let nums = |ns: &[f32]| {
            ns.iter()
                .map(|n| format!("{n:.4}"))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let mut out = format!("linear {}\n", nums(&self.linear));
        for u in &self.units {
            let _ = writeln!(out, "unit {:.4} {} {:.4}", u.bias, nums(&u.weights), u.out);
        }
        out
    }
}

/// How likely `word` is after `before` (a natural log), from a language's
/// n-grams ([`crate::ngram::NgramModel`], which keep each pair's share of
/// the text) and how likely each word is alone: the pair's share less the
/// word before's. A pair the n-grams do not keep was rarer than the rarest
/// they do (their unknown), and is taken to be no likelier than that, nor
/// than `word` alone: after a common word, a common word that never
/// follows it there is told so.
#[must_use]
pub fn log_prob_after(
    model: &crate::ngram::NgramModel,
    before: &str,
    word: &str,
    before_alone: f32,
    alone: f32,
) -> f32 {
    let pair = model.score(word, &[before]);
    if pair.total_cmp(&model.unk_log_prob).is_eq() {
        (model.unk_log_prob - before_alone).min(alone)
    } else {
        pair - before_alone
    }
}

/// The words a table's readings are checked against: a language's
/// lexicon.
pub trait Lexicon {
    /// Whether some word starts with `prefix`.
    fn has_prefix(&self, prefix: &str) -> bool;

    /// How likely `word` is (a natural log of its share of the words of
    /// running text), when it is a word.
    fn log_prob(&self, word: &str) -> Option<f32>;

    /// The likelihood a reading that is no word is given, as
    /// [`Lexicon::log_prob`] gives a word's: [`UNKNOWN_BELOW_RAREST`]
    /// below the rarest word's.
    fn unknown_log_prob(&self) -> f32;

    /// The likeliest words longer than `prefix` that start with it, at most
    /// `limit`, likeliest first, with their likelihoods: what a word typed
    /// so far may become. None, unless the lexicon gives them.
    fn completions(&self, prefix: &str, limit: usize) -> Vec<(String, f32)> {
        let _ = (prefix, limit);
        Vec::new()
    }

    /// How likely `word` is after the words written before it (a natural
    /// log), where the lexicon knows: none, unless it was given those
    /// words and holds the pair. A reading's word gains (or loses)
    /// [`CONTEXT_WEIGHT`] times what that is above its likelihood alone.
    fn context_log_prob(&self, word: &str) -> Option<f32> {
        let _ = word;
        None
    }
}

/// How a script writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Script {
    /// Consonants carry a vowel, which signs change; a virama takes it
    /// away (Devanagari, Bengali, Tamil).
    Abugida,
    /// Every sound its letter (Greek, Armenian).
    Alphabet,
    /// Consonants, and the long vowels (Arabic, Urdu, Persian).
    Abjad,
}

/// What a rule reads: which matters to an abugida's joining.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Class {
    /// Nothing yet: the word's start.
    Start,
    Consonant,
    Vowel,
    Other,
}

/// What a rule needs after its letters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ahead {
    Consonant,
    Vowel,
}

/// One way of writing a rule's Latin.
#[derive(Debug, Clone, PartialEq)]
struct Way {
    /// What it writes, or for an abugida's vowel the letter.
    letter: String,
    /// An abugida's vowel's sign, written after a consonant.
    sign: Option<String>,
    cost: f32,
    /// Its place in [`Scheme::costs`].
    index: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct Rule {
    latin: Vec<char>,
    /// In lower case: reads its letters in either case.
    folds: bool,
    class: Class,
    start: bool,
    end: bool,
    ahead: Option<Ahead>,
    ways: Vec<Way>,
    /// The rules of the same letters (in lower case) and kind share it:
    /// of those that read at a place, the most specific apply.
    group: usize,
    /// How much a rule asks of where it reads: one for each of `^`, `$`
    /// or `>`, and a capital.
    specific: u8,
}

/// A transliteration table: how a language writes the Latin letters typed
/// for it.
#[derive(Debug, Clone)]
pub struct Scheme {
    script: Script,
    virama: String,
    cluster: f32,
    schwa: f32,
    final_bare: f32,
    final_virama: f32,
    vowels: Vec<char>,
    rules: Vec<Rule>,
    /// The rules by their first letter, in lower case.
    by_first: HashMap<char, Vec<usize>>,
    /// The characters some rule reads, in lower case.
    reads: HashSet<char>,
    /// Whether the script has capitals (Greek, Armenian), which a capital
    /// typed first then gives the word.
    cased: bool,
    /// Whether the script writes no space between its words (`unspaced`).
    unspaced: bool,
    /// How many ways the rules give, all told: where the settings' places
    /// in [`Scheme::costs`] start.
    ways: u32,
    /// The Latin letters' names (`[letters]`), each by its letter in
    /// lower case, and what reading letters as them costs.
    letters: HashMap<char, String>,
    acronym: f32,
    /// The marks written on a syllable that nothing typed says (`tones`),
    /// what each costs, and the letter that carries a vowel alone.
    tones: Vec<String>,
    tone: f32,
    carrier: Option<String>,
    /// What the vowels' signs with `◌` write before their consonant, each
    /// once: a reading that ends on a consonant may yet take one there.
    before_onset: Vec<String>,
    /// How its readings are ranked, when it has learned to (`[ranker]`).
    ranker: Option<Ranker>,
    /// The English words it was given ([`Scheme::with_english`]), each
    /// by its letters in lower case, and what reading them so costs.
    english: BTreeMap<String, Vec<String>>,
    english_cost: f32,
}

/// The part of a table a line is in.
#[derive(Clone, Copy)]
enum Part {
    Settings,
    Rules(Class),
    Letters,
    Ranker,
}

/// The settings' places in [`Scheme::costs`], after the ways'.
const CLUSTER: u32 = 0;
const SCHWA: u32 = 1;
const FINAL: u32 = 2;
const FINAL_VIRAMA: u32 = 3;
const ACRONYM: u32 = 4;
const TONE: u32 = 5;
const SETTINGS: usize = 6;

/// What reading letters typed as the Latin letters' names costs, where
/// the table does not say (`acronym`).
pub const ACRONYM_COST: f32 = 1.0;

/// What writing a tone mark nothing typed costs, where the table does not
/// say (`tone`).
pub const TONE_COST: f32 = 1.0;

/// Where a vowel's sign puts the consonant it is written around.
const AROUND: char = '\u{25CC}';

/// What joining two consonants with a zero-width non-joiner costs beyond
/// joining them without one: the joiner keeps apart two letters the script
/// would write as one, after a virama or a sign (ಗುಜರಾತ್‌ನ, മുൻ‌നിര), as
/// the words of some languages are written. Only a word of the lexicon is
/// read so.
const ZWNJ_JOIN: f32 = 1.0;
const ZWNJ: char = '\u{200C}';

/// What reading an English word typed as the language writes it costs,
/// where the table does not say (`english`): it is taken to be this many
/// nats less likely than as common a word the letters spell out, for an
/// English word people type is often also a word of the language's own
/// spelled so (men: में, not मेन). Four puts the word meant first most
/// often on the dev halves of Dakshina's twelve languages' romanized
/// sentences, 1.1 points more than without them (`eval-translit`); between
/// three and five they move by under 0.1.
pub const ENGLISH_COST: f32 = 4.0;

/// The fewest letters typed whose English words' words are offered as
/// completions ([`Scheme::with_english`]), and how many English words the
/// letters begin are looked at.
const ENGLISH_PREFIX: usize = 3;

/// What reading the letters after an English word on from the word that
/// writes it costs, beyond the English word's own cost: it is taken to be
/// this many nats less likely than the English word typed alone.
const STEM_COST: f32 = 1.0;

/// What each word that writes an English word after its first costs
/// beyond the first, the likeliest of them written first: the second is
/// offered on the strip without taking the first's place from a word of
/// the language's own.
const LATER_ENGLISH: f32 = 2.0;
const ENGLISH_SCANNED: usize = 64;

/// The most letters read as an abbreviation's.
const MAX_ACRONYM: usize = 5;

/// One reading of the letters typed.
#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    /// The word, in the language's script.
    pub text: String,
    /// What reading it so cost, in nats.
    pub cost: f32,
    /// Its likelihood as a word, when the lexicon knows it.
    pub log_prob: Option<f32>,
    /// A word the letters typed only begin.
    pub completes: bool,
}

impl Candidate {
    /// How likely the reading is: the word's likelihood (or `unknown`'s
    /// when it is no word) less what reading it so cost.
    #[must_use]
    pub fn score(&self, unknown: f32) -> f32 {
        self.log_prob.unwrap_or(unknown) - self.cost
    }
}

/// A reading of the letters typed and the ways it was read
/// ([`Scheme::readings`]): what a tool fitting a table's costs weighs.
#[derive(Debug, Clone, PartialEq)]
pub struct Reading {
    /// The word, in the language's script.
    pub text: String,
    /// What reading it cost: its cheapest way's.
    pub cost: f32,
    /// Its likelihood as a word, when the lexicon knows it.
    pub log_prob: Option<f32>,
    /// The cheapest ways it was read, at most [`PATHS`], the cheapest
    /// first: other costs would read it another way.
    pub paths: Vec<Path>,
}

/// How many of the cheapest ways a reading was read [`Scheme::readings`]
/// gives.
pub const PATHS: usize = 8;

/// One way a reading was read ([`Reading::paths`]).
#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    /// What reading it this way cost, in nats.
    pub cost: f32,
    /// Each cost it was charged, by its place in [`Scheme::costs`], as
    /// often as it was. The rest of `cost` no table sets: a small step
    /// for each rule read, and a letter no rule reads written as typed.
    pub charges: Vec<u32>,
}

/// Why a table could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslitError {
    /// A line the format does not allow, and why.
    Line {
        /// 1-based.
        line: usize,
        /// What is wrong with it.
        why: String,
    },
    /// Something the table as a whole lacks.
    Table(String),
}

impl fmt::Display for TranslitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Line { line, why } => write!(f, "line {line}: {why}"),
            Self::Table(why) => f.write_str(why),
        }
    }
}

impl std::error::Error for TranslitError {}

/// A reading part way through the letters typed, and what it keeps of how
/// it was read.
#[derive(Debug, Clone)]
struct State<T = ()> {
    text: String,
    prev: Class,
    /// Where in `text` what the last consonant rule wrote begins: what a
    /// vowel's sign with `◌` is written around.
    onset: usize,
    cost: f32,
    trace: T,
}

/// What tells readings apart for the search: their text, the kind of
/// the rule that read last, and where its consonant begins.
type Key = (String, Class, usize);

/// What a reading keeps of how it was read: nothing, as the keyboard
/// reads (`()`), or its cheapest ways ([`Paths`], for
/// [`Scheme::readings`]).
trait Trace: Clone + Default {
    /// This trace, then the cost at `index` charged.
    #[must_use]
    fn and(&self, index: u32) -> Self;

    /// Whether, of two readings that cost the same, this one's is kept:
    /// the same one every time.
    fn before(&self, other: &Self) -> bool;

    /// The reading kept, at `cost`, and another of the same text, at
    /// `other_cost`, as one: the cheaper's cost, and what the trace keeps
    /// of both.
    fn merge(cost: &mut f32, kept: &mut Self, other_cost: f32, other: Self);
}

impl Trace for () {
    fn and(&self, _: u32) -> Self {}

    fn before(&self, (): &Self) -> bool {
        false
    }

    fn merge(cost: &mut f32, (): &mut Self, other_cost: f32, (): Self) {
        *cost = cost.min(other_cost);
    }
}

/// A reading's cheapest ways, at most [`PATHS`]: each what it costs above
/// the cheapest and its charges, the cheapest first.
#[derive(Debug, Clone)]
struct Paths(Vec<(f32, Vec<u32>)>);

impl Default for Paths {
    fn default() -> Self {
        Self(vec![(0.0, Vec::new())])
    }
}

impl Trace for Paths {
    fn and(&self, index: u32) -> Self {
        Self(
            self.0
                .iter()
                .map(|(above, charges)| {
                    let mut charged = Vec::with_capacity(charges.len() + 1);
                    charged.extend_from_slice(charges);
                    charged.push(index);
                    (*above, charged)
                })
                .collect(),
        )
    }

    fn before(&self, other: &Self) -> bool {
        self.0.first().map(|p| &p.1) < other.0.first().map(|p| &p.1)
    }

    fn merge(cost: &mut f32, kept: &mut Self, other_cost: f32, other: Self) {
        let mut all: Vec<(f32, Vec<u32>)> = kept
            .0
            .drain(..)
            .map(|(above, c)| (*cost + above, c))
            .chain(
                other
                    .0
                    .into_iter()
                    .map(|(above, c)| (other_cost + above, c)),
            )
            .collect();
        all.sort_by(|a, b| a.0.total_cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        // A way read twice (the same charges in another order) is one.
        let mut seen: Vec<Vec<u32>> = Vec::with_capacity(all.len());
        all.retain(|(_, c)| {
            let mut sorted = c.clone();
            sorted.sort_unstable();
            let new = !seen.contains(&sorted);
            if new {
                seen.push(sorted);
            }
            new
        });
        all.truncate(PATHS);
        let cheapest = all.first().map_or(*cost, |p| p.0);
        *cost = cheapest;
        kept.0 = all.into_iter().map(|(c, ch)| (c - cheapest, ch)).collect();
    }
}

/// Keep, of the reading of `key` kept and this one, what the trace keeps.
fn keep<K: Hash + Eq, T: Trace>(kept: &mut HashMap<K, (f32, T)>, key: K, cost: f32, trace: T) {
    match kept.entry(key) {
        Entry::Occupied(mut o) => {
            let (was, kept_trace) = o.get_mut();
            T::merge(was, kept_trace, cost, trace);
        }
        Entry::Vacant(v) => {
            v.insert((cost, trace));
        }
    }
}

impl Scheme {
    /// Read a table (`translit/<lang>.txt`).
    ///
    /// # Errors
    ///
    /// A line the format does not allow (an unknown setting or section, a
    /// rule with no way of writing it, a cost that is no number), an
    /// abugida without its virama, or a table without a rule.
    #[allow(clippy::too_many_lines)] // one pass over the lines, a branch per setting
    pub fn parse(text: &str) -> Result<Self, TranslitError> {
        let mut script = None;
        let mut virama = None;
        let mut cluster = 0.0;
        let mut schwa = 2.0;
        let mut final_bare = 0.0;
        let mut final_virama = 3.0;
        let mut acronym = ACRONYM_COST;
        let mut tone = TONE_COST;
        let mut tones: Vec<String> = Vec::new();
        let mut carrier: Option<String> = None;
        let mut english_cost = ENGLISH_COST;
        let mut vowels: Vec<char> = "aeiou".chars().collect();
        let mut unspaced = false;
        let mut part = Part::Settings;
        let mut rules: Vec<Rule> = Vec::new();
        let mut letters: HashMap<char, String> = HashMap::new();
        let mut ranker: Option<Ranker> = None;
        for (i, raw) in text.trim_start_matches('\u{feff}').lines().enumerate() {
            let line_no = i + 1;
            let err = |why: String| TranslitError::Line { line: line_no, why };
            let line = raw.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
                part = match name.trim() {
                    "consonants" => Part::Rules(Class::Consonant),
                    "vowels" => Part::Rules(Class::Vowel),
                    "other" => Part::Rules(Class::Other),
                    "letters" => Part::Letters,
                    "ranker" => Part::Ranker,
                    other => return Err(err(format!("no section [{other}]"))),
                };
                continue;
            }
            let mut fields = line.split_whitespace();
            let head = fields.next().unwrap_or_default();
            let rest: Vec<&str> = fields.collect();
            if let Part::Ranker = part {
                let nums: Vec<f32> = rest
                    .iter()
                    .map(|v| v.parse::<f32>().ok().filter(|n| n.is_finite()))
                    .collect::<Option<_>>()
                    .ok_or_else(|| err(format!("`{head}`: a weight that is no number")))?;
                let r = ranker.get_or_insert_with(|| Ranker {
                    linear: [0.0; RANK_FEATURES],
                    units: Vec::new(),
                });
                match head {
                    "linear" if nums.len() == RANK_FEATURES => {
                        r.linear.copy_from_slice(&nums);
                    }
                    "unit" if nums.len() == RANK_FEATURES + 2 => {
                        let mut weights = [0.0; RANK_FEATURES];
                        weights.copy_from_slice(&nums[1..=RANK_FEATURES]);
                        r.units.push(RankUnit {
                            bias: nums[0],
                            weights,
                            out: nums[RANK_FEATURES + 1],
                        });
                    }
                    _ => {
                        return Err(err(format!(
                            "`{head}`: a ranker's lines are `linear` and {RANK_FEATURES} weights, \
                             or `unit`, a bias, {RANK_FEATURES} weights and an out"
                        )));
                    }
                }
                continue;
            }
            if let Part::Letters = part {
                let mut chars = head.chars();
                let letter = match (chars.next(), chars.next()) {
                    (Some(c), None) if c.is_ascii_lowercase() => c,
                    _ => return Err(err(format!("`{head}` is no Latin letter in lower case"))),
                };
                let [name] = rest[..] else {
                    return Err(err(format!("`{head}` takes one name")));
                };
                if letters.insert(letter, name.nfc().collect()).is_some() {
                    return Err(err(format!("`{head}` is named twice")));
                }
                continue;
            }
            let Part::Rules(class) = part else {
                let one = |rest: &[&str]| -> Result<String, TranslitError> {
                    match rest {
                        [v] => Ok((*v).to_string()),
                        _ => Err(err(format!("`{head}` takes one value"))),
                    }
                };
                let cost = |rest: &[&str]| -> Result<f32, TranslitError> {
                    let v = one(rest)?;
                    v.parse::<f32>()
                        .ok()
                        .filter(|c| c.is_finite() && *c >= 0.0)
                        .ok_or_else(|| err(format!("`{head}`: `{v}` is no cost")))
                };
                match head {
                    "script" => {
                        script = Some(match one(&rest)?.as_str() {
                            "abugida" => Script::Abugida,
                            "alphabet" => Script::Alphabet,
                            "abjad" => Script::Abjad,
                            other => return Err(err(format!("no script kind `{other}`"))),
                        });
                    }
                    "virama" => {
                        let v = one(&rest)?;
                        virama = Some(if v == "-" {
                            String::new()
                        } else {
                            v.nfc().collect::<String>()
                        });
                    }
                    "tones" if !rest.is_empty() => {
                        tones = rest.iter().map(|m| m.nfc().collect()).collect();
                    }
                    "tones" => return Err(err("`tones` takes the marks".into())),
                    "tone" => tone = cost(&rest)?,
                    "carrier" => carrier = Some(one(&rest)?.nfc().collect()),
                    "cluster" => cluster = cost(&rest)?,
                    "schwa" => schwa = cost(&rest)?,
                    "final" => final_bare = cost(&rest)?,
                    "final-virama" => final_virama = cost(&rest)?,
                    "acronym" => acronym = cost(&rest)?,
                    "english" => english_cost = cost(&rest)?,
                    "vowels" => {
                        vowels = one(&rest)?.chars().collect();
                    }
                    "unspaced" if rest.is_empty() => unspaced = true,
                    "unspaced" => return Err(err("`unspaced` takes no value".into())),
                    other => return Err(err(format!("no setting `{other}`"))),
                }
                continue;
            };
            let rule =
                parse_rule(head, &rest, class, script == Some(Script::Abugida)).map_err(err)?;
            rules.push(rule);
        }
        let script = script.ok_or_else(|| TranslitError::Table("no `script` line".into()))?;
        let virama = match (script, virama) {
            (Script::Abugida, Some(v)) => v,
            (Script::Abugida, None) => {
                return Err(TranslitError::Table("an abugida needs a `virama`".into()));
            }
            (_, Some(_)) => {
                return Err(TranslitError::Table(
                    "only an abugida has a `virama`".into(),
                ));
            }
            (_, None) => String::new(),
        };
        if rules.is_empty() {
            return Err(TranslitError::Table("no rules".into()));
        }
        let mut groups: HashMap<(Vec<char>, Class), usize> = HashMap::new();
        let mut ways = 0u32;
        for r in &mut rules {
            let key = (r.latin.iter().map(|&c| lower(c)).collect(), r.class);
            let next = groups.len();
            r.group = *groups.entry(key).or_insert(next);
            for w in &mut r.ways {
                w.index = ways;
                ways += 1;
            }
        }
        let mut before_onset: Vec<String> = Vec::new();
        for w in rules.iter().flat_map(|r| &r.ways) {
            if let Some((before, _)) = w.sign.as_deref().and_then(|s| s.split_once(AROUND))
                && !before.is_empty()
                && !before_onset.iter().any(|b| b == before)
            {
                before_onset.push(before.to_string());
            }
        }
        let mut by_first: HashMap<char, Vec<usize>> = HashMap::new();
        let mut reads = HashSet::new();
        let mut cased = false;
        for (i, r) in rules.iter().enumerate() {
            let first = lower(r.latin[0]);
            by_first.entry(first).or_default().push(i);
            reads.extend(r.latin.iter().map(|&c| lower(c)));
            cased |= r.ways.iter().any(|w| {
                w.letter
                    .chars()
                    .any(|c| c.to_uppercase().ne(std::iter::once(c)))
            });
        }
        Ok(Self {
            script,
            virama,
            cluster,
            schwa,
            final_bare,
            final_virama,
            vowels,
            rules,
            by_first,
            reads,
            cased,
            unspaced,
            ways,
            letters,
            acronym,
            tones,
            tone,
            carrier,
            before_onset,
            ranker,
            english: BTreeMap::new(),
            english_cost,
        })
    }

    /// The table, reading the English words of `text` as the language
    /// writes them ([module docs](self)): an English word a line, in lower
    /// case, then the words that write it, separated by spaces; `#` starts
    /// a comment.
    ///
    /// # Errors
    ///
    /// [`TranslitError::Line`] for a line without a word that writes its
    /// English word, or whose English word is not of Latin letters in
    /// lower case (and `'` and `-` within).
    pub fn with_english(mut self, text: &str) -> Result<Self, TranslitError> {
        let mut english: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (i, raw) in text.trim_start_matches('\u{feff}').lines().enumerate() {
            let err = |why: String| TranslitError::Line { line: i + 1, why };
            let line = raw.split('#').next().unwrap_or("").trim();
            let mut fields = line.split_whitespace();
            let Some(word) = fields.next() else { continue };
            let latin = |c: char| c.is_ascii_lowercase() || c == '\'' || c == '-';
            if !word.chars().all(latin) || !word.starts_with(|c: char| c.is_ascii_lowercase()) {
                return Err(err(format!("`{word}` is no English word in lower case")));
            }
            let written: Vec<String> = fields.map(|w| w.nfc().collect()).collect();
            if written.is_empty() {
                return Err(err(format!("`{word}`: no word writes it")));
            }
            english.entry(word.to_string()).or_default().extend(written);
        }
        self.english = english;
        Ok(self)
    }

    /// How many English words it was given ([`Scheme::with_english`]).
    #[must_use]
    pub fn english_words(&self) -> usize {
        self.english.len()
    }

    /// What reading an English word as the language writes it costs
    /// (`english`): to try another.
    pub fn set_english_cost(&mut self, cost: f32) {
        self.english_cost = cost;
    }

    /// How the readings are ranked: the table's learned ranker, or
    /// [`Ranker::standard`].
    #[must_use]
    pub fn ranker(&self) -> Ranker {
        self.ranker.clone().unwrap_or_else(Ranker::standard)
    }

    /// The table, ranking its readings by `ranker`.
    #[must_use]
    pub fn with_ranker(mut self, ranker: Option<Ranker>) -> Self {
        self.ranker = ranker;
        self
    }

    /// The costs the table sets, each at its place: every rule's ways, the
    /// rules in the table's order and a rule's ways in its line's, then
    /// `cluster`, `schwa`, `final` and `final-virama` (which only an
    /// abugida charges), `acronym` (which only a table that names its
    /// letters does) and `tone` (which only one with `tones` does).
    #[must_use]
    pub fn costs(&self) -> Vec<f32> {
        let mut costs: Vec<f32> = self
            .rules
            .iter()
            .flat_map(|r| r.ways.iter().map(|w| w.cost))
            .collect();
        costs.extend([
            self.cluster,
            self.schwa,
            self.final_bare,
            self.final_virama,
            self.acronym,
            self.tone,
        ]);
        costs
    }

    /// Set the costs [`Scheme::costs`] gives, each at its place: for a
    /// tool fitting them.
    ///
    /// # Errors
    ///
    /// When `costs` are not as many as the table sets, or one is no cost
    /// (below 0, or no number).
    pub fn set_costs(&mut self, costs: &[f32]) -> Result<(), TranslitError> {
        let want = self.ways as usize + SETTINGS;
        if costs.len() != want {
            return Err(TranslitError::Table(format!(
                "{} costs for a table that sets {want}",
                costs.len()
            )));
        }
        if let Some(bad) = costs.iter().find(|c| !c.is_finite() || **c < 0.0) {
            return Err(TranslitError::Table(format!("`{bad}` is no cost")));
        }
        for w in self.rules.iter_mut().flat_map(|r| r.ways.iter_mut()) {
            w.cost = costs[w.index as usize];
        }
        let at = |which: u32| costs[(self.ways + which) as usize];
        self.cluster = at(CLUSTER);
        self.schwa = at(SCHWA);
        self.final_bare = at(FINAL);
        self.final_virama = at(FINAL_VIRAMA);
        self.acronym = at(ACRONYM);
        self.tone = at(TONE);
        Ok(())
    }

    /// The reading of `latin` the keyboard offers among its readings that
    /// are words of `lexicon`, a word or not: its cheapest
    /// ([`Scheme::literal`]), or the names of the letters of an
    /// abbreviation in its place (the module's docs say where); and its
    /// way of reading it, for a tool fitting the table's costs. Its
    /// likelihood is not looked up.
    #[must_use]
    pub fn cheapest_reading(&self, latin: &str, lexicon: &dyn Lexicon) -> Reading {
        let (input, capital) = self.prepare(latin);
        let cheapest: State<Paths> = self
            .offered(&input, lexicon)
            .into_iter()
            .next()
            .unwrap_or_else(|| self.cheapest(&input));
        Reading {
            paths: cheapest
                .trace
                .0
                .into_iter()
                .map(|(above, charges)| Path {
                    cost: cheapest.cost + above,
                    charges,
                })
                .collect(),
            text: Self::cap(cheapest.text, capital),
            cost: cheapest.cost,
            log_prob: None,
        }
    }

    /// `latin`'s readings that are words of `lexicon`, and its cheapest
    /// reading of all, each with its cheapest ways of reading it: what
    /// [`Scheme::candidates`] ranks, but for the words they begin,
    /// unranked.
    #[must_use]
    pub fn readings(&self, latin: &str, lexicon: &dyn Lexicon) -> Vec<Reading> {
        let (input, capital) = self.prepare(latin);
        let ends: Vec<State<Paths>> = if input.len() <= MAX_INPUT {
            self.search(&input, lexicon)
        } else {
            Vec::new()
        };
        let paths = |cost: f32, p: Paths| -> Vec<Path> {
            p.0.into_iter()
                .map(|(above, charges)| Path {
                    cost: cost + above,
                    charges,
                })
                .collect()
        };
        let mut readings: Vec<Reading> = self
            .words(&ends, lexicon)
            .into_iter()
            .map(|(c, p)| Reading {
                paths: paths(c.cost, p),
                text: c.text,
                cost: c.cost,
                log_prob: c.log_prob,
            })
            .collect();
        for extra in self.offered::<Paths>(&input, lexicon) {
            if !readings.iter().any(|r| r.text == extra.text) {
                readings.push(Reading {
                    log_prob: lexicon.log_prob(&extra.text),
                    paths: paths(extra.cost, extra.trace),
                    text: extra.text,
                    cost: extra.cost,
                });
            }
        }
        readings.sort_by(|a, b| a.text.cmp(&b.text));
        for r in &mut readings {
            r.text = Self::cap(std::mem::take(&mut r.text), capital);
        }
        readings
    }

    /// How the script writes.
    #[must_use]
    pub fn script(&self) -> Script {
        self.script
    }

    /// Whether the script writes no space between its words (`unspaced`):
    /// a word is ended without one.
    #[must_use]
    pub fn unspaced(&self) -> bool {
        self.unspaced
    }

    /// Whether some rule reads `c` (in either case): a letter that goes on
    /// composing a word, where any other character ends it.
    #[must_use]
    pub fn reads(&self, c: char) -> bool {
        self.reads.contains(&lower(c))
    }

    /// The cheapest reading of `latin`, a word or not: what a word the
    /// lexicon lacks is written as.
    #[must_use]
    pub fn literal(&self, latin: &str) -> String {
        let (input, capital) = self.prepare(latin);
        let cheapest: State = self.cheapest(&input);
        Self::cap(cheapest.text, capital)
    }

    /// The Latin letters that type `word`, a word in the script: of the
    /// rules whose ways write its letters one after another, the letters
    /// that take the fewest taps (`taps`: what typing a letter costs, `None`
    /// for one no key types), then the cheapest ways, checked by reading
    /// them back ([`Scheme::literal`]). What eval-predict types a held-out
    /// word as, to measure a model offered through the table (Vai's).
    /// `None` for a word no ways write so, or whose letters read otherwise;
    /// and for an abugida's table, whose joined consonants and vowel signs
    /// this does not undo.
    #[must_use]
    pub fn spelling(&self, word: &str, taps: impl Fn(char) -> Option<u32>) -> Option<String> {
        if self.script == Script::Abugida || word.is_empty() {
            return None;
        }
        // What typing each rule's letters takes, where keys type them.
        let rule_taps: Vec<Option<u32>> = self
            .rules
            .iter()
            .map(|r| r.latin.iter().map(|&c| taps(c)).sum())
            .collect();
        // At each of the word's character boundaries: the fewest taps, then
        // the cheapest ways, that type the word so far, and their letters.
        let mut best: Vec<Option<(u32, f32, String)>> = vec![None; word.len() + 1];
        best[0] = Some((0, 0.0, String::new()));
        for (at, _) in word.char_indices() {
            let Some((spent, cost, typed)) = best[at].clone() else {
                continue;
            };
            let rest = &word[at..];
            for (rule, rule_taps) in self.rules.iter().zip(&rule_taps) {
                let Some(rule_taps) = *rule_taps else {
                    continue;
                };
                if rule.ahead.is_some() || (rule.start && at > 0) {
                    continue;
                }
                for way in &rule.ways {
                    if way.sign.is_some() || way.letter.is_empty() || !rest.starts_with(&way.letter)
                    {
                        continue;
                    }
                    let to = at + way.letter.len();
                    if rule.end && to < word.len() {
                        continue;
                    }
                    let (taps_to, cost_to) = (spent + rule_taps, cost + way.cost);
                    let fewer = best[to]
                        .as_ref()
                        .is_none_or(|(t, c, _)| taps_to.cmp(t).then(cost_to.total_cmp(c)).is_lt());
                    if fewer {
                        let letters = typed.chars().chain(rule.latin.iter().copied()).collect();
                        best[to] = Some((taps_to, cost_to, letters));
                    }
                }
            }
        }
        let (_, _, typed) = best[word.len()].take()?;
        (self.literal(&typed) == word).then_some(typed)
    }

    /// What `latin` may be, at most `limit`: first its likeliest reading
    /// (what Space takes), of its readings that are words of `lexicon`
    /// and its cheapest reading of all; then the others and the words its
    /// likeliest readings begin (of a table written with no spaces, the
    /// syllables its letters begin too: [`Scheme::syllables_begun`]), the
    /// likeliest first ([`Candidate::score`]).
    #[must_use]
    pub fn candidates(&self, latin: &str, lexicon: &dyn Lexicon, limit: usize) -> Vec<Candidate> {
        let (mut readings, more, capital) = self.gather(latin, lexicon);
        let typed = latin.chars().count();
        let unknown = lexicon.unknown_log_prob();
        let ranker = self.ranker.as_ref();
        let score = |c: &Candidate| -> f32 {
            let context = Self::context(c, lexicon);
            match ranker {
                Some(r) => r.score(&Ranker::features(c, unknown, context, typed)),
                None => c.score(unknown) + CONTEXT_WEIGHT * context,
            }
        };
        let likelier = |a: &(f32, Candidate), b: &(f32, Candidate)| {
            b.0.total_cmp(&a.0).then_with(|| a.1.text.cmp(&b.1.text))
        };
        let mut scored: Vec<(f32, Candidate)> =
            readings.drain(..).map(|c| (score(&c), c)).collect();
        scored.sort_by(likelier);
        // A word the letters only begin never comes first.
        let mut rest = scored.split_off(1.min(scored.len()));
        rest.extend(more.into_iter().map(|c| (score(&c), c)));
        rest.sort_by(likelier);
        scored.extend(rest);
        scored.truncate(limit);
        scored
            .into_iter()
            .map(|(_, mut c)| {
                c.text = Self::cap(std::mem::take(&mut c.text), capital);
                c
            })
            .collect()
    }

    /// The readings of `latin` [`Scheme::candidates`] ranks, unranked:
    /// those it spells out, and the words they begin (each `completes`),
    /// and what [`Ranker::features`] reads of each. For learning a ranker
    /// (`fit-ranker`).
    #[must_use]
    pub fn rankable(
        &self,
        latin: &str,
        lexicon: &dyn Lexicon,
    ) -> Vec<(Candidate, [f32; RANK_FEATURES])> {
        let (readings, more, _) = self.gather(latin, lexicon);
        let typed = latin.chars().count();
        let unknown = lexicon.unknown_log_prob();
        readings
            .into_iter()
            .chain(more)
            .map(|c| {
                let x = Ranker::features(&c, unknown, Self::context(&c, lexicon), typed);
                (c, x)
            })
            .collect()
    }

    /// What the words before a reading add to its likelihood, where the
    /// lexicon knows both.
    fn context(c: &Candidate, lexicon: &dyn Lexicon) -> f32 {
        match (c.log_prob, lexicon.context_log_prob(&c.text)) {
            (Some(alone), Some(after)) => after - alone,
            _ => 0.0,
        }
    }

    /// The readings of `latin` it spells out, the words they begin but for
    /// those spelled out, and whether the word starts with a capital.
    fn gather(&self, latin: &str, lexicon: &dyn Lexicon) -> (Vec<Candidate>, Vec<Candidate>, bool) {
        let (input, capital) = self.prepare(latin);
        let ends: Vec<State> = if input.len() <= MAX_INPUT {
            self.search(&input, lexicon)
        } else {
            Vec::new()
        };
        let mut readings: Vec<Candidate> = self
            .words(&ends, lexicon)
            .into_iter()
            .map(|(c, ())| c)
            .collect();
        for extra in self.offered::<()>(&input, lexicon) {
            if !readings.iter().any(|c| c.text == extra.text) {
                readings.push(Candidate {
                    log_prob: lexicon.log_prob(&extra.text),
                    text: extra.text,
                    cost: extra.cost,
                    completes: false,
                });
            }
        }
        let typed = latin.to_lowercase();
        if let Some(written) = self.english.get(&typed) {
            let mut cost = self.english_cost;
            for text in written {
                if !readings.iter().any(|c| c.text == *text) {
                    readings.push(Candidate {
                        log_prob: lexicon.log_prob(text),
                        text: text.clone(),
                        cost,
                        completes: false,
                    });
                }
                cost += LATER_ENGLISH;
            }
        }
        for c in self.english_stems::<()>(&input, &typed, lexicon) {
            if !readings.iter().any(|r| r.text == c.text) {
                readings.push(c);
            }
        }
        let spelled: Vec<&str> = readings.iter().map(|c| c.text.as_str()).collect();
        let mut more = Self::completions(ends, lexicon, &spelled);
        let english = self.english_completions(&typed, lexicon, &spelled, &more);
        more.extend(english);
        if self.unspaced {
            for c in self.syllables_begun(&input, lexicon) {
                if !spelled.contains(&c.text.as_str()) && !more.iter().any(|m| m.text == c.text) {
                    more.push(c);
                }
            }
        }
        (readings, more, capital)
    }

    /// Of a table whose script writes no space between its words (Nuosu's,
    /// each of whose rules writes a syllable), the syllables of `lexicon`
    /// whose letters the letters typed begin, from the word's start: `n`
    /// may be ꆈ (nuo) or ꆀ (nip), `nuo` ꆇ (nuox) or ꆉ (nuop), each at its way's
    /// cost and [`COMPLETION_COST`], so that the syllable likeliest after
    /// the one before is offered from its first letter, as a word is. A
    /// syllable typed after another is read as the rules read it.
    fn syllables_begun(&self, input: &[char], lexicon: &dyn Lexicon) -> Vec<Candidate> {
        let Some(&first) = input.first() else {
            return Vec::new();
        };
        let mut best: BTreeMap<&str, f32> = BTreeMap::new();
        for &i in self
            .by_first
            .get(&lower(first))
            .map_or(&[][..], Vec::as_slice)
        {
            let r = &self.rules[i];
            let begun = r.ahead.is_none()
                && r.latin.len() > input.len()
                && input
                    .iter()
                    .zip(&r.latin)
                    .all(|(&typed, &c)| typed == c || (r.folds && lower(typed) == c));
            if !begun {
                continue;
            }
            for w in &r.ways {
                if w.sign.is_some() || w.letter.is_empty() {
                    continue;
                }
                let cost = w.cost + STEP + COMPLETION_COST;
                let slot = best.entry(w.letter.as_str()).or_insert(cost);
                *slot = slot.min(cost);
            }
        }
        best.into_iter()
            .filter_map(|(text, cost)| {
                Some(Candidate {
                    log_prob: Some(lexicon.log_prob(text)?),
                    text: text.to_string(),
                    cost,
                    completes: true,
                })
            })
            .collect()
    }

    /// The words of `typed` read as an English word and what the language
    /// adds to it (collegenalli: ಕಾಲೇಜಿನಲ್ಲಿ, germanyile: ജർമനിയിലെ): where
    /// its first letters, from [`ENGLISH_PREFIX`] of them, are an English
    /// word the table was given, the rest read on from each word that
    /// writes it, as the rules read after a consonant or a vowel; those
    /// that are words of the lexicon, at the setting `english`'s cost and
    /// [`STEM_COST`] beyond the rest's.
    fn english_stems<T: Trace>(
        &self,
        input: &[char],
        typed: &str,
        lexicon: &dyn Lexicon,
    ) -> Vec<Candidate> {
        let mut out: Vec<Candidate> = Vec::new();
        if self.english.is_empty() || input.len() > MAX_INPUT {
            return out;
        }
        let chars: Vec<char> = typed.chars().collect();
        for i in ENGLISH_PREFIX..chars.len() {
            let stem: String = chars[..i].iter().collect();
            let Some(written) = self.english.get(&stem) else {
                continue;
            };
            let mut cost = self.english_cost + STEM_COST;
            for w in written {
                let start = State {
                    text: w.clone(),
                    prev: Class::Consonant,
                    onset: w.len() - w.chars().next_back().map_or(0, char::len_utf8),
                    cost,
                    trace: T::default(),
                };
                cost += LATER_ENGLISH;
                let Some(start) = self.stem_state(start) else {
                    continue;
                };
                let ends = self.search_from(input, i, start, lexicon);
                for (c, _) in self.words(&ends, lexicon) {
                    match out.iter_mut().find(|o| o.text == c.text) {
                        Some(o) => o.cost = o.cost.min(c.cost),
                        None => out.push(c),
                    }
                }
            }
        }
        out
    }

    /// How a word that writes an English word ends, for what follows to be
    /// read on from it: a last consonant with the virama as the bare
    /// consonant, a chillu as its consonant, a vowel sign as a vowel; `None`
    /// for what the rules cannot follow.
    fn stem_state<T: Trace>(&self, mut s: State<T>) -> Option<State<T>> {
        if self.script != Script::Abugida {
            return Some(s);
        }
        let last = s.text.chars().next_back()?;
        if !self.virama.is_empty() && s.text.ends_with(self.virama.as_str()) {
            s.text.truncate(s.text.len() - self.virama.len());
            s.prev = Class::Consonant;
        } else if let Some(base) = chillu_consonant(last) {
            s.text.truncate(s.text.len() - last.len_utf8());
            s.text.push(base);
            s.prev = Class::Consonant;
        } else if is_mark(last) {
            s.prev = Class::Vowel;
        } else {
            s.prev = Class::Consonant;
        }
        Some(s)
    }

    /// The words that write the English words `typed` begins, but for
    /// those `spelled` out or offered as completions already: the
    /// likeliest [`COMPLETIONS`] of those of the first [`ENGLISH_SCANNED`]
    /// English words, each at the setting `english`'s cost and
    /// [`COMPLETION_COST`], from [`ENGLISH_PREFIX`] letters typed.
    fn english_completions(
        &self,
        typed: &str,
        lexicon: &dyn Lexicon,
        spelled: &[&str],
        offered: &[Candidate],
    ) -> Vec<Candidate> {
        if typed.chars().count() < ENGLISH_PREFIX {
            return Vec::new();
        }
        let mut found: Vec<(f32, &str)> = Vec::new();
        let begun = self
            .english
            .range::<str, _>((Bound::Excluded(typed), Bound::Unbounded))
            .take_while(|(word, _)| word.starts_with(typed))
            .take(ENGLISH_SCANNED);
        for (_, written) in begun {
            for text in written {
                if spelled.contains(&text.as_str())
                    || offered.iter().any(|c| c.text == *text)
                    || found.iter().any(|(_, t)| t == text)
                {
                    continue;
                }
                if let Some(log_prob) = lexicon.log_prob(text) {
                    found.push((log_prob, text));
                }
            }
        }
        found.sort_by(|a, b| b.0.total_cmp(&a.0).then_with(|| a.1.cmp(b.1)));
        found
            .into_iter()
            .take(COMPLETIONS)
            .map(|(log_prob, text)| Candidate {
                text: text.to_string(),
                cost: self.english_cost + COMPLETION_COST,
                log_prob: Some(log_prob),
                completes: true,
            })
            .collect()
    }

    /// The names of the Latin letters `input` spells, run together, when
    /// the table names them and `input` is one to [`MAX_ACRONYM`] of them:
    /// how the script writes an abbreviation typed letter by letter, or an
    /// initial (k: കെ, as in K. Karunakaran; on Dakshina's Malayalam test
    /// sentences 0.5 points more words first, the others up to 0.2).
    fn acronym(&self, input: &[char]) -> Option<String> {
        if self.letters.is_empty() || !(1..=MAX_ACRONYM).contains(&input.len()) {
            return None;
        }
        input
            .iter()
            .map(|&c| self.letters.get(&lower(c)).map(String::as_str))
            .collect::<Option<Vec<&str>>>()
            .map(|names| names.concat())
    }

    /// What `input` is offered besides the words it reads as: its cheapest
    /// reading of all, a word or not; but where that is no word, `input`
    /// holds no vowel and spells an abbreviation, the names of its letters
    /// in its place, and where those names are a word, they too.
    fn offered<T: Trace>(&self, input: &[char], lexicon: &dyn Lexicon) -> Vec<State<T>> {
        let literal: State<T> = self.cheapest(input);
        let Some(names) = self.acronym(input) else {
            return vec![literal];
        };
        let acronym = State {
            onset: names.len(),
            text: names,
            prev: Class::Other,
            cost: self.acronym,
            trace: T::default().and(self.ways + ACRONYM),
        };
        if lexicon.log_prob(&acronym.text).is_some() {
            vec![literal, acronym]
        } else if lexicon.log_prob(&literal.text).is_none()
            && !input.iter().any(|&c| self.vowels.contains(&lower(c)))
        {
            vec![acronym]
        } else {
            vec![literal]
        }
    }

    /// The words the cheapest readings that end the letters typed begin,
    /// but for those `spelled` out already, each at its reading's cost and
    /// [`COMPLETION_COST`].
    fn completions(
        mut ends: Vec<State>,
        lexicon: &dyn Lexicon,
        spelled: &[&str],
    ) -> Vec<Candidate> {
        ends.sort_by(|a, b| a.cost.total_cmp(&b.cost).then_with(|| a.text.cmp(&b.text)));
        let mut best: HashMap<String, Candidate> = HashMap::new();
        let mut seen: HashSet<&str> = HashSet::new();
        for s in ends
            .iter()
            .filter(|s| seen.insert(s.text.as_str()))
            .take(COMPLETED)
        {
            for (word, log_prob) in lexicon.completions(&s.text, COMPLETIONS) {
                if word.len() <= s.text.len() || spelled.contains(&word.as_str()) {
                    continue;
                }
                let cost = s.cost + COMPLETION_COST;
                let slot = best.entry(word.clone()).or_insert(Candidate {
                    text: word,
                    cost,
                    log_prob: Some(log_prob),
                    completes: true,
                });
                slot.cost = slot.cost.min(cost);
            }
        }
        best.into_values().collect()
    }

    /// The letters typed as the rules read them, and whether the word
    /// starts with a capital the script should write.
    fn prepare(&self, latin: &str) -> (Vec<char>, bool) {
        let input: Vec<char> = latin.chars().collect();
        let capital = self.cased && input.first().is_some_and(|c| c.is_uppercase());
        let input = if self.cased {
            input.into_iter().map(lower).collect()
        } else {
            input
        };
        (input, capital)
    }

    fn cap(text: String, capital: bool) -> String {
        if !capital {
            return text;
        }
        let mut chars = text.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().chain(chars).collect(),
            None => text,
        }
    }

    /// The readings of all of `input` that start a word of `lexicon`, each
    /// at its cheapest, its letters kept only while they start one.
    fn search<T: Trace>(&self, input: &[char], lexicon: &dyn Lexicon) -> Vec<State<T>> {
        let start = State {
            text: String::new(),
            prev: Class::Start,
            onset: 0,
            cost: 0.0,
            trace: T::default(),
        };
        self.search_from(input, 0, start, lexicon)
    }

    /// [`Scheme::search`] from `start`, the reading of `input[..from]`.
    fn search_from<T: Trace>(
        &self,
        input: &[char],
        from: usize,
        start: State<T>,
        lexicon: &dyn Lexicon,
    ) -> Vec<State<T>> {
        let n = input.len();
        let mut at: Vec<HashMap<Key, (f32, T)>> = vec![HashMap::new(); n + 1];
        at[from].insert(
            (start.text, start.prev, start.onset),
            (start.cost, start.trace),
        );
        let state = |((text, prev, onset), (cost, trace)): (Key, (f32, T))| State {
            text,
            prev,
            onset,
            cost,
            trace,
        };
        for pos in from..n {
            let mut states: Vec<State<T>> = std::mem::take(&mut at[pos])
                .into_iter()
                .map(state)
                .collect();
            states.sort_by(|a, b| {
                a.cost
                    .total_cmp(&b.cost)
                    .then_with(|| a.text.cmp(&b.text))
                    .then_with(|| a.prev.cmp(&b.prev))
                    .then_with(|| a.onset.cmp(&b.onset))
            });
            states.truncate(BEAM);
            for s in &states {
                self.expand(input, pos, s, &mut |next, s| {
                    if self.may_begin(&s, lexicon) {
                        keep(&mut at[next], (s.text, s.prev, s.onset), s.cost, s.trace);
                    }
                });
            }
        }
        std::mem::take(&mut at[n]).into_iter().map(state).collect()
    }

    /// Whether reading `s` may yet be read on to a word of `lexicon`: it
    /// begins one, or, ending on a consonant, it does with a vowel's first
    /// part written before that consonant, as a vowel read next may write
    /// it (Lao's ເ: `ke` is ເກ).
    fn may_begin<T>(&self, s: &State<T>, lexicon: &dyn Lexicon) -> bool {
        lexicon.has_prefix(&s.text)
            || (s.prev == Class::Consonant
                && self.before_onset.iter().any(|before| {
                    lexicon.has_prefix(&format!(
                        "{}{before}{}",
                        &s.text[..s.onset],
                        &s.text[s.onset..]
                    ))
                }))
    }

    /// The readings of `ends` (as [`Scheme::search`] leaves them) that are
    /// words of `lexicon`, each at its cheapest.
    fn words<T: Trace>(&self, ends: &[State<T>], lexicon: &dyn Lexicon) -> Vec<(Candidate, T)> {
        let mut best: HashMap<String, (f32, T)> = HashMap::new();
        for s in ends {
            for f in self.finish(s.clone()) {
                keep(&mut best, f.text, f.cost, f.trace);
            }
        }
        best.into_iter()
            .filter_map(|(text, (cost, trace))| {
                let log_prob = lexicon.log_prob(&text)?;
                Some((
                    Candidate {
                        text,
                        cost,
                        log_prob: Some(log_prob),
                        completes: false,
                    },
                    trace,
                ))
            })
            .collect()
    }

    /// The cheapest reading of `input`: what follows a letter depends only
    /// on what kind of rule read it, so the cheapest reading of each kind
    /// up to each letter is all there is to carry on.
    fn cheapest<T: Trace>(&self, input: &[char]) -> State<T> {
        let n = input.len();
        let mut at: Vec<HashMap<Class, State<T>>> = vec![HashMap::new(); n + 1];
        let start = State {
            text: String::new(),
            prev: Class::Start,
            onset: 0,
            cost: 0.0,
            trace: T::default(),
        };
        at[0].insert(Class::Start, start.clone());
        for pos in 0..n {
            let states: Vec<State<T>> = std::mem::take(&mut at[pos]).into_values().collect();
            for s in &states {
                self.expand(
                    input,
                    pos,
                    s,
                    &mut |next, state| match at[next].entry(state.prev) {
                        Entry::Occupied(mut o) => {
                            if cheaper(&state, o.get()) {
                                o.insert(state);
                            }
                        }
                        Entry::Vacant(v) => {
                            v.insert(state);
                        }
                    },
                );
            }
        }
        let mut best: Option<State<T>> = None;
        for s in std::mem::take(&mut at[n]).into_values() {
            for f in self.finish(s) {
                if best.as_ref().is_none_or(|b| cheaper(&f, b)) {
                    best = Some(f);
                }
            }
        }
        best.unwrap_or(start)
    }

    /// Every way the rules read on from `s` at `pos`, each handed to `put`
    /// with the position it reaches; a character no rule reads is written
    /// as it is.
    #[allow(clippy::too_many_lines)] // a branch for each class a rule reads after
    fn expand<T: Trace>(
        &self,
        input: &[char],
        pos: usize,
        s: &State<T>,
        put: &mut dyn FnMut(usize, State<T>),
    ) {
        let first = lower(input[pos]);
        let read: Vec<(&Rule, usize)> = self
            .by_first
            .get(&first)
            .map_or(&[][..], Vec::as_slice)
            .iter()
            .filter_map(|&i| {
                let r = &self.rules[i];
                self.reads_at(r, input, pos).map(|next| (r, next))
            })
            .collect();
        for &(r, next) in &read {
            if read
                .iter()
                .any(|(o, _)| o.group == r.group && o.specific > r.specific)
            {
                continue;
            }
            for w in &r.ways {
                let cost = s.cost + w.cost + STEP;
                let trace = s.trace.and(w.index);
                let abugida = self.script == Script::Abugida;
                match r.class {
                    Class::Consonant if abugida && s.prev == Class::Consonant => {
                        let joined = format!("{}{}", s.text, self.virama);
                        put(
                            next,
                            State {
                                onset: joined.len(),
                                text: format!("{joined}{}", w.letter),
                                prev: Class::Consonant,
                                cost: cost + self.cluster,
                                trace: trace.and(self.ways + CLUSTER),
                            },
                        );
                        let apart = format!("{joined}{ZWNJ}");
                        put(
                            next,
                            State {
                                onset: apart.len(),
                                text: format!("{apart}{}", w.letter),
                                prev: Class::Consonant,
                                cost: cost + self.cluster + ZWNJ_JOIN,
                                trace: trace.and(self.ways + CLUSTER),
                            },
                        );
                        put(
                            next,
                            State {
                                onset: s.text.len(),
                                text: format!("{}{}", s.text, w.letter),
                                prev: Class::Consonant,
                                cost: cost + self.schwa,
                                trace: trace.and(self.ways + SCHWA),
                            },
                        );
                    }
                    Class::Consonant if abugida && s.prev == Class::Other => {
                        put(
                            next,
                            State {
                                onset: s.text.len(),
                                text: format!("{}{}", s.text, w.letter),
                                prev: Class::Consonant,
                                cost,
                                trace: trace.clone(),
                            },
                        );
                        put(
                            next,
                            State {
                                onset: s.text.len() + ZWNJ.len_utf8(),
                                text: format!("{}{ZWNJ}{}", s.text, w.letter),
                                prev: Class::Consonant,
                                cost: cost + ZWNJ_JOIN,
                                trace,
                            },
                        );
                    }
                    Class::Vowel if abugida && s.prev == Class::Consonant => {
                        let sign = w.sign.as_deref().unwrap_or(&w.letter);
                        let (text, mark_at) = around(&s.text, s.onset, sign);
                        self.toned(next, &text, Some(mark_at), s.onset, cost, &trace, put);
                        put(
                            next,
                            State {
                                text,
                                prev: Class::Vowel,
                                onset: s.onset,
                                cost,
                                trace,
                            },
                        );
                    }
                    Class::Vowel if abugida => {
                        let text = format!("{}{}", s.text, w.letter);
                        let mark_at = self.carrier.as_deref().and_then(|c| {
                            let at = s.text.len() + w.letter.find(c)? + c.len();
                            Some(at + marks_after(&text[at..]))
                        });
                        self.toned(next, &text, mark_at, s.text.len(), cost, &trace, put);
                        put(
                            next,
                            State {
                                text,
                                prev: Class::Vowel,
                                onset: s.text.len(),
                                cost,
                                trace,
                            },
                        );
                    }
                    class => put(
                        next,
                        State {
                            onset: s.text.len(),
                            text: format!("{}{}", s.text, w.letter),
                            prev: class,
                            cost,
                            trace,
                        },
                    ),
                }
            }
        }
        if read.is_empty() {
            put(
                pos + 1,
                State {
                    onset: s.text.len(),
                    text: format!("{}{}", s.text, input[pos]),
                    prev: Class::Other,
                    cost: s.cost + UNREAD,
                    trace: s.trace.clone(),
                },
            );
        }
    }

    /// `text` with each of the table's tones written at `mark_at` (where a
    /// vowel was just read), each at the setting `tone`'s cost, handed to
    /// `put` as read up to `next`.
    #[allow(clippy::too_many_arguments)] // a reading's parts, as `expand` has them
    fn toned<T: Trace>(
        &self,
        next: usize,
        text: &str,
        mark_at: Option<usize>,
        onset: usize,
        cost: f32,
        trace: &T,
        put: &mut dyn FnMut(usize, State<T>),
    ) {
        let Some(at) = mark_at else {
            return;
        };
        for mark in &self.tones {
            put(
                next,
                State {
                    text: format!("{}{mark}{}", &text[..at], &text[at..]),
                    prev: Class::Vowel,
                    onset,
                    cost: cost + self.tone,
                    trace: trace.and(self.ways + TONE),
                },
            );
        }
    }

    /// Where rule `r` reading at `pos` ends, when it reads there.
    fn reads_at(&self, r: &Rule, input: &[char], pos: usize) -> Option<usize> {
        let end = pos + r.latin.len();
        if end > input.len() || (r.start && pos != 0) || (r.end && end != input.len()) {
            return None;
        }
        let same = input[pos..end]
            .iter()
            .zip(&r.latin)
            .all(|(&typed, &c)| typed == c || (r.folds && lower(typed) == c));
        if !same {
            return None;
        }
        if let Some(ahead) = r.ahead {
            let next = lower(*input.get(end)?);
            let vowel = self.vowels.contains(&next);
            let fits = match ahead {
                Ahead::Vowel => vowel,
                Ahead::Consonant => !vowel && next.is_alphabetic(),
            };
            if !fits {
                return None;
            }
        }
        Some(end)
    }

    /// A reading at the word's end: an abugida's last consonant as it is,
    /// and with the virama.
    fn finish<T: Trace>(&self, s: State<T>) -> Vec<State<T>> {
        if self.script == Script::Abugida && s.prev == Class::Consonant {
            let with = State {
                text: format!("{}{}", s.text, self.virama),
                prev: s.prev,
                onset: s.onset,
                cost: s.cost + self.final_virama,
                trace: s.trace.and(self.ways + FINAL_VIRAMA),
            };
            let bare = State {
                cost: s.cost + self.final_bare,
                trace: s.trace.and(self.ways + FINAL),
                ..s
            };
            vec![bare, with]
        } else {
            vec![s]
        }
    }
}

/// A rule's line: its Latin (with its `^`, `$`, `>c`, `>v`) and its ways.
fn parse_rule(head: &str, ways: &[&str], class: Class, abugida: bool) -> Result<Rule, String> {
    let mut latin = head;
    let start = latin.starts_with('^') && latin.len() > 1;
    if start {
        latin = &latin[1..];
    }
    let mut ahead = None;
    for (suffix, a) in [(">c", Ahead::Consonant), (">v", Ahead::Vowel)] {
        if let Some(l) = latin.strip_suffix(suffix).filter(|l| !l.is_empty()) {
            latin = l;
            ahead = Some(a);
        }
    }
    let end = latin.ends_with('$') && latin.len() > 1;
    if end {
        if ahead.is_some() {
            return Err(format!("`{head}`: `$` and `>` exclude each other"));
        }
        latin = &latin[..latin.len() - 1];
    }
    if latin.contains(['^', '$', '>']) {
        return Err(format!(
            "`{head}`: `^` starts a rule's letters, `$` or `>c` or `>v` ends them"
        ));
    }
    let latin: Vec<char> = latin.chars().collect();
    if latin.is_empty() {
        return Err("a rule reads at least one letter".into());
    }
    if ways.is_empty() {
        return Err(format!("`{head}` has no way of writing it"));
    }
    let folds = latin.iter().all(|&c| lower(c) == c);
    let specific = u8::from(start) + u8::from(end) + u8::from(ahead.is_some()) + u8::from(!folds);
    let vowel_sign = abugida && class == Class::Vowel;
    let ways = ways
        .iter()
        .map(|w| parse_way(w, vowel_sign).map_err(|why| format!("`{head}`: {why}")))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Rule {
        latin,
        folds,
        class,
        start,
        end,
        ahead,
        ways,
        group: 0,
        specific,
    })
}

/// One way: `text`, `letter/sign` for an abugida's vowel, each with an
/// optional `:cost`.
fn parse_way(way: &str, vowel_sign: bool) -> Result<Way, String> {
    let (body, cost) = match way.rsplit_once(':') {
        Some((body, cost)) if !body.is_empty() => {
            let c = cost
                .parse::<f32>()
                .ok()
                .filter(|c| c.is_finite() && *c >= 0.0)
                .ok_or_else(|| format!("`{cost}` is no cost"))?;
            (body, c)
        }
        _ => (way, 0.0),
    };
    let text = |s: &str| -> String {
        if s == "-" {
            String::new()
        } else {
            s.nfc().collect()
        }
    };
    if vowel_sign {
        let (letter, sign) = body
            .split_once('/')
            .ok_or_else(|| format!("`{body}`: a vowel is written as letter/sign"))?;
        Ok(Way {
            letter: text(letter),
            sign: Some(text(sign)),
            cost,
            index: 0,
        })
    } else {
        Ok(Way {
            letter: text(body),
            sign: None,
            cost,
            index: 0,
        })
    }
}

/// Whether reading `a` beats `b`: the cheaper, or of two that cost the
/// same the earlier text (then trace), so that a table reads the same way
/// every time.
fn cheaper<T: Trace>(a: &State<T>, b: &State<T>) -> bool {
    match a.cost.total_cmp(&b.cost).then_with(|| a.text.cmp(&b.text)) {
        Ordering::Less => true,
        Ordering::Equal => a.trace.before(&b.trace),
        Ordering::Greater => false,
    }
}

/// The consonant a Malayalam chillu writes without its vowel (ൽ: ല), which
/// a vowel or a suffix after it takes back.
fn chillu_consonant(c: char) -> Option<char> {
    match c {
        '\u{0D7A}' => Some('\u{0D23}'),
        '\u{0D7B}' => Some('\u{0D28}'),
        '\u{0D7C}' => Some('\u{0D30}'),
        '\u{0D7D}' => Some('\u{0D32}'),
        '\u{0D7E}' => Some('\u{0D33}'),
        '\u{0D7F}' => Some('\u{0D15}'),
        _ => None,
    }
}

/// Whether `c` is a combining mark (a vowel sign, a nasal sign).
fn is_mark(c: char) -> bool {
    unicode_normalization::char::is_combining_mark(c)
}

/// How many bytes of combining marks `text` starts with.
fn marks_after(text: &str) -> usize {
    text.chars()
        .take_while(|&c| is_mark(c))
        .map(char::len_utf8)
        .sum()
}

/// `text`, its consonant from `onset` on, with a vowel's `sign` written
/// after it, or around it where the sign has `◌` in it; and where in what
/// it gives a tone is written: after the consonant and the marks the sign
/// puts right after it.
fn around(text: &str, onset: usize, sign: &str) -> (String, usize) {
    match sign.split_once(AROUND) {
        Some((before, after)) => {
            let out = format!("{}{before}{}{after}", &text[..onset], &text[onset..]);
            let at = text.len() + before.len();
            (out, at + marks_after(after))
        }
        None => (format!("{text}{sign}"), text.len() + marks_after(sign)),
    }
}

fn lower(c: char) -> char {
    let mut l = c.to_lowercase();
    match (l.next(), l.next()) {
        (Some(one), None) => one,
        _ => c,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    /// A lexicon of a few words, each with its count.
    struct Words(BTreeMap<String, f32>);

    impl Words {
        fn of(words: &[(&str, u32)]) -> Self {
            let total: u32 = words.iter().map(|(_, n)| n).sum();
            #[allow(clippy::cast_precision_loss)]
            Self(
                words
                    .iter()
                    .map(|(w, n)| (w.nfc().collect(), (*n as f32).ln() - (total as f32).ln()))
                    .collect(),
            )
        }
    }

    impl Lexicon for Words {
        fn has_prefix(&self, prefix: &str) -> bool {
            self.0
                .range(prefix.to_string()..)
                .next()
                .is_some_and(|(w, _)| w.starts_with(prefix))
        }
        fn log_prob(&self, word: &str) -> Option<f32> {
            self.0.get(word).copied()
        }
        fn unknown_log_prob(&self) -> f32 {
            -12.0
        }
        fn completions(&self, prefix: &str, limit: usize) -> Vec<(String, f32)> {
            let mut found: Vec<(String, f32)> = self
                .0
                .range(prefix.to_string()..)
                .take_while(|(w, _)| w.starts_with(prefix))
                .filter(|(w, _)| w.len() > prefix.len())
                .map(|(w, lp)| (w.clone(), *lp))
                .collect();
            found.sort_by(|a, b| b.1.total_cmp(&a.1));
            found.truncate(limit);
            found
        }
    }

    const HINDI: &str = "
        # a table of a few letters, as the real one reads them
        script abugida
        virama ्
        schwa 1.5
        final-virama 4

        [consonants]
        k   क
        kh  ख
        t   त  ट:0.5
        m   म
        n   न
        s   स  श:1
        l   ल
        h   ह
        y   य
        r   र

        [vowels]
        a    अ/-  आ/ा:1
        a$   आ/ा  अ/-:0.7
        aa   आ/ा
        e    ए/े
        i    इ/ि  ई/ी:0.7
        ai   ऐ/ै

        [other]
        n>c  ं:0.8
        n$   ं:1
    ";

    fn hindi() -> Scheme {
        Scheme::parse(HINDI).expect("table")
    }

    #[test]
    fn consonants_join_without_a_vowel_between_them() {
        let s = hindi();
        assert_eq!(s.literal("namaste"), "नमस्ते");
        assert_eq!(s.literal("kya"), "क्या");
        // A word's last consonant keeps its vowel unwritten.
        assert_eq!(s.literal("kamal"), "कमल");
    }

    #[test]
    fn a_vowel_is_a_letter_first_and_a_sign_after_a_consonant() {
        let s = hindi();
        assert_eq!(s.literal("ek"), "एक");
        assert_eq!(s.literal("kaa"), "का");
        assert_eq!(s.literal("aa"), "आ");
        // Two letters read as one rule where they can.
        assert_eq!(s.literal("khaa"), "खा");
    }

    #[test]
    fn anchored_rules_read_only_where_they_may() {
        let s = hindi();
        // `a$`, not `a`, reads a word's last a: long.
        assert_eq!(s.literal("kya"), "क्या");
        assert_eq!(s.literal("hamla"), "हम्ला");
        assert_eq!(s.literal("hamal"), "हमल");
        // `n$` writes the nasal sign at the end, at a cost; न costs
        // nothing.
        assert_eq!(s.literal("main"), "मैन");
    }

    #[test]
    fn the_lexicon_chooses_among_the_readings() {
        let s = hindi();
        let words = Words::of(&[("कमला", 50), ("नमस्ते", 100), ("मैं", 400), ("हिंदी", 30)]);
        let top = |latin: &str| {
            s.candidates(latin, &words, 3)
                .first()
                .map(|c| c.text.clone())
                .unwrap_or_default()
        };
        // The vowel between m and l, unwritten in Latin.
        assert_eq!(top("kamla"), "कमला");
        // The nasal sign, for a word's last n.
        assert_eq!(top("main"), "मैं");
        assert_eq!(top("namaste"), "नमस्ते");
    }

    #[test]
    fn a_ranker_is_read_from_its_section_and_ranks() {
        let words = Words::of(&[("कमला", 50), ("नमस्ते", 100), ("टमाटर", 10)]);
        let plain = hindi();
        // The default ranker, written out, ranks as no ranker does.
        let written = format!("{HINDI}\n[ranker]\n{}", Ranker::standard().to_table());
        let same = Scheme::parse(&written).unwrap();
        assert_eq!(same.ranker(), Ranker::standard());
        for latin in ["kamla", "namas", "tamaatar"] {
            assert_eq!(
                same.candidates(latin, &words, 3),
                plain.candidates(latin, &words, 3)
            );
        }
        // Weights that heed only the cost put the cheapest reading first,
        // the word or not.
        let cheapest =
            format!("{HINDI}\n[ranker]\nlinear 0 -1 0 0 0 0 0\nunit 0 0 0 0 0 0 0 0 0\n");
        let s = Scheme::parse(&cheapest).unwrap();
        assert_eq!(s.ranker().units.len(), 1);
        assert_eq!(s.candidates("tamaatar", &words, 3)[0].text, "तमातर");
        assert_eq!(plain.candidates("tamaatar", &words, 3)[0].text, "टमाटर");
        // A line of the wrong length is refused.
        assert!(Scheme::parse(&format!("{HINDI}\n[ranker]\nlinear 1 -1\n")).is_err());
    }

    #[test]
    fn a_word_written_with_a_non_joiner_is_read_so() {
        // The lexicon writes these two with a zero-width non-joiner where the
        // script would join their letters: after the virama, after a sign.
        let words = Words::of(&[("कम\u{94d}\u{200c}ला", 10), ("हं\u{200c}स", 10)]);
        let s = hindi();
        assert_eq!(
            s.candidates("kamla", &words, 3)[0].text,
            "कम\u{94d}\u{200c}ला"
        );
        assert_eq!(s.candidates("hans", &words, 3)[0].text, "हं\u{200c}स");
        // A word the lexicon writes without one is read without one.
        let plain = Words::of(&[("कमला", 10)]);
        assert_eq!(s.candidates("kamla", &plain, 3)[0].text, "कमला");
        assert!(
            s.candidates("kamla", &plain, 3)
                .iter()
                .all(|c| !c.text.contains('\u{200c}'))
        );
    }

    #[test]
    fn an_english_word_is_read_as_the_language_writes_it() {
        // Read by its letters, "computer" is no word; given the English
        // word, it is कंप्यूटर, which then comes first, typed in any case.
        let words = Words::of(&[("कंप्यूटर", 20), ("नमस्ते", 100)]);
        let plain = hindi();
        assert!(
            plain
                .candidates("computer", &words, 3)
                .iter()
                .all(|c| c.text != "कंप्यूटर")
        );
        let s = hindi()
            .with_english("# a comment\ncomputer कंप्यूटर\n\nnamaste नमस्ते\n")
            .unwrap();
        assert_eq!(s.english_words(), 2);
        assert_eq!(s.candidates("computer", &words, 3)[0].text, "कंप्यूटर");
        assert_eq!(s.candidates("Computer", &words, 3)[0].text, "कंप्यूटर");
        // From three letters, the words of the English words they begin
        // are offered too, after the readings.
        let compu = s.candidates("compu", &words, 4);
        assert!(
            compu.iter().any(|c| c.text == "कंप्यूटर" && c.completes),
            "{compu:?}"
        );
        assert!(!compu[0].completes);
        assert!(
            s.candidates("co", &words, 4)
                .iter()
                .all(|c| c.text != "कंप्यूटर")
        );
        // An English word with what the language adds to it: the rest read
        // on from the word that writes it.
        let words = Words::of(&[("कॉलेजसे", 5), ("कॉलेज", 20)]);
        let s2 = hindi().with_english("college कॉलेज\n").unwrap();
        assert_eq!(s2.candidates("collegese", &words, 3)[0].text, "कॉलेजसे");
        assert!(
            hindi()
                .candidates("collegese", &words, 3)
                .iter()
                .all(|c| c.text != "कॉलेजसे")
        );
        // One the letters read as already is not read twice.
        let namaste = s.candidates("namaste", &words, 3);
        assert_eq!(namaste.iter().filter(|c| c.text == "नमस्ते").count(), 1);
        // A word that is not English, or that no word writes, is refused.
        assert!(hindi().with_english("Computer कंप्यूटर\n").is_err());
        assert!(hindi().with_english("computer\n").is_err());
    }

    #[test]
    fn a_word_the_lexicon_lacks_is_its_cheapest_reading() {
        let s = hindi();
        let words = Words::of(&[("नमस्ते", 100)]);
        let c = s.candidates("kamal", &words, 3);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].text, "कमल");
        assert_eq!(c[0].log_prob, None);
    }

    #[test]
    fn a_word_outranks_a_cheaper_reading_that_is_none() {
        let s = hindi();
        // ट costs 0.5 more than त, and only the word has it.
        let words = Words::of(&[("टमाटर", 10), ("नमस्ते", 100)]);
        let c = s.candidates("tamaatar", &words, 3);
        assert_eq!(c[0].text, "टमाटर");
        assert_eq!(c[1].text, "तमातर");
    }

    #[test]
    fn a_word_the_letters_begin_comes_after_their_reading() {
        let s = hindi();
        let words = Words::of(&[("नमस्ते", 100), ("नमक", 50)]);
        let c = s.candidates("namas", &words, 3);
        // What Space takes is the reading of the letters as typed, though
        // no word; the word they begin is offered after it.
        assert_eq!(c[0].text, "नमस");
        assert!(!c[0].completes);
        assert_eq!(c[1].text, "नमस्ते");
        assert!(c[1].completes);
        // From the first letter.
        assert!(s.candidates("n", &words, 3).iter().any(|c| c.completes));
    }

    #[test]
    fn a_word_spelled_out_is_no_completion_of_itself() {
        let s = hindi();
        let words = Words::of(&[("नमस्ते", 100), ("नमस्तेजी", 1)]);
        let c = s.candidates("namaste", &words, 3);
        assert_eq!(c[0].text, "नमस्ते");
        assert!(!c[0].completes);
        assert_eq!(c.iter().filter(|c| c.text == "नमस्ते").count(), 1);
    }

    #[test]
    fn a_letter_no_rule_reads_is_written_as_typed() {
        let s = hindi();
        assert_eq!(s.literal("kq"), "कq");
        assert!(s.reads('k') && s.reads('K') && !s.reads('q') && !s.reads('1'));
    }

    #[test]
    fn a_final_consonant_takes_the_virama_where_the_table_says() {
        let tamil = Scheme::parse(
            "
            script abugida
            virama ்
            schwa 3
            final 2
            final-virama 0
            [consonants]
            v  வ
            n  ன
            L  ள
            l  ல
            [vowels]
            a  அ/-
            ",
        )
        .expect("table");
        assert_eq!(tamil.literal("avan"), "அவன்");
        // A capital rule reads only its capital.
        assert_eq!(tamil.literal("avaL"), "அவள்");
        assert_eq!(tamil.literal("aval"), "அவல்");
    }

    #[test]
    fn an_alphabet_writes_its_final_forms_and_capitals() {
        let greek = Scheme::parse(
            "
            script alphabet
            [consonants]
            k  κ
            l  λ
            s  σ
            s$ ς
            [vowels]
            a  α  ά:0.3
            o  ο  ό:0.3  ω:0.5
            ",
        )
        .expect("table");
        // Two rules read the final s: σ and ς, at no cost either.
        let words = Words::of(&[("καλός", 10)]);
        let c = greek.candidates("kalos", &words, 3);
        assert_eq!(c[0].text, "καλός");
        let c = greek.candidates("Kalos", &words, 3);
        assert_eq!(c[0].text, "Καλός");
    }

    #[test]
    fn an_abjad_leaves_short_vowels_unwritten() {
        let urdu = Scheme::parse(
            "
            script abjad
            [consonants]
            k  ک
            l  ل
            m  م
            q  ق
            [vowels]
            ^a  ا
            a   -  ا:1
            a$  ا  ہ:0.5
            ",
        )
        .expect("table");
        assert_eq!(urdu.literal("qalam"), "قلم");
        // `^a`, not `a`, reads a word's first a.
        assert_eq!(urdu.literal("am"), "ام");
        assert_eq!(urdu.literal("kala"), "کلا");
    }

    #[test]
    fn bad_tables_are_refused() {
        let bad = |t: &str| Scheme::parse(t).expect_err(t);
        bad("[consonants]\nk क");
        bad("script abugida\n[consonants]\nk क");
        bad("script alphabet\nvirama ्\n[consonants]\nk κ");
        bad("script alphabet\n[letters]\nk κ");
        bad("script alphabet\n[consonants]\nk");
        bad("script alphabet\n[consonants]\nk κ:x");
        bad("script abugida\nvirama ्\n[vowels]\na अ");
        bad("script alphabet\n[other]\nn>c$ ν");
        bad("script alphabet\n[other]\nn$>c ν");
        bad("script alphabet\nfoo 1\n[other]\nn ν");
        bad("script alphabet");
        assert!(matches!(
            bad("script alphabet\n[consonants]\nk κ:-1"),
            TranslitError::Line { line: 3, .. }
        ));
    }

    #[test]
    fn a_reading_says_what_it_was_charged() {
        let s = hindi();
        let lexicon = Words::of(&[("नमस्ते", 10), ("नमस", 1), ("कमल", 5)]);
        let costs = s.costs();
        let ways = costs.len() - SETTINGS;
        let readings = s.readings("namaste", &lexicon);
        // The word, and the cheapest reading, which is the word too.
        assert_eq!(readings.len(), 1);
        let word = &readings[0];
        assert_eq!(word.text, "नमस्ते");
        assert_eq!(word.log_prob, lexicon.log_prob("नमस्ते"));
        // n a m a s t e, a way each, and s and t joined; the one way.
        assert_eq!(word.paths.len(), 1);
        let first = &word.paths[0].charges;
        assert_eq!(first.len(), 8);
        let cluster = u32::try_from(ways).unwrap() + CLUSTER;
        assert_eq!(first.iter().filter(|&&c| c == cluster).count(), 1);
        // A way costs what it was charged and a step for each rule read.
        let kamal = s.readings("kamal", &lexicon);
        for r in kamal.iter().chain(&readings) {
            assert_eq!(r.cost, r.paths[0].cost);
            for p in &r.paths {
                let read = p.charges.iter().filter(|&&c| (c as usize) < ways).count();
                let charged: f32 = p.charges.iter().map(|&c| costs[c as usize]).sum();
                #[allow(clippy::cast_precision_loss)]
                let steps = STEP * read as f32;
                assert!((p.cost - charged - steps).abs() < 1e-4, "{r:?}");
            }
        }
        // A word's last consonant as it is: `final`, charged.
        let word = kamal.iter().find(|r| r.text == "कमल").expect("कमल");
        let bare = u32::try_from(ways).unwrap() + FINAL;
        assert!(word.paths[0].charges.contains(&bare));
    }

    #[test]
    fn the_cheapest_reading_is_the_literal() {
        let s = hindi();
        let lexicon = Words::of(&[("कमल", 1)]);
        for latin in ["tak", "namaste", "kamal", "kya"] {
            let r = s.cheapest_reading(latin, &lexicon);
            assert_eq!(r.text, s.literal(latin));
            assert_eq!(r.paths.len(), 1);
            assert_eq!(r.cost, r.paths[0].cost);
            assert!(r.log_prob.is_none());
        }
    }

    /// Hindi's table above, naming some Latin letters.
    fn hindi_with_letters() -> Scheme {
        Scheme::parse(&format!(
            "{HINDI}\n[letters]\nb  बी\nc  सी\nt  टी\nv  वी\ns  एस\nm  एम\na  ए\n"
        ))
        .expect("table")
    }

    #[test]
    fn letters_typed_are_read_as_an_abbreviation() {
        let s = hindi_with_letters();
        let lexicon = Words::of(&[("टीवी", 50), ("बीबीसी", 10), ("तब", 100)]);
        // Where the names run together are a word, they are offered.
        assert_eq!(s.candidates("tv", &lexicon, 2)[0].text, "टीवी");
        assert_eq!(s.candidates("bbc", &lexicon, 2)[0].text, "बीबीसी");
        // No word, no vowel: the names in the cheapest reading's place.
        let got = s.candidates("sms", &lexicon, 3);
        assert!(got.iter().any(|c| c.text == "एसएमएस"), "{got:?}");
        assert!(!got.iter().any(|c| c.text == s.literal("sms")), "{got:?}");
        // A word read the usual way still comes first, and the names run
        // together take the place of a cheapest reading that is no word.
        let got = s.candidates("tm", &Words::of(&[("तम", 100)]), 3);
        assert_eq!(got[0].text, "तम");
        assert!(got.iter().any(|c| c.text == "टीएम"), "{got:?}");
        // With a vowel typed, names that are no word are not offered, nor
        // more than five letters'; one letter's is, an initial.
        let got = s.candidates("ta", &lexicon, 3);
        assert!(!got.iter().any(|c| c.text == "टीए"), "{got:?}");
        assert!(
            s.candidates("t", &lexicon, 3)
                .iter()
                .any(|c| c.text == "टी")
        );
        assert!(
            !s.candidates("tvtvtv", &lexicon, 3)
                .iter()
                .any(|c| c.text.starts_with("टीवीटीवी"))
        );
        // A table that names no letters reads none so.
        assert!(
            !hindi()
                .candidates("tv", &lexicon, 3)
                .iter()
                .any(|c| c.text == "टीवी")
        );
    }

    #[test]
    fn an_abbreviation_is_charged_its_setting() {
        let s = hindi_with_letters();
        let lexicon = Words::of(&[("टीवी", 50)]);
        let costs = s.costs();
        let acronym = u32::try_from(costs.len() - SETTINGS).unwrap() + ACRONYM;
        let readings = s.readings("tv", &lexicon);
        let word = readings.iter().find(|r| r.text == "टीवी").expect("टीवी");
        assert_eq!(word.paths[0].charges, [acronym]);
        assert_eq!(word.cost, ACRONYM_COST);
        // Where the names take the cheapest reading's place, so for a
        // tool fitting the costs.
        let none = Words::of(&[("तब", 1)]);
        assert_eq!(s.cheapest_reading("sms", &none).text, "एसएमएस");
        assert_eq!(s.cheapest_reading("tak", &none).text, s.literal("tak"));
        // The setting is read, and set.
        let s = Scheme::parse(&format!(
            "{}\n[letters]\nt टी\nv वी\n",
            HINDI.replace("schwa 1.5", "schwa 1.5\n        acronym 0.4")
        ))
        .expect("table");
        assert_eq!(s.costs()[s.costs().len() - 2], 0.4);
    }

    #[test]
    fn bad_letters_are_refused() {
        for bad in ["ab  अब", "B  बी", "b", "b  बी  बि", "b  बी\nb  बि", "1  वन"]
        {
            let table = format!("{HINDI}\n[letters]\n{bad}\n");
            assert!(Scheme::parse(&table).is_err(), "{bad}");
        }
    }

    /// A lexicon of a few words that knows what follows `before`.
    struct After<'a> {
        words: Words,
        follows: &'a [(&'a str, f32)],
    }

    impl Lexicon for After<'_> {
        fn has_prefix(&self, prefix: &str) -> bool {
            self.words.has_prefix(prefix)
        }
        fn log_prob(&self, word: &str) -> Option<f32> {
            self.words.log_prob(word)
        }
        fn unknown_log_prob(&self) -> f32 {
            self.words.unknown_log_prob()
        }
        fn context_log_prob(&self, word: &str) -> Option<f32> {
            self.follows
                .iter()
                .find(|(w, _)| *w == word)
                .map(|(_, lp)| *lp)
        }
    }

    #[test]
    fn the_words_before_choose_among_readings() {
        let s = hindi();
        // तम is the likelier word alone; after these words, टम is.
        let words = || Words::of(&[("तम", 30), ("टम", 10)]);
        assert_eq!(s.candidates("tam", &words(), 2)[0].text, "तम");
        let after = After {
            words: words(),
            follows: &[("टम", -0.5), ("तम", -6.0)],
        };
        assert_eq!(s.candidates("tam", &after, 2)[0].text, "टम");
        // Nothing known of the pair: the order of the words alone.
        let none = After {
            words: words(),
            follows: &[],
        };
        assert_eq!(s.candidates("tam", &none, 2)[0].text, "तम");
    }

    #[test]
    fn a_word_after_another_is_its_pair_s_share_of_that_word() {
        // In 1000 words: वह 400 times, है 50, हैं 20; वह है 300 times, वह
        // हैं not kept, the rarest pair kept 5 times.
        let ln = |n: f32| (n / 1000.0).ln();
        let mut model = crate::ngram::NgramModel::empty();
        model.add_bigram("वह", "है", ln(300.0));
        model.add_bigram("वे", "हैं", ln(5.0));
        model.unk_log_prob = ln(5.0) - 1.0;
        let after = log_prob_after(&model, "वह", "है", ln(400.0), ln(50.0));
        assert!((after - (300.0f32 / 400.0).ln()).abs() < 1e-5);
        // A pair not kept after a common word: rarer than the rarest kept,
        // so less likely than the word alone.
        let after = log_prob_after(&model, "वह", "हैं", ln(400.0), ln(20.0));
        assert!((after - (ln(5.0) - 1.0 - ln(400.0))).abs() < 1e-5);
        assert!(after < ln(20.0));
        // After a rare word, nothing below the word alone is known.
        let after = log_prob_after(&model, "कभी", "हैं", ln(2.0), ln(20.0));
        assert!((after - ln(20.0)).abs() < 1e-5);
    }

    #[test]
    fn a_reading_read_two_ways_keeps_both() {
        // kk as one rule (κ:0.5), and as k twice, one of them written as
        // nothing (-:1), first or second: one way, the same charges.
        let s =
            Scheme::parse("script alphabet\n[consonants]\nk  κ  -:1\nkk  κ:0.5\n").expect("table");
        let lexicon = Words::of(&[("κ", 1)]);
        let readings = s.readings("kk", &lexicon);
        let word = readings.iter().find(|r| r.text == "κ").expect("κ");
        let ways: Vec<(f32, &[u32])> = word
            .paths
            .iter()
            .map(|p| (p.cost, p.charges.as_slice()))
            .collect();
        assert_eq!(ways.len(), 2, "{word:?}");
        assert!((ways[0].0 - 0.55).abs() < 1e-4 && ways[0].1 == [2]);
        assert!((ways[1].0 - 1.1).abs() < 1e-4);
        let mut second = ways[1].1.to_vec();
        second.sort_unstable();
        assert_eq!(second, [0, 1]);
    }

    #[test]
    fn a_syllabary_reads_the_fewest_syllables_and_may_write_no_spaces() {
        let s = Scheme::parse(
            "script alphabet\nunspaced\n[consonants]\nbi  ꀘ\nbit  ꀖ\nta  ꄤ\n[vowels]\na  ꀊ:0.1\n",
        )
        .expect("table");
        assert!(s.unspaced());
        assert_eq!(s.literal("bit"), "ꀖ");
        // A tone letter that could end a syllable or begin the next: the
        // vowel alone costs more, so it begins the next.
        assert_eq!(s.literal("bita"), "ꀘꄤ");
        assert!(!hindi().unspaced());
        assert!(Scheme::parse("script alphabet\nunspaced 1\n[consonants]\nk  κ\n").is_err());
    }

    #[test]
    fn a_syllabary_written_without_spaces_offers_the_syllables_its_letters_begin() {
        let yi = Scheme::parse(
            "script alphabet\nunspaced\n[consonants]\nbi  ꀘ\nbit  ꀖ\nbix  ꀗ\nbo  ꀧ\nta  ꄤ\n\
             [vowels]\na  ꀊ:0.1\n",
        )
        .expect("table");
        let lexicon = Words::of(&[("ꀘ", 1), ("ꀖ", 8), ("ꀗ", 3), ("ꀧ", 2), ("ꄤ", 5)]);
        let texts = |latin: &str| -> Vec<String> {
            yi.candidates(latin, &lexicon, 6)
                .into_iter()
                .filter(|c| c.log_prob.is_some())
                .map(|c| c.text)
                .collect()
        };
        // b: its syllables, the likeliest first; none of t's.
        assert_eq!(texts("b"), ["ꀖ", "ꀗ", "ꀧ", "ꀘ"]);
        // bi is ꀘ, Space's; then the syllables bi begins.
        assert_eq!(texts("bi"), ["ꀘ", "ꀖ", "ꀗ"]);
        assert!(yi.candidates("bi", &lexicon, 6)[1].completes);
        // A syllable typed after another is read as the rules read it.
        assert_eq!(texts("tab"), Vec::<String>::new());
        // A table written with spaces offers no syllable its letters begin.
        let spaced = Scheme::parse(
            "script alphabet\n[consonants]\nbi  ꀘ\nbit  ꀖ\nbix  ꀗ\nbo  ꀧ\nta  ꄤ\n[vowels]\na  ꀊ:0.1\n",
        )
        .expect("table");
        let spaced_texts: Vec<String> = spaced
            .candidates("b", &lexicon, 6)
            .into_iter()
            .filter(|c| c.log_prob.is_some())
            .map(|c| c.text)
            .collect();
        assert_eq!(spaced_texts, Vec::<String>::new());
    }

    /// Taps on a keyboard of small letters with þ and ŋ on long presses,
    /// and no Shift.
    fn small_letters(c: char) -> Option<u32> {
        match c {
            'a'..='z' => Some(1),
            'þ' | 'ŋ' => Some(2),
            _ => None,
        }
    }

    #[test]
    fn a_word_is_spelled_in_the_fewest_taps_that_read_it_back() {
        // Vai's syllables, as its table types them.
        let vai = Scheme::parse(
            "script alphabet\n[consonants]\nka  ꕪ\nTa  ꕛ\nþa  ꕛ\nnga  ꕋ\nŋa  ꕋ\nmbe  ꔈ\n\
             [vowels]\na  ꕉ\ne  ꔀ\n",
        )
        .expect("table");
        // No Shift: THA is þa, not Ta.
        assert_eq!(vai.spelling("ꕛꕉ", small_letters).as_deref(), Some("þaa"));
        assert_eq!(
            vai.spelling("ꕛ", |c| small_letters(c).or(Some(2)))
                .as_deref(),
            Some("Ta"),
            "as few taps, the table's first"
        );
        // As few taps either way: the table's first.
        assert_eq!(vai.spelling("ꕋꔈ", small_letters).as_deref(), Some("ngambe"));
        // A letter no rule writes, and nothing.
        assert_eq!(vai.spelling("ꕪb", small_letters), None);
        assert_eq!(vai.spelling("", small_letters), None);
        // Letters that read otherwise: bit then a is bi then ta.
        let yi = Scheme::parse(
            "script alphabet\nunspaced\n[consonants]\nbi  ꀘ\nbit  ꀖ\nta  ꄤ\n[vowels]\na  ꀊ:0.1\n",
        )
        .expect("table");
        assert_eq!(yi.spelling("ꀘꄤ", small_letters).as_deref(), Some("bita"));
        assert_eq!(yi.spelling("ꀖꀊ", small_letters), None);
        // An abugida's vowel signs and joined consonants are not undone.
        assert_eq!(hindi().spelling("तक", small_letters), None);
    }

    /// Lao's way: a vowel written around its consonant (ເ◌ົາ), a cluster
    /// read as one consonant, no virama, and a tone nothing typed read
    /// where the lexicon has it, after the consonant and the signs above
    /// it, and after the letter that carries a vowel alone.
    #[test]
    fn a_vowel_written_around_its_consonant_and_an_untyped_tone() {
        let lao = Scheme::parse(
            "script abugida\nvirama -\ntones ່ ້\ntone 1.5\ncarrier ອ\n\
             [consonants]\nk  ກ\nn  ນ\nkhw  ຂວ\n\
             [vowels]\na  ອະ/ະ  ອັ/ັ  ອາ/າ:0.5\nao  ເອົາ/ເ◌ົາ\ne  ເອ/ເ◌\ni  ອິ/ິ\nai  ອ້າຍ/◌າຍ\n",
        )
        .expect("table");
        let words = Words::of(&[("ເກົ້າ", 10), ("ກັນເກ", 5), ("ເຂວ", 3), ("ອ້າຍ", 4), ("ກີ", 1)]);
        let first = |latin: &str| {
            lao.candidates(latin, &words, 4)
                .into_iter()
                .next()
                .map(|c| c.text)
        };
        // The vowel's first part before the consonant, the tone after the
        // consonant and the sign above it.
        assert_eq!(first("kao").as_deref(), Some("ເກົ້າ"));
        // After a closed syllable, the vowel goes around the next
        // consonant only.
        assert_eq!(first("kanke").as_deref(), Some("ກັນເກ"));
        // Around a cluster read as one rule.
        assert_eq!(first("khwe").as_deref(), Some("ເຂວ"));
        // A tone the carrier takes, a vowel alone.
        assert_eq!(first("ai").as_deref(), Some("ອ້າຍ"));
        // No tone where no word has one.
        assert_eq!(lao.literal("ki"), "ກິ");
        // The tone's cost is a setting, fitted with the others.
        assert_eq!(lao.costs()[lao.costs().len() - 1], 1.5);
        let reading = lao.readings("kao", &words);
        let toned = reading.iter().find(|r| r.text == "ເກົ້າ").expect("ເກົ້າ");
        assert!(toned.paths[0].charges.contains(&(lao.ways + TONE)));
    }

    #[test]
    fn costs_set_anew_read_anew() {
        let mut s = hindi();
        let mut costs = s.costs();
        // k, kh, then t's two ways, त and ट:0.5.
        assert_eq!(costs[2..4], [0.0, 0.5]);
        // The settings last: cluster, schwa 1.5, final, final-virama 4,
        // acronym, tone.
        assert_eq!(
            costs[costs.len() - SETTINGS..],
            [0.0, 1.5, 0.0, 4.0, ACRONYM_COST, TONE_COST]
        );
        assert_eq!(s.literal("tak"), "तक");
        costs[2] = 1.0;
        costs[3] = 0.0;
        s.set_costs(&costs)
            .expect("as many costs as the table sets");
        assert_eq!(s.literal("tak"), "टक");
        assert_eq!(s.costs(), costs);
        assert!(s.set_costs(&costs[1..]).is_err());
        costs[0] = -1.0;
        assert!(s.set_costs(&costs).is_err());
        costs[0] = f32::NAN;
        assert!(s.set_costs(&costs).is_err());
    }
}
