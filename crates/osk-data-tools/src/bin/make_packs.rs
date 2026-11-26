//! `make-packs` — write one release of language packs.
//!
//! Reads `packs.toml` (which files make up each pack, their licences and
//! sources) and `sources.lock` (the sha256 of each pack's upstream
//! inputs), and writes a release directory, every file of it a release
//! asset:
//!
//! - `packs.json` — the [`osk_data::pack::Index`]: every pack's manifest,
//!   and the asset it is served in;
//! - `pack.<id>.tar.gz` — each pack's files, at their tree paths, in one
//!   gzipped ustar archive ([`osk_data::pack::pack_archive`]): one asset a
//!   pack, as GitHub keeps at most 1 000 in a release;
//! - `NOTICES.md` — the licence notices that go with any pack.
//!
//! The release version is the crates' (`osk_data::RELEASE_VERSION`), so
//! the format crate and the data it was released with share one number.
//!
//! A language's keyboard layouts (`layouts/<id>.json`) join its pack, as
//! `packs.toml`'s `[layouts.packs]` says; a language with layouts and no
//! model gets a pack of its layouts alone, named in `[layouts.names]`. A
//! layout made from another source than xkeyboard-config's (Keyman's) adds
//! that source's licence to its pack, as `[layouts.added]` says.
//! A language's transliteration table (`translit/<lang>.txt`) joins the
//! pack of the same id, with `[translit]`'s licence and source, or, for a
//! table made from another's work (a Keyman keyboard's rules), those
//! `[translit.added]` gives it, and those of each `[[translit.fitted]]`
//! naming it, whose dataset's romanizations its costs were fitted to
//! (`make fit-translit`).
//!
//! A prediction pack names the corpora its model was built from as
//! `sources.lock` records them: Tatoeba's (`corpora/<lang>.txt`), the
//! Dakshina dataset's Wikipedia text (`dakshina/<lang>.txt`), the wikis'
//! own (`wikipedia/<lang>.txt`), FineWeb-2's web text (`fineweb/<lang>.txt`),
//! FineWeb's, English's (`fineweb-v1/en.txt`), MADLAD-400's (`madlad/<lang>.txt`), the African Storybook's stories
//! (`storybooks/<lang>.txt`), StoryWeaver's (`storyweaver/<lang>.txt`),
//! SMOL's translations (`smol/<lang>.txt`), GlotCC's (`glotcc/<lang>.txt`),
//! with their licences; and eBible.org's Bibles (`ebible/<lang>.txt`), each
//! under its own copyright, which `[prediction.ebible.bibles]` names as its
//! language's source (a language's several Bibles, read into one corpus, as
//! its sources, each with its licence).
//!
//! It refuses rather than guesses:
//! - a committed data file that no pack claims, or that two packs claim;
//! - a prediction language with files but no name in `packs.toml`, or a
//!   name with no files, or a model `sources.lock` records no corpus of;
//! - a transliteration table no pack of its language takes, or a fitted
//!   or added one that is not there;
//! - layouts given to a pack that does not exist and has no name, or a
//!   name for a pack that has no layouts or exists already;
//! - a component that is missing or whose path climbs out of the tree;
//! - an output directory holding files it did not write.
//!
//! What it wrote is then read back: the index must be sound and every
//! pack's asset must open to exactly the tree files its manifest
//! describes.
//!
//! With `--bundle` it also writes the all-languages bundle,
//! `ki-osk-lang-<version>.tar`: every pack installed from those assets the
//! way a client installs one, so the bundle is a client install by
//! construction rather than a second copy of the recipe. It is a plain
//! ustar archive whose bytes depend only on the release: entries in name
//! order, owner 0, modes 0644 and 0755, every time `--mtime`.
//!
//! Usage:
//!   make-packs [--root .] [--packs packs.toml] [--lock sources.lock] \
//!              [--out dist/packs] [--bundle [--mtime SECONDS]]

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, bail, ensure};
use clap::Parser;
use osk_data::RELEASE_VERSION;
use osk_data::pack::{
    Component, Download, INDEX_SCHEMA, Index, License, Manifest, NOTICES_PATH, SCHEMA, Source,
    encoding, kind, pack_archive, sha256_hex,
};
use osk_data::ustar;
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "make-packs")]
struct Args {
    /// The data tree.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Pack definitions.
    #[arg(long, default_value = "packs.toml")]
    packs: PathBuf,
    /// The lockfile recording each output's inputs.
    #[arg(long, default_value = "sources.lock")]
    lock: PathBuf,
    /// The release directory to write.
    #[arg(long, default_value = "dist/packs")]
    out: PathBuf,
    /// Also write the all-languages bundle, `ki-osk-lang-<version>.tar`.
    #[arg(long)]
    bundle: bool,
    /// The time every bundle entry carries, in seconds since the epoch
    /// (the release commit's, for a release).
    #[arg(long, default_value_t = 0)]
    mtime: u64,
}

