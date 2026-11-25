#!/usr/bin/env python3
"""ebible-text.py — a Bible's verses as sentences, one a line: what a
language's model learns from, for languages whose keyboards have no other
open text, and beside the web text of one that has little (Gamo's).

eBible.org publishes Bible translations as plain text: `<id>_vpl.txt`,
one verse a line, its book's code, chapter and verse first (`GEN 1:1
Waaqi jalqaba…`), the text alone (no notes, headings or verse numbers in
it). Each chapter's verses are read as running text, cut into sentences
after a full stop, question or exclamation mark (and the closing quotes
after one), a sentence of one word left out, each kept once. Every 10th
chapter is held out: its sentences the corpus does not have are what
eval-predict measures the model on.

The text is read as the keyboard types it: NFC (Yaka's Bible writes its
accents as combining marks, which its keys type precomposed: á); an
apostrophe written ’ or ‘ between letters as '; footnote marks (`*`)
and the brackets round words some translations doubt (`[` `]`) left
out.

A language's Bibles, when it has more than one (Gamo's Bible and its New
Testament, two translations), are read in turn into one corpus, each
sentence kept once, every 10th chapter of each held out.

    ebible-text.py OUT EVAL_OUT gax_vpl.txt
    ebible-text.py OUT EVAL_OUT gmvl_vpl.txt gmvggm_vpl.txt

`--letters` prints instead the letters a Bible writes, with their counts.
"""

import argparse
import collections
import re
import sys
import unicodedata

APOSTROPHES = "’‘"
DROPPED = str.maketrans("", "", "*[]")
VERSE = re.compile(r"^([0-9A-Z]{3}) (\d+):(\d+) (.*)$")
# A sentence ends after . ! or ? and any closing quotes after it.
END = re.compile(r"(?<=[.!?])[»›”\"’']*\s+")


def is_letter(c):
    return unicodedata.category(c)[0] in "LM"


def between_letters(text, chars, to):
    out = list(text)
    for i, c in enumerate(text):
        if c in chars and 0 < i < len(text) - 1 and is_letter(text[i - 1]) and is_letter(text[i + 1]):
            out[i] = to
    return "".join(out)


def clean(text):
    s = unicodedata.normalize("NFC", text).translate(DROPPED)
    s = between_letters(s, APOSTROPHES, "'")
    return " ".join(s.split())


def words(sentence):
    """The whitespace-separated tokens with a letter in them."""
    return [t for t in sentence.split() if any(unicodedata.category(c)[0] == "L" for c in t)]


def chapters(path):
    """The file's chapters, in its order, each its verses' text joined."""
    current, verses = None, []
    with open(path, encoding="utf-8") as f:
        for line in f:
            m = VERSE.match(line.rstrip("\n"))
            if not m:
                continue
            key = (m.group(1), m.group(2))
            if key != current and verses:
                yield " ".join(verses)
                verses = []
            current = key
            verses.append(m.group(4))
    if verses:
        yield " ".join(verses)


def sentences(chapter):
    for s in END.split(clean(chapter)):
        s = s.strip()
        if len(words(s)) >= 2:
            yield s


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--holdout", type=int, default=10, help="every K-th chapter held out")
    ap.add_argument("--letters", action="store_true", help="print the letters the Bible writes instead")
    ap.add_argument("paths", nargs="+", help="OUT EVAL_OUT VPL..., or VPL with --letters")
    opts = ap.parse_args()
    if opts.letters:
        letters = collections.Counter()
        count = 0
        for path in opts.paths:
            for chapter in chapters(path):
                for s in sentences(chapter):
                    for w in words(s):
                        count += 1
                        letters.update(c for c in w.lower() if is_letter(c) or c == "'")
        print(f"{count} words")
        for c, n in letters.most_common():
            print(f"{c}\tU+{ord(c):04X}\t{n}\t{unicodedata.name(c, '?')}")
        return
    if len(opts.paths) < 3:
        ap.error("OUT, EVAL_OUT and the Bibles' _vpl.txt")
    out_path, eval_path, *vpls = opts.paths
    seen = set()
    held = {}
    corpus = count = 0
    with open(out_path, "w", encoding="utf-8") as out:
        for vpl in vpls:
            for n, chapter in enumerate(chapters(vpl), 1):
                count += 1
                for s in sentences(chapter):
                    if n % opts.holdout == 0:
                        held.setdefault(s, None)
                    elif s not in seen:
                        seen.add(s)
                        out.write(s + "\n")
                        corpus += 1
    kept = [s for s in held if s not in seen]
    with open(eval_path, "w", encoding="utf-8") as ev:
        for s in kept:
            ev.write(s + "\n")
    if corpus == 0 or not kept:
        sys.exit(f"ebible-text: {' '.join(vpls)}: no text")
    print(f"{out_path}: {corpus} sentences from {count} chapters, {len(kept)} held out")


if __name__ == "__main__":
    main()
