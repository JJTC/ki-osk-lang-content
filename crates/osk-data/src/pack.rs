//! Language packs: the unit a device installs, and the index a release
//! offers them through.
//!
//! A pack is a set of files from the language-data tree plus a
//! [`Manifest`] that describes them. Component paths are tree paths
//! (`predict/en.fst`, `kana/mozc-kanji.bin`), so installing any number of
//! packs into one directory yields the layout ki-osk's runtime already
//! searches (`/usr/share/ki-osk`): a kiosk that installs every pack and a
//! phone that installs two read their data the same way.
//!
//! Every pack names its licences. Several are share-alike (CC BY-SA), and
//! that obligation travels with the files: whoever ships a pack ships its
//! licence notice.
//!
//! # Releases
//!
//! A release offers its packs through an [`Index`] (`packs.json`): every
//! pack's manifest and, for each file, a [`Download`] — the release asset
//! that carries it, gzip-compressed, with the asset's own size and
//! checksum. A client reads the index, picks packs and fetches only their
//! assets. Each file is checked twice: the asset against the index before
//! it is decompressed, and the result against the manifest before it is
//! written. Keeping a release's index pins that release byte for byte,
//! since every checksum is in it.
//!
//! An install tree holds each file at its path, each installed pack's
//! manifest at [`Manifest::install_path`] (`packs/<id>.json`), and the
//! release's licence notices at [`NOTICES_PATH`]. With the `install`
//! feature this module also opens downloads ([`Download::open`]) and
//! writes files into a tree ([`install_file`]).
//!
//! # Compatibility
//!
//! [`Manifest::schema`] and [`Index::schema`] version these documents;
//! [`Manifest::format`] is the [`crate::codec::VERSION`] the `.bin`
//! components were written with. A component's [`Component::kind`] is a
//! plain string, and readers must skip kinds they do not know — that is
//! how a pack gains a new kind of data (a dialect table, text rules)
//! without breaking older readers. For the same reason, unknown fields
//! are ignored, not rejected.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

/// The schema of the manifests and indexes this crate reads and writes.
pub const SCHEMA: u32 = 1;

/// Where an install tree keeps the release's licence notices.
pub const NOTICES_PATH: &str = "NOTICES.md";

/// The directory of an install tree that holds the installed packs'
/// manifests. No component may live under it.
pub const MANIFEST_DIR: &str = "packs";

/// The component kinds this crate knows. Readers must tolerate others.
pub mod kind {
    /// Word → corpus count, an `fst::Map` (`predict/<lang>.fst`).
    pub const LEXICON: &str = "lexicon";
    /// A [`crate::ngram::NgramModel`] (`predict/<lang>.ngram.bin`).
    pub const NGRAM: &str = "ngram";
    /// Each word whose usual form is not lower case, an `fst::Map` from
    /// its identity to its capitals ([`crate::words::Splitter::case_mask`];
    /// `predict/<lang>.forms.fst`).
    pub const FORMS: &str = "forms";
    /// A [`crate::cedict::CedictTable`] keyed by toneless Pinyin.
    pub const PINYIN: &str = "pinyin";
    /// A [`crate::cedict::CedictTable`] keyed by toneless Bopomofo.
    pub const ZHUYIN: &str = "zhuyin";
    /// A [`crate::cedict::CedictTable`] keyed by toneless Jyutping.
    pub const JYUTPING: &str = "jyutping";
    /// A [`crate::kana::KanjiTable`].
    pub const KANJI: &str = "kanji";
    /// A [`crate::kana::ConnectionBlob`].
    pub const CONNECTION: &str = "connection";
}

/// How a release asset carries its file.
pub mod encoding {
    /// The file itself.
    pub const IDENTITY: &str = "identity";
    /// The file gzip-compressed: one RFC 1952 member.
    pub const GZIP: &str = "gzip";

    /// The encodings this crate can open.
    pub const KNOWN: [&str; 2] = [IDENTITY, GZIP];
}

/// Everything a device needs to know about one pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    /// Manifest schema; [`SCHEMA`] for every manifest this crate writes.
    pub schema: u32,
    /// BCP 47 tag, unique within a release: `en`, `zh-Hans`, `yue`.
    pub id: String,
    /// English display name.
    pub name: String,
    /// The release this pack belongs to (semver).
    pub version: String,
    /// [`crate::codec::VERSION`] of the `.bin` components.
    pub format: u16,
    /// The files, by tree path.
    pub components: Vec<Component>,
    /// Every licence the files carry. Ship their notices with them.
    pub licenses: Vec<License>,
    /// What the files were built from.
    pub sources: Vec<Source>,
}

