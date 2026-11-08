#!/usr/bin/env python3
"""Maintain `sources.lock`: what every committed output was built from.

Each entry names one output group (`predict/en`, `pinyin`, `kana/mozc-kanji`,
...) and records three things:

  inputs   sha256 of every input file, by a stable name
           (`corpora/en.txt`, `cedict.txt`, ...)
  recipe   the tuning values passed to the builder (vocab caps, epochs, ...)
  outputs  sha256 of every file the build wrote

plus, for information only, the commit of this repository whose builders
encoded it.

An entry is FRESH when its inputs and recipe match what would be used now
AND its outputs on disk still hash to what was recorded. Anything else is
stale and gets rebuilt by `make <target> ONLY_STALE=1`. That is what keeps
the scheduled refresh from rewriting files every month for nothing: the
committed n-gram tables predate deterministic builds and cannot be
rebuilt byte for byte, so "rebuild and see if it changed" would say yes
for inputs that never moved.

The outputs check also catches a file rebuilt by hand outside the recipe
— a seed table standing in for a full one, say: its hash no longer
matches, so the next refresh rebuilds it the recorded way.

Subcommands (all take --lock, default sources.lock):

  fresh  NAME [--input N=PATH ...] [--recipe K=V ...] [--output PATH ...]
         exit 0 if fresh, 1 if stale (the reason goes to stdout)
  record NAME [same args] [--encoder REV]
         write the entry for NAME from the files as they are now
  verify [--inputs PREFIX=DIR ...]
         check every entry's outputs (and optionally inputs) against disk
  stamp  --tag TAG [--run URL]
         note which refresh built this lock's outputs: the release they
         were built for, and the workflow run (provenance; the release
         workflow finds a refresh's draft by the version, v<version>)
  get    KEY
         print a top-level `refresh` field (tag, run), empty if unset

Writers (record, stamp) hold an exclusive flock on the lock file's
directory for their read-modify-write, because `make -j` runs several
`record`s at once; without it, concurrent writers lost each other's
entries.

Standard library (POSIX) only.
"""

from __future__ import annotations

import argparse
import contextlib
import fcntl
import hashlib
import json
import os
import sys
import tempfile
from pathlib import Path

LOCK_VERSION = 1


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def load(path: Path) -> dict:
    if not path.exists():
        return {"version": LOCK_VERSION, "outputs": {}}
    data = json.loads(path.read_text(encoding="utf-8"))
    if data.get("version") != LOCK_VERSION:
        sys.exit(f"{path}: unsupported lock version {data.get('version')!r}")
    data.setdefault("outputs", {})
    return data


@contextlib.contextmanager
def exclusive(lock: Path):
    """Serialise read-modify-write of `lock` across processes. Locks the
    directory, not the file: save() replaces the file, so a lock held on
    the old inode would not exclude a writer that opened the new one."""
    fd = os.open(lock.parent, os.O_RDONLY)
    try:
        fcntl.flock(fd, fcntl.LOCK_EX)
        yield
    finally:
        os.close(fd)  # releases the flock


def save(path: Path, data: dict) -> None:
    text = json.dumps(data, indent=2, sort_keys=True, ensure_ascii=False) + "\n"
    # Atomic replace: an interrupted write must not leave a truncated
    # lock that the next run would read as "everything is stale".
    fd, tmp = tempfile.mkstemp(dir=path.parent or ".", prefix=".lock-")
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as f:
            f.write(text)
        os.replace(tmp, path)
    except BaseException:
        os.unlink(tmp)
        raise


def pairs(items: list[str], what: str) -> dict[str, str]:
    out = {}
    for item in items:
        key, sep, val = item.partition("=")
        if not sep or not key:
            sys.exit(f"--{what} wants NAME=VALUE, got {item!r}")
        out[key] = val
    return out


def current(args) -> tuple[dict, dict]:
    inputs = {}
    for name, path in pairs(args.input, "input").items():
        p = Path(path)
        if not p.is_file():
            sys.exit(f"{args.name}: input {name} not found at {path}")
        inputs[name] = sha256(p)
    return inputs, pairs(args.recipe, "recipe")


