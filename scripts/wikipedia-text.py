#!/usr/bin/env python3
"""wikipedia-text.py — a Wikipedia's articles as sentences, one a line:
what a language's model learns from.

The input is the wiki's CirrusSearch dump (`<wiki>_content-*.json.bz2`:
Elasticsearch bulk lines, for each article an index line and a document
line whose "text" is the article's plain text, no markup). Each article's
text is cut into sentences after a full stop, danda, question or
exclamation mark (Latin, Devanagari, Arabic); a sentence of fewer than
three words, or a run of more than 1 000 characters (a list, a table), is
left out. The marks that only steer the direction of text (U+200E and
U+200F, the embeddings and isolates) and byte-order marks are dropped:
Western Punjabi's articles put right-to-left marks inside words (830 000
of them), which would make one word many. The zero-width joiners stay,
which spelling uses. Every HOLDOUT-th article is held out of the corpus, and the
first EVAL sentences of the held-out articles are written apart: the text
eval-predict measures the model on, which it never learned from.

    wikipedia-text.py [--cap N] [--eval N] [--holdout K] OUT EVAL DUMP...

The dumps are read in the order given, the articles in dump order, so
the same dumps always give the same files.
"""

import argparse
import bz2
import json
import re
import sys

END = re.compile(r"(?<=[.!?।॥۔؟])\s+")
STEERING = re.compile("[\u200e\u200f\u202a-\u202e\u2066-\u2069\ufeff]")


def sentences(text):
    for s in END.split(STEERING.sub("", text)):
        s = " ".join(s.split())
        if len(s) <= 1000 and len(s.split()) >= 3:
            yield s


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--cap", type=int, default=400_000, help="sentences of corpus at most")
    ap.add_argument("--eval", type=int, default=500, help="held-out sentences written")
    ap.add_argument("--holdout", type=int, default=20, help="every K-th article held out")
    ap.add_argument("out")
    ap.add_argument("eval_out")
    ap.add_argument("dumps", nargs="+")
    opts = ap.parse_args()
    corpus = held = articles = 0
    with open(opts.out, "w", encoding="utf-8") as out, open(opts.eval_out, "w", encoding="utf-8") as ev:
        for path in opts.dumps:
            with bz2.open(path, "rt", encoding="utf-8") as dump:
                for line in dump:
                    doc = json.loads(line)
                    text = doc.get("text")
                    if not text or doc.get("namespace", 0) != 0:
                        continue
                    articles += 1
                    if articles % opts.holdout == 0:
                        for s in sentences(text):
                            if held < opts.eval:
                                ev.write(s + "\n")
                                held += 1
                        continue
                    for s in sentences(text):
                        if corpus < opts.cap:
                            out.write(s + "\n")
                            corpus += 1
                    if corpus >= opts.cap and held >= opts.eval:
                        break
    if corpus == 0 or held == 0:
        sys.exit(f"wikipedia-text: {opts.dumps}: no text")
    print(f"{opts.out}: {corpus} sentences from {articles} articles, {held} held out")


if __name__ == "__main__":
    main()
