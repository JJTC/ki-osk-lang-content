#!/usr/bin/env bash
# fetch-dialect-sources.sh — download the datasets the dialect flags
# (predict/<lang>.dialects.bin) are built from, into $OUT (default
# /tmp/ki-osk-sources).
#
#   esdb-scowl-pre.txt     English Speller Database (SCOWLv2): spellings by
#                          variety, American, British -ise and -ize,
#                          Canadian, Australian         → en.dialects.bin
#   harper-dictionary.dict Harper's English dictionary, from the
#                          harper-core crate: Indian English words
#                                                       → en.dialects.bin
#
# Both are pinned: ESDB to a commit of its repository, Harper to a release
# of its crate, so the same build reads the same words. `sources.lock`
# records each file's sha256. Licences: NOTICES.md (ESDB's is permissive,
# Harper's Apache-2.0).
set -eu

OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources}

ESDB_COMMIT=c107d90a2a36be118306be8107e4f62a5132d7ad
ESDB_URL=https://raw.githubusercontent.com/en-wl/wordlist/$ESDB_COMMIT/data/scowl-pre.txt
HARPER_VERSION=3.0.0
HARPER_URL=https://static.crates.io/crates/harper-core/harper-core-$HARPER_VERSION.crate

mkdir -p "$OUT"

get() {  # url dest
	curl -fsSL --retry 5 --retry-all-errors --max-time 600 -o "$2.part" "$1" \
		|| { rm -f "$2.part"; echo "!! download failed: $1" >&2; exit 1; }
}

# Floors against error pages and truncation, not exact expectations.
need_lines() {  # file min
	local n
	n=$(wc -l < "$1")
	[ "$n" -ge "$2" ] || { echo "!! $1: $n lines, expected at least $2" >&2; exit 1; }
	printf '  %-30s %9d lines\n' "${1##*/}" "$n"
}

get "$ESDB_URL" "$OUT/esdb-scowl-pre.txt"
mv "$OUT/esdb-scowl-pre.txt.part" "$OUT/esdb-scowl-pre.txt"
need_lines "$OUT/esdb-scowl-pre.txt" 400000

get "$HARPER_URL" "$OUT/harper-core.crate"
tar -xzf "$OUT/harper-core.crate.part" -O "harper-core-$HARPER_VERSION/dictionary.dict" \
	> "$OUT/harper-dictionary.dict.part"
rm -f "$OUT/harper-core.crate.part"
mv "$OUT/harper-dictionary.dict.part" "$OUT/harper-dictionary.dict"
need_lines "$OUT/harper-dictionary.dict" 50000