/// The index's file name in a release.
const INDEX: &str = "packs.json";

#[derive(Deserialize)]
struct Packs {
    prediction: Prediction,
    layouts: Layouts,
    translit: Translit,
    #[serde(default)]
    pack: Vec<PackDef>,
}

/// The transliteration tables: the licence and source each adds to its
/// pack, or those of the work a table was made from in their place
/// (`added`, by language), those each dataset their costs were fitted to
/// adds, and those the English words a language writes in its script add
/// (`translit/<lang>.english.txt`).
#[derive(Deserialize)]
struct Translit {
    license: License,
    source: SourceDef,
    #[serde(default)]
    added: BTreeMap<String, AddedDef>,
    #[serde(default)]
    fitted: Vec<Fitted>,
    english: Option<Added>,
}

/// The tables whose costs were fitted to one dataset's romanizations.
#[derive(Deserialize)]
struct Fitted {
    /// None where the dataset is named for what was learned from it, its
    /// words not in the tables.
    #[serde(default)]
    license: Option<License>,
    source: SourceDef,
    langs: Vec<String>,
}

/// A licence and a source that a kind of file adds to the pack it joins.
#[derive(Deserialize)]
struct Added {
    license: License,
    source: SourceDef,
}

/// The keyboard layouts: the licence and source every pack with one takes,
/// which pack each language's layouts join, the names of the packs of
/// layouts alone, and the licences and sources a layout made from another
/// source than xkeyboard-config adds to its pack (Keyman's).
#[derive(Deserialize)]
struct Layouts {
    license: License,
    source: SourceDef,
    packs: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    names: BTreeMap<String, String>,
    #[serde(default)]
    added: BTreeMap<String, AddedDef>,
}

#[derive(Deserialize)]
struct Prediction {
    license: License,
    source: SourceDef,
    names: BTreeMap<String, String>,
    /// The languages with dialect flags, and the licences and sources
    /// those add to their packs.
    #[serde(default)]
    dialects: BTreeMap<String, AddedDef>,
    /// The languages written without spaces between words, whose packs
    /// carry their word list (`wordbreak/<lang>.txt`), and the licences
    /// and sources it adds.
    #[serde(default)]
    wordbreak: BTreeMap<String, AddedDef>,
    /// The Dakshina dataset's Wikipedia text: the licence and source of
    /// the models built from it.
    dakshina: Added,
    /// The wikis' own text (their CirrusSearch dumps): the licence and
    /// source of the models built from it.
    wikipedia: Added,
    /// FineWeb-2's web text: the licence and source of the models built
    /// from it.
    fineweb: Added,
    /// FineWeb's (FineWeb-2's English predecessor): English's.
    fineweb_v1: Added,
    /// MADLAD-400's web text: the licence and source of the models built
    /// from it.
    madlad: Added,
    /// The African Storybook's stories: the licence and source of the
    /// models built from them.
    storybooks: Added,
    /// StoryWeaver's stories: the licence and source of the models built
    /// from them.
    storyweaver: Added,
    /// SMOL's translations: the licence and source of the models built
    /// from them.
    smol: Added,
    /// GlotCC's web text: the licence and source of the models built from
    /// it.
    glotcc: Added,
    /// eBible.org's Bibles: the licence of the models built from them, and
    /// each language's Bible.
    ebible: Bibles,
}

