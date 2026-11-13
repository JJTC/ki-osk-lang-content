//! fit-translit — fit a transliteration table's costs (`translit/<lang>.txt`)
//! to romanizations people typed, as eval-translit reads them (the word,
//! its romanization and how many typed it so, tab-separated):
//!
//!   fit-translit --table translit/hi.txt --lexicon predict/hi.fst \
//!       --train hi.translit.sampled.train.tsv --dev hi.translit.sampled.dev.tsv \
//!       [--write]
//!
//! The keyboard ranks a romanization's readings by the word's likelihood
//! less what reading it so cost (`osk_data::translit`), and a reading's
//! cost is what the table charges for each way it took (`t ट:0.5`) and an
//! abugida's settings (`schwa 1.5`). So of a pair's readings, a word's
//! likelihood of being the one meant is taken to be its share of all of
//! theirs (a softmax, its scores times `--sharp`), and the costs are those
//! that make the words people meant likeliest, all pairs weighed by how
//! many typed them: Adam's steps follow the gradient down, no cost below
//! 0, each held to the table's own by `--prior` (an L2 penalty, per cost)
//! so that a way the pairs say little of stays as written.
//!
//! A word may be read more than one way (`aa` as one rule, or as two): it
//! costs the soft minimum of its cheapest ways' costs (`--soft` nats;
//! the keyboard takes the cheapest), so that a cost raised on one way
//! lets it be read another, as the keyboard would. Which ways are the
//! cheapest changes with the costs, so the pairs are read again each
//! round (`--rounds`). The keyboard offers a romanization's cheapest
//! reading too, a word or not, at the lexicon's likelihood of an unknown
//! word: so costs that let nonsense read cheaply (every `a` a long ā)
//! cost the words meant too. That reading changes faster, and is read
//! again every `--refresh` steps. A word the lexicon lacks is meant to be
//! it: its own cheapest way of being read joins its pair's readings. A
//! pair whose word no reading writes (an English word in its English
//! spelling, an acronym) teaches nothing and is counted aside.
//!
//! The table's own costs and each round's, rounded to `--grain`, are
//! measured on `--dev` (the word meant first, as eval-translit measures
//! it), and those that put the most first are kept: with `--write` they
//! are written into the table (when they are not its own), each way's
//! cost in its place, the rest of the file as it was. The ways that moved
//! most are printed.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, bail};
use clap::Parser;
use osk_data::translit::{Lexicon, Scheme, UNKNOWN_BELOW_RAREST};
use osk_data::words::Splitter;
use osk_data_tools::lexicon::FstLexicon;

#[derive(Parser)]
#[command(about = "Fit a transliteration table's costs to romanizations people typed")]
struct Args {
    /// The table, `translit/<lang>.txt`.
    #[arg(long)]
    table: PathBuf,
    /// The lexicon, `predict/<lang>.fst`.
    #[arg(long)]
    lexicon: PathBuf,
    /// Pairs to fit on: word, romanization and count, tab-separated, one
    /// pair a line (as eval-translit reads them). Repeatable.
    #[arg(long, required = true)]
    train: Vec<PathBuf>,
    /// Pairs to choose the round by; without them, the last round's costs
    /// are kept. Repeatable.
    #[arg(long)]
    dev: Vec<PathBuf>,
    /// Times the pairs are read and the costs fitted to their readings.
    #[arg(long, default_value_t = 4)]
    rounds: usize,
    /// Gradient steps a round.
    #[arg(long, default_value_t = 200)]
    steps: usize,
    /// How strongly each cost is held to the table's own (L2, per cost).
    #[arg(long, default_value_t = 0.01)]
    prior: f64,
    /// The step size (Adam's).
    #[arg(long, default_value_t = 0.02)]
    rate: f64,
    /// How soft the minimum of a reading's ways' costs is, in nats.
    #[arg(long, default_value_t = 0.1)]
    soft: f64,
    /// How sharply the likelier readings are weighed: their scores times
    /// this before the softmax, so that above 1 the loss looks more at
    /// which comes first, as the keyboard does.
    #[arg(long, default_value_t = 1.0)]
    sharp: f64,
    /// Gradient steps between readings of each romanization's cheapest
    /// reading.
    #[arg(long, default_value_t = 20)]
    refresh: usize,
    /// What the costs written are rounded to.
    #[arg(long, default_value_t = 0.1)]
    grain: f32,
    /// Write the kept costs into the table.
    #[arg(long)]
    write: bool,
    /// Write the table with the kept costs here instead.
    #[arg(long)]
    out: Option<PathBuf>,
    /// The language whose words' identities are compared; the table's
    /// file name (`hi` of `translit/hi.txt`) by default.
    #[arg(long)]
    lang: Option<String>,
    /// Print this many of the ways whose costs moved most.
    #[arg(long, default_value_t = 30)]
    show: usize,
    /// Print this many of the pairs whose word no reading writes.
    #[arg(long, default_value_t = 0)]
    unwritten: usize,
}

