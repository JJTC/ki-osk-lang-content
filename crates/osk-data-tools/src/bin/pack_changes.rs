//! `pack-changes` — what a release holds and what it changed, pack by
//! pack, as Markdown for its release notes.
//!
//! Usage:
//!   pack-changes [--old OLD/packs.json] NEW/packs.json
//!
//! With `--old`, the previous release's index (of any schema: only its
//! packs are read, [`osk_data::pack::manifests`]), it lists the packs whose
//! data changed, was added or was removed ([`osk_data::pack::changes`]).
//! Without it — a first release — it lists every pack.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context as _;
use clap::Parser;
use osk_data::pack::{self, Index, Manifest};

#[derive(Parser)]
#[command(name = "pack-changes")]
struct Args {
    /// The previous release's packs.json.
    #[arg(long)]
    old: Option<PathBuf>,
    /// This release's packs.json.
    new: PathBuf,
}

fn load(path: &Path) -> anyhow::Result<Index> {
    let text = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    Index::parse(&text).with_context(|| format!("parsing {}", path.display()))
}

#[allow(clippy::cast_precision_loss)]
fn mib(bytes: u64) -> String {
    format!("{:.1} MiB", bytes as f64 / 1_048_576.0)
}

fn licences(m: &Manifest) -> String {
    let names: Vec<String> = m
        .licenses
        .iter()
        .map(|l| {
            if l.share_alike {
                format!("{} (share-alike)", l.name)
            } else {
                l.name.clone()
            }
        })
        .collect();
    names.join(", ")
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let new = load(&args.new)?;
    let size: u64 = new.packs.iter().map(Manifest::size).sum();
    let served: u64 = new.assets.values().map(|d| d.size).sum();
    println!(
        "**{} packs**: {} installed, {} to download.",
        new.packs.len(),
        mib(size),
        mib(served)
    );
    println!();

    let Some(old) = &args.old else {
        println!("| Pack | Name | Size | Licences |");
        println!("|---|---|---|---|");
        for m in &new.packs {
            println!(
                "| `{}` | {} | {} | {} |",
                m.id,
                m.name,
                mib(m.size()),
                licences(m)
            );
        }
        return Ok(());
    };
    // Its packs alone: an index of an older schema has the same manifests.
    let old = fs::read_to_string(old).with_context(|| format!("reading {}", old.display()))?;
    let old = pack::manifests(&old).context("reading the previous release's packs")?;
    let old_version = old.first().map_or("?", |m| m.version.as_str());
    let changes = pack::changes(&old, &new.packs);
    if changes.is_empty() {
        println!("No pack's data changed since v{old_version}.");
        return Ok(());
    }
    let (mut changed, mut added, mut removed) = (0, 0, 0);
    let mut rows = Vec::new();
    for c in &changes {
        let name = new
            .pack(&c.id)
            .or_else(|| old.iter().find(|m| m.id == c.id))
            .map_or("", |m| m.name.as_str());
        let (what, sizes) = match (c.before, c.after) {
            (None, Some(after)) => {
                added += 1;
                ("added", mib(after))
            }
            (Some(before), None) => {
                removed += 1;
                ("removed", mib(before))
            }
            (Some(before), Some(after)) => {
                changed += 1;
                ("changed", format!("{} → {}", mib(before), mib(after)))
            }
            (None, None) => unreachable!("a change has a before or an after"),
        };
        rows.push(format!("| `{}` | {name} | {what} | {sizes} |", c.id));
    }
    println!("Since v{old_version}: {changed} changed, {added} added, {removed} removed.");
    println!();
    println!("| Pack | Name | | Size |");
    println!("|---|---|---|---|");
    for row in rows {
        println!("{row}");
    }
    Ok(())
}
