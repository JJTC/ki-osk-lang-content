//! The one place the binary-asset encoding is decided.
//!
//! Every `.bin` in the language data — the per-language n-gram tables,
//! the CC-CEDICT Pinyin and Jyutping tables, the Zhuyin table, the
//! mozc-derived kanji dictionary and its POS-bigram connection matrix —
//! is written and read through [`encode`] and [`decode`] here.
//!
//! That is not tidiness. The writers are this repository's builders and
//! the readers are ki-osk's `osk-core`, and **a decoder that disagrees
//! with its encoder does not crash**: the loader returns `None`, the
//! daemon logs a degrade, falls back to the seed dictionaries, and
//! prediction quietly gets worse. One shared crate plus the header below
//! is what turns that from silent into loud.
//!
//! # Why postcard
//!
//! The assets were `bincode` until 2026-07-28. **bincode is
//! unmaintained**: development ceased after a doxxing and harassment
//! incident, the repository was archived in August 2025, and `3.0.0` is a
//! tombstone release whose `lib.rs` contains nothing but a
//! `compile_error!` announcing the fact. Its own README points readers at
//! `wincode`, `postcard` and `rkyv`.
//!
//! `postcard` was chosen from that list because it is a mature 1.x, is
//! actively maintained, is `MIT OR Apache-2.0` like the rest of the tree,
//! and speaks plain `serde` — so the swap is confined to this file rather
//! than spreading schema traits through every asset type. `wincode` would
//! have kept the old bytes valid, but it is pre-1.0 and its compatibility
//! is conditional on hand-written schemas matching, which is the same
//! silent-mismatch risk this module exists to remove. `rkyv`'s zero-copy
//! model is attractive for the read-mostly 6.9 MB kanji dictionary, but
//! startup decode is already well inside budget and it would mean
//! reworking the loaders.
//!
//! postcard's varint lengths also make the assets smaller than bincode's
//! fixed-width framing did.
//!
//! # The header
//!
//! Every asset starts with [`MAGIC`] and a `u16` [`VERSION`]. Neither the
//! old bincode assets nor a future format change can be mistaken for the
//! current one: [`decode`] rejects anything whose header does not match
//! and says which version it found. That matters because the failure it
//! guards is invisible without it — a wrong-format asset decodes to
//! plausible garbage or an empty table just as easily as it errors.
//!
//! There is deliberately **no compatibility path for the bincode-era
//! files**: none was ever published, and carrying a reader for them would
//! mean keeping the unmaintained dependency that prompted the change.

/// The error every decode and encode returns.
pub use postcard::Error;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Leading bytes of every asset. `KIOS` + a version, so a file that is
/// not one of ours — or is one of ours from a different era — is
/// rejected by inspection rather than by a decode that happens to fail.
pub const MAGIC: [u8; 4] = *b"KIOS";

/// Asset format version. **Bump this whenever the encoding or any asset
/// type's layout changes**, so a stale file fails loudly instead of
/// decoding to something plausible.
///
/// 1 = postcard, replacing the unversioned bincode assets.
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
