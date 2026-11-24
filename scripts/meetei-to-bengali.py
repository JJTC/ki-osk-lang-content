#!/usr/bin/env python3
"""meetei-to-bengali.py — Aksharantar's Meitei words in the Bengali script.

Aksharantar romanizes Meitei written in its own script, Meetei Mayek
(`$AKSHARANTAR_DIR/mni*.tsv`: word, romanization, count). Meitei is also
written in the Bengali script, which its table `translit/mni-Beng.txt`
types and nothing measured: no open data romanizes it. The two scripts
spell Meitei letter for letter, but the Bengali script has more letters
than Meetei Mayek for some of its sounds (স শ ষ for ꯁ, ত ট for ꯇ, ি ী for
ꯤ...) and writes a final consonant (lonsum) with or without its virama.
So each word is spelled every way its letters allow, and the spelling
that the Bengali-script text the model learns from (`fineweb/mni-Beng.txt`)
writes most often, at least MIN_COUNT times, is kept as the word; a word
no spelling of which the text has is left out, as one the lexicon lacks.

  scripts/meetei-to-bengali.py AKSHARANTAR_DIR FINEWEB_TEXT

Writes mni-Beng.tsv, mni-Beng.fit.tsv and mni-Beng.choose.tsv beside
mni.tsv, mni.fit.tsv and mni.choose.tsv in AKSHARANTAR_DIR, and says how
many words of each it kept.
"""
import collections
import itertools
import pathlib
import re
import sys

# Each Meetei Mayek letter's spellings in the Bengali script, the usual
# first.
CONSONANTS = {
    "ꯀ": "ক", "ꯁ": "স শ ষ", "ꯂ": "ল", "ꯃ": "ম", "ꯄ": "প", "ꯅ": "ন ণ",
    "ꯆ": "চ ছ", "ꯇ": "ত ট", "ꯈ": "খ", "ꯉ": "ঙ", "ꯊ": "থ ঠ", "ꯋ": "ৱ ব",
    "ꯌ": "য় য", "ꯍ": "হ", "ꯐ": "ফ", "ꯒ": "গ", "ꯓ": "ঝ", "ꯔ": "র",
    "ꯕ": "ব", "ꯖ": "জ য", "ꯗ": "দ ড", "ꯘ": "ঘ", "ꯙ": "ধ ঢ", "ꯚ": "ভ",
}
# A final consonant (lonsum): the letter, with or without its virama.
LONSUM = {
    "ꯛ": "ক ক্", "ꯜ": "ল ল্", "ꯝ": "ম ম্", "ꯞ": "প প্", "ꯟ": "ন ন্",
    "ꯠ": "ৎ ত ত্ ট", "ꯡ": "ং ঙ ঙ্", "ꯢ": "ই য়",
}
SIGNS = {
    "ꯣ": "ো", "ꯤ": "ি ী", "ꯥ": "া", "ꯦ": "ে", "ꯧ": "ৌ", "ꯨ": "ু ূ",
    "ꯩ": "ৈ", "ꯪ": "ং", "꯭": "্",
}
# A vowel standing alone: ꯑ with a vowel sign, or ꯏ and ꯎ.
INDEPENDENT = {
    "ꯑ": "অ", "ꯑꯥ": "আ", "ꯑꯤ": "ই ঈ", "ꯑꯨ": "উ ঊ", "ꯑꯦ": "এ", "ꯑꯣ": "ও",
    "ꯑꯩ": "ঐ", "ꯑꯧ": "ঔ", "ꯏ": "ই ঈ", "ꯎ": "উ ঊ",
}
MIN_COUNT = 3
MOST_SPELLINGS = 4096
BENGALI_WORD = re.compile(r"[ঀ-৿]+")


def pieces(word):
    """The word's letters, each with its Bengali spellings; None if it has
    a letter none of the tables knows."""
    out = []
    i = 0
    while i < len(word):
        pair = word[i:i + 2]
        if pair in INDEPENDENT:
            out.append(INDEPENDENT[pair].split())
            i += 2
            continue
        c = word[i]
        for table in (INDEPENDENT, CONSONANTS, LONSUM, SIGNS):
            if c in table:
                out.append(table[c].split())
                break
        else:
            if c == "꯬":  # the heavy tone's mark: not written in Bengali script
                i += 1
                continue
            return None
        i += 1
    return out


def spellings(word):
    p = pieces(word)
    if not p:
        return []
    n = 1
    for options in p:
        n *= len(options)
    if n > MOST_SPELLINGS:
        return []
    return ["".join(s) for s in itertools.product(*p)]


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__.split("\n\n")[2])
    adir, text = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
    counts = collections.Counter()
    with text.open(encoding="utf-8") as f:
        for line in f:
            counts.update(BENGALI_WORD.findall(line))
    for suffix in ("", ".fit", ".choose"):
        src = adir / f"mni{suffix}.tsv"
        kept = total = 0
        lines = []
        for line in src.read_text(encoding="utf-8").splitlines():
            parts = line.split("\t")
            if len(parts) != 3:
                continue
            total += 1
            found = [(counts[s], s) for s in spellings(parts[0]) if counts[s] >= MIN_COUNT]
            if not found:
                continue
            best = max(found)[1]
            lines.append(f"{best}\t{parts[1]}\t{parts[2]}")
            kept += 1
        out = adir / f"mni-Beng{suffix}.tsv"
        out.write_text("".join(l + "\n" for l in lines), encoding="utf-8")
        print(f"{out}: {kept} of {total} words spelled as the text spells them")


if __name__ == "__main__":
    main()
