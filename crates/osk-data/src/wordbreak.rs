//! Word lists for the languages written without spaces between words,
//! Thai and Lao: what [`crate::words::Splitter`] cuts their runs of letters
//! into words with, the same way in the trainer, the keyboard and the
//! measure.
//!
//! A list is a plain text file, one word per line, as ICU's word-break
//! dictionaries are (`thaidict.txt`, `laodict.txt`): blank lines, lines starting with `#`
//! and a byte-order mark are skipped. It has no frequencies, so a run is
//! cut into the fewest words of the list that cover as much of it as they
//! can (maximal matching), the earlier word the longer where two cuts tie.
//! What no word of the list covers is a word of its own, as long as it
//! runs. A word starts only where the script lets one start: never before
//! a vowel sign or a tone mark, nor right after a vowel written before its
//! consonant. Lao's letters written two ways ([`lao_fold`]) are one in the
//! list and in what is looked up in it.

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
}

impl Script {
    /// The script `lang` (a BCP-47 tag) is written in, when it is one of
    /// these: `th`, `lo`.
    #[must_use]
    pub fn of_lang(lang: &str) -> Option<Self> {
        match lang.split(['-', '_']).next().unwrap_or(lang) {
            "th" => Some(Self::Thai),
            "lo" => Some(Self::Lao),
            _ => None,
        }
    }

    /// Whether `c` is one of this script's characters, which its runs are
    /// made of. Its digits, its currency sign and its punctuation end a
    /// run, as they end a word anywhere.
    #[must_use]
    pub fn holds(self, c: char) -> bool {
        match self {
            Self::Thai => ('\u{0E01}'..='\u{0E4E}').contains(&c) && c != '\u{0E3F}',
            Self::Lao => {
                ('\u{0E81}'..='\u{0EDF}').contains(&c) && !('\u{0ED0}'..='\u{0ED9}').contains(&c)
            }
        }
    }

    /// Whether a word may start at `c`, after `before`: not on a vowel
    /// sign or a tone mark (marks, and the vowels written after their
    /// consonant: Thai's ะ า ำ ๅ, Lao's ະ າ ຳ and ຽ, and the abbreviation
    /// marks ฯ ຯ), nor right after a vowel written before its consonant
    /// (เ แ โ ใ ไ, ເ ແ ໂ ໃ ໄ).
    fn may_start(self, before: char, c: char) -> bool {
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
        }
    }

    /// Whether `c` alone is a word, though no list holds it: the
    /// repetition mark ๆ (ໆ), written after the word it repeats.
    fn is_word_alone(self, c: char) -> bool {
        match self {
            Self::Thai => c == '\u{0E46}',
            Self::Lao => c == '\u{0EC6}',
        }
    }
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
        let mut bounds = Vec::new();
        let mut before = None;
        for (at, c) in run.char_indices() {
            if before.is_none_or(|b| script.may_start(b, c)) {
                bounds.push(at);
            }
            before = Some(c);
        }
        if bounds.is_empty() {
            return Vec::new();
        }
        bounds.push(run.len());
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