/// One file of a pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Component {
    /// What the file is — one of [`kind`], or a kind a newer release
    /// added, which a reader must skip.
    pub kind: String,
    /// Path relative to the data root, `/`-separated.
    pub path: String,
    /// Size in bytes.
    pub size: u64,
    /// Lower-case hex SHA-256 of the file.
    pub sha256: String,
}

/// A licence the pack's files carry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct License {
    /// As commonly written: `CC BY-SA 4.0`, `BSD-3-Clause`.
    pub name: String,
    /// Where its text lives.
    pub url: String,
    /// Whether redistribution must keep the same licence.
    pub share_alike: bool,
}

/// An upstream dataset the pack was built from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    /// The dataset's name: `Tatoeba`, `CC-CEDICT`.
    pub name: String,
    /// Where it comes from.
    pub url: String,
    /// SHA-256 of the exact input file, when recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
}

/// What a release offers: every pack, and how each of its files is
/// served.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Index {
    /// Index schema; [`SCHEMA`] for every index this crate writes.
    pub schema: u32,
    /// The release (semver). Every manifest carries the same.
    pub version: String,
    /// Every pack in the release.
    pub packs: Vec<Manifest>,
    /// How each pack file is served, by its tree path.
    pub downloads: BTreeMap<String, Download>,
    /// The release's licence notices, served as they are. Whoever
    /// installs a pack installs these too, at [`NOTICES_PATH`]:
    /// share-alike terms travel with the files.
    pub notices: Download,
}

/// One release asset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Download {
    /// The asset's file name: a plain name, no directories.
    pub asset: String,
    /// How the asset carries its file — one of [`encoding`].
    pub encoding: String,
    /// The asset's size in bytes, as downloaded.
    pub size: u64,
    /// Lower-case hex SHA-256 of the asset as downloaded.
    pub sha256: String,
}

/// Why a manifest or an index could not be read.
#[derive(Debug)]
pub enum Error {
    /// Not valid JSON for the document.
    Json(serde_json::Error),
    /// A schema this crate does not read.
    Schema {
        /// The schema the document declares.
        found: u32,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(e) => write!(f, "invalid pack JSON: {e}"),
            Self::Schema { found } => {
                write!(f, "pack schema {found}; this reader understands {SCHEMA}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(e) => Some(e),
            Self::Schema { .. } => None,
        }
    }
}

/// Pretty-printed JSON with a trailing newline.
fn pretty<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let mut s = serde_json::to_string_pretty(value)?;
    s.push('\n');
    Ok(s)
}

impl Manifest {
    /// Parse a manifest.
    ///
    /// # Errors
    /// [`Error::Json`] for malformed JSON, [`Error::Schema`] for a schema
    /// other than [`SCHEMA`].
    pub fn parse(json: &str) -> Result<Self, Error> {
        let m: Self = serde_json::from_str(json).map_err(Error::Json)?;
        if m.schema != SCHEMA {
            return Err(Error::Schema { found: m.schema });
        }
        Ok(m)
    }

    /// The manifest as pretty-printed JSON with a trailing newline.
    ///
    /// # Errors
    /// Propagates [`serde_json::Error`], which these plain types do not
    /// produce in practice.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        pretty(self)
    }

    /// Whether any of the pack's licences is share-alike.
    #[must_use]
    pub fn is_share_alike(&self) -> bool {
        self.licenses.iter().any(|l| l.share_alike)
    }

    /// The components of one kind.
    pub fn components_of<'a>(&'a self, kind: &'a str) -> impl Iterator<Item = &'a Component> {
        self.components.iter().filter(move |c| c.kind == kind)
    }

    /// Where an install tree keeps this manifest: `packs/<id>.json`. Only
    /// meaningful for a plain id, which [`Index::problems`] checks.
    #[must_use]
    pub fn install_path(&self) -> String {
        format!("{MANIFEST_DIR}/{}.json", self.id)
    }

    /// The installed size of the pack: its components' sizes summed.
    #[must_use]
    pub fn size(&self) -> u64 {
        self.components.iter().map(|c| c.size).sum()
    }
}