/// A word and a romanization of it, and how many typed it so.
struct Pair {
    /// The word's identity (`Splitter::normalize`).
    word: String,
    latin: String,
    weight: f64,
}

fn load(path: &PathBuf, splitter: &Splitter) -> anyhow::Result<Vec<Pair>> {
    let text = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut pairs = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let mut fields = line.split('\t');
        let (Some(word), Some(latin)) = (fields.next(), fields.next()) else {
            bail!("{}:{}: not word<TAB>romanization", path.display(), i + 1);
        };
        let weight: f64 = fields.next().and_then(|c| c.parse().ok()).unwrap_or(1.0);
        if word == "</s>" || word == latin || latin.is_empty() {
            continue;
        }
        pairs.push(Pair {
            word: splitter.normalize(word),
            latin: latin.to_string(),
            weight,
        });
    }
    Ok(pairs)
}

/// A lexicon of one word, by its identity: what reads a word's own
/// cheapest reading where the lexicon lacks it.
struct Only<'a> {
    word: &'a str,
    splitter: &'a Splitter,
}

impl Lexicon for Only<'_> {
    fn has_prefix(&self, prefix: &str) -> bool {
        self.word.starts_with(&self.splitter.normalize(prefix))
    }
    fn log_prob(&self, word: &str) -> Option<f32> {
        (self.splitter.normalize(word) == self.word).then_some(0.0)
    }
    fn unknown_log_prob(&self) -> f32 {
        0.0
    }
}

/// The pairs' readings, flat: a pair's words from `starts[p]` to
/// `starts[p + 1]`, a word's ways from `ways[r]` to `ways[r + 1]`, a way's
/// charges from `charged[w]` to `charged[w + 1]`; and each pair's
/// cheapest reading, where it is no word and not the word meant.
#[derive(Default)]
struct Readings {
    /// Each kept pair's place in the pairs.
    pair: Vec<usize>,
    weights: Vec<f64>,
    starts: Vec<usize>,
    /// A reading's likelihood (the lexicon's unknown's where it is no
    /// word), and whether it is the word meant.
    likelihood: Vec<f64>,
    meant: Vec<bool>,
    ways: Vec<usize>,
    /// What of a way's cost no table sets.
    fixed: Vec<f64>,
    charged: Vec<usize>,
    charges: Vec<u32>,
    /// Each pair's cheapest reading, when it is no word and not the word
    /// meant: what of its cost no table sets, and its charges.
    cheapest: Vec<Option<(f64, Vec<u32>)>>,
    unknown: f64,
    /// Pairs whose word no reading writes, by weight, and some of them.
    unwritten: f64,
    unwritten_pairs: Vec<String>,
    total: f64,
}

