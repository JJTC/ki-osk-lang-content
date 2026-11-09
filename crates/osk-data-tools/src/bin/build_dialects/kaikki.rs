//! Dialect flags from Wiktionary (its English edition), through kaikki.org's
//! extracts: one JSON object per entry, `kaikki.org-dictionary-<Language>
//! .jsonl.gz`.
//!
//! Wiktionary says where a spelling or a word is used in several ways, and
//! only some are clear enough to act on:
//!
//! - whose spelling an entry is: a head qualifier (Portuguese `(European
//!   Portuguese spelling)`), a sense qualifier (`only form used in
//!   Spain`), or a gloss naming the other spelling (`Brazilian Portuguese
//!   standard spelling of anónimo`), with the alternatives it lists for
//!   another place (`fato`, Brazil's `facto`);
//! - an entry that is only another spelling of a word somewhere
//!   (`futbol`: Mexico's and Central America's `fútbol`);
//! - a cell of an inflection table that is one place's (`falamos`, Brazil's
//!   first person plural preterite of `falar`, where Portugal writes
//!   `falámos`);
//! - a word whose every sense is tagged with places, none of them only
//!   "chiefly", "colloquially" or the like there (`autocarro`, Portugal's
//!   bus): a word of those places.
//!
//! A word anyone uses otherwise is everyone's: its masks are joined over
//! every entry it has (`fato` is Brazil's `facto`, and everyone's suit). An
//! inflected form takes its lemma's flags (`económica`, `económico`'s), and
//! the forms of two spellings of a word pair up cell by cell
//! (`económicas` and `econômicas`).

use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{Context as _, bail};
use flate2::read::GzDecoder;
use osk_data::dialects::varieties;
use osk_data::words::Splitter;
use serde::Deserialize;

use super::Found;

/// How a language's entries say where they are used.
struct Marks {
    /// Place tags, with the varieties each names.
    places: &'static [(&'static str, &'static [&'static str])],
    /// Head qualifiers saying whose spelling the entry is: the varieties
    /// that write it usually, and those that accept it.
    heads: &'static [(
        &'static str,
        &'static [&'static str],
        &'static [&'static str],
    )],
    /// Sense qualifiers saying so.
    qualifiers: &'static [(&'static str, &'static [&'static str])],
    /// Gloss prefixes naming another variety's spelling of the entry
    /// (`European Portuguese standard spelling of econômico`): the
    /// varieties the entry's spelling is.
    region_glosses: &'static [(&'static str, &'static [&'static str])],
    /// The varieties that still accept the spellings their last reform
    /// replaced (`acção`, Portugal's until 1990).
    prereform: &'static [&'static str],
}

const PORTUGUESE: Marks = Marks {
    places: &[
        ("Brazil", &["BR"]),
        ("Brazilian-Portuguese", &["BR"]),
        ("Portugal", &["PT"]),
        ("European-Portuguese", &["PT"]),
    ],
    heads: &[
        ("(European Portuguese spelling)", &["PT"], &["PT"]),
        ("(only European Portuguese spelling)", &["PT"], &["PT"]),
        ("(Brazilian Portuguese spelling)", &["BR"], &["BR"]),
        (
            "(chiefly Brazilian Portuguese spelling)",
            &["BR"],
            &["BR", "PT"],
        ),
        (
            "(Brazilian Portuguese spelling, superseded in European Portuguese)",
            &["BR"],
            &["BR"],
        ),
        (
            "(Brazilian Portuguese spelling, European Portuguese spelling)",
            &["BR", "PT"],
            &["BR", "PT"],
        ),
    ],
    qualifiers: &[
        ("European Portuguese spelling", &["PT"]),
        ("Brazilian Portuguese spelling", &["BR"]),
    ],
    region_glosses: &[
        ("European Portuguese ", &["PT"]),
        ("Brazilian Portuguese ", &["BR"]),
    ],
    prereform: &["PT"],
};

