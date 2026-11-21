#!/usr/bin/env python3
"""smol-text.py — a language's SMOL translations as sentences, one a line:
what its model learns from, for languages that have no other open text.

SMOL (Google Research, at Hugging Face as google/smol, CC BY 4.0) is
professional translations of English into low-resource languages:
SmolDoc, documents translated a sentence at a time (each row a document,
its sentences in `trgs`), and SmolSent, single sentences (`trg`). A
language's files are read whole: each translated sentence is a line, a
sentence of one word left out, each kept once. Every 10th document (and
every 10th SmolSent sentence) is held out: its sentences the corpus does
not have are what eval-predict measures the model on.

The text is read as the keyboard types it: NFC; an apostrophe written ’
or ‘ between letters (Kurukh's र’ईर) as '; in Devanagari, no zero-width
joiner, non-joiner or space, which the translations scatter between
letters and at words' ends and Hindi's keys do not type; and Garo's
glottal stop, its raised dot, written ∙ or • as ·.

    smol-text.py --lang kru OUT EVAL_OUT smoldoc.en_kru.jsonl [more.jsonl…]

`--letters` prints instead the letters a language's translations write,
with their counts: its keyboard's source when it has too little text for
a model.
"""

import argparse
import collections
import json
import re
import sys
import unicodedata

# Apostrophes and Garo's raised dot written between two letters (a mark
# counts as its letter's).
APOSTROPHES = "’‘"
GARO_DOTS = "∙•"
DEVANAGARI_LANGS = {"ahr", "bfy", "bgq", "bns", "kfy", "kru", "mjl", "noe", "sgj", "sjp", "unr", "xnr"}
JOINERS = re.compile("[​‌‍﻿]")


def is_letter(c):
    return unicodedata.category(c)[0] in "LM"


def between_letters(text, chars, to):
    out = list(text)
    for i, c in enumerate(text):
        if c in chars and 0 < i < len(text) - 1 and is_letter(text[i - 1]) and is_letter(text[i + 1]):
            out[i] = to
    return "".join(out)


def clean(lang, sentence):
    s = unicodedata.normalize("NFC", sentence)
    if lang in DEVANAGARI_LANGS:
        s = JOINERS.sub("", s)
    s = between_letters(s, APOSTROPHES, "'")
    if lang == "grt":
        s = between_letters(s, GARO_DOTS, "·")
    return " ".join(s.split())


def words(sentence):
    """The whitespace-separated tokens with a letter in them."""
    return [t for t in sentence.split() if any(unicodedata.category(c)[0] == "L" for c in t)]


def units(path):
    """A file's documents (SmolDoc) or sentences (SmolSent), each a list
    of its sentences, in the file's order."""
    with open(path, encoding="utf-8") as f:
        for line in f:
            row = json.loads(line)
            yield row["trgs"] if "trgs" in row else [row["trg"]]


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--lang", required=True, help="the language's tag")
    ap.add_argument("--holdout", type=int, default=10, help="every K-th document held out")
    ap.add_argument("--letters", action="store_true", help="print the letters its translations write instead")
    ap.add_argument("paths", nargs="+", help="OUT EVAL_OUT FILE…, or FILE… with --letters")
    opts = ap.parse_args()
    if opts.letters:
        letters = collections.Counter()
        count = 0
        for path in opts.paths:
            for unit in units(path):
                for s in unit:
                    for w in words(clean(opts.lang, s)):
                        count += 1
                        letters.update(c for c in w.lower() if is_letter(c) or c in "'·")
        print(f"{opts.lang}: {count} words")
        for c, n in letters.most_common():
            print(f"{c}\tU+{ord(c):04X}\t{n}\t{unicodedata.name(c, '?')}")
        return
    if len(opts.paths) < 3:
        ap.error("OUT, EVAL_OUT and a file at least")
    out_path, eval_path, files = opts.paths[0], opts.paths[1], opts.paths[2:]
    seen = set()
    held = {}
    corpus = docs = 0
    with open(out_path, "w", encoding="utf-8") as out:
        for path in files:
            for n, unit in enumerate(units(path), 1):
                docs += 1
                for s in unit:
                    s = clean(opts.lang, s)
                    if len(words(s)) < 2:
                        continue
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
        sys.exit(f"smol-text: {opts.lang}: no text")
    print(f"{out_path}: {corpus} sentences from {docs} documents and sentences, {len(kept)} held out")


if __name__ == "__main__":
    main()