/// A relative, `/`-separated path with no `..`, `.` or empty segments.
fn is_safe_tree_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains('\\')
        && path
            .split('/')
            .all(|seg| !seg.is_empty() && seg != "." && seg != "..")
}

impl Component {
    /// Whether [`Component::path`] stays inside the data root: relative,
    /// `/`-separated, with no `..`, `.` or empty segments. A manifest may
    /// arrive from a download, so a reader must check this before joining
    /// the path onto anything.
    #[must_use]
    pub fn has_safe_path(&self) -> bool {
        is_safe_tree_path(&self.path)
    }
}

/// A pack id that is a plain tag: ASCII letters, digits and `-`, starting
/// with a letter or digit. It names a file (`packs/<id>.json`), so it
/// must not carry a path.
fn is_plain_id(id: &str) -> bool {
    id.len() <= 64
        && id.starts_with(|c: char| c.is_ascii_alphanumeric())
        && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// An asset name that is a plain file name: ASCII letters, digits, `.`,
/// `-` and `_`, starting with a letter or digit.
fn is_plain_asset(name: &str) -> bool {
    name.len() <= 255
        && name.starts_with(|c: char| c.is_ascii_alphanumeric())
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_'))
}

/// A path an install tree keeps for itself: the notices, or anything in
/// the manifest directory.
fn is_reserved(path: &str) -> bool {
    path == NOTICES_PATH || path.split('/').next() == Some(MANIFEST_DIR)
}

/// What [`Index::problems`] found wrong with an index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexProblem {
    /// Two packs share an id.
    DuplicatePack(String),
    /// A pack id that is not a plain tag.
    UnsafeId(String),
    /// A pack whose manifest has another schema or release than the
    /// index.
    ForeignManifest(String),
    /// A component path that is not a safe tree path, or that an install
    /// tree reserves.
    UnsafePath(String),
    /// Two components share a path.
    DuplicatePath(String),
    /// A component with no download.
    NoDownload(String),
    /// A download for a path no pack has.
    UnusedDownload(String),
    /// An asset name that is not a plain file name.
    UnsafeAsset(String),
    /// Two downloads share an asset.
    DuplicateAsset(String),
    /// An encoding this crate cannot open.
    UnknownEncoding {
        /// The asset.
        asset: String,
        /// Its encoding.
        encoding: String,
    },
    /// The notices are not served as they are.
    EncodedNotices,
}

impl fmt::Display for IndexProblem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicatePack(id) => write!(f, "two packs are called `{id}`"),
            Self::UnsafeId(id) => {
                write!(
                    f,
                    "pack id {id:?} is not a plain tag (letters, digits, `-`)"
                )
            }
            Self::ForeignManifest(id) => {
                write!(
                    f,
                    "pack `{id}` has a manifest from another release or schema"
                )
            }
            Self::UnsafePath(p) => write!(f, "{p:?} is not a path a pack may install"),
            Self::DuplicatePath(p) => write!(f, "{p} belongs to two packs"),
            Self::NoDownload(p) => write!(f, "{p} has no download"),
            Self::UnusedDownload(p) => write!(f, "a download names {p}, which no pack has"),
            Self::UnsafeAsset(a) => write!(f, "asset name {a:?} is not a plain file name"),
            Self::DuplicateAsset(a) => write!(f, "two downloads share the asset {a}"),
            Self::UnknownEncoding { asset, encoding } => {
                write!(f, "{asset} is {encoding:?}, which this reader cannot open")
            }
            Self::EncodedNotices => write!(f, "the notices must be served as they are"),
        }
    }
}

impl Index {
    /// Parse an index.
    ///
    /// # Errors
    /// [`Error::Json`] for malformed JSON, [`Error::Schema`] for a schema
    /// other than [`SCHEMA`].
    pub fn parse(json: &str) -> Result<Self, Error> {
        let index: Self = serde_json::from_str(json).map_err(Error::Json)?;
        if index.schema != SCHEMA {
            return Err(Error::Schema {
                found: index.schema,
            });
        }
        Ok(index)
    }

