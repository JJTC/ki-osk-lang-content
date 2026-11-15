//! fit-ranker — learn how a transliteration table's readings are ranked
//! (its `[ranker]` section, `osk_data::translit::Ranker`) from
//! romanizations people typed, as eval-translit reads them (the word, its
//! romanization and how many typed it so, tab-separated, `</s>` between
//! sentences):
//!
//!   fit-ranker --table translit/hi.txt --lexicon predict/hi.fst \
//!       --ngrams predict/hi.ngram.bin \
//!       --train hi.translit.sampled.train.tsv --halve hi...dev.tsv \
//!       --dev hi.translit.sampled.dev.tsv [--write]
//!
//! The keyboard offers a romanization's readings (the words the table
//! spells it as, its cheapest reading, the words those begin) in the order
//! of a score: by default the word's likelihood less what reading it so
//! cost, and 0.8 times what the words before it add. A ranker scores each
//! from seven features ([`osk_data::translit::Ranker::features`]): a
//! weighed sum of them, starting at that default, and `--units` units,
//! each a tanh of its own weighed sum, starting at nothing. Of a pair's
//! readings that are spelled out (a word the letters only begin never
//! comes first), the one meant is taken to be likely as its share of
//! their scores (a softmax), and the weights are those that make the
//! words people meant likeliest, all pairs weighed by how many typed them:
//! Adam's steps down the gradient, each weight held to its start by
//! `--prior` (an L2 penalty).
//!
//! A sentence's words are read after the words before them, with the
//! n-grams (`--ngrams`), as the keyboard reads them. `--halve` takes a
//! file of sentences apart: its even sentences are learned from, its odd
//! ones chosen by, beside `--train` and `--dev`. Every `--every` steps the
//! weights so far are measured on the pairs chosen by (the word meant
//! first, as eval-translit measures it), and the best are kept, the
//! default ranking's among them: with `--write` they are written into the
//! table's `[ranker]` section, when they are not the default.

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, bail};
use clap::Parser;
use osk_data::ngram::NgramModel;
use osk_data::translit::{Lexicon, RANK_FEATURES, RankUnit, Ranker, Scheme, UNKNOWN_BELOW_RAREST};
use osk_data::words::Splitter;
use osk_data_tools::lexicon::{FstLexicon, InContext};

#[derive(Parser)]
#[command(about = "Learn how a transliteration table's readings are ranked")]
struct Args {
    /// The table, `translit/<lang>.txt`.
    #[arg(long)]
    table: PathBuf,
    /// The lexicon, `predict/<lang>.fst`.
    #[arg(long)]
    lexicon: PathBuf,
    /// The language's n-grams, for the words before a sentence's word.
    #[arg(long)]
    ngrams: Option<PathBuf>,
    /// The language whose words' identities are compared; the table's file
    /// name by default.
    #[arg(long)]
    lang: Option<String>,
    /// Pairs to learn from.
    #[arg(long)]
    train: Vec<PathBuf>,
    /// Pairs to choose by.
    #[arg(long)]
    dev: Vec<PathBuf>,
    /// Sentences halved: the even ones learned from, the odd chosen by.
    #[arg(long)]
    halve: Vec<PathBuf>,
    /// How many units bend the sum.
    #[arg(long, default_value_t = 4)]
    units: usize,
    /// Adam's steps.
    #[arg(long, default_value_t = 400)]
    steps: usize,
    /// Its step size.
    #[arg(long, default_value_t = 0.01)]
    rate: f64,
    /// The L2 penalty holding each weight to its start.
    #[arg(long, default_value_t = 0.001)]
    prior: f64,
    /// Measure on the pairs chosen by every this many steps.
    #[arg(long, default_value_t = 20)]
    every: usize,
    /// Write the best weights into the table.
    #[arg(long)]
    write: bool,
    /// Print this many of the pairs chosen by whose word the default
    /// ranking puts below another, the commonest first.
    #[arg(long, default_value_t = 0)]
    show: usize,
}

/// A pair: what was typed, the word meant and the word before it.
struct Pair {
    word: String,
    latin: String,
    weight: f64,
    before: Vec<String>,
}

