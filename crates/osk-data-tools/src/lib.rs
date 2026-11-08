//! Shared helpers for the data builders (`build-predict`, `build-pinyin`,
//! `build-zhuyin`, `build-jyutping`, `build-kana`, `build-connection`).
//! Each builder writes one kind of file in this repository through the
//! `osk-data` types; nothing here runs on a device.

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Load a `<word> <count>` frequency list into a lookup table.
///
/// Used to ORDER dictionary candidates. Anything absent scores 0 and
/// keeps its previous relative position, so a partial list degrades
/// instead of reshuffling the tail into nonsense.
///
/// # Errors
/// Propagates IO errors from opening or reading `path`.
pub fn load_freq_list<P: AsRef<Path>>(
    path: P,
) -> anyhow::Result<std::collections::HashMap<String, u64>> {
    let f = File::open(path.as_ref())?;
    let mut out: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
    for line in BufReader::new(f).lines() {
        let line = line?;
        let mut it = line.split_whitespace();
        let (Some(w), Some(n)) = (it.next(), it.next()) else {
            continue;
        };
        if let Ok(n) = n.parse::<u64>() {
            let e = out.entry(w.to_string()).or_insert(0);
            *e = (*e).max(n);
        }
    }
    Ok(out)
}

/// Order the candidate surfaces for one reading, and cap the list.
///
/// THE ordering rule for every CC-CEDICT-derived IME bundle
/// (`build-pinyin`, `build-zhuyin`, `build-jyutping`). It lives here
/// because all three had their own copy of `sort_by(len).then(order)`
/// and all three were therefore wrong in the same way — the defect
/// shape this project keeps meeting.
///
/// `entries` are `(char_count, input_order, surface)`. The rule:
///
/// 1. **Corpus frequency, descending.** CC-CEDICT carries no frequency
///    data and its own order is essentially Unicode codepoint order, so
///    without this the archaic variants sort ahead of the characters
///    people type — `de` reached 㝵 㥁 嘚 before 的, in a panel with
///    three slots. See ki-osk's `crates/osk-core/tests/pinyin_ranking.rs`.
/// 2. Shorter surfaces first, then input order — tie-breaks for
///    everything the frequency list has never seen.
/// 3. Duplicate surfaces dropped: CC-CEDICT lists a form once per
///    sense, and repeats spend the visible slots saying one thing.
#[must_use]
pub fn order_candidates<S: std::hash::BuildHasher>(
    mut entries: Vec<(usize, usize, String)>,
    freq: &std::collections::HashMap<String, u64, S>,
    cap: usize,
) -> Vec<String> {
    entries.sort_by(|a, b| {
        let fa = freq.get(&a.2).copied().unwrap_or(0);
        let fb = freq.get(&b.2).copied().unwrap_or(0);
        fb.cmp(&fa)
            .then_with(|| a.0.cmp(&b.0))
            .then_with(|| a.1.cmp(&b.1))
    });
    let mut seen = std::collections::HashSet::new();
    entries
        .into_iter()
        .map(|(_, _, s)| s)
        .filter(|s| seen.insert(s.clone()))
        .take(cap)
        .collect()
}

#[cfg(test)]
mod rank_tests {
    use super::*;
    use std::collections::HashMap;

    fn e(n: usize, o: usize, s: &str) -> (usize, usize, String) {
        (n, o, s.to_string())
    }

    #[test]
    fn frequency_beats_input_order() {
        // The shipped bug in miniature: 的 is last in CC-CEDICT order and
        // must come first once frequencies are known.
        let mut f = HashMap::new();
        f.insert("的".to_string(), 3_957_141u64);
        let got = order_candidates(vec![e(1, 0, "㝵"), e(1, 1, "嘚"), e(1, 2, "的")], &f, 3);
        assert_eq!(got, ["的", "㝵", "嘚"]);
    }

    #[test]
    fn unknown_surfaces_keep_their_relative_order() {
        let f = HashMap::new();
        let got = order_candidates(vec![e(2, 0, "bb"), e(1, 1, "a"), e(1, 2, "b")], &f, 3);
        assert_eq!(got, ["a", "b", "bb"], "shorter first, then input order");
    }

    #[test]
    fn duplicates_are_dropped_before_the_cap() {
        // Three slots must show three DIFFERENT things.
        let f = HashMap::new();
        let got = order_candidates(
            vec![e(1, 0, "好"), e(1, 1, "好"), e(1, 2, "号"), e(1, 3, "浩")],
            &f,
            3,
        );
        assert_eq!(got, ["好", "号", "浩"]);
    }
}

#[cfg(test)]
mod release_tests {
    /// The format crate spells out its version (other workspaces build it
    /// and must not inherit this one's keys), and a release is named after
    /// it. It must stay the workspace's version.
    #[test]
    fn the_format_crate_has_the_workspace_version() {
        assert_eq!(
            osk_data::RELEASE_VERSION,
            env!("CARGO_PKG_VERSION"),
            "keep crates/osk-data/Cargo.toml's version equal to [workspace.package]"
        );
    }
}
