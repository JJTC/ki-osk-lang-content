//! Pinyin's structure, and the two tables a Pinyin composer reads beside
//! its dictionary: the syllable inventory (`pinyin/syllables.txt`) and the
//! double-Pinyin schemes (`pinyin/double-pinyin.json`, [`double`]).
//!
//! The initials and finals are Pinyin's own orthography. The inventory
//! is every syllable they make that Mandarin has, and the schemes map
//! keys to them: both are data, released with the dictionary they serve.

use std::fmt;

/// The 23 standard initials, ordered so the digraphs `zh ch sh` match
/// before their single-letter prefixes.
pub const INITIALS: &[&str] = &[
    "zh", "ch", "sh", "b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h", "j", "q", "x", "r",
    "z", "c", "s", "y", "w",
];

/// The standard finals, ordered longest-first so the greedy match takes
/// `iang` before `ia`.
///
/// `ue` and `ei` were both **missing** here until 2026-08-05, which split
/// `xue` as `xu`+`e` and `beijing` as `b`+`e`+`i`+`jing`. That was
/// invisible for as long as the only consumer was the dictionary lookup,
/// which rejoins the tokens into one key and so gets the same string
/// either way — but double Pinyin and fuzzy matching operate on the tokens
/// themselves, where a wrong split is a wrong lookup. `ue` is the spelling
/// `üe` takes after `j q x y`.
pub const FINALS: &[&str] = &[
    "iang", "iong", "uang", "ueng", "ang", "eng", "ing", "ong", "iao", "ian", "uai", "uan", "üan",
    "ai", "ao", "an", "ou", "ei", "en", "er", "ia", "ie", "iu", "in", "ua", "uo", "ui", "un", "ue",
    "üe", "ün", "a", "o", "e", "i", "u", "ü",
];

/// Greedy left-to-right split of a toneless Pinyin string into syllables.
/// Recognises the standard initials (including the digraphs zh/ch/sh) and
/// falls back to "initial = empty" when no consonant starts. A character
/// that starts neither is a chunk of its own.
#[must_use]
pub fn parse_syllables(s: &str) -> Vec<String> {
    let lower = s.to_ascii_lowercase();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < lower.len() {
        let remaining = &lower[i..];
        let init_len = INITIALS
            .iter()
            .find_map(|p| remaining.starts_with(p).then_some(p.len()))
            .unwrap_or(0);
        let after_init = &remaining[init_len..];
        let final_len = FINALS
            .iter()
            .find_map(|p| after_init.starts_with(p).then_some(p.len()))
            .unwrap_or(0);

        if init_len == 0 && final_len == 0 {
            // Unrecognised codepoint — take exactly one char (not one
            // byte: the FINALS table includes 'ü' which is 2 bytes in
            // UTF-8, so an unrecognised 'ü' followed by a sliced
            // continuation byte would panic when sliced).
            let ch_len = remaining.chars().next().map_or(1, char::len_utf8);
            let end = i + ch_len;
            out.push(lower[i..end].to_string());
            i = end;
            continue;
        }
        let end = i + init_len + final_len;
        out.push(lower[i..end].to_string());
        i = end;
    }
    out
}

/// Split one syllable into `(initial, final)`, borrowing from the input.
///
/// The initial is `""` for the vowel-initial syllables (`an`, `er`, `ou`).
/// Returns `None` for anything that is not a structurally well-formed
/// syllable, which is a weaker check than [`Syllables::contains`] — `fiao`
/// splits, but is not a syllable.
#[must_use]
pub fn split_initial_final(s: &str) -> Option<(&str, &str)> {
    for init in INITIALS {
        if let Some(rest) = s.strip_prefix(init)
            && FINALS.contains(&rest)
        {
            return Some((&s[..init.len()], rest));
        }
    }
    FINALS.contains(&s).then_some(("", s))
}

/// What is wrong with a Pinyin table.
#[derive(Debug)]
pub enum PinyinError {
    /// The double-Pinyin schemes are not valid JSON of their shape.
    Json(serde_json::Error),
    /// A table breaks a rule of its format.
    Invalid(String),
}

impl fmt::Display for PinyinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(e) => write!(f, "double-Pinyin schemes: {e}"),
            Self::Invalid(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for PinyinError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(e) => Some(e),
            Self::Invalid(_) => None,
        }
    }
}