/// The pairs of `path`, and its sentences apart when it has any (each
/// pair after the words before it in its sentence).
fn load(path: &PathBuf, splitter: &Splitter) -> anyhow::Result<Vec<Vec<Pair>>> {
    let text = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let sentences = text.lines().any(|l| l.starts_with("</s>"));
    let mut out: Vec<Vec<Pair>> = vec![Vec::new()];
    let mut before: Vec<String> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let mut fields = line.split('\t');
        let (Some(word), Some(latin)) = (fields.next(), fields.next()) else {
            bail!("{}:{}: not word<TAB>romanization", path.display(), i + 1);
        };
        let weight: f64 = fields.next().and_then(|c| c.parse().ok()).unwrap_or(1.0);
        let context = if sentences {
            before.clone()
        } else {
            Vec::new()
        };
        if word == "</s>" {
            before.clear();
            out.push(Vec::new());
            continue;
        }
        before.push(splitter.normalize(word));
        if before.len() > 2 {
            before.remove(0);
        }
        if word == latin || latin.is_empty() {
            continue;
        }
        if let Some(last) = out.last_mut() {
            last.push(Pair {
                word: splitter.normalize(word),
                latin: latin.to_string(),
                weight,
                before: context,
            });
        }
    }
    out.retain(|s| !s.is_empty());
    Ok(out)
}

/// A pair's readings that are spelled out, as features, and which of them
/// is the word meant (if any is).
struct Example {
    features: Vec<[f64; RANK_FEATURES]>,
    texts: Vec<String>,
    meant: Option<usize>,
    weight: f64,
    /// Whether the lexicon has the word meant.
    known: bool,
}

fn examples(
    scheme: &Scheme,
    lexicon: &dyn Lexicon,
    model: Option<&NgramModel>,
    splitter: &Splitter,
    pairs: &[Pair],
) -> Vec<Example> {
    pairs
        .iter()
        .map(|p| {
            let in_context = model.map(|m| InContext {
                inner: lexicon,
                model: m,
                before: &p.before,
                splitter,
                scale: 1.0,
            });
            let lexicon: &dyn Lexicon = match &in_context {
                Some(c) => c,
                None => lexicon,
            };
            let mut features = Vec::new();
            let mut texts = Vec::new();
            let mut meant = None;
            for (c, x) in scheme.rankable(&p.latin, lexicon) {
                if c.completes {
                    continue;
                }
                let text = splitter.normalize(&c.text);
                if text == p.word {
                    meant = Some(features.len());
                }
                features.push(x.map(f64::from));
                texts.push(c.text);
            }
            Example {
                features,
                texts,
                meant,
                weight: p.weight,
                known: lexicon.log_prob(&p.word).is_some(),
            }
        })
        .collect()
}

/// The weights, flat: the sum's, then each unit's bias, weights and out.
#[derive(Clone)]
struct Weights(Vec<f64>);

const UNIT: usize = RANK_FEATURES + 2;

impl Weights {
    fn start(units: usize) -> Self {
        let standard = Ranker::standard();
        let mut w: Vec<f64> = standard.linear.iter().copied().map(f64::from).collect();
        // Each unit's weights small and apart, deterministically, its out
        // nothing: it starts adding nothing.
        let mut seed: u64 = 0x9E37_79B9_7F4A_7C15;
        for _ in 0..units {
            w.push(0.0);
            for _ in 0..RANK_FEATURES {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                #[allow(clippy::cast_precision_loss)]
                let r = (seed % 2001) as f64 / 1000.0 - 1.0;
                w.push(0.05 * r);
            }
            w.push(0.0);
        }
        Self(w)
    }

    fn units(&self) -> usize {
        (self.0.len() - RANK_FEATURES) / UNIT
    }

