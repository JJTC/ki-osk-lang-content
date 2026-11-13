//! A prediction lexicon (`predict/<lang>.fst`) as a transliteration table
//! checks its readings against it ([`osk_data::translit::Lexicon`]), read
//! as the keyboard reads it: a word's likelihood its count's share of all
//! the counts. What `eval-translit` and `fit-translit` both read.

use fst::raw::Fst;
use osk_data::translit::Lexicon;

/// A lexicon file in memory.
pub struct FstLexicon {
    map: fst::Map<Vec<u8>>,
    log_total: f32,
    unknown: f32,
}

impl FstLexicon {
    /// The lexicon in `bytes`, a reading that is no word put `below` nats
    /// under its rarest word (the keyboard's is
    /// [`osk_data::translit::UNKNOWN_BELOW_RAREST`]).
    ///
    /// # Errors
    ///
    /// When `bytes` are no FST map.
    pub fn open(bytes: Vec<u8>, below: f32) -> anyhow::Result<Self> {
        let map = fst::Map::new(bytes)?;
        let mut total: u64 = 0;
        let mut rarest = u64::MAX;
        let mut stream = map.stream();
        while let Some((_, v)) = fst::Streamer::next(&mut stream) {
            total = total.saturating_add(v);
            rarest = rarest.min(v.max(1));
        }
        #[allow(clippy::cast_precision_loss)]
        let log_total = (total.max(1) as f32).ln();
        #[allow(clippy::cast_precision_loss)]
        let unknown = (rarest as f32).ln() - log_total - below;
        Ok(Self {
            map,
            log_total,
            unknown,
        })
    }
}

impl Lexicon for FstLexicon {
    fn has_prefix(&self, prefix: &str) -> bool {
        let fst: &Fst<Vec<u8>> = self.map.as_fst();
        let mut node = fst.root();
        for b in prefix.bytes() {
            let Some(i) = node.find_input(b) else {
                return false;
            };
            node = fst.node(node.transition_addr(i));
        }
        node.is_final() || !node.is_empty()
    }

    fn log_prob(&self, word: &str) -> Option<f32> {
        #[allow(clippy::cast_precision_loss)]
        self.map
            .get(word.as_bytes())
            .map(|c| (c.max(1) as f32).ln() - self.log_total)
    }

    fn unknown_log_prob(&self) -> f32 {
        self.unknown
    }

    fn completions(&self, prefix: &str, limit: usize) -> Vec<(String, f32)> {
        use fst::{IntoStreamer, Streamer, automaton::Str};
        let aut = fst::Automaton::starts_with(Str::new(prefix));
        let mut stream = self.map.search(aut).into_stream();
        let mut found: Vec<(String, f32)> = Vec::new();
        while let Some((k, v)) = stream.next() {
            if k.len() > prefix.len()
                && let Ok(w) = std::str::from_utf8(k)
            {
                #[allow(clippy::cast_precision_loss)]
                found.push((w.to_string(), (v.max(1) as f32).ln() - self.log_total));
            }
        }
        found.sort_by(|a, b| b.1.total_cmp(&a.1));
        found.truncate(limit);
        found
    }
}
