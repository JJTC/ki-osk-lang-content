//! The binary encoding of every `.bin` in the language data.
//!
//! The n-gram tables, the Pinyin, Zhuyin and Jyutping tables, the kanji
//! dictionary and its connection matrix are all written and read through
//! [`encode`] and [`decode`]: this repository's builders write them,
//! ki-osk's `osk-core` reads them, and this module is all that stands
//! between. A decoder that disagreed with its encoder would not crash —
//! the loader would return `None` and the keyboard would quietly fall back
//! to its seed dictionaries — so the encoding is decided here, once.
//!
//! The body is [postcard](https://docs.rs/postcard): plain `serde`, varint
//! framing, `MIT OR Apache-2.0`.
//!
//! # The header
//!
//! Every asset starts with [`MAGIC`] and a little-endian `u16`
//! [`VERSION`], and [`decode`] refuses anything whose header does not
//! match. Without it, a file in another format could decode to plausible
//! garbage or an empty table as easily as it failed.

/// The error every decode and encode returns.
pub use postcard::Error;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Leading bytes of every asset. `KIOS` and a version, so a file that is
/// not one of ours, or is in another version of the format, is refused on
/// sight rather than by a decode that happens to fail.
pub const MAGIC: [u8; 4] = *b"KIOS";

/// Asset format version. **Bump it whenever the encoding or any asset
/// type's layout changes**, so a stale file fails loudly instead of
/// decoding to something plausible.
pub const VERSION: u16 = 1;

/// Length of the header [`encode`] writes and [`decode`] expects.
const HEADER_LEN: usize = MAGIC.len() + 2;

/// Encode `value` in the asset format, header included.
///
/// # Errors
/// Propagates [`Error`] — in practice only from a type that
/// cannot be represented, since the writers target `Vec<u8>`.
pub fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    let mut out = Vec::with_capacity(HEADER_LEN);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    postcard::to_extend(value, out)
}

/// Decode an asset previously written by [`encode`].
///
/// # Errors
/// Returns [`Error`] if the header is absent, the version is
/// not [`VERSION`], or the body is not a valid encoding of `T`.
pub fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, Error> {
    if bytes.len() < HEADER_LEN || bytes[..MAGIC.len()] != MAGIC {
        // Not one of ours. `DeserializeBadEncoding` is the closest
        // postcard offers; the callers log the path alongside it.
        return Err(Error::DeserializeBadEncoding);
    }
    let version = u16::from_le_bytes([bytes[4], bytes[5]]);
    if version != VERSION {
        return Err(Error::DeserializeBadEncoding);
    }
    postcard::from_bytes(&bytes[HEADER_LEN..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn round_trips_the_shapes_the_assets_actually_use() {
        // The four asset shapes: a string→strings table (cedict), a
        // nested map (n-grams), a byte blob (the deflated connection
        // matrix) and a float.
        let table: BTreeMap<String, Vec<String>> =
            [("nihao".to_string(), vec!["你好".to_string()])].into();
        let back: BTreeMap<String, Vec<String>> = decode(&encode(&table).unwrap()).unwrap();
        assert_eq!(back, table);

        let ngram: BTreeMap<(u32, u32), f32> = [((1, 2), -3.5f32), ((7, 9), -0.25)].into();
        let back: BTreeMap<(u32, u32), f32> = decode(&encode(&ngram).unwrap()).unwrap();
        assert_eq!(back, ngram);

        let blob: Vec<u8> = vec![0x78, 0x9c, 0x00, 0xff];
        let back: Vec<u8> = decode(&encode(&blob).unwrap()).unwrap();
        assert_eq!(back, blob);
    }

    #[test]
    fn every_asset_carries_the_header() {
        let bytes = encode(&vec![1u8, 2, 3]).unwrap();
        assert_eq!(&bytes[..4], &MAGIC, "assets start with the magic");
        assert_eq!(
            u16::from_le_bytes([bytes[4], bytes[5]]),
            VERSION,
            "…followed by the format version"
        );
        // postcard varint-encodes the length, so the body is 1 + 3 bytes
        // where bincode's fixed-width framing spent 8 + 3.
        assert_eq!(bytes.len(), HEADER_LEN + 4, "got {bytes:?}");
    }

    #[test]
    fn a_foreign_or_stale_asset_is_rejected_rather_than_misread() {
        // The failure this module exists to prevent. A wrong-format file
        // decodes to plausible garbage — or to an empty table, which
        // reads as "this language has no data" — as easily as it errors,
        // and the daemon's response to an error is a quiet degrade.
        assert!(decode::<Vec<u8>>(&[]).is_err(), "empty");
        assert!(decode::<Vec<u8>>(b"KIO").is_err(), "shorter than a header");
        // A bincode-era asset: no magic at all.
        assert!(
            decode::<Vec<u8>>(&[3, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3]).is_err(),
            "an unheadered (pre-postcard) asset must not decode"
        );
        // Right magic, wrong version — the case a future format change
        // hits. Must fail on the version, not stumble into the body.
        let mut stale = encode(&vec![1u8, 2, 3]).unwrap();
        stale[4] = VERSION.wrapping_add(1).to_le_bytes()[0];
        assert!(decode::<Vec<u8>>(&stale).is_err(), "version mismatch");
    }

    #[test]
    fn a_truncated_asset_is_an_error_not_a_panic() {
        // §4.15: a corrupt asset disables that language's prediction and
        // the daemon carries on. It must not take the process down.
        let good = encode(&vec![1u32, 2, 3]).unwrap();
        let err = decode::<Vec<u32>>(&good[..good.len() - 2]);
        assert!(err.is_err(), "a truncated buffer must decode to an error");
    }
}
