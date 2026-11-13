#!/usr/bin/env bash
# release-upload.sh — the release's assets onto its GitHub release: each
# file the release lacks, or holds with other bytes, one a second.
#
# GitHub limits how fast one token creates content, its "secondary" rate
# limits (in general 80 requests a minute and 500 an hour), and each
# asset is a request: gh's own upload, several at a time, was refused at
# the 522nd of v0.12.0's 526 assets. And `gh release upload` looks a
# draft up afresh for every file, two to three seconds each. So the
# release is looked up once, and each file is one request to it, started
# at most one every PACE seconds. A file the release already holds, the
# same size and digest, is skipped; one it holds otherwise (other bytes,
# or an upload cut short) is deleted first. A refused upload is tried
# again after a wait that doubles, from a minute to a quarter of an hour,
# until DEADLINE seconds have passed. Run again, it picks up where it
# stopped.
#
#   scripts/release-upload.sh TAG DIR
#
# Needs gh, with GH_TOKEN and GH_REPO. Env: PACE=1 WAIT=60 DEADLINE=9000
set -euo pipefail

[ $# -eq 2 ] || { echo "usage: release-upload.sh TAG DIR" >&2; exit 2; }
tag=$1
dir=$2
pace_us=$(awk -v p="${PACE:-1}" 'BEGIN { printf "%d", p * 1000000 }')
deadline=$(($(date +%s) + ${DEADLINE:-9000}))

# The release, found once (a draft has no tag to find it by, so gh looks
# for it among the releases): where its assets are listed, and where they
# are uploaded to.
urls=$(gh release view "$tag" --json apiUrl,uploadUrl --jq '.apiUrl + " " + .uploadUrl')
api=${urls%% *}
api=${api#https://api.github.com/}
upload=${urls#* }
upload=${upload%%\{*}
repo=${api%%/releases/*}

# What the release holds: each asset's name, id, size, digest (sha256:…,
# or - where GitHub gives none) and state ("uploaded" once complete).
held() {
	gh api --paginate "$api/assets?per_page=100" \
		--jq '.[] | [.name, (.id | tostring), (.size | tostring), (.digest // "-"), .state] | @tsv'
}

# A content request (an upload, a deletion), started at most one every
# PACE seconds after the last.
next=0
paced() {
	local now us
	now=${EPOCHREALTIME//[!0-9]/}
	if [ "$now" -lt "$next" ]; then
		us=$((next - now))
		sleep "$((us / 1000000)).$(printf '%06d' $((us % 1000000)))"
	fi
	next=$((${EPOCHREALTIME//[!0-9]/} + pace_us))
	"$@"
}

list=$(held)
uploaded=0
skipped=0
for f in "$dir"/*; do
	[ -f "$f" ] || continue
	name=${f##*/}
	size=$(stat -c %s "$f")
	digest=sha256:$(sha256sum "$f" | cut -d' ' -f1)
	wait=${WAIT:-60}
	tried=false
	while :; do
		line=$(awk -F'\t' -v n="$name" '$1 == n { print; exit }' <<<"$list")
		if [ -n "$line" ]; then
			IFS=$'\t' read -r _ id hsize hdigest hstate <<<"$line"
			if [ "$hstate" = uploaded ] && [ "$hsize" = "$size" ] &&
				{ [ "$hdigest" = - ] || [ "$hdigest" = "$digest" ]; }; then
				# There already, or the last try got there after all.
				if $tried; then uploaded=$((uploaded + 1)); else skipped=$((skipped + 1)); fi
				break
			fi
			paced gh api -X DELETE "$repo/releases/assets/$id" --silent || true
		fi
		tried=true
		if paced gh api -X POST "$upload" -f name="$name" --input "$f" \
			-H 'Content-Type: application/octet-stream' --silent; then
			uploaded=$((uploaded + 1))
			break
		fi
		if [ "$(date +%s)" -ge "$deadline" ]; then
			echo "release-upload: out of time at $name ($uploaded uploaded, $skipped already there)" >&2
			exit 1
		fi
		echo "release-upload: $name refused; again in ${wait}s" >&2
		sleep "$wait"
		wait=$((wait * 2 > 900 ? 900 : wait * 2))
		list=$(held)
	done
done
echo "release-upload: $uploaded uploaded, $skipped already there"
