//! Japanese kana → kanji data: the dictionary (`kana/mozc-kanji.bin`)
//! and the POS-bigram connection matrix (`kana/mozc-connection.bin`).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// One kanji candidate of a reading, carrying the mozc **word cost** and
/// **POS ids** the connection-cost lattice needs. `build-kana` emits these.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanjiEntry {
    /// Surface form (kanji/kana/katakana).
    pub surface: String,
    /// Mozc word cost — lower is more common.
    pub cost: i16,
    /// Left POS id (connects to the previous word's `rid`).
    pub lid: u16,
    /// Right POS id (the next word's `lid` connects to this).
    pub rid: u16,
}

/// The kanji dictionary as bundled: hiragana reading → candidates,
/// cheapest first.
pub type KanjiTable = BTreeMap<String, Vec<KanjiEntry>>;

/// The mozc connection matrix as bundled by `build-connection`: a
/// zlib-deflated flat `i16` array, row-major `cost[rid * n + lid]` (the
/// transition cost from a left word's `rid` to a right word's `lid`).
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionBlob {
    /// Matrix dimension (mozc POS-id count).
    pub n: u16,
    /// zlib(flat little-endian `i16` costs, `n * n` of them).
    pub deflated: Vec<u8>,
}