    /// The score of `x`, and (into `grad`, times `by`) its gradient.
    fn score(&self, x: &[f64; RANK_FEATURES], grad: Option<(&mut [f64], f64)>) -> f64 {
        let w = &self.0;
        let mut s: f64 = w[..RANK_FEATURES].iter().zip(x).map(|(a, b)| a * b).sum();
        let mut tanhs = Vec::with_capacity(self.units());
        for u in 0..self.units() {
            let o = RANK_FEATURES + u * UNIT;
            let a = w[o]
                + w[o + 1..=o + RANK_FEATURES]
                    .iter()
                    .zip(x)
                    .map(|(a, b)| a * b)
                    .sum::<f64>();
            let t = a.tanh();
            s += w[o + RANK_FEATURES + 1] * t;
            tanhs.push(t);
        }
        if let Some((g, by)) = grad {
            for (gi, xi) in g[..RANK_FEATURES].iter_mut().zip(x) {
                *gi += by * xi;
            }
            for (u, t) in tanhs.iter().enumerate() {
                let o = RANK_FEATURES + u * UNIT;
                let out = w[o + RANK_FEATURES + 1];
                let d = by * out * (1.0 - t * t);
                g[o] += d;
                for (k, xi) in x.iter().enumerate() {
                    g[o + 1 + k] += d * xi;
                }
                g[o + RANK_FEATURES + 1] += by * t;
            }
        }
        s
    }

    #[allow(clippy::cast_possible_truncation)]
    fn ranker(&self) -> Ranker {
        let w = &self.0;
        let mut linear = [0.0f32; RANK_FEATURES];
        for (l, v) in linear.iter_mut().zip(&w[..RANK_FEATURES]) {
            *l = *v as f32;
        }
        let units = (0..self.units())
            .map(|u| {
                let o = RANK_FEATURES + u * UNIT;
                let mut weights = [0.0f32; RANK_FEATURES];
                for (k, v) in weights.iter_mut().enumerate() {
                    *v = w[o + 1 + k] as f32;
                }
                RankUnit {
                    bias: w[o] as f32,
                    weights,
                    out: w[o + RANK_FEATURES + 1] as f32,
                }
            })
            .collect();
        Ranker { linear, units }
    }
}

/// The weighed mean loss (the meant word's share of its pair's scores,
/// less its log) over `examples`, the prior, and the gradient into `grad`.
fn loss(examples: &[Example], w: &Weights, start: &Weights, prior: f64, grad: &mut [f64]) -> f64 {
    grad.iter_mut().for_each(|g| *g = 0.0);
    let total: f64 = examples
        .iter()
        .filter(|e| e.meant.is_some())
        .map(|e| e.weight)
        .sum();
    let mut sum = 0.0;
    for e in examples {
        let Some(m) = e.meant else { continue };
        let scores: Vec<f64> = e.features.iter().map(|x| w.score(x, None)).collect();
        let top = scores.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let z: f64 = scores.iter().map(|s| (s - top).exp()).sum();
        sum += e.weight * (z.ln() + top - scores[m]);
        for (i, x) in e.features.iter().enumerate() {
            let p = (scores[i] - top).exp() / z;
            let by = e.weight / total * (p - if i == m { 1.0 } else { 0.0 });
            w.score(x, Some((grad, by)));
        }
    }
    let mut penalty = 0.0;
    for (i, (a, b)) in w.0.iter().zip(&start.0).enumerate() {
        penalty += prior * (a - b) * (a - b);
        grad[i] += 2.0 * prior * (a - b);
    }
    sum / total.max(1e-9) + penalty
}

/// How often (by weight) the meant word scores best of a pair's readings
/// spelled out, ties to the text first in order, as the keyboard ranks.
fn first(examples: &[Example], w: &Weights) -> f64 {
    let (mut all, mut right) = (0.0, 0.0);
    for e in examples {
        all += e.weight;
        let Some(m) = e.meant else { continue };
        let mut best: Option<(f64, &str, usize)> = None;
        for (i, x) in e.features.iter().enumerate() {
            let s = w.score(x, None);
            let better = match best {
                None => true,
                Some((bs, bt, _)) => s > bs || (s == bs && e.texts[i].as_str() < bt),
            };
            if better {
                best = Some((s, &e.texts[i], i));
            }
        }
        if best.is_some_and(|(_, _, i)| i == m) {
            right += e.weight;
        }
    }
    if all > 0.0 { 100.0 * right / all } else { 0.0 }
}

