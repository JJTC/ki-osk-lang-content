//! `build-dialects` — build a language's dialect flags
//! (`predict/<lang>.dialects.bin`, `osk_data::dialects`): which of its
//! varieties use a word, and a word's spellings across them.
//!
//! English comes from the English Speller Database (ESDB, SCOWLv2), whose
//! spelling codes say which variety writes each spelling of a word (`A`
//! American, `B` British "ise", `Z` British "ize", `C` Canadian, `D`
//! Australian) and how usual it is there. Indian English follows British
//! spelling, and Harper's dictionary adds the words it marks as Indian
//! alone (`crore`, `lakh`).
//!
//! Portuguese and Spanish come from Wiktionary, through kaikki.org's
//! extracts ([`kaikki`]): Brazil's and Portugal's spellings (`fato` and
//! `facto`, `econômico` and `económico`), Spain's and Latin America's
//! (`vídeo` and `video`), and the words of one of them alone (`autocarro`,
//! `computadora`).
//!
//! The table keeps what a keyboard can use with the language's lexicon:
//! the words of the lexicon whose use differs between the varieties,
//! every spelling of a word the lexicon holds a spelling of, and the
//! varieties' own words the lexicon lacks, which a keyboard offers at the
//! lexicon's floor.
//!
//! Usage:
//!   build-dialects --lang en --esdb esdb-scowl-pre.txt \
//!     --harper harper-dictionary.dict --lexicon predict/en.fst \
//!     --out predict/en.dialects.bin
//!   build-dialects --lang pt --kaikki kaikki-pt.jsonl.gz \
//!     --lexicon predict/pt.fst --forms predict/pt.forms.fst \
//!     --out predict/pt.dialects.bin

mod kaikki;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::PathBuf;

use anyhow::{Context as _, bail, ensure};
use clap::Parser;
use osk_data::dialects::{DialectWord, Dialects, varieties};
use osk_data::words::Splitter;

#[derive(Parser)]
#[command(name = "build-dialects")]
struct Args {
    /// The language (`en`).
    #[arg(long)]
    lang: String,
    /// ESDB's `scowl-pre.txt` (English).
    #[arg(long)]
    esdb: Option<PathBuf>,
    /// Harper's `dictionary.dict` (English): its Indian words.
    #[arg(long)]
    harper: Option<PathBuf>,
    /// kaikki.org's extract of the language's Wiktionary entries
    /// (Portuguese, Spanish, French), gzipped JSON lines.
    #[arg(long)]
    kaikki: Option<PathBuf>,
    /// The language's lexicon, `predict/<lang>.fst`.
    #[arg(long)]
    lexicon: PathBuf,
    /// The language's usual forms, `predict/<lang>.forms.fst`: a word the
    /// lexicon holds as a name (`Tony`) is not taken for the word of one
    /// variety it spells (Chile's `tony`, a clown).
    #[arg(long)]
    forms: Option<PathBuf>,
    /// Where to write the table.
    #[arg(long)]
    out: PathBuf,
}

/// English's varieties' bits, in `osk_data::dialects::varieties("en")`'s
/// order.
const US: u32 = 1;
const GB: u32 = 1 << 1;
const CA: u32 = 1 << 2;
const AU: u32 = 1 << 3;
const IN: u32 = 1 << 4;
const ALL_EN: u32 = US | GB | CA | AU | IN;

/// Words, by identity, with the masks of the varieties that write them
/// usually and that accept them, and the spellings found of one word.
#[derive(Default)]
struct Found {
    masks: HashMap<String, (u32, u32)>,
    spellings: Vec<BTreeSet<String>>,
}