/// eBible.org's Bibles, each under its own copyright: the licence they
/// share, and each language's Bible, or its Bibles, named with their
/// holders, as its model's sources.
#[derive(Deserialize)]
struct Bibles {
    license: License,
    bibles: BTreeMap<String, OneOrMore<BibleDef>>,
}

/// One Bible: its name and holders, where it is, and its licence where it
/// is not the one the Bibles share (Gamo's New Testament's CC BY 4.0).
#[derive(Deserialize)]
struct BibleDef {
    name: String,
    url: String,
    #[serde(default)]
    license: Option<License>,
}

/// A table, or a list of them.
#[derive(Deserialize)]
#[serde(untagged)]
enum OneOrMore<T> {
    One(T),
    More(Vec<T>),
}

impl<T> OneOrMore<T> {
    fn all(&self) -> &[T] {
        match self {
            Self::One(t) => std::slice::from_ref(t),
            Self::More(ts) => ts,
        }
    }
}

/// What a file beside a language's model adds to its pack.
#[derive(Deserialize)]
struct AddedDef {
    licenses: Vec<License>,
    sources: Vec<SourceDef>,
}

#[derive(Deserialize)]
struct PackDef {
    id: String,
    name: String,
    components: Vec<ComponentDef>,
    licenses: Vec<License>,
    sources: Vec<SourceDef>,
}

#[derive(Deserialize)]
struct ComponentDef {
    kind: String,
    path: String,
}

#[derive(Deserialize)]
struct SourceDef {
    name: String,
    url: String,
    #[serde(default)]
    lock: Option<String>,
    #[serde(default)]
    input: Option<String>,
}

