//! Word lists for the languages written without spaces between words,
//! Thai, Lao, Khmer and Burmese, and Shan, Mon, Sgaw Karen, Pa'O and
//! Arakanese: what
//! [`crate::words::Splitter`] cuts their runs of letters into words with,
//! the same way in the trainer, the keyboard and the measure.
//!
//! A list is a plain text file, one word per line, as ICU's word-break
//! dictionaries are (`thaidict.txt`, `laodict.txt`, `khmerdict.txt`,
//! `burmesedict.txt`): blank lines, lines starting with `#`
//! and a byte-order mark are skipped. It has no frequencies, so a run is
//! cut into the fewest words of the list that cover as much of it as they
//! can (maximal matching), the earlier word the longer where two cuts tie.
//! What no word of the list covers is a word of its own, as long as it
//! runs. A word starts only where the script lets one start: never before
//! a vowel sign or a tone mark, nor right after a vowel written before its
//! consonant, nor on a consonant joined below the one before it (Khmer's
//! coeng, Burmese's virama), nor on a Burmese consonant its asat makes the
//! last of the syllable before. Lao's letters written two ways
//! ([`lao_fold`]) are one in the list and in what is looked up in it.
//!
//! Shan, Mon, Sgaw Karen, Pa'O and Arakanese, written in the Myanmar
//! script, have no word list of their own to be had: theirs is the
//! syllables their text writes ([`Script::syllables`], made by
//! `build-syllables`), so each syllable is a word, as each of Tibetan's
//! is. Nuosu's Yi, whose every letter is a syllable, needs no list at all:
//! each letter is a word ([`Script::Yi`]).

use std::borrow::Cow;
use std::fmt;
use std::ops::Range;

use unicode_normalization::char::is_combining_mark;
use unicode_normalization::{UnicodeNormalization, is_nfc};

/// A script written without spaces between words, and where in its text
/// a word may start.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Script {
    /// Thai, U+0E00 to U+0E7F.
    Thai,
    /// Lao, U+0E80 to U+0EFF.
    Lao,
    /// Khmer, U+1780 to U+17FF.
    Khmer,
    /// Myanmar, U+1000 to U+109F: Burmese.
    Myanmar,
    /// Myanmar as Shan, Mon, Pa'O and Arakanese write it, cut into syllables:
    /// Myanmar Extended-A's and -B's letters and signs too (Pa'O's tone
    /// ꩻ), and a consonant stacked over the next starts one, as Mon's
    /// minor syllables do (ပ္ဍဲ).
    MyanmarSyllables,
    /// Myanmar as Sgaw Karen writes it, cut into syllables as Shan's are,
    /// but its asat a tone, which ends no consonant: ဒ် (like) is a
    /// syllable of its own, as Karen's syllables end in a vowel.
    KarenSyllables,
    /// Yi, U+A000 to U+A48C, as Nuosu writes it: each letter a syllable,
    /// each syllable a word, with no list (its iteration mark ꀕ too, a
    /// syllable that repeats the one before).
    Yi,
}

/// Khmer's coeng, which writes the consonant after it below the one before.
const COENG: char = '\u{17D2}';
/// Myanmar's virama, which stacks the consonant after it under the one
/// before, and its asat, which takes a consonant's vowel away.
const MYANMAR_VIRAMA: char = '\u{1039}';
const ASAT: char = '\u{103A}';

impl Script {
    /// The script `lang` (a BCP-47 tag) is written in, when it is one of
    /// these: `th`, `lo`, `km`, `my`; `shn`, `mnw`, `ksw`, `blk`.
    #[must_use]
    pub fn of_lang(lang: &str) -> Option<Self> {
        match lang.split(['-', '_']).next().unwrap_or(lang) {
            "th" => Some(Self::Thai),
            "lo" => Some(Self::Lao),
            "km" => Some(Self::Khmer),
            "my" => Some(Self::Myanmar),
            "shn" | "mnw" | "blk" | "rki" => Some(Self::MyanmarSyllables),
            "ksw" => Some(Self::KarenSyllables),
            "ii" => Some(Self::Yi),
            _ => None,
        }
    }