/// The table's text with `ranker` as its `[ranker]` section, any before it
/// taken out.
fn written(table: &str, ranker: &Ranker) -> String {
    let mut out = String::new();
    let mut skipping = false;
    for line in table.lines() {
        let t = line.trim();
        if t.starts_with('[') && t.ends_with(']') {
            skipping = t == "[ranker]";
            if skipping {
                // and the comment lines just before it, which are its own
                while out.ends_with("\n\n") {
                    out.pop();
                }
                let mut lines: Vec<&str> = out.lines().collect();
                while lines.last().is_some_and(|l| {
                    l.starts_with("# The readings' ranking") || l.starts_with("# learned")
                }) {
                    lines.pop();
                }
                out = lines.join("\n");
                out.push('\n');
                continue;
            }
        }
        if !skipping {
            out.push_str(line);
            out.push('\n');
        }
    }
    while out.ends_with("\n\n") {
        out.pop();
    }
    out.push_str(
        "\n# The readings' ranking, learned from romanizations people typed\n\
         # (make fit-ranker; osk_data::translit::Ranker).\n[ranker]\n",
    );
    out.push_str(&ranker.to_table());
    out
}

#[allow(clippy::too_many_lines)]
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let text = fs::read_to_string(&args.table)
        .with_context(|| format!("reading {}", args.table.display()))?;
    // Learned against the default ranking: the readings are the same
    // whatever ranks them.
    let scheme = Scheme::parse(&text)
        .with_context(|| format!("{}", args.table.display()))?
        .with_ranker(None);
    let lexicon = FstLexicon::open(
        fs::read(&args.lexicon).with_context(|| format!("reading {}", args.lexicon.display()))?,
        UNKNOWN_BELOW_RAREST,
    )?;
    let model = match &args.ngrams {
        Some(p) => Some(
            NgramModel::from_bytes(
                &fs::read(p).with_context(|| format!("reading {}", p.display()))?,
            )
            .with_context(|| format!("{}", p.display()))?,
        ),
        None => None,
    };
    let lang = match &args.lang {
        Some(l) => l.clone(),
        None => args
            .table
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .context("no language in the table's name")?,
    };
    let splitter = Splitter::new(&lang);
    let mut train_pairs: Vec<Pair> = Vec::new();
    let mut dev_pairs: Vec<Pair> = Vec::new();
    for p in &args.train {
        train_pairs.extend(load(p, &splitter)?.into_iter().flatten());
    }
    for p in &args.dev {
        dev_pairs.extend(load(p, &splitter)?.into_iter().flatten());
    }
    for p in &args.halve {
        for (i, s) in load(p, &splitter)?.into_iter().enumerate() {
            if i % 2 == 0 {
                train_pairs.extend(s);
            } else {
                dev_pairs.extend(s);
            }
        }
    }
    if train_pairs.is_empty() || dev_pairs.is_empty() {
        bail!("nothing to learn from or to choose by");
    }
    let train = examples(&scheme, &lexicon, model.as_ref(), &splitter, &train_pairs);
    let dev = examples(&scheme, &lexicon, model.as_ref(), &splitter, &dev_pairs);
    let start = Weights::start(args.units);
    let mut w = start.clone();
    let mut grad = vec![0.0; w.0.len()];
    let (mut m, mut v) = (vec![0.0; w.0.len()], vec![0.0; w.0.len()]);
    let (b1, b2) = (0.9_f64, 0.999_f64);
    let first_dev = first(&dev, &start);
    if args.show > 0 {
        let mut misses: std::collections::HashMap<(String, String, String), f64> =
            std::collections::HashMap::new();
        for (e, p) in dev.iter().zip(&dev_pairs) {
            let Some(m) = e.meant else { continue };
            let (mut bi, mut bs) = (0, f64::NEG_INFINITY);
            for (i, x) in e.features.iter().enumerate() {
                let sc = start.score(x, None);
                if sc > bs || (sc == bs && e.texts[i] < e.texts[bi]) {
                    (bi, bs) = (i, sc);
                }
            }
            if bi != m {
                let key = (p.latin.clone(), e.texts[m].clone(), e.texts[bi].clone());
                *misses.entry(key).or_default() += e.weight;
            }
        }
        let mut misses: Vec<_> = misses.into_iter().collect();
        misses.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        for ((latin, meant, got), n) in misses.into_iter().take(args.show) {
            println!("  {n:>6.0}  {latin}\t{meant} <- {got}");
        }
        // And the words the lexicon has that no reading writes.
        let mut unreached: std::collections::HashMap<(String, String, String), f64> =
            std::collections::HashMap::new();
        for (e, p) in dev.iter().zip(&dev_pairs) {
            if e.known && e.meant.is_none() {
                let got = e.texts.first().cloned().unwrap_or_default();
                *unreached
                    .entry((p.latin.clone(), p.word.clone(), got))
                    .or_default() += e.weight;
            }
        }
        let mut unreached: Vec<_> = unreached.into_iter().collect();
        unreached.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        println!("  known words no reading writes:");
        for ((latin, meant, got), n) in unreached.into_iter().take(args.show) {
            println!("  {n:>6.0}  {latin}\t{meant}  (a reading: {got})");
        }
    }
    {
        // How often the meant word is a reading at all: the most a ranker
        // can put first.
        let reach = |ex: &[Example]| -> f64 {
            let all: f64 = ex.iter().map(|e| e.weight).sum();
            let there: f64 = ex
                .iter()
                .filter(|e| e.meant.is_some())
                .map(|e| e.weight)
                .sum();
            100.0 * there / all.max(1e-9)
        };
        println!(
            "  the word meant is a reading: learned-from {:.2}%, chosen-by {:.2}%",
            reach(&train),
            reach(&dev)
        );
        let known: Vec<&Example> = dev.iter().filter(|e| e.known).collect();
        let all: f64 = dev.iter().map(|e| e.weight).sum();
        let k: f64 = known.iter().map(|e| e.weight).sum();
        let kr: f64 = known
            .iter()
            .filter(|e| e.meant.is_some())
            .map(|e| e.weight)
            .sum();
        println!(
            "  of those chosen by, the lexicon has {:.2}%, and of those the word meant is a reading for {:.2}%",
            100.0 * k / all.max(1e-9),
            100.0 * kr / k.max(1e-9)
        );
    }
    println!(
        "{}: {} pairs to learn from, {} to choose by",
        args.table.display(),
        train.len(),
        dev.len()
    );
    println!(
        "  the default ranking: learned-from first {:.2}%, chosen-by first {first_dev:.2}%",
        first(&train, &start)
    );
    let mut best = (first_dev, start.clone(), 0);
    for step in 1..=args.steps {
        let l = loss(&train, &w, &start, args.prior, &mut grad);
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let t = step as i32;
        for i in 0..w.0.len() {
            m[i] = b1 * m[i] + (1.0 - b1) * grad[i];
            v[i] = b2 * v[i] + (1.0 - b2) * grad[i] * grad[i];
            let mh = m[i] / (1.0 - b1.powi(t));
            let vh = v[i] / (1.0 - b2.powi(t));
            w.0[i] -= args.rate * mh / (vh.sqrt() + 1e-8);
        }
        if step % args.every == 0 || step == args.steps {
            let d = first(&dev, &w);
            println!(
                "  step {step}: loss {l:.4}, learned-from first {:.2}%, chosen-by first {d:.2}%",
                first(&train, &w)
            );
            if d > best.0 {
                best = (d, w.clone(), step);
            }
        }
    }
    let (d, w, step) = best;
    println!("  kept: step {step} (chosen-by first {d:.2}%, from {first_dev:.2}%)");
    let ranker = w.ranker();
    print!("{}", ranker.to_table());
    if args.write && step > 0 {
        fs::write(&args.table, written(&text, &ranker))
            .with_context(|| format!("writing {}", args.table.display()))?;
        println!("  written to {}", args.table.display());
    }
    Ok(())
}