/// Every Mandarin syllable in toneless Pinyin (`pinyin/syllables.txt`, one
/// a line, sorted): what can be typed, as opposed to what merely splits
/// into an initial and a final.
///
/// Derived from CC-CEDICT: each entry's reading split on spaces, tone
/// digits dropped, `u:`/`v` written `ü`, and kept only if it splits into
/// [`INITIALS`] + [`FINALS`]. That filter removes CC-CEDICT's English
/// loanwords and abbreviations (`coser`, `cp`, `call`) without also
/// removing the rare syllables a frequency cutoff would (`den`, `dia`,
/// `chua`, `rua`, `nou`, `kei`). The interjections with no such structure
/// (`m`, `n`, `ng`, `hm`, `hng`) are left out: nothing can type them, and
/// admitting them would only weaken [`Self::contains`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Syllables {
    sorted: Vec<String>,
}

impl Syllables {
    /// Read the inventory: one syllable a line, sorted and without
    /// repeats, each an initial and a final.
    ///
    /// # Errors
    /// A line that is not such a syllable, or out of order, or repeated;
    /// or no syllable at all.
    pub fn parse(text: &str) -> Result<Self, PinyinError> {
        let mut sorted: Vec<String> = Vec::new();
        for (n, line) in text.lines().enumerate() {
            let s = line.trim();
            if split_initial_final(s).is_none() {
                return Err(PinyinError::Invalid(format!(
                    "syllables line {}: {s:?} is not an initial and a final",
                    n + 1
                )));
            }
            if let Some(prev) = sorted.last()
                && prev.as_str() >= s
            {
                return Err(PinyinError::Invalid(format!(
                    "syllables line {}: {s:?} after {prev:?}: not sorted, or repeated",
                    n + 1
                )));
            }
            sorted.push(s.to_string());
        }
        if sorted.is_empty() {
            return Err(PinyinError::Invalid("syllables: none".into()));
        }
        Ok(Self { sorted })
    }

    /// Whether `s` is a Mandarin syllable in toneless Pinyin.
    #[must_use]
    pub fn contains(&self, s: &str) -> bool {
        self.sorted
            .binary_search_by(|probe| probe.as_str().cmp(s))
            .is_ok()
    }

    /// Every syllable, in order.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.sorted.iter().map(String::as_str)
    }

    /// How many syllables there are.
    #[must_use]
    pub fn len(&self) -> usize {
        self.sorted.len()
    }

    /// Whether there are none (never, for a parsed inventory).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sorted.is_empty()
    }
}

/// Double Pinyin (双拼): one keystroke for the initial, one for the final.
///
/// Full Pinyin spells `zhuang` in six keys; double Pinyin spells every
/// syllable in exactly two — one key for `zh`, one carrying `uang`. On a
/// touch panel, where every key is a deliberate tap rather than a finger
/// already resting on home row, halving the taps is worth more than it is
/// on a physical keyboard.
///
/// A composer applies it as a **pre-transform**: the typed keys are
/// expanded into ordinary Pinyin ([`Table::expand`]) and handed to the
/// parser and dictionary it already has.
///
/// ## The tables
///
/// Each scheme is a published keyboard mapping — which key carries which
/// initial and which finals — as specified by its authors: data about a
/// layout, in the same category as an xkb symbols file. The six here were
/// transcribed from the schemes' definitions as tabulated by libpinyin,
/// then **verified independently** rather than trusted: no libpinyin code,
/// structure or generated file is copied. A wrong entry is a syllable a
/// user simply cannot type, and it would be invisible to any test that
/// spot-checks a few words — there are 413 syllables and each table has
/// ~27 entries — so the committed tables are checked exhaustively
/// (`tests/pinyin.rs`): every syllable of the inventory is encoded to its
/// two keys and decoded back, for every scheme. That passes for all six
/// with exactly one exception, [`UNREACHABLE`].
pub mod double {
    use std::collections::BTreeMap;

    use serde::Deserialize;

    use super::{PinyinError, Syllables, split_initial_final};

