#!/usr/bin/env bash
# fetch-corpora.sh — download per-language sentence corpora from Tatoeba
# for the prediction models, mapping ki-osk layout ids to ISO 639-3 codes.
#
# Tatoeba sentence data is CC BY 2.0 FR (attribution in NOTICES.md). Each
# export is `<iso3>_sentences.tsv.bz2` with columns id / lang / text; we
# keep column 3, cap at $CAP sentences, and write a headerless
# `corpora/<id>.txt` that `build-predict` (see `make predict-assets`)
# turns into `<id>.fst` (+ `.ngram.bin`).
#
# These corpora are NOT committed (they're large and fully reproducible
# from here); only the built .fst/.ngram.bin artifacts are. Run this, then
# `make predict-assets PREDICT_LANGS="…"` to regenerate a language.
#
#   scripts/fetch-corpora.sh [id ...]     # default: the whole map below
#
# Lives in the ki-osk-lang-content repository (moved 2026-08-17 with
# the data it produces); paths are relative to that repo's root.
#
# Env: CAP=120000  CACHE=/tmp/ki-osk-tatoeba  OUT=<corpora dir>
# NB: no `pipefail` — `head` closing the extract pipe early SIGPIPEs bzcat,
# which is expected, not an error.
set -eu

CAP=${CAP:-120000}
CACHE=${CACHE:-/tmp/ki-osk-tatoeba}
OUT=${OUT:-predict/corpora}
BASE=https://downloads.tatoeba.org/exports/per_language

# ki-osk model code (= a layout's `prediction_lang`, usually the same
# 2-letter code as the layout id) → Tatoeba ISO 639-3 code. The two differ
# only where a layout id is an ISO-3166 country code: `da` is the Danish
# (`dk`) layout, `cs` the Czech (`cz`) one. Latin/Cyrillic/Greek/Arabic/
# Hebrew/Devanagari/Bengali/Armenian/Georgian are all space-delimited, so
# build-predict's whitespace tokenizer handles them. `ku` uses kmr
# (Kurmanji, Latin) to match the Latin layout; `sr` uses srp (mixed
# script — Cyrillic input prefix-matches the Cyrillic entries).
MAP="
tr:tur ru:rus hu:hun eo:epo he:heb uk:ukr lt:lit fi:fin cs:ces fil:tgl
mk:mkd ar:ara da:dan sr:srp hy:hye el:ell ro:ron vi:vie sk:slk fa:pes
id:ind bg:bul no:nob hi:hin bn:ben be:bel az:aze is:isl ca:cat ug:uig
lv:lvs ku:kmr ka:kat hr:hrv et:est kab:kab
oc:oci gl:glg br:bre eu:eus
tt:tat mr:mar as:asm ms:zsm os:oss
"

iso_for() { for p in $MAP; do [ "${p%%:*}" = "$1" ] && { echo "${p##*:}"; return; }; done; }

mkdir -p "$CACHE" "$OUT"
ids=${*:-$(for p in $MAP; do echo "${p%%:*}"; done)}
for id in $ids; do
	iso=$(iso_for "$id")
	[ -n "$iso" ] || { echo "!! no ISO-639-3 mapping for '$id'" >&2; continue; }
	bz="$CACHE/$iso.bz2"
	[ -s "$bz" ] || curl -sSL --max-time 120 -o "$bz" "$BASE/$iso/${iso}_sentences.tsv.bz2"
	bzcat "$bz" | cut -f3 | grep -v '^$' | head -"$CAP" > "$OUT/$id.txt"
	printf '%-5s %s  %d lines\n' "$id" "$iso" "$(wc -l < "$OUT/$id.txt")"
done
