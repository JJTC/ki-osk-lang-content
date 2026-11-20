#!/usr/bin/env python3
"""storybook-text.py — a language's stories from the African Storybook as
sentences, one a line: what its model learns from, for languages that
have no other open text.

The African Storybook (africanstorybook.org, an initiative of Saide)
publishes picture books for children written and translated by teachers
and writers across Africa, each under a Creative Commons licence, CC BY
or CC BY-NC. The Global African Storybook Project's asp-raw-db
repository keeps every story's text and metadata as the site had them in
February 2023, one file a story: its language as the site names it
(Lumasaaba, Ng’aturkana), its licence, its author and translator, and its
pages. Only the CC BY stories (3.0 and 4.0) are read.

A page is left out where it writes English's or Swahili's own function
words, two or more and 3% of its words or more (words the four languages
read here do not write: not Masaaba's wewe or Turkana's it): a story
filed under one language now and then holds another's pages, one Masaaba
story the pages of a Swahili one and an English one. A page's sentences
end at its full stops and its line breaks (written @@), and a sentence
of one word is left out; each is kept once. Every 10th story is held
out: its sentences the corpus does not have are what eval-predict
measures the model on.

    storybook-text.py --lang myx --name Lumasaaba OUT EVAL_OUT DB.tar.gz

The database is read from its archive as GitHub serves it (scripts/
fetch-storybooks.sh pins it by sha256); `--letters` prints instead the
letters the language's stories write, with their counts, its keyboard's
source when it has too little text for a model.
"""

import argparse
import collections
import re
import sys
import tarfile
import unicodedata

END = re.compile(r"(?<=[.!?])\s+")
WORD = re.compile(r"[^\W\d_]+(?:['’][^\W\d_]+)*")
FIELD = re.compile(r"^(\w+):\t(.*)$", re.M)
PAGE = re.compile(r"^P\d+\t(.*)$", re.M)
# An apostrophe inside a word written ’: the ' its keyboard types.
INSIDE = re.compile(r"(?<=[^\W\d_])’(?=[^\W\d_])")

# English's and Swahili's commonest words that none of the languages read
# here writes.
RIVALS = set(
    "the and of to was were are with that this they his her said what there from have had not but"
    " for you my your".split()
) | set(
    "kwenye katika alisema akasema lakini hakuna kwamba sana pia hii hiyo huyo wakati kutoka mimi"
    " yeye sisi wao kama zaidi".split()
)


def stories(archive, names):
    """The CC BY stories filed under one of `names`, in the order of
    their ids: (id, pages)."""
    found = []
    with tarfile.open(archive) as tar:
        for member in tar:
            if not (member.isfile() and re.search(r"/data/\d+\.txt$", member.name)):
                continue
            text = tar.extractfile(member).read().decode("utf-8")
            head, _, rest = text.partition("page_text:")
            fields = dict(FIELD.findall(head))
            licence = fields.get("lic", "")
            if fields.get("lang", "").strip() not in names:
                continue
            if "Attribution" not in licence or "Non" in licence or "No Deriv" in licence:
                continue
            body = rest.split("\nimages:", 1)[0]
            pages = [INSIDE.sub("'", unicodedata.normalize("NFC", p.strip())) for p in PAGE.findall(body)]
            found.append((int(fields["id"]), [p for p in pages if p]))
    found.sort()
    return found


def rival(page):
    words = [w.lower() for w in WORD.findall(page)]
    hits = sum(w in RIVALS for w in words)
    return hits >= 2 and hits >= 0.03 * len(words)


def sentences(page):
    for line in page.split("@@"):
        for s in END.split(line):
            s = " ".join(s.split())
            if len(WORD.findall(s)) >= 2:
                yield s


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--lang", required=True, help="the language's tag, for messages")
    ap.add_argument("--name", action="append", required=True, help="its name in the database (repeat)")
    ap.add_argument("--holdout", type=int, default=10, help="every K-th story held out")
    ap.add_argument("--letters", action="store_true", help="print the letters its stories write instead")
    ap.add_argument("out", nargs="?")
    ap.add_argument("eval_out", nargs="?")
    ap.add_argument("archive")
    opts = ap.parse_args()
    found = stories(opts.archive, set(opts.name))
    if not found:
        sys.exit(f"storybook-text: no CC BY story of {opts.lang} ({', '.join(opts.name)})")
    if opts.letters:
        letters = collections.Counter()
        words = 0
        for _, pages in found:
            for page in pages:
                if rival(page):
                    continue
                for s in sentences(page):
                    for w in WORD.findall(s):
                        words += 1
                        letters.update(c for c in w if c.isalpha() or c in "'’")
        print(f"{opts.lang}: {len(found)} stories, {words} words")
        for c, n in letters.most_common():
            print(f"{c}\tU+{ord(c):04X}\t{n}\t{unicodedata.name(c, '?')}")
        return
    if not (opts.out and opts.eval_out):
        ap.error("OUT and EVAL_OUT are needed, but with --letters")
    seen = set()
    held = {}
    corpus = left = 0
    with open(opts.out, "w", encoding="utf-8") as out:
        for n, (_, pages) in enumerate(found, 1):
            for page in pages:
                if rival(page):
                    left += 1
                    continue
                for s in sentences(page):
                    if n % opts.holdout == 0:
                        held.setdefault(s, None)
                    elif s not in seen:
                        seen.add(s)
                        out.write(s + "\n")
                        corpus += 1
    kept = [s for s in held if s not in seen]
    with open(opts.eval_out, "w", encoding="utf-8") as ev:
        for s in kept:
            ev.write(s + "\n")
    if corpus == 0 or not kept:
        sys.exit(f"storybook-text: {opts.lang}: no text")
    print(f"{opts.out}: {corpus} sentences from {len(found)} stories, {left} pages left out as another language's, {len(kept)} held out")


if __name__ == "__main__":
    main()
