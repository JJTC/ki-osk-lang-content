//! The ki-osk language-data format.
//!
//! One crate owns the contract between the files this repository builds
//! and the code that reads them:
//!
//! - [`codec`] — the `KIOS` header and postcard encoding every `.bin`
//!   uses.
//! - The serialized types: [`ngram::NgramModel`] (`predict/<lang>.ngram.bin`),
//!   [`cedict::CedictTable`] (`pinyin/`, `zhuyin/`, `jyutping/`),
//!   [`kana::KanjiTable`] and [`kana::ConnectionBlob`] (`kana/`). The
//!   `predict/<lang>.fst` lexicons are plain `fst::Map`s of word → count.
//! - [`pack`] — the manifest describing one language pack, the unit a
//!   device installs, and the index a release offers packs through.
//!   With the `install` feature it also opens downloads and writes them
//!   into an install tree.
//!
//! This repository's builders (`osk-data-tools`) write through these
//! types and ki-osk's `osk-core` reads through them, so an encoder and a
//! decoder that disagree are a compile error rather than a keyboard that
//! quietly falls back to its seed dictionaries.

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

/// The release this crate belongs to: its own version, which is also the
/// version of the data released with it. One number names both, so a
/// consumer that pins a data release also knows which reader matches it.
pub const RELEASE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod cedict;
pub mod codec;
pub mod kana;
pub mod ngram;
pub mod pack;
