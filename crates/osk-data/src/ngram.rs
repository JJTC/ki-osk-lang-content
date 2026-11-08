//! Bigram + trigram re-ranking.
//!
//! At runtime we hold:
//!   - `vocab`: word → id (u32)
//!   - `bigram`: (w_{-1}, w) → log P(w | w_{-1})
//!   - `trigram`: (w_{-2}, w_{-1}, w) → log P(w | w_{-2}, w_{-1})
//!
//! Re-ranking score = `alpha * unigram + beta * bigram + gamma * trigram`
//! with weights chosen at load time.

use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

/// Stored n-gram model. Built by `build-predict` (osk-data-tools).
///
/// The maps are written in key order (see `sorted`), so a model always
/// encodes to the same bytes. Lookups stay `HashMap`s; only the encoding
/// is ordered, and the wire format is a plain postcard map, so the order
/// is the writer's choice rather than part of the format.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NgramModel {
    #[serde(serialize_with = "sorted")]
    vocab: HashMap<String, u32>,
    #[serde(serialize_with = "sorted")]
    bigram: HashMap<(u32, u32), f32>,
    #[serde(serialize_with = "sorted")]
    trigram: HashMap<(u32, u32, u32), f32>,
    /// Log-prob to subtract for unknown n-grams (smoothing).
    pub unk_log_prob: f32,
}

/// Serialize a `HashMap` in key order. Its own iteration order is seeded
/// per map instance, so serializing it directly would give every build
/// different bytes for the same model, and a rebuilt table could not be
/// told apart from a changed one.
fn sorted<K, V, S>(map: &HashMap<K, V>, s: S) -> Result<S::Ok, S::Error>
where
    K: Ord + Serialize,
    V: Serialize,
    S: Serializer,
{
    let mut entries: Vec<(&K, &V)> = map.iter().collect();
    entries.sort_unstable_by(|a, b| a.0.cmp(b.0));
    s.collect_map(entries)
}

