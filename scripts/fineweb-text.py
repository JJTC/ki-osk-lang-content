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
Lao, written without spaces between words, ends a sentence at a line's
end too, its sentences being as often unpunctuated, and a short one is
one of fewer than 15 letters rather than 3 words; FineWeb-2 has far more
of it than a model needs (780 million characters), so every 12th of its
row groups is read (`every`), 70 million characters. Its sentences are
cut where a word in another script stands (English names, mostly: half
the words of a model learned with them were English, each rare), and
its letters written two ways are written as the keyboard types them
(ໍາ as ຳ, ຫນ ຫມ ຫລ as ໜ ໝ ຫຼ; osk_data::wordbreak::lao_fold).
Kokborok (trp), written in Latin letters, has English against it, its
words counted in lower case, and a sentence of it whose words are more
than 15% English's (`sentence_rival`) is left out (39 of the 4 843 of
the documents kept). Mizo is read so too, every third row group of it
(100 MB in all). Khasi has Pnar (Jaintia) against it as well, which
FineWeb-2 files under Khasi (wyrta.com, two fifths of its documents):
Khasi writes ba jong bad kiba kaba, Pnar wa ya kiwa kawa ko wow.

Each kept document is cut into sentences after a full stop, danda,
question or exclamation mark; a sentence of fewer than three words, of
more than 1 000 characters, with wiki markup, or fewer than half of whose
letters are the language's script is left out, and each sentence is
written once. The corpus is the `train` split's; the held-out text, at
most EVAL sentences of the `test` split (which FineWeb-2 keeps apart for
measuring) that the corpus does not have, is what eval-predict measures
the model on. Neither is committed: the text is the web's.

Garhwali and Rajasthani come from MADLAD-400 (Google; Common Crawl's
text, audited by language), its JSON lines read the same way: a
document is kept where Hindi's function words are under 6% of its words
and its own (Garhwali's मा च छौ ह्वे कु…, Rajasthani's री सूं मांय रो…)
are at least 1% and more than Hindi's, and for Garhwali Nepali's too
(पनि भएको हुन्छ…: its casino spam shares मा and नि). With no test split, every 20th
document kept is held out instead (`--hold`).

    fineweb-text.py --lang L [--eval N] OUT EVAL --train F... [--test F... | --hold N]