fn read(
    scheme: &Scheme,
    lexicon: &dyn Lexicon,
    splitter: &Splitter,
    pairs: &[Pair],
    costs: &[f32],
) -> Readings {
    let unknown = f64::from(lexicon.unknown_log_prob());
    let mut out = Readings {
        unknown,
        ..Readings::default()
    };
    out.starts.push(0);
    out.ways.push(0);
    out.charged.push(0);
    for (at, p) in pairs.iter().enumerate() {
        out.total += p.weight;
        // The words; the cheapest reading that is none is read apart.
        let mut readings: Vec<_> = scheme
            .readings(&p.latin, lexicon)
            .into_iter()
            .filter(|r| r.log_prob.is_some())
            .collect();
        let mut meant: Vec<bool> = readings
            .iter()
            .map(|r| splitter.normalize(&r.text) == p.word)
            .collect();
        if !meant.contains(&true) {
            let only = Only {
                word: &p.word,
                splitter,
            };
            for mut r in scheme.readings(&p.latin, &only) {
                if splitter.normalize(&r.text) == p.word {
                    r.log_prob = lexicon.log_prob(&r.text);
                    readings.push(r);
                    meant.push(true);
                }
            }
            if !meant.contains(&true) {
                out.unwritten += p.weight;
                out.unwritten_pairs.push(format!("{}\t{}", p.latin, p.word));
                continue;
            }
        }
        for (r, is_meant) in readings.iter().zip(meant) {
            out.likelihood.push(r.log_prob.map_or(unknown, f64::from));
            out.meant.push(is_meant);
            for way in &r.paths {
                let set: f64 = way
                    .charges
                    .iter()
                    .map(|&c| f64::from(costs[c as usize]))
                    .sum();
                out.fixed.push(f64::from(way.cost) - set);
                out.charges.extend_from_slice(&way.charges);
                out.charged.push(out.charges.len());
            }
            out.ways.push(out.fixed.len());
        }
        out.pair.push(at);
        out.weights.push(p.weight);
        out.starts.push(out.likelihood.len());
        out.cheapest.push(None);
    }
    out
}

/// Each pair's cheapest reading at `costs` (which `scheme` has).
fn read_cheapest(
    r: &mut Readings,
    scheme: &Scheme,
    lexicon: &dyn Lexicon,
    splitter: &Splitter,
    pairs: &[Pair],
    costs: &[f32],
) {
    for (k, &at) in r.pair.iter().enumerate() {
        let p = &pairs[at];
        let c = scheme.cheapest_reading(&p.latin);
        r.cheapest[k] =
            if lexicon.log_prob(&c.text).is_some() || splitter.normalize(&c.text) == p.word {
                // A word, which is among the words already; or the word meant,
                // whose cheapest way is.
                None
            } else {
                c.paths.into_iter().next().map(|way| {
                    let set: f64 = way
                        .charges
                        .iter()
                        .map(|&i| f64::from(costs[i as usize]))
                        .sum();
                    (f64::from(way.cost) - set, way.charges)
                })
            };
    }
}

