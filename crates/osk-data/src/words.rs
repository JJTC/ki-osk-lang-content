//! How prediction splits text into words.
//!
//! One splitter for the three places that must agree on what a word is:
//! the trainer (`build-predict` counts the words it finds in a corpus),
//! the keyboard (ki-osk reads the text before the caret through
//! [`Splitter::typing`]) and the measure (`eval-predict` types held-out
//! text word by word). A word the trainer kept whole and the keyboard cut,
//! or the reverse, is a word the keyboard never offers.
//!
//! A word is a run of word characters inside one whitespace-separated
//! token: letters, and the marks, joiners, apostrophes and hyphens that
//! sit inside words. Everything else (punctuation, digits, symbols)
//! separates words, and a token with no word in it breaks the chain of
//! context, as does the end of a sentence or a line.
//!
//! - Letters are Unicode's Alphabetic characters, which include Indic vowel
//!   signs and U+02BC MODIFIER LETTER APOSTROPHE.
//! - A combining mark after a word character belongs to the word: viramas
//!   and nuktas (`क्या`, `ज़्यादा`), stress accents (`бо́льшую`).
//! - ZWNJ and ZWJ between word characters belong to it: the Persian
//!   half-space (`می‌شوند`), Indic conjunct forms (`वाहणार्‍या`).
//! - An apostrophe between letters belongs to it (`don't`, `п'ять`,
//!   `c'hoazh`, `Tom'un`), and so does one that ends a word (`students'`,
//!   Greek `απ'`, Italian `po'`) unless a single quote is open on the line;
//!   a hyphen between letters does too (`что-то`, `anak-anak`, `est-ce`).
//!   So a word is offered and learned whole. French, Italian, Catalan and
//!   Occitan instead split after an elided word: `l'homme` is `l'` and
//!   `homme`, the second glued to the first, with no space between; and
//!   Kabyle after a hyphen, before its clitics (`fell-` and `i`).
//! - Per language: Catalan's middle dot (`col·legi`), Armenian's question,
//!   emphasis and exclamation marks, written on a word's stressed vowel
//!   (`Ի՞նչ`), Hebrew's gershayim in acronyms (`לחו"ל`), a leading
//!   apostrophe in Breton (`'zo`) and in Dutch's `'s`, `'t`, `'n`, `'k` and
//!   `'m`.
//! - Thai is written without spaces between words: a run of Thai letters is
//!   cut into the words of the language's word list
//!   ([`crate::wordbreak`], given with [`Splitter::with_word_list`]), each
//!   after the first glued to the one before, as `homme` is to `l'`; so a
//!   keyboard types no space after a Thai word. Without a list, a run is
//!   one word.
//!
//! A word's identity, what lexicons and n-gram tables are keyed on, is
//! [`Splitter::normalize`]: NFC, lower case, one apostrophe (U+0027), one
//! hyphen (U+002D), and per language one spelling of a letter the texts
//! and the keyboards write two ways.

use std::ops::Range;
use std::sync::Arc;

use unicode_normalization::UnicodeNormalization;
use unicode_normalization::char::is_combining_mark;

use crate::wordbreak::{Script, WordList};

/// The splitter's rules, numbered. What a lexicon's words are depends on
/// them, so a change to them is a new generation, and every model the old
/// one built is out of date: the data repository's lock records it.
/// Generation 2 gave a sentence's first word [`SENTENCE_START`] for
/// context, so the models count what starts sentences. Cutting Thai with
/// its word list came after, with Thai's first model, and changed no other
/// language's words; the list is among that model's inputs.
pub const GENERATION: u32 = 2;

/// The context of a sentence's first word, as if a word: the n-gram tables
/// count the words that start sentences after it, and the keyboard offers
/// them where a sentence starts. No lexicon holds it.
pub const SENTENCE_START: &str = "<s>";

/// How one language joins characters into words, and ends sentences.
#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct Splitter {
    elide: Elide,
    lead: Lead,
    fold: Fold,
    /// A hyphen between letters ends a word, which keeps it, and glues the
    /// next one on (Kabyle `fell-i`).
    hyphen_glues: bool,
    /// U+00B7 between letters (Catalan `l·l`).
    middle_dot: bool,
    /// Armenian: marks written on a word's vowel, and `:` written for the
    /// full stop `։`.
    armenian: bool,
    /// Hebrew gershayim between letters.
    hebrew: bool,
    /// `;` is a question mark (Greek), so it ends a sentence.
    greek: bool,
    /// Dotted and dotless i (Turkish, Azerbaijani): `I` lower-cases to
    /// `ı`, `İ` to `i`, and `i` upper-cases to `İ`.
    turkic: bool,
    /// The script the language writes without spaces between words (Thai).
    script: Option<Script>,
    /// The words its runs are cut into.
    word_list: Option<Arc<WordList>>,
}

/// Where an apostrophe inside a word ends it.
#[derive(Debug, Clone, Copy)]
enum Elide {
    /// Never: the apostrophe is part of the word.
    Never,
    /// After one of these words (lower case, without the apostrophe).
    After(&'static [&'static str]),
    /// After any word but these, whose apostrophe ends them (Italian `po'`).
    AllBut(&'static [&'static str]),
}

/// Whether a leading apostrophe before a letter begins a word.
#[derive(Debug, Clone, Copy)]
enum Lead {
    /// Never: it is a quote.
    Never,
    /// When the letters after it are one of these words.
    Words(&'static [&'static str]),
    /// Always.
    Always,
}

/// One language's spellings of one letter two ways, made one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fold {
    None,
    /// The texts write Latin æ, the keyboard types Cyrillic ӕ.
    Ossetian,
    /// Arabic ي ى ك for Persian ی ک; the short-vowel marks and the tatweel
    /// dropped.
    Persian,
    /// Arabic and Uyghur: the short-vowel marks and the tatweel dropped.
    Arabic,
    /// The question, emphasis and exclamation marks dropped, `և` as `եվ`.
    Armenian,
    /// Greek γ ε for Latin ɣ ɛ.
    Kabyle,
    /// The eyelash ra and the candra a as the keyboard types them.
    Marathi,
    /// Latin as Cyrillic: the texts write both, the keyboard types Cyrillic.
    Serbian,
}

/// The elided words French writes with an apostrophe before a vowel.
/// Others stay whole: `aujourd'hui`, `quelqu'un`, `presqu'île`.
const FRENCH_ELISIONS: &[&str] = &[
    "l", "d", "j", "m", "t", "s", "n", "c", "ç", "qu", "lorsqu", "puisqu", "jusqu", "quoiqu",
];
/// Catalan's proclitic elisions; enclitics stay whole (`veure'l`).
const CATALAN_ELISIONS: &[&str] = &["l", "d", "m", "t", "s", "n"];
/// Occitan's.
const OCCITAN_ELISIONS: &[&str] = &["l", "d", "qu", "m", "t", "s", "n"];
/// Italian's apocopes: words cut short, written with an apostrophe and a
/// space after (`un po' di`), not elided.
const ITALIAN_APOCOPES: &[&str] = &[
    "po", "di", "fa", "va", "sta", "da", "be", "mo", "to", "ve", "ca", "pie",
];
/// Dutch's clipped words (`'s avonds`, `'t is`, `'n huis`).
const DUTCH_CLIPPED: &[&str] = &["s", "t", "n", "k", "m"];

