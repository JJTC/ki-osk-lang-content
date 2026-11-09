#!/usr/bin/env python3
"""Gate a rebuilt tree against a git revision before it is committed.

Compares every output file in the working tree with the same file at
--base (default HEAD) and FAILS when:

  - a file that exists at --base is gone;
  - a `.bin` lost its `KIOS` header, or an `.fst` is implausibly small;
  - a file shrank by more than --max-shrink (default 10%) — the shape of
    every quiet downgrade this repository has had: seed dictionaries
    replacing full ones, an empty corpus, a truncated download.

Growth is never a failure. Everything is written to --report as Markdown
(the refresh workflow puts it in the PR body), failures included.

Standard library only; needs `git` on PATH.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

GLOBS = [
    "predict/*.fst",
    "predict/*.ngram.bin",
    "pinyin/*.bin",
    "zhuyin/*.bin",
    "jyutping/*.bin",
    "kana/*.bin",
]
DIRS = ["predict", "pinyin", "zhuyin", "jyutping", "kana"]


def git(*args: str, data: bytes | None = None) -> bytes:
    return subprocess.run(["git", *args], input=data, check=True,
                          capture_output=True).stdout


def is_output(path: str) -> bool:
    return any(Path(path).match(g) for g in GLOBS) and not path.startswith("predict/corpora/")


def base_files(rev: str) -> dict[str, tuple[str, int]]:
    out = {}
    for line in git("ls-tree", "-r", "--long", rev, "--", *DIRS).decode().splitlines():
        meta, path = line.split("\t", 1)
        _mode, kind, sha, size = meta.split()
        if kind == "blob" and is_output(path):
            out[path] = (sha, int(size))
    return out


def work_files() -> list[str]:
    found = set()
    for g in GLOBS:
        found.update(str(p) for p in Path(".").glob(g) if p.is_file())
    return sorted(p for p in found if is_output(p))


def blob_ids(paths: list[str]) -> dict[str, str]:
    if not paths:
        return {}
    ids = git("hash-object", "--stdin-paths", data="\n".join(paths).encode()).decode().split()
    return dict(zip(paths, ids))


def header_problem(path: str) -> str | None:
    p = Path(path)
    with open(p, "rb") as f:
        head = f.read(16)
    if path.endswith(".bin"):
        if head[:4] != b"KIOS":
            return "no KIOS header"
    elif path.endswith(".fst"):
        # A lexicon this small is a broken build; a word-forms table is
        # empty where the script has no case.
        if p.stat().st_size < 64 and not p.name.endswith(".forms.fst"):
            return "implausibly small FST"
    return None


def human(n: int) -> str:
    if n >= 1 << 20:
        return f"{n / (1 << 20):.2f} MB"
    if n >= 1 << 10:
        return f"{n / (1 << 10):.1f} KB"
    return f"{n} B"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--base", default="HEAD")
    ap.add_argument("--max-shrink", type=float, default=0.10)
    ap.add_argument("--report", type=Path)
    args = ap.parse_args()

    base = base_files(args.base)
    work = work_files()
    ids = blob_ids(work)

    failures: list[str] = []
    rows: list[str] = []
    unchanged = 0

    for path in sorted(set(base) | set(work)):
        if path not in ids:
            failures.append(f"`{path}` exists at {args.base} but is missing")
            continue
        problem = header_problem(path)
        if problem:
            failures.append(f"`{path}`: {problem}")
        size = Path(path).stat().st_size
        if path not in base:
            rows.append(f"| `{path}` | — | {human(size)} | new |")
            continue
        old_sha, old_size = base[path]
        if ids[path] == old_sha:
            unchanged += 1
            continue
        change = (size - old_size) / old_size if old_size else 0.0
        mark = ""
        if change < -args.max_shrink:
            failures.append(
                f"`{path}` shrank {-change:.1%} ({human(old_size)} → {human(size)}), "
                f"more than the {args.max_shrink:.0%} allowed")
            mark = " ❌"
        rows.append(f"| `{path}` | {human(old_size)} | {human(size)} | {change:+.1%}{mark} |")

    out = ["## Language content changes", ""]
    if failures:
        out += ["**Checks failed:**", ""] + [f"- {f}" for f in failures] + [""]
    if rows:
        out += ["| Output | Before | After | Change |", "|---|--:|--:|--:|", *rows, ""]
    else:
        out += ["No output changed.", ""]
    out.append(f"{unchanged} output file(s) unchanged.")
    report = "\n".join(out) + "\n"

    if args.report:
        args.report.write_text(report, encoding="utf-8")
    print(report)
    if failures:
        print(f"check-assets: {len(failures)} failure(s)", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
