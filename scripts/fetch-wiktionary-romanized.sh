#!/usr/bin/env bash
# fetch-wiktionary-romanized.sh — words and their romanizations, from
# Wiktionary as kaikki.org extracts it, to measure a transliteration table
# whose language has no romanizations people typed in open data, where
# Wiktionary's romanization is the one people type: Yiddish's, YIVO's
# (sholem: שלום). Only `make eval-translit` reads them: nothing of them
# is committed or shipped (Wiktionary is CC BY-SA 4.0).
#
#   $OUT/<lang>.tsv    the word, its romanization and 1, tab-separated,
#                      a pair a line, each pair once: what eval-translit
#                      --pairs reads
#
# An entry's romanizations are its forms tagged `romanization`, in the
# order of its forms (כלב, plural כלבים: kelev, klovem); an entry with more
# forms than romanizations gives only its headword's, the first. A word
# with a space, or a romanization with anything but ASCII letters and an
# apostrophe, is left out. kaikki.org keeps only its latest extract,
# rebuilt from each Wiktionary dump: a kept copy is read again unless
# REFRESH=1, and the sha256 of the one read is printed, as the measure
# moves with it.
#
#   scripts/fetch-wiktionary-romanized.sh        # every language (yi)
#
# Env: OUT=$TMPDIR/ki-osk-sources/wiktionary  REFRESH=
set -euo pipefail

KAIKKI=https://kaikki.org/dictionary
# language  kaikki.org's name
LANGS="
yi  Yiddish
"

PAIRS='
import gzip, json, re, sys, unicodedata
ok = re.compile(r"[a-z\x27]+")
seen = set()
def native(form):
    # "plural כלבים": the form without its label, which is in Latin letters
    words = [w for w in form.split() if not w.isascii()]
    return unicodedata.normalize("NFC", words[0]) if len(words) == 1 else None
for line in gzip.open(sys.argv[1], "rt", encoding="utf-8"):
    e = json.loads(line)
    forms = e.get("forms", [])
    can = [native(f["form"]) for f in forms if "canonical" in f.get("tags", [])]
    rom = [f["form"] for f in forms if f.get("tags") == ["romanization"]]
    if not rom:
        continue
    # the forms and their romanizations are in the same order when as
    # many; else only the first romanization is sure: the headword'\''s
    pairs = zip(can, rom) if len(can) == len(rom) else [(native(e["word"]), rom[0])]
    for word, latin in pairs:
        latin = latin.strip().lower().replace("’", "\x27")
        if word and ok.fullmatch(latin) and (word, latin) not in seen:
            seen.add((word, latin))
            print(f"{word}\t{latin}\t1")
'

OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/wiktionary}
langs=${*:-$(echo "$LANGS" | awk 'NF {print $1}')}
mkdir -p "$OUT/raw"
for lang in $langs; do
	name=$(echo "$LANGS" | awk -v l="$lang" '$1 == l {print $2}')
	[ -n "$name" ] || { echo "fetch-wiktionary-romanized: no $lang" >&2; exit 2; }
	file="$OUT/raw/kaikki-$lang.jsonl.gz"
	if [ ! -f "$file" ] || [ -n "${REFRESH:-}" ]; then
		curl -fsSL --retry 5 --retry-all-errors -o "$file.part" \
			"$KAIKKI/$name/kaikki.org-dictionary-$name.jsonl.gz"
		mv -f "$file.part" "$file"
	fi
	python3 -I -c "$PAIRS" "$file" > "$OUT/$lang.tsv.part"
	mv -f "$OUT/$lang.tsv.part" "$OUT/$lang.tsv"
	echo "$OUT/$lang.tsv: $(wc -l < "$OUT/$lang.tsv") pairs, from $(sha256sum "$file" | cut -c1-16)…"
done
