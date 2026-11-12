#!/usr/bin/env python3
"""The release version: one number for the format crate and the data.

    version.py get          print it
    version.py next PART    print what bumping PART would give
    version.py bump PART    raise it everywhere it is spelled out
    version.py check        a released version names one content: the
                            version is not below the latest release, and
                            if it is released, the tree changes nothing a
                            release contains (what ci.yml runs)

It is the workspace version in Cargo.toml. crates/osk-data/Cargo.toml
spells it out again, because other workspaces build that crate and must
not inherit this one's keys, and Cargo.lock records both packages. `bump`
changes all three and refuses if they disagree beforehand.

PART is `patch` when the format and the crate's API stay as they are (new
data, as the monthly refresh brings; fixes; documentation), `minor` when
either changes or the set of packs does, and `major` once the format is
stable. A release is tagged
v<version> (release.yml), and a released version never names other
content (ci.yml).
"""

import re
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
WORKSPACE = ROOT / "Cargo.toml"
CRATE = ROOT / "crates/osk-data/Cargo.toml"
LOCK = ROOT / "Cargo.lock"
SEMVER = re.compile(r"(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)")
PARTS = ("patch", "minor", "major")

# What a release contains: the data, the pack definitions, the notices and
# the format crate's source. Changing any of it needs a new version.
RELEASED = (
    "predict", "wordbreak", "pinyin", "zhuyin", "jyutping", "kana", "rules",
    "layouts", "emoji", "offensive",
    "packs.toml", "NOTICES.md", "crates/osk-data",
)

# A `version = "…"` key at the start of a line: in Cargo.toml only the
# package version is spelled that way (dependencies carry theirs inline).
KEY = re.compile(r'^(version\s*=\s*")([^"]+)(")', re.M)
# The two packages of this workspace in Cargo.lock.
LOCKED = re.compile(
    r'^(\[\[package\]\]\nname = "(?:osk-data|osk-data-tools)"\nversion = ")([^"]+)(")',
    re.M,
)


def fail(msg: str) -> "NoReturn":  # noqa: F821
    sys.exit(f"version.py: {msg}")


def get() -> str:
    ws = tomllib.loads(WORKSPACE.read_text())["workspace"]["package"]["version"]
    crate = tomllib.loads(CRATE.read_text())["package"]["version"]
    if ws != crate:
        fail(f"Cargo.toml says {ws} and crates/osk-data/Cargo.toml says {crate}; they must agree")
    if not SEMVER.fullmatch(ws):
        fail(f"{ws!r} is not MAJOR.MINOR.PATCH")
    locked = [m.group(2) for m in LOCKED.finditer(LOCK.read_text())]
    if locked != [ws, ws]:
        fail(f"Cargo.lock records {locked} for osk-data and osk-data-tools, not {ws}")
    return ws


def next_version(version: str, part: str) -> str:
    major, minor, patch = (int(n) for n in version.split("."))
    if part == "patch":
        return f"{major}.{minor}.{patch + 1}"
    if part == "minor":
        return f"{major}.{minor + 1}.0"
    if part == "major":
        return f"{major + 1}.0.0"
    fail(f"PART is one of {', '.join(PARTS)}, not {part!r}")


def rewrite(path: Path, pattern: re.Pattern, old: str, new: str, count: int) -> None:
    text = path.read_text()
    found = [m.group(2) for m in pattern.finditer(text)]
    if found != [old] * count:
        fail(f"{path.relative_to(ROOT)}: expected {count} version(s) {old}, found {found}")
    path.write_text(pattern.sub(lambda m: m.group(1) + new + m.group(3), text))


def bump(part: str) -> str:
    old = get()
    new = next_version(old, part)
    rewrite(WORKSPACE, KEY, old, new, 1)
    rewrite(CRATE, KEY, old, new, 1)
    rewrite(LOCK, LOCKED, old, new, 2)
    if get() != new:
        fail("the bump did not take")
    return new


def git(*args: str) -> str:
    return subprocess.run(
        ["git", *args], cwd=ROOT, check=True, capture_output=True, text=True
    ).stdout


def semver_key(tag: str) -> tuple[int, int, int]:
    major, minor, patch = (int(n) for n in tag.removeprefix("v").split("."))
    return major, minor, patch


def check() -> None:
    version = get()
    tag = f"v{version}"
    tags = [t for t in git("tag", "--list", "v*").split() if SEMVER.fullmatch(t[1:])]
    latest = max(tags, key=semver_key, default=None)
    if latest and semver_key(tag) < semver_key(latest):
        fail(f"version {version} is below the latest release, {latest}")
    if tag not in tags:
        print(f"{tag} is not released yet: merging it to the default branch releases it")
        return
    diff = git("diff", "--stat", tag, "--", *RELEASED)
    if diff.strip():
        print(diff, end="")
        fail(
            f"{tag} is released, and this changes what it contains. Bump the version: "
            "`make bump-version` (PART=patch unless the format, the crate's API or the "
            "set of packs changed)"
        )
    print(f"{tag} is released, and nothing here changes what it contains")


def main(argv: list[str]) -> int:
    match argv:
        case ["get"]:
            print(get())
        case ["next", part]:
            print(next_version(get(), part))
        case ["bump", part]:
            old = get()
            print(f"{old} -> {bump(part)}")
        case ["check"]:
            check()
        case _:
            print(__doc__.strip(), file=sys.stderr)
            return 2
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