    /// The index as pretty-printed JSON with a trailing newline.
    ///
    /// # Errors
    /// Propagates [`serde_json::Error`], which these plain types do not
    /// produce in practice.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        pretty(self)
    }

    /// The pack called `id`.
    #[must_use]
    pub fn pack(&self, id: &str) -> Option<&Manifest> {
        self.packs.iter().find(|m| m.id == id)
    }

    /// Everything structurally wrong with the index; empty when it is
    /// sound. An index may arrive from a download, so a client checks
    /// this before acting on any of it: every id names a file, every
    /// path is safe and has exactly one owner and one download, and every
    /// asset is a plain name this crate can open.
    #[must_use]
    pub fn problems(&self) -> Vec<IndexProblem> {
        let mut out = Vec::new();
        let mut ids = BTreeSet::new();
        let mut paths = BTreeSet::new();
        for m in &self.packs {
            if !is_plain_id(&m.id) {
                out.push(IndexProblem::UnsafeId(m.id.clone()));
            }
            if !ids.insert(m.id.as_str()) {
                out.push(IndexProblem::DuplicatePack(m.id.clone()));
            }
            if m.schema != self.schema || m.version != self.version {
                out.push(IndexProblem::ForeignManifest(m.id.clone()));
            }
            for c in &m.components {
                if !c.has_safe_path() || is_reserved(&c.path) {
                    out.push(IndexProblem::UnsafePath(c.path.clone()));
                } else if !paths.insert(c.path.as_str()) {
                    out.push(IndexProblem::DuplicatePath(c.path.clone()));
                } else if !self.downloads.contains_key(&c.path) {
                    out.push(IndexProblem::NoDownload(c.path.clone()));
                }
            }
        }
        for path in self.downloads.keys() {
            if !paths.contains(path.as_str()) {
                out.push(IndexProblem::UnusedDownload(path.clone()));
            }
        }
        let mut assets = BTreeSet::new();
        for d in self.downloads.values().chain([&self.notices]) {
            if !is_plain_asset(&d.asset) {
                out.push(IndexProblem::UnsafeAsset(d.asset.clone()));
            }
            if !assets.insert(d.asset.as_str()) {
                out.push(IndexProblem::DuplicateAsset(d.asset.clone()));
            }
            if !encoding::KNOWN.contains(&d.encoding.as_str()) {
                out.push(IndexProblem::UnknownEncoding {
                    asset: d.asset.clone(),
                    encoding: d.encoding.clone(),
                });
            }
        }
        if self.notices.encoding != encoding::IDENTITY {
            out.push(IndexProblem::EncodedNotices);
        }
        out
    }

    /// The packs whose data differs from `older`'s: added, removed, or
    /// with other files (paths or checksums) — what a client would have
    /// to download again. Packs with the same files are left out, even if
    /// their metadata moved. By id.
    #[must_use]
    pub fn changes_since(&self, older: &Index) -> Vec<PackChange> {
        let ids: BTreeSet<&str> = self
            .packs
            .iter()
            .chain(&older.packs)
            .map(|m| m.id.as_str())
            .collect();
        ids.into_iter()
            .filter_map(|id| match (older.pack(id), self.pack(id)) {
                (Some(before), Some(after)) if before.components == after.components => None,
                (before, after) => Some(PackChange {
                    id: id.to_string(),
                    before: before.map(Manifest::size),
                    after: after.map(Manifest::size),
                }),
            })
            .collect()
    }
}

/// How one pack differs between two releases ([`Index::changes_since`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackChange {
    /// The pack.
    pub id: String,
    /// Its installed size in the older release; `None` when it is new.
    pub before: Option<u64>,
    /// Its installed size in this release; `None` when it is gone.
    pub after: Option<u64>,
}

/// What [`Manifest::verify`] found wrong with one component.
#[cfg(feature = "verify")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Problem {
    /// The path is absolute or climbs out of the data root.
    UnsafePath(String),
    /// The file is not there.
    Missing(String),
    /// The file has the wrong size.
    Size {
        /// The component's path.
        path: String,
        /// What the manifest says.
        expected: u64,
        /// What is on disk.
        found: u64,
    },
    /// The file's contents differ from what the manifest recorded.
    Checksum(String),
}

#[cfg(feature = "verify")]
impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsafePath(p) => write!(f, "{p:?} is not a safe tree path"),
            Self::Missing(p) => write!(f, "{p} is missing"),
            Self::Size {
                path,
                expected,
                found,
            } => write!(f, "{path} is {found} bytes, not {expected}"),
            Self::Checksum(p) => write!(f, "{p} has another checksum"),
        }
    }
}

