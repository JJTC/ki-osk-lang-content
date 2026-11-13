#!/usr/bin/env bash
# fetch-wikipedia.sh — the text of a language's Wikipedia, for the
# languages whose models learn from it alone (the Makefile's
# WIKIPEDIA_LANGS): Bhojpuri, Maithili, Mongolian, Nepali, Odia, Western
# Punjabi and Pashto.
#
# The text comes from each wiki's CirrusSearch dump (Wikimedia's
# cirrus_search_index, an article a JSON document whose "text" is its
# plain text, no markup), of one date, each file pinned by its sha256
# below; Wikipedia's text is CC BY-SA 4.0. scripts/wikipedia-text.py cuts
# the articles into sentences and writes
#
#   $OUT/<lang>.txt     its sentences, at most CAP: the corpus
#                       build-predict learns the language's model from
#                       (`make predict-assets`), which sources.lock records
#   eval/<lang>.txt     the first EVAL_CAP sentences of every 20th article,
#                       held out of the corpus: what eval-predict measures
#                       the model on (committed)
#
# The dumps are kept in $OUT/raw, and a kept one whose sha256 is right is
# not downloaded again.
#
#   scripts/fetch-wikipedia.sh            # every language
#   scripts/fetch-wikipedia.sh ne or      # some
#
# Env: OUT=$TMPDIR/ki-osk-sources/wikipedia  CAP=400000  EVAL_CAP=500
#      EVAL_OUT=eval
set -euo pipefail

DATE=20261108
BASE=https://dumps.wikimedia.org/other/cirrus_search_index/$DATE
# language  wiki  dump file  sha256
DUMPS="
bho  bh   bhwiki_content-20261108-00000.json.bz2   e1becbc89778d7994831288afa94b9c1cce69d1b6b65a485d3500b34a5e822cc
mai  mai  maiwiki_content-20261108-00000.json.bz2  1119ae395e8ca315a20422db6d09d5280e01e7d2530d301ebfb486212fcb5f4c
mn   mn   mnwiki_content-20261108-00000.json.bz2   7712756bc38a38469c054fa6ea3239bf2d7bb593c4038f0208bcf48256b8f355
ne   ne   newiki_content-20261108-00000.json.bz2   4e1254afeb877796bdc2caf1408a4400ab0ba6f5d1341fcf4be0f298addd0d5c
or   or   orwiki_content-20261108-00000.json.bz2   26367cdc4341073a1d43ce6f79267b7832d65d6ccfc0939d24e7d2d9d1cec913
pnb  pnb  pnbwiki_content-20261108-00000.json.bz2  ce4acfa4f564706f8643bd4cc2e39b82576a1aa6720571fefdfa6ea8fd7117b6
ps   ps   pswiki_content-20261108-00000.json.bz2   7f4e3cb5f365405c45ac34851f6edf68c26d457da265d44492d835726a436451
"

HERE=$(cd "$(dirname "$0")" && pwd)
OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/wikipedia}
CAP=${CAP:-400000}
EVAL_CAP=${EVAL_CAP:-500}
EVAL_OUT=${EVAL_OUT:-eval}
langs=${*:-$(echo "$DUMPS" | awk 'NF {print $1}')}

mkdir -p "$OUT/raw" "$EVAL_OUT"
for lang in $langs; do
	row=$(echo "$DUMPS" | awk -v l="$lang" '$1 == l')
	[ -n "$row" ] || { echo "fetch-wikipedia: no dump for $lang" >&2; exit 2; }
	read -r _ wiki file sha <<<"$row"
	dump="$OUT/raw/$file"
	if [ ! -f "$dump" ] || [ "$(sha256sum "$dump" | cut -d' ' -f1)" != "$sha" ]; then
		curl -fsSL --retry 5 --retry-all-errors -o "$dump.part" \
			"$BASE/index_name%3D${wiki}wiki_content/$file"
		got=$(sha256sum "$dump.part" | cut -d' ' -f1)
		if [ "$got" != "$sha" ]; then
			rm -f "$dump.part"
			echo "fetch-wikipedia: $file's sha256 is $got, not $sha" >&2
			exit 1
		fi
		mv -f "$dump.part" "$dump"
	fi
	python3 -I "$HERE/wikipedia-text.py" --cap "$CAP" --eval "$EVAL_CAP" \
		"$OUT/$lang.txt" "$EVAL_OUT/$lang.txt" "$dump"
done
