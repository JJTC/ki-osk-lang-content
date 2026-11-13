#!/usr/bin/env bash
# fetch-aksharantar.sh — romanizations people typed, word by word, for
# the transliteration tables Dakshina has none for: Assamese, Maithili,
# Nepali, Odia, Sanskrit, Konkani, Kashmiri, Meitei, Bodo and Dogri (the
# Makefile's AKSHARANTAR_LANGS). Only `make eval-translit` reads them:
# nothing of them is committed or shipped.
#
# They are the test split of AI4Bharat's Aksharantar dataset
# ("Aksharantar: Towards Building Open Transliteration Tools for the Next
# Billion Users", arXiv 2205.03018, which its authors ask to be cited),
# whose words were romanized by hand: frequent words of each language, and
# Indian and foreign names. Its manually collected data is CC BY, its
# mined data CC0. Each language is a zip at Hugging Face
# (ai4bharat/Aksharantar), pinned by sha256 below; the test split
# (<code>_test.json, in a folder in some zips; a JSON object a line:
# "native word", "english word", "source") becomes
#
#   $OUT/<lang>.tsv    the word, its romanization and 1, tab-separated,
#                      a pair a line: what eval-translit --pairs reads
#
# The zips are kept in $OUT/raw, and a kept one whose sha256 is right is
# not downloaded again.
#
#   scripts/fetch-aksharantar.sh            # every language
#   scripts/fetch-aksharantar.sh sa ne      # some
#
# Env: OUT=$TMPDIR/ki-osk-sources/aksharantar
set -euo pipefail

BASE=https://huggingface.co/datasets/ai4bharat/Aksharantar/resolve/main
# language  Aksharantar's code  sha256 of its zip
ZIPS="
as   asm  5ecd0dbbffdfa4026c0624106bbf3f95df3d79bbcdbb653a76fc74a759b909de
mai  mai  bef959e8b98c981d22c03c4a0e23cac25ca69b74c2cbc5bf8e5d0a2061ce3dfd
ne   nep  8bdcc5957d701ea36b63c00c713b43bc30902774b70736737d480e77929de92c
or   ori  9095e8dc4221708ad02826e84c1bbb7e5033e1cbf4e4f7396272ded0947b7c0b
sa   san  8cdc6e500743194116f9d40938c72f8995bd2e9073a7183bee9da3d3cb655168
kok  kok  c08a2896ef3b85f44937c6fcfa75fefdc0736ffb3deceaf0cf9cf64227cc85e9
ks   kas  e2488d3972b440bf75ce16db5b43333a83362c9b2c633c914e50dfb6e07d255b
mni  mni  55d59376fcf08b376dd3db87735869d7d6dd49c78b237582dfcad9f956ed31ad
brx  brx  4117f9dab514ca613174fd0af6c79fcd74233c6f2a1916e8cc73de0f8ffc0d1b
doi  doi  a5f79317baa51be6815d89b1e10fd86c5456fb2877b26bdfb5b979b102b33885
"

# A test split's JSON lines to word<TAB>romanization<TAB>1, the word in
# NFC; pairs with a space in either are left out.
CONVERT='
import json, sys, unicodedata
for line in sys.stdin:
    if not line.strip():
        continue
    row = json.loads(line)
    word = unicodedata.normalize("NFC", row["native word"].strip())
    latin = row["english word"].strip()
    if word and latin and " " not in word and " " not in latin:
        print(f"{word}\t{latin}\t1")
'

OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/aksharantar}
langs=${*:-$(echo "$ZIPS" | awk 'NF {print $1}')}

mkdir -p "$OUT/raw"
for lang in $langs; do
	row=$(echo "$ZIPS" | awk -v l="$lang" '$1 == l')
	[ -n "$row" ] || { echo "fetch-aksharantar: no zip for $lang" >&2; exit 2; }
	read -r _ code sha <<<"$row"
	zip="$OUT/raw/$code.zip"
	if [ ! -f "$zip" ] || [ "$(sha256sum "$zip" | cut -d' ' -f1)" != "$sha" ]; then
		curl -fsSL --retry 5 --retry-all-errors -o "$zip.part" "$BASE/$code.zip"
		got=$(sha256sum "$zip.part" | cut -d' ' -f1)
		if [ "$got" != "$sha" ]; then
			rm -f "$zip.part"
			echo "fetch-aksharantar: $code.zip's sha256 is $got, not $sha" >&2
			exit 1
		fi
		mv -f "$zip.part" "$zip"
	fi
	test=$(unzip -Z1 "$zip" | grep -E "(^|/)${code}_test\.json\$")
	unzip -p "$zip" "$test" | python3 -I -c "$CONVERT" > "$OUT/$lang.tsv.part"
	mv -f "$OUT/$lang.tsv.part" "$OUT/$lang.tsv"
	echo "$OUT/$lang.tsv: $(wc -l < "$OUT/$lang.tsv") pairs"
done
