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
//!   joins it to the next.
//! - `cluster 0` and `schwa 1.5`: an abugida's costs of the two ways of
//!   writing two consonants typed with no vowel between them: joined by the
//!   virama (`namaste`, नमस्ते), or each with its own vowel, the one
//!   between them unwritten in Latin as it is unspoken (`kamla`, कमला).
//! - `final 0` and `final-virama 4`: an abugida's costs of a word's last
//!   consonant written as it is (`kamal`, कमल) and with the virama
//!   (`avan`, அவன்).
//! - `vowels aeiou`: the Latin letters a rule's `>v` means; `>c` means
//!   every other letter.
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
//! after a consonant (`-` for the vowel a consonant carries unwritten).
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
//! A tool fitting a table's costs to romanizations people typed reads
//! every cost each reading was charged ([`Scheme::readings`]) and tries
//! others ([`Scheme::costs`], [`Scheme::set_costs`]).

use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

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
    /// How many ways the rules give, all told: where the settings' places
    /// in [`Scheme::costs`] start.
    ways: u32,
    /// The Latin letters' names (`[letters]`), each by its letter in
    /// lower case, and what reading letters as them costs.
    letters: HashMap<char, String>,
    acronym: f32,
}

/// The part of a table a line is in.
#[derive(Clone, Copy)]
enum Part {
    Settings,
    Rules(Class),
    Letters,
}

/// The settings' places in [`Scheme::costs`], after the ways'.
const CLUSTER: u32 = 0;
const SCHWA: u32 = 1;
const FINAL: u32 = 2;
const FINAL_VIRAMA: u32 = 3;
const ACRONYM: u32 = 4;
const SETTINGS: usize = 5;

