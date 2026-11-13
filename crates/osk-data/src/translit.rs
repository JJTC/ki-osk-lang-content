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

use std::collections::{HashMap, HashSet};
use std::fmt;

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
/// romanized sentences, 18% of Urdu's, 16% of Tamil's (`eval-translit
/// --keystrokes`), the reading Space takes unchanged; between 0.5 and 4
/// nats, savings move by under a point.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}

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

/// A reading part way through the letters typed.
#[derive(Debug, Clone)]
struct State {
    text: String,
    prev: Class,
    cost: f32,
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
        let mut vowels: Vec<char> = "aeiou".chars().collect();
        let mut section: Option<Class> = None;
        let mut rules: Vec<Rule> = Vec::new();
        for (i, raw) in text.trim_start_matches('\u{feff}').lines().enumerate() {
            let line_no = i + 1;
            let err = |why: String| TranslitError::Line { line: line_no, why };
            let line = raw.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
                section = Some(match name.trim() {
                    "consonants" => Class::Consonant,
                    "vowels" => Class::Vowel,
                    "other" => Class::Other,
                    other => return Err(err(format!("no section [{other}]"))),
                });
                continue;
            }
            let mut fields = line.split_whitespace();
            let head = fields.next().unwrap_or_default();
            let rest: Vec<&str> = fields.collect();
            let Some(class) = section else {
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
        for r in &mut rules {
            let key = (r.latin.iter().map(|&c| lower(c)).collect(), r.class);
            let next = groups.len();
            r.group = *groups.entry(key).or_insert(next);
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
        })
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
        let text = self.cheapest(&input).text;
        Self::cap(text, capital)
    }

    /// What `latin` may be, at most `limit`: first its likeliest reading
    /// (what Space takes), of its readings that are words of `lexicon`
    /// and its cheapest reading of all; then the others and the words its
    /// likeliest readings begin, the likeliest first
    /// ([`Candidate::score`]).
    #[must_use]
    pub fn candidates(&self, latin: &str, lexicon: &dyn Lexicon, limit: usize) -> Vec<Candidate> {
        let (input, capital) = self.prepare(latin);
        let ends = if input.len() <= MAX_INPUT {
            self.search(&input, lexicon)
        } else {
            Vec::new()
        };
        let mut readings = self.words(&ends, lexicon);
        let literal = self.cheapest(&input);
        if !readings.iter().any(|c| c.text == literal.text) {
            readings.push(Candidate {
                log_prob: lexicon.log_prob(&literal.text),
                text: literal.text,
                cost: literal.cost,
                completes: false,
            });
        }
        let unknown = lexicon.unknown_log_prob();
        let likelier = |a: &Candidate, b: &Candidate| {
            b.score(unknown)
                .total_cmp(&a.score(unknown))
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
    fn search(&self, input: &[char], lexicon: &dyn Lexicon) -> Vec<State> {
        let n = input.len();
        let mut at: Vec<HashMap<(String, Class), f32>> = vec![HashMap::new(); n + 1];
        at[0].insert((String::new(), Class::Start), 0.0);
        for pos in 0..n {
            let mut states: Vec<State> = std::mem::take(&mut at[pos])
                .into_iter()
                .map(|((text, prev), cost)| State { text, prev, cost })
                .collect();
            states.sort_by(|a, b| a.cost.total_cmp(&b.cost));
            states.truncate(BEAM);
            for s in &states {
                self.expand(input, pos, s, &mut |next, state| {
                    if !lexicon.has_prefix(&state.text) {
                        return;
                    }
                    let slot = at[next]
                        .entry((state.text, state.prev))
                        .or_insert(f32::INFINITY);
                    *slot = slot.min(state.cost);
                });
            }
        }
        std::mem::take(&mut at[n])
            .into_iter()
            .map(|((text, prev), cost)| State { text, prev, cost })
            .collect()
    }

    /// The readings of `ends` (as [`Scheme::search`] leaves them) that are
    /// words of `lexicon`, each at its cheapest.
    fn words(&self, ends: &[State], lexicon: &dyn Lexicon) -> Vec<Candidate> {
        let mut best: HashMap<String, f32> = HashMap::new();
        for s in ends {
            for (word, cost) in self.finish(s.clone()) {
                let slot = best.entry(word).or_insert(f32::INFINITY);
                *slot = slot.min(cost);
            }
        }
        best.into_iter()
            .filter_map(|(text, cost)| {
                let log_prob = lexicon.log_prob(&text)?;
                Some(Candidate {
                    text,
                    cost,
                    log_prob: Some(log_prob),
                    completes: false,
                })
            })
            .collect()
    }

    /// The cheapest reading of `input`: what follows a letter depends only
    /// on what kind of rule read it, so the cheapest reading of each kind
    /// up to each letter is all there is to carry on.
    fn cheapest(&self, input: &[char]) -> State {
        let n = input.len();
        let mut at: Vec<HashMap<Class, State>> = vec![HashMap::new(); n + 1];
        at[0].insert(
            Class::Start,
            State {
                text: String::new(),
                prev: Class::Start,
                cost: 0.0,
            },
        );
        for pos in 0..n {
            let states: Vec<State> = std::mem::take(&mut at[pos]).into_values().collect();
            for s in &states {
                self.expand(input, pos, s, &mut |next, state| {
                    let slot = at[next].entry(state.prev);
                    match slot {
                        std::collections::hash_map::Entry::Occupied(mut o) => {
                            if cheaper(&state.text, state.cost, &o.get().text, o.get().cost) {
                                o.insert(state);
                            }
                        }
                        std::collections::hash_map::Entry::Vacant(v) => {
                            v.insert(state);
                        }
                    }
                });
            }
        }
        let mut best = State {
            text: String::new(),
            prev: Class::Start,
            cost: f32::INFINITY,
        };
        for s in std::mem::take(&mut at[n]).into_values() {
            for (text, cost) in self.finish(s) {
                if cheaper(&text, cost, &best.text, best.cost) {
                    best = State {
                        text,
                        prev: Class::Other,
                        cost,
                    };
                }
            }
        }
        if best.cost.is_infinite() {
            best.cost = 0.0;
        }
        best
    }

    /// Every way the rules read on from `s` at `pos`, each handed to `put`
    /// with the position it reaches; a character no rule reads is written
    /// as it is.
    fn expand(&self, input: &[char], pos: usize, s: &State, put: &mut dyn FnMut(usize, State)) {
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
                let abugida = self.script == Script::Abugida;
                match r.class {
                    Class::Consonant if abugida && s.prev == Class::Consonant => {
                        put(
                            next,
                            State {
                                text: format!("{}{}{}", s.text, self.virama, w.letter),
                                prev: Class::Consonant,
                                cost: cost + self.cluster,
                            },
                        );
                        put(
                            next,
                            State {
                                text: format!("{}{}", s.text, w.letter),
                                prev: Class::Consonant,
                                cost: cost + self.schwa,
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
                            },
                        );
                    }
                    class => put(
                        next,
                        State {
                            text: format!("{}{}", s.text, w.letter),
                            prev: class,
                            cost,
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
    fn finish(&self, s: State) -> Vec<(String, f32)> {
        if self.script == Script::Abugida && s.prev == Class::Consonant {
            let with = format!("{}{}", s.text, self.virama);
            vec![
                (s.text, s.cost + self.final_bare),
                (with, s.cost + self.final_virama),
            ]
        } else {
            vec![(s.text, s.cost)]
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
        })
    } else {
        Ok(Way {
            letter: text(body),
            sign: None,
            cost,
        })
    }
}

/// Whether a reading of `cost` beats one of `other_cost`, the earlier
/// text first where they cost the same, so that a table reads the same
/// way every time.
fn cheaper(text: &str, cost: f32, other: &str, other_cost: f32) -> bool {
    cost.total_cmp(&other_cost).then_with(|| text.cmp(other)) == std::cmp::Ordering::Less
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
}