    /// Whether `c` is one of this script's characters, which its runs are
    /// made of. Its digits, its currency sign and its punctuation end a
    /// run, as they end a word anywhere, and so does a zero-width space,
    /// which Khmer and Burmese text often puts between words; the
    /// zero-width joiner and non-joiner, which a few of their words hold,
    /// do not.
    #[must_use]
    pub fn holds(self, c: char) -> bool {
        match self {
            Self::Thai => ('\u{0E01}'..='\u{0E4E}').contains(&c) && c != '\u{0E3F}',
            Self::Lao => {
                ('\u{0E81}'..='\u{0EDF}').contains(&c) && !('\u{0ED0}'..='\u{0ED9}').contains(&c)
            }
            // Not ។ ៕ ៖ (sentence and colon), ៘ ៙ ៚ ៛ (marks, the riel),
            // nor the digits.
            Self::Khmer => {
                ('\u{1780}'..='\u{17D3}').contains(&c)
                    || matches!(
                        c,
                        '\u{17D7}' | '\u{17DC}' | '\u{17DD}' | '\u{200C}' | '\u{200D}'
                    )
            }
            // Not ၊ ။ (comma and full stop), nor the digits, Burmese's and
            // Shan's; ၌ ၍ ၎ ၏ are words.
            Self::Myanmar => {
                (('\u{1000}'..='\u{109F}').contains(&c)
                    && !('\u{1040}'..='\u{104B}').contains(&c)
                    && !('\u{1090}'..='\u{1099}').contains(&c))
                    || matches!(c, '\u{200C}' | '\u{200D}')
            }
            // Extended-A, and Extended-B but its Tai Laing digits.
            Self::MyanmarSyllables | Self::KarenSyllables => {
                Self::Myanmar.holds(c)
                    || ('\u{AA60}'..='\u{AA7F}').contains(&c)
                    || (('\u{A9E0}'..='\u{A9FF}').contains(&c)
                        && !('\u{A9F0}'..='\u{A9F9}').contains(&c))
            }
            Self::Yi => ('\u{A000}'..='\u{A48C}').contains(&c),
        }
    }

    /// Whether a word may start at `c`, after `before` and before `after`:
    /// not on a vowel sign or a tone mark (marks, and the vowels written
    /// after their consonant: Thai's ะ า ำ ๅ, Lao's ະ າ ຳ and ຽ, and the
    /// abbreviation marks ฯ ຯ), nor right after a vowel written before its
    /// consonant (เ แ โ ใ ไ, ເ ແ ໂ ໃ ໄ); in Khmer and Burmese not on a
    /// consonant joined below the one before (after ្, after ္), nor in
    /// Burmese on one the asat or the virama after it makes the last of a
    /// syllable (the န of ကန်, the မ of the kinzi's own). Shan's, Mon's,
    /// Karen's and Pa'O's syllables start on a consonant stacked over the
    /// next (the ပ of Mon's ပ္ဍဲ), though, but for Karen's, whose asat is a
    /// tone, not on one the asat ends a syllable with.
    fn may_start(self, before: char, c: char, after: Option<char>) -> bool {
        match self {
            Self::Thai => {
                !is_combining_mark(c)
                    && !matches!(
                        c,
                        '\u{0E30}' | '\u{0E32}' | '\u{0E33}' | '\u{0E45}' | '\u{0E2F}'
                    )
                    && !('\u{0E40}'..='\u{0E44}').contains(&before)
            }
            Self::Lao => {
                !is_combining_mark(c)
                    && !matches!(
                        c,
                        '\u{0EB0}' | '\u{0EB2}' | '\u{0EB3}' | '\u{0EBD}' | '\u{0EAF}'
                    )
                    && !('\u{0EC0}'..='\u{0EC4}').contains(&before)
            }
            Self::Khmer => !is_combining_mark(c) && before != COENG && !is_joiner(c),
            Self::Myanmar => {
                !is_combining_mark(c)
                    && before != MYANMAR_VIRAMA
                    && !is_joiner(c)
                    && !matches!(after, Some(ASAT | MYANMAR_VIRAMA))
            }
            Self::MyanmarSyllables => {
                !is_combining_mark(c)
                    && before != MYANMAR_VIRAMA
                    && !is_joiner(c)
                    && after != Some(ASAT)
            }
            Self::KarenSyllables => {
                !is_combining_mark(c) && before != MYANMAR_VIRAMA && !is_joiner(c)
            }
            // Every letter a syllable.
            Self::Yi => true,
        }
    }

