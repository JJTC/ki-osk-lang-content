#!/usr/bin/env bash
# fetch-wiktionary-romanized.sh — words and their romanizations, from
# Wiktionary as kaikki.org extracts it, to measure the transliteration
# tables of languages with no romanizations people typed in open data:
# Yiddish, whose Wiktionary romanization (YIVO's) is the one people type
# (sholem: שלום), and Armenian, Persian, Pashto, Mongolian, Santali and Lao,
# whose scholarly romanizations scripts/wiktionary-romanized.py folds to
# the letters their tables read (Armenian's ġ is gh, Persian's x kh). Only
# `make eval-translit` reads them: nothing of them is committed or shipped
# (Wiktionary is CC BY-SA 4.0).
#
#   $OUT/<lang>.tsv    the word, its romanization and 1, tab-separated,
#                      a pair a line, each pair once: what eval-translit
#                      --pairs reads
#
# Which romanizations, and how they are folded, is the options column
# below (scripts/wiktionary-romanized.py says what each does). kaikki.org
# keeps only its latest extract, rebuilt from each Wiktionary dump: a kept
# copy is read again unless REFRESH=1, and the sha256 of the one read is
# printed, as the measure moves with it.
#
#   scripts/fetch-wiktionary-romanized.sh            # every language
#   scripts/fetch-wiktionary-romanized.sh yi fa      # some
#
# Env: OUT=$TMPDIR/ki-osk-sources/wiktionary  REFRESH=
set -euo pipefail

KAIKKI=https://kaikki.org/dictionary
# language  kaikki.org's name  wiktionary-romanized.py's options
LANGS="
yi   Yiddish    -
hy   Armenian   --fold hy --headword --script ARMENIAN
fa   Persian    --fold fa --headword --last --script ARABIC
ps   Pashto     --fold ps --headword --script ARABIC
mn   Mongolian  --fold mn --headword --script CYRILLIC
sat  Santali    --fold sat --headword --script OL_CHIKI
lo   Lao        --fold lo --headword --script LAO
"

HERE=$(cd "$(dirname "$0")" && pwd)
OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/wiktionary}
langs=${*:-$(echo "$LANGS" | awk 'NF {print $1}')}
mkdir -p "$OUT/raw"
for lang in $langs; do
	row=$(echo "$LANGS" | awk -v l="$lang" '$1 == l')
	[ -n "$row" ] || { echo "fetch-wiktionary-romanized: no $lang" >&2; exit 2; }
	read -r _ name options <<<"$row"
	[ "$options" != - ] || options=
	file="$OUT/raw/kaikki-$lang.jsonl.gz"
	if [ ! -f "$file" ] || [ -n "${REFRESH:-}" ]; then
		curl -fsSL --retry 5 --retry-all-errors -o "$file.part" \
			"$KAIKKI/$name/kaikki.org-dictionary-$name.jsonl.gz"
		mv -f "$file.part" "$file"
	fi
	# shellcheck disable=SC2086 # the options are words
	python3 -I "$HERE/wiktionary-romanized.py" $options "$file" > "$OUT/$lang.tsv.part"
	mv -f "$OUT/$lang.tsv.part" "$OUT/$lang.tsv"
	echo "$OUT/$lang.tsv: $(wc -l < "$OUT/$lang.tsv") pairs, from $(sha256sum "$file" | cut -c1-16)…"
done
