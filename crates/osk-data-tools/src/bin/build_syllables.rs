//! `build-syllables` — the word list of a language cut into syllables
//! (Shan, Mon, Sgaw Karen, Pa'O, Arakanese:
//! `osk_data::wordbreak::Script::syllables`), which has no list of its
//! words to be had: the syllables its text writes, each seen `--min`
//! times or more, the commonest first.
//!
//! Usage:
//!   build-syllables --lang shn --corpus shn.txt --out wordbreak/shn.txt
//!
//! The text is cut as the keyboard and build-predict cut it, in NFC, as
//! the list's words and what is looked up in it are. A syllable the list
//! leaves out runs on, where it is written, with the unknown ones beside
//! it, one word ([`osk_data::wordbreak::WordList::cut`]).

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context as _, bail};
use clap::Parser;
use osk_data::wordbreak::Script;
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
    let mut counts: HashMap<String, u64> = HashMap::new();
    for path in &args.corpus {
        let text =
            fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        for line in text.lines() {
            let line: String = line.nfc().collect();
            let mut start = None;
            // A space after the line ends its last run.
            for (at, c) in line.char_indices().chain([(line.len(), ' ')]) {
                match (start, script.holds(c)) {
                    (None, true) => start = Some(at),
                    (Some(s), false) => {
                        let run = &line[s..at];
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
