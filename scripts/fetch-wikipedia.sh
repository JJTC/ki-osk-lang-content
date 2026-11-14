#!/usr/bin/env bash
# fetch-wikipedia.sh — the text of a language's Wikipedia, for the
# languages whose models learn from it alone (the Makefile's
# WIKIPEDIA_LANGS): Bhojpuri, Maithili, Mongolian, Nepali, Odia, Western
# Punjabi, Pashto, Sanskrit, Santali, Yiddish, Konkani (the Goan wiki's
# Devanagari articles), Kashmiri, Meitei, Awadhi, Saraiki and Tulu; and
# the text of the Wikimedia Incubator's test wikis of languages with no
# Wikipedia yet: Magahi (beside its FineWeb-2 text), Adilabad Gondi, in
# the Telugu script, and Khowar.
#
# The text comes from each wiki's CirrusSearch dump (Wikimedia's
# cirrus_search_index, an article a JSON document whose "text" is its
# plain text, no markup), of one date, each file pinned by its sha256
# below; Wikipedia's text is CC BY-SA 4.0. scripts/wikipedia-text.py cuts
# the articles into sentences, keeping where a script is named only those
# at least half of whose letters are of it, and writes
#
#   $OUT/<lang>.txt     its sentences, at most CAP: the corpus
#                       build-predict learns the language's model from
#                       (`make predict-assets`), which sources.lock records
#   eval/<lang>.txt     the first EVAL_CAP sentences of every 20th article,
#                       held out of the corpus: what eval-predict measures
#                       the model on (committed)
#
# The Incubator's dump holds all its test wikis: a language's articles
# are those whose title starts with its prefix (Wp/mag/), and with a
# neighbour named, an article heavy with that neighbour's function words
# is left out (wikipedia-text.py --neighbour): a test wiki's writers paste
# their neighbour's articles in. The dumps are kept in $OUT/raw, and a
# kept one whose sha256 is right is not downloaded again.
#
#   scripts/fetch-wikipedia.sh            # every language
#   scripts/fetch-wikipedia.sh ne or      # some
#
# Env: OUT=$TMPDIR/ki-osk-sources/wikipedia  CAP=400000  EVAL_CAP=500
#      EVAL_OUT=eval
set -euo pipefail

DATE=20261108
BASE=https://dumps.wikimedia.org/other/cirrus_search_index/$DATE
# language  wiki  dump file  sha256  script (its Unicode name, _ for a
# space; - keeps every sentence)
DUMPS="
bho  bh   bhwiki_content-20261108-00000.json.bz2   e1becbc89778d7994831288afa94b9c1cce69d1b6b65a485d3500b34a5e822cc  -
mai  mai  maiwiki_content-20261108-00000.json.bz2  1119ae395e8ca315a20422db6d09d5280e01e7d2530d301ebfb486212fcb5f4c  -
mn   mn   mnwiki_content-20261108-00000.json.bz2   7712756bc38a38469c054fa6ea3239bf2d7bb593c4038f0208bcf48256b8f355  -
ne   ne   newiki_content-20261108-00000.json.bz2   4e1254afeb877796bdc2caf1408a4400ab0ba6f5d1341fcf4be0f298addd0d5c  -
or   or   orwiki_content-20261108-00000.json.bz2   26367cdc4341073a1d43ce6f79267b7832d65d6ccfc0939d24e7d2d9d1cec913  -
pnb  pnb  pnbwiki_content-20261108-00000.json.bz2  ce4acfa4f564706f8643bd4cc2e39b82576a1aa6720571fefdfa6ea8fd7117b6  -
ps   ps   pswiki_content-20261108-00000.json.bz2   7f4e3cb5f365405c45ac34851f6edf68c26d457da265d44492d835726a436451  -
sa   sa   sawiki_content-20261108-00000.json.bz2   2bd0f5c4d0afe46270fb88f474fd5663c50458babbd9f18f632f5d4d60d3871c  DEVANAGARI
sat  sat  satwiki_content-20261108-00000.json.bz2  bf8da87ab8e60974ab304b2486cdf55941c57900bc09cb9b64dea05974d7500c  OL_CHIKI
yi   yi   yiwiki_content-20261108-00000.json.bz2   1beb539d0d40d423894bed9891b614c964f349056d6a88cfc65b13b133faae3c  HEBREW
kok  gom  gomwiki_content-20261108-00000.json.bz2  0a60c2753284ff11a8d4871ee64dcd72bcf753f7380b622cf152a04f3d93337c  DEVANAGARI
ks   ks   kswiki_content-20261108-00000.json.bz2   0be241d83fa3e8f7dd6fabb6a9a97152e0cffd6450f02b8f2f8fbe5b4f94efb4  ARABIC
mni  mni  mniwiki_content-20261108-00000.json.bz2  99c8a4cf0de0a86b641d8c396755f5a1b838d46b73e4432b90d76c7ab3992d7f  MEETEI
awa  awa  awawiki_content-20261108-00000.json.bz2  b9b119cc5dcb3b2f7c0e9c08d72384780ce21cd23db06474c5081a89008f0de5  DEVANAGARI
skr  skr  skrwiki_content-20261108-00000.json.bz2  879bbf4d040968cda18a2476c43777f0ff84ba6dc229696be2172c06e863140b  ARABIC
tcy  tcy  tcywiki_content-20261108-00000.json.bz2  985c3225bcfab543d358122f3ced04060446c1a2a5ddc1d7c16784e3801020e5  KANNADA
"
INCUBATOR_FILE=incubatorwiki_content-20261108-00000.json.bz2
INCUBATOR_SHA=e5c0f801331e14919436a737a4b71dd31fb8b792d77392992173435e14cc52d1
# language  prefix  script  neighbour (- for none)
INCUBATOR="
mag  Wp/mag/  DEVANAGARI  hi
wsg  Wp/wsg/  TELUGU      -
khw  Wp/khw/  ARABIC      ur
"

HERE=$(cd "$(dirname "$0")" && pwd)
OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/wikipedia}
CAP=${CAP:-400000}
EVAL_CAP=${EVAL_CAP:-500}
EVAL_OUT=${EVAL_OUT:-eval}
langs=${*:-$(echo "$DUMPS$INCUBATOR" | awk 'NF {print $1}')}

mkdir -p "$OUT/raw" "$EVAL_OUT"
for lang in $langs; do
	row=$(echo "$DUMPS" | awk -v l="$lang" '$1 == l')
	extra=()
	if [ -n "$row" ]; then
		read -r _ wiki file sha script <<<"$row"
	else
		row=$(echo "$INCUBATOR" | awk -v l="$lang" '$1 == l')
		[ -n "$row" ] || { echo "fetch-wikipedia: no dump for $lang" >&2; exit 2; }
		read -r _ prefix script neighbour <<<"$row"
		wiki=incubator file=$INCUBATOR_FILE sha=$INCUBATOR_SHA
		extra=(--prefix "$prefix")
		[ "$neighbour" = - ] || extra+=(--neighbour "$neighbour")
	fi
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
	keep=()
	[ "$script" = - ] || keep=(--script "${script//_/ }")
	python3 -I "$HERE/wikipedia-text.py" --cap "$CAP" --eval "$EVAL_CAP" "${keep[@]}" "${extra[@]}" \
		"$OUT/$lang.txt" "$EVAL_OUT/$lang.txt" "$dump"
done