const SPANISH: Marks = Marks {
    places: &[
        ("Spain", &["ES"]),
        ("Canary-Islands", &["ES"]),
        ("Andalusia", &["ES"]),
        ("Aragon", &["ES"]),
        ("Asturias", &["ES"]),
        ("Galicia", &["ES"]),
        ("Valencia", &["ES"]),
        ("Navarre", &["ES"]),
        ("Tenerife", &["ES"]),
        ("Seville", &["ES"]),
        ("Catalonia", &["ES"]),
        ("Latin-America", &["419"]),
        ("Mexico", &["419"]),
        ("Central-America", &["419"]),
        ("South-America", &["419"]),
        ("Caribbean", &["419"]),
        ("Antilles", &["419"]),
        ("Rioplatense", &["419"]),
        ("Lunfardo", &["419"]),
        ("Argentina", &["419"]),
        ("Northwestern-Argentina", &["419"]),
        ("Belize", &["419"]),
        ("Bolivia", &["419"]),
        ("Chile", &["419"]),
        ("Colombia", &["419"]),
        ("Costa-Rica", &["419"]),
        ("Cuba", &["419"]),
        ("Dominican-Republic", &["419"]),
        ("Ecuador", &["419"]),
        ("El-Salvador", &["419"]),
        ("Guatemala", &["419"]),
        ("Honduras", &["419"]),
        ("Nicaragua", &["419"]),
        ("Panama", &["419"]),
        ("Paraguay", &["419"]),
        ("Peru", &["419"]),
        ("Puerto-Rico", &["419"]),
        ("Uruguay", &["419"]),
        ("Venezuela", &["419"]),
        ("US", &["419"]),
        ("California", &["419"]),
        ("Texas", &["419"]),
        ("New-York-City", &["419"]),
    ],
    heads: &[],
    qualifiers: &[("only form used in Spain", &["ES"])],
    region_glosses: &[],
    prereform: &[],
};

const FRENCH: Marks = Marks {
    places: &[
        ("France", &["FR"]),
        ("Paris", &["FR"]),
        ("Lyon", &["FR"]),
        ("Alsace", &["FR"]),
        ("Lorraine", &["FR"]),
        ("Normandy", &["FR"]),
        ("Picardy", &["FR"]),
        ("Quebec", &["CA"]),
        ("Montreal", &["CA"]),
        ("Canada", &["CA"]),
        ("Canadian", &["CA"]),
        ("North-America", &["CA"]),
        ("Belgium", &["BE"]),
        ("Switzerland", &["CH"]),
        ("Europe", &["FR", "BE", "CH"]),
    ],
    heads: &[],
    qualifiers: &[],
    region_glosses: &[],
    prereform: &[],
};

/// Tags that make a sense's place a matter of degree or of register, not
/// of where the word is used at all, or the form something other than a
/// spelling (`chiefly`, `colloquial`, `eye-dialect`).
const HEDGES: &[&str] = &[
    "colloquial",
    "informal",
    "slang",
    "especially",
    "chiefly",
    "sometimes",
    "dated",
    "uncommon",
    "rare",
    "archaic",
    "obsolete",
    "broadly",
    "often",
    "mainly",
    "usually",
    "now",
    "also",
    "vulgar",
    "derogatory",
    "humorous",
    "nonstandard",
    "dialectal",
    "regional",
    "figuratively",
    "by-extension",
    "Internet",
    "abbreviation",
    "eye-dialect",
    "pronunciation-spelling",
    "clipping",
    "censored",
    "euphemistic",
    "childish",
    "acronym",
    "initialism",
    "contraction",
    "misspelling",
    "jocular",
    "proscribed",
    "nonce-word",
    "historical",
    "poetic",
    "literary",
];

