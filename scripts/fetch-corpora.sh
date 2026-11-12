#!/usr/bin/env bash
# fetch-corpora.sh — download per-language sentence corpora from Tatoeba
# for the prediction models, mapping ki-osk model codes to ISO 639-3.
#
# Tatoeba sentence data is CC BY 2.0 FR (attribution in NOTICES.md). Each
# export is `<iso3>_sentences.tsv.bz2` with columns id / lang / text; we
# keep column 3, cap at $CAP sentences (the first $CAP in id order), and
# write a headerless `predict/corpora/<id>.txt` that `build-predict` (see
# `make predict-assets`) turns into `<id>.fst` + `<id>.ngram.bin`.
#
# Every $HOLDOUT-th sentence by id is held out of training, and so is any
# sentence with the same text as one of them; the first $EVAL_CAP held out
# are written to `eval/<id>.txt` (committed), the text ki-osk's
# `toolbox eval-predict` measures keystroke savings on.
#
# The corpora are not committed. `sources.lock` records the sha256 of the
# corpus each committed model was built from, and every release attaches
# the exact corpora as an asset, so a model can always be traced back to
# the text it was trained on.
#
#   scripts/fetch-corpora.sh [id ...]     # default: the whole map below
#
# Env: CAP=120000  CACHE=/tmp/ki-osk-tatoeba  OUT=predict/corpora
#      HOLDOUT=20  EVAL_CAP=1500  EVAL_OUT=eval
#
# Failure is loud. Every language is attempted; any that fails is named at
# the end and the script exits 1. A download is only trusted after
# `curl -f` succeeded AND `bzip2 -t` passed, and an empty corpus is an
# error — before 2026-11-08 a 404 was cached as the .bz2, reused forever,
# and silently turned into an empty corpus while the script said "0 lines"
# and exited 0.
set -eu

CAP=${CAP:-120000}
CACHE=${CACHE:-/tmp/ki-osk-tatoeba}
OUT=${OUT:-predict/corpora}
HOLDOUT=${HOLDOUT:-20}
EVAL_CAP=${EVAL_CAP:-1500}
EVAL_OUT=${EVAL_OUT:-eval}
BASE=https://downloads.tatoeba.org/exports/per_language

# ki-osk model code (= a layout's `prediction_lang`, usually the same
# 2-letter code as the layout id) → Tatoeba ISO 639-3 code. The two differ
# only where a layout id is an ISO-3166 country code: `da` is the Danish
# (`dk`) layout, `cs` the Czech (`cz`) one. Latin/Cyrillic/Greek/Arabic/
# Hebrew/Devanagari/Bengali/Armenian/Georgian are all space-delimited, so
# build-predict's whitespace tokenizer handles them. Thai is not: its
# sentences are cut into words with its word list (wordbreak/th.txt,
# scripts/fetch-word-lists.sh). `ku` uses kmr
# (Kurmanji, Latin) to match the Latin layout; `sr` uses srp (mixed
# script — Cyrillic input prefix-matches the Cyrillic entries).
#
# Keep this in step with PREDICT_LANGS in the Makefile: a language there
# without an entry here cannot be refreshed.
MAP="
en:eng de:deu fr:fra es:spa it:ita nl:nld pt:por sv:swe pl:pol
tr:tur ru:rus hu:hun eo:epo he:heb uk:ukr lt:lit fi:fin cs:ces fil:tgl
mk:mkd ar:ara da:dan sr:srp hy:hye el:ell ro:ron vi:vie sk:slk fa:pes
id:ind bg:bul no:nob hi:hin bn:ben be:bel az:aze is:isl ca:cat ug:uig
lv:lvs ku:kmr ka:kat hr:hrv et:est kab:kab
oc:oci gl:glg br:bre eu:eus
tt:tat mr:mar as:asm ms:zsm os:oss
th:tha
"

iso_for() { for p in $MAP; do [ "${p%%:*}" = "$1" ] && { echo "${p##*:}"; return; }; done; }

