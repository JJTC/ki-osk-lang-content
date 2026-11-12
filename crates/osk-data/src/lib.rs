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
//! - [`words`] — how prediction splits text into words: the one rule
//!   the trainer, the keyboard and the measure share.
//! - [`wordbreak`] — the word lists (`wordbreak/<lang>.txt`) that cut the
//!   text of a language written without spaces between words (Thai) into
//!   its words.
//! - [`dialects`] — dialect flags (`predict/<lang>.dialects.bin`): which
//!   varieties of a language use a word, and its spellings across them.
//! - [`rules`] — the text rules (`rules/*.weir`): what typing a key does
//!   to the text before the caret beyond typing it, and where autocorrect
//!   acts besides space; the keyboard and the measure read the same.
//! - [`layout`] — keyboard layouts (`layouts/<id>.json`): their keys, what
//!   each types, and the views they page through.
//! - [`emoji`] — the emoji search catalogue (`emoji/catalogue.json`).
//! - [`offensive`] — the offensive-word lists a keyboard can keep off its
//!   suggestion strip (`offensive/<lang>.txt`).
//! - [`pinyin`] — Pinyin's initials and finals, the syllable inventory
//!   (`pinyin/syllables.txt`) and the double-Pinyin schemes
//!   (`pinyin/double-pinyin.json`) a Pinyin composer reads beside its
//!   dictionary.
//! - [`pack`] — the manifest describing one language pack, the unit a
//!   device installs, and the index a release offers packs through.
//!   With the `install` feature it also opens downloads and writes them
//!   into an install tree.
//!
//! This repository's builders (`osk-data-tools`) write through these
//! types and ki-osk's `osk-core` reads through them, so an encoder and a
//! decoder that disagree are a compile error rather than a keyboard that
//! quietly offers nothing.

#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

/// The release this crate belongs to: its own version, which is also the
/// version of the data released with it. One number names both, so a
/// consumer that pins a data release also knows which reader matches it.
pub const RELEASE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod cedict;
pub mod codec;
pub mod dialects;
pub mod emoji;
pub mod kana;
pub mod layout;
pub mod ngram;
pub mod offensive;
pub mod pack;
pub mod pinyin;
pub mod rules;
pub mod wordbreak;
pub mod words;