/// Apostrophes: ASCII, the typographic right single quote, and the Hebrew
/// geresh. U+02BC is a letter already.
fn is_apostrophe(c: char) -> bool {
    matches!(c, '\'' | '\u{2019}' | '\u{05F3}')
}

/// Characters that open or close a single quote when they belong to no
/// word.
fn is_single_quote(c: char) -> bool {
    matches!(c, '\'' | '\u{2018}' | '\u{2019}' | '\u{201A}' | '\u{201B}')
}

/// Hyphens: ASCII, U+2010, the non-breaking U+2011, the Armenian hyphen and
/// the Hebrew maqaf.
fn is_hyphen(c: char) -> bool {
    matches!(c, '-' | '\u{2010}' | '\u{2011}' | '\u{058A}' | '\u{05BE}')
}

fn is_joiner(c: char) -> bool {
    matches!(c, '\u{200C}' | '\u{200D}')
}

/// The Arabic short-vowel marks (fathatan to sukun), the superscript alef
/// and the tatweel.
fn is_harakah(c: char) -> bool {
    matches!(c, '\u{064B}'..='\u{0652}' | '\u{0670}' | '\u{0640}')
}

/// Whether `c` ends a sentence for prediction: `.`, `!` and `?`, the
/// Devanagari and Bengali danda and double danda, the Armenian full stop,
/// the Arabic question mark and full stop, the Greek question mark, and a
/// new line or a tab, which leave the line or the field. Context does not
/// carry across one.
#[must_use]
pub fn ends_sentence(c: char) -> bool {
    matches!(
        c,
        '.' | '!'
            | '?'
            | '\u{964}'
            | '\u{965}'
            | '\u{589}'
            | '\u{61F}'
            | '\u{6D4}'
            | '\u{37E}'
            | '\n'
            | '\t'
    )
}

/// Whether `c`, right after the caret, may carry on the word before it: a
/// letter, a mark, a joiner, an apostrophe or a hyphen. A word that goes on
/// past the caret is none to complete.
#[must_use]
pub fn continues_word(c: char) -> bool {
    c.is_alphabetic() || is_combining_mark(c) || is_joiner(c) || is_apostrophe(c) || is_hyphen(c)
}

/// A word in a token: its bytes in the token, and whether it is glued to
/// the word before it (`homme` after `l'`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    /// Its bytes in the token.
    pub range: Range<usize>,
    /// It follows an elided word with no space between.
    pub glued: bool,
}

/// How a word follows the words before it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Start {
    /// It follows them: they are its context.
    Follows,
    /// It starts a sentence: at the text's start, after a sentence end or a
    /// line end.
    Sentence,
    /// It follows a token with no word in it (a number, a dash): the
    /// context breaks, though the sentence goes on.
    Break,
}

/// A word of a text, as prediction reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextWord {
    /// Its bytes in the text.
    pub range: Range<usize>,
    /// Its identity: [`Splitter::normalize`]d.
    pub text: String,
    /// It follows an elided word with no space between (`homme` in
    /// `l'homme`).
    pub glued: bool,
    /// Other characters come before it in its token (`(see`, `«Oui`,
    /// `a,b`): no word starts there by itself, so none is offered before
    /// its first letter.
    pub after_other: bool,
    /// How it follows the words before it.
    pub start: Start,
    /// It is its token's last word: whitespace or the text's end follows,
    /// perhaps after other characters (`word,`).
    pub last_in_token: bool,
}

/// Where the caret is, for prediction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Caret {
    /// After whitespace, at the start, or right after an elided word
    /// (`l'`): a word starts here, and the strip may offer the next one.
    #[default]
    WordStart,
    /// In a token, before its first letter (after an opening quote or a
    /// digit): a letter starts a word, but no next word is offered, since
    /// a tap would glue it on.
    BeforeWord,
    /// In a word: the word in progress.
    InWord,
    /// Past the token's last word (`end.`, `word,`), or in a word that
    /// goes on after the caret: no word until whitespace.
    AfterWord,
}

/// What prediction reads from the text before the caret.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Typing {
    /// The last words before the word at the caret, in its sentence,
    /// oldest first, normalised.
    pub context: Vec<String>,
    /// The word in progress, as typed: the word the caret is in, with a
    /// character that joins words once a letter follows (`don'`, `что-`).
    pub word: String,
    /// Where the caret is.
    pub caret: Caret,
}

/// The words a context keeps, at most `n`, the newest last.
struct Context {
    words: Vec<String>,
    n: usize,
}

impl Context {
    /// Start again where a word starts as `start` says: after
    /// [`SENTENCE_START`] at a sentence's start, from nothing after a
    /// break; as it was when the word follows.
    fn restart(&mut self, start: Option<Start>) {
        match start {
            Some(Start::Sentence) => {
                self.words.clear();
                self.words.push(SENTENCE_START.to_string());
            }
            Some(Start::Break) => self.words.clear(),
            Some(Start::Follows) | None => {}
        }
    }

    fn push(&mut self, w: String) {
        self.words.push(w);
        if self.words.len() > self.n {
            self.words.remove(0);
        }
    }
}

/// What reading a text leaves for what comes after it.
#[derive(Clone, Copy)]
struct After {
    /// How the next word follows.
    start: Start,
    /// A single quote is open on the line.
    quote: bool,
}

impl Splitter {
    /// The splitter for `lang`, a BCP-47 tag (`fr`, `pt-BR`); unknown
    /// languages get the defaults.
    #[must_use]
    pub fn new(lang: &str) -> Self {
        let primary = lang.split(['-', '_']).next().unwrap_or(lang);
        let mut s = Self {
            elide: Elide::Never,
            lead: Lead::Never,
            fold: Fold::None,
            hyphen_glues: false,
            middle_dot: false,
            armenian: false,
            hebrew: false,
            greek: false,
            turkic: false,
            script: Script::of_lang(primary),
            word_list: None,
        };
        match primary {
            "fr" => s.elide = Elide::After(FRENCH_ELISIONS),
            "it" => s.elide = Elide::AllBut(ITALIAN_APOCOPES),
            "ca" => {
                s.elide = Elide::After(CATALAN_ELISIONS);
                s.middle_dot = true;
            }
            "oc" => s.elide = Elide::After(OCCITAN_ELISIONS),
            "br" => s.lead = Lead::Always,
            "nl" => s.lead = Lead::Words(DUTCH_CLIPPED),
            "hy" => {
                s.armenian = true;
                s.fold = Fold::Armenian;
            }
            "he" => s.hebrew = true,
            "el" => s.greek = true,
            "tr" | "az" => s.turkic = true,
            "kab" => {
                s.hyphen_glues = true;
                s.fold = Fold::Kabyle;
            }
            "os" => s.fold = Fold::Ossetian,
            "fa" => s.fold = Fold::Persian,
            "ar" | "ug" => s.fold = Fold::Arabic,
            "mr" => s.fold = Fold::Marathi,
            "sr" => s.fold = Fold::Serbian,
            _ => {}
        }
        s
    }

