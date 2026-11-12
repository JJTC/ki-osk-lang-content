//! The emoji search catalogue (`emoji/catalogue.json`): every
//! fully-qualified Unicode emoji with its CLDR short name, CLDR keywords,
//! category and skin-tone variants — the index an emoji picker's search
//! bar queries. Built by `scripts/build-emoji.py` from Unicode's
//! `emoji-test.txt` and CLDR's English annotations.
//!
//! The picker's grid is a layout of its own, `layouts/emoji.json`
//! ([`crate::layout`]), a curated subset built by
//! `scripts/gen-emoji-layout.py`.

use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Unicode emoji categories (UTS #51).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EmojiCategory {
    /// 😀 😂 😭
    SmileysEmotion,
    /// 👋 👨 👩
    PeopleBody,
    /// 🐶 🌹
    AnimalsNature,
    /// 🍎 🍕
    FoodDrink,
    /// ✈️ 🚗 🏔️
    TravelPlaces,
    /// ⚽ 🎉
    Activities,
    /// 💡 📱
    Objects,
    /// ✅ ❌ ♻️
    Symbols,
    /// 🏳️ 🇺🇸
    Flags,
    /// Pinned by a picker's UI; not part of the Unicode taxonomy, and never
    /// in the catalogue.
    RecentlyUsed,
}

/// One emoji entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Emoji {
    /// UTF-8 form (the actual characters to commit).
    pub glyph: String,
    /// Canonical CLDR short name, e.g. `"smiling face with heart-eyes"`.
    pub name: String,
    /// Space-separated CLDR keywords (English).
    pub keywords: String,
    /// Which category tab the emoji lives under.
    pub category: EmojiCategory,
    /// Its skin-tone variants (the long-press menu), if it takes them.
    pub skin_tones: Vec<String>,
}

/// What is wrong with a catalogue.
#[derive(Debug)]
pub enum EmojiError {
    /// Not JSON of the catalogue's shape.
    Json(serde_json::Error),
    /// An entry breaks a rule of the catalogue.
    Invalid(String),
}

impl fmt::Display for EmojiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(e) => write!(f, "emoji catalogue: {e}"),
            Self::Invalid(msg) => write!(f, "emoji catalogue: {msg}"),
        }
    }
}

impl std::error::Error for EmojiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(e) => Some(e),
            Self::Invalid(_) => None,
        }
    }
}

/// Read a catalogue: a JSON array of [`Emoji`].
///
/// # Errors
/// JSON of another shape; no entry; an entry with no glyph or no name, in
/// [`EmojiCategory::RecentlyUsed`], or with a glyph another entry has.
pub fn parse(json: &str) -> Result<Vec<Emoji>, EmojiError> {
    let entries: Vec<Emoji> = serde_json::from_str(json).map_err(EmojiError::Json)?;
    if entries.is_empty() {
        return Err(EmojiError::Invalid("no entries".into()));
    }
    let mut seen = HashSet::new();
    for e in &entries {
        if e.glyph.is_empty() || e.name.trim().is_empty() {
            return Err(EmojiError::Invalid(format!(
                "an entry without a glyph or a name: {e:?}"
            )));
        }
        if e.category == EmojiCategory::RecentlyUsed {
            return Err(EmojiError::Invalid(format!(
                "{}: recently-used is the picker's, not a category of the data",
                e.glyph
            )));
        }
        if !seen.insert(e.glyph.as_str()) {
            return Err(EmojiError::Invalid(format!("{} twice", e.glyph)));
        }
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: &str = r#"[{"glyph": "😀", "name": "grinning face",
        "keywords": "face grin", "category": "smileys-emotion", "skin_tones": []}]"#;

    #[test]
    fn a_catalogue_parses() {
        let c = parse(ONE).unwrap();
        assert_eq!(c[0].category, EmojiCategory::SmileysEmotion);
    }

    #[test]
    fn a_catalogue_is_refused_with_a_repeat_or_a_picker_category() {
        let entry = &ONE[1..ONE.len() - 1];
        let twice = format!("[{entry},{entry}]");
        assert!(matches!(parse(&twice), Err(EmojiError::Invalid(_))));
        let recent = ONE.replace("smileys-emotion", "recently-used");
        assert!(parse(&recent).is_err());
        assert!(parse("[]").is_err());
    }
}
