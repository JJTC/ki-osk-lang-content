//! Keyboard layouts (`layouts/<id>.json`): which keys a layout has, what
//! each types, and which views it pages through.
//!
//! The JSON a layout file holds, and the checks it must pass. What a
//! keyboard does with a layout beyond that (a Tab key on request, the
//! standard accent menus, the dismiss key) is the keyboard's own: ki-osk's
//! `osk_core::layout` adds it on loading.

use std::fmt;

use serde::{Deserialize, Serialize};

/// A short identifier like `"us"`, `"fr-azerty"`, `"cn-pinyin"`. Used in
/// config, persistence, and the CLI.
pub type LayoutId = String;

/// Current schema version. Bump only on backward-incompatible
/// shape changes; loaders reject layouts whose `schema_version`
/// is `> CURRENT_SCHEMA_VERSION`.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Top-level layout description.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutDef {
    /// Schema version. Defaults to `1` so existing JSON without the
    /// field keeps loading. Future incompatible changes bump this
    /// and update the loader's accept set.
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    /// e.g. `"de"`, `"fr-azerty"`.
    pub id: LayoutId,
    /// Human-readable name shown in the picker.
    pub name: String,
    /// xkbcommon layout name; drives keymap generation for Latin layouts.
    /// `None` for IME-driven or text-commit layouts.
    #[serde(default)]
    pub xkb_layout: Option<String>,
    /// xkbcommon variant, e.g. `"nodeadkeys"`.
    #[serde(default)]
    pub xkb_variant: Option<String>,
    /// Writing direction for the preedit strip / key grid.
    #[serde(default)]
    pub direction: Direction,
    /// If set, keypresses feed an IME instead of the virtual keyboard.
    #[serde(default)]
    pub ime: Option<ImeKind>,
    /// Whether swipe typing is allowed on this layout.
    #[serde(default)]
    pub swipe_enabled: bool,
    /// Whether n-gram prediction applies.
    #[serde(default = "default_true")]
    pub prediction_enabled: bool,
    /// Canonical language tag for prediction/dictionary lookup — a BCP-47
    /// / ISO-639 code, **not** the layout `id`. It names the
    /// `predict/<lang>.fst` model and lets several layouts share one
    /// language: `us`/`gb`/`us-intl` → `"en"`, `se` → `"sv"`, `dk` →
    /// `"da"`, `cz` → `"cs"`, `fr-azerty` → `"fr"`. Defaults to `id` (fine
    /// for the many layouts whose id already is the language code, e.g.
    /// `ru`, `hi`). `None` for IME / non-text layouts.
    #[serde(default)]
    pub prediction_lang: Option<String>,
    /// The variety of the prediction language that the layout's users
    /// write, as a BCP 47 region subtag (`GB` on the UK layout, `BR` on the
    /// Brazilian one): the strip offers its spellings, and autocorrect
    /// writes them, where the language has dialect flags
    /// ([`crate::dialects`]). `--dialect` overrides it. `None` where a
    /// layout's users write several: the language as its lexicon has it,
    /// the spellings of all its varieties together (the US layout's, which
    /// Australians, Canadians and Indians type on too; the Spanish one's,
    /// typed in Spain and Latin America alike).
    #[serde(default)]
    pub dialect: Option<String>,
    /// Some of the language's own letters are on long presses (Polish
    /// `ł`, Vietnamese tones, Greek `ά`): the long-press gesture is on for
    /// this layout whatever `--long-press-alternates` says, since without
    /// it they could not be typed at all.
    #[serde(default)]
    pub long_press_letters: bool,
    /// Script-fallback font group this layout's keycaps need beyond the
    /// `DejaVu` UI face — e.g. `"cjk"`, `"thai"`, `"arabic"`,
    /// `"devanagari"`, `"tamil"`. `None` for Latin/Greek/Cyrillic/core-Arabic
    /// layouts `DejaVu` already covers. The keyboard loads the matching face
    /// on demand the first time such a layout is shown, so a build that
    /// never selects, say, a Tamil layout never touches the Tamil font. IME
    /// layouts (Pinyin/Kana/Hangul) additionally pull the CJK face for
    /// their candidate bar regardless of this field.
    #[serde(default)]
    pub font_script: Option<String>,
    /// Cap on how wide a 1-unit key may be, as a multiple of the row
    /// height. When set, the grid stops stretching at that point and is
    /// CENTRED in the panel instead.
    ///
    /// For the numeric keypad. A full-width 4-column grid on a 1920 px
    /// panel gives 480 px digits — a keypad the size of a keyboard, which
    /// reads as broken rather than as a phone. `None` (every other layout)
    /// keeps the old behaviour of filling the width, which is what a
    /// letter layout wants.
    #[serde(default)]
    pub max_key_aspect: Option<f32>,
    /// Ordered views: base, shift, symbol pages.
    pub views: Vec<View>,
}