/// Forms that are no cell of a word's inflection table.
const NOT_INFLECTIONS: &[&str] = &[
    "table-tags",
    "inflection-template",
    "class",
    "romanization",
    "canonical",
    "alternative",
    "obsolete",
    "archaic",
    "dated",
    "misspelling",
    "nonstandard",
    "rare",
    "dialectal",
    "pronunciation-spelling",
    "Internet",
    "abbreviation",
    "proscribed",
    "uncommon",
    "colloquial",
    "eye-dialect",
    "error-unknown-tag",
];

#[derive(Deserialize)]
struct Entry {
    #[serde(default)]
    word: String,
    #[serde(default)]
    pos: String,
    #[serde(default)]
    head_templates: Vec<Head>,
    #[serde(default)]
    senses: Vec<Sense>,
    #[serde(default)]
    forms: Vec<Form>,
}

#[derive(Deserialize)]
struct Head {
    #[serde(default)]
    expansion: String,
}

#[derive(Deserialize)]
struct Sense {
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    glosses: Vec<String>,
    #[serde(default)]
    qualifier: Option<String>,
    #[serde(default)]
    alt_of: Vec<Link>,
    #[serde(default)]
    form_of: Vec<Link>,
}

#[derive(Deserialize)]
struct Link {
    #[serde(default)]
    word: String,
}

#[derive(Deserialize)]
struct Form {
    #[serde(default)]
    form: String,
    #[serde(default)]
    tags: Vec<String>,
}

/// Whether `w` is one word: letters and digits, apostrophes within, and
/// hyphens between them (`guarda-chuva`, not `à:` or `fato de macaco`).
fn wordlike(w: &str) -> bool {
    let mut prev_hyphen = true;
    for c in w.chars() {
        if c == '-' {
            if prev_hyphen {
                return false;
            }
            prev_hyphen = true;
        } else if c.is_alphanumeric() || (c == '\'' || c == '’') && !prev_hyphen {
            prev_hyphen = false;
        } else {
            return false;
        }
    }
    !prev_hyphen
}

/// The edit distance between `a` and `b`, in characters.
fn distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut cur = vec![i + 1];
        for (j, cb) in b.iter().enumerate() {
            cur.push(
                (prev[j + 1] + 1)
                    .min(cur[j] + 1)
                    .min(prev[j] + usize::from(ca != *cb)),
            );
        }
        prev = cur;
    }
    prev[b.len()]
}

/// The word a gloss says the entry is a spelling of, when it is one word
/// and the gloss ends with it or goes on after punctuation.
fn gloss_target(rest: &str) -> Option<&str> {
    let end = rest.find([';', ',', '.']).unwrap_or(rest.len());
    let (word, after) = rest.split_at(end);
    (wordlike(word) && (after.is_empty() || !after.starts_with(char::is_alphanumeric)))
        .then_some(word)
}

/// What one pass over the dump finds.
struct Reader<'a> {
    marks: &'a Marks,
    names: &'a [&'a str],
    splitter: &'a Splitter,
    found: &'a mut Found,
    /// Lemmas' flags by word and part of speech, for their forms.
    lemmas: HashMap<(String, String), (u32, u32)>,
    /// Inflected forms with no flags of their own: word, part of speech,
    /// lemmas.
    inherit: Vec<(String, String, Vec<String>)>,
    /// Two spellings of one word whose inflections pair up, as written.
    spellings: Vec<(String, String)>,
}