def cmd_fresh(args) -> int:
    entry = load(args.lock)["outputs"].get(args.name)
    if entry is None:
        print(f"{args.name}: stale (not in {args.lock})")
        return 1
    inputs, recipe = current(args)
    if entry.get("inputs") != inputs:
        changed = sorted(
            k for k in set(inputs) | set(entry.get("inputs", {}))
            if inputs.get(k) != entry.get("inputs", {}).get(k)
        )
        print(f"{args.name}: stale (inputs changed: {', '.join(changed)})")
        return 1
    if entry.get("recipe") != recipe:
        print(f"{args.name}: stale (recipe changed: {entry.get('recipe')} -> {recipe})")
        return 1
    recorded = entry.get("outputs", {})
    if sorted(recorded) != sorted(args.output):
        print(f"{args.name}: stale (output set changed)")
        return 1
    for out in args.output:
        p = Path(out)
        if not p.is_file():
            print(f"{args.name}: stale ({out} missing)")
            return 1
        if sha256(p) != recorded[out]:
            print(f"{args.name}: stale ({out} differs from what the lock recorded)")
            return 1
    print(f"{args.name}: up to date")
    return 0


def cmd_record(args) -> int:
    inputs, recipe = current(args)
    outputs = {}
    for out in args.output:
        p = Path(out)
        if not p.is_file():
            sys.exit(f"{args.name}: output {out} was not written")
        outputs[out] = sha256(p)
    entry = {"inputs": inputs, "recipe": recipe, "outputs": outputs}
    if args.encoder:
        entry["encoder"] = args.encoder
    with exclusive(args.lock):
        data = load(args.lock)
        data["outputs"][args.name] = entry
        save(args.lock, data)
    return 0


def cmd_verify(args) -> int:
    data = load(args.lock)
    roots = pairs(args.inputs, "inputs")
    bad = []
    for name, entry in sorted(data["outputs"].items()):
        for out, digest in sorted(entry.get("outputs", {}).items()):
            p = Path(out)
            if not p.is_file():
                bad.append(f"{name}: {out} is missing")
            elif sha256(p) != digest:
                bad.append(f"{name}: {out} does not match the lock")
        for prefix, root in roots.items():
            for inp, digest in sorted(entry.get("inputs", {}).items()):
                if not inp.startswith(prefix):
                    continue
                p = Path(root) / inp[len(prefix):]
                if not p.is_file():
                    bad.append(f"{name}: input {inp} not found under {root}")
                elif sha256(p) != digest:
                    bad.append(f"{name}: input {inp} under {root} does not match the lock")
    for line in bad:
        print(line)
    if bad:
        print(f"{len(bad)} problem(s) against {args.lock}")
        return 1
    print(f"{args.lock}: {len(data['outputs'])} entries match the tree")
    return 0


def cmd_stamp(args) -> int:
    with exclusive(args.lock):
        data = load(args.lock)
        data["refresh"] = {"tag": args.tag, **({"run": args.run} if args.run else {})}
        save(args.lock, data)
    return 0


def cmd_get(args) -> int:
    print(load(args.lock).get("refresh", {}).get(args.key, ""))
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--lock", type=Path, default=Path("sources.lock"))
    sub = ap.add_subparsers(dest="cmd", required=True)

    for name in ("fresh", "record"):
        p = sub.add_parser(name)
        p.add_argument("name")
        p.add_argument("--input", action="append", default=[], metavar="NAME=PATH")
        p.add_argument("--recipe", action="append", default=[], metavar="KEY=VALUE")
        p.add_argument("--output", action="append", default=[], metavar="PATH")
        if name == "record":
            p.add_argument("--encoder", default="")

    p = sub.add_parser("verify")
    p.add_argument("--inputs", action="append", default=[], metavar="PREFIX=DIR",
                   help="also check inputs named PREFIX... against files in DIR")

    p = sub.add_parser("stamp")
    p.add_argument("--tag", required=True)
    p.add_argument("--run", default="")

    p = sub.add_parser("get")
    p.add_argument("key", choices=["tag", "run"])

    args = ap.parse_args()
    return {
        "fresh": cmd_fresh, "record": cmd_record, "verify": cmd_verify,
        "stamp": cmd_stamp, "get": cmd_get,
    }[args.cmd](args)


if __name__ == "__main__":
    sys.exit(main())