/// The loss (the pairs' mean surprise at the word meant, and the prior's
/// penalty) and its gradient, at `costs`. A word costs the soft minimum
/// of its ways' costs (at `soft` nats; the keyboard takes the cheapest),
/// so that a cost raised on its cheapest way lets it be read another
/// way, as the keyboard would.
fn loss(r: &Readings, costs: &[f64], own: &[f64], args: &Args, grad: &mut [f64]) -> f64 {
    grad.fill(0.0);
    let (soft, sharp) = (args.soft, args.sharp);
    let total: f64 = r.weights.iter().sum();
    let charged = |ch: &[u32]| -> f64 { ch.iter().map(|&i| costs[i as usize]).sum() };
    let mut loss = 0.0;
    let mut scores: Vec<f64> = Vec::new();
    // A way's share of its word (its softmin weight).
    let mut share: Vec<f64> = Vec::new();
    for (p, &w) in r.weights.iter().enumerate() {
        let (lo, hi) = (r.starts[p], r.starts[p + 1]);
        scores.clear();
        share.clear();
        for c in lo..hi {
            let first = share.len();
            for way in r.ways[c]..r.ways[c + 1] {
                share.push(r.fixed[way] + charged(&r.charges[r.charged[way]..r.charged[way + 1]]));
            }
            let min = share[first..].iter().copied().fold(f64::INFINITY, f64::min);
            let mut z = 0.0;
            for x in &mut share[first..] {
                *x = (-(*x - min) / soft).exp();
                z += *x;
            }
            for x in &mut share[first..] {
                *x /= z;
            }
            scores.push(sharp * (r.likelihood[c] - (min - soft * z.ln())));
        }
        let cheapest = r.cheapest[p].as_ref();
        if let Some((fixed, ch)) = cheapest {
            scores.push(sharp * (r.unknown - fixed - charged(ch)));
        }
        let max = scores.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let mut all = 0.0;
        let mut meant = 0.0;
        for (k, s) in scores.iter_mut().enumerate() {
            *s = (*s - max).exp();
            all += *s;
            if lo + k < hi && r.meant[lo + k] {
                meant += *s;
            }
        }
        let w = w / total;
        loss += w * (all.ln() - meant.ln());
        // The gradient of the scores, times `sharp`.
        let w = w * sharp;
        let mut at = 0;
        for (k, &e) in scores[..hi - lo].iter().enumerate() {
            let c = lo + k;
            let of_meant = if r.meant[c] { e / meant } else { 0.0 };
            let coef = w * (of_meant - e / all);
            for way in r.ways[c]..r.ways[c + 1] {
                let g = coef * share[at];
                at += 1;
                if g != 0.0 {
                    for &i in &r.charges[r.charged[way]..r.charged[way + 1]] {
                        grad[i as usize] += g;
                    }
                }
            }
        }
        if let Some((_, ch)) = cheapest {
            let g = -w * scores[hi - lo] / all;
            for &i in ch {
                grad[i as usize] += g;
            }
        }
    }
    for ((g, &c), &o) in grad.iter_mut().zip(costs).zip(own) {
        loss += args.prior / 2.0 * (c - o) * (c - o);
        *g += args.prior * (c - o);
    }
    loss
}

/// Adam's steps down the loss, no cost below 0, the pairs' cheapest
/// readings read again (by `again`, at the costs reached) every
/// `--refresh` steps.
fn descend(
    r: &mut Readings,
    start: &[f32],
    own: &[f32],
    args: &Args,
    again: &mut dyn FnMut(&mut Readings, &[f32]) -> anyhow::Result<()>,
) -> anyhow::Result<(Vec<f32>, f64, f64)> {
    let own: Vec<f64> = own.iter().copied().map(f64::from).collect();
    let mut costs: Vec<f64> = start.iter().copied().map(f64::from).collect();
    #[allow(clippy::cast_possible_truncation)]
    let narrow = |c: &[f64]| -> Vec<f32> { c.iter().map(|&x| x as f32).collect() };
    let mut grad = vec![0.0; costs.len()];
    let mut m = vec![0.0; costs.len()];
    let mut v = vec![0.0; costs.len()];
    let (b1, b2) = (0.9_f64, 0.999_f64);
    again(r, start)?;
    let first = loss(r, &costs, &own, args, &mut grad);
    let mut last = first;
    for step in 1..=args.steps {
        if step > 1 && args.refresh > 0 && (step - 1) % args.refresh == 0 {
            again(r, &narrow(&costs))?;
        }
        last = loss(r, &costs, &own, args, &mut grad);
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let t = step as i32;
        for i in 0..costs.len() {
            m[i] = b1 * m[i] + (1.0 - b1) * grad[i];
            v[i] = b2 * v[i] + (1.0 - b2) * grad[i] * grad[i];
            let mh = m[i] / (1.0 - b1.powi(t));
            let vh = v[i] / (1.0 - b2.powi(t));
            costs[i] = (costs[i] - args.rate * mh / (vh.sqrt() + 1e-8)).max(0.0);
        }
    }
    Ok((narrow(&costs), first, last))
}