    /// Whether the language is written without spaces between words, so
    /// that its words are found with a word list ([`Self::with_word_list`]):
    /// Thai.
    #[must_use]
    pub fn wants_word_list(&self) -> bool {
        self.script.is_some()
    }

    /// This splitter, cutting the runs of letters of a language written
    /// without spaces between words into the words of `list`. A language
    /// written with spaces ignores it.
    #[must_use]
    pub fn with_word_list(mut self, list: Arc<WordList>) -> Self {
        if self.script.is_some() {
            self.word_list = Some(list);
        }
        self
    }

    /// Whether `c` ends a sentence in this language: [`ends_sentence`];
    /// in Greek `;`, its question mark (what the Greek keymap types for it,
    /// and what U+037E normalises to); in Armenian `:`, written for its full
    /// stop `։`.
    #[must_use]
    pub fn ends_sentence(&self, c: char) -> bool {
        ends_sentence(c) || (self.greek && c == ';') || (self.armenian && c == ':')
    }

    /// A word's identity: NFC; lower case (Turkish and Azerbaijani with
    /// their dotted and dotless i, Greek with its final sigma); apostrophes
    /// as U+0027 and hyphens as U+002D; and the language's letters written
    /// two ways written one way.
    #[must_use]
    pub fn normalize(&self, word: &str) -> String {
        let folded = self.fold(word);
        // Lower-cased whole, not character by character: a final sigma
        // depends on what follows it.
        let lower = if self.turkic {
            folded
                .replace('I', "ı")
                .replace('\u{130}', "i")
                .to_lowercase()
        } else {
            folded.to_lowercase()
        };
        match self.fold {
            Fold::Marathi => lower
                .replace("\u{930}\u{94D}\u{200D}", "\u{931}\u{94D}")
                .replace("\u{905}\u{200D}\u{945}", "\u{972}")
                .replace("\u{905}\u{945}", "\u{972}"),
            Fold::Serbian => cyrillic(&lower),
            _ => lower,
        }
    }

    /// `word` in NFC, its apostrophes and hyphens one each, and the
    /// language's letters written two ways written one way: its identity,
    /// but with its case as written.
    fn fold(&self, word: &str) -> String {
        let mut mapped = String::with_capacity(word.len());
        for c in word.nfc() {
            match c {
                c if is_apostrophe(c) || c == '\u{02BC}' => mapped.push('\''),
                c if is_hyphen(c) => mapped.push('-'),
                '\u{05F4}' => mapped.push('"'),
                c => self.fold_char(c, &mut mapped),
            }
        }
        mapped
    }

    /// The capitals of `word` as written, over the characters of its
    /// identity: bit `i` is set when the `i`th is a capital (`Haus` 1,
    /// `E-Mail` 5, `USA` 7, `İstanbul` in Turkish 1). `Some(0)` for a word
    /// written in lower case; `None` when the capitals cannot be placed on
    /// its identity (another number of characters, or more than 64).
    #[must_use]
    pub fn case_mask(&self, word: &str) -> Option<u64> {
        let cased = self.fold(word);
        let identity = self.normalize(word);
        let n = identity.chars().count();
        if n == 0 || n > 64 || cased.chars().count() != n {
            return None;
        }
        Some(
            cased
                .chars()
                .enumerate()
                .filter(|(_, c)| c.is_uppercase())
                .fold(0u64, |m, (i, _)| m | (1 << i)),
        )
    }

