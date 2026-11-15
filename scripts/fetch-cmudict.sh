#!/usr/bin/env bash
# fetch-cmudict.sh — CMUdict, the Carnegie Mellon University Pronouncing
# Dictionary: how American English says some 135,000 words, in ARPAbet
# (computer: K AH0 M P Y UW1 T ER0). `make english-translit` reads it to
# write the English words a language with a transliteration table writes
# in its own script (translit/<lang>.english.txt), which then ship in its
# pack with CMUdict's notice (BSD-2-Clause-style; NOTICES.md).
#
#   $OUT/cmudict.dict   a word and its sounds a line
#   $OUT/LICENSE        its notice
#
# Both, at one commit of github.com/cmusphinx/cmudict, are pinned by
# sha256 below; a kept one whose sha256 is right is not downloaded again.
#
# Env: OUT=$TMPDIR/ki-osk-sources/cmudict
set -euo pipefail

REV=74790861f652b15e4ac49015a90074ad62a27690
BASE=https://raw.githubusercontent.com/cmusphinx/cmudict/$REV
# file  sha256
FILES="
cmudict.dict  81917843c7f44ce2b094ac63873c2c7a4cf802040792c455ba3ca406891c3d22
LICENSE       bd4ce8e44170a5f9f481310ca85c51de3c4f851a65e679b40e603b143bd3542a
"

OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/cmudict}
mkdir -p "$OUT"
while read -r file sha; do
	[ -n "$file" ] || continue
	if [ ! -f "$OUT/$file" ] || [ "$(sha256sum "$OUT/$file" | cut -d' ' -f1)" != "$sha" ]; then
		curl -fsSL --retry 5 --retry-all-errors -o "$OUT/$file.part" "$BASE/$file"
		got=$(sha256sum "$OUT/$file.part" | cut -d' ' -f1)
		if [ "$got" != "$sha" ]; then
			rm -f "$OUT/$file.part"
			echo "fetch-cmudict: $file's sha256 is $got, not $sha" >&2
			exit 1
		fi
		mv -f "$OUT/$file.part" "$OUT/$file"
	fi
done <<<"$FILES"
echo "$OUT/cmudict.dict: $(wc -l < "$OUT/cmudict.dict") lines, at cmusphinx/cmudict ${REV:0:12}"
