#!/usr/bin/env python3
"""dakshina-halves.py — Dakshina's romanized sentences, cut into its two
halves.

The dataset romanizes 10 000 Wikipedia sentences a language, half of them
its dev set and half its test set (romanized/<lang>.romanized.rejoined.
{dev,test}.native.txt), but its file of them aligned word by word
(<lang>.romanized.rejoined.aligned.cased_nopunct.tsv, a word and its
romanization a line, </s> after each sentence) holds them all, in the
order of <lang>.romanized.rejoined.tsv. This writes each half's aligned
sentences beside it,

  <lang>.romanized.rejoined.aligned.cased_nopunct.dev.tsv
  <lang>.romanized.rejoined.aligned.cased_nopunct.test.tsv

so that a table's costs can be fitted on the one (fit-translit) and
measured on the other (eval-translit). A sentence found in both halves
(a few dozen: Wikipedia repeats itself) goes with the test half, so that
no sentence measured on was fitted on.

  scripts/dakshina-halves.py DIR LANG...    # DIR: fetch-dakshina's OUT
"""

import os
import sys
import unicodedata


def sentences(path):
    """The aligned file's sentences, each its lines."""
    out, cur = [], []
    with open(path, encoding="utf-8") as f:
        for line in f:
            cur.append(line)
            if line.split("\t", 1)[0].strip() == "</s>":
                out.append(cur)
                cur = []
    if cur:
        sys.exit(f"{path}: a sentence with no </s> after it")
    return out


def letters(text):
    """What of a sentence the aligned file keeps: its letters and digits."""
    return "".join(
        c for c in unicodedata.normalize("NFC", text)
        if unicodedata.category(c)[0] in "LMN"
    )


def halves(d, lang):
    base = os.path.join(d, lang, "romanized", f"{lang}.romanized.rejoined")
    rows = []
    with open(base + ".tsv", encoding="utf-8") as f:
        for line in f:
            rows.append(line.rstrip("\n").split("\t")[0])
    with open(base + ".dev.native.txt", encoding="utf-8") as f:
        dev = {line.rstrip("\n") for line in f}
    with open(base + ".test.native.txt", encoding="utf-8") as f:
        test = {line.rstrip("\n") for line in f}
    aligned = base + ".aligned.cased_nopunct.tsv"
    sents = sentences(aligned)
    if len(sents) != len(rows):
        sys.exit(f"{lang}: {len(sents)} aligned sentences, {len(rows)} rows")
    out = {"dev": [], "test": []}
    unsure = 0
    for row, sent in zip(rows, sents):
        words = "".join(letters(line.split("\t", 1)[0]) for line in sent[:-1])
        if words != letters(row):
            # The aligned file drops a word now and then; its letters stay
            # in the row's order.
            it = iter(letters(row))
            if not all(c in it for c in words):
                sys.exit(f"{lang}: aligned sentence {words[:40]!r} is not row {row[:40]!r}")
            unsure += 1
        half = "test" if row in test else "dev" if row in dev else None
        if half is None:
            sys.exit(f"{lang}: {row[:40]!r} in neither half")
        out[half].append(sent)
    for half, sents_of in out.items():
        path = f"{aligned[:-len('.tsv')]}.{half}.tsv"
        with open(path + ".part", "w", encoding="utf-8") as f:
            for sent in sents_of:
                f.writelines(sent)
        os.replace(path + ".part", path)
    print(f"{lang}: {len(out['dev'])} dev and {len(out['test'])} test sentences"
          + (f" ({unsure} with a word the alignment dropped)" if unsure else ""))


def main():
    if len(sys.argv) < 3:
        sys.exit(__doc__.strip().splitlines()[-1].strip())
    for lang in sys.argv[2:]:
        halves(sys.argv[1], lang)


if __name__ == "__main__":
    main()