/// How often the word meant comes first, and among the strip's two, of
/// `pairs` by weight: as eval-translit measures it.
fn measure(
    scheme: &Scheme,
    lexicon: &dyn Lexicon,
    splitter: &Splitter,
    pairs: &[Pair],
) -> (f64, f64) {
    let (mut all, mut first, mut two) = (0.0, 0.0, 0.0);
    for p in pairs {
        let got = scheme.candidates(&p.latin, lexicon, 2);
        all += p.weight;
        if got
            .first()
            .is_some_and(|c| splitter.normalize(&c.text) == p.word)
        {
            first += p.weight;
        }
        if got.iter().any(|c| splitter.normalize(&c.text) == p.word) {
            two += p.weight;
        }
    }
    let pct = |a: f64| if all > 0.0 { 100.0 * a / all } else { 0.0 };
    (pct(first), pct(two))
}

/// `costs` rounded to `grain`, each as the table will read it written.
fn rounded(costs: &[f32], grain: f32) -> Vec<f32> {
    costs
        .iter()
        .map(|&c| {
            cost_text(((c / grain).round() * grain).max(0.0))
                .parse()
                .unwrap_or(c)
        })
        .collect()
}

/// Where the table writes each cost it sets: a way's token (its line and
/// byte range) in [`Scheme::costs`]'s order, then each setting's value,
/// when the table gives it.
struct Places {
    ways: Vec<(usize, std::ops::Range<usize>)>,
    labels: Vec<String>,
    settings: [Option<(usize, std::ops::Range<usize>)>; 4],
    /// The last line of the settings, where one the table lacks goes.
    last_setting: usize,
}

const SETTING_NAMES: [&str; 4] = ["cluster", "schwa", "final", "final-virama"];
const SETTING_DEFAULTS: [f32; 4] = [0.0, 2.0, 0.0, 3.0];

/// The whitespace-separated tokens of `code`, with their byte ranges.
fn tokens(code: &str) -> Vec<(std::ops::Range<usize>, &str)> {
    let mut out = Vec::new();
    let mut start = None;
    for (i, c) in code
        .char_indices()
        .chain(std::iter::once((code.len(), ' ')))
    {
        match (c.is_whitespace(), start) {
            (true, Some(s)) => {
                out.push((s..i, &code[s..i]));
                start = None;
            }
            (false, None) => start = Some(i),
            _ => {}
        }
    }
    out
}

/// A way's text without its cost, as `osk_data::translit` reads it.
fn body(way: &str) -> &str {
    match way.rsplit_once(':') {
        Some((body, _)) if !body.is_empty() => body,
        _ => way,
    }
}

fn places(table: &str) -> Places {
    let mut p = Places {
        ways: Vec::new(),
        labels: Vec::new(),
        settings: [None, None, None, None],
        last_setting: 0,
    };
    let mut in_rules = false;
    for (n, raw) in table.lines().enumerate() {
        let code = raw.split('#').next().unwrap_or("");
        let trimmed = code.trim().trim_start_matches('\u{feff}');
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_rules = true;
            continue;
        }
        let toks = tokens(code);
        let Some((_, head)) = toks.first() else {
            continue;
        };
        if in_rules {
            for (range, way) in &toks[1..] {
                p.ways.push((n, range.clone()));
                p.labels.push(format!(
                    "{} {}",
                    head.trim_start_matches('\u{feff}'),
                    body(way)
                ));
            }
        } else {
            p.last_setting = n;
            if let Some(k) = SETTING_NAMES.iter().position(|s| s == head)
                && let Some((range, _)) = toks.get(1)
            {
                p.settings[k] = Some((n, range.clone()));
            }
        }
    }
    p
}