impl Found {
    fn mark(&mut self, word: String, usual: u32, accepted: u32) {
        let m = self.masks.entry(word).or_insert((0, 0));
        m.0 |= usual;
        m.1 |= accepted;
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Args::parse();
    let names = varieties(&args.lang);
    ensure!(!names.is_empty(), "no varieties for `{}`", args.lang);
    let splitter = Splitter::new(&args.lang);
    let mut found = Found::default();
    let mut lone: BTreeMap<String, u32> = BTreeMap::new();
    match args.lang.as_str() {
        "en" => {
            let esdb = args.esdb.as_ref().context("English needs --esdb")?;
            let text =
                fs::read_to_string(esdb).with_context(|| format!("reading {}", esdb.display()))?;
            esdb_words(&text, &splitter, &mut found);
            if let Some(path) = &args.harper {
                let text = fs::read_to_string(path)
                    .with_context(|| format!("reading {}", path.display()))?;
                for w in harper_indian(&text) {
                    lone.insert(splitter.normalize(&w), IN);
                }
            }
        }
        "pt" | "es" | "fr" => {
            let path = args
                .kaikki
                .as_ref()
                .context("this language needs --kaikki")?;
            kaikki::read(&args.lang, path, &splitter, &mut found)?;
        }
        other => bail!("no dialect source for `{other}` yet"),
    }

    let lexicon = fst::Map::new(
        fs::read(&args.lexicon).with_context(|| format!("reading {}", args.lexicon.display()))?,
    )
    .with_context(|| format!("{} is not a lexicon", args.lexicon.display()))?;
    let forms = match &args.forms {
        Some(path) => Some(
            fst::Map::new(fs::read(path).with_context(|| format!("reading {}", path.display()))?)
                .with_context(|| format!("{} is not a forms table", path.display()))?,
        ),
        None => None,
    };
    // Capitalised as a rule, where not at a sentence's start.
    let is_name = |w: &str| {
        forms
            .as_ref()
            .and_then(|f| f.get(w))
            .is_some_and(|m| m & 1 != 0)
    };
    let table = assemble(names, &found, &lone, &|w| lexicon.contains_key(w), &is_name);
    let problems = table.problems(&args.lang);
    ensure!(problems.is_empty(), "the table is unsound: {problems:?}");
    let bytes = table.to_bytes()?;
    let tmp = args.out.with_extension("bin.tmp");
    fs::write(&tmp, &bytes).with_context(|| format!("writing {}", tmp.display()))?;
    fs::rename(&tmp, &args.out).with_context(|| format!("writing {}", args.out.display()))?;
    let per: Vec<String> = names
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let n = table
                .words
                .iter()
                .filter(|w| w.usual & (1 << i) != 0)
                .count();
            format!("{v} {n}")
        })
        .collect();
    log::info!(
        "wrote {} ({} words in {} groups; usual in {}; {} bytes)",
        args.out.display(),
        table.words.len(),
        table.groups.len(),
        per.join(", "),
        bytes.len()
    );
    Ok(())
}

/// The table: of what was `found`, the groups of spellings the lexicon
/// holds one of, the lexicon's words whose use differs, but for those it
/// holds as names, and the `lone` words of some varieties alone that the
/// lexicon lacks.
fn assemble(
    names: &[&str],
    found: &Found,
    lone: &BTreeMap<String, u32>,
    in_lexicon: &dyn Fn(&str) -> bool,
    is_name: &dyn Fn(&str) -> bool,
) -> Dialects {
    let all = (1u32 << names.len()) - 1;
    // Spellings of one word, merged where they share one.
    let mut parent: Vec<usize> = (0..found.spellings.len()).collect();
    fn root(parent: &mut [usize], mut i: usize) -> usize {
        while parent[i] != i {
            parent[i] = parent[parent[i]];
            i = parent[i];
        }
        i
    }
    let mut first: HashMap<&str, usize> = HashMap::new();
    for (i, set) in found.spellings.iter().enumerate() {
        for w in set {
            if let Some(&j) = first.get(w.as_str()) {
                let (a, b) = (root(&mut parent, i), root(&mut parent, j));
                parent[a] = b;
            } else {
                first.insert(w, i);
            }
        }
    }
    let mut merged: BTreeMap<usize, BTreeSet<&str>> = BTreeMap::new();
    for (i, set) in found.spellings.iter().enumerate() {
        let r = root(&mut parent, i);
        merged
            .entry(r)
            .or_default()
            .extend(set.iter().map(String::as_str));
    }
    let groups: Vec<BTreeSet<&str>> = merged
        .into_values()
        .filter(|g| g.len() > 1 && g.iter().any(|w| in_lexicon(w)))
        .collect();

    let mut words: BTreeMap<String, (u32, u32, Option<u32>)> = BTreeMap::new();
    for (g, set) in groups.iter().enumerate() {
        for w in set {
            let (u, a) = found.masks.get(*w).copied().unwrap_or((all, all));
            words.insert((*w).to_string(), (u, a, u32::try_from(g).ok()));
        }
    }
    for (w, &(u, a)) in &found.masks {
        if (u, a) != (all, all) && in_lexicon(w) && !is_name(w) && !words.contains_key(w) {
            words.insert(w.clone(), (u, a, None));
        }
    }
    for (w, &mask) in lone {
        if !in_lexicon(w) && !words.contains_key(w) {
            words.insert(w.clone(), (mask, mask, None));
        }
    }
    // Every variety must accept a word in some form; drop one none does.
    words.retain(|_, (_, a, _)| *a != 0);

    let index: HashMap<&str, u32> = words
        .keys()
        .enumerate()
        .map(|(i, w)| (w.as_str(), u32::try_from(i).unwrap_or(u32::MAX)))
        .collect();
    let mut table_groups: Vec<Vec<u32>> = vec![Vec::new(); groups.len()];
    for (w, &(_, _, g)) in &words {
        if let Some(g) = g {
            table_groups[g as usize].push(index[w.as_str()]);
        }
    }
    // Groups whose spellings all fell away.
    let keep: Vec<bool> = table_groups.iter().map(|g| g.len() > 1).collect();
    let renumber: Vec<Option<u32>> = keep
        .iter()
        .scan(0u32, |n, &k| {
            let out = k.then_some(*n);
            *n += u32::from(k);
            Some(out)
        })
        .collect();
    Dialects {
        varieties: names.iter().map(ToString::to_string).collect(),
        words: words
            .into_iter()
            .map(|(word, (usual, accepted, g))| DialectWord {
                word,
                usual,
                accepted,
                group: g.and_then(|g| renumber[g as usize]),
            })
            .collect(),
        groups: table_groups.into_iter().filter(|g| g.len() > 1).collect(),
    }
}