impl Reader<'_> {
    fn mask(&self, varieties: &[&str]) -> u32 {
        varieties
            .iter()
            .filter_map(|v| self.names.iter().position(|n| n == v))
            .fold(0, |m, i| m | (1 << i))
    }

    fn place_mask(&self, tags: &[String]) -> u32 {
        tags.iter()
            .filter_map(|t| self.marks.places.iter().find(|(p, _)| p == t))
            .fold(0, |m, (_, v)| m | self.mask(v))
    }

    fn is_place(&self, tag: &str) -> bool {
        self.marks.places.iter().any(|(p, _)| *p == tag)
    }

    fn mark(&mut self, word: &str, usual: u32, accepted: u32) {
        if wordlike(word) {
            self.found
                .mark(self.splitter.normalize(word), usual, accepted);
        }
    }

    fn pair(&mut self, a: &str, b: &str) {
        if wordlike(a) && wordlike(b) {
            let (a, b) = (self.splitter.normalize(a), self.splitter.normalize(b));
            if a != b {
                self.found.spellings.push(BTreeSet::from([a, b]));
            }
        }
    }

    /// The entry's spelling, if it says whose it is: the varieties that
    /// write it usually and those that accept it.
    fn spelling(&mut self, e: &Entry) -> Option<(u32, u32)> {
        let mut spelling = None;
        for h in &e.head_templates {
            if let Some((_, u, a)) = self.marks.heads.iter().find(|(x, _, _)| *x == h.expansion) {
                spelling = Some((self.mask(u), self.mask(a)));
            }
        }
        for s in &e.senses {
            if let Some(q) = &s.qualifier
                && let Some((_, v)) = self.marks.qualifiers.iter().find(|(x, _)| x == q)
            {
                let m = self.mask(v);
                spelling = Some((m, m));
            }
            let Some(gloss) = s.glosses.first() else {
                continue;
            };
            for (prefix, v) in self.marks.region_glosses {
                let target = gloss.strip_prefix(prefix).and_then(|g| {
                    let g = g.strip_prefix("standard ").unwrap_or(g);
                    g.strip_prefix("spelling of ")
                        .or_else(|| g.strip_prefix("form of "))
                        .and_then(gloss_target)
                });
                if let Some(target) = target {
                    let m = self.mask(v);
                    spelling = Some((m, m));
                    self.pair(&e.word, target);
                    self.spellings.push((e.word.clone(), target.to_string()));
                }
            }
            if !self.marks.prereform.is_empty()
                && let Some(rest) = gloss.strip_prefix("pre-reform spelling (used until ")
                && let Some((years, rest)) = rest.split_once(") of ")
                && (years == "1990"
                    || years == "1990 in Portugal"
                    || years == "1943 in Brazil and 1990 in Portugal")
                && let Some(target) = gloss_target(rest)
            {
                spelling = Some((0, self.mask(self.marks.prereform)));
                self.pair(&e.word, target);
            }
        }
        // An entry that is only another spelling of a word, in places.
        if spelling.is_none()
            && !e.senses.is_empty()
            && e.senses.iter().all(|s| {
                s.tags.iter().any(|t| t == "alt-of")
                    && self.place_mask(&s.tags) != 0
                    && !s.tags.iter().any(|t| HEDGES.contains(&t.as_str()))
                    && s.alt_of.first().is_some_and(|l| wordlike(&l.word))
            })
        {
            let mut m = 0;
            for s in &e.senses {
                m |= self.place_mask(&s.tags);
                let target = s.alt_of[0].word.clone();
                self.pair(&e.word, &target);
                self.spellings.push((e.word.clone(), target));
            }
            spelling = Some((m, m));
        }
        // A spelling's alternatives for another place, and nothing else.
        if spelling.is_some() {
            for f in &e.forms {
                let pm = self.place_mask(&f.tags);
                if pm != 0
                    && f.tags.iter().any(|t| t == "alternative")
                    && f.tags
                        .iter()
                        .all(|t| t == "alternative" || self.is_place(t))
                    && wordlike(&f.form)
                {
                    self.pair(&e.word, &f.form);
                    self.spellings.push((e.word.clone(), f.form.clone()));
                    self.mark(&f.form, pm, pm);
                }
            }
        }
        spelling
    }

    /// The places of a word all of whose senses are tagged with some,
    /// unhedged.
    fn places(&self, e: &Entry) -> Option<u32> {
        let hedged = e.head_templates.iter().any(|h| {
            h.expansion.starts_with('(')
                && ["chiefly", "colloquial", "especially", "mainly"]
                    .iter()
                    .any(|w| h.expansion.contains(w))
        });
        if hedged || e.senses.is_empty() {
            return None;
        }
        let mut m = 0;
        for s in &e.senses {
            let pm = self.place_mask(&s.tags);
            if pm == 0 || s.tags.iter().any(|t| HEDGES.contains(&t.as_str())) {
                return None;
            }
            m |= pm;
        }
        Some(m)
    }

    fn entry(&mut self, e: &Entry) {
        if e.word.is_empty() || !wordlike(&e.word) {
            return;
        }
        let all = (1u32 << self.names.len()) - 1;
        let is_form = !e.senses.is_empty()
            && e.senses
                .iter()
                .all(|s| !s.form_of.is_empty() || s.tags.iter().any(|t| t == "form-of"));
        let spelling = self
            .spelling(e)
            .or_else(|| (!is_form).then(|| self.places(e)).flatten().map(|m| (m, m)));
        let (u, a) = match spelling {
            Some(s) => s,
            None if is_form => {
                let lemmas = e
                    .senses
                    .iter()
                    .flat_map(|s| s.form_of.iter().map(|l| l.word.clone()))
                    .collect();
                self.inherit.push((e.word.clone(), e.pos.clone(), lemmas));
                return;
            }
            None => (all, all),
        };
        self.mark(&e.word, u, a);
        if !is_form {
            let m = self
                .lemmas
                .entry((e.word.clone(), e.pos.clone()))
                .or_insert((0, 0));
            m.0 |= u;
            m.1 |= a;
        }
        // The lemma's inflection table, cell by cell: a cell tagged with
        // places is theirs, the others' in its slot the rest's.
        let mut slots: HashMap<Vec<&str>, Vec<(&str, u32)>> = HashMap::new();
        for f in &e.forms {
            let rest: Vec<&str> = f
                .tags
                .iter()
                .map(String::as_str)
                .filter(|t| !self.is_place(t))
                .collect();
            if rest.is_empty()
                || !wordlike(&f.form)
                || f.tags.iter().any(|t| NOT_INFLECTIONS.contains(&t.as_str()))
            {
                continue;
            }
            let mut rest = rest;
            rest.sort_unstable();
            slots
                .entry(rest)
                .or_default()
                .push((f.form.as_str(), self.place_mask(&f.tags)));
        }
        for cells in slots.values() {
            let tagged = cells.iter().fold(0, |m, (_, pm)| m | pm);
            for &(form, pm) in cells {
                if pm != 0 {
                    self.mark(form, u & pm, a & pm);
                } else if tagged != 0 && u & !tagged != 0 {
                    self.mark(form, u & !tagged, a & !tagged);
                } else {
                    self.mark(form, u, a);
                }
            }
            let plain: Vec<&str> = cells.iter().filter(|c| c.1 == 0).map(|c| c.0).collect();
            let placed: Vec<&str> = cells.iter().filter(|c| c.1 != 0).map(|c| c.0).collect();
            if let ([p], [q]) = (plain.as_slice(), placed.as_slice()) {
                self.pair(p, q);
            }
        }
    }
}