/// The directories whose files must all belong to a pack.
const DATA_DIRS: [&str; 11] = [
    "predict",
    "wordbreak",
    "translit",
    "pinyin",
    "zhuyin",
    "jyutping",
    "kana",
    "rules",
    "layouts",
    "emoji",
    "offensive",
];

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    let defs: Packs = toml::from_str(
        &fs::read_to_string(&args.packs)
            .with_context(|| format!("reading {}", args.packs.display()))?,
    )
    .with_context(|| format!("parsing {}", args.packs.display()))?;
    let lock: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&args.lock)
            .with_context(|| format!("reading {}", args.lock.display()))?,
    )
    .with_context(|| format!("parsing {}", args.lock.display()))?;
    let recorded = |entry: &str, input: &str| -> Option<String> {
        lock["outputs"][entry]["inputs"][input]
            .as_str()
            .map(str::to_string)
    };

    let mut manifests: Vec<Manifest> = Vec::new();

    // Prediction packs, one per lexicon + n-gram pair.
    let langs = prediction_langs(&args.root)?;
    let named: BTreeSet<&String> = defs.prediction.names.keys().collect();
    let found: BTreeSet<&String> = langs.iter().collect();
    if let Some(l) = found.difference(&named).next() {
        bail!("predict/{l}.* is committed but packs.toml gives `{l}` no name");
    }
    if let Some(l) = named.difference(&found).next() {
        bail!("packs.toml names `{l}` but predict/{l}.fst is not in the tree");
    }
    for lang in defs.prediction.dialects.keys() {
        if !langs.contains(lang) {
            bail!(
                "packs.toml gives `{lang}` dialect flags but predict/{lang}.fst is not in the tree"
            );
        }
    }
    for lang in defs.prediction.wordbreak.keys() {
        if !langs.contains(lang) {
            bail!(
                "packs.toml gives `{lang}` a word list but predict/{lang}.fst is not in the tree"
            );
        }
    }
    for lang in &langs {
        let p = &defs.prediction;
        let dialects_path = format!("predict/{lang}.dialects.bin");
        let dialects = added(
            &args.root,
            &dialects_path,
            p.dialects.get(lang),
            "dialects",
            lang,
        )?;
        let wordbreak_path = format!("wordbreak/{lang}.txt");
        let wordbreak = added(
            &args.root,
            &wordbreak_path,
            p.wordbreak.get(lang),
            "wordbreak",
            lang,
        )?;
        let mut components = vec![
            component(&args.root, kind::LEXICON, &format!("predict/{lang}.fst"))?,
            component(
                &args.root,
                kind::NGRAM,
                &format!("predict/{lang}.ngram.bin"),
            )?,
            component(
                &args.root,
                kind::FORMS,
                &format!("predict/{lang}.forms.fst"),
            )?,
        ];
        // The corpora the model was built from, as the lock records them:
        // the language's own, or, for a model built from another's text
        // (Hinglish, hi-Latn, from Hindi's), its primary subtag's.
        let entry = format!("predict/{lang}");
        let primary = lang.split('-').next().unwrap_or(lang);
        let mut licenses = Vec::new();
        let mut sources = Vec::new();
        for (dir, license, source) in [
            ("corpora", &p.license, &p.source),
            ("dakshina", &p.dakshina.license, &p.dakshina.source),
            ("wikipedia", &p.wikipedia.license, &p.wikipedia.source),
            ("fineweb", &p.fineweb.license, &p.fineweb.source),
            ("fineweb-v1", &p.fineweb_v1.license, &p.fineweb_v1.source),
            ("madlad", &p.madlad.license, &p.madlad.source),
            ("storybooks", &p.storybooks.license, &p.storybooks.source),
            ("storyweaver", &p.storyweaver.license, &p.storyweaver.source),
            ("smol", &p.smol.license, &p.smol.source),
            ("glotcc", &p.glotcc.license, &p.glotcc.source),
        ] {
            let sha = recorded(&entry, &format!("{dir}/{lang}.txt"))
                .or_else(|| recorded(&entry, &format!("{dir}/{primary}.txt")));
            if let Some(sha256) = sha {
                licenses.push(license.clone());
                sources.push(Source {
                    name: source.name.clone(),
                    url: source.url.clone(),
                    sha256: Some(sha256),
                });
            }
        }
        // A Bible's model names that Bible and its holders, each of a
        // language's Bibles, read into one corpus, with its licence.
        if let Some(sha256) = recorded(&entry, &format!("ebible/{lang}.txt")) {
            let bibles =
                p.ebible.bibles.get(lang).with_context(|| {
                    format!("[prediction.ebible.bibles] names no Bible of {lang}")
                })?;
            ensure!(
                !bibles.all().is_empty(),
                "[prediction.ebible.bibles] names no Bible of {lang}"
            );
            for bible in bibles.all() {
                let license = bible.license.as_ref().unwrap_or(&p.ebible.license);
                if !licenses.contains(license) {
                    licenses.push(license.clone());
                }
                sources.push(Source {
                    name: bible.name.clone(),
                    url: bible.url.clone(),
                    sha256: Some(sha256.clone()),
                });
            }
        }
        ensure!(
            !sources.is_empty(),
            "sources.lock records no corpus of {entry} (corpora/, dakshina/, wikipedia/, fineweb/, fineweb-v1/, madlad/, storybooks/, storyweaver/, smol/, glotcc/ or ebible/, {lang}.txt or {primary}.txt)"
        );
        // Dialect flags and word lists carry their own sources' licences
        // into the pack.
        for (def, kind, path) in [
            (dialects, kind::DIALECTS, &dialects_path),
            (wordbreak, kind::WORDBREAK, &wordbreak_path),
        ] {
            let Some(def) = def else { continue };
            components.push(component(&args.root, kind, path)?);
            // A licence the pack names already (a word list made from its
            // model's corpus, Shan's syllables) is named once.
            for l in &def.licenses {
                if !licenses.contains(l) {
                    licenses.push(l.clone());
                }
            }
            sources.extend(def.sources.iter().map(|s| Source {
                name: s.name.clone(),
                url: s.url.clone(),
                sha256: match (&s.lock, &s.input) {
                    (Some(entry), Some(input)) => recorded(entry, input),
                    _ => None,
                },
            }));
        }
        manifests.push(Manifest {
            schema: SCHEMA,
            id: lang.clone(),
            name: p.names[lang].clone(),
            version: RELEASE_VERSION.to_string(),
            format: osk_data::codec::VERSION,
            components,
            licenses,
            sources,
        });
    }

    // The input-method packs, as listed.
    for def in &defs.pack {
        let mut components = Vec::new();
        for c in &def.components {
            components.push(component(&args.root, &c.kind, &c.path)?);
        }
        let sources = def
            .sources
            .iter()
            .map(|s| Source {
                name: s.name.clone(),
                url: s.url.clone(),
                sha256: match (&s.lock, &s.input) {
                    (Some(entry), Some(input)) => recorded(entry, input),
                    _ => None,
                },
            })
            .collect();
        manifests.push(Manifest {
            schema: SCHEMA,
            id: def.id.clone(),
            name: def.name.clone(),
            version: RELEASE_VERSION.to_string(),
            format: osk_data::codec::VERSION,
            components,
            licenses: def.licenses.clone(),
            sources,
        });
    }

    add_layouts(&args.root, &defs.layouts, &mut manifests)?;
    add_translit(&args.root, &defs.translit, &recorded, &mut manifests)?;
    check_coverage(&args.root, &manifests)?;

    prepare_out(&args.out)?;
    let mut assets = BTreeMap::new();
    for m in &manifests {
        let files = m
            .components
            .iter()
            .map(|c| Ok((c.path.as_str(), fs::read(args.root.join(&c.path))?)))
            .collect::<std::io::Result<Vec<_>>>()?;
        let files: Vec<(&str, &[u8])> = files.iter().map(|(p, b)| (*p, b.as_slice())).collect();
        let asset = format!("pack.{}.tar.gz", m.id);
        let packed = pack_archive(&files).with_context(|| format!("archiving pack {}", m.id))?;
        fs::write(args.out.join(&asset), &packed).with_context(|| format!("writing {asset}"))?;
        assets.insert(
            m.id.clone(),
            Download {
                asset,
                encoding: encoding::TAR_GZIP.to_string(),
                size: packed.len() as u64,
                sha256: sha256_hex(&packed),
            },
        );
    }
    let notices = fs::read(args.root.join(NOTICES_PATH)).context("reading NOTICES.md")?;
    fs::write(args.out.join(NOTICES_PATH), &notices)?;
    let index = Index {
        schema: INDEX_SCHEMA,
        version: RELEASE_VERSION.to_string(),
        packs: manifests,
        assets,
        notices: Download {
            asset: NOTICES_PATH.to_string(),
            encoding: encoding::IDENTITY.to_string(),
            size: notices.len() as u64,
            sha256: sha256_hex(&notices),
        },
    };
    let problems = index.problems();
    if let Some(p) = problems.first() {
        bail!(
            "the index is unsound ({} problems), first: {p}",
            problems.len()
        );
    }
    fs::write(args.out.join(INDEX), index.to_json()?).context("writing packs.json")?;

    let (files, size, packed) = check_release(&args.root, &args.out)?;
    let share_alike = index.packs.iter().filter(|m| m.is_share_alike()).count();
    log::info!(
        "release {RELEASE_VERSION} in {}: {} packs ({share_alike} share-alike), \
         {files} files, {:.1} MiB served as {:.1} MiB",
        args.out.display(),
        index.packs.len(),
        mib(size),
        mib(packed)
    );
    if args.bundle {
        let name = format!("ki-osk-lang-{RELEASE_VERSION}.tar");
        let tar = bundle(&args.out, args.mtime)?;
        fs::write(args.out.join(&name), &tar).with_context(|| format!("writing {name}"))?;
        log::info!("bundle {name}: {:.1} MiB", mib(tar.len() as u64));
    }
    Ok(())
}

