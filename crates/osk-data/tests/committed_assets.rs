//! Every data file this repository commits decodes as its reader expects.
//!
//! The format guard: a decoder that disagrees with its encoder does not
//! crash — a loader returns `None` and the keyboard quietly falls back to
//! its seed dictionaries. This walks the committed tree, not a sample, and
//! an unrecognised `.bin` fails rather than going unchecked.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use osk_data::cedict::CedictTable;
use osk_data::codec::decode;
use osk_data::kana::{ConnectionBlob, KanjiTable};
use osk_data::ngram::NgramModel;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn files(dir: &str, ext: &str) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(root().join(dir))
        .unwrap_or_else(|e| panic!("{dir}/: {e}"))
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.to_string_lossy().ends_with(ext))
        .collect();
    out.sort();
    out
}

fn name(p: &Path) -> String {
    p.strip_prefix(root()).unwrap_or(p).display().to_string()
}

#[test]
fn every_prediction_pair_decodes() {
    let (forms, lexicons): (Vec<PathBuf>, Vec<PathBuf>) = files("predict", ".fst")
        .into_iter()
        .partition(|p| p.to_string_lossy().ends_with(".forms.fst"));
    let tables = files("predict", ".ngram.bin");
    let langs = |v: &[PathBuf], suffix: &str| -> BTreeSet<String> {
        v.iter()
            .map(|p| {
                p.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .trim_end_matches(suffix)
                    .to_string()
            })
            .collect()
    };
    assert_eq!(
        langs(&lexicons, ".fst"),
        langs(&tables, ".ngram.bin"),
        "every language needs both its lexicon and its n-gram table"
    );
    assert_eq!(
        langs(&lexicons, ".fst"),
        langs(&forms, ".forms.fst"),
        "every language needs its usual forms, empty where its script has no case"
    );
    assert!(
        lexicons.len() >= 54,
        "expected the full set, found {}",
        lexicons.len()
    );

    for p in &lexicons {
        let bytes = std::fs::read(p).unwrap();
        let map = fst::Map::new(bytes).unwrap_or_else(|e| panic!("{}: {e}", name(p)));
        assert!(map.len() > 1000, "{}: only {} words", name(p), map.len());
    }
    for p in &forms {
        fst::Map::new(std::fs::read(p).unwrap()).unwrap_or_else(|e| panic!("{}: {e}", name(p)));
    }
    for p in &tables {
        let m = NgramModel::from_bytes(&std::fs::read(p).unwrap())
            .unwrap_or_else(|e| panic!("{}: {e}", name(p)));
        // A wrong configuration can decode to a plausible empty model
        // rather than an error.
        assert!(
            m.bigram_len() > 0 && m.trigram_len() > 0,
            "{}: decoded but empty",
            name(p)
        );
    }
}

#[test]
fn every_dialect_table_decodes_and_is_sound() {
    let tables = files("predict", ".dialects.bin");
    assert!(!tables.is_empty(), "no dialect table is committed");
    for p in &tables {
        let file = p.file_name().unwrap().to_string_lossy().to_string();
        let lang = file.trim_end_matches(".dialects.bin").to_string();
        let lexicon = root().join(format!("predict/{lang}.fst"));
        assert!(lexicon.is_file(), "{}: no lexicon beside it", name(p));
        let d = osk_data::dialects::Dialects::from_bytes(&std::fs::read(p).unwrap())
            .unwrap_or_else(|e| panic!("{}: {e}", name(p)));
        let problems = d.problems(&lang);
        assert!(problems.is_empty(), "{}: {problems:?}", name(p));
        assert!(
            !d.groups.is_empty(),
            "{}: no spellings across varieties",
            name(p)
        );
    }
}

#[test]
fn every_input_method_table_decodes() {
    for dir in ["pinyin", "zhuyin", "jyutping"] {
        let p = root().join(dir).join("cedict.bin");
        let t: CedictTable =
            decode(&std::fs::read(&p).unwrap()).unwrap_or_else(|e| panic!("{}: {e}", name(&p)));
        // The full bundles hold tens of thousands of readings; the seeds
        // that once replaced them by accident hold ~150.
        assert!(
            t.len() > 10_000,
            "{}: only {} readings — a seed?",
            name(&p),
            t.len()
        );
    }
    let p = root().join("kana/mozc-kanji.bin");
    let t: KanjiTable =
        decode(&std::fs::read(&p).unwrap()).unwrap_or_else(|e| panic!("{}: {e}", name(&p)));
    assert!(
        t.len() > 10_000,
        "{}: only {} readings — a seed?",
        name(&p),
        t.len()
    );

    let p = root().join("kana/mozc-connection.bin");
    let blob: ConnectionBlob =
        decode(&std::fs::read(&p).unwrap()).unwrap_or_else(|e| panic!("{}: {e}", name(&p)));
    assert!(
        blob.n > 0 && !blob.deflated.is_empty(),
        "{}: empty matrix",
        name(&p)
    );
}

#[test]
fn no_bin_goes_unchecked() {
    // A new kind of `.bin` must get a decode rule above, not slip past.
    let known = ["predict", "pinyin", "zhuyin", "jyutping", "kana"];
    for entry in std::fs::read_dir(root()).unwrap().flatten() {
        let p = entry.path();
        let dir = p.file_name().unwrap().to_string_lossy().to_string();
        let not_data = ["target", "crates", "dist", "seeds"];
        if !p.is_dir() || dir.starts_with('.') || not_data.contains(&dir.as_str()) {
            continue;
        }
        let has_bin = std::fs::read_dir(&p)
            .unwrap()
            .flatten()
            .any(|e| e.path().extension().is_some_and(|x| x == "bin"));
        assert!(
            !has_bin || known.contains(&dir.as_str()),
            "{dir}/ holds .bin files this test does not decode"
        );
    }
}
