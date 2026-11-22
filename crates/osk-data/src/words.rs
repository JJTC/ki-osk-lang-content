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
//!   Kabyle after a hyphen, before its clitics (`fell-` and `i`), as do
//!   Maltese after its articles and elisions (`il-` and `kelb`, `b'` and
//!   `mod`) and Irish after the t- and n- before a vowel (`t-` and `athair`),
//!   and Southern Min and Hakka at the hyphens between a word's syllables
//!   (`chhin-` and `chhiūⁿ`, `Hak-` and `kâ`).
//!   Uzbek's apostrophes for the ʻ of oʻ and gʻ are that letter.
//! - Per language: Catalan's middle dot (`col·legi`) and Garo's, its
//!   glottal stop (`A·chik`), Armenian's question,
//!   emphasis and exclamation marks, written on a word's stressed vowel
//!   (`Ի՞նչ`), Hebrew's gershayim in acronyms (`לחו"ל`), a leading
//!   apostrophe in Breton (`'zo`) and in Dutch's and Veluws' `'s`, `'t`,
//!   `'n`, `'k` and `'m`.
//! - Thai, Lao, Khmer and Burmese are written without spaces between
//!   words, and so are Shan, Mon, Sgaw Karen, Pa'O, Arakanese and Eastern
//!   Pwo Karen: a run of their letters is cut into the words of the
//!   language's word list (the syllables, for the last six)
//!   ([`crate::wordbreak`], given with [`Splitter::with_word_list`]), each
//!   after the first glued to the one before, as `homme` is to `l'`; so a
//!   keyboard types no space after a Thai, Lao, Khmer or Burmese word. A
//!   zero-width space, which their text often puts between words, parts
//!   words as a space does, in every language. Without a list, a run is
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
/// language's words; the list is among that model's inputs. A change to
/// one language's words alone gives that language a later generation
/// ([`LATER_GENERATIONS`], [`generation`]), and leaves the others' models
/// as they are.
pub const GENERATION: u32 = 2;

/// The languages (primary subtags) whose words a later generation changed,
/// and changed alone, with that generation. Generation 3 made Sindhi's two
/// h letters one: its texts write ه (U+0647) and ھ (U+06BE) for the same
/// h, six to one, and its lexicon kept the same word twice. Generation 4
/// made Pashto's letters written two ways one (its ي inside a word, ك, گ,
/// ى): its wiki writes 22 000 words both ways. Generation 5 wrote
/// Malayalam's old spellings as its keyboard types them (ൻറ as ന്റ, ൌ as
/// ൗ, a last ു് as ്, a chillu as one letter, no zero-width joiner): its
/// lexicon held some 1 700 words the old way. Generation 6 kept the glottal
/// stop and ejective of Yucatec Maya, Kʼicheʼ, Qʼeqchiʼ and Cusco Quechua
/// as ʼ (U+02BC), the letter their keyboards type: an apostrophe was read
/// as U+0027, so the strip offered `k'iin` where the keys type `kʼ`.
/// Generation 7 gave Crimean Tatar, Zazaki and Talysh Turkish's dotted and
/// dotless i, which their texts write (94%, 80% and 81% of the capitals
/// of their i-words are `İ`): `I` lower-cased to `i`, so `ılık` and `ilk`
/// started sentences as `Ilık` and `Ilk`, and a word written with `İ` had
/// no case to keep. Kirmanjki's text writes `I` for 73% of them, the
/// Kurdish alphabet's way, and keeps its rules. Generation 8 cut Nuosu's
/// Yi, which writes no space between its words, into its syllables, each
/// letter a word ([`Script::Yi`]): a run of them was one word.
pub const LATER_GENERATIONS: &[(&str, u32)] = &[
    ("sd", 3),
    ("ps", 4),
    ("ml", 5),
    ("yua", 6),
    ("quc", 6),
    ("kek", 6),
    ("quz", 6),
    ("crh", 7),
    ("diq", 7),
    ("tly", 7),
    ("ii", 8),
];