# Download $1 to $2, keeping a valid cached copy when the server says it is
# unchanged. Returns non-zero unless $2 ends up a valid bzip2 file.
fetch() {
	local url=$1 dest=$2 part=$2.part
	rm -f "$part"
	# A cached file that is not valid bzip2 (an HTML error page, a
	# truncated transfer) is discarded rather than trusted.
	if [ -s "$dest" ] && ! bzip2 -t "$dest" 2>/dev/null; then
		echo "   discarding invalid cached $dest" >&2
		rm -f "$dest"
	fi
	if [ -s "$dest" ]; then
		# -z: only transfer if the server copy is newer than ours (-R
		# keeps the server's mtime on what we save, so this compares
		# like with like).
		curl -fsSL -R --retry 5 --retry-all-errors --max-time 600 \
			-z "$dest" -o "$part" "$url" || { rm -f "$part"; return 1; }
	else
		curl -fsSL -R --retry 5 --retry-all-errors --max-time 600 \
			-o "$part" "$url" || { rm -f "$part"; return 1; }
	fi
	if [ -s "$part" ]; then
		bzip2 -t "$part" 2>/dev/null || {
			echo "   $url: response is not a valid bzip2 stream" >&2
			rm -f "$part"; return 1; }
		mv -f "$part" "$dest"
	fi
	rm -f "$part"
	[ -s "$dest" ]
}

mkdir -p "$CACHE" "$OUT" "$EVAL_OUT"
ids=${*:-$(for p in $MAP; do echo "${p%%:*}"; done)}
failed=""
for id in $ids; do
	iso=$(iso_for "$id")
	if [ -z "$iso" ]; then
		echo "!! $id: no ISO-639-3 mapping in fetch-corpora.sh" >&2
		failed="$failed $id"; continue
	fi
	bz="$CACHE/$iso.bz2"
	if ! fetch "$BASE/$iso/${iso}_sentences.tsv.bz2" "$bz"; then
		echo "!! $id: download of $iso failed" >&2
		failed="$failed $id"; continue
	fi
	# No pipefail here on purpose: awk exiting at its cap SIGPIPEs bzcat,
	# which is expected. The archive was verified above, and the line
	# count below catches an empty result.
	#
	# Held out first: every $HOLDOUT-th sentence by Tatoeba id, the first
	# $EVAL_CAP of them, for eval-predict. The ids never change, so the
	# set is stable as Tatoeba grows.
	etmp="$EVAL_OUT/$id.txt.part"
	bzcat "$bz" | awk -F'\t' -v k="$HOLDOUT" -v cap="$EVAL_CAP" \
		'$3 != "" && $1 % k == 0 { print $3; if (++n == cap) exit }' > "$etmp" || true
	# Then the training text, which skips those ids AND any sentence with
	# the same text as a held-out one: Tatoeba repeats short sentences
	# under different ids, and evaluating on a sentence the model was
	# trained on would flatter it.
	tmp="$OUT/$id.txt.part"
	bzcat "$bz" | awk -F'\t' -v k="$HOLDOUT" -v cap="$CAP" -v held_out="$etmp" \
		'FILENAME == held_out { held[$0] = 1; next }
		 $3 != "" && $1 % k != 0 && !($3 in held) { print $3; if (++n == cap) exit }' \
		"$etmp" - > "$tmp" || true
	n=$(wc -l < "$tmp")
	if [ "$n" -eq 0 ]; then
		echo "!! $id: $iso export produced an empty corpus" >&2
		rm -f "$tmp" "$etmp"; failed="$failed $id"; continue
	fi
	mv -f "$tmp" "$OUT/$id.txt"
	mv -f "$etmp" "$EVAL_OUT/$id.txt"
	printf '%-5s %s  %d lines, %d held out\n' "$id" "$iso" "$n" "$(wc -l < "$EVAL_OUT/$id.txt")"
done

if [ -n "$failed" ]; then
	echo "fetch-corpora: FAILED for:$failed" >&2
	exit 1
fi