/// A spelling code and its variant level, as ESDB writes them (`Cv`).
type Variant = (char, String);

/// One ESDB line, as far as the dialect flags need it.
struct Line {
    variants: Option<Vec<Variant>>,
    lemma: String,
    pos: String,
    defn: String,
    entries: Vec<(String, Option<Vec<Variant>>)>,
}

/// The variant info `s` holds (`A Cv DV`, `B {1}`), if it is one.
fn variant_info(s: &str) -> Option<Vec<Variant>> {
    let mut out = Vec::new();
    for t in s.split_whitespace() {
        if t.len() > 2 && t.starts_with('{') && t.ends_with('}') {
            continue;
        }
        let mut cs = t.chars();
        let code = cs.next().filter(|c| "ABZCD_".contains(*c))?;
        let level: String = cs.collect();
        if level.chars().count() > 1 || !level.chars().all(|c| ".=?v~V@-x".contains(c)) {
            return None;
        }
        out.push((code, level));
    }
    (!out.is_empty()).then_some(out)
}

/// A word without ESDB's annotations (`-`, `@`, `!` before it; `*`, `-`,
/// `@`, `~`, `!`, `†` after it).
fn bare(w: &str) -> &str {
    w.trim()
        .trim_start_matches(['-', '@', '!'])
        .trim_end_matches(['*', '-', '@', '~', '!', '†'])
}

fn parse_line(line: &str) -> Option<Line> {
    let line = line.split('#').next()?.trim_end();
    let (_, rest) = line.split_once(": ")?;
    let (first, after) = match rest.split_once(": ") {
        Some((a, b)) => (a, Some(b)),
        None => (rest, None),
    };
    let (variants, lemma_info, entries) = match (variant_info(first), after) {
        (Some(v), Some(after)) => match after.split_once(": ") {
            Some((l, e)) => (Some(v), l, Some(e)),
            None => (Some(v), after, None),
        },
        (None, Some(after)) if first == "+" => match after.split_once(": ") {
            Some((l, e)) => (None, l, Some(e)),
            None => (None, after, None),
        },
        _ => (None, first, after),
    };
    // LEMMA [' <' POS '>'] [' {' DEFN '}'] [' (' USAGE ')']
    let cut = [" <", " {", " ("]
        .iter()
        .filter_map(|p| lemma_info.find(p))
        .min()
        .unwrap_or(lemma_info.len());
    let lemma = lemma_info[..cut].to_string();
    let between = |open: char, close: char| {
        lemma_info[cut..]
            .split_once(open)
            .and_then(|(_, r)| r.split_once(close))
            .map_or_else(String::new, |(v, _)| v.to_string())
    };
    let mut parsed = Vec::new();
    for e in entries.unwrap_or("").split(", ").filter(|e| !e.is_empty()) {
        if let Some(inner) = e.strip_prefix('(').and_then(|e| e.strip_suffix(')')) {
            for alt in inner.split('|') {
                match alt.trim().split_once(": ") {
                    Some((vi, w)) => parsed.push((bare(w).to_string(), variant_info(vi))),
                    None => parsed.push((bare(alt).to_string(), None)),
                }
            }
        } else {
            parsed.push((bare(e).to_string(), None));
        }
    }
    Some(Line {
        variants,
        lemma,
        pos: between('<', '>'),
        defn: between('{', '}'),
        entries: parsed,
    })
}

