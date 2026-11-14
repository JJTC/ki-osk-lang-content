#!/usr/bin/env python3
"""wikipedia-text.py — a Wikipedia's articles as sentences, one a line:
what a language's model learns from.

The input is the wiki's CirrusSearch dump (`<wiki>_content-*.json.bz2`:
Elasticsearch bulk lines, for each article an index line and a document
line whose "text" is the article's plain text, no markup). Each
article's text is cut into sentences after a full stop, danda, question
or exclamation mark (Latin, Devanagari, Arabic, Ol Chiki, Meetei Mayek);
a sentence of fewer than three words, a run of more than 1 000
characters (a list, a table), or one with wiki markup in it
([[ ]] {{ }}, where a template failed: 9 000 of the Sanskrit wiki's
sentences) is left out, and with --script, one of whose letters fewer
than half are of that script (Unicode's name for it, as "OL CHIKI"): the
Santali wiki has whole articles in English. The marks that only steer
the direction of text (U+200E and U+200F, the embeddings and isolates)
and byte-order marks are dropped: Western Punjabi's articles put
right-to-left marks inside words (830 000 of them), which would make one
word many. The zero-width joiners stay, which spelling uses. A sentence
the corpus already has is not written again: wikis that grew from
templates repeat theirs (a quarter of the Meitei wiki's sentences, a
fifth of Konkani's, an eighth of Maithili's are such copies). Every
HOLDOUT-th article is held out of the corpus, and the first EVAL
sentences of the held-out articles are written apart, each once and none
the corpus has: the text eval-predict measures the model on, which it
never learned from (a template's sentence it learned would measure
nothing).

The Wikimedia Incubator's dump holds every test wiki, each article's
title starting with its prefix (`Wp/mag/`): with --prefix only those
articles are read. With --neighbour, an article more than 6% of whose
words are a big neighbour's own function words (Hindi's है का में को…,
Urdu's ہے کا میں کو…, words the language itself does not use) is left
out, as scripts/fineweb-text.py leaves out such a web page: a test wiki's
writers paste their neighbour's articles in; and so is a sentence more
than 15% of whose words are those, from an article that mixes the two.

    wikipedia-text.py [--cap N] [--eval N] [--holdout K] [--script NAME]
                      [--prefix P] [--neighbour hi|ur] OUT EVAL DUMP...

The dumps are read in the order given, the articles in dump order, so
the same dumps always give the same files.
"""

import argparse
import bz2
import json
import re
import sys
import unicodedata

END = re.compile(r"(?<=[.!?।॥۔؟᱾᱿꯫])\s+")
STEERING = re.compile("[\u200e\u200f\u202a-\u202e\u2066-\u2069\ufeff]")
MARKUP = re.compile(r"\[\[|\]\]|\{\{|\}\}")
# A big neighbour's own function words: scripts/fineweb-text.py's.
NEIGHBOURS = {
    "hi": set("है हैं था थी थे का की में को ने नहीं यह वह और लिए".split()),
    "ur": set("ہے ہیں تھا تھی تھے کا کی میں کو نے نہیں یہ وہ اور لیے کے سے".split()),
}
WORDS = re.compile(r"[\s.,;:!?।॥۔؟،\"'()\[\]{}<>«»“”‘’\-–—/|*#@&%$+=~`^]+")


def neighbourly(text, vocab, share=0.06):
    ws = [w for w in WORDS.split(text) if w]
    return bool(ws) and sum(w in vocab for w in ws) / len(ws) > share


def in_script(s, script):
    letters = [c for c in s if c.isalpha()]
    own = sum(unicodedata.name(c, "").startswith(script) for c in letters)
    return 2 * own >= len(letters)


def sentences(text, script, neighbour=None):
    for s in END.split(STEERING.sub("", text)):
        s = " ".join(s.split())
        if (
            len(s) <= 1000
            and len(s.split()) >= 3
            and not MARKUP.search(s)
            and (not script or in_script(s, script))
            and not (neighbour and neighbourly(s, neighbour, 0.15))
        ):
            yield s


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--cap", type=int, default=400_000, help="sentences of corpus at most")
    ap.add_argument("--eval", type=int, default=500, help="held-out sentences written")
    ap.add_argument("--holdout", type=int, default=20, help="every K-th article held out")
    ap.add_argument("--script", help="keep sentences at least half of whose letters are of this script")
    ap.add_argument("--prefix", help="read only the articles whose title starts so (the Incubator's Wp/mag/)")
    ap.add_argument("--neighbour", choices=sorted(NEIGHBOURS), help="leave out articles heavy with its words")
    ap.add_argument("out")
    ap.add_argument("eval_out")
    ap.add_argument("dumps", nargs="+")
    opts = ap.parse_args()
    seen = set()  # the corpus's sentences
    held = {}  # the held-out articles' sentences, in order, each once
    corpus = articles = neighbours = 0
    neighbour = NEIGHBOURS.get(opts.neighbour)

    def unlearned():
        return [s for s in held if s not in seen]

    with open(opts.out, "w", encoding="utf-8") as out:
        for path in opts.dumps:
            with bz2.open(path, "rt", encoding="utf-8") as dump:
                for line in dump:
                    doc = json.loads(line)
                    text = doc.get("text")
                    if not text or doc.get("namespace", 0) != 0:
                        continue
                    if opts.prefix and not doc.get("title", "").startswith(opts.prefix):
                        continue
                    if neighbour and neighbourly(text, neighbour):
                        neighbours += 1
                        continue
                    articles += 1
                    if articles % opts.holdout == 0:
                        for s in sentences(text, opts.script, neighbour):
                            held.setdefault(s, None)
                        continue
                    for s in sentences(text, opts.script, neighbour):
                        if corpus < opts.cap and s not in seen:
                            seen.add(s)
                            out.write(s + "\n")
                            corpus += 1
                    if corpus >= opts.cap and len(unlearned()) >= opts.eval:
                        break
    kept = unlearned()[: opts.eval]
    with open(opts.eval_out, "w", encoding="utf-8") as ev:
        for s in kept:
            ev.write(s + "\n")
    if corpus == 0 or not kept:
        sys.exit(f"wikipedia-text: {opts.dumps}: no text")
    left = f", {neighbours} more left out as the neighbour's" if neighbour else ""
    print(f"{opts.out}: {corpus} sentences from {articles} articles{left}, {len(kept)} held out")


if __name__ == "__main__":
    main()