/// What reading letters typed as the Latin letters' names costs, where
/// the table does not say (`acronym`).
pub const ACRONYM_COST: f32 = 1.0;

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
    cost: f32,
    trace: T,
}

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
        let mut vowels: Vec<char> = "aeiou".chars().collect();
        let mut part = Part::Settings;
        let mut rules: Vec<Rule> = Vec::new();
        let mut letters: HashMap<char, String> = HashMap::new();
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
                    other => return Err(err(format!("no section [{other}]"))),
                };
                continue;
            }
            let mut fields = line.split_whitespace();
            let head = fields.next().unwrap_or_default();
            let rest: Vec<&str> = fields.collect();
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
                    "virama" => virama = Some(one(&rest)?.nfc().collect::<String>()),
                    "cluster" => cluster = cost(&rest)?,
                    "schwa" => schwa = cost(&rest)?,
                    "final" => final_bare = cost(&rest)?,
                    "final-virama" => final_virama = cost(&rest)?,
                    "acronym" => acronym = cost(&rest)?,
                    "vowels" => {
                        vowels = one(&rest)?.chars().collect();
                    }
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
            ways,
            letters,
            acronym,
        })
    }

    /// The costs the table sets, each at its place: every rule's ways, the
    /// rules in the table's order and a rule's ways in its line's, then
    /// `cluster`, `schwa`, `final` and `final-virama` (which only an
    /// abugida charges) and `acronym` (which only a table that names its
    /// letters does).
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

    /// What `latin` may be, at most `limit`: first its likeliest reading
    /// (what Space takes), of its readings that are words of `lexicon`
    /// and its cheapest reading of all; then the others and the words its
    /// likeliest readings begin, the likeliest first
    /// ([`Candidate::score`]).
    #[must_use]
    pub fn candidates(&self, latin: &str, lexicon: &dyn Lexicon, limit: usize) -> Vec<Candidate> {
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
        let unknown = lexicon.unknown_log_prob();
        // A word the words before it make likelier, or less likely.
        let here = |c: &Candidate| -> f32 {
            match (c.log_prob, lexicon.context_log_prob(&c.text)) {
                (Some(alone), Some(after)) => CONTEXT_WEIGHT * (after - alone),
                _ => 0.0,
            }
        };
        let likelier = |a: &Candidate, b: &Candidate| {
            (b.score(unknown) + here(b))
                .total_cmp(&(a.score(unknown) + here(a)))
                .then_with(|| a.text.cmp(&b.text))
        };
        readings.sort_by(likelier);
        let mut rest = readings.split_off(1.min(readings.len()));
        let spelled: Vec<&str> = readings
            .iter()
            .chain(&rest)
            .map(|c| c.text.as_str())
            .collect();
        let more = Self::completions(ends, lexicon, &spelled);
        rest.extend(more);
        rest.sort_by(likelier);
        readings.extend(rest);
        readings.truncate(limit);
        for c in &mut readings {
            c.text = Self::cap(std::mem::take(&mut c.text), capital);
        }
        readings
    }

    /// The names of the Latin letters `input` spells, run together, when
    /// the table names them and `input` is two to [`MAX_ACRONYM`] of them:
    /// how the script writes an abbreviation typed letter by letter.
    fn acronym(&self, input: &[char]) -> Option<String> {
        if self.letters.is_empty() || !(2..=MAX_ACRONYM).contains(&input.len()) {
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
        let n = input.len();
        let mut at: Vec<HashMap<(String, Class), (f32, T)>> = vec![HashMap::new(); n + 1];
        at[0].insert((String::new(), Class::Start), (0.0, T::default()));
        let state = |((text, prev), (cost, trace)): ((String, Class), (f32, T))| State {
            text,
            prev,
            cost,
            trace,
        };
        for pos in 0..n {
            let mut states: Vec<State<T>> = std::mem::take(&mut at[pos])
                .into_iter()
                .map(state)
                .collect();
            states.sort_by(|a, b| {
                a.cost
                    .total_cmp(&b.cost)
                    .then_with(|| a.text.cmp(&b.text))
                    .then_with(|| a.prev.cmp(&b.prev))
            });
            states.truncate(BEAM);
            for s in &states {
                self.expand(input, pos, s, &mut |next, s| {
                    if lexicon.has_prefix(&s.text) {
                        keep(&mut at[next], (s.text, s.prev), s.cost, s.trace);
                    }
                });
            }
        }
        std::mem::take(&mut at[n]).into_iter().map(state).collect()
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
                        put(
                            next,
                            State {
                                text: format!("{}{}{}", s.text, self.virama, w.letter),
                                prev: Class::Consonant,
                                cost: cost + self.cluster,
                                trace: trace.and(self.ways + CLUSTER),
                            },
                        );
                        put(
                            next,
                            State {
                                text: format!("{}{}", s.text, w.letter),
                                prev: Class::Consonant,
                                cost: cost + self.schwa,
                                trace: trace.and(self.ways + SCHWA),
                            },
                        );
                    }
                    Class::Vowel if abugida && s.prev == Class::Consonant => {
                        let sign = w.sign.as_deref().unwrap_or(&w.letter);
                        put(
                            next,
                            State {
                                text: format!("{}{sign}", s.text),
                                prev: Class::Vowel,
                                cost,
                                trace,
                            },
                        );
                    }
                    class => put(
                        next,
                        State {
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
                    text: format!("{}{}", s.text, input[pos]),
                    prev: Class::Other,
                    cost: s.cost + UNREAD,
                    trace: s.trace.clone(),
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
        // With a vowel typed, names that are no word are not offered; nor
        // are one letter's, or more than five.
        let got = s.candidates("ta", &lexicon, 3);
        assert!(!got.iter().any(|c| c.text == "टीए"), "{got:?}");
        assert!(
            !s.candidates("t", &lexicon, 3)
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
        assert_eq!(s.costs()[s.costs().len() - 1], 0.4);
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
    fn costs_set_anew_read_anew() {
        let mut s = hindi();
        let mut costs = s.costs();
        // k, kh, then t's two ways, त and ट:0.5.
        assert_eq!(costs[2..4], [0.0, 0.5]);
        // The settings last: cluster, schwa 1.5, final, final-virama 4,
        // acronym.
        assert_eq!(
            costs[costs.len() - SETTINGS..],
            [0.0, 1.5, 0.0, 4.0, ACRONYM_COST]
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