Needs pyarrow for parquet. The files are read in the order given, so
the same files always give the same text.
"""

import argparse
import gzip
import json
import re
import sys
import unicodedata


END = re.compile(r"(?<=[.!?।॥۔؟])\s+")
STEERING = re.compile("[‎‏‪-‮⁦-⁩﻿]")
MARKUP = re.compile(r"\[\[|\]\]|\{\{|\}\}")
WORDS = re.compile(r"[\s.,;:!?।॥۔؟،\"'()\[\]{}<>«»“”‘’\-–—/|*#@&%$+=~`^]+")

HINDI = set("है हैं था थी थे का की में को ने नहीं यह वह और लिए".split())
URDU = set("ہے ہیں تھا تھی تھے کا کی میں کو نے نہیں یہ وہ اور لیے کے سے".split())
PUNJABI = set("नूं तों विच नाल वी सी लई हुण".split())
DOGRI = set("ऐ गी कन्नै कन्ने नेई बी लेई एह् ओह् च".split())
GARHWALI = set("मा च छौ छन छा ह्वे कु बि नि तैं".split())
# Nepali's, which Garhwali does not use (it shares मा and नि; its and is
# अर, Nepali's र), and the words of the Nepali casino spam MADLAD-400's
# Garhwali holds.
NEPALI = set("पनि भएको गरेको हुन्छ थियो गर्न छन् हुने गरी र छैन तपाईंको तपाइँको "
             "गर्नुहोस् मार्फत क्यासिनो कैसीनो बोनस".split())
RAJASTHANI = set("री सूं मांय रो रै नीं पण नै रा".split())
ENGLISH = set("the of and to in is that for on with as was are by this it from be at an or".split())
KOKBOROK = set("tei ni no bai khe bo kha kaisa tongo bini hwnwi wngkha twi nwng ang".split())
MIZO = set("chu leh hi kan te hian ah pawh tur chuan ni ang tak tih awm".split())
KHASI = set("ba jong bad kiba kaba uba ia ïa".split())
# Pnar (Jaintia), Khasi's neighbour, which FineWeb-2 files under Khasi.
PNAR = set("wa ya kiwa kawa uwa ko wow".split())
BENGALI = set("এবং করে হয় এই যে থেকে জন্য করা তার আমি আমাদের হয়েছে একটি সঙ্গে করেন হবে".split())

# Sindhi's implosives: the letter, or (on the web) its plain consonant
# with the underline after any vowel signs.
IMPLOSIVE = {"ग": "ॻ", "ज": "ॼ", "ड": "ॾ", "द": "ॾ", "ब": "ॿ"}
UNDERLINED = re.compile("([गजडदब])([\u0900-\u0903\u093a-\u094c\u094e\u094f\u0955-\u0957\u0962\u0963]*)\u0952")


def lao(text):
    return (text.replace("\u0ecd\u0eb2", "\u0eb3").replace("\u0eab\u0e99", "\u0edc")
            .replace("\u0eab\u0ea1", "\u0edd").replace("\u0eab\u0ea5", "\u0eab\u0ebc"))


def sindhi(text):
    text = UNDERLINED.sub(lambda m: IMPLOSIVE[m.group(1)] + m.group(2), text)
    return text.replace("\u0952", "")


# Each language: its script (Unicode's name for it), the neighbour whose
# function words mark a document as not its own, for a language that a
# rival is mistaken for, its own words and the rival's (or its own letter
# and the rival's, which it must write more often), and how its text is
# read where the web spells it two ways.
LANGS = {
    "brx": {"script": "DEVANAGARI", "neighbour": HINDI},
    "doi": {"script": "DEVANAGARI", "neighbour": HINDI, "own": DOGRI, "rival": PUNJABI},
    "hne": {"script": "DEVANAGARI", "neighbour": HINDI},
    "mag": {"script": "DEVANAGARI", "neighbour": HINDI},
    "brh": {"script": "ARABIC", "neighbour": URDU},
    "sd-Deva": {"script": "DEVANAGARI", "neighbour": HINDI - {"में", "का"}, "normalize": sindhi},
    "mni-Beng": {"script": "BENGALI", "neighbour": BENGALI},
    # Assamese shares Bengali's script and এই, যে, হয়; its r is ৰ where
    # Bengali's is র, which a document must write more often.
    "as": {"script": "BENGALI", "neighbour": BENGALI - {"এই", "যে", "হয়"}, "letters": ("ৰ", "র"),
           "every": 6},
    # MADLAD-400's (no test split: every 20th document held out).
    "gbm": {"script": "DEVANAGARI", "neighbour": HINDI, "own": GARHWALI, "rival": NEPALI},
    "raj": {"script": "DEVANAGARI", "neighbour": HINDI, "own": RAJASTHANI, "rival": set()},
    # Kokborok, in Latin letters: a document whose own words outnumber
    # English's, the English sentences in it left out.
    "trp": {"script": "LATIN", "neighbour": set(), "own": KOKBOROK, "rival": ENGLISH,
            "sentence_rival": ENGLISH},
    # Mizo, in Latin letters: as Kokborok; every 3rd row group.
    "lus": {"script": "LATIN", "neighbour": set(), "own": MIZO, "rival": ENGLISH,
            "sentence_rival": ENGLISH, "every": 3},
    # Khasi: a document whose own words outnumber Pnar's and English's.
    "kha": {"script": "LATIN", "neighbour": set(), "own": KHASI, "rival": PNAR | ENGLISH,
            "sentence_rival": ENGLISH},
    "lo": {"script": "LAO", "neighbour": set(), "lines": True, "unspaced": True, "every": 12,
           "normalize": lao, "script_only": True},
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
    # In lower case: Latin's function words start sentences too.
    ws = words(text.lower())
    if junk(text, ws, conf["script"]):
        return False
    if share(ws, conf["neighbour"]) > 0.06:
        return False
    if "letters" in conf:
        own, rival = conf["letters"]
        if text.count(own) <= text.count(rival):
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


def long_enough(s, conf):
    if conf.get("unspaced"):
        return sum(c.isalpha() for c in s) >= 15
    return len(s.split()) >= 3


def sentences(text, conf):
    script = conf["script"]
    text = STEERING.sub("", text)
    if "normalize" in conf:
        text = conf["normalize"](text)
    pieces = text.split("\n") if conf.get("lines") else [text]
    for piece in pieces:
        for s in END.split(piece):
            for s in script_runs(s, script) if conf.get("script_only") else [" ".join(s.split())]:
                if "sentence_rival" in conf and share(words(s.lower()), conf["sentence_rival"]) > 0.15:
                    continue
                if len(s) <= 1000 and long_enough(s, conf) and not MARKUP.search(s) and in_script(s, script):
                    yield s


def script_runs(s, script):
    """`s` cut where a word has letters of another script, those left out."""
    runs, run = [], []
    for w in s.split():
        if all(unicodedata.name(c, "").startswith(script) for c in w if c.isalpha()):
            run.append(w)
        elif run:
            runs.append(" ".join(run))
            run = []
    if run:
        runs.append(" ".join(run))
    return runs


def documents(paths, every=1):
    for path in paths:
        if path.endswith(".jsonl.gz"):
            # MADLAD-400's: a JSON object a line, its text in "text".
            with gzip.open(path, "rt", encoding="utf-8") as f:
                for line in f:
                    if line.strip():
                        # Its line breaks are written as the two
                        # characters \ and n.
                        yield json.loads(line)["text"].replace("\\n", "\n")
            continue
        import pyarrow.parquet as pq

        f = pq.ParquetFile(path)
        for g in range(0, f.metadata.num_row_groups, every):
            yield from f.read_row_group(g, columns=["text"]).column("text").to_pylist()


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--lang", required=True, choices=sorted(LANGS))
    ap.add_argument("--eval", type=int, default=500, help="held-out sentences written at most")
    ap.add_argument("out")
    ap.add_argument("eval_out")
    ap.add_argument("--train", nargs="+", required=True, help="the train split's files")
    ap.add_argument("--test", nargs="+", help="the test split's files")
    ap.add_argument("--hold", type=int, default=20,
                    help="with no test split, every HOLDth document kept is held out")
    opts = ap.parse_args()
    train, test = opts.train, opts.test or []
    conf = LANGS[opts.lang]
    seen = set()
    docs = dropped = 0
    held_docs = []
    with open(opts.out, "w", encoding="utf-8") as out:
        for text in documents(train, conf.get("every", 1)):
            docs += 1
            if not kept(text, conf):
                dropped += 1
                continue
            if not test and (docs - dropped) % opts.hold == 0:
                held_docs.append(text)
                continue
            for s in sentences(text, conf):
                if s not in seen:
                    seen.add(s)
                    out.write(s + "\n")
    held = {}
    for text in held_docs or (t for t in documents(test) if kept(t, conf)):
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
