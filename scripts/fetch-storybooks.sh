#!/usr/bin/env bash
# fetch-storybooks.sh — the stories of languages no other open source has
# text enough of, from the African Storybook (the Makefile's
# STORYBOOK_LANGS): Masaaba and Turkana.
#
# The African Storybook (africanstorybook.org, an initiative of Saide)
# publishes picture books for children written and translated by teachers
# and writers across Africa, each under CC BY or CC BY-NC; the Global
# African Storybook Project's asp-raw-db repository holds every story's
# text and metadata as the site had them in February 2023. Its archive at
# one commit is pinned by sha256 below. scripts/storybook-text.py reads
# the CC BY stories filed under the language's name, leaves out the pages
# another language's, and holds out every 10th story. It writes
#
#   $OUT/<lang>.txt       the sentences of the stories kept, each once:
#                         the corpus build-predict learns the model from
#   $EVAL_OUT/<lang>.txt  the held-out stories' sentences the corpus does
#                         not have: what eval-predict measures it on
#
# Neither is committed; the archive is kept in $OUT/raw, and not
# downloaded again while its sha256 is right.
#
#   scripts/fetch-storybooks.sh            # every language
#   scripts/fetch-storybooks.sh tuv        # some
#
# Env: OUT=$TMPDIR/ki-osk-sources/storybooks  EVAL_OUT=$OUT/eval  PYTHON=python3
set -euo pipefail

REV=096a27fe24ec8a268d38f4032d98fe366dd3188a
URL=https://codeload.github.com/global-asp/asp-raw-db/tar.gz/$REV
SHA=7a69e5f85d16ca66d50af74dd59ed7b70b87d712fa1eb766c1024854e3d10c70
# language|its name in the database
NAMES="
myx|Lumasaaba
tuv|Ng’aturkana
"

OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/storybooks}
EVAL_OUT=${EVAL_OUT:-$OUT/eval}
PYTHON=${PYTHON:-python3}
langs=${*:-$(echo "$NAMES" | awk -F'|' 'NF {print $1}')}

mkdir -p "$OUT/raw" "$EVAL_OUT"
archive="$OUT/raw/asp-raw-db-$REV.tar.gz"
if [ ! -f "$archive" ] || [ "$(sha256sum "$archive" | cut -d' ' -f1)" != "$SHA" ]; then
	curl -fsSL --retry 5 --retry-all-errors -o "$archive.part" "$URL"
	got=$(sha256sum "$archive.part" | cut -d' ' -f1)
	if [ "$got" != "$SHA" ]; then
		rm -f "$archive.part"
		echo "fetch-storybooks: the archive's sha256 is $got, not $SHA" >&2
		exit 1
	fi
	mv -f "$archive.part" "$archive"
fi
for lang in $langs; do
	name=$(echo "$NAMES" | awk -F'|' -v l="$lang" '$1 == l {print $2}')
	[ -n "$name" ] || { echo "fetch-storybooks: no stories named for $lang" >&2; exit 2; }
	"$PYTHON" -I "$(dirname "$0")/storybook-text.py" --lang "$lang" --name "$name" \
		"$OUT/$lang.txt.part" "$EVAL_OUT/$lang.txt.part" "$archive"
	mv -f "$OUT/$lang.txt.part" "$OUT/$lang.txt"
	mv -f "$EVAL_OUT/$lang.txt.part" "$EVAL_OUT/$lang.txt"
done