    /// `identity` with the capitals of `mask` ([`Self::case_mask`]), as this
    /// language writes them (`i` is `İ` in Turkish).
    #[must_use]
    pub fn display(&self, identity: &str, mask: u64) -> String {
        identity
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i < 64 && mask & (1 << i) != 0 {
                    self.upper(c)
                } else {
                    c.to_string()
                }
            })
            .collect()
    }

    /// Push `c` as this language writes it one way.
    fn fold_char(&self, c: char, out: &mut String) {
        match (self.fold, c) {
            (Fold::Ossetian, 'æ') => out.push('ӕ'),
            (Fold::Ossetian, 'Æ') => out.push('Ӕ'),
            (Fold::Persian, '\u{064A}' | '\u{0649}') => out.push('\u{06CC}'),
            (Fold::Persian, '\u{0643}') => out.push('\u{06A9}'),
            (Fold::Persian | Fold::Arabic, c) if is_harakah(c) => {}
            (Fold::Armenian, '\u{055E}' | '\u{055B}' | '\u{055C}') => {}
            (Fold::Armenian, '\u{0587}') => out.push_str("եվ"),
            (Fold::Kabyle, 'γ') => out.push('ɣ'),
            (Fold::Kabyle, 'Γ') => out.push('Ɣ'),
            (Fold::Kabyle, 'ε') => out.push('ɛ'),
            (Fold::Kabyle, 'Ε') => out.push('Ɛ'),
            (_, c) => out.push(c),
        }
    }

    /// `c` in upper case, as this language writes it (`i` is `İ` in
    /// Turkish).
    #[must_use]
    pub fn upper(&self, c: char) -> String {
        match c {
            'i' if self.turkic => "\u{130}".to_string(),
            c => c.to_uppercase().collect(),
        }
    }

    /// How much of `suggestion` `typed` already is: the bytes of the
    /// shortest start of it that is the same word ([`Self::normalize`]d),
    /// or `None` when no start of it is. A keyboard completing what was
    /// typed keeps it as typed and types only the rest.
    #[must_use]
    pub fn covered(&self, typed: &str, suggestion: &str) -> Option<usize> {
        let want = self.normalize(typed);
        if want.is_empty() {
            return Some(0);
        }
        suggestion
            .char_indices()
            .map(|(i, c)| i + c.len_utf8())
            .find(|&end| self.normalize(&suggestion[..end]) == want)
    }

    /// Whether a leading apostrophe at `cs[i]` begins a word.
    fn leads(&self, cs: &[(usize, char)], i: usize) -> bool {
        match self.lead {
            Lead::Never => false,
            Lead::Always => true,
            Lead::Words(set) => {
                let word: String = cs[i + 1..]
                    .iter()
                    .map(|&(_, c)| c)
                    .take_while(|c| c.is_alphabetic())
                    .flat_map(char::to_lowercase)
                    .collect();
                set.contains(&word.as_str())
            }
        }
    }

    /// The words of `token`, a run of text with no whitespace in it, read
    /// with no single quote open before it.
    #[must_use]
    pub fn pieces(&self, token: &str) -> Vec<Piece> {
        self.pieces_quoted(token, false, false).0
    }

    /// The words of `token`, and whether a single quote is open after it,
    /// given whether one is before it.
    ///
    /// `at_caret`: the token ends at the caret, so a run cut with the word
    /// list ends in the word being typed, which may be the start of one.
    fn pieces_quoted(&self, token: &str, mut quote: bool, at_caret: bool) -> (Vec<Piece>, bool) {
        let cs: Vec<(usize, char)> = token.char_indices().collect();
        let letter_at = |i: usize| cs.get(i).is_some_and(|&(_, c)| c.is_alphabetic());
        let mut out: Vec<Piece> = Vec::new();
        // The piece being read: where it starts, and whether glued.
        let mut open: Option<(usize, bool)> = None;
        // Whether the character before belongs to a word.
        let mut prev_in = false;
        for (i, &(at, c)) in cs.iter().enumerate() {
            let next_letter = letter_at(i + 1);
            let in_word = if c.is_alphabetic() {
                true
            } else if is_combining_mark(c) {
                prev_in
            } else if is_joiner(c) {
                prev_in
                    && cs
                        .get(i + 1)
                        .is_some_and(|&(_, n)| n.is_alphabetic() || is_combining_mark(n))
            } else if is_apostrophe(c) {
                if next_letter {
                    prev_in || self.leads(&cs, i)
                } else {
                    // At a word's end: part of it (`students'`, `απ'`,
                    // `po'`), unless it closes a quote.
                    prev_in
                        && !quote
                        && open.is_some_and(|(start, _)| !opens_with_quote(&cs, start))
                }
            } else if is_hyphen(c) {
                prev_in && next_letter
            } else if c == '\u{00B7}' {
                self.middle_dot && prev_in && next_letter
            } else if matches!(c, '\u{055E}' | '\u{055B}' | '\u{055C}') {
                self.armenian && prev_in
            } else if matches!(c, '"' | '\u{05F4}') {
                self.hebrew && prev_in && next_letter
            } else {
                false
            };
            if !in_word {
                if let Some((start, glued)) = open.take() {
                    out.push(Piece {
                        range: start..at,
                        glued,
                    });
                }
                if is_single_quote(c) {
                    quote = !quote;
                }
                prev_in = false;
                continue;
            }
            // An elided word ends at its apostrophe, a Kabyle word before
            // its clitics at the hyphen; the next is glued on.
            if (is_apostrophe(c) || (self.hyphen_glues && is_hyphen(c)))
                && prev_in
                && next_letter
                && let Some((start, glued)) = open
                && (is_hyphen(c) || self.elides(&token[start..at]))
            {
                let end = at + c.len_utf8();
                out.push(Piece {
                    range: start..end,
                    glued,
                });
                open = Some((end, true));
                continue;
            }
            if open.is_none() {
                open = Some((at, false));
            }
            prev_in = true;
        }
        if let Some((start, glued)) = open {
            out.push(Piece {
                range: start..token.len(),
                glued,
            });
        }
        (self.cut_runs(token, out, at_caret), quote)
    }

    /// `pieces`, with each run of the script written without spaces cut
    /// into the words of the word list: those after the first glued on.
    /// What comes after a run in its piece belongs to its last word, so a
    /// letter typed after it (the probe of [`Self::typing_after`]) carries
    /// that word on; what comes before the first is a word of its own.
    /// `at_caret`: the token ends at the caret ([`Self::pieces_quoted`]).
    fn cut_runs(&self, token: &str, pieces: Vec<Piece>, at_caret: bool) -> Vec<Piece> {
        let (Some(script), Some(list)) = (self.script, self.word_list.as_deref()) else {
            return pieces;
        };
        let mut out = Vec::with_capacity(pieces.len());
        for p in pieces {
            let text = &token[p.range.clone()];
            let mut runs: Vec<Range<usize>> = Vec::new();
            for (at, c) in text.char_indices() {
                if !script.holds(c) {
                    continue;
                }
                match runs.last_mut() {
                    Some(r) if r.end == at => r.end = at + c.len_utf8(),
                    _ => runs.push(at..at + c.len_utf8()),
                }
            }
            let Some(first) = runs.first() else {
                out.push(p);
                continue;
            };
            let base = p.range.start;
            let mut glued = p.glued;
            if first.start > 0 {
                out.push(Piece {
                    range: base..base + first.start,
                    glued,
                });
                glued = true;
            }
            let open = at_caret && p.range.end == token.len();
            for (i, run) in runs.iter().enumerate() {
                let last_run = i + 1 == runs.len();
                let tail = runs.get(i + 1).map_or(text.len(), |r| r.start);
                let words = list.cut(script, &text[run.clone()], open && last_run);
                let n = words.len();
                for (j, w) in words.into_iter().enumerate() {
                    let end = if j + 1 == n { tail } else { run.start + w.end };
                    out.push(Piece {
                        range: base + run.start + w.start..base + end,
                        glued,
                    });
                    glued = true;
                }
            }
        }
        out
    }

    /// Whether the next word follows `word` with no space between: it is
    /// elided (`l'`, `dell'`, Kabyle `fell-`), or ends in a script written
    /// without spaces between words (Thai), so a keyboard that types it
    /// types no space after it.
    #[must_use]
    pub fn glues_on(&self, word: &str) -> bool {
        if self
            .script
            .is_some_and(|s| word.chars().next_back().is_some_and(|c| s.holds(c)))
        {
            return true;
        }
        self.pieces(&format!("{word}a"))
            .last()
            .is_some_and(|p| p.glued && p.range.start == word.len())
    }

    /// Whether `word`, before an apostrophe, is one this language elides.
    fn elides(&self, word: &str) -> bool {
        match self.elide {
            Elide::Never => false,
            Elide::After(set) => set.contains(&self.normalize(word).as_str()),
            Elide::AllBut(set) => !set.contains(&self.normalize(word).as_str()),
        }
    }

    /// The words of `text`, in order, with what prediction needs to know
    /// about each: its identity, whether it is glued to the one before,
    /// whether other characters precede it in its token, how it follows
    /// the words before it, and whether it ends its token.
    #[must_use]
    pub fn words(&self, text: &str) -> Vec<TextWord> {
        let mut out = Vec::new();
        self.scan(text, &mut |w| out.push(w));
        out
    }

    /// Feed `each` the words of `text`; what is left for what comes after.
    fn scan(&self, text: &str, each: &mut dyn FnMut(TextWord)) -> After {
        self.scan_from(text, Start::Sentence, each)
    }

    /// [`Self::scan`] a text whose first word starts as `start` says: a
    /// sentence, at a text's start, or a break, in a text cut from a
    /// longer one.
    fn scan_from(&self, text: &str, start: Start, each: &mut dyn FnMut(TextWord)) -> After {
        let mut after = After {
            start,
            quote: false,
        };
        let mut token_start: Option<usize> = None;
        for (at, c) in text.char_indices() {
            if c.is_whitespace() {
                if let Some(start) = token_start.take() {
                    self.token_words(text, start..at, &mut after, each);
                }
                if self.ends_sentence(c) {
                    after.start = Start::Sentence;
                }
                if c == '\n' {
                    after.quote = false;
                }
            } else if token_start.is_none() {
                token_start = Some(at);
            }
        }
        if let Some(start) = token_start {
            self.token_words(text, start..text.len(), &mut after, each);
        }
        after
    }

    /// Feed `each` the words of the token `range` of `text`, updating what
    /// is left for the word after the token.
    fn token_words(
        &self,
        text: &str,
        range: Range<usize>,
        after: &mut After,
        each: &mut dyn FnMut(TextWord),
    ) {
        let token = &text[range.clone()];
        let (pieces, quote) = self.pieces_quoted(token, after.quote, false);
        after.quote = quote;
        if pieces.is_empty() {
            if after.start == Start::Follows {
                after.start = Start::Break;
            }
            if self.any_end(token) {
                after.start = Start::Sentence;
            }
            return;
        }
        let mut gap_start = 0;
        let last = pieces.len() - 1;
        for (i, p) in pieces.iter().enumerate() {
            if self.any_end(&token[gap_start..p.range.start]) {
                after.start = Start::Sentence;
            }
            each(TextWord {
                range: range.start + p.range.start..range.start + p.range.end,
                text: self.normalize(&token[p.range.clone()]),
                glued: p.glued,
                after_other: p.range.start > 0 && !p.glued,
                start: after.start,
                last_in_token: i == last,
            });
            after.start = Start::Follows;
            gap_start = p.range.end;
        }
        if self.any_end(&token[gap_start..]) {
            after.start = Start::Sentence;
        }
    }

    fn any_end(&self, s: &str) -> bool {
        s.chars().any(|c| self.ends_sentence(c))
    }

    /// Read the text before the caret: the context, the word in progress
    /// and where the caret is. `n` bounds the context.
    ///
    /// The token at the caret is read as if a letter came next: where one
    /// would join the last word (`don'`, `что-`), the caret is in it; where
    /// one would start a word glued on (`l'`), a word starts at the caret;
    /// where neither (`end.`), the caret is past the token's word.
    ///
    /// A sentence's first word has [`SENTENCE_START`] for context; the
    /// text's start is a sentence's ([`Self::typing_after`] when it is not
    /// known to be).
    #[must_use]
    pub fn typing(&self, before_caret: &str, n: usize) -> Typing {
        self.typing_after(before_caret, n, Start::Sentence)
    }

    /// [`Self::typing_after`] where the keyboard knows a word ends at byte
    /// `word_end` of the text before the caret: it typed one there from the
    /// strip, with no space after it, as Thai is written. No word runs
    /// across it, as though a space were there, though none is: right
    /// after it, a word starts.
    #[must_use]
    pub fn typing_after_word(
        &self,
        before_caret: &str,
        word_end: usize,
        n: usize,
        start: Start,
    ) -> Typing {
        if word_end == 0 || !before_caret.is_char_boundary(word_end) {
            return self.typing_after(before_caret, n, start);
        }
        let (done, rest) = before_caret.split_at(word_end);
        self.typing_after(&format!("{done} {rest}"), n, start)
    }

    /// [`Self::typing`] a text before the caret whose start is not the
    /// text's, when `start` says so: [`Start::Break`] for a text whose
    /// start is not known (cut from a longer one, or read after the
    /// keyboard lost track), where no context is assumed.
    #[must_use]
    pub fn typing_after(&self, before_caret: &str, n: usize, start: Start) -> Typing {
        let cut = before_caret
            .char_indices()
            .rev()
            .find(|&(_, c)| c.is_whitespace())
            .map_or(0, |(i, c)| i + c.len_utf8());
        let (done, token) = before_caret.split_at(cut);
        let mut context = Context {
            words: Vec::new(),
            n,
        };
        let after = self.scan_from(done, start, &mut |w| {
            context.restart(Some(w.start));
            context.push(w.text);
        });
        // How the token's first word starts, unless a sentence ends in it.
        let mut fresh = (after.start != Start::Follows).then_some(after.start);
        if token.is_empty() {
            context.restart(fresh);
            return Typing {
                context: context.words,
                word: String::new(),
                caret: Caret::WordStart,
            };
        }
        let (pieces, _) = self.pieces_quoted(token, after.quote, true);
        let probe = format!("{token}a");
        let (probed, _) = self.pieces_quoted(&probe, after.quote, true);
        // An apostrophe right after the last word that reading the token
        // as it is left out closes a quote: a letter after it would not
        // make it part of the word.
        let closes_quote = pieces.last().is_some_and(|l| {
            token[l.range.end..]
                .chars()
                .next()
                .is_some_and(is_apostrophe)
        });
        // The token's finished words, and the caret after them.
        let (finished, word, caret): (&[Piece], Option<Range<usize>>, Caret) =
            match (pieces.last(), probed.last()) {
                (_, Some(p)) if p.glued && p.range.start == token.len() => {
                    (&probed[..probed.len() - 1], None, Caret::WordStart)
                }
                (Some(last), _) if last.range.end == token.len() => (
                    &pieces[..pieces.len() - 1],
                    Some(last.range.clone()),
                    Caret::InWord,
                ),
                (_, Some(p)) if p.range.start < token.len() && !closes_quote => {
                    let before = probed.len() - 1;
                    (
                        &probed[..before],
                        Some(p.range.start..token.len()),
                        Caret::InWord,
                    )
                }
                (None, _) => (&[], None, Caret::BeforeWord),
                _ => (&pieces[..], None, Caret::AfterWord),
            };
        let mut gap_start = 0;
        let sentence_or = |gap: &str, fresh: Option<Start>| {
            if self.any_end(gap) {
                Some(Start::Sentence)
            } else {
                fresh
            }
        };
        for p in finished {
            context.restart(sentence_or(&token[gap_start..p.range.start], fresh));
            fresh = None;
            context.push(self.normalize(&token[p.range.clone()]));
            gap_start = p.range.end;
        }
        let rest_end = word.as_ref().map_or(token.len(), |w| w.start);
        let rest = &token[gap_start.min(rest_end)..rest_end];
        context.restart(sentence_or(rest, fresh));
        Typing {
            context: context.words,
            word: word.map(|r| token[r].to_string()).unwrap_or_default(),
            caret,
        }
    }
}

