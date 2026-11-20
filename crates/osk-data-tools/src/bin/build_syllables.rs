//! `build-syllables` — the word list of a language cut into syllables
//! (Shan, Mon, Sgaw Karen, Pa'O, Arakanese:
//! `osk_data::wordbreak::Script::syllables`), which has no list of its
//! words to be had: the syllables its text writes, each seen `--min`
//! times or more, the commonest first.
//!
//! Usage:
//!   build-syllables --lang shn --corpus shn.txt --out wordbreak/shn.txt
//!
//! The text is read as the keyboard and build-predict read it, in NFC, as
//! the list's words and what is looked up in it are: its words as
//! [`osk_data::words::Splitter`] finds them, and their runs of the script's
//! characters cut into syllables, as the splitter cuts them with the list.
//! So no syllable is one the splitter never makes: Myanmar's symbols ၌ ၍ ၎
//! ၏, which end a word as punctuation does (three of the 300 commonest
//! syllables Arakanese's list had when they were counted as its runs').
//! A syllable the list leaves out runs on, where it is written, with the
//! unknown ones beside it, one word ([`osk_data::wordbreak::WordList::cut`]).

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context as _, bail};
use clap::Parser;
use osk_data::wordbreak::Script;
use osk_data::words::Splitter;
use unicode_normalization::UnicodeNormalization;

#[derive(Parser)]
#[command(name = "build-syllables")]
struct Args {
    /// The language: one cut into syllables (`shn`, `mnw`, `ksw`, `blk`, `rki`).
    #[arg(long)]
    lang: String,
    /// Its text, plain UTF-8. Repeat for several.
    #[arg(long, required = true)]
    corpus: Vec<PathBuf>,
    /// The fewest times a syllable is seen to be kept.
    #[arg(long, default_value_t = 2)]
    min: u64,
    /// The list written.
    #[arg(long)]
    out: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let script = match Script::of_lang(&args.lang) {
        Some(s) if s.cut_into_syllables() => s,
        _ => bail!("{} is not a language cut into syllables", args.lang),
    };
    let splitter = Splitter::new(&args.lang);
    let mut counts: HashMap<String, u64> = HashMap::new();
    for path in &args.corpus {
        let text =
            fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        count(&splitter, script, &text, &mut counts);
    }
    let seen = counts.len();
    let mut kept: Vec<(String, u64)> = counts.into_iter().filter(|&(_, n)| n >= args.min).collect();
    if kept.is_empty() {
        bail!("no syllable of {} is seen {} times", args.lang, args.min);
    }
    kept.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let mut out = format!(
        "# The syllables of `{}`'s text, as osk_data::wordbreak cuts it: the {} of {} seen {} times or more, the commonest first (build-syllables).\n",
        args.lang,
        kept.len(),
        seen,
        args.min
    );
    for (syllable, _) in &kept {
        out.push_str(syllable);
        out.push('\n');
    }
    fs::write(&args.out, out).with_context(|| format!("writing {}", args.out.display()))?;
    eprintln!(
        "{}: {} syllables of {} kept",
        args.out.display(),
        kept.len(),
        seen
    );
    Ok(())
}

/// Count the syllables of `text`'s words into `counts`: the runs of
/// `script`'s characters in each word the splitter (with no list) finds,
/// cut into syllables.
fn count(splitter: &Splitter, script: Script, text: &str, counts: &mut HashMap<String, u64>) {
    for line in text.lines() {
        let line: String = line.nfc().collect();
        for word in splitter.words(&line) {
            let word = &line[word.range];
            let mut start = None;
            // A space after the word ends its last run.
            for (at, c) in word.char_indices().chain([(word.len(), ' ')]) {
                match (start, script.holds(c)) {
                    (None, true) => start = Some(at),
                    (Some(s), false) => {
                        let run = &word[s..at];
                        for r in script.syllables(run) {
                            *counts.entry(run[r].to_string()).or_default() += 1;
                        }
                        start = None;
                    }
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syllables_are_those_the_splitter_makes() {
        let lang = "rki";
        let script = Script::of_lang(lang).unwrap();
        let mut counts = HashMap::new();
        // ၏ and ၎ end a word as punctuation does: neither is a syllable,
        // and the syllable after ၎ starts a word.
        count(&Splitter::new(lang), script, "ကမ္ဘာ၏ ၎င်း", &mut counts);
        let mut got: Vec<(&str, u64)> = counts.iter().map(|(s, n)| (s.as_str(), *n)).collect();
        got.sort_unstable();
        assert_eq!(got, [("က", 1), ("င\u{103a}း", 1), ("မ\u{1039}ဘာ", 1)]);
    }
}