fn default_true() -> bool {
    true
}
fn default_schema_version() -> u32 {
    CURRENT_SCHEMA_VERSION
}

/// The composer a layout types through, named in its `ime` field.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImeKind {
    /// Mandarin Pinyin → Hanzi.
    Pinyin,
    /// Korean 2-set Hangul syllable composer.
    Hangul,
    /// Japanese Romaji → Kana → Kanji.
    Kana,
    /// Amharic/Tigrinya Ge'ez phonetic Latin → fidel composer.
    Geez,
    /// Zhuyin (Bopomofo) → Traditional Hanzi.
    Zhuyin,
    /// Cantonese Jyutping (Latin romanisation) → Traditional Hanzi.
    Jyutping,
    /// Latin letters → the script of the layout's `prediction_lang`, by its
    /// transliteration table ([`crate::translit`]) and its lexicon: Hindi,
    /// Tamil, Urdu, Greek typed in Latin.
    Translit,
}

/// Text direction for RTL layouts.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    /// Left-to-right (default).
    #[default]
    Ltr,
    /// Right-to-left (Arabic, Hebrew, Farsi).
    Rtl,
}

/// Which "page" of the keyboard is active.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewId {
    /// Default view; letters.
    Base,
    /// Shift latched/locked.
    Shift,
    /// First symbol page (numbers + common punctuation).
    Symbol1,
    /// Second symbol page (more symbols).
    Symbol2,
    /// Emoji-picker search page: a QWERTY that builds a query string
    /// (the keyboard shows ranked results in the top strip) rather than
    /// committing keystrokes.
    Search,
}

/// A concrete page of rows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
    /// Which page this is.
    pub id: ViewId,
    /// Rows top-to-bottom.
    pub rows: Vec<Row>,
}

/// A row of keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    /// Height in "row units"; normalized against the total when laying out.
    pub height_units: f32,
    /// Keys left-to-right (visually; storage is always reading-order).
    pub keys: Vec<Key>,
}

/// A single key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    /// Width in key-units. Conventionally 1.0 = letter width; space is 4.0-5.0.
    pub width_units: f32,
    /// Text shown on the keycap.
    pub label: String,
    /// What the key emits on tap.
    pub output: KeyOutput,
    /// Alternatives shown on long-press popup.
    #[serde(default)]
    pub long_press: Vec<KeyOutput>,
    /// Special behavior in addition to `output` (shift toggle, etc.).
    #[serde(default)]
    pub action: Option<KeyAction>,
}

/// What a tap produces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum KeyOutput {
    /// Raw evdev keycode (no +8 offset). Sent via virtual-keyboard.
    Keycode {
        /// evdev code, e.g. `KEY_A = 30`.
        code: u32,
    },
    /// UTF-8 committed via input-method-v2 `commit_string`.
    Text {
        /// The literal string to commit.
        text: String,
    },
    /// Combining mark; composed with the next character via NFC.
    DeadKey {
        /// The combining codepoint (e.g. `'\u{0301}'` for acute).
        mark: char,
    },
    /// A text-editing command, sent as its conventional control chord
    /// (`Ctrl` plus a letter) rather than as literal text.
    ///
    /// Reached through the long-press menu on the letter each command is
    /// conventionally bound to, which is where a touch-only kiosk can
    /// offer them at all: there is no physical `Ctrl` to hold.
    Editing {
        /// Which command.
        action: EditAction,
    },
    /// Nothing — used by action-only keys like Shift.
    None,
}

