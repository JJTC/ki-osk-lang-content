#!/usr/bin/env bash
# fetch-madlad.sh — web text of languages neither Wikipedia nor FineWeb-2
# has enough of, from MADLAD-400 (the Makefile's MADLAD_LANGS): Garhwali
# and Rajasthani.
#
# MADLAD-400 (Google, at Hugging Face as allenai/MADLAD-400) is Common
# Crawl's text sorted by language and audited by hand; its card releases
# it under CC BY 4.0 (its metadata says ODC-By), its use subject to Common
# Crawl's Terms of Use. Each language has a clean file and a noisy one,
# both read: scripts/fineweb-text.py keeps the documents that read as the
# language (the noisy file holds as much Hindi as Garhwali), and holds out
# every 20th of them, MADLAD-400 having no test split. The files, at one
# revision, are pinned by sha256 below. It writes
#
#   $OUT/<lang>.txt       the sentences of the documents kept, each once:
#                         the corpus build-predict learns the model from
#   $EVAL_OUT/<lang>.txt  the held-out documents' sentences the corpus
#                         does not have: what eval-predict measures it on
#
# Neither is committed: the text is the web's. The files are kept in
# $OUT/raw, and a kept one whose sha256 is right is not downloaded again.
#
#   scripts/fetch-madlad.sh            # every language
#   scripts/fetch-madlad.sh gbm        # some
#
# Env: OUT=$TMPDIR/ki-osk-sources/madlad  EVAL_OUT=$OUT/eval  PYTHON=python3
set -euo pipefail

REV=9d886a76bd8fa69b294f2dd3843dacb8388ee5a5
BASE=https://huggingface.co/datasets/allenai/MADLAD-400/resolve/$REV/data
# language  file  sha256
FILES="
gbm  gbm/gbm_clean_0000.jsonl.gz  6516174f88d38e5c6db032a3e3d83d002e20a34df0f02f52c9188884b6bb6999
gbm  gbm/gbm_noisy_0000.jsonl.gz  3e11960fff77a149d570afb7f3a07706f3aa5788165bb8175b0519c4295c1f40
raj  raj/raj_clean_0000.jsonl.gz  4fe8eec12497caabb7bdfa55fa0561c90ed6f2a6215588de9db7c9c89499626f
raj  raj/raj_noisy_0000.jsonl.gz  7128d2555a00acb7f003efed15de90b12ded0afcf8ae4b0affa7a45c7d8464bc
"

OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/madlad}
EVAL_OUT=${EVAL_OUT:-$OUT/eval}
PYTHON=${PYTHON:-python3}
langs=${*:-$(echo "$FILES" | awk 'NF {print $1}' | sort -u)}

mkdir -p "$OUT/raw" "$EVAL_OUT"
for lang in $langs; do
	rows=$(echo "$FILES" | awk -v l="$lang" '$1 == l')
	[ -n "$rows" ] || { echo "fetch-madlad: no files for $lang" >&2; exit 2; }
	train=()
	while read -r _ file sha; do
		local_file="$OUT/raw/${file##*/}"
		if [ ! -f "$local_file" ] || [ "$(sha256sum "$local_file" | cut -d' ' -f1)" != "$sha" ]; then
			curl -fsSL --retry 5 --retry-all-errors -o "$local_file.part" "$BASE/$file"
			got=$(sha256sum "$local_file.part" | cut -d' ' -f1)
			if [ "$got" != "$sha" ]; then
				rm -f "$local_file.part"
				echo "fetch-madlad: $file's sha256 is $got, not $sha" >&2
				exit 1
			fi
			mv -f "$local_file.part" "$local_file"
		fi
		train+=("$local_file")
	done <<<"$rows"
	"$PYTHON" -I "$(dirname "$0")/fineweb-text.py" --lang "$lang" \
		"$OUT/$lang.txt.part" "$EVAL_OUT/$lang.txt.part" --train "${train[@]}" --hold 20
	mv -f "$OUT/$lang.txt.part" "$OUT/$lang.txt"
	mv -f "$EVAL_OUT/$lang.txt.part" "$EVAL_OUT/$lang.txt"
done
