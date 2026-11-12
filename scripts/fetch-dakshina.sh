#!/usr/bin/env bash
# fetch-dakshina.sh — Wikipedia text and romanizations for twelve South
# Asian languages, from the Dakshina dataset (Google Research, v1.0,
# CC BY-SA 4.0; https://github.com/google-research-datasets/dakshina).
#
# The dataset is one 2.0 GB archive, pinned here by its sha256. It is read
# as it downloads, and only what is used is kept:
#
#   $OUT/<lang>.txt      the first $CAP lines of its filtered Wikipedia
#                        text (wiki-filt.train), the corpus build-predict
#                        adds to Tatoeba's for Hindi, Bengali and Marathi
#                        and uses alone for the other nine (`make
#                        predict-assets`), which sources.lock records
#   $OUT/<lang>/         its romanization lexicons and romanized sentences
#                        (lexicons/, romanized/), what `make eval-translit`
#                        measures the transliteration tables on
#   eval/<lang>.txt      for the nine with no Tatoeba corpus, the first
#                        $EVAL_CAP lines of the held-out Wikipedia text
#                        (wiki-filt.valid), what eval-predict measures
#                        their models on (committed)
#   eval/hi-Latn.txt     the first $EVAL_CAP of Hindi's romanized test
#                        sentences, as people typed them in Latin: what
#                        the Hinglish model is measured on (committed)
#
# The archive is not kept unless KEEP=<path> names where; a kept or given
# archive (TAR=<path>) is read instead of downloading.
#
#   scripts/fetch-dakshina.sh            # every language
#   scripts/fetch-dakshina.sh ta te      # some
#
# Env: OUT=$TMPDIR/ki-osk-sources/dakshina  CAP=300000  EVAL_CAP=500
#      EVAL_OUT=eval  TAR=  KEEP=
set -euo pipefail

URL=https://storage.googleapis.com/gresearch/dakshina/dakshina_dataset_v1.0.tar
SHA256=83530bec211893d9df429298f42e63c350c25ad56f9f5172f85edb0575ee5b03
ALL="bn gu hi kn ml mr pa sd si ta te ur"
# The languages Tatoeba has a corpus of (scripts/fetch-corpora.sh's MAP):
# their held-out text is Tatoeba's, not Wikipedia's.
TATOEBA="bn hi mr"

OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/dakshina}
CAP=${CAP:-300000}
EVAL_CAP=${EVAL_CAP:-500}
EVAL_OUT=${EVAL_OUT:-eval}
langs=${*:-$ALL}

for l in $langs; do
	case " $ALL " in *" $l "*) ;; *) echo "fetch-dakshina: no $l in the dataset" >&2; exit 2 ;; esac
done

mkdir -p "$OUT" "$EVAL_OUT"
work=$(mktemp -d "$OUT/.fetch.XXXXXX")
trap 'rm -rf "${work:?}"' EXIT

members=()
for l in $langs; do
	members+=("dakshina_dataset_v1.0/$l/lexicons/*" "dakshina_dataset_v1.0/$l/romanized/*"
		"dakshina_dataset_v1.0/$l/native_script_wikipedia/$l.wiki-filt.train.text.shuf.txt.gz"
		"dakshina_dataset_v1.0/$l/native_script_wikipedia/$l.wiki-filt.valid.text.shuf.txt.gz")
done

# The archive, hashed as it is read: tar takes what it needs, and nothing
# extracted is used unless the whole archive was the pinned one.
if [ -n "${TAR:-}" ]; then
	src=(cat "$TAR")
else
	src=(curl -fsSL --retry 5 --retry-all-errors "$URL")
fi
echo "dakshina: reading ${TAR:-$URL}"
keep=(); [ -n "${KEEP:-}" ] && keep=("$KEEP")
"${src[@]}" | tee "${keep[@]}" >(sha256sum | cut -d' ' -f1 > "$work/sha256") \
	| tar -x -C "$work" --wildcards "${members[@]}"
# tee's process substitutions finish after it; wait for the hash.
for _ in $(seq 100); do [ -s "$work/sha256" ] && break; sleep 0.1; done
got=$(cat "$work/sha256")
if [ "$got" != "$SHA256" ]; then
	echo "fetch-dakshina: the archive's sha256 is $got, not $SHA256" >&2
	exit 1
fi

for l in $langs; do
	d="$work/dakshina_dataset_v1.0/$l"
	# No pipefail for these two: head exiting at its cap stops zcat.
	(set +o pipefail; zcat "$d/native_script_wikipedia/$l.wiki-filt.train.text.shuf.txt.gz" | head -n "$CAP") > "$OUT/$l.txt.part"
	n=$(wc -l < "$OUT/$l.txt.part")
	[ "$n" -gt 0 ] || { echo "fetch-dakshina: $l: no text" >&2; exit 1; }
	mv -f "$OUT/$l.txt.part" "$OUT/$l.txt"
	rm -rf "${OUT:?}/$l"
	mkdir -p "$OUT/$l"
	mv "$d/lexicons" "$d/romanized" "$OUT/$l/"
	if [ "$l" = hi ]; then
		head -n "$EVAL_CAP" "$OUT/hi/romanized/hi.romanized.rejoined.test.roman.txt" > "$EVAL_OUT/hi-Latn.txt.part"
		mv -f "$EVAL_OUT/hi-Latn.txt.part" "$EVAL_OUT/hi-Latn.txt"
	fi
	case " $TATOEBA " in
		*" $l "*) held="(Tatoeba's held-out text kept)" ;;
		*)
			(set +o pipefail; zcat "$d/native_script_wikipedia/$l.wiki-filt.valid.text.shuf.txt.gz" | head -n "$EVAL_CAP") > "$EVAL_OUT/$l.txt.part"
			mv -f "$EVAL_OUT/$l.txt.part" "$EVAL_OUT/$l.txt"
			held="$(wc -l < "$EVAL_OUT/$l.txt") held out"
			;;
	esac
	printf '%-3s %d lines, %s\n' "$l" "$n" "$held"
done