/// A clipboard / text-editing command offered on a long press.
///
/// No `Undo`, and the reason is the client rather than us.
///
/// It was offered on `z` and silently did nothing. The keyboard's side is
/// correct — `Ctrl+A` visibly selects the whole field and `Ctrl+X` empties
/// it, over the same chord path — so command dispatch reaches `WebKit`
/// fine. Asked from inside the page instead, with text inserted through
/// `WebKit`'s own editing pipeline:
///
/// ```text
/// queryCommandSupported(undo): true
/// after insertText: "hello"
/// queryCommandEnabled(undo): false     <- the stack is empty
/// execCommand(undo) returned: false
/// after undo: "hello"
/// ```
///
/// `execCommand('insertText')` is exactly what registers an undo step, so
/// an empty stack after it means `WebKit`-WPE never registers any. `WebKit`
/// leaves the undo client to the embedder; the GTK port implements it, the
/// WPE port does not, and there is nothing for `Ctrl+Z` to undo however it
/// is delivered.
///
/// So no key press can make this work under cog, and a button that cannot
/// work is worse than an absent one. Restoring it is a variant plus a
/// `for_keycode` arm, for a deployment on a client that implements undo.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditAction {
    /// Select the whole field (`Ctrl+A`).
    SelectAll,
    /// Cut the selection (`Ctrl+X`).
    Cut,
    /// Copy the selection (`Ctrl+C`).
    Copy,
    /// Paste the clipboard (`Ctrl+V`).
    Paste,
}

impl EditAction {
    /// The evdev keycode this command's chord uses, with `Ctrl` held.
    #[must_use]
    pub fn keycode(self) -> u32 {
        match self {
            Self::SelectAll => 30, // a
            Self::Cut => 45,       // x
            Self::Copy => 46,      // c
            Self::Paste => 47,     // v
        }
    }

    /// Icon shown in the long-press menu.
    ///
    /// Emoji rather than symbol glyphs, chosen because the panel already
    /// carries a colour-emoji font and a monochrome emoji fallback, so
    /// these render in both — where the natural symbol candidates
    /// (`U+2398` copy, `U+29C9` copy, `U+238C` undo, `U+2B1A` dotted box)
    /// are absent from the shipped faces and would have drawn as tofu.
    /// Verified against the shipped `NotoColorEmoji`: all five have colour
    /// strikes.
    #[must_use]
    pub fn icon(self) -> &'static str {
        match self {
            Self::SelectAll => "\u{1F533}", // white square button
            Self::Cut => "\u{2702}",        // scissors
            Self::Copy => "\u{1F4C4}",      // page facing up
            Self::Paste => "\u{1F4CB}",     // clipboard
        }
    }

    /// Human-readable name. Not what the popup draws — see
    /// [`Self::icon`] — but what the logs and the accessibility tree say,
    /// so the meaning is not lost with the glyph.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::SelectAll => "Select all",
            Self::Cut => "Cut",
            Self::Copy => "Copy",
            Self::Paste => "Paste",
        }
    }

    /// The command conventionally bound to `code`, if any.
    #[must_use]
    pub fn for_keycode(code: u32) -> Option<Self> {
        match code {
            30 => Some(Self::SelectAll),
            45 => Some(Self::Cut),
            46 => Some(Self::Copy),
            47 => Some(Self::Paste),
            _ => None,
        }
    }
}

/// Non-output behaviors: page switch, modifier, picker.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyAction {
    /// Latch Shift (tap) / lock Shift (double-tap).
    Shift,
    /// Switch to `Symbol1` / `Symbol2` / back to `Base`.
    SwitchView(ViewId),
    /// Open the layout picker.
    SwitchLayout,
    /// Evdev `KEY_BACKSPACE`; repeats on hold.
    Backspace,
    /// Commit a newline (`\n`) or evdev `KEY_ENTER` per layout.
    Enter,
    /// Evdev `KEY_SPACE`, plus word-commit in prediction mode.
    Space,
    /// Evdev `KEY_TAB` — moves focus to the next form field.
    ///
    /// An *action* rather than a plain `Keycode`, which is what it was
    /// first built as. A keycode commit applies the Shift modifier
    /// whenever the shifted view is live, and auto-capitalisation makes
    /// that view live on every empty field — so Tab went out as
    /// Shift+Tab and walked the form BACKWARDS, re-arming itself on each
    /// empty field it landed in. Tab is a function key like
    /// [`KeyAction::Enter`] and [`KeyAction::Backspace`], and those take
    /// no modifier for exactly this reason.
    Tab,
    /// Open emoji picker layer.
    Emoji,
    /// Cycle to next enabled layout.
    Globe,
    /// Hide the keyboard.
    Hide,
}

