#!/usr/bin/env bash
# fetch-word-lists.sh — the word lists of the languages written without
# spaces between words, which cut their text into words
# (osk_data::wordbreak): ICU's word-break dictionaries, pinned below by
# ICU release and sha256, and committed as wordbreak/<lang>.txt as ICU
# has them, their copyright lines with them. Unicode License V3, and the
# ICU licence before it for what IBM and Apple wrote (NOTICES.md).
#
# A committed list that matches its sha256 is left alone; one missing or
# changed is downloaded again, and refused unless it matches.
#
#   scripts/fetch-word-lists.sh            # every list below
#
# Lao's is the Lao Word List of Brian Eugene Wilson and Robert Martin
# Campbell, as ICU has it, with its own BSD notice besides (NOTICES.md).
# Khmer and Burmese could follow the same way: ICU has their lists too
# (khmerdict.txt, burmesedict.txt), Burmese's with a notice of its own.
set -eu

ICU=release-79.1
BASE=https://raw.githubusercontent.com/unicode-org/icu/$ICU/icu4c/source/data/brkitr/dictionaries
OUT=${OUT:-wordbreak}

# lang  ICU's file  its sha256
LISTS="
th thaidict.txt 3166abde40c0f44ab91c28f5ce96d7d1472cb7882e1c0bda0a72f8f69dba4274
lo laodict.txt 3c876934a3fa81031d2333525eafaca6a7c9f842e3b98f18c38880420afb5d36
"

sha() { sha256sum "$1" | cut -d' ' -f1; }

mkdir -p "$OUT"
echo "$LISTS" | while read -r lang file want; do
	[ -n "$lang" ] || continue
	dest=$OUT/$lang.txt
	if [ -f "$dest" ] && [ "$(sha "$dest")" = "$want" ]; then
		echo "$dest: ICU $ICU $file, as pinned"
		continue
	fi
	part=$dest.part
	curl -fsSL --retry 5 --retry-all-errors --max-time 300 -o "$part" "$BASE/$file"
	got=$(sha "$part")
	if [ "$got" != "$want" ]; then
		rm -f "$part"
		echo "!! $BASE/$file: sha256 $got, pinned $want" >&2
		exit 1
	fi
	mv -f "$part" "$dest"
	echo "$dest: ICU $ICU $file, fetched"
done
