//! Dialect flags: which varieties of a language use a word, and a word's
//! spellings across them (`color` and `colour`, `fato` and `facto`), so
//! that one pack serves en-US and en-GB, pt-BR and pt-PT.
//!
//! A language's table (`predict/<lang>.dialects.bin`, [`Dialects`]) lists
//! only the words whose use differs between its varieties; a word it does
//! not list is everyone's. Each listed word says which varieties write it
//! so usually and which accept it, and, when the same word is spelled
//! otherwise elsewhere, its group of spellings. A keyboard set to one
//! variety offers that variety's spelling of a word, borrowing how common
//! the word is from whichever spelling the lexicon holds, and leaves alone
//! the spellings that variety accepts.

use serde::{Deserialize, Serialize};

/// The varieties of `lang` that dialect data distinguishes, in the order
/// of the bits of a [`DialectWord`]'s masks: BCP 47 region subtags, and
/// `419` for Latin America. Empty for a language without any.
#[must_use]
pub fn varieties(lang: &str) -> &'static [&'static str] {
    match lang.split(['-', '_']).next().unwrap_or(lang) {
        "en" => &["US", "GB", "CA", "AU", "IN"],
        "pt" => &["BR", "PT"],
        "es" => &["ES", "419"],
        "fr" => &["FR", "CA", "BE", "CH"],
        _ => &[],
    }
}

/// A word whose use differs between the varieties.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialectWord {
    /// The word, as the lexicon spells it ([`crate::words::Splitter::normalize`]).
    pub word: String,
    /// The varieties that write it so usually, one bit each
    /// ([`Dialects::varieties`]).
    pub usual: u32,
    /// The varieties that accept it: those that write it usually, and
    /// those that accept it as a less usual spelling.
    pub accepted: u32,
    /// Its group of spellings in [`Dialects::groups`], when the word is
    /// spelled otherwise in some variety.
    pub group: Option<u32>,
}

/// One language's dialect flags.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Dialects {
    /// The varieties, in the order of the masks' bits.
    pub varieties: Vec<String>,
    /// The words whose use differs between the varieties, sorted by word.
    pub words: Vec<DialectWord>,
    /// The spellings of one word across the varieties, each as indices
    /// into [`Self::words`].
    pub groups: Vec<Vec<u32>>,
}

impl Dialects {
    /// Serialize, framed by [`crate::codec`].
    ///
    /// # Errors
    /// As [`crate::codec::encode`].
    pub fn to_bytes(&self) -> Result<Vec<u8>, crate::codec::Error> {
        crate::codec::encode(self)
    }

