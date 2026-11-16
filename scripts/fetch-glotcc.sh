#!/usr/bin/env bash
# fetch-glotcc.sh — web text of languages whose wiki is mostly a bot's
# stubs, from GlotCC (the Makefile's GLOTCC_LANGS): Newari, whose wiki is
# half village stubs a bot wrote and whose web text is news; and of
# Nigerian Pidgin, whose web text FineWeb-2 has too little of alone.
#
# GlotCC (cis-lmu/GlotCC-V1 at Hugging Face; Kargaran, Yvon and Schütze,
# "GlotCC: An Open Broad-Coverage CommonCrawl Corpus and Pipeline for
# Minority Languages", NeurIPS 2024) is Common Crawl's text sorted by
# language for a thousand minority languages; its card licenses its
# packaging, metadata and annotations CC0 1.0, the text itself being under
# Common Crawl's Terms of Use. Each language's parquet file, at one
# revision, is pinned by sha256 below. scripts/fineweb-text.py keeps the
# documents that read as the language, and holds out every 20th of them,
# GlotCC having no test split. A language FineWeb-2 has as well
# (ALSO_FINEWEB) is read after scripts/fetch-fineweb.sh has written its
# text: a sentence that text has, its corpus's or held out, is neither
# written again nor held out, so that the language's held-out text is in
# neither corpus. It writes
#
#   $OUT/<lang>.txt       the sentences of the documents kept, each once:
#                         the corpus build-predict learns the model from
#   $EVAL_OUT/<lang>.txt  the held-out documents' sentences the corpus
#                         does not have: what eval-predict measures it on
#
# Neither is committed: the text is the web's. The files are kept in
# $OUT/raw, and a kept one whose sha256 is right is not downloaded again.
# Reading them needs pyarrow: PYTHON names a Python that has it.
#
#   scripts/fetch-glotcc.sh            # every language
#   scripts/fetch-glotcc.sh new        # some
#
# Env: OUT=$TMPDIR/ki-osk-sources/glotcc  EVAL_OUT=$OUT/eval  PYTHON=python3
#      FINEWEB_OUT=$OUT/../fineweb (scripts/fetch-fineweb.sh's OUT)
set -euo pipefail

REV=9ad140b6be3ac7b539606a2b4809b49d122823de
BASE=https://huggingface.co/datasets/cis-lmu/GlotCC-V1/resolve/$REV
# language  file  sha256
FILES="
new  v1.0/new-Deva/new-Deva_0.parquet  77ab27dc09914605ecc0d102f5acd56e9f4a6889fc9cd96fc51cf6c930360b68
pcm  v1.0/pcm-Latn/pcm-Latn_0.parquet  490ffb1a6813455318b27b533bcfc6a634ba199f69eaea369ca07640c3795d73
"
ALSO_FINEWEB="pcm"

OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/glotcc}
EVAL_OUT=${EVAL_OUT:-$OUT/eval}
FINEWEB_OUT=${FINEWEB_OUT:-$OUT/../fineweb}
PYTHON=${PYTHON:-python3}
langs=${*:-$(echo "$FILES" | awk 'NF {print $1}' | sort -u)}

"$PYTHON" -c 'import pyarrow' 2>/dev/null ||
	{ echo "fetch-glotcc: $PYTHON has no pyarrow (pip install pyarrow, or set PYTHON)" >&2; exit 2; }
mkdir -p "$OUT/raw" "$EVAL_OUT"
for lang in $langs; do
	rows=$(echo "$FILES" | awk -v l="$lang" '$1 == l')
	[ -n "$rows" ] || { echo "fetch-glotcc: no files for $lang" >&2; exit 2; }
	train=()
	while read -r _ file sha; do
		local_file="$OUT/raw/${file##*/}"
		if [ ! -f "$local_file" ] || [ "$(sha256sum "$local_file" | cut -d' ' -f1)" != "$sha" ]; then
			curl -fsSL --retry 5 --retry-all-errors -o "$local_file.part" "$BASE/$file"
			got=$(sha256sum "$local_file.part" | cut -d' ' -f1)
			if [ "$got" != "$sha" ]; then
				rm -f "$local_file.part"
				echo "fetch-glotcc: $file's sha256 is $got, not $sha" >&2
				exit 1
			fi
			mv -f "$local_file.part" "$local_file"
		fi
		train+=("$local_file")
	done <<<"$rows"
	exclude=()
	case " $ALSO_FINEWEB " in
		*" $lang "*)
			for f in "$FINEWEB_OUT/$lang.txt" "$FINEWEB_OUT/eval/$lang.txt"; do
				[ -f "$f" ] || { echo "fetch-glotcc: no $f (scripts/fetch-fineweb.sh $lang first)" >&2; exit 2; }
			done
			exclude=(--exclude "$FINEWEB_OUT/$lang.txt" "$FINEWEB_OUT/eval/$lang.txt") ;;
	esac
	"$PYTHON" -I "$(dirname "$0")/fineweb-text.py" --lang "$lang" \
		"$OUT/$lang.txt.part" "$EVAL_OUT/$lang.txt.part" --train "${train[@]}" --hold 20 "${exclude[@]}"
	mv -f "$OUT/$lang.txt.part" "$OUT/$lang.txt"
	mv -f "$EVAL_OUT/$lang.txt.part" "$EVAL_OUT/$lang.txt"
done