    /// A double-Pinyin keyboard scheme.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Scheme {
        /// 小鹤双拼 — the most widely used modern scheme.
        Xiaohe,
        /// 微软双拼 — ships with Microsoft IME; the most widely *installed*.
        Microsoft,
        /// 自然码 — the original, and the ancestor of most of the others.
        Ziranma,
        /// 智能ABC.
        Abc,
        /// 紫光拼音.
        Ziguang,
        /// 拼音加加.
        Pyjj,
    }

    /// Every scheme, for CLI help and for the exhaustive checks.
    pub const ALL_SCHEMES: &[Scheme] = &[
        Scheme::Xiaohe,
        Scheme::Microsoft,
        Scheme::Ziranma,
        Scheme::Abc,
        Scheme::Ziguang,
        Scheme::Pyjj,
    ];

    /// The one syllable no scheme can express.
    ///
    /// `lo` (咯) collides with `luo` on every scheme here: both are spelled
    /// with the same two keys, and `luo` — being enormously more common —
    /// is what the key pair has to mean. This is a property of double
    /// Pinyin itself, not of these tables, and mainstream double-Pinyin
    /// IMEs have the same hole. It is named so the round-trip check can
    /// assert *exactly* one exception rather than tolerate an open-ended
    /// allowlist.
    pub const UNREACHABLE: &[&str] = &["lo"];

    impl Scheme {
        /// Parse a scheme name as spelled on the command line.
        #[must_use]
        pub fn from_name(s: &str) -> Option<Self> {
            match s.to_ascii_lowercase().as_str() {
                "xiaohe" | "flypy" | "xhe" => Some(Self::Xiaohe),
                "microsoft" | "mspy" | "ms" => Some(Self::Microsoft),
                "ziranma" | "zrm" | "natural" => Some(Self::Ziranma),
                "abc" => Some(Self::Abc),
                "ziguang" | "zgpy" => Some(Self::Ziguang),
                "pyjj" | "jiajia" => Some(Self::Pyjj),
                _ => None,
            }
        }

        /// The name used on the command line, and in the schemes file.
        #[must_use]
        pub fn name(self) -> &'static str {
            match self {
                Self::Xiaohe => "xiaohe",
                Self::Microsoft => "microsoft",
                Self::Ziranma => "ziranma",
                Self::Abc => "abc",
                Self::Ziguang => "ziguang",
                Self::Pyjj => "pyjj",
            }
        }
    }

    /// The 27 keys a scheme assigns: `a`..`z` then `;`.
    pub const KEYS: &str = "abcdefghijklmnopqrstuvwxyz;";

    fn key_index(c: char) -> Option<usize> {
        KEYS.find(c)
    }

    /// One scheme's mapping.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Table {
        /// The initial each key carries, by position in [`KEYS`]. `'`
        /// marks the zero-initial key — the one that stands in front of a
        /// vowel-initial syllable.
        sheng: Vec<Option<String>>,
        /// Up to two finals per key. Which one applies is decided by
        /// whether it forms a real syllable with the initial typed with it.
        ///
        /// Spelled in the schemes' own convention, not in Pinyin's: `v` is
        /// `ü`, and `ue` covers both `ue` and `üe`. [`compose`] converts.
        yun: Vec<Vec<String>>,
        /// Whole two-key spellings for vowel-initial syllables, in the
        /// schemes that give them dedicated pairs (`ah` → `ang`) instead of
        /// routing them through the zero-initial key.
        pairs: Vec<(String, String)>,
    }

    /// One scheme as `pinyin/double-pinyin.json` spells it: key → initial,
    /// key → finals, and two keys → a whole syllable.
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct TableJson {
        initials: BTreeMap<String, String>,
        finals: BTreeMap<String, Vec<String>>,
        #[serde(default)]
        pairs: BTreeMap<String, String>,
    }

    /// Every scheme's table (`pinyin/double-pinyin.json`).
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Schemes {
        tables: BTreeMap<Scheme, Table>,
    }

    impl Schemes {
        /// Read the schemes file: an object holding, under each scheme's
        /// [`Scheme::name`], its `initials` (key → initial, `'` for the
        /// zero-initial key), its `finals` (key → one or two finals) and
        /// its `pairs` (two keys → a whole vowel-initial syllable).
        ///
        /// # Errors
        /// JSON of another shape; a scheme missing or unknown; a key that
        /// is not one of [`KEYS`]; an initial that is not one of
        /// [`super::INITIALS`] or `'`; a key with no final or more than
        /// two; a pair that is not two keys.
        pub fn parse(json: &str) -> Result<Self, PinyinError> {
            let raw: BTreeMap<String, TableJson> =
                serde_json::from_str(json).map_err(PinyinError::Json)?;
            let mut tables = BTreeMap::new();
            for (name, t) in raw {
                let scheme = ALL_SCHEMES
                    .iter()
                    .copied()
                    .find(|s| s.name() == name)
                    .ok_or_else(|| PinyinError::Invalid(format!("unknown scheme {name:?}")))?;
                tables.insert(scheme, Table::from_json(&name, t)?);
            }
            if let Some(missing) = ALL_SCHEMES.iter().find(|s| !tables.contains_key(s)) {
                return Err(PinyinError::Invalid(format!(
                    "no table for {}",
                    missing.name()
                )));
            }
            Ok(Self { tables })
        }

        /// The table of `scheme`.
        #[must_use]
        pub fn table(&self, scheme: Scheme) -> &Table {
            // `parse` refuses a file without every scheme.
            &self.tables[&scheme]
        }
    }

    impl Table {
        fn from_json(name: &str, t: TableJson) -> Result<Self, PinyinError> {
            let bad = |msg: String| PinyinError::Invalid(format!("{name}: {msg}"));
            let index = |key: &str| -> Result<usize, PinyinError> {
                let mut cs = key.chars();
                match (cs.next(), cs.next()) {
                    (Some(c), None) => key_index(c),
                    _ => None,
                }
                .ok_or_else(|| bad(format!("{key:?} is not one of the keys {KEYS:?}")))
            };
            let mut sheng = vec![None; KEYS.len()];
            for (key, initial) in t.initials {
                if initial != "'" && !super::INITIALS.contains(&initial.as_str()) {
                    return Err(bad(format!("{key:?} carries {initial:?}, not an initial")));
                }
                sheng[index(&key)?] = Some(initial);
            }
            let mut yun = vec![Vec::new(); KEYS.len()];
            for (key, finals) in t.finals {
                if finals.is_empty() || finals.len() > 2 {
                    return Err(bad(format!(
                        "{key:?} carries {} finals, not one or two",
                        finals.len()
                    )));
                }
                yun[index(&key)?] = finals;
            }
            let mut pairs = Vec::new();
            for (keys, syllable) in t.pairs {
                if keys.chars().count() != 2 || !keys.chars().all(|c| key_index(c).is_some()) {
                    return Err(bad(format!("pair {keys:?} is not two keys")));
                }
                pairs.push((keys, syllable));
            }
            Ok(Self { sheng, yun, pairs })
        }

        /// Decode two keys into the full-Pinyin syllable they spell, one of
        /// `syllables`.
        ///
        /// `None` when the pair spells nothing in this scheme. Callers
        /// should treat that as "not a syllable" rather than an error —
        /// [`Self::expand`] echoes the keys through instead of dropping
        /// them.
        #[must_use]
        pub fn decode_syllable(&self, syllables: &Syllables, a: char, b: char) -> Option<String> {
            let pair: String = [a, b].iter().collect();
            if let Some((_, syl)) = self.pairs.iter().find(|(p, _)| *p == pair) {
                return Some(syl.clone());
            }
            let initial = match self.sheng[key_index(a)?].as_deref()? {
                "'" => "",
                other => other,
            };
            self.yun[key_index(b)?]
                .iter()
                .map(|fin| compose(initial, fin))
                .find(|s| syllables.contains(s))
        }

        /// Encode a full-Pinyin syllable into the two keys that spell it.
        ///
        /// For the round-trip check and for anything that wants to show a
        /// user which keys a syllable takes. `None` for a syllable outside
        /// the standard inventory.
        #[must_use]
        pub fn encode_syllable(&self, syllable: &str) -> Option<(char, char)> {
            if let Some((pair, _)) = self.pairs.iter().find(|(_, s)| s == syllable) {
                let mut cs = pair.chars();
                return Some((cs.next()?, cs.next()?));
            }
            let (initial, fin) = split_initial_final(syllable)?;
            let want = if initial.is_empty() { "'" } else { initial };
            let a = self
                .sheng
                .iter()
                .position(|s| s.as_deref() == Some(want))
                .and_then(|i| KEYS.chars().nth(i))?;
            let fin = decompose(fin);
            let b = self
                .yun
                .iter()
                .position(|alts| alts.iter().any(|f| f == fin))
                .and_then(|i| KEYS.chars().nth(i))?;
            Some((a, b))
        }

        /// Expand a whole double-Pinyin buffer into full Pinyin.
        ///
        /// Keys are consumed in pairs. A trailing odd key passes through
        /// unchanged, so the preedit keeps showing what the user typed
        /// while they are halfway through a syllable — dropping it would
        /// make the panel look like it had missed the keystroke.
        #[must_use]
        pub fn expand(&self, syllables: &Syllables, buffer: &str) -> String {
            let chars: Vec<char> = buffer.chars().collect();
            let mut out = String::with_capacity(buffer.len() * 3);
            let mut i = 0;
            while i + 1 < chars.len() {
                if let Some(syl) = self.decode_syllable(syllables, chars[i], chars[i + 1]) {
                    out.push_str(&syl);
                } else {
                    // Not a legal pair — echo it rather than swallowing it.
                    out.push(chars[i]);
                    out.push(chars[i + 1]);
                }
                i += 2;
            }
            if i < chars.len() {
                out.push(chars[i]);
            }
            out
        }
    }

    /// Join an initial with a scheme-spelled final, restoring Pinyin's own
    /// orthography.
    ///
    /// The two rules Pinyin applies and the scheme tables do not: `ü` is
    /// written `u` after `j q x y` (`ju`, not `jü`), and `üe` keeps its
    /// umlaut only after `n` and `l` (`nüe`, `lüe` — but `jue`, `xue`).
    #[must_use]
    pub fn compose(initial: &str, fin: &str) -> String {
        match fin {
            "v" if matches!(initial, "j" | "q" | "x" | "y") => format!("{initial}u"),
            "v" => format!("{initial}ü"),
            "ue" if matches!(initial, "n" | "l") => format!("{initial}üe"),
            other => format!("{initial}{other}"),
        }
    }

    /// The inverse of [`compose`]: Pinyin's spelling of a final → the
    /// scheme's.
    fn decompose(fin: &str) -> &str {
        match fin {
            "ü" => "v",
            "üe" => "ue",
            "üan" => "uan",
            "ün" => "un",
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitting_keeps_the_finals_whole() {
        assert_eq!(parse_syllables("xuexi"), ["xue", "xi"]);
        assert_eq!(parse_syllables("beijing"), ["bei", "jing"]);
        assert_eq!(split_initial_final("zhuang"), Some(("zh", "uang")));
        assert_eq!(split_initial_final("an"), Some(("", "an")));
        assert_eq!(split_initial_final("xyz"), None);
    }

    #[test]
    fn an_unrecognised_ü_is_one_chunk_not_a_panic() {
        assert_eq!(parse_syllables("ǖ"), ["ǖ"]);
    }

    #[test]
    fn the_inventory_must_be_sorted_unique_syllables() {
        let s = Syllables::parse("a\nai\nzhuang\n").unwrap();
        assert!(s.contains("ai") && !s.contains("ia"));
        assert_eq!(s.len(), 3);
        assert!(Syllables::parse("ai\na\n").is_err(), "out of order");
        assert!(Syllables::parse("a\na\n").is_err(), "repeated");
        assert!(
            Syllables::parse("a\nng\n").is_err(),
            "not an initial and a final"
        );
        assert!(Syllables::parse("").is_err(), "empty");
    }

    #[test]
    fn a_schemes_file_must_hold_every_scheme_and_only_keys() {
        let one =
            r#"{"initials": {"b": "b", "o": "'"}, "finals": {"a": ["a"]}, "pairs": {"aa": "a"}}"#;
        let all = |t: &str| {
            let body: Vec<String> = double::ALL_SCHEMES
                .iter()
                .map(|s| format!("{:?}: {t}", s.name()))
                .collect();
            format!("{{{}}}", body.join(","))
        };
        let schemes = double::Schemes::parse(&all(one)).unwrap();
        let syl = Syllables::parse("a\nba\n").unwrap();
        let t = schemes.table(double::Scheme::Abc);
        assert_eq!(t.expand(&syl, "baaa"), "baa");
        assert_eq!(t.encode_syllable("ba"), Some(('b', 'a')));
        // One scheme missing.
        let five = all(one).replacen("\"xiaohe\"", "\"nonsense\"", 1);
        assert!(double::Schemes::parse(&five).is_err());
        // A key that is not one.
        let bad = all(&one.replacen("\"b\": \"b\"", "\"B\": \"b\"", 1));
        assert!(double::Schemes::parse(&bad).is_err());
        // An initial that is not one.
        let bad = all(&one.replacen("\"b\": \"b\"", "\"b\": \"bx\"", 1));
        assert!(double::Schemes::parse(&bad).is_err());
        // Three finals on one key.
        let bad = all(&one.replacen("[\"a\"]", "[\"a\", \"o\", \"e\"]", 1));
        assert!(double::Schemes::parse(&bad).is_err());
    }

    #[test]
    fn the_umlaut_rules_are_applied() {
        // ü is written u after j/q/x/y, and üe keeps its umlaut only
        // after n and l.
        assert_eq!(double::compose("j", "v"), "ju");
        assert_eq!(double::compose("n", "v"), "nü");
        assert_eq!(double::compose("l", "ue"), "lüe");
        assert_eq!(double::compose("x", "ue"), "xue");
    }
}
