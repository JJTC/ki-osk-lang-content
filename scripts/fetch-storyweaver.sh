#!/usr/bin/env bash
# fetch-storyweaver.sh — the stories of languages no other open source has
# text enough of, from StoryWeaver (the Makefile's STORYWEAVER_LANGS).
#
# StoryWeaver (storyweaver.org.in, Pratham Books) publishes picture books
# for children in hundreds of languages, each under CC BY 4.0, and shows
# their pages to a signed-in account. The stories read for each language
# are pinned in scripts/storyweaver-stories.tsv, each by its id and the
# sha256 of its text; scripts/storyweaver-text.py reads them through the
# site's reader, signed in as $STORYWEAVER_EMAIL with
# $STORYWEAVER_PASSWORD, one every five seconds or slower, into $OUT/raw
# (a story already there is not read again, and with all of a language's
# there no account is needed), and writes
#
#   $OUT/<lang>.txt       the sentences of the stories kept, each once:
#                         the corpus build-predict learns the model from
#   $EVAL_OUT/<lang>.txt  the held-out stories' sentences the corpus does
#                         not have: what eval-predict measures it on
#
# Neither is committed, nor the stories. With stories still to read and
# no account, it reads none and writes neither, and predict-assets keeps
# those languages' models. A story whose text is no longer
# as pinned (StoryWeaver's editors changed it) is an error: `--pin`
# lists the language's stories again and pins them anew.
#
#   scripts/fetch-storyweaver.sh             # every language
#   scripts/fetch-storyweaver.sh hlb         # some
#   scripts/fetch-storyweaver.sh --pin hlb   # list and pin its stories anew
#
# Env: OUT=$TMPDIR/ki-osk-sources/storyweaver  EVAL_OUT=$OUT/eval  PYTHON=python3
#      STORYWEAVER_EMAIL, STORYWEAVER_PASSWORD (an account's: for stories
#      $OUT/raw lacks)
set -euo pipefail

STORIES=$(dirname "$0")/storyweaver-stories.tsv
# language|how its text is read|its names at StoryWeaver
NAMES="
hlb|--script DEVANAGARI|Halbi
kfq|--script DEVANAGARI|Korku
nah|--script LATIN --rival es|Nahuatl|Nawatl
"

OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/storyweaver}
EVAL_OUT=${EVAL_OUT:-$OUT/eval}
PYTHON=${PYTHON:-python3}
TEXT=$(dirname "$0")/storyweaver-text.py
pin=
if [ "${1:-}" = --pin ]; then
	pin=1
	shift
fi
langs=${*:-$(echo "$NAMES" | awk -F'|' 'NF {print $1}')}

mkdir -p "$OUT/raw" "$EVAL_OUT"
for lang in $langs; do
	echo "$NAMES" | grep -q "^$lang|" || { echo "fetch-storyweaver: no stories named for $lang" >&2; exit 2; }
done
if [ -n "$pin" ]; then
	for lang in $langs; do
		names=()
		while IFS= read -r name; do names+=(--name "$name"); done \
			< <(echo "$NAMES" | awk -F'|' -v l="$lang" '$1 == l {for (i = 3; i <= NF; i++) print $i}')
		"$PYTHON" -I "$TEXT" pin --stories "$STORIES" --raw "$OUT/raw" --lang "$lang" "${names[@]}"
	done
fi
status=0
# shellcheck disable=SC2086 # the languages are words
"$PYTHON" -I "$TEXT" fetch --stories "$STORIES" --raw "$OUT/raw" $langs || status=$?
if [ "$status" = 3 ]; then
	echo "fetch-storyweaver: no account: $langs keep their models" >&2
	exit 0
elif [ "$status" != 0 ]; then
	exit "$status"
fi
for lang in $langs; do
	read -r -a how < <(echo "$NAMES" | awk -F'|' -v l="$lang" '$1 == l {print $2}')
	"$PYTHON" -I "$TEXT" text --stories "$STORIES" --raw "$OUT/raw" --lang "$lang" "${how[@]}" \
		"$OUT/$lang.txt.part" "$EVAL_OUT/$lang.txt.part"
	mv -f "$OUT/$lang.txt.part" "$OUT/$lang.txt"
	mv -f "$EVAL_OUT/$lang.txt.part" "$EVAL_OUT/$lang.txt"
done