#[cfg(feature = "verify")]
impl Manifest {
    /// Check every component against the files under `root`. An empty
    /// result means the pack is intact.
    #[must_use]
    pub fn verify(&self, root: &std::path::Path) -> Vec<Problem> {
        let mut problems = Vec::new();
        for c in &self.components {
            if !c.has_safe_path() {
                problems.push(Problem::UnsafePath(c.path.clone()));
                continue;
            }
            let Ok(bytes) = std::fs::read(root.join(&c.path)) else {
                problems.push(Problem::Missing(c.path.clone()));
                continue;
            };
            let found = bytes.len() as u64;
            if found != c.size {
                problems.push(Problem::Size {
                    path: c.path.clone(),
                    expected: c.size,
                    found,
                });
            } else if sha256_hex(&bytes) != c.sha256 {
                problems.push(Problem::Checksum(c.path.clone()));
            }
        }
        problems
    }
}

#[cfg(feature = "verify")]
impl Component {
    /// Whether `bytes` are this component's file, by size and checksum.
    #[must_use]
    pub fn matches(&self, bytes: &[u8]) -> bool {
        bytes.len() as u64 == self.size && sha256_hex(bytes) == self.sha256
    }
}

#[cfg(feature = "verify")]
impl Download {
    /// Whether `bytes` are this asset, by size and checksum.
    #[must_use]
    pub fn matches(&self, bytes: &[u8]) -> bool {
        bytes.len() as u64 == self.size && sha256_hex(bytes) == self.sha256
    }
}

/// Lower-case hex SHA-256, the form every checksum here uses.
#[cfg(feature = "verify")]
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    use std::fmt::Write as _;
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(64);
    for b in digest {
        let _ = write!(out, "{b:02x}");
    }
    out
}

/// Why [`Download::open`] refused an asset.
#[cfg(feature = "install")]
#[derive(Debug)]
pub enum OpenError {
    /// The bytes are not the asset the index describes: wrong size or
    /// checksum. Nothing was decompressed.
    NotTheAsset(String),
    /// An encoding this crate cannot open.
    Encoding {
        /// The asset.
        asset: String,
        /// Its encoding.
        encoding: String,
    },
    /// The asset did not decompress, or decompressed past its limit.
    Decode {
        /// The asset.
        asset: String,
        /// What went wrong.
        error: std::io::Error,
    },
}

#[cfg(feature = "install")]
impl fmt::Display for OpenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotTheAsset(a) => {
                write!(
                    f,
                    "{a}: not the asset the index describes (size or checksum)"
                )
            }
            Self::Encoding { asset, encoding } => {
                write!(f, "{asset}: cannot open the {encoding:?} encoding")
            }
            Self::Decode { asset, error } => write!(f, "{asset}: {error}"),
        }
    }
}

#[cfg(feature = "install")]
impl std::error::Error for OpenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Decode { error, .. } => Some(error),
            _ => None,
        }
    }
}

#[cfg(feature = "install")]
impl Download {
    /// Check that `bytes` are this asset, then decode them into the file
    /// it carries, decompressing no more than `limit` bytes — the size
    /// the manifest gives that file. The checksum comes first, so nothing
    /// unexpected is ever decompressed. Check the result against its
    /// [`Component`] before writing it anywhere.
    ///
    /// # Errors
    /// [`OpenError`] when the bytes are not this asset, its encoding is
    /// unknown, or it does not decompress within `limit`.
    pub fn open(&self, bytes: &[u8], limit: u64) -> Result<Vec<u8>, OpenError> {
        use std::io::Read as _;
        if !self.matches(bytes) {
            return Err(OpenError::NotTheAsset(self.asset.clone()));
        }
        match self.encoding.as_str() {
            encoding::IDENTITY => Ok(bytes.to_vec()),
            encoding::GZIP => {
                let decode = |error| OpenError::Decode {
                    asset: self.asset.clone(),
                    error,
                };
                let mut out = Vec::new();
                flate2::read::GzDecoder::new(bytes)
                    .take(limit.saturating_add(1))
                    .read_to_end(&mut out)
                    .map_err(decode)?;
                if out.len() as u64 > limit {
                    return Err(decode(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("decompresses past the {limit} bytes it should hold"),
                    )));
                }
                Ok(out)
            }
            other => Err(OpenError::Encoding {
                asset: self.asset.clone(),
                encoding: other.to_string(),
            }),
        }
    }
}

