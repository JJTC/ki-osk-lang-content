//! `build-predict` — build a bundled prediction dictionary + n-gram
//! model for one language from a corpus file.
//!
//! Output (in `--out <dir>`):
//!   - `<lang>.fst`       — unigram frequencies as an `fst::Map`
//!   - `<lang>.ngram.bin` — `osk_data::codec`-encoded [`NgramModel`] with
//!     bigram + trigram log-probabilities (only emitted when the
//!     corpus is big enough to yield any n-grams)
//!
//! Usage:
//!   build-predict --lang en --corpus en.txt --out predict/
//!
//! The readers live in ki-osk's `osk-core`: its FST and n-gram loaders
//! feed a `SuggestionEngine`, which re-ranks FST-prefix hits by the
//! committed-word context.
//!
//! The n-gram counter keeps only the top `--ngram-keep` bigrams and
//! trigrams by count to bound disk size: a full-corpus dump of trigram
//! frequencies is easily hundreds of megabytes.

use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Context as _;
use clap::Parser;
use osk_data::ngram::NgramModel;
use osk_data::words::{Splitter, Start};

#[derive(Parser)]
#[command(name = "build-predict")]
struct Args {
    /// BCP-47 tag used for the output file name.
    #[arg(long)]
    lang: String,
    /// Corpus path: plain UTF-8 text, a sentence or a paragraph per line.
    /// Words are found as the keyboard reads them (`osk_data::words`).
    #[arg(long)]
    corpus: PathBuf,
    /// Output directory. Files written as `<out>/<lang>.fst` and
    /// `<out>/<lang>.ngram.bin`.
    #[arg(long)]
    out: PathBuf,
    /// Cap on vocabulary size (keep the top-N most frequent words).
    #[arg(long, default_value_t = 50_000)]
    vocab_size: usize,
    /// Cap on bigram + trigram table size (keep top-N by count each).
    #[arg(long, default_value_t = 200_000)]
    ngram_keep: usize,
    /// Skip n-gram emission (FST-only build).
    #[arg(long, default_value_t = false)]
    no_ngrams: bool,
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    let mut f = fs::File::open(&args.corpus)
        .with_context(|| format!("opening {}", args.corpus.display()))?;
    let mut text = String::new();
    f.read_to_string(&mut text)?;

    // --- unigram + n-gram counts ---
    //
    // A single sweep over the words the keyboard would read (the same
    // splitter, so a word kept whole here is one the keyboard offers
    // whole), tracking the preceding two words so bigram and trigram
    // counts fall out of the same pass. The chain breaks where the
    // keyboard clears its context: a sentence end, a line end, a token
    // with no word in it (a number, a dash); n-grams across one would
    // never be asked for.
    let splitter = Splitter::new(&args.lang);
    let mut freq: HashMap<String, u64> = HashMap::new();
    let mut bigrams: HashMap<(String, String), u64> = HashMap::new();
    let mut trigrams: HashMap<(String, String, String), u64> = HashMap::new();
    let mut prev2: Option<String> = None;
    let mut prev1: Option<String> = None;
    for word in text.lines().flat_map(|line| splitter.words(line)) {
        if word.start != Start::Follows {
            prev2 = None;
            prev1 = None;
        }
        let w = word.text;
        *freq.entry(w.clone()).or_default() += 1;
        if let Some(p1) = &prev1 {
            *bigrams.entry((p1.clone(), w.clone())).or_default() += 1;
            if let Some(p2) = &prev2 {
                *trigrams
                    .entry((p2.clone(), p1.clone(), w.clone()))
                    .or_default() += 1;
            }
        }
        prev2 = prev1.take();
        prev1 = Some(w);
    }

    let mut vocab: Vec<(String, u64)> = freq.into_iter().collect();
    vocab.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    vocab.truncate(args.vocab_size);
    log::info!("{}: vocab kept = {}", args.lang, vocab.len());

    fs::create_dir_all(&args.out).with_context(|| format!("mkdir -p {}", args.out.display()))?;