/// The generation of the rules `lang`'s words are read by: [`GENERATION`],
/// or its primary subtag's later one ([`LATER_GENERATIONS`]).
#[must_use]
pub fn generation(lang: &str) -> u32 {
    let primary = lang.split(['-', '_']).next().unwrap_or(lang);
    LATER_GENERATIONS
        .iter()
        .find(|(l, _)| *l == primary)
        .map_or(GENERATION, |&(_, g)| g)
}

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
    /// next one on (Kabyle `fell-i`, Maltese `il-kelb`, Tamazight in
    /// Tifinagh `ⴷ-ⵓⵙⵉⵖ`; Southern Min's, Hakka's and Eastern Min's
    /// syllables, `chhin-chhiūⁿ`, `Mìng-dĕ̤ng-ngṳ̄`).
    hyphen_glues: bool,
    /// Uzbek: the apostrophes its texts write for the ʻ of oʻ and gʻ (‘, the
    /// grave accent, ʼ and the others) belong to the word, and are that ʻ
    /// (U+02BB).
    uzbek: bool,
    /// Tibetan and Dzongkha: a syllable ends at its tsheg (U+0F0B, and the
    /// non-breaking U+0F0C written for it), which belongs to it, and the
    /// next is glued on, as an elided word's is: `བོད་ཀྱི` is `བོད་` and
    /// `ཀྱི`. A syllable is a word, as the keyboard offers it: with its
    /// tsheg, so the next starts right after it.
    tsheg: bool,
    /// The Mayan languages' and Cusco's and South Bolivian Quechua's glottal
    /// stop and ejective (Kʼicheʼ, Qʼeqchiʼ, chʼaki, qʼala), and Akoose's
    /// glottal stop (ʼɛ): ʼ
    /// (U+02BC), as their keyboards type it, and an apostrophe written
    /// after a letter is that letter (generation 6).
    ejective: bool,
    /// U+00B7 between letters (Catalan `l·l`, Garo's glottal stop in
    /// `A·chik`).
    middle_dot: bool,
    /// Armenian: marks written on a word's vowel, and `:` written for the
    /// full stop `։`.
    armenian: bool,
    /// Hebrew gershayim between letters.
    hebrew: bool,
    /// `;` is a question mark (Greek, Pontic Greek), so it ends a sentence.
    greek: bool,
    /// Dotted and dotless i (Turkish, Azerbaijani, Crimean Tatar, Gagauz,
    /// Zazaki, Talysh): `I` lower-cases to `ı`, `İ` to `i`, and `i`
    /// upper-cases to `İ`.
    turkic: bool,
    /// The script the language writes without spaces between words (Thai,
    /// Lao, Khmer, Burmese; Shan, Mon, Sgaw Karen, Pa'O, Arakanese, Eastern
    /// Pwo Karen; Nuosu's Yi).
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
    /// Always: Breton's (`'zo`), and Zeelandic's, which writes Dutch's
    /// clipped words and drops h and ge- the same way (`'oôdstad`,
    /// `'ebouwd`).
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
    /// Not for Serbian in Latin letters (`sr-Latn`), whose keyboard types
    /// Latin and whose model is read from Latin text.
    Serbian,
    /// Sindhi's ھ as ه, its usual h: its texts write both for the same
    /// letter (generation 3).
    Sindhi,
    /// Pashto's letters as its keyboard types them (generation 4): ی for
    /// ي inside a word, where the two are one letter (at a word's end ي is
    /// -i, ی -ay), ک for Arabic ك, ګ for Persian گ, ی for ى.
    Pashto,
    /// Lao's ໍາ, ຫນ, ຫມ and ຫລ as the keyboard types them: ຳ ໜ ໝ ຫຼ
    /// ([`crate::wordbreak::lao_fold`]).
    Lao,
    /// Malayalam's old spellings as its keyboard types them (generation
    /// 5): ൻറ as ന്റ, the two-part ൌ as ൗ, a last ു് as ്, a chillu written
    /// as its consonant, the virama and a zero-width joiner as the one
    /// letter, and no zero-width joiner else.
    Malayalam,
}

