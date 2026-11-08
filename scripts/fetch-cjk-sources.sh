#!/usr/bin/env bash
# fetch-cjk-sources.sh — download the upstream datasets the CJK
# dictionaries are built from, into $OUT (default /tmp/ki-osk-sources).
#
#   cedict.txt                    CC-CEDICT, MDBG export      → pinyin, zhuyin
#   zh_cn_50k.txt                 FrequencyWords zh_cn        → ranking for all three
#   cccedict-canto-readings.txt   CC-Canto Cantonese readings → jyutping
#   mozc-dict.tsv                 mozc dictionary00–09        → kana/mozc-kanji.bin
#   mozc-connection.txt           mozc connection matrix      → kana/mozc-connection.bin
#
# Only the built .bins are committed; `sources.lock` records the sha256 of
# each of these files so a committed dictionary names the exact input it
# came from. Licences and attribution: NOTICES.md (several are share-alike).
#
# Every file is downloaded with `curl -f`, written atomically, and checked
# for a plausible minimum size, so an error page or a truncated transfer
# fails here instead of turning into a quietly smaller dictionary.
set -eu

OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources}

CEDICT_URL=https://www.mdbg.net/chinese/export/cedict/cedict_1_0_ts_utf-8_mdbg.txt.gz
# Word frequencies for ranking the candidates. CC-CEDICT carries no
# frequency data at all, and without this the order is CC-CEDICT's own
# (≈ codepoint order), which puts archaic variants ahead of everyday
# characters — see ki-osk's crates/osk-core/tests/pinyin_ranking.rs,
# which fails against a bundle built without it.
#
# The SIMPLIFIED list ranks the Traditional IMEs too (zhuyin, jyutping).
# That is what the committed bundles were built with — verified on
# 2026-11-08 by rebuilding jyutping/cedict.bin byte-for-byte from this
# file. The characters the IMEs most need first (的 是 不 我 你 好) are
# the same in both scripts; zh_tw_50k.txt would rank the Traditional-only
# forms better, but switching is a ranking change to evaluate, not a fix.
ZHFREQ_URL=https://raw.githubusercontent.com/hermitdave/FrequencyWords/master/content/2018/zh_cn/zh_cn_50k.txt
# The readings file, NOT the CC-Canto dictionary (cccanto-*.zip) next to
# it: CC-CEDICT's headwords with Jyutping added in {braces}, which is the
# line format build-jyutping parses. Dated 2015-09-23 and still the
# latest; a newer release would arrive under a new file name.
CANTO_URL=https://cantonese.org/cccedict-canto-readings-150923.zip
MOZC_RAW=https://raw.githubusercontent.com/google/mozc/master/src/data/dictionary_oss

mkdir -p "$OUT"

get() {  # url dest
	curl -fsSL --retry 5 --retry-all-errors --max-time 600 -o "$2.part" "$1" \
		|| { rm -f "$2.part"; echo "!! download failed: $1" >&2; exit 1; }
}

# Refuse a file with fewer lines than any real release has had. These are
# floors against error pages and truncation, not exact expectations.
need_lines() {  # file min
	local n
	n=$(wc -l < "$1")
	[ "$n" -ge "$2" ] || { echo "!! $1: $n lines, expected at least $2" >&2; exit 1; }
	printf '  %-30s %9d lines\n' "${1##*/}" "$n"
}

get "$CEDICT_URL" "$OUT/cedict.txt.gz"
gzip -dc "$OUT/cedict.txt.gz.part" > "$OUT/cedict.txt.part"
rm -f "$OUT/cedict.txt.gz.part"
need_lines "$OUT/cedict.txt.part" 100000
mv -f "$OUT/cedict.txt.part" "$OUT/cedict.txt"

get "$ZHFREQ_URL" "$OUT/zh_cn_50k.txt"
need_lines "$OUT/zh_cn_50k.txt.part" 50000
mv -f "$OUT/zh_cn_50k.txt.part" "$OUT/zh_cn_50k.txt"

get "$CANTO_URL" "$OUT/canto.zip"
unzip -p "$OUT/canto.zip.part" 'cccedict-canto-readings-*.txt' > "$OUT/cccedict-canto-readings.txt.part"
rm -f "$OUT/canto.zip.part"
need_lines "$OUT/cccedict-canto-readings.txt.part" 100000
mv -f "$OUT/cccedict-canto-readings.txt.part" "$OUT/cccedict-canto-readings.txt"

: > "$OUT/mozc-dict.tsv.part"
for i in 0 1 2 3 4 5 6 7 8 9; do
	get "$MOZC_RAW/dictionary0$i.txt" "$OUT/mozc-dict-$i"
	cat "$OUT/mozc-dict-$i.part" >> "$OUT/mozc-dict.tsv.part"
	rm -f "$OUT/mozc-dict-$i.part"
done
need_lines "$OUT/mozc-dict.tsv.part" 1000000
mv -f "$OUT/mozc-dict.tsv.part" "$OUT/mozc-dict.tsv"

get "$MOZC_RAW/connection_single_column.txt" "$OUT/mozc-connection.txt"
# First line is the POS count N, then N×N costs (2672² ≈ 7.1 M today).
need_lines "$OUT/mozc-connection.txt.part" 1000000
mv -f "$OUT/mozc-connection.txt.part" "$OUT/mozc-connection.txt"

echo "cjk sources in $OUT"
