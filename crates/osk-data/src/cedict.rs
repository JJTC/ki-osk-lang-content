//! The Chinese input tables: `pinyin/cedict.bin` (Pinyin → Simplified),
//! `zhuyin/cedict.bin` (Bopomofo → Traditional) and `jyutping/cedict.bin`
//! (Jyutping → Traditional).

use std::collections::BTreeMap;

/// Toneless reading → candidate surfaces, most likely first.
///
/// The key is the reading exactly as the keyboard emits it: toneless
/// Pinyin (`nihao`), Bopomofo (`ㄋㄧㄏㄠ`) or Jyutping (`neihou`). The
/// three tables share this one shape, which is why one runtime type
/// reads all of them.
pub type CedictTable = BTreeMap<String, Vec<String>>;