    /// Whether `c` alone is a word, though no list holds it: the
    /// repetition mark ๆ (ໆ), written after the word it repeats; and every
    /// syllable of Yi.
    fn is_word_alone(self, c: char) -> bool {
        match self {
            Self::Thai => c == '\u{0E46}',
            Self::Lao => c == '\u{0EC6}',
            Self::Khmer => c == '\u{17D7}',
            Self::Myanmar | Self::MyanmarSyllables | Self::KarenSyllables => {
                ('\u{104C}'..='\u{104F}').contains(&c)
            }
            Self::Yi => true,
        }
    }

    /// Whether the script is cut with no list at all, each of its letters
    /// a word alone: Yi.
    #[must_use]
    pub fn needs_no_list(self) -> bool {
        self == Self::Yi
    }

    /// Whether the script's word lists are its syllables
    /// ([`Self::syllables`]): Shan's, Mon's, Sgaw Karen's, Pa'O's.
    #[must_use]
    pub fn cut_into_syllables(self) -> bool {
        matches!(self, Self::MyanmarSyllables | Self::KarenSyllables)
    }

    /// Where in `run` a word may start ([`Self::may_start`]), in order,
    /// and the run's end; nothing for an empty run.
    fn bounds(self, run: &str) -> Vec<usize> {
        let mut bounds = Vec::new();
        let mut before = None;
        let mut chars = run.char_indices().peekable();
        while let Some((at, c)) = chars.next() {
            let after = chars.peek().map(|&(_, a)| a);
            if before.is_none_or(|b| self.may_start(b, c, after)) {
                bounds.push(at);
            }
            before = Some(c);
        }
        if !bounds.is_empty() {
            bounds.push(run.len());
        }
        bounds
    }

    /// The syllables of `run`, a run of this script's characters
    /// ([`Self::holds`]): what lies between the places a word may start,
    /// as byte ranges of it, in order. The word list of a language cut
    /// into syllables (Shan, Mon, Sgaw Karen, Pa'O, Arakanese) is those its
    /// text writes.
    #[must_use]
    pub fn syllables(self, run: &str) -> Vec<Range<usize>> {
        self.bounds(run).windows(2).map(|w| w[0]..w[1]).collect()
    }
}

/// The zero-width joiner or non-joiner, which joins or parts the letters
/// either side within a word.
fn is_joiner(c: char) -> bool {
    matches!(c, '\u{200C}' | '\u{200D}')
}

/// Lao's letters written two ways, written the way its keyboard types
/// them: ໍ and າ as ຳ (U+0EB3), ຫ before ນ, ມ and ລ as the ligatures ໜ and
/// ໝ and as ຫຼ, the subscript lo. The web writes both (a third of its ຳ is
/// ໍາ), and so does ICU's word list. Other text is left as it is.
#[must_use]
pub fn lao_fold(s: &str) -> Cow<'_, str> {
    if !s.contains(['\u{0ECD}', '\u{0EAB}']) {
        return Cow::Borrowed(s);
    }
    Cow::Owned(
        s.replace("\u{0ECD}\u{0EB2}", "\u{0EB3}")
            .replace("\u{0EAB}\u{0E99}", "\u{0EDC}")
            .replace("\u{0EAB}\u{0EA1}", "\u{0EDD}")
            .replace("\u{0EAB}\u{0EA5}", "\u{0EAB}\u{0EBC}"),
    )
}

