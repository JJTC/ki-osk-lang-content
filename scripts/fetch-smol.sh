#!/usr/bin/env bash
# fetch-smol.sh — professional translations into languages no other open
# source has text enough of, from SMOL (the Makefile's SMOL_LANGS): twelve
# of India's in Devanagari, Garo in Latin letters, and Zarma.
#
# SMOL (Google Research, at Hugging Face as google/smol) is English text
# translated by professional translators into some 220 low-resource
# languages, released under CC BY 4.0: SmolDoc, documents translated a
# sentence at a time, and SmolSent, single sentences. A language's files,
# at one revision, are pinned by sha256 below. scripts/smol-text.py reads
# them as the keyboard types them and holds out every 10th document (and
# every 10th sentence of SmolSent's). It writes
#
#   $OUT/<lang>.txt       the translations' sentences, each once: the
#                         corpus build-predict learns the model from
#   $EVAL_OUT/<lang>.txt  the held-out documents' sentences the corpus
#                         does not have: what eval-predict measures it on
#
# Neither is committed. The files are kept in $OUT/raw, and a kept one
# whose sha256 is right is not downloaded again.
#
#   scripts/fetch-smol.sh            # every language
#   scripts/fetch-smol.sh kru        # some
#
# Env: OUT=$TMPDIR/ki-osk-sources/smol  EVAL_OUT=$OUT/eval  PYTHON=python3
set -euo pipefail

REV=fdaff3a1a019f89fa30a562a85d7c1d3e9150444
BASE=https://huggingface.co/datasets/google/smol/resolve/$REV
# language  file  sha256
FILES="
ahr  smoldoc/en_ahr.jsonl       a9d22214e1feb70b8c7a3d8fb59cbabfd504099855c73f1f49da8d9bb58acd49
bfy  smoldoc/en_bfy.jsonl       576d7d6094c62f120a3792c941ea11d7f2425dfaea6111b1a7148a702d7f69cf
bgq  smoldoc/en_bgq.jsonl       17ea6126ce1c47e7ff920f50feadae9673ed6fccae02d4da85c7c0c640d55ba3
bns  smoldoc/en_bns.jsonl       e27ceb8be7a2ddd8ef5be133aa002e45dcd7d78f244ffd9a908f0c7dc8c1395d
kfy  smoldoc/en_kfy.jsonl       de415c6a6633750839845f83804554d374a7b68250be329e9f4a38f0bfc87563
kru  smoldoc/en_kru.jsonl       ec2635f6c711d7272106183f920f2a5aea495def8e13081e958a9dca19bafea2
mjl  smoldoc/en_mjl.jsonl       e27032159716cc047d8aee0704886aefca3cf1bf111939306a07434e5072df8d
noe  smoldoc/en_noe.jsonl       1536b67cb44583036b0c3473143cf20bbb9c361102663efb0dda9003228b0934
sgj  smoldoc/en_sgj.jsonl       baec8efaa938b21b7f8fa77fd0ad8150cabb6796697245ffa735e12438adb73f
sjp  smoldoc/en_sjp.jsonl       32bf7929c55b103f389af42856f51745e2bedd2d59b6e8ce7e90ef05530ae5f7
unr  smoldoc/en_unr-Deva.jsonl  76a8f14eb89f628f345c259564d198ea7a10aad80253eb74aab30455e08074f0
xnr  smoldoc/en_xnr.jsonl       b512949a151aaa597223bfe424c9eae7e2dcc54da651a56075130011bbe7a04c
grt  smoldoc/en_grt-Latn.jsonl  3c7374dd17779ba5731635e9d90d1a25238a499fe8084b2218efffb16edf6ef8
dje  smoldoc/en_dje.jsonl       7853c6c14a014c75bffc4ced81e181fbc7d426b00985450066738ae4fa81d0fb
dje  smolsent/en_dje.jsonl      91854aa3c617af5a752d02b647acf95147ffafabc00c1d6e4382d7cac1c99a97
"

OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/smol}
EVAL_OUT=${EVAL_OUT:-$OUT/eval}
PYTHON=${PYTHON:-python3}
langs=${*:-$(echo "$FILES" | awk 'NF {print $1}' | uniq)}

mkdir -p "$OUT/raw" "$EVAL_OUT"
for lang in $langs; do
	rows=$(echo "$FILES" | awk -v l="$lang" '$1 == l')
	[ -n "$rows" ] || { echo "fetch-smol: no files for $lang" >&2; exit 2; }
	files=()
	while read -r _ file sha; do
		# smoldoc/en_aa.jsonl and smolsent/en_aa.jsonl: one name each.
		local_file="$OUT/raw/${file%%/*}.${file##*/}"
		if [ ! -f "$local_file" ] || [ "$(sha256sum "$local_file" | cut -d' ' -f1)" != "$sha" ]; then
			curl -fsSL --retry 5 --retry-all-errors -o "$local_file.part" "$BASE/$file"
			got=$(sha256sum "$local_file.part" | cut -d' ' -f1)
			if [ "$got" != "$sha" ]; then
				rm -f "$local_file.part"
				echo "fetch-smol: $file's sha256 is $got, not $sha" >&2
				exit 1
			fi
			mv -f "$local_file.part" "$local_file"
		fi
		files+=("$local_file")
	done <<<"$rows"
	"$PYTHON" -I "$(dirname "$0")/smol-text.py" --lang "$lang" \
		"$OUT/$lang.txt.part" "$EVAL_OUT/$lang.txt.part" "${files[@]}"
	mv -f "$OUT/$lang.txt.part" "$OUT/$lang.txt"
	mv -f "$EVAL_OUT/$lang.txt.part" "$EVAL_OUT/$lang.txt"
done
