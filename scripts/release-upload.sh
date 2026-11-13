#!/usr/bin/env bash
# release-upload.sh — the release's assets onto its GitHub release: each
# file the release lacks, or holds with other bytes, a few at a time.
#
# GitHub limits how fast one token creates content, its "secondary" rate
# limits: about 80 requests a minute and 500 an hour, and each asset is a
# request. A release has an asset for every pack file, 526 for v0.12.0,
# whose upload GitHub refused at the 522nd. So a file the release already
# holds, the same size and digest, is skipped; the uploads are spaced
# PAUSE seconds apart; and a refused upload is tried again after a wait
# that doubles, from a minute to a quarter of an hour, until DEADLINE
# seconds have passed. Run again, it picks up where it stopped.
#
#   scripts/release-upload.sh TAG DIR
#
# Needs gh, with GH_TOKEN and GH_REPO. Env: PAUSE=1 WAIT=60 DEADLINE=9000
set -euo pipefail

[ $# -eq 2 ] || { echo "usage: release-upload.sh TAG DIR" >&2; exit 2; }
tag=$1
dir=$2
pause=${PAUSE:-1}
deadline=$(($(date +%s) + ${DEADLINE:-9000}))

# What the release holds: each asset's name, size, digest (sha256:…,
# where GitHub gives one) and state ("uploaded" once complete).
api=$(gh release view "$tag" --json apiUrl --jq .apiUrl)
held=$(gh api --paginate "${api#https://api.github.com/}/assets?per_page=100" \
	--jq '.[] | [.name, (.size | tostring), (.digest // ""), .state] | @tsv')

uploaded=0
skipped=0
for f in "$dir"/*; do
	[ -f "$f" ] || continue
	name=${f##*/}
	size=$(stat -c %s "$f")
	digest=sha256:$(sha256sum "$f" | cut -d' ' -f1)
	if line=$(awk -F'\t' -v n="$name" '$1 == n { print; found = 1 } END { exit !found }' <<<"$held"); then
		IFS=$'\t' read -r _ hsize hdigest hstate <<<"$line"
		if [ "$hstate" = uploaded ] && [ "$hsize" = "$size" ] &&
			{ [ -z "$hdigest" ] || [ "$hdigest" = "$digest" ]; }; then
			skipped=$((skipped + 1))
			continue
		fi
	fi
	wait=${WAIT:-60}
	until gh release upload "$tag" "$f" --clobber; do
		if [ "$(date +%s)" -ge "$deadline" ]; then
			echo "release-upload: out of time at $name ($uploaded uploaded, $skipped already there)" >&2
			exit 1
		fi
		echo "release-upload: $name refused; again in ${wait}s" >&2
		sleep "$wait"
		wait=$((wait * 2 > 900 ? 900 : wait * 2))
	done
	uploaded=$((uploaded + 1))
	sleep "$pause"
done
echo "release-upload: $uploaded uploaded, $skipped already there"