/// Every pack of the release in `out`, installed as a client installs
/// them — each asset checked against the index and opened, each file
/// checked against its manifest, the manifests and the notices beside
/// them — as one ustar archive.
fn bundle(out: &Path, mtime: u64) -> anyhow::Result<Vec<u8>> {
    let index = Index::parse(&fs::read_to_string(out.join(INDEX))?)?;
    // Name → contents; a directory is its name with a trailing `/` and no
    // contents. Name order is the archive's order, so it is fixed.
    let mut entries: BTreeMap<String, Option<Vec<u8>>> = BTreeMap::new();
    let mut add = |path: String, bytes: Vec<u8>| {
        let mut dir = path.as_str();
        while let Some((parent, _)) = dir.rsplit_once('/') {
            entries.insert(format!("{parent}/"), None);
            dir = parent;
        }
        entries.insert(path, Some(bytes));
    };
    for m in &index.packs {
        let d = &index.assets[&m.id];
        for (path, file) in d.open_pack(&fs::read(out.join(&d.asset))?, m)? {
            add(path, file);
        }
        add(m.install_path(), m.to_json()?.into_bytes());
    }
    let notices = fs::read(out.join(&index.notices.asset))?;
    ensure!(index.notices.matches(&notices), "NOTICES.md changed");
    add(NOTICES_PATH.to_string(), notices);

    let mut tar = Vec::new();
    for (name, bytes) in &entries {
        ustar::entry(&mut tar, name, bytes.as_deref(), mtime)?;
    }
    ustar::end(&mut tar);
    Ok(tar)
}

