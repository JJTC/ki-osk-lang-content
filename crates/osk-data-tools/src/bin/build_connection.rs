//! `build-connection` — pack mozc's connection-cost matrix into the runtime
//! bundle the kana→kanji lattice reads.
//!
//! Input is mozc's `connection_single_column.txt`: a header line with the
//! matrix dimension `N` (the POS-id count, e.g. 2672), then `N*N` integer
//! costs, one per line, row-major `cost[rid * N + lid]` — the transition
//! cost from a left word's `rid` to a right word's `lid`. Lower is more
//! grammatical; ~16383 is mozc's "very unlikely".
//!
//! As raw `i16` the matrix is ~14 MB; it compresses to ~1.9 MB (many
//! repeated per-row values), so the bundle is a zlib-deflated flat `i16`
//! array plus its dimension — the [`ConnectionBlob`] the runtime inflates.
//!
//! Usage:
//!   build-connection --matrix connection_single_column.txt \
//!                    --out kana/

use std::fs;
use std::io::Write as _;
use std::path::PathBuf;

use anyhow::Context as _;
use clap::Parser;
use flate2::{Compression, write::ZlibEncoder};
use osk_data::kana::ConnectionBlob;

#[derive(Parser)]
#[command(name = "build-connection")]
struct Args {
    /// mozc `connection_single_column.txt`.
    #[arg(long)]
    matrix: PathBuf,
    /// Output directory. File written as `<out>/mozc-connection.bin`.
    #[arg(long)]
    out: PathBuf,
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Args::parse();

    let text = fs::read_to_string(&args.matrix)
        .with_context(|| format!("reading {}", args.matrix.display()))?;
    let mut it = text.split_ascii_whitespace();
    let n: usize = it
        .next()
        .context("empty matrix file")?
        .parse()
        .context("bad matrix dimension header")?;
    log::info!("build-connection: matrix dimension N = {n}");

    let mut costs: Vec<i16> = Vec::with_capacity(n * n);
    for tok in it {
        // mozc costs are non-negative and fit i16 (max ~16383); clamp
        // defensively so a stray large value can't wrap.
        let v: i32 = tok.parse().context("non-integer connection cost")?;
        costs.push(v.clamp(0, i32::from(i16::MAX)) as i16);
    }
    anyhow::ensure!(
        costs.len() == n * n,
        "expected {} costs, got {}",
        n * n,
        costs.len()
    );

    // Flatten to little-endian i16, then zlib-deflate.
    let mut flat = Vec::with_capacity(costs.len() * 2);
    for c in &costs {
        flat.extend_from_slice(&c.to_le_bytes());
    }
    let deflated = {
        let mut e = ZlibEncoder::new(Vec::new(), Compression::best());
        e.write_all(&flat).context("deflate")?;
        e.finish().context("deflate finish")?
    };
    log::info!(
        "build-connection: {} → {} bytes deflated ({:.1}%)",
        flat.len(),
        deflated.len(),
        100.0 * deflated.len() as f64 / flat.len() as f64,
    );

    let blob = ConnectionBlob {
        n: u16::try_from(n).context("POS-id count exceeds u16")?,
        deflated,
    };
    fs::create_dir_all(&args.out).with_context(|| format!("mkdir -p {}", args.out.display()))?;
    let out_path = args.out.join("mozc-connection.bin");
    let bytes = osk_data::codec::encode(&blob).context("encode connection blob")?;
    fs::write(&out_path, &bytes).with_context(|| format!("writing {}", out_path.display()))?;
    log::info!("wrote {} ({} bytes)", out_path.display(), bytes.len());
    Ok(())
}
