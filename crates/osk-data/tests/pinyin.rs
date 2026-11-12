//! The committed Pinyin tables: the syllable inventory
//! (`pinyin/syllables.txt`) and the double-Pinyin schemes
//! (`pinyin/double-pinyin.json`), checked exhaustively — every syllable,
//! every scheme, encoded and decoded back.

use std::path::Path;

use osk_data::pinyin::Syllables;
use osk_data::pinyin::double::{ALL_SCHEMES, Scheme, Schemes, UNREACHABLE};

fn read(rel: &str) -> String {
    let p = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

fn syllables() -> Syllables {
    Syllables::parse(&read("pinyin/syllables.txt")).expect("pinyin/syllables.txt")
}

fn schemes() -> Schemes {
    Schemes::parse(&read("pinyin/double-pinyin.json")).expect("pinyin/double-pinyin.json")
}

fn expand(scheme: Scheme, buffer: &str) -> String {
    schemes().table(scheme).expand(&syllables(), buffer)
}

#[test]
fn the_inventory_is_mandarins_413_syllables() {
    let s = syllables();
    assert_eq!(s.len(), 413);
    for rare in ["den", "dia", "chua", "rua", "nou", "kei", "lü", "nüe"] {
        assert!(s.contains(rare), "{rare} is a syllable");
    }
    // CC-CEDICT's loanwords and the interjections are not.
    for not in ["coser", "cp", "call", "m", "ng", "hm"] {
        assert!(!s.contains(not), "{not} is not a syllable");
    }
}

#[test]
fn scheme_names_round_trip() {
    for s in ALL_SCHEMES {
        assert_eq!(Scheme::from_name(s.name()), Some(*s));
    }
    assert_eq!(Scheme::from_name("XiaoHe"), Some(Scheme::Xiaohe));
    assert_eq!(Scheme::from_name("nonsense"), None);
}

/// THE table check. Every syllable, every scheme, encode then decode.
#[test]
fn every_syllable_survives_a_round_trip() {
    let (all, syl) = (schemes(), syllables());
    for scheme in ALL_SCHEMES {
        let t = all.table(*scheme);
        let mut unencodable = Vec::new();
        let mut wrong = Vec::new();
        for s in syl.iter() {
            if UNREACHABLE.contains(&s) {
                continue;
            }
            let Some((a, b)) = t.encode_syllable(s) else {
                unencodable.push(s);
                continue;
            };
            match t.decode_syllable(&syl, a, b) {
                Some(back) if back == s => {}
                other => wrong.push((s, a, b, other)),
            }
        }
        assert!(
            unencodable.is_empty(),
            "{}: {} syllables cannot be typed at all: {:?}",
            scheme.name(),
            unencodable.len(),
            &unencodable[..unencodable.len().min(12)]
        );
        assert!(
            wrong.is_empty(),
            "{}: {} syllables decode to something else: {:?}",
            scheme.name(),
            wrong.len(),
            &wrong[..wrong.len().min(12)]
        );
    }
}

/// The allowlist must stay a statement of fact, not a dumping ground. If a
/// table change makes `lo` reachable, or makes something else unreachable,
/// this fails and the docs get revisited.
#[test]
fn the_unreachable_list_is_exactly_the_syllables_that_collide() {
    let (all, syl) = (schemes(), syllables());
    for scheme in ALL_SCHEMES {
        let t = all.table(*scheme);
        for s in UNREACHABLE {
            let (a, b) = t
                .encode_syllable(s)
                .unwrap_or_else(|| panic!("{}: {s} does not even encode", scheme.name()));
            assert_ne!(
                t.decode_syllable(&syl, a, b).as_deref(),
                Some(*s),
                "{}: {s} is reachable after all — take it off UNREACHABLE",
                scheme.name()
            );
        }
    }
}

#[test]
fn expansion_matches_full_pinyin() {
    // 你好 in Xiaohe: n+i, then h+ao(c).
    assert_eq!(expand(Scheme::Xiaohe, "nihc"), "nihao");
    // 中国: zh(v)+ong(s), then g+uo(o).
    assert_eq!(expand(Scheme::Xiaohe, "vsgo"), "zhongguo");
    // The same two words in Microsoft, which shares those keys.
    assert_eq!(expand(Scheme::Microsoft, "nihk"), "nihao");
}

#[test]
fn zero_initial_syllables_work_in_both_styles() {
    // Xiaohe gives them dedicated pairs...
    assert_eq!(expand(Scheme::Xiaohe, "ah"), "ang");
    assert_eq!(expand(Scheme::Xiaohe, "aa"), "a");
    // ...Microsoft routes them through the zero-initial key `o`.
    assert_eq!(expand(Scheme::Microsoft, "oj"), "an");
    assert_eq!(expand(Scheme::Microsoft, "oh"), "ang");
}

#[test]
fn the_umlaut_rules_are_applied() {
    // ü is written u after j/q/x/y, and üe keeps its umlaut only after n
    // and l.
    assert_eq!(expand(Scheme::Xiaohe, "nv"), "nü");
    assert_eq!(expand(Scheme::Xiaohe, "lt"), "lüe");
    assert_eq!(expand(Scheme::Xiaohe, "xt"), "xue");
}

#[test]
fn a_half_typed_syllable_is_echoed_not_swallowed() {
    // The preedit must keep showing what the user typed while they are
    // between the two keys of a syllable.
    assert_eq!(expand(Scheme::Xiaohe, "n"), "n");
    assert_eq!(expand(Scheme::Xiaohe, "nihcv"), "nihaov");
}

#[test]
fn an_illegal_pair_is_passed_through() {
    // `;` carries nothing in Xiaohe; the keys must not vanish.
    assert_eq!(expand(Scheme::Xiaohe, ";;"), ";;");
}

/// Each scheme must be distinguishable from the others, or a user picking
/// one would silently get another. Compares full keymaps.
#[test]
fn the_schemes_are_actually_different() {
    let (all, syl) = (schemes(), syllables());
    let maps: Vec<Vec<Option<(char, char)>>> = ALL_SCHEMES
        .iter()
        .map(|s| {
            syl.iter()
                .map(|x| all.table(*s).encode_syllable(x))
                .collect()
        })
        .collect();
    for i in 0..maps.len() {
        for j in (i + 1)..maps.len() {
            assert_ne!(
                maps[i],
                maps[j],
                "{} and {} are the same table",
                ALL_SCHEMES[i].name(),
                ALL_SCHEMES[j].name()
            );
        }
    }
}