impl NgramModel {
    /// Empty model — all n-gram scores fall back to `unk_log_prob`.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            unk_log_prob: -6.0,
            ..Self::default()
        }
    }

    /// Insert a vocabulary entry. Stable id = insertion order.
    #[allow(clippy::cast_possible_truncation)] // vocab capped at ~20k words, far below u32::MAX
    pub fn insert_word(&mut self, w: &str) -> u32 {
        let next = self.vocab.len() as u32;
        *self.vocab.entry(w.to_string()).or_insert(next)
    }

    /// Add a bigram (prev, word) with log-probability.
    pub fn add_bigram(&mut self, prev: &str, word: &str, lp: f32) {
        let p = self.insert_word(prev);
        let w = self.insert_word(word);
        self.bigram.insert((p, w), lp);
    }

    /// Add a trigram (p2, p1, word) with log-probability.
    pub fn add_trigram(&mut self, p2: &str, p1: &str, word: &str, lp: f32) {
        let a = self.insert_word(p2);
        let b = self.insert_word(p1);
        let c = self.insert_word(word);
        self.trigram.insert((a, b, c), lp);
    }

    /// Score `word` given optional trailing context (most-recent last).
    ///
    /// `context` is a slice of already-committed words, oldest to newest.
    /// Uses trigram if two words of context, falls back to bigram, then to
    /// `unk_log_prob`.
    #[must_use]
    pub fn score(&self, word: &str, context: &[&str]) -> f32 {
        let Some(&w) = self.vocab.get(word) else {
            return self.unk_log_prob;
        };
        if context.len() >= 2 {
            let p2 = self.vocab.get(context[context.len() - 2]);
            let p1 = self.vocab.get(context[context.len() - 1]);
            if let (Some(&a), Some(&b)) = (p2, p1)
                && let Some(&lp) = self.trigram.get(&(a, b, w))
            {
                return lp;
            }
        }
        if !context.is_empty()
            && let Some(&p1) = self.vocab.get(context[context.len() - 1])
            && let Some(&lp) = self.bigram.get(&(p1, w))
        {
            return lp;
        }
        self.unk_log_prob
    }

    /// Number of vocabulary entries.
    #[must_use]
    pub fn vocab_len(&self) -> usize {
        self.vocab.len()
    }

    /// Number of bigram entries.
    #[must_use]
    pub fn bigram_len(&self) -> usize {
        self.bigram.len()
    }

    /// Number of trigram entries.
    #[must_use]
    pub fn trigram_len(&self) -> usize {
        self.trigram.len()
    }

    /// Serialize to a compact byte buffer suitable for bundling as an
    /// asset. Framing is [`crate::codec`]'s — see there before
    /// changing it.
    ///
    /// # Errors
    /// Propagates [`crate::codec::Error`] (shouldn't happen in
    /// practice since all fields are simple primitives and maps).
    pub fn to_bytes(&self) -> Result<Vec<u8>, crate::codec::Error> {
        crate::codec::encode(self)
    }

    /// Deserialize from a byte buffer produced by [`Self::to_bytes`]. Typically
    /// called on an `include_bytes!`'d asset at startup.
    ///
    /// # Errors
    /// Returns [`crate::codec::Error`] if the bytes are not a valid encoding
    /// of `NgramModel`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, crate::codec::Error> {
        crate::codec::decode(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigram_beats_bigram_when_present() {
        let mut m = NgramModel::empty();
        m.add_bigram("is", "good", -2.0);
        m.add_trigram("all", "is", "good", -0.5);
        assert!((m.score("good", &["is"]) - (-2.0)).abs() < 1e-6);
        assert!((m.score("good", &["all", "is"]) - (-0.5)).abs() < 1e-6);
    }

    #[test]
    fn unknown_word_is_unk() {
        let m = NgramModel::empty();
        assert!((m.score("zzz", &["foo"]) - m.unk_log_prob).abs() < 1e-6);
    }

    #[test]
    fn roundtrip_through_the_codec_preserves_scores() {
        let mut m = NgramModel::empty();
        m.add_bigram("is", "good", -2.0);
        m.add_trigram("all", "is", "good", -0.5);
        let bytes = m.to_bytes().unwrap();
        let back = NgramModel::from_bytes(&bytes).unwrap();
        assert!((back.score("good", &["is"]) - (-2.0)).abs() < 1e-6);
        assert!((back.score("good", &["all", "is"]) - (-0.5)).abs() < 1e-6);
        assert_eq!(back.vocab_len(), m.vocab_len());
        assert_eq!(back.bigram_len(), m.bigram_len());
        assert_eq!(back.trigram_len(), m.trigram_len());
    }

    #[test]
    fn the_same_model_always_encodes_to_the_same_bytes() {
        // Two models built separately from the same entries hold
        // HashMaps with different random seeds, so they iterate in
        // different orders; serializing in key order is what makes
        // their bytes equal.
        let build = || {
            let mut m = NgramModel::empty();
            for i in 0u16..300 {
                m.add_bigram(
                    &format!("w{i}"),
                    &format!("w{}", i + 1),
                    -1.0 - f32::from(i) / 300.0,
                );
                m.add_trigram(
                    &format!("w{i}"),
                    &format!("w{}", i + 1),
                    &format!("w{}", i + 2),
                    -2.0,
                );
            }
            m
        };
        let (a, b) = (build(), build());
        assert_eq!(a.to_bytes().unwrap(), b.to_bytes().unwrap());
        // And the ordered encoding still decodes to the same model.
        let back = NgramModel::from_bytes(&a.to_bytes().unwrap()).unwrap();
        assert_eq!(back.to_bytes().unwrap(), a.to_bytes().unwrap());
        assert!((back.score("w2", &["w0", "w1"]) - -2.0).abs() < 1e-6);
    }
}