/// Malayalam's old spellings of `word` as its keyboard types them
/// ([`Fold::Malayalam`]).
fn malayalam(word: &str) -> String {
    let mut s = word.to_string();
    // A chillu as its consonant, the virama and a zero-width joiner.
    for (consonant, chillu) in [
        ('\u{0D23}', '\u{0D7A}'),
        ('\u{0D28}', '\u{0D7B}'),
        ('\u{0D30}', '\u{0D7C}'),
        ('\u{0D32}', '\u{0D7D}'),
        ('\u{0D33}', '\u{0D7E}'),
        ('\u{0D15}', '\u{0D7F}'),
    ] {
        s = s.replace(&format!("{consonant}\u{0D4D}\u{200D}"), &chillu.to_string());
    }
    s = s
        .replace('\u{200D}', "")
        .replace("\u{0D7B}\u{0D31}", "\u{0D28}\u{0D4D}\u{0D31}")
        .replace('\u{0D4C}', "\u{0D57}");
    match s.strip_suffix("\u{0D41}\u{0D4D}") {
        Some(stem) => format!("{stem}\u{0D4D}"),
        None => s,
    }
}

/// Whether `c` parts words as a space does: whitespace, and the zero-width
/// space, which Khmer and Burmese text puts between words where it shows
/// none.
fn is_space(c: char) -> bool {
    c.is_whitespace() || c == '\u{200B}'
}

/// The elided words French writes with an apostrophe before a vowel.
/// Others stay whole: `aujourd'hui`, `quelqu'un`, `presqu'île`.
const FRENCH_ELISIONS: &[&str] = &[
    "l", "d", "j", "m", "t", "s", "n", "c", "ç", "qu", "lorsqu", "puisqu", "jusqu", "quoiqu",
];
/// Picard's: French's, and its own `ch'` (`ch'est`), `ed'`, `éch'`,
/// `ej'`, `em'`, the liaison's `z'` (`z'ont`) and `pasqu'`; `p'tit` and
/// `r'venir`, a letter left out inside a word, stay whole.
const PICARD_ELISIONS: &[&str] = &[
    "l", "d", "j", "m", "t", "s", "n", "c", "ç", "qu", "ch", "z", "ed", "éd", "ech", "éch", "ej",
    "éj", "em", "ém", "lorsqu", "puisqu", "jusqu", "dusqu", "quoiqu", "pasqu",
];
/// Catalan's proclitic elisions; enclitics stay whole (`veure'l`).
const CATALAN_ELISIONS: &[&str] = &["l", "d", "m", "t", "s", "n"];
/// Occitan's.
const OCCITAN_ELISIONS: &[&str] = &["l", "d", "qu", "m", "t", "s", "n"];
/// Maltese's prepositions and its negation elided before a vowel (`b'mod`,
/// `f'kull`, `x'inhu`, `m'għandux`).
const MALTESE_ELISIONS: &[&str] = &["b", "f", "x", "m", "s", "l", "t"];
/// Italian's apocopes: words cut short, written with an apostrophe and a
/// space after (`un po' di`), not elided.
const ITALIAN_APOCOPES: &[&str] = &[
    "po", "di", "fa", "va", "sta", "da", "be", "mo", "to", "ve", "ca", "pie",
];
/// Dutch's clipped words (`'s avonds`, `'t is`, `'n huis`), which Veluws
/// writes too (`'t Boerenklokjen`).
const DUTCH_CLIPPED: &[&str] = &["s", "t", "n", "k", "m"];

/// Apostrophes: ASCII, the typographic right single quote, and the Hebrew
/// geresh. U+02BC is a letter already.
/// Whether the character before `cs[i]` is an o or a g: Uzbek's oʻ and gʻ.
fn after_o_or_g(cs: &[(usize, char)], i: usize) -> bool {
    i.checked_sub(1)
        .and_then(|j| cs.get(j))
        .is_some_and(|&(_, c)| matches!(c, 'o' | 'O' | 'g' | 'G'))
}

fn is_apostrophe(c: char) -> bool {
    matches!(c, '\'' | '\u{2019}' | '\u{05F3}')
}

