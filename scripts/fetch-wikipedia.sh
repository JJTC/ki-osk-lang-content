#!/usr/bin/env bash
# fetch-wikipedia.sh — the text of a language's Wikipedia, for the
# languages whose models learn from it alone (the Makefile's
# WIKIPEDIA_LANGS): Bhojpuri, Maithili, Mongolian, Nepali, Odia, Western
# Punjabi, Pashto, Sanskrit, Santali, Yiddish, Konkani (the Goan wiki's
# Devanagari articles), Kashmiri, Meitei, Awadhi, Saraiki, Tulu, Dotyali,
# Angika and Newari (beside its web text, scripts/fetch-glotcc.sh),
# Eastern Min (in Bàng-uâ-cê, the Han its sentences write beside left
# out), Fiji Hindi (its references, English, left out: --own), Emilian
# (the Emilian–Romagnol wiki's, its Romagnol left out), Church Slavonic
# (its sentences that write a letter of its own, ѣ є ꙑ ѥ…: its
# references are Russian) and Zeelandic (its bots' French communes and
# calendar days left out); of
# Armenian and Persian beside their Tatoeba sentences (the Makefile's
# WIKIPEDIA_PLUS_LANGS, whose held-out text goes elsewhere: EVAL_OUT); and
# the text of the Wikimedia Incubator's test wikis of languages with no
# Wikipedia yet: Magahi (beside its FineWeb-2 text), Adilabad Gondi, in
# the Telugu script, Khowar, and Montenegrin, in Latin letters (its few
# Cyrillic sentences left out); Rinconada Bikol, Jambi, Makassar, Manado,
# Palembang (Musi) and Kelantan-Pattani Malay, Makassarese, Pamona,
# Ibibio and East Franconian, Southern Kurdish and Western Balochi in the
# Arabic script, Rangpuri in the Bengali script, and Abron (the sentences
# that write its ɛ or ɔ: a third of its wiki's are English).
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
#   eval/<lang>.txt     the first EVAL_CAP sentences of every 20th article
#                       (of the small test wikis', every 10th), held out
#                       of the corpus: what eval-predict measures the
#                       model on (committed)
#
# The Incubator's dump holds all its test wikis: a language's articles
# are those whose title starts with its prefix (Wp/mag/), and with a
# neighbour named, an article heavy with that neighbour's function words
# is left out (wikipedia-text.py --neighbour): a test wiki's writers paste
# their neighbour's articles in (Indonesian's and English's beside the
# Malays). The small wikis' bot-made stubs are left out but three of each
# shape (--shapes: Rinconada Bikol's towns, Western Balochi's German
# towns and calendar days). The dumps are kept in $OUT/raw, and a kept
# one whose sha256 is right is not downloaded again.
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
# space; - keeps every sentence)  [wikipedia-text.py's options: a
# neighbour whose articles and sentences are left out, as the
# Incubator's below; --script-only; --shapes, its bots' stubs; --own,
# --letters, --emilian]
DUMPS="
bho  bh   bhwiki_content-20261108-00000.json.bz2   e1becbc89778d7994831288afa94b9c1cce69d1b6b65a485d3500b34a5e822cc  DEVANAGARI  --script-only
mai  mai  maiwiki_content-20261108-00000.json.bz2  1119ae395e8ca315a20422db6d09d5280e01e7d2530d301ebfb486212fcb5f4c  DEVANAGARI  --script-only
mn   mn   mnwiki_content-20261108-00000.json.bz2   7712756bc38a38469c054fa6ea3239bf2d7bb593c4038f0208bcf48256b8f355  CYRILLIC  --script-only
ne   ne   newiki_content-20261108-00000.json.bz2   4e1254afeb877796bdc2caf1408a4400ab0ba6f5d1341fcf4be0f298addd0d5c  DEVANAGARI  --script-only
or   or   orwiki_content-20261108-00000.json.bz2   26367cdc4341073a1d43ce6f79267b7832d65d6ccfc0939d24e7d2d9d1cec913  ORIYA  --script-only
pnb  pnb  pnbwiki_content-20261108-00000.json.bz2  ce4acfa4f564706f8643bd4cc2e39b82576a1aa6720571fefdfa6ea8fd7117b6  ARABIC  --script-only
ps   ps   pswiki_content-20261108-00000.json.bz2   7f4e3cb5f365405c45ac34851f6edf68c26d457da265d44492d835726a436451  ARABIC  --script-only
sa   sa   sawiki_content-20261108-00000.json.bz2   2bd0f5c4d0afe46270fb88f474fd5663c50458babbd9f18f632f5d4d60d3871c  DEVANAGARI  --script-only
sat  sat  satwiki_content-20261108-00000.json.bz2  bf8da87ab8e60974ab304b2486cdf55941c57900bc09cb9b64dea05974d7500c  OL_CHIKI  --script-only
yi   yi   yiwiki_content-20261108-00000.json.bz2   1beb539d0d40d423894bed9891b614c964f349056d6a88cfc65b13b133faae3c  HEBREW  --script-only
kok  gom  gomwiki_content-20261108-00000.json.bz2  0a60c2753284ff11a8d4871ee64dcd72bcf753f7380b622cf152a04f3d93337c  DEVANAGARI
ks   ks   kswiki_content-20261108-00000.json.bz2   0be241d83fa3e8f7dd6fabb6a9a97152e0cffd6450f02b8f2f8fbe5b4f94efb4  ARABIC  --script-only
mni  mni  mniwiki_content-20261108-00000.json.bz2  99c8a4cf0de0a86b641d8c396755f5a1b838d46b73e4432b90d76c7ab3992d7f  MEETEI
awa  awa  awawiki_content-20261108-00000.json.bz2  b9b119cc5dcb3b2f7c0e9c08d72384780ce21cd23db06474c5081a89008f0de5  DEVANAGARI
skr  skr  skrwiki_content-20261108-00000.json.bz2  879bbf4d040968cda18a2476c43777f0ff84ba6dc229696be2172c06e863140b  ARABIC  --script-only
tcy  tcy  tcywiki_content-20261108-00000.json.bz2  985c3225bcfab543d358122f3ced04060446c1a2a5ddc1d7c16784e3801020e5  KANNADA
dty  dty  dtywiki_content-20261108-00000.json.bz2  42b44cbca756eb46985b9ef5680aa95642bd4223a8ed636c520e399c55e95a31  DEVANAGARI
anp  anp  anpwiki_content-20261108-00000.json.bz2  19363d8660dd1a8d540fcd7253851a9d90737ba4ac8cc59606a476fa6df458a3  DEVANAGARI  --neighbour hi
new  new  newwiki_content-20261108-00000.json.bz2  c8d5a485491d164b541fed44b20f4af5d185cdf13d0544d151d02b233e2f7e73  DEVANAGARI
hy   hy   hywiki_content-20261108-00000.json.bz2   66aa04cde62eb5d4f16317e0bd00dfffca8b5c4def9bb9f86a2f673f3f220ea8  ARMENIAN  --script-only
fa   fa   fawiki_content-20261108-00000.json.bz2   1c793a3006d5ed7f78e58cf3f12059fe423c6fd30efbff1fe696a53c7e240ae1  ARABIC  --script-only
cdo  cdo  cdowiki_content-20261108-00000.json.bz2  b087ff5764e512a440b6f2486288a9523b891bb9c2c676987f3230324c4c443a  LATIN  --script-only --shapes 3
hif  hif  hifwiki_content-20261108-00000.json.bz2  1490558966f73af1384d677456499fb15cc084db754d350fd03dd2475da45ae7  LATIN  --own hif --shapes 3
egl  eml  emlwiki_content-20261108-00000.json.bz2  d9123a8b5d3fd5cf83bc0283bb34d44b791a021cfe2d82d2e72360687dc8971a  LATIN  --emilian --shapes 3
cu   cu   cuwiki_content-20261108-00000.json.bz2   12aac7a8422656a861bcc6756d47ef540c68ecdbc7db316b94c6cb0a9eca68c5  CYRILLIC  --script-only --shapes 3 --letters єѣꙑѥїꙁꙗѧѫѩѡіѭѵѷѳꙋѯѱѿѻѕѹ
zea  zea  zeawiki_content-20261108-00000.json.bz2  25f44b4a3d2d9de7f626493df4d725c736e783e5b611f93faaa3d6ff2a904edc  LATIN  --shapes 3
"
INCUBATOR_FILE=incubatorwiki_content-20261108-00000.json.bz2
INCUBATOR_SHA=e5c0f801331e14919436a737a4b71dd31fb8b792d77392992173435e14cc52d1
# language  prefix  script  neighbours (- for none)  [wikipedia-text.py's
# options: the small wikis' every 10th article held out, at most three
# sentences of a shape (--shapes, their bots' stubs), Southern Kurdish
# read as Sorani's keys type it, Abron's sentences those that write its
# ɛ or ɔ (--letters)]
INCUBATOR="
mag  Wp/mag/  DEVANAGARI  hi
wsg  Wp/wsg/  TELUGU      -
khw  Wp/khw/  ARABIC      ur
cnr  Wp/cnr/  LATIN       -
bto  Wp/bto/  LATIN       en     --shapes 3 --holdout 10
jax  Wp/jax/  LATIN       id,en  --shapes 3 --holdout 10
mfp  Wp/mfp/  LATIN       id,en  --shapes 3 --holdout 10
xmm  Wp/xmm/  LATIN       id,en  --shapes 3 --holdout 10
mui  Wp/mui/  LATIN       id,en  --shapes 3 --holdout 10
mak  Wp/mak/  LATIN       id,en  --shapes 3 --holdout 10
pmf  Wp/pmf/  LATIN       -      --shapes 3 --holdout 10
mfa  Wp/mfa/  LATIN       id,en  --shapes 3 --holdout 10
ibb  Wp/ibb/  LATIN       en     --shapes 3 --holdout 10
vmf  Wp/vmf/  LATIN       -      --shapes 3 --holdout 10
sdh  Wp/sdh/  ARABIC      fa     --shapes 3 --holdout 10 --sorani
bgn  Wp/bgn/  ARABIC      fa,ur  --shapes 3 --holdout 10
rkt  Wp/rkt/  BENGALI     -      --shapes 3 --holdout 10
abr  Wp/abr/  LATIN       -      --shapes 3 --holdout 10 --letters ɛɔ
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
		read -r _ wiki file sha script options <<<"$row"
		# shellcheck disable=SC2206 # the options are words
		[ -z "$options" ] || extra=($options)
	else
		row=$(echo "$INCUBATOR" | awk -v l="$lang" '$1 == l')
		[ -n "$row" ] || { echo "fetch-wikipedia: no dump for $lang" >&2; exit 2; }
		read -r _ prefix script neighbour options <<<"$row"
		wiki=incubator file=$INCUBATOR_FILE sha=$INCUBATOR_SHA
		extra=(--prefix "$prefix")
		[ "$neighbour" = - ] || extra+=(--neighbour "$neighbour")
		# shellcheck disable=SC2206 # the options are words
		[ -z "$options" ] || extra+=($options)
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