/// A cost as a table writes it: no more digits than it needs.
fn cost_text(c: f32) -> String {
    let s = format!("{c:.2}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s.is_empty() { "0".into() } else { s.into() }
}

/// The table with `costs` in their places, its columns kept where a token
/// grows or shrinks.
fn rewrite(table: &str, costs: &[f32]) -> String {
    let p = places(table);
    let ways = p.ways.len();
    let mut edits: Vec<Vec<(std::ops::Range<usize>, String)>> =
        vec![Vec::new(); table.lines().count()];
    for (k, (line, range)) in p.ways.iter().enumerate() {
        let old = &table.lines().nth(*line).unwrap_or("")[range.clone()];
        let b = body(old);
        let c = costs[k];
        let new = if c > 0.0 {
            format!("{b}:{}", cost_text(c))
        } else {
            b.to_string()
        };
        edits[*line].push((range.clone(), new));
    }
    let mut missing = Vec::new();
    for (k, place) in p.settings.iter().enumerate() {
        let c = costs[ways + k];
        match place {
            Some((line, range)) => edits[*line].push((range.clone(), cost_text(c))),
            None if (c - SETTING_DEFAULTS[k]).abs() > f32::EPSILON => {
                missing.push(format!("{} {}", SETTING_NAMES[k], cost_text(c)));
            }
            None => {}
        }
    }
    let mut out = String::with_capacity(table.len() + 64);
    for (n, line) in table.lines().enumerate() {
        let mut text = line.to_string();
        let mut list = std::mem::take(&mut edits[n]);
        list.sort_by_key(|(r, _)| std::cmp::Reverse(r.start));
        for (range, new) in list {
            // Keep the next column where it was: take the difference out of
            // (or put it into) the spaces after the token, but for the two
            // between ways.
            let old_len = text[range.clone()].chars().count();
            let new_len = new.chars().count();
            let after = text[range.end..].chars().take_while(|c| *c == ' ').count();
            let more = text[range.end + after..].chars().next().is_some();
            let spaces = if more && after > 0 {
                (after + old_len).saturating_sub(new_len).max(after.min(2))
            } else {
                after
            };
            text.replace_range(
                range.start..range.end + after,
                &format!("{new}{}", " ".repeat(spaces)),
            );
        }
        out.push_str(&text);
        out.push('\n');
        if n == p.last_setting {
            for m in &missing {
                let _ = writeln!(out, "{m}");
            }
        }
    }
    if !table.ends_with('\n') {
        out.pop();
    }
    out
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let table = fs::read_to_string(&args.table)
        .with_context(|| format!("reading {}", args.table.display()))?;
    let mut scheme = Scheme::parse(&table).with_context(|| format!("{}", args.table.display()))?;
    let lexicon = FstLexicon::open(
        fs::read(&args.lexicon).with_context(|| format!("reading {}", args.lexicon.display()))?,
        UNKNOWN_BELOW_RAREST,
    )?;
    let lang = match &args.lang {
        Some(lang) => lang.clone(),
        None => args
            .table
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .with_context(|| format!("{}: no language in its name", args.table.display()))?,
    };
    let splitter = Splitter::new(&lang);
    let mut train = Vec::new();
    for path in &args.train {
        train.extend(load(path, &splitter)?);
    }
    let mut dev = Vec::new();
    for path in &args.dev {
        dev.extend(load(path, &splitter)?);
    }
    let own = scheme.costs();
    let p = places(&table);
    if p.ways.len() + 4 != own.len() {
        bail!(
            "{}: {} ways found to write, the table sets {}",
            args.table.display(),
            p.ways.len(),
            own.len() - 4
        );
    }
    let (f1, f2) = measure(&scheme, &lexicon, &splitter, &dev);
    println!(
        "{}: {} pairs to fit on, {} to choose by",
        args.table.display(),
        train.len(),
        dev.len()
    );
    println!("  the table's own costs: dev top-1 {f1:.2}%  top-2 {f2:.2}%");
    let mut best = (f1, f2, own.clone(), 0);
    let mut costs = own.clone();
    for round in 1..=args.rounds {
        scheme.set_costs(&costs)?;
        let mut readings = read(&scheme, &lexicon, &splitter, &train, &costs);
        if round == 1 {
            for line in readings.unwritten_pairs.iter().take(args.unwritten) {
                println!("    unwritten: {line}");
            }
        }
        let mut again = |r: &mut Readings, at: &[f32]| -> anyhow::Result<()> {
            let mut s = scheme.clone();
            s.set_costs(&at.iter().map(|c| c.max(0.0)).collect::<Vec<_>>())?;
            read_cheapest(r, &s, &lexicon, &splitter, &train, at);
            Ok(())
        };
        let (fitted, before, after) = descend(&mut readings, &costs, &own, &args, &mut again)?;
        costs = fitted;
        let kept = rounded(&costs, args.grain);
        scheme.set_costs(&kept)?;
        let (t1, t2) = measure(&scheme, &lexicon, &splitter, &dev);
        println!(
            "  round {round}: {:.1}% of the pairs' words unwritten; loss {before:.4} -> {after:.4}; dev top-1 {t1:.2}%  top-2 {t2:.2}%",
            100.0 * readings.unwritten / readings.total.max(1.0),
        );
        if dev.is_empty() || t1 > best.0 || (t1 == best.0 && t2 > best.1) {
            best = (t1, t2, kept, round);
        }
    }
    let (b1, b2, kept, round) = best;
    println!("  kept: round {round} (dev top-1 {b1:.2}%  top-2 {b2:.2}%)");
    let mut moved: Vec<(f32, String)> = kept
        .iter()
        .zip(&own)
        .enumerate()
        .filter(|(_, (k, o))| (*k - *o).abs() > f32::EPSILON)
        .map(|(i, (k, o))| {
            let label = p
                .labels
                .get(i)
                .cloned()
                .unwrap_or_else(|| SETTING_NAMES[i - p.labels.len()].to_string());
            (
                (k - o).abs(),
                format!("{label}: {} -> {}", cost_text(*o), cost_text(*k)),
            )
        })
        .collect();
    moved.sort_by(|a, b| b.0.total_cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    println!("  {} of {} costs moved; the most:", moved.len(), own.len());
    for (_, line) in moved.iter().take(args.show) {
        println!("    {line}");
    }
    let to = args
        .out
        .clone()
        .or_else(|| args.write.then(|| args.table.clone()));
    if let Some(to) = to.filter(|_| !moved.is_empty()) {
        let text = rewrite(&table, &kept);
        let check = Scheme::parse(&text).context("the rewritten table")?;
        if check.costs() != kept {
            bail!("the rewritten table does not read back as fitted");
        }
        fs::write(&to, text).with_context(|| format!("writing {}", to.display()))?;
        println!("  written to {}", to.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABLE: &str = "# a table\nscript abugida\nvirama ्\nschwa 1.5\n\n[consonants]\nt    त  ट:0.5   # two\nk    क\n\n[vowels]\na    अ/-  आ/ा:1\n";

    #[test]
    fn costs_are_written_in_their_places() {
        let scheme = Scheme::parse(TABLE).unwrap();
        let mut costs = scheme.costs();
        // t: त ट, k: क, a: अ आ; then cluster, schwa, final, final-virama.
        assert_eq!(costs.len(), 5 + 4);
        costs[0] = 0.3;
        costs[1] = 0.0;
        costs[3] = 0.25;
        costs[4] = 1.4;
        costs[6] = 2.0;
        costs[8] = 3.5;
        let text = rewrite(TABLE, &costs);
        assert_eq!(
            text,
            "# a table\nscript abugida\nvirama ्\nschwa 2\nfinal-virama 3.5\n\n[consonants]\nt    त:0.3  ट       # two\nk    क\n\n[vowels]\na    अ/-:0.25  आ/ा:1.4\n"
        );
        assert_eq!(Scheme::parse(&text).unwrap().costs(), costs);
    }
}