/// A language's words, for cutting its text where it has no spaces.
#[derive(Clone, PartialEq, Eq)]
pub struct WordList {
    /// NFC, sorted, each once.
    words: Vec<String>,
}

impl fmt::Debug for WordList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WordList")
            .field("words", &self.words.len())
            .finish()
    }
}

/// Why a word list did not parse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WordListError {
    /// The file holds no word.
    Empty,
    /// A line holds whitespace inside its word (1-based).
    Spaced {
        /// The line.
        line: usize,
    },
}

impl fmt::Display for WordListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "the word list holds no word"),
            Self::Spaced { line } => write!(f, "line {line}: whitespace inside a word"),
        }
    }
}

impl std::error::Error for WordListError {}

impl WordList {
    /// A list from its file: one word per line; blank lines, lines starting
    /// with `#` (after whitespace) and a byte-order mark skipped.
    ///
    /// # Errors
    ///
    /// When the file holds no word, or a line holds two.
    pub fn parse(text: &str) -> Result<Self, WordListError> {
        let text = text.strip_prefix('\u{FEFF}').unwrap_or(text);
        let mut words = Vec::new();
        for (i, line) in text.lines().enumerate() {
            let word = line.trim();
            if word.is_empty() || word.starts_with('#') {
                continue;
            }
            if word.contains(char::is_whitespace) {
                return Err(WordListError::Spaced { line: i + 1 });
            }
            words.push(word);
        }
        if words.is_empty() {
            return Err(WordListError::Empty);
        }
        Ok(Self::from_words(words))
    }

    /// A list of `words`, as given.
    pub fn from_words<I, S>(words: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut words: Vec<String> = words
            .into_iter()
            .map(|w| lao_fold(&w.as_ref().nfc().collect::<String>()).into_owned())
            .filter(|w| !w.is_empty())
            .collect();
        words.sort_unstable();
        words.dedup();
        Self { words }
    }

    /// How many words it holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.words.len()
    }

    /// Whether it holds none.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }

    /// Whether `word` is one of its words.
    #[must_use]
    pub fn contains(&self, word: &str) -> bool {
        nfc_then(word, |w| {
            self.words.binary_search_by(|x| x.as_str().cmp(w)).is_ok()
        })
    }

    /// Whether one of its words starts with `prefix`, or is it.
    #[must_use]
    pub fn begins(&self, prefix: &str) -> bool {
        nfc_then(prefix, |p| {
            let i = self.words.partition_point(|x| x.as_str() < p);
            self.words.get(i).is_some_and(|x| x.starts_with(p))
        })
    }

    /// The words of `run`, a run of `script`'s characters ([`Script::holds`]),
    /// as byte ranges of it, in order. `open`: the run ends at the caret,
    /// so its last word may be one still being typed, the start of a word
    /// of the list.
    #[must_use]
    pub fn cut(&self, script: Script, run: &str, open: bool) -> Vec<Range<usize>> {
        // Where a word may start, and the run's end.
        let bounds = script.bounds(run);
        if bounds.is_empty() {
            return Vec::new();
        }
        let n = bounds.len() - 1;
        // For each bound, the best cut of the rest of the run: the
        // characters it leaves in no word of the list, the words it makes,
        // the bound its first word ends on, and whether that word is known.
        let mut best: Vec<(usize, usize, usize, bool)> = vec![(0, 0, n, true); n + 1];
        for k in (0..n).rev() {
            let start = bounds[k];
            let first = &run[start..bounds[k + 1]];
            let alone =
                first.chars().count() == 1 && first.chars().all(|c| script.is_word_alone(c));
            // The characters up to the next bound, in no word.
            let mut pick = if alone {
                (best[k + 1].0, best[k + 1].1 + 1, k + 1, true)
            } else {
                (
                    best[k + 1].0 + first.chars().count(),
                    best[k + 1].1 + 1,
                    k + 1,
                    false,
                )
            };
            // Each word of the list that starts here and ends on a bound:
            // the longest of those that cut the rest as well.
            for e in k + 1..=n {
                let word = &run[start..bounds[e]];
                if !self.begins(word) {
                    break;
                }
                if self.contains(word) || (open && e == n) {
                    let cut = (best[e].0, best[e].1 + 1, e, true);
                    if (cut.0, cut.1) <= (pick.0, pick.1) {
                        pick = cut;
                    }
                }
            }
            best[k] = pick;
        }
        // The words, those in no word of the list run together as one.
        let mut out: Vec<Range<usize>> = Vec::new();
        let mut last_known = true;
        let mut k = 0;
        while k < n {
            let (_, _, e, known) = best[k];
            let range = bounds[k]..bounds[e];
            match out.last_mut() {
                Some(prev) if !known && !last_known => prev.end = range.end,
                _ => out.push(range),
            }
            last_known = known;
            k = e;
        }
        out
    }
}

