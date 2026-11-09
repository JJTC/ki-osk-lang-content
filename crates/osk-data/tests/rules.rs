//! Every rule file parses and passes its own tests: `rules/all.weir` on its
//! own, and each language's `rules/<lang>.weir` with `all.weir` after it,
//! as the keyboard reads them. A language's file is a prediction
//! language's.

use std::fs;
use std::path::{Path, PathBuf};

use osk_data::rules::{RuleFile, Rules};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn parse(path: &Path) -> RuleFile {
    let name = path.file_name().expect("a file").to_string_lossy();
    let src = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    RuleFile::parse(&name, &src).unwrap_or_else(|e| panic!("{e}"))
}

#[test]
fn every_rule_file_passes_its_tests() {
    let dir = root().join("rules");
    let all = parse(&dir.join("all.weir"));
    let mut failures = Rules::new(vec![all.clone()])
        .expect("all.weir resolves")
        .failures();
    let mut langs = 0;
    for entry in fs::read_dir(&dir).expect("rules/") {
        let path = entry.expect("an entry").path();
        let Some(lang) = path
            .file_name()
            .and_then(|n| n.to_str())
            .and_then(|n| n.strip_suffix(".weir"))
            .filter(|l| *l != "all")
        else {
            assert_eq!(
                path.file_name().and_then(|n| n.to_str()),
                Some("all.weir"),
                "rules/ holds .weir files only"
            );
            continue;
        };
        assert!(
            root().join(format!("predict/{lang}.fst")).is_file(),
            "rules/{lang}.weir is for no prediction language"
        );
        let rules = Rules::new(vec![parse(&path), all.clone()]).unwrap_or_else(|e| panic!("{e}"));
        failures.extend(rules.failures());
        langs += 1;
    }
    assert!(langs > 0, "no language has rules of its own");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