/// Errors surfaced while parsing or validating a layout.
#[derive(Debug)]
pub enum LayoutError {
    /// JSON parse error.
    Parse(serde_json::Error),
    /// Semantic validation error (e.g. empty rows).
    Invalid {
        /// Layout ID.
        id: LayoutId,
        /// Human-readable explanation.
        msg: String,
    },
}

impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "parse: {e}"),
            Self::Invalid { id, msg } => write!(f, "invalid layout {id}: {msg}"),
        }
    }
}

impl std::error::Error for LayoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parse(e) => Some(e),
            Self::Invalid { .. } => None,
        }
    }
}

impl From<serde_json::Error> for LayoutError {
    fn from(e: serde_json::Error) -> Self {
        Self::Parse(e)
    }
}

impl LayoutDef {
    /// Parse a layout from JSON and validate it: bails on obviously-broken
    /// input (empty rows, zero widths, duplicate view IDs). The layout
    /// exactly as its file has it, nothing added.
    ///
    /// # Errors
    /// Returns `LayoutError::Parse` on JSON errors and `LayoutError::Invalid`
    /// for semantic issues.
    pub fn parse(src: &str) -> Result<Self, LayoutError> {
        let def: Self = serde_json::from_str(src)?;
        def.validate()?;
        Ok(def)
    }

    /// Check structural invariants. Called automatically by [`Self::parse`].
    ///
    /// # Errors
    /// Returns `LayoutError::Invalid` on: no views, duplicate views, empty
    /// rows, zero widths, or missing `Base` view.
    pub fn validate(&self) -> Result<(), LayoutError> {
        let bad = |msg: &str| LayoutError::Invalid {
            id: self.id.clone(),
            msg: msg.into(),
        };
        if self.schema_version > CURRENT_SCHEMA_VERSION {
            return Err(bad(&format!(
                "schema_version {} > supported {}; upgrade ki-osk or downgrade the layout file",
                self.schema_version, CURRENT_SCHEMA_VERSION,
            )));
        }
        if self.views.is_empty() {
            return Err(bad("no views"));
        }
        if !self.views.iter().any(|v| v.id == ViewId::Base) {
            return Err(bad("missing Base view"));
        }
        let mut seen = Vec::new();
        for v in &self.views {
            if seen.contains(&v.id) {
                return Err(bad("duplicate view id"));
            }
            seen.push(v.id);
            if v.rows.is_empty() {
                return Err(bad("view has no rows"));
            }
            for row in &v.rows {
                if row.height_units <= 0.0 {
                    return Err(bad("row height_units must be > 0"));
                }
                if row.keys.is_empty() {
                    return Err(bad("row has no keys"));
                }
                for k in &row.keys {
                    if k.width_units <= 0.0 {
                        return Err(bad("key width_units must be > 0"));
                    }
                }
            }
        }
        Ok(())
    }