/// `f` of `s` in NFC and Lao's letters folded, as the list's words are.
fn nfc_then(s: &str, f: impl Fn(&str) -> bool) -> bool {
    if is_nfc(s) {
        f(&lao_fold(s))
    } else {
        f(&lao_fold(&s.nfc().collect::<String>()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cut(list: &WordList, run: &str, open: bool) -> Vec<String> {
        list.cut(Script::Thai, run, open)
            .into_iter()
            .map(|r| run[r].to_string())
            .collect()
    }

    fn thai() -> WordList {
        WordList::from_words([
            "ฉัน",
            "รัก",
            "รักษา",
            "ภาษา",
            "ไทย",
            "ภาษาไทย",
            "เป็น",
            "ใน",
            "ประเทศ",
            "ประเทศไทย",
            "ราชการ",
            "ไป",
            "โรงเรียน",
            "เด็ก",
            "กิน",
            "ข้าว",
        ])
    }

    #[test]
    fn parses_icu_s_file() {
        let list = WordList::parse(
            "\u{FEFF}  # Copyright (C) 2016 and later: Unicode, Inc. and others.\n\
             # License & terms of use: http://www.unicode.org/copyright.html\n\
             กก\n\nกกขนาก\nกก\n",
        )
        .unwrap();
        assert_eq!(list.len(), 2);
        assert!(list.contains("กกขนาก"));
        assert!(list.begins("กกข"));
        assert!(!list.begins("ขน"));
        assert_eq!(WordList::parse("# none\n\n"), Err(WordListError::Empty));
        assert_eq!(
            WordList::parse("กก\nกก ขน\n"),
            Err(WordListError::Spaced { line: 2 })
        );
    }

    #[test]
    fn the_fewest_words_that_cover_the_run() {
        let list = thai();
        assert_eq!(
            cut(&list, "ภาษาไทยเป็นภาษาราชการในประเทศไทย", false),
            ["ภาษาไทย", "เป็น", "ภาษา", "ราชการ", "ใน", "ประเทศไทย"]
        );
        assert_eq!(cut(&list, "ฉันรักภาษาไทย", false), ["ฉัน", "รัก", "ภาษาไทย"]);
    }

    #[test]
    fn what_no_word_covers_is_one_word() {
        let list = thai();
        // A name: one word, between the words around it.
        assert_eq!(
            cut(&list, "ฉันกินสมชายข้าว", false),
            ["ฉัน", "กิน", "สมชาย", "ข้าว"]
        );
        assert_eq!(cut(&list, "สมชาย", false), ["สมชาย"]);
    }

    #[test]
    fn no_word_starts_on_a_mark_or_after_a_leading_vowel() {
        // The list's ก and ิน would cut กิน in two, but no word starts on
        // the vowel sign ิ: กิน is one word, in no list.
        let list = WordList::from_words(["ก", "ิน"]);
        assert_eq!(cut(&list, "กิน", false), ["กิน"]);
        // Nor on the mark ็: เด็ก cannot be เด and ็ก.
        let list = WordList::from_words(["เด", "็ก"]);
        assert_eq!(cut(&list, "เด็ก", false), ["เด็ก"]);
        // เ is written before its consonant: no word starts right after it.
        let list = WordList::from_words(["เ", "ป็น"]);
        assert_eq!(cut(&list, "เป็น", false), ["เป็น"]);
    }

    #[test]
    fn the_repetition_mark_is_a_word() {
        let list = thai();
        assert_eq!(cut(&list, "เด็กๆ", false), ["เด็ก", "ๆ"]);
    }

    #[test]
    fn an_open_run_ends_in_the_word_being_typed() {
        let list = thai();
        // `รักษ` is no word, but the start of `รักษา`: at the caret, the
        // word being typed, not `รัก` and a word in no list.
        assert_eq!(cut(&list, "ฉันรักษ", true), ["ฉัน", "รักษ"]);
        assert_eq!(cut(&list, "ฉันรักษ", false), ["ฉัน", "รัก", "ษ"]);
        // A whole word at the caret is the word being typed too.
        assert_eq!(cut(&list, "ฉันรัก", true), ["ฉัน", "รัก"]);
    }

    #[test]
    fn text_not_in_nfc_is_found() {
        // Sara u (ุ, combining class 103) typed after the tone mark mai ek
        // (่, 107): NFC puts it first, as the list has it.
        let list = WordList::from_words(["ป\u{0E38}\u{0E48}ม"]);
        assert!(list.contains("ป\u{0E48}\u{0E38}ม"));
        assert!(list.begins("ป\u{0E48}\u{0E38}"));
    }

    #[test]
    fn lao_is_cut_as_thai_is() {
        let list = WordList::from_words([
            "ພາສາ",
            "ລາວ",
            "ພາສາລາວ",
            "ເປັນ",
            "ຂອງ",
            "ປະເທດ",
            "ປະເທດລາວ",
            "ເດັກ",
            "ກິນ",
            "ເຂົ້າ",
        ]);
        let cut = |run: &str| -> Vec<String> {
            list.cut(Script::Lao, run, false)
                .into_iter()
                .map(|r| run[r].to_string())
                .collect()
        };
        assert_eq!(
            cut("ພາສາລາວເປັນພາສາຂອງປະເທດລາວ"),
            ["ພາສາລາວ", "ເປັນ", "ພາສາ", "ຂອງ", "ປະເທດລາວ"]
        );
        // No word starts after ເ, nor on ະ or ັ; ໆ is a word of its own.
        assert_eq!(cut("ເດັກໆກິນເຂົ້າ"), ["ເດັກ", "ໆ", "ກິນ", "ເຂົ້າ"]);
        let list = WordList::from_words(["ເ", "ດັກ", "ປ", "ະ"]);
        assert_eq!(
            list.cut(Script::Lao, "ເດັກ", false).len(),
            1,
            "ເດັກ is one word"
        );
        assert_eq!(
            list.cut(Script::Lao, "ປະ", false).len(),
            1,
            "ປະ is one word"
        );
    }

    #[test]
    fn khmer_is_cut_where_its_words_may_start() {
        let list = WordList::from_words(["ខ្ញុំ", "ស្រលាញ់", "ភាសា", "ខ្មែរ", "ខ", "ញុំ", "ៗ"]);
        let cut = |run: &str| -> Vec<String> {
            list.cut(Script::Khmer, run, false)
                .into_iter()
                .map(|r| run[r].to_string())
                .collect()
        };
        assert_eq!(cut("ខ្ញុំស្រលាញ់ភាសាខ្មែរ"), ["ខ្ញុំ", "ស្រលាញ់", "ភាសា", "ខ្មែរ"]);
        // No word starts on the consonant the coeng writes below another:
        // ខ្ញុំ is not ខ and ញុំ.
        let list = WordList::from_words(["ខ", "ញុំ"]);
        assert_eq!(list.cut(Script::Khmer, "ខ្ញុំ", false).len(), 1);
        // ៗ repeats the word before it, and ends no run; ។ ends one.
        assert!(Script::Khmer.holds('ៗ') && !Script::Khmer.holds('។'));
        assert!(!Script::Khmer.holds('\u{200B}') && Script::Khmer.holds('\u{200C}'));
    }

    #[test]
    fn burmese_is_cut_at_its_syllables_starts() {
        let list = WordList::from_words(["မြန်မာ", "စာ", "မင်္ဂလာ", "ပါ", "သူ"]);
        let cut = |run: &str| -> Vec<String> {
            list.cut(Script::Myanmar, run, false)
                .into_iter()
                .map(|r| run[r].to_string())
                .collect()
        };
        assert_eq!(cut("မြန်မာစာ"), ["မြန်မာ", "စာ"]);
        assert_eq!(cut("မင်္ဂလာပါ"), ["မင်္ဂလာ", "ပါ"]);
        // ၏ is a word, though no list holds it.
        assert_eq!(cut("သူ၏"), ["သူ", "၏"]);
        // A consonant its asat or virama makes the last of a syllable
        // starts no word (the န of မြန်), nor does one stacked under it.
        let list = WordList::from_words(["မြ", "န်မာ", "ဂလာ"]);
        assert_eq!(list.cut(Script::Myanmar, "မြန်မာ", false).len(), 1);
        assert_eq!(list.cut(Script::Myanmar, "မင်္ဂလာ", false).len(), 1);
        // ။ and the digits end a run.
        assert!(!Script::Myanmar.holds('။') && !Script::Myanmar.holds('၁'));
        assert_eq!(Script::of_lang("my"), Some(Script::Myanmar));
        assert_eq!(Script::of_lang("km"), Some(Script::Khmer));
    }

    #[test]
    fn shan_mon_karen_and_pao_are_cut_into_syllables() {
        let syllables = |script: Script, run: &str| -> Vec<String> {
            script
                .syllables(run)
                .into_iter()
                .map(|r| run[r].to_string())
                .collect()
        };
        let s = Script::MyanmarSyllables;
        // Shan: a syllable ends after its final's asat and its tone.
        assert_eq!(syllables(s, "ၵႂၢမ်းတႆး"), ["ၵႂၢမ်း", "တႆး"]);
        // Mon: a consonant stacked over the next starts a syllable, a
        // minor syllable (ပ္ဍဲ); one the asat ends a syllable with does
        // not (ဍုၚ်).
        assert_eq!(syllables(s, "ဍုၚ်ပ္ဍဲ"), ["ဍုၚ်", "ပ္ဍဲ"]);
        // Burmese's kinzi stays in its syllable.
        assert_eq!(syllables(s, "မင်္ဂလာ"), ["မင်္ဂ", "လာ"]);
        // Pa'O's tone in Extended-A, after its final's asat.
        assert_eq!(syllables(s, "နဝ်ꩻဖုံႏ"), ["နဝ်ꩻ", "ဖုံႏ"]);
        // Sgaw Karen's signs; its asat a tone, ending no consonant: ဒ်
        // (like) is a syllable.
        let k = Script::KarenSyllables;
        assert_eq!(syllables(k, "တၢ်ကညီ"), ["တၢ်", "က", "ညီ"]);
        assert_eq!(syllables(k, "လဲၤဒ်ဆံး"), ["လဲၤ", "ဒ်", "ဆံး"]);
        assert_eq!(syllables(s, "လဲၤဒ်ဆံး"), ["လဲၤဒ်", "ဆံး"]);
        assert!(k.holds('ၢ') && k.cut_into_syllables() && s.cut_into_syllables());
        assert!(!Script::Myanmar.cut_into_syllables());
        assert!(s.holds('ꩻ') && s.holds('\u{A9E0}') && !s.holds('\u{A9F1}'));
        assert!(!s.holds('။') && !s.holds('၁') && !s.holds('႑'));
        // Burmese is cut as it was: no syllable starts on the ပ of ပ္ဍဲ.
        assert_eq!(syllables(Script::Myanmar, "ဍုၚ်ပ္ဍဲ"), ["ဍုၚ်ပ္ဍဲ"]);
        assert!(!Script::Myanmar.holds('ꩻ'));
        assert_eq!(s.syllables(""), []);
        // Arakanese writes Burmese's letters, its Pali stacked as Pa'O's is.
        assert_eq!(syllables(s, "ရခိုင်ဘာသာ"), ["ရ", "ခိုင်", "ဘာ", "သာ"]);
        assert_eq!(syllables(s, "ကမ္ဘာ"), ["က", "မ္ဘာ"]);
        for lang in ["shn", "mnw", "blk", "rki"] {
            assert_eq!(Script::of_lang(lang), Some(s), "{lang}");
        }
        assert_eq!(Script::of_lang("ksw"), Some(k));
        // Their word lists are their syllables: a syllable no list holds
        // runs on with the next unknown one, as a word does.
        let list = WordList::from_words(["ၵႂၢမ်း", "တႆး"]);
        assert_eq!(
            list.cut(s, "ၵႂၢမ်းတႆး", false)
                .into_iter()
                .map(|r| &"ၵႂၢမ်းတႆး"[r])
                .collect::<Vec<_>>(),
            ["ၵႂၢမ်း", "တႆး"]
        );
    }

    #[test]
    fn lao_written_two_ways_is_one() {
        assert_eq!(lao_fold("ສໍາລັບ"), "ສຳລັບ");
        assert_eq!(lao_fold("ນ້ໍາ"), "ນ້ຳ");
        assert_eq!(lao_fold("ຫນ້າ ຫມາກ ຫລາຍ"), "ໜ້າ ໝາກ ຫຼາຍ");
        assert!(matches!(lao_fold("ບໍ່"), Cow::Owned(ref w) if w == "ບໍ່"));
        assert!(matches!(lao_fold("ภาษาไทย"), Cow::Borrowed(_)));
        // A word the list has one way is found written the other, in a
        // run of the other.
        let list = WordList::from_words(["ສໍາລັບ", "ໜ້າ", "ເດັກ"]);
        assert!(list.contains("ສຳລັບ") && list.contains("ສໍາລັບ"));
        assert!(list.contains("ຫນ້າ") && list.begins("ຫນ"));
        let run = "ສຳລັບເດັກຫນ້າ";
        let words: Vec<&str> = list
            .cut(Script::Lao, run, false)
            .into_iter()
            .map(|r| &run[r])
            .collect();
        assert_eq!(words, ["ສຳລັບ", "ເດັກ", "ຫນ້າ"]);
    }

    #[test]
    fn the_script_s_characters() {
        assert!(Script::Thai.holds('ก'));
        assert!(Script::Thai.holds('ๆ'));
        assert!(!Script::Thai.holds('๑'));
        assert!(!Script::Thai.holds('฿'));
        assert!(!Script::Thai.holds('a'));
        assert_eq!(Script::of_lang("th-TH"), Some(Script::Thai));
        assert_eq!(Script::of_lang("en"), None);
        assert!(Script::Lao.holds('ກ') && Script::Lao.holds('ໆ') && Script::Lao.holds('ໜ'));
        assert!(!Script::Lao.holds('໑') && !Script::Lao.holds('ก'));
        assert_eq!(Script::of_lang("lo"), Some(Script::Lao));
    }
}