/// The Tibetan script's syllable mark, the tsheg, and its non-breaking
/// form.
fn is_tsheg(c: char) -> bool {
    matches!(c, '\u{0F0B}' | '\u{0F0C}')
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
            | '\u{F0D}'
            | '\u{F0E}'
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
        let latin = lang
            .split(['-', '_'])
            .nth(1)
            .is_some_and(|t| t.eq_ignore_ascii_case("latn"));
        let mut s = Self {
            elide: Elide::Never,
            lead: Lead::Never,
            fold: Fold::None,
            hyphen_glues: false,
            uzbek: false,
            ejective: false,
            tsheg: false,
            middle_dot: false,
            armenian: false,
            hebrew: false,
            greek: false,
            turkic: false,
            script: Script::of_lang(primary),
            // A script cut with no list (Yi) has its empty one.
            word_list: Script::of_lang(primary)
                .filter(|s| s.needs_no_list())
                .map(|_| Arc::new(WordList::from_words(std::iter::empty::<&str>()))),
        };
        match primary {
            "fr" => s.elide = Elide::After(FRENCH_ELISIONS),
            "pcd" => s.elide = Elide::After(PICARD_ELISIONS),
            "it" => s.elide = Elide::AllBut(ITALIAN_APOCOPES),
            "ca" => {
                s.elide = Elide::After(CATALAN_ELISIONS);
                s.middle_dot = true;
            }
            "grt" => s.middle_dot = true,
            "oc" => s.elide = Elide::After(OCCITAN_ELISIONS),
            "br" | "zea" => s.lead = Lead::Always,
            "nl" | "vel" => s.lead = Lead::Words(DUTCH_CLIPPED),
            "hy" => {
                s.armenian = true;
                s.fold = Fold::Armenian;
            }
            "he" => s.hebrew = true,
            "el" | "pnt" => s.greek = true,
            "tr" | "az" | "crh" | "gag" | "diq" | "tly" => s.turkic = true,
            "kab" => {
                s.hyphen_glues = true;
                s.fold = Fold::Kabyle;
            }
            "mt" => {
                s.hyphen_glues = true;
                s.elide = Elide::After(MALTESE_ELISIONS);
            }
            "ga" | "ber" | "nan" | "hak" | "cdo" => s.hyphen_glues = true,
            "uz" => s.uzbek = true,
            "yua" | "quc" | "kek" | "quz" | "quh" | "bss" => s.ejective = true,
            "bo" | "dz" => s.tsheg = true,
            "os" => s.fold = Fold::Ossetian,
            "fa" => s.fold = Fold::Persian,
            "ar" | "ug" => s.fold = Fold::Arabic,
            "mr" => s.fold = Fold::Marathi,
            "sr" if !latin => s.fold = Fold::Serbian,
            "lo" => s.fold = Fold::Lao,
            "sd" => s.fold = Fold::Sindhi,
            "ps" => s.fold = Fold::Pashto,
            "ml" => s.fold = Fold::Malayalam,
            _ => {}
        }
        s
    }

    /// Whether the language is written without spaces between words, so
    /// that its words are found with a word list ([`Self::with_word_list`]):
    /// Thai, Lao, Khmer, Burmese; Shan, Mon, Sgaw Karen, Pa'O, Arakanese,
    /// Eastern Pwo Karen. Not Nuosu, whose Yi is cut into its syllables
    /// with no list.
    #[must_use]
    pub fn wants_word_list(&self) -> bool {
        self.script.is_some_and(|s| !s.needs_no_list())
    }

    /// This splitter, cutting the runs of letters of a language written
    /// without spaces between words into the words of `list`. A language
    /// written with spaces ignores it, and so does one cut into its
    /// syllables with no list (Nuosu's Yi).
    #[must_use]
    pub fn with_word_list(mut self, list: Arc<WordList>) -> Self {
        if self.wants_word_list() {
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
            Fold::Lao => crate::wordbreak::lao_fold(&lower).into_owned(),
            Fold::Malayalam => malayalam(&lower),
            _ => lower,
        }
    }

    /// `word` in NFC, its apostrophes and hyphens one each, and the
    /// language's letters written two ways written one way: its identity,
    /// but with its case as written.
    fn fold(&self, word: &str) -> String {
        let mut mapped = String::with_capacity(word.len());
        let mut chars = word.nfc().peekable();
        while let Some(c) = chars.next() {
            match c {
                // Uzbek's oʻ and gʻ, whichever apostrophe the text writes.
                c if self.uzbek
                    && matches!(
                        c,
                        '\'' | '\u{2019}' | '\u{2018}' | '`' | '\u{02BC}' | '\u{02BB}'
                    )
                    && mapped.ends_with(['o', 'O', 'g', 'G']) =>
                {
                    mapped.push('\u{02BB}');
                }
                // The Mayan and Quechuan glottal stop and ejective: ʼ, and
                // an apostrophe after a letter written for it.
                c if self.ejective
                    && (c == '\u{02BC}'
                        || (is_apostrophe(c)
                            && mapped.chars().next_back().is_some_and(char::is_alphabetic))) =>
                {
                    mapped.push('\u{02BC}');
                }
                c if is_apostrophe(c) || c == '\u{02BC}' => mapped.push('\''),
                c if is_hyphen(c) => mapped.push('-'),
                '\u{0F0C}' if self.tsheg => mapped.push('\u{0F0B}'),
                '\u{05F4}' => mapped.push('"'),
                // Pashto's ي before another letter: its ی.
                '\u{064A}'
                    if self.fold == Fold::Pashto
                        && chars.peek().is_some_and(|n| n.is_alphabetic()) =>
                {
                    mapped.push('\u{06CC}');
                }
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
            // Pashto's ي is folded only inside a word (`fold`).
            (Fold::Persian, '\u{064A}' | '\u{0649}') | (Fold::Pashto, '\u{0649}') => {
                out.push('\u{06CC}');
            }
            (Fold::Persian | Fold::Pashto, '\u{0643}') => out.push('\u{06A9}'),
            (Fold::Persian | Fold::Arabic, c) if is_harakah(c) => {}
            (Fold::Armenian, '\u{055E}' | '\u{055B}' | '\u{055C}') => {}
            (Fold::Armenian, '\u{0587}') => out.push_str("եվ"),
            (Fold::Kabyle, 'γ') => out.push('ɣ'),
            (Fold::Kabyle, 'Γ') => out.push('Ɣ'),
            (Fold::Kabyle, 'ε') => out.push('ɛ'),
            (Fold::Kabyle, 'Ε') => out.push('Ɛ'),
            (Fold::Sindhi, '\u{06BE}') => out.push('\u{0647}'),
            (Fold::Pashto, '\u{06AF}') => out.push('\u{06AB}'),
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
            } else if self.uzbek && matches!(c, '\u{2018}' | '`' | '\u{02BC}') {
                prev_in && (next_letter || after_o_or_g(&cs, i))
            } else if self.tsheg && is_tsheg(c) {
                prev_in
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
            // its clitics at the hyphen, a Tibetan syllable at its tsheg;
            // the next is glued on.
            let tsheg = self.tsheg && is_tsheg(c);
            if (is_apostrophe(c) || (self.hyphen_glues && is_hyphen(c)) || tsheg)
                && prev_in
                && next_letter
                && let Some((start, glued)) = open
                && (is_hyphen(c) || tsheg || self.elides(&token[start..at]))
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
    /// without spaces between words (Thai, Lao, Khmer, Burmese, Shan…), so a
    /// keyboard that types it
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
            if is_space(c) {
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
            .find(|&(_, c)| is_space(c))
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
    fn sindhi_s_two_h_letters_are_one() {
        let sd = Splitter::new("sd");
        assert_eq!(sd.normalize("ھن"), sd.normalize("هن"));
        assert_eq!(sd.normalize("آھن"), "آهن");
        // Urdu and Western Punjabi write their aspirates with ھ, apart.
        assert_eq!(Splitter::new("ur").normalize("بھی"), "بھی");
        assert_eq!(Splitter::new("pnb").normalize("ھن"), "ھن");
        assert_eq!(generation("sd"), 3);
        assert_eq!(generation("sd-Deva"), 3);
        assert_eq!(generation("hi"), GENERATION);
    }

    #[test]
    fn maltese_s_articles_and_elisions_glue_the_word_on() {
        assert_eq!(split("mt", "il-kelb"), ["il-", "+kelb"]);
        assert_eq!(split("mt", "għall-familja"), ["għall-", "+familja"]);
        assert_eq!(split("mt", "b\u{2019}mod"), ["b\u{2019}", "+mod"]);
        assert_eq!(split("mt", "x'inhu"), ["x'", "+inhu"]);
        // ta' before a space is a word of its own.
        assert_eq!(split("mt", "ta'"), ["ta'"]);
        // Irish's t- and n- before a vowel glue the noun on.
        assert_eq!(split("ga", "t-athair"), ["t-", "+athair"]);
    }

    #[test]
    fn uzbek_s_okina_is_one_letter_whichever_the_text_writes() {
        let s = Splitter::new("uz");
        for written in [
            "o\u{02BB}zbek",
            "o'zbek",
            "o\u{2018}zbek",
            "o\u{2019}zbek",
            "o`zbek",
            "o\u{02BC}zbek",
        ] {
            assert_eq!(split("uz", written).len(), 1, "{written}");
            assert_eq!(s.normalize(written), "o\u{02BB}zbek", "{written}");
        }
        assert_eq!(s.normalize("bog\u{2018}"), "bog\u{02BB}");
        // An apostrophe after another letter is the tutuq belgisi, as written.
        assert_eq!(s.normalize("ma'no"), "ma'no");
        // Other languages keep their apostrophes.
        assert_eq!(Splitter::new("en").normalize("o'clock"), "o'clock");
    }

    #[test]
    fn malayalam_s_old_spellings_are_its_keyboard_s() {
        let s = Splitter::new("ml");
        // ൻറ as ന്റ, ൌ as ൗ, a last ു് as ്.
        assert_eq!(s.normalize("എൻറെ"), "എന്റെ");
        assert_eq!(s.normalize("ബ്രൌൺ"), "ബ്രൗൺ");
        assert_eq!(s.normalize("ചെയ്തു്"), "ചെയ്ത്");
        // A chillu of the virama and a joiner is the one letter; a stray
        // joiner is dropped; a non-joiner stays.
        assert_eq!(s.normalize("അവന്\u{200D}"), "അവൻ");
        assert_eq!(s.normalize("അവർ\u{200D}ക്കായി"), "അവർക്കായി");
        assert_eq!(s.normalize("മുൻ\u{200C}നിര"), "മുൻ\u{200C}നിര");
        // ു് inside a word, and other languages, are as they are.
        assert_eq!(s.normalize("കു്ക"), "കു്ക");
        assert_eq!(Splitter::new("ta").normalize("எൻറ"), "எൻറ");
        assert_eq!(generation("ml"), 5);
    }

    #[test]
    fn pashto_s_letters_written_two_ways_are_one() {
        let ps = Splitter::new("ps");
        // ي inside a word is ی; at its end it is -i, apart from ی, -ay.
        assert_eq!(ps.normalize("روغتيا"), "روغتیا");
        assert_eq!(ps.normalize("سړي"), "سړي");
        assert_eq!(ps.normalize("سړی"), "سړی");
        assert_eq!(ps.normalize("يو"), "یو");
        assert_eq!(ps.normalize("كور"), "کور");
        assert_eq!(ps.normalize("گل"), "ګل");
        assert_eq!(ps.normalize("دى"), "دی");
        // Persian and Urdu keep their own.
        assert_eq!(Splitter::new("ur").normalize("گل"), "گل");
        assert_eq!(generation("ps"), 4);
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
        assert_eq!(split("pcd", "ch'est"), ["ch'", "+est"]);
        assert_eq!(split("pcd", "éd'l'iau"), ["éd'", "+l'", "+iau"]);
        assert_eq!(split("pcd", "p'tit"), ["p'tit"]);
        // Lombard keeps its clitics whole: written gh'è and g'è both, cut
        // off they were corrected one to the other.
        assert_eq!(split("lmo", "gh'è"), ["gh'è"]);
        assert_eq!(split("ca", "l'escola"), ["l'", "+escola"]);
        assert_eq!(split("ca", "veure'l"), ["veure'l"]);
        assert_eq!(split("ca", "col·legi"), ["col·legi"]);
        // Garo's glottal stop is a letter of its words; a dot ending one
        // is not.
        assert_eq!(split("grt", "A·chikku"), ["A·chikku"]);
        assert_eq!(split("grt", "an·chingni ong·a."), ["an·chingni", "ong·a"]);
        assert_eq!(split("grt", "A· chik"), ["A", "chik"]);
        assert_eq!(split("oc", "qu'es"), ["qu'", "+es"]);
        assert_eq!(split("kab", "fell-i"), ["fell-", "+i"]);
        // A Tibetan syllable ends at its tsheg, which it keeps, and glues
        // the next on; the last before a shad has none.
        assert_eq!(split("bo", "བོད་ཀྱི་རྒྱལ་ཁབ།"), ["བོད་", "+ཀྱི་", "+རྒྱལ་", "+ཁབ"]);
        assert_eq!(split("dz", "ང་ཨིན།"), ["ང་", "+ཨིན"]);
        // Elsewhere a tsheg is no letter of a word.
        assert_eq!(split("en", "a་b"), ["a", "b"]);
        // Tamazight in Tifinagh glues its clitics on as Kabyle does.
        assert_eq!(split("ber", "ⴷ-ⵓⵙⵉⵖ"), ["ⴷ-", "+ⵓⵙⵉⵖ"]);
        assert_eq!(
            split("kab", "ad-as-d-iniɣ"),
            ["ad-", "+as-", "+d-", "+iniɣ"]
        );
        // Southern Min (Pe̍h-ōe-jī) and Hakka (Pha̍k-fa-sṳ) at the hyphens
        // between a word's syllables, its ⁿ and o͘'s dot (U+0358) letters
        // of it.
        assert_eq!(split("nan", "chhin-chhiūⁿ"), ["chhin-", "+chhiūⁿ"]);
        assert_eq!(split("nan", "hō\u{358}"), ["hō\u{358}"]);
        assert_eq!(split("hak", "Hak-kâ-ngî"), ["Hak-", "+kâ-", "+ngî"]);
        assert_eq!(split("hak", "ṳ\u{302}"), ["ṳ\u{302}"]);
        // Eastern Min (Bàng-uâ-cê) so too, its vowels' diaeresis below
        // (U+0324) a letter of them.
        assert_eq!(
            split("cdo", "Mìng-dĕ\u{324}ng-ngṳ̄"),
            ["Mìng-", "+dĕ\u{324}ng-", "+ngṳ̄"]
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
        assert_eq!(split("zea", "'oôdstad"), ["'oôdstad"]);
        assert_eq!(split("zea", "'t"), ["'t"]);
        assert_eq!(split("zea", "d'r"), ["d'r"]);
        assert_eq!(split("vel", "'t"), ["'t"]);
        assert_eq!(split("vel", "'Hallo'"), ["Hallo"]);
        assert_eq!(split("fr", "'maison'"), ["maison"]);
    }

    #[test]
    fn identity_is_normalised() {
        let tr = Splitter::new("tr");
        assert_eq!(tr.normalize("Işık"), "ışık");
        assert_eq!(tr.normalize("İstanbul"), "istanbul");
        assert_eq!(tr.upper('i'), "\u{130}");
        // So do the Turkic languages written with Turkish's i and ı.
        let gag = Splitter::new("gag");
        assert_eq!(gag.normalize("İlk"), "ilk");
        assert_eq!(gag.normalize("Işık"), "ışık");
        assert_eq!(Splitter::new("crh").upper('i'), "\u{130}");
        assert_eq!(generation("crh"), 7);
        // Kirmanjki writes I for i's capital, as Kurdish does.
        assert_eq!(Splitter::new("kiu").normalize("In"), "in");
        assert_eq!(generation("kiu"), GENERATION);
        assert_eq!(generation("gag"), GENERATION);
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
        // Serbian in Latin letters keeps them.
        let sr_latn = Splitter::new("sr-Latn");
        assert_eq!(sr_latn.normalize("Ljubav"), "ljubav");
        assert_eq!(sr_latn.normalize("kuća"), "kuća");
        assert_eq!(Splitter::new("sr-latn-RS").normalize("Džep"), "džep");
        assert_eq!(Splitter::new("sr-RS").normalize("kuća"), "кућа");
        // The Mayan ejective is the letter its keyboard types, however
        // the text writes it; elsewhere ʼ is an apostrophe.
        let quc = Splitter::new("quc");
        assert_eq!(quc.normalize("K'iche'"), "kʼicheʼ");
        assert_eq!(quc.normalize("Kʼicheʼ"), "kʼicheʼ");
        assert_eq!(quc.normalize("k\u{2019}o"), "kʼo");
        assert_eq!(Splitter::new("quz").normalize("ch'aki"), "chʼaki");
        assert_eq!(Splitter::new("quh").normalize("q’ala"), "qʼala");
        // Akoose's glottal stop, which its text writes ʼ.
        assert_eq!(Splitter::new("bss").normalize("ʼɛ"), "ʼɛ");
        assert_eq!(Splitter::new("bss").normalize("nyaʼ"), "nyaʼ");
        assert_eq!(Splitter::new("gn").normalize("mbaʼe"), "mba'e");
        // Elsewhere nothing folds.
        assert_eq!(Splitter::new("da").normalize("Æble"), "æble");
    }

    #[test]
    fn a_tibetan_syllable_is_typed_with_its_tsheg_and_the_next_starts_after_it() {
        let bo = Splitter::new("bo");
        // A strip syllable is typed with its tsheg and no space after it.
        assert!(bo.glues_on("བོད་"));
        assert!(!bo.glues_on("ཁབ"));
        // The non-breaking tsheg is the tsheg.
        assert_eq!(bo.normalize("བོད\u{0F0C}"), "བོད་");
        // After a tsheg typed, the next syllable starts: the strip offers
        // what follows.
        let t = bo.typing("བོད་", 2);
        assert_eq!(t.caret, Caret::WordStart);
        assert_eq!(t.context.last().map(String::as_str), Some("བོད་"));
        let t = bo.typing("བོད་ཀྱ", 2);
        assert_eq!(t.caret, Caret::InWord);
        assert_eq!(t.word, "ཀྱ");
        // A shad ends the sentence.
        assert!(ends_sentence('།'));
        let t = bo.typing("བོད་ཡིན། ", 2);
        assert_eq!(t.context, [SENTENCE_START]);
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
        assert_eq!(read("pnt", "Ντο; Ναι"), ["|ντο", "|ναι"]);
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
    fn nuosu_s_yi_is_cut_into_its_syllables_with_no_list() {
        // ꆈꌠꉙ (nuosuhxop): three syllables, each a word glued on; its
        // iteration mark ꀕ a syllable too. Its comma, written with no
        // space after it, parts the runs of one token.
        assert_eq!(
            read("ii", "ꆈꌠꉙꀕ，ꉬꌠ。"),
            ["|ꆈ_", "+ꌠ_", "+ꉙ_", "+ꀕ_", "~ꉬ_", "+ꌠ"]
        );
        let ii = Splitter::new("ii");
        assert!(!ii.wants_word_list());
        assert!(ii.glues_on("ꆈ"));
        // A list given changes nothing.
        let listed = Splitter::new("ii").with_word_list(Arc::new(WordList::from_words(["ꆈꌠ"])));
        assert_eq!(read_with(&listed, "ꆈꌠ"), ["|ꆈ_", "+ꌠ"]);
        // The syllable at the caret is the one being typed.
        let t = ii.typing("ꆈꌠ", 2);
        assert_eq!(
            (t.context, t.word, t.caret),
            (ws(&["<s>", "ꆈ"]), "ꌠ".into(), Caret::InWord)
        );
        assert_eq!(generation("ii"), 8);
    }

    #[test]
    fn a_zero_width_space_parts_words() {
        let khmer = Splitter::new("km").with_word_list(Arc::new(WordList::from_words([
            "ខ្ញុំ",
            "ស្រលាញ់",
            "ភាសា",
            "ខ្មែរ",
        ])));
        // Between two runs, as Khmer text often has it: no word holds it.
        assert_eq!(
            read_with(&khmer, "ខ្ញុំ\u{200B}ស្រលាញ់ភាសាខ្មែរ"),
            ["|ខ្ញុំ", "ស្រលាញ់_", "+ភាសា_", "+ខ្មែរ"]
        );
        assert_eq!(read("en", "two\u{200B}words"), ["|two", "words"]);
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