/// The masks of the varieties that write a spelling with `variants`
/// usually and that accept it, in a group whose lines name the spelling
/// codes `codes`: ESDB's inheritance applied (no `Z` line: `B` implies
/// `Z`; no `C`: `Z` implies `C`; no `D`: `B` implies `D`), `Z` accepted in
/// Britain, and India following Britain. A spelling with no codes is
/// everyone's; one coded `_`, the same everywhere.
///
/// Usual: ESDB's preferred forms and the forms it holds equal to them
/// (levels none, `.` and `=`: `among` and `amongst` in Britain). Accepted:
/// those and its variants up to "acceptable" (`?`, `v`, `~`, `V`), not
/// the uncommon, archaic or invalid ones.
fn masks(variants: Option<&[Variant]>, codes: &BTreeSet<char>) -> (u32, u32) {
    let Some(vs) = variants else {
        return (ALL_EN, ALL_EN);
    };
    let level = |l: &str| match l {
        "" | "." | "=" => (true, true),
        "?" | "v" | "~" | "V" => (false, true),
        _ => (false, false),
    };
    let mut lv: BTreeMap<char, String> = vs.iter().cloned().collect();
    if let Some(l) = lv.get(&'_') {
        let (u, a) = level(l);
        return (if u { ALL_EN } else { 0 }, if a { ALL_EN } else { 0 });
    }
    if !codes.contains(&'Z')
        && let Some(b) = lv.get(&'B').cloned()
    {
        lv.entry('Z').or_insert(b);
    }
    if !codes.contains(&'C')
        && let Some(z) = lv.get(&'Z').cloned()
    {
        lv.entry('C').or_insert(z);
    }
    if !codes.contains(&'D')
        && let Some(b) = lv.get(&'B').cloned()
    {
        lv.entry('D').or_insert(b);
    }
    let (mut usual, mut accepted) = (0, 0);
    for (code, bit) in [('A', US), ('B', GB), ('C', CA), ('D', AU)] {
        if let Some(l) = lv.get(&code) {
            let (u, a) = level(l);
            usual |= if u { bit } else { 0 };
            accepted |= if a { bit } else { 0 };
        }
    }
    if lv.get(&'Z').is_some_and(|l| level(l).1) {
        accepted |= GB;
    }
    if usual & GB != 0 {
        usual |= IN;
    }
    if accepted & GB != 0 {
        accepted |= IN;
    }
    (usual, accepted)
}

/// Read ESDB's groups into `found`: each word's masks, over every line it
/// is on, and the spellings of one word, which a group lists in parallel
/// (`color <n>` and `colour <n>`, then their inflections in the same
/// order). Open compounds (`color bar`) are no one word, and are left out.
fn esdb_words(text: &str, splitter: &Splitter, found: &mut Found) {
    for group in text.split("\n\n") {
        let lines: Vec<Line> = group
            .lines()
            .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
            .filter_map(parse_line)
            .collect();
        let codes: BTreeSet<char> = lines
            .iter()
            .flat_map(|l| {
                l.variants
                    .iter()
                    .flatten()
                    .chain(l.entries.iter().flat_map(|(_, v)| v.iter().flatten()))
            })
            .map(|(c, _)| *c)
            .collect();
        let mut slots: BTreeMap<(String, String, String, usize, usize), BTreeSet<String>> =
            BTreeMap::new();
        let mut dashes: HashMap<(String, String, String), usize> = HashMap::new();
        for l in &lines {
            let (u, a) = masks(l.variants.as_deref(), &codes);
            let sig: String = l
                .variants
                .iter()
                .flatten()
                .map(|(c, _)| *c)
                .collect::<BTreeSet<char>>()
                .into_iter()
                .collect();
            let base = if l.lemma == "-" {
                let n = dashes
                    .entry((l.pos.clone(), l.defn.clone(), sig))
                    .or_insert(0);
                *n += 1;
                ("dash", *n)
            } else if l.lemma.contains(' ') {
                continue;
            } else {
                let w = splitter.normalize(bare(&l.lemma));
                found.mark(w.clone(), u, a);
                if l.variants.is_some() {
                    slots
                        .entry(("lemma".into(), l.pos.clone(), l.defn.clone(), 0, 0))
                        .or_default()
                        .insert(w);
                }
                ("entries", 0)
            };
            for (i, (w, vs)) in l.entries.iter().enumerate() {
                if w.is_empty() || w == "-" || w.contains(' ') {
                    continue;
                }
                let (uu, aa) = if vs.is_some() {
                    masks(vs.as_deref(), &codes)
                } else {
                    (u, a)
                };
                let w = splitter.normalize(w);
                found.mark(w.clone(), uu, aa);
                if l.variants.is_some() || vs.is_some() {
                    slots
                        .entry((base.0.into(), l.pos.clone(), l.defn.clone(), base.1, i))
                        .or_default()
                        .insert(w);
                }
            }
        }
        found
            .spellings
            .extend(slots.into_values().filter(|s| s.len() > 1));
    }
}

