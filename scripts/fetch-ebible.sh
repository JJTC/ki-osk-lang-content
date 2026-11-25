#!/usr/bin/env bash
# fetch-ebible.sh — Bibles in languages whose keyboards have no other open
# text, from eBible.org (the Makefile's EBIBLE_LANGS): Southern Oromo's
# (Guji's Kitaaba Woyyuu), Kituba of the Congo Republic's (Mambu ya Nzambi)
# and Yaka's (its New Testament); and Gamo's two, its Bible (Geeshsha
# Mexaafa, in Latin letters) and its New Testament (Gamo Geesha Maxxafa),
# beside its web text, which lost more than half its words when bible.is's
# pages were left out (data 0.84.0).
#
# eBible.org publishes Bible translations as plain text, each under its
# own copyright and licence; these are CC BY-SA 4.0 but Gamo's New
# Testament, CC BY 4.0, as their about pages say (NOTICES.md names each
# one's holders); those under a licence that forbids derived works or
# commercial use are not read. A translation's verses,
# `<id>_vpl.txt` in its archive `<id>_vpl.zip`, are pinned by sha256
# below: the archive itself is made again whenever the site is, and its
# address names no version. scripts/ebible-text.py reads the verses as the
# keyboard types them, a language's Bibles in turn when it has more than
# one, and holds out every 10th chapter. It writes
#
#   $OUT/<lang>.txt       the Bible's sentences, each once: the corpus
#                         build-predict learns the model from
#   $EVAL_OUT/<lang>.txt  the held-out chapters' sentences the corpus does
#                         not have: what eval-predict measures it on (but
#                         Gamo's, measured on its web text's held out)
#
# Neither is committed. A translation whose text has changed since its pin
# is not read, nor its language's text written, and predict-assets keeps
# its language's model until the pin is moved. The verses are kept in
# $OUT/raw, and kept ones whose sha256 is right are not downloaded again.
#
#   scripts/fetch-ebible.sh            # every language
#   scripts/fetch-ebible.sh yaf        # some
#
# Env: OUT=$TMPDIR/ki-osk-sources/ebible  EVAL_OUT=$OUT/eval  PYTHON=python3
set -euo pipefail

BASE=https://ebible.org/Scriptures
# language  translation  sha256 of its <id>_vpl.txt (a row each)
FILES="
gax  gax  8c299dfb58e45add11f0440557f2c1d6bba5c2b010295e5a166b531c080f4dad
mkw  mkw  e839d53a74b326f26dabebf70f5c7e7744239e3c5c89cfd15ed584c813d02217
yaf  yaf  9cb20b4b8aba6dfdc9ddec06e96ccee47638bb4dda060d8fa8f274f1e7e922fd
gmv  gmvl    cd7ff8e159e762f1a5d1f9678ba561b711cfec4af333eb868b68a9bb4382009e
gmv  gmvggm  ae9a274019ffda49a3c5306e0c7f0344bde42c638f05a5ab03032c63fe64e14e
"

OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/ebible}
EVAL_OUT=${EVAL_OUT:-$OUT/eval}
PYTHON=${PYTHON:-python3}
langs=${*:-$(echo "$FILES" | awk 'NF && !seen[$1]++ {print $1}')}

mkdir -p "$OUT/raw" "$EVAL_OUT"
for lang in $langs; do
	rows=$(echo "$FILES" | awk -v l="$lang" '$1 == l')
	[ -n "$rows" ] || { echo "fetch-ebible: no Bible for $lang" >&2; exit 2; }
	all=() changed=
	while read -r _ id sha; do
		verses="$OUT/raw/${id}_vpl.txt"
		all+=("$verses")
		[ -f "$verses" ] && [ "$(sha256sum "$verses" | cut -d' ' -f1)" = "$sha" ] && continue
		curl -fsSL --retry 5 --retry-all-errors -o "$OUT/raw/${id}_vpl.zip.part" "$BASE/${id}_vpl.zip"
		unzip -p "$OUT/raw/${id}_vpl.zip.part" "${id}_vpl.txt" >"$verses.part"
		rm -f "$OUT/raw/${id}_vpl.zip.part"
		got=$(sha256sum "$verses.part" | cut -d' ' -f1)
		if [ "$got" != "$sha" ]; then
			rm -f "$verses.part"
			echo "fetch-ebible: ${id}_vpl.txt's sha256 is $got, not $sha: the translation changed; $lang keeps its model" >&2
			changed=1
			continue
		fi
		mv -f "$verses.part" "$verses"
	done <<<"$rows"
	if [ -n "$changed" ]; then
		rm -f "$OUT/$lang.txt" "$EVAL_OUT/$lang.txt"
		continue
	fi
	"$PYTHON" -I "$(dirname "$0")/ebible-text.py" "$OUT/$lang.txt.part" "$EVAL_OUT/$lang.txt.part" "${all[@]}"
	mv -f "$OUT/$lang.txt.part" "$OUT/$lang.txt"
	mv -f "$EVAL_OUT/$lang.txt.part" "$EVAL_OUT/$lang.txt"
done
