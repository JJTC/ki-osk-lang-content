#!/usr/bin/env python3
"""fineweb-text.py — a language's FineWeb-2 web text as sentences, one a
line: what its model learns from, for languages with no Wikipedia of
their own.

FineWeb-2 (Hugging Face; ODC-By 1.0, subject to Common Crawl's Terms of
Use) is web text from Common Crawl, sorted by language with the GlotLID
classifier, which errs most between close languages: its Dogri holds
Punjabi poetry, its Angika is Hindi news. So a document is kept only
when it reads as its language:

- a junk document is left out: one whose words are mostly single
  characters or digits (scanned books read by OCR), or whose letters are
  more than a fifth combining marks;
- so is one in which a big neighbour's own function words (Hindi's है का
  में को…, Urdu's ہے کا میں کو…, words the language itself does not use)
  are more than 6% of the words;
- and for a language with RIVAL words in the table below, one whose own
  function words (OWN) are not more than the rival's and Hindi's
  together, or are under 1% of its words (Dogri's ऐ गी कन्नै नेई against
  Punjabi's नूं तों विच नाल).

Sindhi in Devanagari (sd-Deva) shares में and का with Hindi, so Hindi's
other words mark a document as Hindi; and its implosives, written with
the letters Unicode has for them (ॻ ॼ ॾ ॿ) or, more often on the web,
as ग ज ड द ब with the underline ॒ (U+0952, after any vowel sign), are
read as those letters, which the keyboard types. Meitei in the Bengali
script (mni-Beng) has Bengali's function words (এবং করে হয়…) against it.

Each kept document is cut into sentences after a full stop, danda,
question or exclamation mark; a sentence of fewer than three words, of
more than 1 000 characters, with wiki markup, or fewer than half of whose
letters are the language's script is left out, and each sentence is
written once. The corpus is the `train` split's; the held-out text, at
most EVAL sentences of the `test` split (which FineWeb-2 keeps apart for
measuring) that the corpus does not have, is what eval-predict measures
the model on. Neither is committed: the text is the web's.

    fineweb-text.py --lang L [--eval N] OUT EVAL --train F.parquet... --test F.parquet...

Needs pyarrow. The files are read in the order given, so the same files
always give the same text.
"""

import argparse
import re
import sys
import unicodedata

import pyarrow.parquet as pq

END = re.compile(r"(?<=[.!?।॥۔؟])\s+")
STEERING = re.compile("[‎‏‪-‮⁦-⁩﻿]")
MARKUP = re.compile(r"\[\[|\]\]|\{\{|\}\}")
WORDS = re.compile(r"[\s.,;:!?।॥۔؟،\"'()\[\]{}<>«»“”‘’\-–—/|*#@&%$+=~`^]+")

HINDI = set("है हैं था थी थे का की में को ने नहीं यह वह और लिए".split())
URDU = set("ہے ہیں تھا تھی تھے کا کی میں کو نے نہیں یہ وہ اور لیے کے سے".split())
PUNJABI = set("नूं तों विच नाल वी सी लई हुण".split())
DOGRI = set("ऐ गी कन्नै कन्ने नेई बी लेई एह् ओह् च".split())
BENGALI = set("এবং করে হয় এই যে থেকে জন্য করা তার আমি আমাদের হয়েছে একটি সঙ্গে করেন হবে".split())

# Sindhi's implosives: the letter, or (on the web) its plain consonant
# with the underline after any vowel signs.
IMPLOSIVE = {"ग": "ॻ", "ज": "ॼ", "ड": "ॾ", "द": "ॾ", "ब": "ॿ"}
UNDERLINED = re.compile("([गजडदब])([\u0900-\u0903\u093a-\u094c\u094e\u094f\u0955-\u0957\u0962\u0963]*)\u0952")


def sindhi(text):
    text = UNDERLINED.sub(lambda m: IMPLOSIVE[m.group(1)] + m.group(2), text)
    return text.replace("\u0952", "")