#[allow(clippy::cast_precision_loss)]
fn mib(bytes: u64) -> f64 {
    bytes as f64 / 1_048_576.0
}

/// Make `out` an empty release directory. A previous run's files go;
/// anything else is refused rather than deleted, so a mistaken `--out`
/// cannot empty a directory that is not ours.
fn prepare_out(out: &Path) -> anyhow::Result<()> {
    if out.exists() {
        let mut previous = Vec::new();
        for entry in fs::read_dir(out).with_context(|| format!("reading {}", out.display()))? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            let ours = entry.file_type()?.is_file()
                && (name.ends_with(".gz")
                    || name.ends_with(".json")
                    || name.ends_with(".tar")
                    || name.ends_with(".tar.xz")
                    || name == NOTICES_PATH
                    || name == "SHA256SUMS");
            ensure!(
                ours,
                "{} holds {name}, which make-packs did not write; give it its own directory",
                out.display()
            );
            previous.push(entry.path());
        }
        for p in previous {
            fs::remove_file(&p).with_context(|| format!("removing {}", p.display()))?;
        }
    }
    fs::create_dir_all(out).with_context(|| format!("mkdir -p {}", out.display()))?;
    Ok(())
}

/// Read the release back as a client would: the index must be sound, the
/// tree must match every manifest, and every pack's asset must open to
/// exactly the files its manifest describes. Returns (files, bytes, bytes
/// served).
fn check_release(root: &Path, out: &Path) -> anyhow::Result<(usize, u64, u64)> {
    let index = Index::parse(&fs::read_to_string(out.join(INDEX))?)?;
    ensure!(index.problems().is_empty(), "packs.json reads back unsound");
    let (mut files, mut size, mut packed) = (0, 0, 0);
    for m in &index.packs {
        let problems = m.verify(root);
        ensure!(problems.is_empty(), "pack {}: {problems:?}", m.id);
        let d = &index.assets[&m.id];
        let opened = d.open_pack(&fs::read(out.join(&d.asset))?, m)?;
        files += opened.len();
        size += m.size();
        packed += d.size;
    }
    let notices = fs::read(out.join(&index.notices.asset))?;
    ensure!(
        index.notices.matches(&notices),
        "NOTICES.md reads back changed"
    );
    Ok((files, size, packed))
}

/// The languages with a lexicon in `predict/`.
fn prediction_langs(root: &Path) -> anyhow::Result<Vec<String>> {
    let mut out: Vec<String> = fs::read_dir(root.join("predict"))
        .context("reading predict/")?
        .flatten()
        .filter_map(|e| {
            let n = e.file_name().to_string_lossy().to_string();
            // `<lang>.fst`, not `<lang>.forms.fst`.
            n.strip_suffix(".fst")
                .filter(|l| !l.contains('.'))
                .map(str::to_string)
        })
        .collect();
    out.sort();
    Ok(out)
}

