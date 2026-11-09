//! `make-packs` — write one release of language packs.
//!
//! Reads `packs.toml` (which files make up each pack, their licences and
//! sources) and `sources.lock` (the sha256 of each pack's upstream
//! inputs), and writes a release directory, every file of it a release
//! asset:
//!
//! - `packs.json` — the [`osk_data::pack::Index`]: every pack's manifest,
//!   and how each file is served;
//! - `<path with / as .>.gz` — each pack file, gzipped;
//! - `NOTICES.md` — the licence notices that go with any pack.
//!
//! The release version is the crates' (`osk_data::RELEASE_VERSION`), so
//! the format crate and the data it was released with share one number.
//!
//! It refuses rather than guesses:
//! - a committed data file that no pack claims, or that two packs claim;
//! - a prediction language with files but no name in `packs.toml`, or a
//!   name with no files;
//! - a component that is missing or whose path climbs out of the tree;
//! - an output directory holding files it did not write.
//!
//! What it wrote is then read back: the index must be sound and every
//! asset must open to exactly the tree file its manifest describes.
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
use std::io::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, bail, ensure};
use clap::Parser;
use osk_data::RELEASE_VERSION;
use osk_data::pack::{
    Component, Download, Index, License, Manifest, NOTICES_PATH, SCHEMA, Source, encoding, kind,
    sha256_hex,
};
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
    #[serde(default)]
    pack: Vec<PackDef>,
}

#[derive(Deserialize)]
struct Prediction {
    license: License,
    source: SourceDef,
    names: BTreeMap<String, String>,
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
const DATA_DIRS: [&str; 6] = ["predict", "pinyin", "zhuyin", "jyutping", "kana", "rules"];

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
    for lang in &langs {
        let p = &defs.prediction;
        manifests.push(Manifest {
            schema: SCHEMA,
            id: lang.clone(),
            name: p.names[lang].clone(),
            version: RELEASE_VERSION.to_string(),
            format: osk_data::codec::VERSION,
            components: vec![
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
            ],
            licenses: vec![p.license.clone()],
            sources: vec![Source {
                name: p.source.name.clone(),
                url: p.source.url.clone(),
                sha256: recorded(&format!("predict/{lang}"), &format!("corpora/{lang}.txt")),
            }],
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

    check_coverage(&args.root, &manifests)?;

    prepare_out(&args.out)?;
    let mut downloads = BTreeMap::new();
    for m in &manifests {
        for c in &m.components {
            let bytes = fs::read(args.root.join(&c.path))?;
            let asset = format!("{}.gz", c.path.replace('/', "."));
            let packed = gzip(&bytes)?;
            fs::write(args.out.join(&asset), &packed)
                .with_context(|| format!("writing {asset}"))?;
            downloads.insert(
                c.path.clone(),
                Download {
                    asset,
                    encoding: encoding::GZIP.to_string(),
                    size: packed.len() as u64,
                    sha256: sha256_hex(&packed),
                },
            );
        }
    }
    let notices = fs::read(args.root.join(NOTICES_PATH)).context("reading NOTICES.md")?;
    fs::write(args.out.join(NOTICES_PATH), &notices)?;
    let index = Index {
        schema: SCHEMA,
        version: RELEASE_VERSION.to_string(),
        packs: manifests,
        downloads,
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
        for c in &m.components {
            let d = &index.downloads[&c.path];
            let file = d.open(&fs::read(out.join(&d.asset))?, c.size)?;
            ensure!(c.matches(&file), "{} does not open to {}", d.asset, c.path);
            add(c.path.clone(), file);
        }
        add(m.install_path(), m.to_json()?.into_bytes());
    }
    let notices = fs::read(out.join(&index.notices.asset))?;
    ensure!(index.notices.matches(&notices), "NOTICES.md changed");
    add(NOTICES_PATH.to_string(), notices);

    let mut tar = Vec::new();
    for (name, bytes) in &entries {
        ustar_entry(&mut tar, name, bytes.as_deref(), mtime)?;
    }
    // The end of an archive: two zero blocks.
    tar.resize(tar.len() + 1024, 0);
    Ok(tar)
}

/// Append one POSIX ustar entry: a file holding `bytes`, or with `None` a
/// directory (named with a trailing `/`). Owner 0, mode 0644 or 0755, time
/// `mtime` — nothing of the machine that built it.
fn ustar_entry(
    tar: &mut Vec<u8>,
    name: &str,
    bytes: Option<&[u8]>,
    mtime: u64,
) -> anyhow::Result<()> {
    ensure!(
        name.len() <= 100,
        "{name}: longer than the 100 bytes a ustar name holds"
    );
    let mut header = [0u8; 512];
    header[..name.len()].copy_from_slice(name.as_bytes());
    // An octal number filling all but the last byte of `field`, which
    // stays NUL.
    let octal = |field: &mut [u8], value: u64| -> anyhow::Result<()> {
        let width = field.len() - 1;
        let digits = format!("{value:0width$o}");
        ensure!(digits.len() == width, "{value} does not fit a ustar field");
        field[..width].copy_from_slice(digits.as_bytes());
        Ok(())
    };
    octal(
        &mut header[100..108],
        if bytes.is_some() { 0o644 } else { 0o755 },
    )?;
    octal(&mut header[108..116], 0)?; // uid
    octal(&mut header[116..124], 0)?; // gid
    octal(&mut header[124..136], bytes.map_or(0, |b| b.len() as u64))?;
    octal(&mut header[136..148], mtime)?;
    header[156] = if bytes.is_some() { b'0' } else { b'5' };
    header[257..263].copy_from_slice(b"ustar\0");
    header[263..265].copy_from_slice(b"00");
    // The checksum is the sum of the header's bytes, counting its own
    // field as eight spaces.
    header[148..156].copy_from_slice(b"        ");
    let sum: u32 = header.iter().map(|&b| u32::from(b)).sum();
    header[148..156].copy_from_slice(format!("{sum:06o}\0 ").as_bytes());
    tar.extend_from_slice(&header);
    if let Some(bytes) = bytes {
        tar.extend_from_slice(bytes);
        tar.resize(tar.len().next_multiple_of(512), 0);
    }
    Ok(())
}

#[allow(clippy::cast_precision_loss)]
fn mib(bytes: u64) -> f64 {
    bytes as f64 / 1_048_576.0
}

/// Gzip with no file name and a zero mtime in the header, so the same
/// file always becomes the same asset and a release rebuilds byte for
/// byte.
fn gzip(bytes: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut e = flate2::GzBuilder::new()
        .mtime(0)
        .write(Vec::new(), flate2::Compression::best());
    e.write_all(bytes)?;
    e.finish()
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
/// tree must match every manifest, and every asset must open to exactly
/// the file its manifest describes. Returns (files, bytes, bytes served).
fn check_release(root: &Path, out: &Path) -> anyhow::Result<(usize, u64, u64)> {
    let index = Index::parse(&fs::read_to_string(out.join(INDEX))?)?;
    ensure!(index.problems().is_empty(), "packs.json reads back unsound");
    let (mut files, mut size, mut packed) = (0, 0, 0);
    for m in &index.packs {
        let problems = m.verify(root);
        ensure!(problems.is_empty(), "pack {}: {problems:?}", m.id);
        for c in &m.components {
            let d = &index.downloads[&c.path];
            let asset = fs::read(out.join(&d.asset))?;
            let file = d.open(&asset, c.size)?;
            ensure!(c.matches(&file), "{} does not open to {}", d.asset, c.path);
            files += 1;
            size += c.size;
            packed += d.size;
        }
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
