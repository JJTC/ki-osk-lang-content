#!/usr/bin/env bash
# fetch-greeklish.sh — Greek as people type it in Latin letters
# ("Greeklish"), word by word, to measure Greek's transliteration table:
# only `make eval-translit` reads it, and nothing of it is committed or
# shipped.
#
# It is the real-world test set of AUEB's "Still All Greeklish to Me:
# Greeklish to Greek Transliteration" (Toumazatos, Pavlopoulos,
# Androutsopoulos and Vassos, LREC-COLING 2024, which its authors ask to be
# cited; github.com/nlpaueb/greeklish, Apache-2.0): 339 lines of forum
# comments and chat messages typed in Greeklish, each written out in Greek
# by four or six people. Its files, at one commit, are pinned by sha256
# below. A line's words are paired with each person's Greek where the two
# have as many words, each person's pair weighing alike, so a word counts
# as often as the people who wrote it so; words left in Latin (names,
# English) are left out. It becomes
#
#   $OUT/el.tsv    the Greek word, its Greeklish and the weight,
#                  tab-separated, a pair a line: what eval-translit
#                  --pairs reads
#
# The files are kept in $OUT/raw, and a kept one whose sha256 is right is
# not downloaded again.
#
# Env: OUT=$TMPDIR/ki-osk-sources/greeklish
set -euo pipefail

REV=db2b6481e6b2da3b8fad95e39cf851cefd8616e9
BASE=https://raw.githubusercontent.com/nlpaueb/greeklish/$REV/data/real_world
# file  sha256
FILES="
greeklish/greeklish_gazzetta_accepted.txt  64963742655ebaa1c7cb8b2c355b3a09b7f8a331acec99ec977cb55014871e17
greek/greek_gazzetta_accepted.txt          93d0801e0e8a51fc68d4e4d652e43e180e5b8aa3af0b95524821fdab19211ebb
greeklish/greeklish_gazzetta_rejected.txt  0df8f4e77543c546699c9b2a33943a4494a4a03593e9f4fed6fc8de7f2d36b7b
greek/greek_gazzetta_rejected.txt          bbfc90bbe8ac8bbbb8e971d9b109b5f82038aa7f93457f53c903c6baf32557cf
greeklish/greeklish_survivorbot_msc.txt    0fe46f772624b3cdbb71f59ccd46bb1cc261b7e4e8daa59603c51c9321231ce9
greek/greek_survivorbot_msc.txt            7f5afe84853d7f0a7ef34c8b218c325ef3c6936c999695dbcf044cd731f9e988
"

# Each Greeklish file's lines, and its Greek file's: every person's
# version of all of them, one block after another. Words are runs of
# letters and digits, which Greeklish writes for letters (8a: θα, pai3ei:
# παίζει), and an apostrophe inside one; a pair is kept where the
# Greeklish word is Latin and the Greek one Greek.
PAIRS='
import re, sys, unicodedata
from collections import defaultdict
WORD = re.compile(r"[^\W_]+(?:[\x27’][^\W_]+)*")
def words(line):
    return WORD.findall(unicodedata.normalize("NFC", line))
def latin(w):
    return all(c.isascii() for c in w)
def greek(w):
    return all("GREEK" in unicodedata.name(c, "") for c in w if c.isalpha())
weights = defaultdict(float)
args = sys.argv[1:]
for gl_path, el_path in zip(args[::2], args[1::2]):
    gl = [l.rstrip("\n") for l in open(gl_path, encoding="utf-8")]
    el = [l.rstrip("\n") for l in open(el_path, encoding="utf-8")]
    n = len(gl)
    if n == 0 or len(el) % n:
        sys.exit(f"{el_path}: {len(el)} lines, not a whole number of versions of {n}")
    versions = [el[k * n:(k + 1) * n] for k in range(len(el) // n)]
    for i, line in enumerate(gl):
        typed = words(line)
        meant = [words(v[i]) for v in versions]
        meant = [m for m in meant if len(m) == len(typed)]
        for m in meant:
            for t, w in zip(typed, m):
                if latin(t) and greek(w) and t.lower() != w.lower():
                    weights[(w, t)] += 1 / len(meant)
for (w, t), n in sorted(weights.items()):
    print(f"{w}\t{t}\t{n:.4f}")
'

OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/greeklish}
mkdir -p "$OUT/raw"
args=()
while read -r file sha; do
	[ -n "$file" ] || continue
	local_file="$OUT/raw/${file##*/}"
	if [ ! -f "$local_file" ] || [ "$(sha256sum "$local_file" | cut -d' ' -f1)" != "$sha" ]; then
		curl -fsSL --retry 5 --retry-all-errors -o "$local_file.part" "$BASE/$file"
		got=$(sha256sum "$local_file.part" | cut -d' ' -f1)
		if [ "$got" != "$sha" ]; then
			rm -f "$local_file.part"
			echo "fetch-greeklish: $file's sha256 is $got, not $sha" >&2
			exit 1
		fi
		mv -f "$local_file.part" "$local_file"
	fi
	args+=("$local_file")
done <<<"$FILES"
python3 -I -c "$PAIRS" "${args[@]}" > "$OUT/el.tsv.part"
mv -f "$OUT/el.tsv.part" "$OUT/el.tsv"
echo "$OUT/el.tsv: $(wc -l < "$OUT/el.tsv") pairs"
