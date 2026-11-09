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
#   kaikki-<lang>.jsonl.gz Wiktionary's entries for Portuguese and Spanish,
#                          as kaikki.org extracts them: Brazil's and
#                          Portugal's, Spain's and Latin America's
#                          spellings and words   → pt, es.dialects.bin
#
# ESDB and Harper are pinned: ESDB to a commit of its repository, Harper
# to a release of its crate, so the same build reads the same words.
# kaikki.org keeps only its latest extracts, rebuilt from each Wiktionary
# dump, so those are not: a cached copy the server says is current is
# kept, as fetch-corpora.sh keeps Tatoeba's exports. `sources.lock`
# records the sha256 of each file a table was built from. Licences:
# NOTICES.md (ESDB's is permissive, Harper's Apache-2.0, Wiktionary's CC
# BY-SA 4.0).
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

# Download $1 to $2 unless the cached $2 is current, keeping it then.
# Returns non-zero unless $2 ends up a valid gzip file.
get_current() {  # url dest
	local url=$1 dest=$2 part=$2.part
	rm -f "$part"
	if [ -s "$dest" ] && ! gzip -t "$dest" 2>/dev/null; then
		echo "   discarding invalid cached $dest" >&2
		rm -f "$dest"
	fi
	if [ -s "$dest" ]; then
		# -z: only transfer if the server's copy is newer than ours (-R
		# keeps the server's mtime on what we save).
		curl -fsSL -R --retry 5 --retry-all-errors --max-time 1800 \
			-z "$dest" -o "$part" "$url" || { rm -f "$part"; return 1; }
	else
		curl -fsSL -R --retry 5 --retry-all-errors --max-time 1800 \
			-o "$part" "$url" || { rm -f "$part"; return 1; }
	fi
	if [ -s "$part" ]; then
		gzip -t "$part" 2>/dev/null || {
			echo "   $url: response is not a valid gzip stream" >&2
			rm -f "$part"; return 1; }
		mv -f "$part" "$dest"
	fi
	rm -f "$part"
	[ -s "$dest" ]
}

need_entries() {  # file min
	local n
	n=$(gzip -dc "$1" | wc -l)
	[ "$n" -ge "$2" ] || { echo "!! $1: $n entries, expected at least $2" >&2; exit 1; }
	printf '  %-30s %9d entries\n' "${1##*/}" "$n"
}

KAIKKI=https://kaikki.org/dictionary
for pair in pt:Portuguese es:Spanish; do
	lang=${pair%%:*} name=${pair#*:}
	get_current "$KAIKKI/$name/kaikki.org-dictionary-$name.jsonl.gz" "$OUT/kaikki-$lang.jsonl.gz" \
		|| { echo "!! download failed: $name" >&2; exit 1; }
	need_entries "$OUT/kaikki-$lang.jsonl.gz" 200000
done