/// Write `bytes` to `path`, a tree path, under `root`, replacing any file
/// already there atomically: the bytes go to a temporary file beside it,
/// synced, then renamed into place, so a reader sees the old file or the
/// new one and never part of either.
///
/// # Errors
/// [`std::io::ErrorKind::InvalidInput`] for a path that is not a safe tree
/// path; otherwise the error from creating directories or writing.
#[cfg(feature = "install")]
pub fn install_file(root: &std::path::Path, path: &str, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write as _;
    if !is_safe_tree_path(path) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("{path:?} is not a safe tree path"),
        ));
    }
    let dest = root.join(path);
    let (Some(dir), Some(name)) = (dest.parent(), dest.file_name()) else {
        unreachable!("a safe tree path has a parent and a file name");
    };
    std::fs::create_dir_all(dir)?;
    let tmp = dir.join(format!(
        ".{}.{}.tmp",
        name.to_string_lossy(),
        std::process::id()
    ));
    let written = std::fs::File::create(&tmp)
        .and_then(|mut f| f.write_all(bytes).and_then(|()| f.sync_all()))
        .and_then(|()| std::fs::rename(&tmp, &dest));
    if written.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    written
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Manifest {
        Manifest {
            schema: SCHEMA,
            id: "en".into(),
            name: "English".into(),
            version: "0.1.0".into(),
            format: crate::codec::VERSION,
            components: vec![Component {
                kind: kind::LEXICON.into(),
                path: "predict/en.fst".into(),
                size: 3,
                sha256: "ab".into(),
            }],
            licenses: vec![License {
                name: "CC BY 2.0 FR".into(),
                url: "https://creativecommons.org/licenses/by/2.0/fr/".into(),
                share_alike: false,
            }],
            sources: vec![Source {
                name: "Tatoeba".into(),
                url: "https://tatoeba.org/".into(),
                sha256: None,
            }],
        }
    }

    fn download(asset: &str, encoding: &str) -> Download {
        Download {
            asset: asset.into(),
            encoding: encoding.into(),
            size: 3,
            sha256: "cd".into(),
        }
    }

    fn sample_index() -> Index {
        Index {
            schema: SCHEMA,
            version: "0.1.0".into(),
            packs: vec![sample()],
            downloads: BTreeMap::from([(
                "predict/en.fst".to_string(),
                download("predict.en.fst.gz", encoding::GZIP),
            )]),
            notices: download("NOTICES.md", encoding::IDENTITY),
        }
    }

    #[test]
    fn round_trips_through_json() {
        let m = sample();
        assert_eq!(Manifest::parse(&m.to_json().unwrap()).unwrap(), m);
        let index = sample_index();
        assert_eq!(Index::parse(&index.to_json().unwrap()).unwrap(), index);
    }

    #[test]
    fn another_schema_is_refused() {
        let mut m = sample();
        m.schema = SCHEMA + 1;
        let err = Manifest::parse(&m.to_json().unwrap()).unwrap_err();
        assert!(
            matches!(err, Error::Schema { found } if found == SCHEMA + 1),
            "{err}"
        );
        let mut index = sample_index();
        index.schema = SCHEMA + 1;
        let err = Index::parse(&index.to_json().unwrap()).unwrap_err();
        assert!(matches!(err, Error::Schema { .. }), "{err}");
    }

    /// A newer release may add fields and component kinds; an older
    /// reader must still read what it understands.
    #[test]
    fn unknown_fields_and_kinds_are_tolerated() {
        let json = sample()
            .to_json()
            .unwrap()
            .replacen(
                "\"schema\": 1,",
                "\"schema\": 1, \"variants\": [\"en-GB\"],",
                1,
            )
            .replacen("\"lexicon\"", "\"dialect-table\"", 1);
        let m = Manifest::parse(&json).unwrap();
        assert_eq!(m.components_of(kind::LEXICON).count(), 0);
        assert_eq!(m.components_of("dialect-table").count(), 1);

        let json = sample_index().to_json().unwrap().replacen(
            "\"schema\": 1,",
            "\"schema\": 1, \"channel\": \"stable\",",
            1,
        );
        assert!(Index::parse(&json).unwrap().problems().is_empty());
    }

    #[test]
    fn paths_must_stay_inside_the_data_root() {
        let with = |p: &str| Component {
            path: p.into(),
            ..sample().components[0].clone()
        };
        assert!(with("predict/en.fst").has_safe_path());
        for bad in [
            "",
            "/etc/passwd",
            "../x",
            "predict/../../x",
            "predict//en.fst",
            "./x",
            "a\\b",
        ] {
            assert!(!with(bad).has_safe_path(), "{bad:?} must be refused");
        }
    }

    #[test]
    fn share_alike_is_reported() {
        let mut m = sample();
        assert!(!m.is_share_alike());
        m.licenses[0].share_alike = true;
        assert!(m.is_share_alike());
    }

    #[test]
    fn a_sound_index_has_no_problems() {
        let index = sample_index();
        assert_eq!(index.problems(), vec![]);
        assert_eq!(index.pack("en").unwrap().install_path(), "packs/en.json");
        assert!(index.pack("de").is_none());
    }

    #[test]
    fn changes_since_names_the_packs_whose_data_moved() {
        let older = sample_index();
        let mut newer = older.clone();
        assert_eq!(newer.changes_since(&older), vec![]);

        // New metadata alone is not a change: nothing to download again.
        newer.packs[0].name = "English (all)".into();
        assert_eq!(newer.changes_since(&older), vec![]);

        newer.packs[0].components[0].sha256 = "ef".into();
        newer.packs[0].components[0].size = 5;
        newer.packs.push(Manifest {
            id: "de".into(),
            ..sample()
        });
        let mut gone = sample();
        gone.id = "fr".into();
        let mut older_with_fr = older.clone();
        older_with_fr.packs.push(gone);
        let change = |id: &str, before, after| PackChange {
            id: id.into(),
            before,
            after,
        };
        assert_eq!(
            newer.changes_since(&older_with_fr),
            vec![
                change("de", None, Some(3)),
                change("en", Some(3), Some(5)),
                change("fr", Some(3), None),
            ]
        );
    }

    /// Each kind of problem, one at a time, on an otherwise sound index.
    #[test]
    fn index_problems_are_each_found() {
        let found = |edit: &dyn Fn(&mut Index)| {
            let mut index = sample_index();
            edit(&mut index);
            index.problems()
        };
        assert_eq!(
            found(&|i| i.packs.push(Manifest {
                components: vec![],
                ..sample()
            })),
            vec![IndexProblem::DuplicatePack("en".into())]
        );
        assert_eq!(
            found(&|i| i.packs[0].id = "../en".into()),
            vec![IndexProblem::UnsafeId("../en".into())]
        );
        assert_eq!(
            found(&|i| i.packs[0].version = "0.0.9".into()),
            vec![IndexProblem::ForeignManifest("en".into())]
        );
        for reserved in ["NOTICES.md", "packs/en.json", "../x"] {
            let problems = found(&|i| i.packs[0].components[0].path = reserved.into());
            assert_eq!(
                problems[0],
                IndexProblem::UnsafePath(reserved.into()),
                "{reserved}"
            );
        }
        assert_eq!(
            found(&|i| {
                let c = i.packs[0].components[0].clone();
                i.packs[0].components.push(c);
            }),
            vec![IndexProblem::DuplicatePath("predict/en.fst".into())]
        );
        assert_eq!(
            found(&|i| {
                let d = i.downloads.remove("predict/en.fst").unwrap();
                i.downloads.insert("predict/de.fst".into(), d);
            }),
            vec![
                IndexProblem::NoDownload("predict/en.fst".into()),
                IndexProblem::UnusedDownload("predict/de.fst".into()),
            ]
        );
        assert_eq!(
            found(&|i| i.notices.asset = "../NOTICES.md".into()),
            vec![IndexProblem::UnsafeAsset("../NOTICES.md".into())]
        );
        assert_eq!(
            found(&|i| i.notices.asset = "predict.en.fst.gz".into()),
            vec![IndexProblem::DuplicateAsset("predict.en.fst.gz".into())]
        );
        assert_eq!(
            found(&|i| {
                i.downloads.get_mut("predict/en.fst").unwrap().encoding = "zstd".into();
            }),
            vec![IndexProblem::UnknownEncoding {
                asset: "predict.en.fst.gz".into(),
                encoding: "zstd".into()
            }]
        );
        assert_eq!(
            found(&|i| i.notices.encoding = encoding::GZIP.into()),
            vec![IndexProblem::EncodedNotices]
        );
    }

    #[cfg(feature = "verify")]
    #[test]
    fn verify_reports_each_kind_of_problem() {
        let dir = std::env::temp_dir().join(format!("osk-data-verify-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("predict")).unwrap();
        std::fs::write(dir.join("predict/good.bin"), b"abc").unwrap();
        std::fs::write(dir.join("predict/size.bin"), b"abcd").unwrap();
        std::fs::write(dir.join("predict/sum.bin"), b"xyz").unwrap();
        let c = |path: &str, size, sha: &str| Component {
            kind: kind::NGRAM.into(),
            path: path.into(),
            size,
            sha256: sha.into(),
        };
        let abc = sha256_hex(b"abc");
        let mut m = sample();
        m.components = vec![
            c("predict/good.bin", 3, &abc),
            c("predict/size.bin", 3, &abc),
            c("predict/sum.bin", 3, &abc),
            c("predict/gone.bin", 3, &abc),
            c("../escape.bin", 3, &abc),
        ];
        let problems = m.verify(&dir);
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(
            problems,
            vec![
                Problem::Size {
                    path: "predict/size.bin".into(),
                    expected: 3,
                    found: 4
                },
                Problem::Checksum("predict/sum.bin".into()),
                Problem::Missing("predict/gone.bin".into()),
                Problem::UnsafePath("../escape.bin".into()),
            ]
        );
        assert!(m.components[0].matches(b"abc"));
        assert!(!m.components[0].matches(b"abd"));
    }

    #[cfg(feature = "install")]
    fn gzip(bytes: &[u8]) -> Vec<u8> {
        use std::io::Write as _;
        let mut e = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
        e.write_all(bytes).unwrap();
        e.finish().unwrap()
    }

    #[cfg(feature = "install")]
    fn download_of(asset: &[u8], encoding: &str) -> Download {
        Download {
            asset: "a.gz".into(),
            encoding: encoding.into(),
            size: asset.len() as u64,
            sha256: sha256_hex(asset),
        }
    }

    #[cfg(feature = "install")]
    #[test]
    fn open_decodes_exactly_the_promised_asset() {
        let file = b"the file the asset carries".repeat(20);
        let asset = gzip(&file);
        let d = download_of(&asset, encoding::GZIP);
        assert_eq!(d.open(&asset, file.len() as u64).unwrap(), file);
        let plain = download_of(&file, encoding::IDENTITY);
        assert_eq!(plain.open(&file, file.len() as u64).unwrap(), file);

        // Wrong bytes are refused before anything is decompressed.
        let mut tampered = asset.clone();
        *tampered.last_mut().unwrap() ^= 1;
        assert!(matches!(
            d.open(&tampered, file.len() as u64),
            Err(OpenError::NotTheAsset(_))
        ));
        // A file larger than its manifest says is cut off, not inflated.
        assert!(matches!(
            d.open(&asset, file.len() as u64 - 1),
            Err(OpenError::Decode { .. })
        ));
        // A checksum-correct asset that is not gzip does not decode.
        let not_gzip = download_of(&file, encoding::GZIP);
        assert!(matches!(
            not_gzip.open(&file, 10_000),
            Err(OpenError::Decode { .. })
        ));
        let unknown = download_of(&asset, "zstd");
        assert!(matches!(
            unknown.open(&asset, 10_000),
            Err(OpenError::Encoding { .. })
        ));
    }

    #[cfg(feature = "install")]
    #[test]
    fn install_file_replaces_atomically_and_refuses_escapes() {
        let root = std::env::temp_dir().join(format!("osk-data-install-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        install_file(&root, "predict/en.fst", b"old").unwrap();
        install_file(&root, "predict/en.fst", b"new").unwrap();
        assert_eq!(std::fs::read(root.join("predict/en.fst")).unwrap(), b"new");
        let left: Vec<_> = std::fs::read_dir(root.join("predict"))
            .unwrap()
            .flatten()
            .map(|e| e.file_name())
            .collect();
        assert_eq!(left.len(), 1, "no temporary file stays behind: {left:?}");
        for bad in ["../escape", "/abs", "a//b", ""] {
            let err = install_file(&root, bad, b"x").unwrap_err();
            assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput, "{bad:?}");
        }
        let _ = std::fs::remove_dir_all(&root);
    }
}