/// The words Harper's dictionary marks as Indian English alone (`₹`, and
/// none of its American `<`, British `!`, Canadian `@` or Australian `_`):
/// India's own words, where the rest it marks Indian are British spellings.
fn harper_indian(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|l| l.split('#').next()?.trim().split_once('/'))
        .filter(|(_, flags)| flags.contains('₹') && !flags.contains(['<', '!', '@', '_']))
        .map(|(w, _)| w.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ESDB: &str = "\
35 [12dicts]: A Cv DV: color <n>
35 [12dicts]: B C D: colour <n>
35 [12dicts]: A Cv DV: - <n>: colors
35 [12dicts]: B C D: - <n>: colours

35 [12dicts]: A Z: realize <v>
35 [12dicts]: B: realise <v>
35 [12dicts]: A Z: - <v>: realized, realizing, realizes
35 [12dicts]: B: - <v>: realised, realising, realises

60 [12dicts]: color bar <n>

35 [12dicts]: A C Dv: fulfill <v>: fulfilled, fulfilling, fulfills
35 [12dicts]: B Cv D: fulfil <v>: fulfilled, fulfilling, fulfils
";

    #[test]
    fn spellings_pair_up_and_each_variety_gets_its_own() {
        let mut found = Found::default();
        esdb_words(ESDB, &Splitter::new("en"), &mut found);
        // Not the open compound: color stays American.
        assert_eq!(found.masks["color"], (US, US | CA | AU));
        assert_eq!(
            found.masks["colours"],
            (GB | CA | AU | IN, GB | CA | AU | IN)
        );
        // Britain accepts -ize, and writes -ise.
        assert_eq!(found.masks["realize"].1 & GB, GB);
        assert_eq!(found.masks["realised"].0, GB | AU | IN);
        // A form both write is both's.
        assert_eq!(found.masks["fulfilled"].0, ALL_EN);
        let lexicon = ["color", "realize", "fulfill"];
        let lone = BTreeMap::from([("lakh".to_string(), IN)]);
        let d = assemble(
            varieties("en"),
            &found,
            &lone,
            &|w| lexicon.contains(&w),
            &|_| false,
        );
        assert!(d.problems("en").is_empty(), "{:?}", d.problems("en"));
        let gb = d.variety("GB").unwrap();
        assert_eq!(d.spelling_in("color", gb), Some("colour"));
        assert_eq!(d.usual_spelling("realize", gb), Some("realise"));
        assert_eq!(d.spelling_in("fulfill", gb), Some("fulfil"));
        assert!(d.get("fulfilled").is_none());
        assert_eq!(d.get("lakh").map(|w| w.usual), Some(IN));
        // Groups the lexicon holds no spelling of are left out.
        assert!(d.get("colours").is_none());
    }

    /// ESDB's own lines: forms it holds equal are both usual.
    #[test]
    fn equal_forms_are_both_usual() {
        let esdb = "\
35 [12dicts]: A B= Z: among <pp>
35 [12dicts]: AV B Zv: amongst <pp>

35 [12dicts]: dream <v>: (A B Z C=: dreamed | B= Zv C: dreamt), dreaming, dreams
";
        let mut found = Found::default();
        esdb_words(esdb, &Splitter::new("en"), &mut found);
        assert_eq!(found.masks["among"].0, ALL_EN);
        assert_eq!(found.masks["amongst"].0, GB | AU | IN);
        assert_eq!(found.masks["dreamed"].0, ALL_EN);
        assert_eq!(found.masks["dreamt"].0 & (GB | CA), GB | CA);
        let lexicon = ["among", "dreamed"];
        let d = assemble(
            varieties("en"),
            &found,
            &BTreeMap::new(),
            &|w| lexicon.contains(&w),
            &|_| false,
        );
        let gb = d.variety("GB").unwrap();
        assert_eq!(d.usual_spelling("among", gb), None);
        assert_eq!(d.usual_spelling("dreamed", d.variety("CA").unwrap()), None);
    }

    #[test]
    fn harper_gives_indias_own_words() {
        let dict = "lakh/N\nlakhs\ncolour/!_₹\nprepone/₹V # a verb\ncrore/₹";
        assert_eq!(harper_indian(dict), ["prepone", "crore"]);
    }
}
