//! Offensive-word lists (`offensive/<lang>.txt`): words a keyboard keeps
//! off its suggestion strip when asked to (ki-osk's `--filter-offensive`,
//! off by default). They never restrict what can be typed.
//!
//! One word a line; blank lines are skipped, and matching is whole-word
//! and case-insensitive, so each word is taken in lower case.

/// The words of one list, each trimmed and in lower case, blank lines
/// skipped.
pub fn words(text: &str) -> impl Iterator<Item = String> + '_ {
    text.lines()
        .map(str::trim)
        .filter(|w| !w.is_empty())
        .map(str::to_lowercase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_list_is_its_lines_in_lower_case() {
        let w: Vec<String> = words("Damn\n\n  hell \n").collect();
        assert_eq!(w, ["damn", "hell"]);
    }
}