# Each language: its script (Unicode's name for it), the neighbour whose
# function words mark a document as not its own, for a language that a
# rival is mistaken for, its own words and the rival's, and how its text
# is read where the web spells it two ways.
LANGS = {
    "brx": {"script": "DEVANAGARI", "neighbour": HINDI},
    "doi": {"script": "DEVANAGARI", "neighbour": HINDI, "own": DOGRI, "rival": PUNJABI},
    "hne": {"script": "DEVANAGARI", "neighbour": HINDI},
    "mag": {"script": "DEVANAGARI", "neighbour": HINDI},
    "brh": {"script": "ARABIC", "neighbour": URDU},
    "sd-Deva": {"script": "DEVANAGARI", "neighbour": HINDI - {"में", "का"}, "normalize": sindhi},
    "mni-Beng": {"script": "BENGALI", "neighbour": BENGALI},
}


def words(text):
    return [w for w in WORDS.split(text) if w]


def share(ws, vocab):
    return sum(w in vocab for w in ws) / len(ws) if ws else 0.0


def junk(text, ws, script):
    if not ws:
        return True
    if sum(len(w) == 1 or w.isdigit() for w in ws) > 0.5 * len(ws):
        return True
    if script != "ARABIC":
        # An abugida's vowel signs are marks: only Arabic's are suspect.
        return False
    letters = [c for c in text if c.isalpha() or unicodedata.category(c).startswith("M")]
    marks = sum(unicodedata.category(c).startswith("M") for c in letters)
    return marks > 0.2 * len(letters)


def kept(text, conf):
    ws = words(text)
    if junk(text, ws, conf["script"]):
        return False
    if share(ws, conf["neighbour"]) > 0.06:
        return False
    if "own" in conf:
        own = share(ws, conf["own"])
        if own < 0.01 or own <= share(ws, conf["rival"]) + share(ws, conf["neighbour"]):
            return False
    return True


def in_script(s, script):
    letters = [c for c in s if c.isalpha()]
    own = sum(unicodedata.name(c, "").startswith(script) for c in letters)
    return 2 * own >= len(letters)


def sentences(text, conf):
    script = conf["script"]
    text = STEERING.sub("", text)
    if "normalize" in conf:
        text = conf["normalize"](text)
    for s in END.split(text):
        s = " ".join(s.split())
        if len(s) <= 1000 and len(s.split()) >= 3 and not MARKUP.search(s) and in_script(s, script):
            yield s


def documents(paths):
    for path in paths:
        for batch in pq.ParquetFile(path).iter_batches(columns=["text"], batch_size=1000):
            yield from batch.column("text").to_pylist()


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--lang", required=True, choices=sorted(LANGS))
    ap.add_argument("--eval", type=int, default=500, help="held-out sentences written at most")
    ap.add_argument("out")
    ap.add_argument("eval_out")
    ap.add_argument("--train", nargs="+", required=True, help="the train split's parquet files")
    ap.add_argument("--test", nargs="+", required=True, help="the test split's parquet files")
    opts = ap.parse_args()
    train, test = opts.train, opts.test
    conf = LANGS[opts.lang]
    seen = set()
    docs = dropped = 0
    with open(opts.out, "w", encoding="utf-8") as out:
        for text in documents(train):
            docs += 1
            if not kept(text, conf):
                dropped += 1
                continue
            for s in sentences(text, conf):
                if s not in seen:
                    seen.add(s)
                    out.write(s + "\n")
    held = {}
    for text in documents(test):
        if kept(text, conf):
            for s in sentences(text, conf):
                if s not in seen:
                    held.setdefault(s, None)
    held = list(held)[: opts.eval]
    with open(opts.eval_out, "w", encoding="utf-8") as ev:
        for s in held:
            ev.write(s + "\n")
    if not seen or not held:
        sys.exit(f"fineweb-text: {opts.lang}: no text")
    print(f"{opts.out}: {len(seen)} sentences from {docs - dropped} of {docs} documents, {len(held)} held out")


if __name__ == "__main__":
    main()