    /// Fetch a view by id.
    #[must_use]
    pub fn view(&self, id: ViewId) -> Option<&View> {
        self.views.iter().find(|v| v.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
      "id": "us", "name": "English (US)", "xkb_layout": "us", "swipe_enabled": true,
      "views": [{ "id": "base", "rows": [
        { "height_units": 1.0, "keys": [
          { "width_units": 1.0, "label": "a", "output": { "kind": "keycode", "code": 30 } }
        ]}
      ]}]
    }"#;

    #[test]
    fn parses_minimal_layout() {
        let d = LayoutDef::parse(SAMPLE).unwrap();
        assert_eq!(d.id, "us");
        assert!(d.swipe_enabled);
        assert_eq!(d.views.len(), 1);
    }

    #[test]
    fn parsing_adds_nothing_to_the_file() {
        // A space bar and no dismiss key: the format keeps it so. The
        // keyboard adds its dismiss key on loading, not the format.
        let json = r#"{
            "id": "t", "name": "T",
            "views": [{ "id": "base", "rows": [
                { "height_units": 1.0, "keys": [
                    { "width_units": 4, "label": "space", "output": {"kind":"keycode","code":57}, "action": "space" }
                ]}
            ]}]
        }"#;
        let d = LayoutDef::parse(json).unwrap();
        assert_eq!(d.views[0].rows[0].keys.len(), 1);
    }

    #[test]
    fn rejects_empty_rows() {
        let broken = r#"{"id":"x","name":"","views":[{"id":"base","rows":[]}]}"#;
        assert!(LayoutDef::parse(broken).is_err());
    }

    #[test]
    fn rejects_zero_width() {
        let broken = r#"{
          "id":"x","name":"",
          "views":[{"id":"base","rows":[{"height_units":1,"keys":[
            {"width_units":0,"label":"a","output":{"kind":"none"}}
          ]}]}]
        }"#;
        assert!(LayoutDef::parse(broken).is_err());
    }

    #[test]
    fn rejects_missing_base() {
        let broken = r#"{
          "id":"x","name":"",
          "views":[{"id":"shift","rows":[{"height_units":1,"keys":[
            {"width_units":1,"label":"a","output":{"kind":"none"}}
          ]}]}]
        }"#;
        assert!(LayoutDef::parse(broken).is_err());
    }

    #[test]
    fn rejects_a_newer_schema() {
        let newer = SAMPLE.replacen("\"id\"", "\"schema_version\": 2, \"id\"", 1);
        let e = LayoutDef::parse(&newer).unwrap_err();
        assert!(e.to_string().contains("schema_version 2"), "{e}");
    }
}

#[cfg(test)]
mod edit_action_tests {
    use super::EditAction;

    const ALL: [EditAction; 4] = [
        EditAction::SelectAll,
        EditAction::Cut,
        EditAction::Copy,
        EditAction::Paste,
    ];

    /// Each command draws a distinct icon, and every icon is one the
    /// emoji path can actually resolve.
    ///
    /// The gate is the first scalar being at or above U+1000, because that
    /// is precisely the test both of ki-osk-wayland's renderers apply
    /// before trying the colour-emoji font. An icon below it would silently
    /// fall through to the UI face, which has no glyph for any of these —
    /// tofu.
    #[test]
    fn every_icon_is_distinct_and_reaches_the_emoji_path() {
        let mut seen = std::collections::HashSet::new();
        for a in ALL {
            let icon = a.icon();
            assert!(!icon.is_empty(), "{a:?} has no icon");
            let first = icon.chars().next().expect("non-empty");
            assert!(
                (first as u32) >= 0x1000,
                "{a:?}: icon U+{:04X} is below the emoji cutoff both \
                 renderers gate on, so it would draw as tofu",
                first as u32
            );
            assert!(seen.insert(icon), "{a:?}: icon {icon:?} is not unique");
        }
    }

    /// The words survive alongside the icons — logs and the accessibility
    /// tree still need to say what a command is.
    #[test]
    fn labels_stay_human_readable() {
        for a in ALL {
            let l = a.label();
            assert!(
                l.chars().all(|c| c.is_ascii_alphabetic() || c == ' '),
                "{a:?}: label {l:?} should be words, not a glyph"
            );
        }
        assert_eq!(EditAction::SelectAll.label(), "Select all");
    }

    /// The chord keycodes are the conventional ones, and `for_keycode` is
    /// the exact inverse — the keyboard relies on that round-trip to
    /// decide which letters get a command.
    #[test]
    fn keycodes_round_trip() {
        for a in ALL {
            assert_eq!(EditAction::for_keycode(a.keycode()), Some(a), "{a:?}");
        }
        assert_eq!(EditAction::for_keycode(16), None, "q has no command");
        assert_eq!(EditAction::for_keycode(44), None, "z: Undo was removed");
    }
}