    // --- FST ---
    // fst::MapBuilder wants sorted keys for insertion; freq-sort only
    // set the truncation order, so re-sort lexicographically.
    vocab.sort_by(|a, b| a.0.cmp(&b.0));
    let out_fst = args.out.join(format!("{}.fst", args.lang));
    // Atomic write: build into a sibling .tmp file then rename. A
    // Ctrl-C / OOM mid-build leaves a half-finished `.fst.tmp` (which
    // the runtime loader ignores) instead of a corrupt `<lang>.fst`
    // that ships through `make predict-assets` and into the binary
    // via include_bytes!.
    let tmp_fst = out_fst.with_extension("fst.tmp");
    let buffered = std::io::BufWriter::new(
        fs::File::create(&tmp_fst).with_context(|| format!("creating {}", tmp_fst.display()))?,
    );
    let mut build = fst::MapBuilder::new(buffered).context("building fst::MapBuilder")?;
    for (word, count) in &vocab {
        build.insert(word.as_bytes(), *count)?;
    }
    build.finish()?;
    fs::rename(&tmp_fst, &out_fst)
        .with_context(|| format!("rename {} → {}", tmp_fst.display(), out_fst.display()))?;
    log::info!("wrote {}", out_fst.display());

    // --- n-gram .bin ---
    if args.no_ngrams || (bigrams.is_empty() && trigrams.is_empty()) {
        return Ok(());
    }
    let total_tokens: u64 = vocab.iter().map(|(_, c)| *c).sum();
    // Cast to f64 directly — `f64::from(u32)` would silently truncate
    // total counts above 2³² (Tatoeba-100k caps each language at
    // ~5 M tokens, well under that, but a million-line Wikipedia dump
    // dropped onto `corpora/<lang>.txt` would absolutely overflow).
    // f64 → u64 is lossless up to 2⁵³, which is the realistic ceiling.
    #[allow(clippy::cast_precision_loss)]
    let log_total = (total_tokens.max(1) as f64).ln();

    // Keep the top-N bigrams and trigrams by count. Anything rarer than
    // this is low signal and blows up file size.
    //
    // Ties are broken by the n-gram itself, as the vocabulary sort above
    // already did. The counts come out of a HashMap, whose iteration
    // order differs on every run, and thousands of n-grams share the
    // count at the cut — so sorting by count alone kept a different
    // subset each time, and two builds of the same corpus never matched.
    // The insertion order below assigns the model's vocabulary ids, so it
    // has to be stable too.
    let mut big_vec: Vec<((String, String), u64)> = bigrams.into_iter().collect();
    big_vec.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    big_vec.truncate(args.ngram_keep);

    let mut tri_vec: Vec<((String, String, String), u64)> = trigrams.into_iter().collect();
    tri_vec.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    tri_vec.truncate(args.ngram_keep);

    let mut model = NgramModel::empty();
    let mut min_lp = f32::INFINITY;
    for ((p, w), c) in &big_vec {
        // u64 → f64 (lossless up to 2⁵³) → ln → cast to f32. The old
        // `f64::from(*c as u32)` route truncated counts above 2³² to
        // wraparound noise; ln of that is meaningless.
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        let lp = ((*c as f64).ln() - log_total) as f32;
        if lp < min_lp {
            min_lp = lp;
        }
        model.add_bigram(p, w, lp);
    }
    for ((p2, p1, w), c) in &tri_vec {
        // u64 → f64 (lossless up to 2⁵³) → ln → cast to f32. The old
        // `f64::from(*c as u32)` route truncated counts above 2³² to
        // wraparound noise; ln of that is meaningless.
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        let lp = ((*c as f64).ln() - log_total) as f32;
        if lp < min_lp {
            min_lp = lp;
        }
        model.add_trigram(p2, p1, w, lp);
    }
    // Derive `unk_log_prob` from observed data: anchor it just below
    // the rarest kept n-gram so unknown bigrams always lose to known
    // ones. A fixed floor (the `-6.0` default) can sit ABOVE a real
    // corpus's common-but-not-frequent bigrams and silently flip the
    // suggestion ranking. The 1-nat margin keeps even the rarest known bigram beating unk
    // without blowing up arithmetic.
    if min_lp.is_finite() {
        model.unk_log_prob = min_lp - 1.0;
    }

    let out_ngram = args.out.join(format!("{}.ngram.bin", args.lang));
    let bytes = model.to_bytes().context("encode NgramModel")?;
    // Atomic write — rationale identical to the FST above.
    let tmp_ngram = out_ngram.with_extension("ngram.bin.tmp");
    fs::write(&tmp_ngram, &bytes).with_context(|| format!("writing {}", tmp_ngram.display()))?;
    fs::rename(&tmp_ngram, &out_ngram)
        .with_context(|| format!("rename {} → {}", tmp_ngram.display(), out_ngram.display()))?;
    log::info!(
        "wrote {} ({} bigrams, {} trigrams, {} bytes)",
        out_ngram.display(),
        big_vec.len(),
        tri_vec.len(),
        bytes.len(),
    );
    Ok(())
}
