#!/usr/bin/env bash
# fetch-fineweb.sh — web text of the languages that have no Wikipedia of
# their own, from FineWeb-2 (the Makefile's FINEWEB_LANGS): Bodo, Dogri,
# Chhattisgarhi, Magahi and Brahui, and Sindhi in Devanagari and Meitei in
# the Bengali script, whose wikis are in their other scripts.
#
# FineWeb-2 (Hugging Face) is Common Crawl's web text sorted by language;
# it is ODC-By 1.0, and its use subject to Common Crawl's Terms of Use.
# Each language's parquet files, its `train` and `test` splits at one
# revision of the dataset, are pinned by sha256 below.
# scripts/fineweb-text.py keeps the documents that read as their language
# (FineWeb-2's language classifier mistakes close languages: its Dogri
# holds Punjabi poetry) and writes
#
#   $OUT/<lang>.txt       the train split's sentences, each once: the
#                         corpus build-predict learns the model from
#   $EVAL_OUT/<lang>.txt  the test split's, none the corpus has: what
#                         eval-predict measures the model on
#
# Neither is committed: the text is the web's. The parquet files are kept
# in $OUT/raw, and a kept one whose sha256 is right is not downloaded
# again. Reading them needs pyarrow: PYTHON names a Python that has it.
#
#   scripts/fetch-fineweb.sh            # every language
#   scripts/fetch-fineweb.sh brx sd-Deva    # some
#
# Env: OUT=$TMPDIR/ki-osk-sources/fineweb  EVAL_OUT=$OUT/eval  PYTHON=python3
set -euo pipefail

REV=af9c13333eb981300149d5ca60a8e9d659b276b9
BASE=https://huggingface.co/datasets/HuggingFaceFW/fineweb-2/resolve/$REV/data
# language  configuration  file  sha256
FILES="
brx  brx_Deva  train/000_00000.parquet  76b80110a811ad10b062dc5bc486fff813397d26f6289c6a04bdc5b862f761a8
brx  brx_Deva  test/000_00000.parquet   349eafb3641435fd7aebeb1dd1541c0e22cd27a0998d2428062bac00a339d42d
doi  doi_Deva  train/000_00000.parquet  18605bd59592a6ad4f9e97f48753996f41dec5c32842f49ecd8e70433900521e
doi  doi_Deva  test/000_00000.parquet   78011a0fc0cf93cbc2d057184fff07420657a3bc6ff3a1842404f5d2b356d51a
hne  hne_Deva  train/000_00000.parquet  b1a92142dc14e50b3faf46b4c1c228f01a88575d39210aea8ff892af10fab9b5
hne  hne_Deva  test/000_00000.parquet   b9ac9ba7cf3fabec8173fe96141447404499d5fda97c09733919f327a94b3497
mag  mag_Deva  train/000_00000.parquet  fcc6243b472151eb3b2078c7b44f8c415cfc5a882e18b8ecc10f7a3957d85177
mag  mag_Deva  test/000_00000.parquet   48cc4df0baa1595ea27aa1f194d51eec8758ead19362b02e8a1dde1099069f7c
brh  brh_Arab  train/000_00000.parquet  1b8ba9a400a80d9a8e510e2ea1ab308804d3fc68a6512e867a6ad67b4f05c2fb
brh  brh_Arab  test/000_00000.parquet   cd25fcf97bcd2311b89c1cd72a1f54e52594bfef38e30886c1538cdfbfdb4de2
sd-Deva   snd_Deva  train/000_00000.parquet  7f1ec469c99291070d1260138aaae2baea07bde8d67054762ffdf3681a66b9c9
sd-Deva   snd_Deva  test/000_00000.parquet   cf891207f43a93754a5f0c870fe9c4da73fb28e1e575e6c7d1e29523fa2a8638
mni-Beng  mni_Beng  train/000_00000.parquet  70c880e2be17a13639b86139961cda481a96c25429ba0f136e6061cf6288a73e
mni-Beng  mni_Beng  test/000_00000.parquet   92dd42dc4360a753b7e1a96a5c2a92edcf52a7f747c1c9e73346b24df8875773
"

HERE=$(cd "$(dirname "$0")" && pwd)
OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/fineweb}
EVAL_OUT=${EVAL_OUT:-$OUT/eval}
PYTHON=${PYTHON:-python3}
langs=${*:-$(echo "$FILES" | awk 'NF {print $1}' | uniq)}

"$PYTHON" -c 'import pyarrow' 2>/dev/null ||
	{ echo "fetch-fineweb: $PYTHON has no pyarrow (pip install pyarrow, or set PYTHON)" >&2; exit 2; }
mkdir -p "$OUT/raw" "$EVAL_OUT"
for lang in $langs; do
	rows=$(echo "$FILES" | awk -v l="$lang" '$1 == l')
	[ -n "$rows" ] || { echo "fetch-fineweb: no files for $lang" >&2; exit 2; }
	train=()
	test=()
	while read -r _ config file sha; do
		local_file="$OUT/raw/$config/$file"
		mkdir -p "$(dirname "$local_file")"
		if [ ! -f "$local_file" ] || [ "$(sha256sum "$local_file" | cut -d' ' -f1)" != "$sha" ]; then
			curl -fsSL --retry 5 --retry-all-errors -o "$local_file.part" "$BASE/$config/$file"
			got=$(sha256sum "$local_file.part" | cut -d' ' -f1)
			if [ "$got" != "$sha" ]; then
				rm -f "$local_file.part"
				echo "fetch-fineweb: $config/$file's sha256 is $got, not $sha" >&2
				exit 1
			fi
			mv -f "$local_file.part" "$local_file"
		fi
		case "$file" in
			train/*) train+=("$local_file") ;;
			test/*) test+=("$local_file") ;;
		esac
	done <<<"$rows"
	"$PYTHON" -I "$HERE/fineweb-text.py" --lang "$lang" "$OUT/$lang.txt" "$EVAL_OUT/$lang.txt" \
		--train "${train[@]}" --test "${test[@]}"
done