    /// Read a table written by [`Self::to_bytes`].
    ///
    /// # Errors
    /// [`crate::codec::Error`] when the bytes are not one.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, crate::codec::Error> {
        crate::codec::decode(bytes)
    }

    /// The bit of `region` (`GB`, `BR`, `419`), if the table has it.
    #[must_use]
    pub fn variety(&self, region: &str) -> Option<usize> {
        self.varieties
            .iter()
            .position(|v| v.eq_ignore_ascii_case(region))
    }

    /// `word`'s flags, when its use differs between the varieties.
    #[must_use]
    pub fn get(&self, word: &str) -> Option<&DialectWord> {
        self.words
            .binary_search_by(|w| w.word.as_str().cmp(word))
            .ok()
            .map(|i| &self.words[i])
    }

    /// Whether the variety with bit `variety` accepts `word`: every word
    /// the table does not list.
    #[must_use]
    pub fn accepts(&self, word: &str, variety: usize) -> bool {
        self.get(word)
            .is_none_or(|w| w.accepted & (1 << variety) != 0)
    }

    /// The other spellings of `word`, itself included, when it has any.
    pub fn spellings(&self, word: &str) -> impl Iterator<Item = &DialectWord> {
        self.get(word)
            .and_then(|w| w.group)
            .and_then(|g| self.groups.get(g as usize))
            .into_iter()
            .flatten()
            .filter_map(|&i| self.words.get(i as usize))
    }

    /// How the variety with bit `variety` usually spells `word`, when that
    /// is not `word` itself and the variety does not accept `word` as it
    /// is either (`color` is `colour` in en-GB; Canadian, which accepts
    /// both, keeps what was typed).
    #[must_use]
    pub fn spelling_in(&self, word: &str, variety: usize) -> Option<&str> {
        if self.accepts(word, variety) {
            return None;
        }
        self.spellings(word)
            .find(|w| w.word != word && w.usual & (1 << variety) != 0)
            .map(|w| w.word.as_str())
    }

    /// The spelling the variety with bit `variety` usually writes for a
    /// word it does not write as `word`: as [`Self::spelling_in`], but
    /// also for a word it accepts, as the strip would offer it (`color`
    /// is offered as `colour` in en-CA).
    #[must_use]
    pub fn usual_spelling(&self, word: &str, variety: usize) -> Option<&str> {
        let w = self.get(word)?;
        if w.usual & (1 << variety) != 0 {
            return None;
        }
        self.spellings(word)
            .find(|s| s.word != word && s.usual & (1 << variety) != 0)
            .map(|s| s.word.as_str())
    }

    /// What is wrong with the table, if anything: its varieties are not
    /// `lang`'s ([`varieties`]), its words are not sorted or repeat, a
    /// mask names a variety it does not have, a word accepted nowhere, or
    /// a group refers to a word not listed or not in that group.
    #[must_use]
    pub fn problems(&self, lang: &str) -> Vec<String> {
        let mut out = Vec::new();
        if self.varieties != varieties(lang) {
            out.push(format!(
                "varieties {:?}, not {:?}",
                self.varieties,
                varieties(lang)
            ));
        }
        let all = (1u32 << self.varieties.len()) - 1;
        for pair in self.words.windows(2) {
            if pair[0].word >= pair[1].word {
                out.push(format!("{:?} before {:?}", pair[0].word, pair[1].word));
            }
        }
        for w in &self.words {
            if w.usual & !w.accepted != 0 || w.accepted & !all != 0 || w.accepted == 0 {
                out.push(format!(
                    "{:?}: usual {:b}, accepted {:b}",
                    w.word, w.usual, w.accepted
                ));
            }
        }
        for (g, members) in self.groups.iter().enumerate() {
            for &i in members {
                let ok = self
                    .words
                    .get(i as usize)
                    .is_some_and(|w| w.group == Some(u32::try_from(g).unwrap_or(u32::MAX)));
                if !ok {
                    out.push(format!("group {g} holds word {i}, which is not in it"));
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// en: `color` (US, a variant in Canada), `colour` (GB, CA, AU, IN),
    /// `realize` (US, accepted in GB), `realise` (GB, AU, IN), `lakh` (IN).
    fn sample() -> Dialects {
        let (us, gb, ca, au, ind) = (1, 2, 4, 8, 16);
        let w = |word: &str, usual: u32, accepted: u32, group: Option<u32>| DialectWord {
            word: word.into(),
            usual,
            accepted,
            group,
        };
        Dialects {
            varieties: varieties("en").iter().map(ToString::to_string).collect(),
            words: vec![
                w("color", us, us | ca, Some(0)),
                w("colour", gb | ca | au | ind, gb | ca | au | ind, Some(0)),
                w("lakh", ind, ind, None),
                w("realise", gb | au | ind, gb | au | ind, Some(1)),
                w("realize", us | ca, us | ca | gb, Some(1)),
            ],
            groups: vec![vec![0, 1], vec![3, 4]],
        }
    }

    #[test]
    fn each_variety_writes_its_own_spelling() {
        let d = sample();
        let (us, gb, ca) = (0, 1, 2);
        assert_eq!(d.variety("gb"), Some(gb));
        assert_eq!(d.spelling_in("color", gb), Some("colour"));
        assert_eq!(d.spelling_in("colour", us), Some("color"));
        // A spelling the variety accepts stays as typed, though the strip
        // offers the usual one.
        assert_eq!(d.spelling_in("color", ca), None);
        assert_eq!(d.usual_spelling("color", ca), Some("colour"));
        assert_eq!(d.spelling_in("realize", gb), None);
        assert_eq!(d.usual_spelling("realize", gb), Some("realise"));
        // A word everyone uses, and one with no other spelling.
        assert_eq!(d.spelling_in("the", gb), None);
        assert!(d.accepts("the", us));
        assert!(!d.accepts("lakh", us));
        assert_eq!(d.spelling_in("lakh", us), None);
        assert!(d.problems("en").is_empty(), "{:?}", d.problems("en"));
    }

    #[test]
    fn a_table_round_trips_and_is_checked() {
        let d = sample();
        assert_eq!(Dialects::from_bytes(&d.to_bytes().unwrap()).unwrap(), d);
        assert!(
            !d.problems("pt").is_empty(),
            "an English table passes as Portuguese"
        );
        let mut bad = d.clone();
        bad.words.swap(0, 1);
        assert!(
            !bad.problems("en").is_empty(),
            "a table with two words out of order passes"
        );
        let mut bad = d;
        bad.groups[0].push(2);
        assert!(
            !bad.problems("en").is_empty(),
            "a group holding a word not in it passes"
        );
    }
}