/// Whether the word starting at byte `start` follows a single quote that
/// opens it.
fn opens_with_quote(cs: &[(usize, char)], start: usize) -> bool {
    cs.iter()
        .position(|&(at, _)| at == start)
        .is_some_and(|i| i > 0 && is_single_quote(cs[i - 1].1))
}

/// Serbian Latin, lower case, as Cyrillic: the digraphs first, then letter
/// for letter. A word with q, w, x or y is a foreign one, left as it is.
fn cyrillic(word: &str) -> String {
    if !word
        .chars()
        .any(|c| matches!(c, 'a'..='z' | 'č' | 'ć' | 'đ' | 'š' | 'ž'))
        || word.contains(['q', 'w', 'x', 'y'])
    {
        return word.to_string();
    }
    let mut out = String::with_capacity(word.len() * 2);
    let mut cs = word.chars().peekable();
    while let Some(c) = cs.next() {
        let next = cs.peek().copied();
        let two = match (c, next) {
            ('l', Some('j')) => Some('љ'),
            ('n', Some('j')) => Some('њ'),
            ('d', Some('ž')) => Some('џ'),
            _ => None,
        };
        if let Some(t) = two {
            out.push(t);
            cs.next();
            continue;
        }
        out.push(match c {
            'a' => 'а',
            'b' => 'б',
            'c' => 'ц',
            'č' => 'ч',
            'ć' => 'ћ',
            'd' => 'д',
            'đ' => 'ђ',
            'e' => 'е',
            'f' => 'ф',
            'g' => 'г',
            'h' => 'х',
            'i' => 'и',
            'j' => 'ј',
            'k' => 'к',
            'l' => 'л',
            'm' => 'м',
            'n' => 'н',
            'o' => 'о',
            'p' => 'п',
            'r' => 'р',
            's' => 'с',
            'š' => 'ш',
            't' => 'т',
            'u' => 'у',
            'v' => 'в',
            'z' => 'з',
            'ž' => 'ж',
            c => c,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn split(lang: &str, token: &str) -> Vec<String> {
        let s = Splitter::new(lang);
        s.pieces(token)
            .into_iter()
            .map(|p| {
                let w = &token[p.range];
                if p.glued {
                    format!("+{w}")
                } else {
                    w.to_string()
                }
            })
            .collect()
    }

    #[test]
    fn indic_marks_and_joiners_stay_in_the_word() {
        assert_eq!(split("hi", "क्या"), ["क्या"]);
        assert_eq!(split("hi", "ज़्यादा"), ["ज़्यादा"]);
        assert_eq!(split("bn", "গুরুত্বপূর্ণ।"), ["গুরুত্বপূর্ণ"]);
        assert_eq!(
            split("mr", "वाहणार\u{94D}\u{200D}या"),
            ["वाहणार\u{94D}\u{200D}या"]
        );
        assert_eq!(split("fa", "می\u{200C}شوند."), ["می\u{200C}شوند"]);
        // A joiner with no word after it is not part of one.
        assert_eq!(split("fa", "می\u{200C}"), ["می"]);
    }

    #[test]
    fn apostrophes_and_hyphens_stay_in_the_word() {
        assert_eq!(split("en", "don't"), ["don't"]);
        assert_eq!(split("en", "Let's,"), ["Let's"]);
        assert_eq!(split("uk", "п'ять"), ["п'ять"]);
        assert_eq!(split("br", "c'hoazh"), ["c'hoazh"]);
        assert_eq!(split("tr", "Tom'un"), ["Tom'un"]);
        assert_eq!(split("ru", "что-то"), ["что-то"]);
        assert_eq!(split("id", "anak-anak."), ["anak-anak"]);
        // One that ends a word is part of it, unless it closes a quote.
        assert_eq!(split("en", "students'"), ["students'"]);
        assert_eq!(split("el", "απ'"), ["απ'"]);
        assert_eq!(split("en", "'hello'"), ["hello"]);
        assert_eq!(split("en", "'hello',"), ["hello"]);
        // Not between letters: punctuation.
        assert_eq!(split("ru", "что-"), ["что"]);
        assert_eq!(split("en", "2-3"), Vec::<String>::new());
    }

    #[test]
    fn a_quote_opened_earlier_on_the_line_is_closed_after_a_word() {
        let w = |text: &str| -> Vec<String> {
            Splitter::new("en")
                .words(text)
                .into_iter()
                .map(|w| w.text)
                .collect()
        };
        assert_eq!(
            w("'I know them' said she"),
            ["i", "know", "them", "said", "she"]
        );
        assert_eq!(w("the students' books"), ["the", "students'", "books"]);
        // A new line closes it.
        assert_eq!(w("'open\nthe dogs'"), ["open", "the", "dogs'"]);
    }

    #[test]
    fn romance_elisions_and_kabyle_clitics_split_and_glue() {
        assert_eq!(split("fr", "l'homme"), ["l'", "+homme"]);
        assert_eq!(split("fr", "Qu'est-ce"), ["Qu'", "+est-ce"]);
        assert_eq!(split("fr", "C\u{2019}est"), ["C\u{2019}", "+est"]);
        assert_eq!(split("fr", "aujourd'hui"), ["aujourd'hui"]);
        assert_eq!(split("fr", "quelqu'un"), ["quelqu'un"]);
        assert_eq!(split("fr", "qu'aujourd'hui"), ["qu'", "+aujourd'hui"]);
        assert_eq!(split("fr", "chef-d'œuvre"), ["chef-d'œuvre"]);
        assert_eq!(split("it", "dell'anno"), ["dell'", "+anno"]);
        assert_eq!(split("it", "c'è"), ["c'", "+è"]);
        assert_eq!(split("it", "po'"), ["po'"]);
        assert_eq!(split("it", "'menù'"), ["menù"]);
        assert_eq!(split("ca", "l'escola"), ["l'", "+escola"]);
        assert_eq!(split("ca", "veure'l"), ["veure'l"]);
        assert_eq!(split("ca", "col·legi"), ["col·legi"]);
        assert_eq!(split("oc", "qu'es"), ["qu'", "+es"]);
        assert_eq!(split("kab", "fell-i"), ["fell-", "+i"]);
        assert_eq!(
            split("kab", "ad-as-d-iniɣ"),
            ["ad-", "+as-", "+d-", "+iniɣ"]
        );
        // English never elides this way, nor Russian at a hyphen.
        assert_eq!(split("en", "o'clock"), ["o'clock"]);
        assert_eq!(split("ru", "кто-нибудь"), ["кто-нибудь"]);
    }

    #[test]
    fn an_elided_word_glues_the_next_one_on() {
        let fr = Splitter::new("fr");
        assert!(fr.glues_on("l'") && fr.glues_on("qu\u{2019}") && fr.glues_on("L'"));
        assert!(!fr.glues_on("aujourd'") && !fr.glues_on("le") && !fr.glues_on("l"));
        let it = Splitter::new("it");
        assert!(it.glues_on("dell'") && !it.glues_on("po'"));
        assert!(Splitter::new("kab").glues_on("fell-"));
        assert!(!Splitter::new("en").glues_on("don'"));
    }

    #[test]
    fn per_language_marks() {
        assert_eq!(split("hy", "Ի՞նչ"), ["Ի՞նչ"]);
        assert_eq!(split("he", "לחו\"ל."), ["לחו\"ל"]);
        assert_eq!(split("en", "say\"hi"), ["say", "hi"]);
        assert_eq!(split("nl", "'s"), ["'s"]);
        assert_eq!(split("nl", "'Hallo'"), ["Hallo"]);
        assert_eq!(split("br", "'zo"), ["'zo"]);
        assert_eq!(split("fr", "'maison'"), ["maison"]);
    }

    #[test]
    fn identity_is_normalised() {
        let tr = Splitter::new("tr");
        assert_eq!(tr.normalize("Işık"), "ışık");
        assert_eq!(tr.normalize("İstanbul"), "istanbul");
        assert_eq!(tr.upper('i'), "\u{130}");
        let en = Splitter::new("en");
        assert_eq!(en.normalize("Don\u{2019}t"), "don't");
        assert_eq!(en.normalize("I"), "i");
        assert_eq!(Splitter::new("uk").normalize("П\u{2BC}ять"), "п'ять");
        assert_eq!(Splitter::new("ru").normalize("Что\u{2011}то"), "что-то");
        assert_eq!(Splitter::new("el").normalize("ΟΔΟΣ"), "οδος");
        // NFC: a decomposed é is one character.
        assert_eq!(en.normalize("cafe\u{301}"), "café");
    }

    #[test]
    fn letters_written_two_ways_are_one() {
        assert_eq!(Splitter::new("os").normalize("Æз"), "ӕз");
        assert_eq!(Splitter::new("fa").normalize("كتابي"), "کتابی");
        assert_eq!(Splitter::new("ar").normalize("جداً"), "جدا");
        assert_eq!(Splitter::new("hy").normalize("Ի՞նչ"), "ինչ");
        assert_eq!(Splitter::new("hy").normalize("Երևան"), "երեվան");
        assert_eq!(Splitter::new("kab").normalize("γur"), "ɣur");
        assert_eq!(
            Splitter::new("mr").normalize("दुसर\u{94D}\u{200D}या"),
            "दुस\u{931}\u{94D}या"
        );
        let sr = Splitter::new("sr");
        assert_eq!(sr.normalize("Ljubav"), "љубав");
        assert_eq!(sr.normalize("džep"), "џеп");
        assert_eq!(sr.normalize("кућа"), "кућа");
        assert_eq!(sr.normalize("kuća"), "кућа");
        assert_eq!(sr.normalize("wifi"), "wifi");
        // Elsewhere nothing folds.
        assert_eq!(Splitter::new("da").normalize("Æble"), "æble");
    }

    #[test]
    fn a_word_keeps_its_capitals_over_its_identity() {
        let de = Splitter::new("de");
        assert_eq!(de.case_mask("Haus"), Some(1));
        assert_eq!(de.case_mask("E-Mail"), Some(0b101));
        assert_eq!(de.display("e-mail", 0b101), "E-Mail");
        assert_eq!(de.case_mask("haus"), Some(0));
        let en = Splitter::new("en");
        assert_eq!(en.case_mask("USA"), Some(0b111));
        assert_eq!(en.case_mask("I\u{2019}m"), Some(1));
        assert_eq!(en.display("i'm", 1), "I'm");
        let tr = Splitter::new("tr");
        assert_eq!(tr.case_mask("İstanbul"), Some(1));
        assert_eq!(tr.display("istanbul", 1), "İstanbul");
        assert_eq!(tr.display("ışık", 1), "Işık");
        // Elsewhere İ lower-cases to two characters: no place for it.
        assert_eq!(en.case_mask("İstanbul"), None);
        assert_eq!(Splitter::new("hi").case_mask("क्या"), Some(0));
    }

    #[test]
    fn a_suggestion_covers_what_was_typed_as_typed() {
        let hy = Splitter::new("hy");
        // The typed mark stays: only the rest is typed.
        assert_eq!(hy.covered("Ի՞ն", "ինչ"), Some("ին".len()));
        let en = Splitter::new("en");
        assert_eq!(en.covered("Don\u{2019}", "don't"), Some(4));
        assert_eq!(en.covered("cat", "dog"), None);
        assert_eq!(en.covered("", "dog"), Some(0));
        let bn = Splitter::new("bn");
        // Typed precomposed, the lexicon's word decomposed (NFC).
        assert_eq!(bn.covered("\u{9DF}", "\u{9AF}\u{9BC}া"), Some(6));
    }

    fn read(lang: &str, text: &str) -> Vec<String> {
        read_with(&Splitter::new(lang), text)
    }

    fn read_with(splitter: &Splitter, text: &str) -> Vec<String> {
        splitter
            .words(text)
            .into_iter()
            .map(|w| {
                let mut s = String::new();
                match w.start {
                    Start::Sentence => s.push('|'),
                    Start::Break => s.push('#'),
                    Start::Follows => {}
                }
                if w.glued {
                    s.push('+');
                }
                if w.after_other {
                    s.push('~');
                }
                s.push_str(&w.text);
                if !w.last_in_token {
                    s.push('_');
                }
                s
            })
            .collect()
    }

    #[test]
    fn context_breaks_where_sentences_and_lines_end() {
        assert_eq!(
            read("fr", "Je pense que l'homme est là. C'est vrai\nOui, 3 fois"),
            [
                "|je", "pense", "que", "l'_", "+homme", "est", "là", "|c'_", "+est", "vrai",
                "|oui", "#fois"
            ]
        );
        assert_eq!(read("el", "Τι; Ναι"), ["|τι", "|ναι"]);
        assert_eq!(read("hy", "Այո: Ոչ"), ["|այո", "|ոչ"]);
        assert_eq!(read("en", "(see «this»,a)"), ["|~see", "~this_", "~a"]);
        assert_eq!(read("en", "end.The"), ["|end_", "|~the"]);
        // A sentence start survives a token with no word.
        assert_eq!(read("en", "Go. 42 times"), ["|go", "|times"]);
    }

    fn typing(lang: &str, text: &str) -> (Vec<String>, String, Caret) {
        let t = Splitter::new(lang).typing(text, 2);
        (t.context, t.word, t.caret)
    }

    fn ws(w: &[&str]) -> Vec<String> {
        w.iter().map(ToString::to_string).collect()
    }

    #[test]
    fn the_caret_is_read_as_if_a_letter_came_next() {
        use Caret::{AfterWord, BeforeWord, InWord, WordStart};
        assert_eq!(typing("en", ""), (ws(&["<s>"]), String::new(), WordStart));
        assert_eq!(
            typing("en", "I saw hel"),
            (ws(&["i", "saw"]), "hel".into(), InWord)
        );
        assert_eq!(
            typing("en", "I saw "),
            (ws(&["i", "saw"]), String::new(), WordStart)
        );
        // A character that joins once a letter follows is in the word.
        assert_eq!(
            typing("en", "I don'"),
            (ws(&["<s>", "i"]), "don'".into(), InWord)
        );
        assert_eq!(
            typing("ru", "это что-"),
            (ws(&["<s>", "это"]), "что-".into(), InWord)
        );
        assert_eq!(
            typing("fa", "او می\u{200C}"),
            (ws(&["<s>", "او"]), "می\u{200C}".into(), InWord)
        );
        // An elided word is finished, and a word starts glued after it.
        assert_eq!(
            typing("fr", "Je vois l'"),
            (ws(&["vois", "l'"]), String::new(), WordStart)
        );
        assert_eq!(
            typing("fr", "Je vois l'h"),
            (ws(&["vois", "l'"]), "h".into(), InWord)
        );
        assert_eq!(
            typing("it", "Quest'"),
            (ws(&["<s>", "quest'"]), String::new(), WordStart)
        );
        assert_eq!(
            typing("kab", "fell-"),
            (ws(&["<s>", "fell-"]), String::new(), WordStart)
        );
        // An apocope is not: the word goes on.
        assert_eq!(
            typing("it", "un po'"),
            (ws(&["<s>", "un"]), "po'".into(), InWord)
        );
        // Past the word, or before any.
        assert_eq!(
            typing("en", "I saw it,"),
            (ws(&["saw", "it"]), String::new(), AfterWord)
        );
        assert_eq!(
            typing("en", "I saw it."),
            (ws(&["<s>"]), String::new(), AfterWord)
        );
        assert_eq!(
            typing("en", "I saw («"),
            (ws(&["i", "saw"]), String::new(), BeforeWord)
        );
        assert_eq!(typing("en", "end.Th"), (ws(&["<s>"]), "Th".into(), InWord));
        assert_eq!(
            typing("en", "I saw\n"),
            (ws(&["<s>"]), String::new(), WordStart)
        );
        // A quote closed after a word is not part of it.
        assert_eq!(
            typing("en", "'yes'"),
            (ws(&["<s>", "yes"]), String::new(), AfterWord)
        );
        // A Dutch quote is not a clipped word.
        assert_eq!(
            typing("nl", "Hij zei '"),
            (ws(&["hij", "zei"]), String::new(), BeforeWord)
        );
        assert_eq!(typing("nl", "'"), (ws(&["<s>"]), String::new(), BeforeWord));
        assert_eq!(typing("br", "'"), (ws(&["<s>"]), "'".into(), InWord));
    }

    #[test]
    fn a_sentence_starts_after_sentence_start_and_a_break_after_nothing() {
        // A sentence's first word, and the words after it.
        assert_eq!(typing("en", "Go. I").0, ws(&["<s>"]));
        assert_eq!(typing("en", "Go. I s").0, ws(&["<s>", "i"]));
        // A number breaks the context but starts no sentence.
        assert_eq!(typing("en", "I have 42 do").0, ws(&[]));
        // A text whose start is not known starts with no context.
        let s = Splitter::new("en");
        assert_eq!(s.typing_after("", 2, Start::Break).context, ws(&[]));
        assert_eq!(
            s.typing_after("so we", 2, Start::Break).context,
            ws(&["so"])
        );
        assert_eq!(
            s.typing_after("so. we", 2, Start::Break).context,
            ws(&["<s>"])
        );
    }

    fn thai() -> Splitter {
        Splitter::new("th").with_word_list(Arc::new(WordList::from_words([
            "ฉัน",
            "รัก",
            "รักษา",
            "ภาษา",
            "ไทย",
            "ภาษาไทย",
            "เป็น",
            "กิน",
            "ข้าว",
        ])))
    }

    #[test]
    fn thai_is_cut_into_words_glued_on() {
        assert_eq!(
            read_with(&thai(), "ฉันรักภาษาไทย กินข้าว"),
            ["|ฉัน_", "+รัก_", "+ภาษาไทย", "กิน_", "+ข้าว"]
        );
        // Before a run, letters of another script are a word of their own.
        assert_eq!(read_with(&thai(), "iPhoneรัก"), ["|iphone_", "+รัก"]);
        // Without its word list, a run is one word; other languages are
        // not cut at all.
        assert_eq!(read("th", "ฉันรักภาษาไทย"), ["|ฉันรักภาษาไทย"]);
        let en = Splitter::new("en").with_word_list(thai().word_list.unwrap());
        assert_eq!(read_with(&en, "ฉันรักภาษาไทย"), ["|ฉันรักภาษาไทย"]);
        assert!(thai().wants_word_list());
        assert!(!en.wants_word_list());
    }

    #[test]
    fn a_thai_word_glues_the_next_on() {
        assert!(thai().glues_on("รัก"));
        assert!(Splitter::new("th").glues_on("รัก"));
        assert!(!thai().glues_on("ok"));
        assert!(!Splitter::new("en").glues_on("รัก"));
    }

    #[test]
    fn typing_thai_the_word_at_the_caret_is_the_run_s_last() {
        let s = thai();
        let t = s.typing("ฉันรักษ", 2);
        assert_eq!(
            (t.context, t.word, t.caret),
            (ws(&["<s>", "ฉัน"]), "รักษ".into(), Caret::InWord)
        );
        // A whole word at the caret may still grow (`รัก`, `รักษา`).
        let t = s.typing("ฉันรัก", 2);
        assert_eq!(
            (t.context, t.word, t.caret),
            (ws(&["<s>", "ฉัน"]), "รัก".into(), Caret::InWord)
        );
        // Where the keyboard typed a word from the strip, the next starts.
        let end = "ฉันรัก".len();
        let t = s.typing_after_word("ฉันรัก", end, 2, Start::Sentence);
        assert_eq!(
            (t.context, t.word, t.caret),
            (ws(&["ฉัน", "รัก"]), String::new(), Caret::WordStart)
        );
        let t = s.typing_after_word("ฉันรักษ", end, 2, Start::Sentence);
        assert_eq!(
            (t.context, t.word, t.caret),
            (ws(&["ฉัน", "รัก"]), "ษ".into(), Caret::InWord)
        );
    }
}
