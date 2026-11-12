#!/usr/bin/env python3
"""build-hinglish.py — Hindi text respelled in Latin letters, as Hindi is
typed in Latin ("Hinglish"): the corpus of the hi-Latn model.

Nobody spells Hindi in Latin one way (nahi, nahin, nai), so no single
romanization is right. The Dakshina dataset attests, for some 30 000
Hindi words, how people romanized each and how often
(hi/lexicons/hi.translit.sampled.{train,dev,test}.tsv: word,
romanization, count). Every word of the text that it attests is written
in one of those spellings, chosen in proportion to how often each was
typed, by a hash of the word and how many times it has come before, so
the same text always comes out the same. Any other Devanagari word is
spelled by rule: each letter its usual Latin, a vowel sign its vowel, the
vowel a consonant carries as `a`, but where Hindi does not speak it (at
a word's end, and between two consonants each with a spoken vowel
beside it: कमला, kamla). Danda is a full stop, Devanagari digits are
ASCII ones, the zero-width joiners Devanagari text uses inside words
are dropped, and everything else passes as it is.

    build-hinglish.py --lexicons DIR [--drop FILE] IN OUT

--drop names a file of lines (Dakshina's romanized test sentences, their
Devanagari side) left out of IN, so the held-out text is not trained on.
"""

import argparse
import collections
import re
import sys
import unicodedata
import zlib

CONSONANTS = {
    "क": "k", "ख": "kh", "ग": "g", "घ": "gh", "ङ": "n",
    "च": "ch", "छ": "chh", "ज": "j", "झ": "jh", "ञ": "n",
    "ट": "t", "ठ": "th", "ड": "d", "ढ": "dh", "ण": "n",
    "त": "t", "थ": "th", "द": "d", "ध": "dh", "न": "n",
    "प": "p", "फ": "ph", "ब": "b", "भ": "bh", "म": "m",
    "य": "y", "र": "r", "ल": "l", "ळ": "l", "व": "v",
    "श": "sh", "ष": "sh", "स": "s", "ह": "h",
}
# A consonant with a nukta: the sound it borrows.
NUKTA = {"क": "q", "ख": "kh", "ग": "gh", "ज": "z", "फ": "f", "ड": "d", "ढ": "dh", "य": "y"}
VOWELS = {
    "अ": "a", "आ": "aa", "इ": "i", "ई": "ee", "उ": "u", "ऊ": "oo", "ऋ": "ri",
    "ए": "e", "ऐ": "ai", "ओ": "o", "औ": "au", "ऍ": "e", "ऑ": "o",
}
SIGNS = {
    "ा": "a", "ि": "i", "ी": "i", "ु": "u", "ू": "u", "ृ": "ri",
    "े": "e", "ै": "ai", "ो": "o", "ौ": "au", "ॅ": "e", "ॉ": "o",
}
VIRAMA, NUKTA_SIGN = "्", "़"
NASAL = {"ं": "n", "ँ": "n", "ः": "h"}
DIGITS = {chr(0x0966 + i): str(i) for i in range(10)}
WORD = re.compile(r"[ऀ-ॣॱ-ॿ]+")


def by_rule(word):
    """A Devanagari word spelled in Latin by rule, its unspoken vowels left out."""
    # Units: (latin, kind), kind C for a consonant whose vowel is still the
    # one it carries, V for a vowel sound, X for anything else.
    units = []
    chars = list(word)
    i = 0
    while i < len(chars):
        c = chars[i]
        if c in CONSONANTS:
            latin = CONSONANTS[c]
            if i + 1 < len(chars) and chars[i + 1] == NUKTA_SIGN:
                latin = NUKTA.get(c, latin)
                i += 1
            nxt = chars[i + 1] if i + 1 < len(chars) else ""
            if nxt in SIGNS:
                units.append((latin, "c"))
                units.append((SIGNS[nxt], "V"))
                i += 2
                continue
            if nxt == VIRAMA:
                units.append((latin, "c"))
                i += 2
                continue
            units.append((latin, "c"))
            units.append(("a", "A"))  # the vowel it carries
        elif c in VOWELS:
            units.append((VOWELS[c], "V"))
        elif c in NASAL:
            units.append((NASAL[c], "X"))
        elif c in SIGNS:
            units.append((SIGNS[c], "V"))
        i += 1
    # A carried vowel is not spoken at the word's end, nor, from the right,
    # between two consonants that each have a spoken vowel on their other
    # side (V C a C V).
    if units and units[-1][1] == "A" and len(units) > 2:
        units.pop()
    spoken = [True] * len(units)
    for j in range(len(units) - 2, 1, -1):
        if units[j][1] != "A":
            continue
        before = units[j - 1][1] == "c" and j >= 2 and units[j - 2][1] in "VA" and spoken[j - 2]
        after = (
            j + 2 < len(units) and units[j + 1][1] == "c" and units[j + 2][1] in "VA"
            and spoken[j + 2]
        )
        if before and after:
            spoken[j] = False
    return "".join(latin for (latin, _), keep in zip(units, spoken) if keep)


def load_lexicons(directory):
    """Each word's romanizations, with how many typed each."""
    import pathlib

    attested = collections.defaultdict(collections.Counter)
    for path in sorted(pathlib.Path(directory).glob("hi.translit.sampled.*.tsv")):
        for line in path.read_text(encoding="utf-8").splitlines():
            parts = line.split("\t")
            if len(parts) < 2:
                continue
            word = unicodedata.normalize("NFC", parts[0])
            latin = parts[1].strip().lower()
            n = int(parts[2]) if len(parts) > 2 and parts[2].isdigit() else 1
            if latin and latin.isascii():
                attested[word][latin] += n
    if not attested:
        sys.exit(f"no romanizations in {directory}")
    return {w: sorted(c.items()) for w, c in attested.items()}


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--lexicons", required=True)
    ap.add_argument("--drop")
    ap.add_argument("input")
    ap.add_argument("output")
    opts = ap.parse_args()
    attested = load_lexicons(opts.lexicons)
    drop = set()
    if opts.drop:
        with open(opts.drop, encoding="utf-8") as f:
            drop = {unicodedata.normalize("NFC", line.strip()) for line in f}
    seen = collections.Counter()
    words = known = dropped = 0

    def spell(m):
        nonlocal words, known
        word = unicodedata.normalize("NFC", m.group(0))
        words += 1
        ways = attested.get(word)
        if not ways:
            return by_rule(word)
        known += 1
        total = sum(n for _, n in ways)
        k = seen[word]
        seen[word] += 1
        pick = zlib.crc32(f"{word}\t{k}".encode()) % total
        for latin, n in ways:
            if pick < n:
                return latin
            pick -= n
        return ways[-1][0]

    with open(opts.input, encoding="utf-8") as src, open(opts.output, "w", encoding="utf-8") as out:
        for line in src:
            line = line.rstrip("\n")
            if drop and unicodedata.normalize("NFC", line.strip()) in drop:
                dropped += 1
                continue
            line = WORD.sub(spell, line.replace("\u200c", "").replace("\u200d", ""))
            line = line.replace("।", ".").replace("॥", ".")
            line = "".join(DIGITS.get(c, c) for c in line)
            out.write(line + "\n")
    share = 100 * known / max(words, 1)
    print(f"{opts.output}: {words} words, {share:.1f}% spelled as attested, {dropped} lines dropped")


if __name__ == "__main__":
    main()