/// The packs.toml entry of `path`, a file beside a language's model
/// (`[prediction.<table>.<lang>]`): both or neither.
fn added<'a>(
    root: &Path,
    path: &str,
    def: Option<&'a AddedDef>,
    table: &str,
    lang: &str,
) -> anyhow::Result<Option<&'a AddedDef>> {
    match (root.join(path).is_file(), def) {
        (true, Some(def)) => Ok(Some(def)),
        (true, None) => {
            bail!("{path} is committed but packs.toml gives no [prediction.{table}.{lang}]")
        }
        (false, Some(_)) => {
            bail!("packs.toml gives [prediction.{table}.{lang}] but {path} is not in the tree")
        }
        (false, None) => Ok(None),
    }
}

fn component(root: &Path, kind: &str, path: &str) -> anyhow::Result<Component> {
    let mut c = Component {
        kind: kind.to_string(),
        path: path.to_string(),
        size: 0,
        sha256: String::new(),
    };
    ensure!(c.has_safe_path(), "{path}: not a safe tree path");
    let bytes = fs::read(root.join(path)).with_context(|| format!("reading {path}"))?;
    c.size = bytes.len() as u64;
    c.sha256 = sha256_hex(&bytes);
    Ok(c)
}

/// Each language's layouts into its pack, with the layouts' licence and
/// source, or into a pack of their own where the language has none.
fn add_layouts(root: &Path, l: &Layouts, manifests: &mut Vec<Manifest>) -> anyhow::Result<()> {
    for id in l.names.keys() {
        if manifests.iter().any(|m| &m.id == id) {
            bail!("[layouts.names] names `{id}`, which is a pack already");
        }
        if !l.packs.contains_key(id) {
            bail!("[layouts.names] names `{id}`, but [layouts.packs] gives it no layouts");
        }
    }
    for id in l.added.keys() {
        if !l.packs.values().flatten().any(|i| i == id) {
            bail!("[layouts.added] names `{id}`, but [layouts.packs] gives no pack that layout");
        }
    }
    for (pack, ids) in &l.packs {
        ensure!(!ids.is_empty(), "[layouts.packs] gives `{pack}` no layouts");
        let mut components = Vec::new();
        let mut licenses = vec![l.license.clone()];
        let mut sources = vec![Source {
            name: l.source.name.clone(),
            url: l.source.url.clone(),
            sha256: None,
        }];
        for id in ids {
            components.push(component(
                root,
                kind::LAYOUT,
                &format!("layouts/{id}.json"),
            )?);
            if let Some(a) = l.added.get(id) {
                for license in &a.licenses {
                    if !licenses.contains(license) {
                        licenses.push(license.clone());
                    }
                }
                sources.extend(a.sources.iter().map(|s| Source {
                    name: s.name.clone(),
                    url: s.url.clone(),
                    sha256: None,
                }));
            }
        }
        if let Some(m) = manifests.iter_mut().find(|m| &m.id == pack) {
            m.components.extend(components);
            for license in licenses {
                if !m.licenses.contains(&license) {
                    m.licenses.push(license);
                }
            }
            m.sources.extend(sources);
        } else {
            let name = l.names.get(pack).with_context(|| {
                format!(
                    "[layouts.packs] gives `{pack}` layouts, but no pack has that id \
                     and [layouts.names] gives it no name"
                )
            })?;
            manifests.push(Manifest {
                schema: SCHEMA,
                id: pack.clone(),
                name: name.clone(),
                version: RELEASE_VERSION.to_string(),
                format: osk_data::codec::VERSION,
                components,
                licenses,
                sources,
            });
        }
    }
    Ok(())
}

