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
Urdu's ہے کا میں کو…, Persian's است را شده…, Indonesian's dengan untuk
adalah…, English's the and that…: words the language itself does not
use, or seldom) is left out, as scripts/fineweb-text.py leaves out such a
web page: a test wiki's writers paste their neighbour's articles in (the
Jambi Malay wiki's hundred one-line stubs of villages are Indonesian's
"… adalah Dusun di kecamatan …"); and so is a sentence more than 15% of
whose words are those, from an article that mixes the two. Neighbours
are named with commas between (`id,en`), their words counted together.

With --shapes K, a sentence is left out once K sentences of its shape
were kept, held out ones too: its four words after the first, or its
last four, digits aside, which a bot's stubs share (three in four of
the Rinconada Bikol wiki's sentences are its towns' "A … usad na
ikatlung klaseng banwaan sa probinsya kan …", "… agko di ragang 74.88
kilometro kwadrado"), and which a model would learn and an eval measure
as if they were the language.

With --sorani, Kurdish in the Arabic script is read as Sorani's keyboard
types it, as scripts/fineweb-text.py reads Sorani's web text: Arabic's
kaf and yeh, alef maksura and the doachashmee heh as ک ی ی ه, and a
word's last ه as ە but after ا (شاه) and in الله; and ە where the old
keyboards wrote ه and a zero-width non-joiner, U+200C (لە and جەنگ
written so: a tenth of the Southern Kurdish wiki's sentences), which
marks it.

With --script-only, a sentence is cut where a word with letters of
another script stands, the word left out, and each piece of three words
or more kept: the Persian and Armenian wikis' references write
`Wayback Machine`, `IMDb` and `displaystyle` into their sentences, a
tenth of the 80 000 commonest words, none of them typed on those
languages' keys.

    wikipedia-text.py [--cap N] [--eval N] [--holdout K] [--script NAME]
                      [--script-only] [--prefix P]
                      [--neighbour hi|ur|fa|id|en[,…]] [--shapes K]
                      [--sorani] OUT EVAL DUMP...

The dumps are read in the order given, the articles in dump order, so
the same dumps always give the same files.
"""

import argparse
import bz2
import collections
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
    "fa": set("است را شده شد بود هستند آنها برای".split()),
    "id": set("dengan untuk tidak adalah akan bahwa bahawa oleh kepada tersebut".split()),
    "en": set("the and that for with was are by this from which have".split()),
}
WORDS = re.compile(r"[\s.,;:!?।॥۔؟،\"'()\[\]{}<>«»“”‘’\-–—/|*#@&%$+=~`^]+")
DIGITS = re.compile(r"\d+")
# Sorani's letters as Arabic and Persian keyboards typed them, and a
# word's last ه: scripts/fineweb-text.py's SORANI_FOLD and FINAL_AE.
SORANI_FOLD = str.maketrans({"\u0643": "\u06A9", "\u064A": "\u06CC", "\u0649": "\u06CC", "\u06BE": "\u0647"})
FINAL_AE = re.compile("(?<=[\u0620-\u06FF])(?<!\u0627)(?<!\u0627\u0644\u0644)\u0647(?![\u0600-\u06FF])")


def sorani(text):
    """Kurdish text as Sorani's keyboard types it."""
    return FINAL_AE.sub("\u06D5", text.replace("\u0647\u200c", "\u06D5").translate(SORANI_FOLD))


def shapes(s):
    """A sentence's shapes: its four words after the first, and its last
    four, each digit run one 0."""
    ws = DIGITS.sub("0", s).split()
    return " ".join(ws[1:5]), " ".join(ws[-4:])


def neighbourly(text, vocab, share=0.06):
    ws = [w for w in WORDS.split(text) if w]
    return bool(ws) and sum(w in vocab for w in ws) / len(ws) > share


def in_script(s, script):
    letters = [c for c in s if c.isalpha()]
    own = sum(unicodedata.name(c, "").startswith(script) for c in letters)
    return 2 * own >= len(letters)


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


def sentences(text, script, neighbour=None, script_only=False):
    for s in END.split(STEERING.sub("", text)):
        s = " ".join(s.split())
        if MARKUP.search(s) or (neighbour and neighbourly(s, neighbour, 0.15)):
            continue
        for s in script_runs(s, script) if script_only and script else [s]:
            if len(s) <= 1000 and len(s.split()) >= 3 and (not script or in_script(s, script)):
                yield s


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--cap", type=int, default=400_000, help="sentences of corpus at most")
    ap.add_argument("--eval", type=int, default=500, help="held-out sentences written")
    ap.add_argument("--holdout", type=int, default=20, help="every K-th article held out")
    ap.add_argument("--script", help="keep sentences at least half of whose letters are of this script")
    ap.add_argument("--script-only", action="store_true",
                    help="with --script, cut a sentence where another script's word stands")
    ap.add_argument("--prefix", help="read only the articles whose title starts so (the Incubator's Wp/mag/)")
    ap.add_argument("--neighbour", help="leave out articles heavy with its words (several: id,en)")
    ap.add_argument("--shapes", type=int, help="at most K sentences of one shape (a bot's stubs)")
    ap.add_argument("--sorani", action="store_true", help="read Kurdish as Sorani's keyboard types it")
    ap.add_argument("out")
    ap.add_argument("eval_out")
    ap.add_argument("dumps", nargs="+")
    opts = ap.parse_args()
    seen = set()  # the corpus's sentences
    held = {}  # the held-out articles' sentences, in order, each once
    shaped = collections.Counter()  # sentences kept of each shape
    corpus = articles = neighbours = stubs = 0
    neighbour = None
    if opts.neighbour:
        names = opts.neighbour.split(",")
        unknown = [n for n in names if n not in NEIGHBOURS]
        if unknown:
            ap.error(f"--neighbour: no words for {', '.join(unknown)} (has {', '.join(NEIGHBOURS)})")
        neighbour = set().union(*(NEIGHBOURS[n] for n in names))

    def unlearned():
        return [s for s in held if s not in seen]

    def fresh(s):
        """Whether `s` is not one more of a shape K sentences were kept of;
        if so it counts towards its shapes."""
        nonlocal stubs
        if not opts.shapes:
            return True
        keys = shapes(s)
        if any(shaped[k] >= opts.shapes for k in keys):
            stubs += 1
            return False
        shaped.update(keys)
        return True

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
                    if opts.sorani:
                        text = sorani(text)
                    articles += 1
                    if articles % opts.holdout == 0:
                        for s in sentences(text, opts.script, neighbour, opts.script_only):
                            if s not in held and fresh(s):
                                held[s] = None
                        continue
                    for s in sentences(text, opts.script, neighbour, opts.script_only):
                        if corpus < opts.cap and s not in seen and fresh(s):
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
    left += f", {stubs} sentences left out as stubs" if opts.shapes else ""
    print(f"{opts.out}: {corpus} sentences from {articles} articles{left}, {len(kept)} held out")


if __name__ == "__main__":
    main()
