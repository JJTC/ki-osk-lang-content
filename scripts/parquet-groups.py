#!/usr/bin/env python3
"""parquet-groups.py — some of a remote parquet file's row groups, not all.

    parquet-groups.py URL EVERY OUT
    parquet-groups.py --local FILE EVERY

Reads the file's footer and every EVERYth row group (the 0th, the EVERYth,
the 2×EVERYth…), each by an HTTP range request, into OUT at their own
offsets: a sparse file the size of the whole, which pyarrow reads those
row groups from as from the whole. fineweb-text.py's `every` must be the
same EVERY: a row group not read is zeros, and reading it fails. A big
language's first FineWeb-2 file is 4.8 GB, of which fineweb-text.py reads
a hundredth or less.

Prints the sha256 of the bytes read, the footer's and then each row
group's in order: what scripts/fetch-fineweb.sh pins for such a file.
With --local, the same sha256 of a file already here (such a sparse copy,
or the whole file), read from it alone.
"""

import hashlib
import os
import re
import sys
import urllib.request

import pyarrow.parquet as pq

AGENT = "ki-osk-lang-content (scripts/parquet-groups.py)"


def fetch(url, span):
    """The bytes of `url` that `span` (an HTTP Range: "0-99", "-8") names,
    and the whole's size."""
    req = urllib.request.Request(url, headers={"Range": f"bytes={span}", "User-Agent": AGENT})
    for attempt in range(5):
        try:
            with urllib.request.urlopen(req, timeout=120) as r:
                if r.status != 206:
                    sys.exit(f"parquet-groups: {url}: no range served (HTTP {r.status})")
                first, last, total = map(int, re.fullmatch(r"bytes (\d+)-(\d+)/(\d+)",
                                                           r.headers["Content-Range"]).groups())
                data = r.read()
            if len(data) != last - first + 1:
                raise OSError(f"{len(data)} bytes, not {last - first + 1}")
            return data, total
        except OSError as e:
            if attempt == 4:
                sys.exit(f"parquet-groups: {url}: {e}")


def groups(md, every):
    """Each row group read's span of bytes, start and end (excluded)."""
    for g in range(0, md.num_row_groups, every):
        rg = md.row_group(g)
        spans = []
        for i in range(rg.num_columns):
            c = rg.column(i)
            first = c.dictionary_page_offset if c.has_dictionary_page and c.dictionary_page_offset else c.data_page_offset
            spans.append((first, first + c.total_compressed_size))
        yield min(s for s, _ in spans), max(e for _, e in spans)


def footer_length(tail):
    if tail[4:] != b"PAR1":
        sys.exit("parquet-groups: not a parquet file")
    return int.from_bytes(tail[:4], "little")


def local(path, every):
    with open(path, "rb") as f:
        size = f.seek(0, 2)
        f.seek(size - 8)
        length = footer_length(f.read(8))
        f.seek(size - 8 - length)
        digest = hashlib.sha256(f.read(length + 8))
        for start, end in groups(pq.read_metadata(path), every):
            f.seek(start)
            digest.update(f.read(end - start))
    print(digest.hexdigest())


def remote(url, every, out):
    # The last 8 bytes: the footer's length and PAR1.
    tail, size = fetch(url, "-8")
    footer, _ = fetch(url, f"{size - 8 - footer_length(tail)}-{size - 1}")
    digest = hashlib.sha256(footer)
    with open(out, "wb") as f:
        f.truncate(size)
        f.seek(size - len(footer))
        f.write(footer)
        f.flush()
        md = pq.read_metadata(out)
        for start, end in groups(md, every):
            data, total = fetch(url, f"{start}-{end - 1}")
            if total != size:
                sys.exit(f"parquet-groups: {url} changed while it was read")
            f.seek(start)
            f.write(data)
            digest.update(data)
    print(digest.hexdigest())
    read = len(range(0, md.num_row_groups, every))
    print(f"parquet-groups: {os.path.basename(out)}: {read} of {md.num_row_groups} row groups", file=sys.stderr)


def main():
    if sys.argv[1] == "--local":
        local(sys.argv[2], int(sys.argv[3]))
    else:
        remote(sys.argv[1], int(sys.argv[2]), sys.argv[3])


if __name__ == "__main__":
    main()