/// Each transliteration table joins the pack of its language, which carries
/// the tables' licence and source besides its own, and the fitted tables'
/// dataset's.
fn add_translit(
    root: &Path,
    t: &Translit,
    recorded: &dyn Fn(&str, &str) -> Option<String>,
    manifests: &mut [Manifest],
) -> anyhow::Result<()> {
    let mut tables: Vec<String> = fs::read_dir(root.join("translit"))
        .context("reading translit/")?
        .flatten()
        .filter(|e| e.path().is_file())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    tables.sort();
    for name in tables {
        if let Some(lang) = name.strip_suffix(".english.txt") {
            let english = t.english.as_ref().with_context(|| {
                format!("translit/{name}: packs.toml has no [translit.english] to say whose")
            })?;
            if !root.join(format!("translit/{lang}.txt")).is_file() {
                bail!("translit/{name}: no table translit/{lang}.txt to read it with");
            }
            let m = manifests
                .iter_mut()
                .find(|m| m.id == lang)
                .with_context(|| format!("translit/{name}: no pack `{lang}` to take it"))?;
            m.components
                .push(component(root, kind::ENGLISH, &format!("translit/{name}"))?);
            if !m.licenses.contains(&english.license) {
                m.licenses.push(english.license.clone());
            }
            // The CMUdict they were made from, as `make english-assets`
            // recorded it.
            m.sources.push(Source {
                name: english.source.name.clone(),
                url: english.source.url.clone(),
                sha256: recorded(&format!("english/{lang}"), "cmudict.dict"),
            });
            continue;
        }
        let lang = name
            .strip_suffix(".txt")
            .with_context(|| format!("translit/{name} is no .txt table"))?;
        let m = manifests
            .iter_mut()
            .find(|m| m.id == lang)
            .with_context(|| format!("translit/{name}: no pack `{lang}` to take it"))?;
        m.components.push(component(
            root,
            kind::TRANSLIT,
            &format!("translit/{name}"),
        )?);
        // The table's own licence and source: this repository's, or those
        // of the work it was made from.
        let (licenses, sources): (Vec<&License>, Vec<&SourceDef>) = match t.added.get(lang) {
            Some(a) => (a.licenses.iter().collect(), a.sources.iter().collect()),
            None => (vec![&t.license], vec![&t.source]),
        };
        let fitted = t
            .fitted
            .iter()
            .filter(|f| f.langs.iter().any(|l| l == lang));
        let licenses = licenses
            .into_iter()
            .chain(fitted.clone().filter_map(|f| f.license.as_ref()));
        for license in licenses {
            if !m.licenses.contains(license) {
                m.licenses.push(license.clone());
            }
        }
        for source in sources.into_iter().chain(fitted.map(|f| &f.source)) {
            m.sources.push(Source {
                name: source.name.clone(),
                url: source.url.clone(),
                sha256: None,
            });
        }
    }
    if let Some(lang) = t
        .added
        .keys()
        .find(|l| !root.join(format!("translit/{l}.txt")).is_file())
    {
        bail!("packs.toml: [translit.added] names {lang}, which has no table");
    }
    if let Some(lang) = t
        .fitted
        .iter()
        .flat_map(|f| &f.langs)
        .find(|l| !root.join(format!("translit/{l}.txt")).is_file())
    {
        bail!("packs.toml: [[translit.fitted]] names {lang}, which has no table");
    }
    Ok(())
}

/// Every data file belongs to exactly one pack.
fn check_coverage(root: &Path, manifests: &[Manifest]) -> anyhow::Result<()> {
    let mut claimed: BTreeMap<&str, &str> = BTreeMap::new();
    for m in manifests {
        for c in &m.components {
            if let Some(other) = claimed.insert(&c.path, &m.id) {
                bail!("{} is claimed by both `{other}` and `{}`", c.path, m.id);
            }
        }
    }
    for dir in DATA_DIRS {
        for entry in fs::read_dir(root.join(dir))
            .with_context(|| format!("reading {dir}/"))?
            .flatten()
        {
            if !entry.path().is_file() {
                continue;
            }
            let rel = format!("{dir}/{}", entry.file_name().to_string_lossy());
            if !claimed.contains_key(rel.as_str()) {
                bail!("{rel} is committed but belongs to no pack — add it to packs.toml");
            }
        }
    }
    Ok(())
}