fn lines(path: &Path) -> anyhow::Result<impl Iterator<Item = std::io::Result<String>>> {
    let file = std::fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
    Ok(BufReader::new(GzDecoder::new(file)).lines())
}

/// Read `lang`'s kaikki dump at `path` into `found`.
pub fn read(lang: &str, path: &Path, splitter: &Splitter, found: &mut Found) -> anyhow::Result<()> {
    let marks = match lang {
        "pt" => &PORTUGUESE,
        "es" => &SPANISH,
        "fr" => &FRENCH,
        other => bail!("no Wiktionary place tags for `{other}`"),
    };
    let names = varieties(lang);
    let all = (1u32 << names.len()) - 1;
    let mut reader = Reader {
        marks,
        names,
        splitter,
        found,
        lemmas: HashMap::new(),
        inherit: Vec::new(),
        spellings: Vec::new(),
    };
    let mut entries = 0usize;
    for (n, line) in lines(path)?.enumerate() {
        let line = line.with_context(|| format!("reading {}", path.display()))?;
        let e: Entry = serde_json::from_str(&line)
            .with_context(|| format!("{}: line {} is no entry", path.display(), n + 1))?;
        reader.entry(&e);
        entries += 1;
    }
    // Inflected forms: their lemmas' flags, or everyone's for a lemma not
    // found.
    for (form, pos, lemmas) in std::mem::take(&mut reader.inherit) {
        let (mut u, mut a) = if lemmas.is_empty() {
            (all, all)
        } else {
            (0, 0)
        };
        for l in &lemmas {
            let (lu, la) = reader
                .lemmas
                .get(&(l.clone(), pos.clone()))
                .copied()
                .unwrap_or((all, all));
            u |= lu;
            a |= la;
        }
        reader.mark(&form, u, a);
    }
    // The inflections of two spellings of a word, cell by cell, where they
    // differ as the spellings do: a second pass, for those lemmas' tables.
    let wanted: HashSet<&str> = reader
        .spellings
        .iter()
        .flat_map(|(a, b)| [a.as_str(), b.as_str()])
        .collect();
    let mut tables: HashMap<(String, String), HashMap<Vec<String>, String>> = HashMap::new();
    for line in lines(path)? {
        let line = line?;
        let Ok(e) = serde_json::from_str::<Entry>(&line) else {
            continue;
        };
        if !wanted.contains(e.word.as_str()) {
            continue;
        }
        let table = tables.entry((e.word.clone(), e.pos.clone())).or_default();
        for f in &e.forms {
            if !wordlike(&f.form)
                || f.tags.is_empty()
                || f.tags
                    .iter()
                    .any(|t| NOT_INFLECTIONS.contains(&t.as_str()) || reader.is_place(t))
            {
                continue;
            }
            let mut tags = f.tags.clone();
            tags.sort_unstable();
            table.entry(tags).or_insert_with(|| f.form.clone());
        }
    }
    let mut cells = 0usize;
    for (a, b) in std::mem::take(&mut reader.spellings) {
        let apart = distance(&a, &b).max(2);
        for ((word, pos), ta) in &tables {
            if *word != a {
                continue;
            }
            let Some(tb) = tables.get(&(b.clone(), pos.clone())) else {
                continue;
            };
            for (tags, fa) in ta {
                if let Some(fb) = tb.get(tags)
                    && fa != fb
                    && distance(fa, fb) <= apart
                {
                    reader.pair(fa, fb);
                    cells += 1;
                }
            }
        }
    }
    // In a canonical order: the pairs came in the order of hash maps, and
    // the groups are numbered in the order of the pairs.
    reader.found.spellings.sort_unstable();
    reader.found.spellings.dedup();
    log::info!(
        "{}: {entries} entries, {} words, {} pairs of spellings ({cells} from inflection tables)",
        path.display(),
        reader.found.masks.len(),
        reader.found.spellings.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DUMP: &[&str] = &[
        // A European spelling, with Brazil's.
        r#"{"word": "facto", "pos": "noun", "head_templates": [{"expansion": "facto m"}, {"expansion": "(European Portuguese spelling)"}], "senses": [{"tags": ["masculine"], "glosses": ["fact"]}], "forms": [{"form": "factos", "tags": ["plural"]}, {"form": "fato", "tags": ["alternative", "Brazil"]}]}"#,
        // Brazil's spelling; and the same letters, everyone's word.
        r#"{"word": "fato", "pos": "noun", "senses": [{"tags": ["Brazil", "alt-of", "masculine", "standard"], "glosses": ["Brazilian Portuguese standard form of facto"], "alt_of": [{"word": "facto"}]}], "forms": [{"form": "fatos", "tags": ["plural"]}]}"#,
        r#"{"word": "fato", "pos": "noun", "senses": [{"tags": ["masculine"], "glosses": ["suit"]}]}"#,
        // A spelling by gloss, and an inflected form of it.
        r#"{"word": "económico", "pos": "adj", "senses": [{"glosses": ["European Portuguese standard spelling of econômico"]}]}"#,
        r#"{"word": "económica", "pos": "adj", "senses": [{"tags": ["form-of", "feminine"], "glosses": ["feminine of económico"], "form_of": [{"word": "económico"}]}]}"#,
        // A cell one place writes otherwise.
        r#"{"word": "falar", "pos": "verb", "senses": [{"glosses": ["to speak"]}], "forms": [{"form": "falámos", "tags": ["first-person", "plural", "preterite"]}, {"form": "falamos", "tags": ["Brazil", "first-person", "plural", "preterite"]}, {"form": "falamos", "tags": ["first-person", "plural", "present"]}]}"#,
        // A word of one place, and one only chiefly there.
        r#"{"word": "autocarro", "pos": "noun", "senses": [{"tags": ["Portugal", "masculine"], "glosses": ["bus"]}]}"#,
        r#"{"word": "legal", "pos": "adj", "senses": [{"tags": ["Brazil", "colloquial"], "glosses": ["cool"]}]}"#,
        // A pre-reform spelling.
        r#"{"word": "acção", "pos": "noun", "senses": [{"glosses": ["pre-reform spelling (used until 1943 in Brazil and 1990 in Portugal) of ação; still used"]}]}"#,
        r#"{"word": "ação", "pos": "noun", "senses": [{"glosses": ["action"]}]}"#,
    ];

    #[test]
    fn places_and_spellings_come_from_clear_marks_alone() {
        let dir = std::env::temp_dir().join(format!("kaikki-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("pt.jsonl.gz");
        let mut gz = flate2::write::GzEncoder::new(
            std::fs::File::create(&path).unwrap(),
            flate2::Compression::fast(),
        );
        std::io::Write::write_all(&mut gz, DUMP.join("\n").as_bytes()).unwrap();
        gz.finish().unwrap();
        let mut found = Found::default();
        read("pt", &path, &Splitter::new("pt"), &mut found).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        let (br, pt) = (1, 2);
        assert_eq!(found.masks["facto"], (pt, pt));
        assert_eq!(found.masks["factos"], (pt, pt));
        // Brazil's `fato`, and everyone's suit.
        assert_eq!(found.masks["fato"], (br | pt, br | pt));
        assert_eq!(found.masks["económica"], (pt, pt));
        assert_eq!(found.masks["falámos"], (pt, pt));
        assert_eq!(found.masks["falamos"], (br | pt, br | pt));
        assert_eq!(found.masks["autocarro"], (pt, pt));
        assert_eq!(found.masks["legal"], (br | pt, br | pt));
        assert_eq!(found.masks["acção"], (0, pt));
        let paired = |a: &str, b: &str| {
            found
                .spellings
                .iter()
                .any(|s| s.contains(a) && s.contains(b))
        };
        assert!(paired("facto", "fato"));
        assert!(paired("factos", "fatos"));
        assert!(paired("económico", "econômico"));
        assert!(paired("falámos", "falamos"));
        assert!(paired("acção", "ação"));
    }

    #[test]
    fn a_gloss_names_one_word() {
        assert_eq!(gloss_target("anónimo"), Some("anónimo"));
        assert_eq!(gloss_target("ação; still used"), Some("ação"));
        assert_eq!(gloss_target("fato de macaco"), None);
        assert!(wordlike("guarda-chuva") && wordlike("d'água"));
        assert!(!wordlike("à:") && !wordlike("-se") && !wordlike("a--b"));
        assert_eq!(distance("económico", "econômico"), 1);
    }
}
