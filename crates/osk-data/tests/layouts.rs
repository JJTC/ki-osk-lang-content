//! Every committed layout (`layouts/<id>.json`) parses, and holds together
//! as a keyboard: its views reachable, its language's letters typeable,
//! its dialect one its language has.
//!
//! What a keyboard adds to a layout on loading (ki-osk's Tab key, accent
//! menus and dismiss key) is tested in ki-osk, against these files as a
//! release delivers them.

use std::collections::{BTreeSet, HashSet};
use std::path::{Path, PathBuf};

use osk_data::layout::{KeyAction, KeyOutput, LayoutDef, ViewId};

fn dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../layouts")
}

/// Every layout, with the path it came from.
fn layouts() -> Vec<(PathBuf, LayoutDef)> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir())
        .expect("layouts/")
        .flatten()
        .map(|e| e.path())
        .collect();
    paths.sort();
    paths
        .into_iter()
        .map(|p| {
            assert_eq!(
                p.extension().and_then(|e| e.to_str()),
                Some("json"),
                "{}: only layouts belong in layouts/",
                p.display()
            );
            let src = std::fs::read_to_string(&p).expect("read");
            let def = LayoutDef::parse(&src)
                .unwrap_or_else(|e| panic!("{}: parse/validate failed: {e}", p.display()));
            (p, def)
        })
        .collect()
}

fn load(id: &str) -> LayoutDef {
    let src = std::fs::read_to_string(dir().join(format!("{id}.json"))).expect(id);
    LayoutDef::parse(&src).expect("parse")
}

#[test]
fn every_layout_parses_under_its_own_name() {
    let mut ids = BTreeSet::new();
    let all = layouts();
    for (path, def) in &all {
        let stem = path.file_stem().unwrap().to_str().unwrap();
        assert_eq!(def.id, stem, "{}: id must match filename", path.display());
        assert!(ids.insert(def.id.clone()), "duplicate id {}", def.id);
    }
    assert!(
        all.len() >= 150,
        "expected the full layout set, saw {}",
        all.len()
    );
}

/// Backspace must be the right-most control on the emoji picker.
///
/// It was the LEFT-most, which is where it looked misplaced: every letter
/// layout puts backspace at the right end of a row, and so does the emoji
/// layout's own SEARCH view — so within one layout the two halves disagreed
/// with each other.
#[test]
fn the_emoji_picker_puts_backspace_on_the_right() {
    let emoji = load("emoji");
    let mut checked = 0;
    for view in &emoji.views {
        let Some(row) = view.rows.iter().find(|r| {
            r.keys
                .iter()
                .any(|k| matches!(k.action, Some(KeyAction::Backspace)))
        }) else {
            continue;
        };
        let idx = row
            .keys
            .iter()
            .position(|k| matches!(k.action, Some(KeyAction::Backspace)))
            .expect("just found it");
        assert_eq!(
            idx,
            row.keys.len() - 1,
            "view {:?}: backspace is at index {idx} of {} keys, not last",
            view.id,
            row.keys.len()
        );
        checked += 1;
    }
    assert!(
        checked >= 5,
        "expected a backspace in every emoji view (4 grids + search), found {checked}"
    );
}

/// The letters a layout's language writes every day and no tap types are
/// on its long presses, and such a layout says so, so the gesture is on
/// for it without `--long-press-alternates`.
#[test]
fn letters_on_long_presses_are_the_languages_own() {
    // What a tap types (a label or a text output) and what a long press
    // offers, on every view.
    let reach = |def: &LayoutDef| {
        let (mut tap, mut held) = (String::new(), String::new());
        for v in &def.views {
            for k in v.rows.iter().flat_map(|r| r.keys.iter()) {
                tap.push_str(&k.label);
                if let KeyOutput::Text { text } = &k.output {
                    tap.push_str(text);
                }
                for o in &k.long_press {
                    if let KeyOutput::Text { text } = o {
                        held.push_str(text);
                    }
                }
            }
        }
        (tap, held)
    };
    for (id, letters) in [
        ("es", "áéíóúü"),
        ("fr-azerty", "àâèêëçîïôœùûü"),
        ("pt", "ãáàâéêíóôõú"),
        ("it", "àèéìòù"),
        ("ca", "àéèíïòóúü"),
        ("cz", "áčďéěíňóřšťúůýž"),
        ("sk", "ďĺóŕ"),
        ("pl", "ąćęłńóśźż"),
        ("lv", "āčēģīķļņšūž"),
        ("is", "áéíóúý"),
        ("el", "άέήίϊόύώ"),
        ("vi", "àáảãạđ"),
        ("as", "ৰৱ"),
        ("pnb", "ݨ"),
        ("sa", "ऽॐ"),
        ("skr", "ٻڄݙڳݨ"),
        ("ks", "ٲۄۆؠۍ"),
        ("brh", "ۺ"),
        ("sd-deva", "ॻॼॾॿ"),
        ("mni-beng", "ৱ"),
        ("gbm", "ळ"),
        ("raj", "ळ"),
        ("khw", "څځݮݯݰݱ"),
        ("lus", "âêîôûṭÂÊÎÔÛṬ"),
        ("kha", "ïñÏÑ"),
        // The ligatures, and patah, qamats, dagesh and rafe.
        ("yi", "װױײ\u{5b7}\u{5b8}\u{5bc}\u{5bf}"),
    ] {
        let def = load(id);
        let (tap, held) = reach(&def);
        for c in letters.chars() {
            assert!(
                tap.contains(c) || held.contains(c),
                "{id}: no tap and no long press types {c}"
            );
        }
        assert!(
            def.long_press_letters,
            "{id} keeps letters on long presses but has no gesture by default"
        );
    }
    // A layout that says so has a letter only a long press types.
    for (_, def) in layouts() {
        if def.long_press_letters {
            let (tap, held) = reach(&def);
            assert!(
                held.chars().any(|c| c.is_alphabetic() && !tap.contains(c)),
                "{} says its letters are on long presses, but taps type them all",
                def.id
            );
        }
    }
}

/// A layout's `dialect` is one of its prediction language's varieties: a
/// misspelt one would quietly leave the language as its lexicon has it.
#[test]
fn every_dialect_is_a_variety_of_the_layouts_language() {
    let mut checked = 0;
    for (path, def) in layouts() {
        let Some(region) = &def.dialect else { continue };
        let lang = def.prediction_lang.as_deref().unwrap_or(&def.id);
        assert!(
            osk_data::dialects::varieties(lang).contains(&region.as_str()),
            "{}: {lang} has no variety {region}",
            path.display()
        );
        checked += 1;
    }
    assert!(checked >= 3, "only {checked} layouts name a dialect");
}

#[test]
fn every_declared_view_is_reachable_and_escapable() {
    let mut checked = 0;
    for (_, def) in layouts() {
        let declared: Vec<ViewId> = def.views.iter().map(|v| v.id).collect();

        // Navigation graph. `SwitchView` names its target; `Shift` is a
        // toggle, so it is an edge in both directions (that is how ki-osk
        // treats it).
        let mut edges: HashSet<(ViewId, ViewId)> = HashSet::new();
        for view in &def.views {
            for key in view.rows.iter().flat_map(|r| &r.keys) {
                match key.action {
                    Some(KeyAction::SwitchView(target)) => {
                        edges.insert((view.id, target));
                    }
                    Some(KeyAction::Shift) => {
                        edges.insert((view.id, ViewId::Shift));
                        edges.insert((ViewId::Shift, view.id));
                    }
                    _ => {}
                }
            }
        }

        let mut seen: HashSet<ViewId> = HashSet::from([ViewId::Base]);
        let mut stack = vec![ViewId::Base];
        while let Some(cur) = stack.pop() {
            for &(from, to) in &edges {
                if from == cur && seen.insert(to) {
                    stack.push(to);
                }
            }
        }

        let id = &def.id;
        for view in &declared {
            assert!(
                seen.contains(view),
                "{id}: view {view:?} is declared but no key navigates to it \
                 (declared: {declared:?})"
            );
            assert!(
                *view == ViewId::Base || edges.iter().any(|(from, _)| from == view),
                "{id}: view {view:?} is a dead end — no key leaves it"
            );
        }
        checked += 1;
    }
    assert!(
        checked >= 150,
        "expected the full layout set, saw {checked}"
    );
}
