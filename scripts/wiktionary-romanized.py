#!/usr/bin/env python3
"""wiktionary-romanized.py — words and their romanizations from a
language's Wiktionary extract (kaikki.org's), a pair a line, for
eval-translit --pairs: the word, its romanization and 1, tab-separated,
each pair once.

An entry's romanizations are its forms tagged `romanization`, in the
order of its forms (כלב, plural כלבים: kelev, klovem); an entry with more
forms than romanizations gives only its headword's, the first. With
--headword, only the headword and its first romanization are taken (the
forms of the other languages' entries carry the vowel marks their
writing leaves out, زُوج for زوج), and with --last its last: Persian's
entries give the Classical one first and the Iranian, which Finglish
follows, last (rušd, rošd). With --script, only words of that script
(Unicode's name for it): Persian's extract has Judeo-Persian words in the
Hebrew script, Mongolian's the traditional script's.

Yiddish's romanization is YIVO's, the one people type. The others'
romanizations are scholarly, with letters no keyboard has: --fold maps
them to the Latin letters the language's table (translit/<lang>.txt)
reads, as its header says they are typed (Armenian's ղ, ġ, is gh;
Persian's x, kh; Mongolian's ж, ž, is j). The pairs then measure the
table on Wiktionary's words spelled as the table expects, the folds'
choices among them, not as people spell them: how often the table and
the lexicon find a word, not whether people type it so.

A word with a space, or a romanization left with anything but ASCII
letters and an apostrophe, is left out.

    wiktionary-romanized.py [--fold LANG] [--headword [--last]] [--script NAME]
                            EXTRACT.jsonl.gz
"""

import argparse
import gzip
import json
import re
import sys
import unicodedata

# What each language's scholarly romanization becomes, in order (a
# replacement sees what the ones before it wrote), on its NFD form in
# lower case, so that a letter with a mark is the letter and its mark;
# marks left after are dropped, and capitals written on the way (CH for
# Armenian's č, which its c → ts must not touch) lowered.
FOLDS = {
    # Armenian (Wiktionary's: ʻ for aspiration, c ծ, cʻ ց, j ձ, č ճ, čʻ
    # չ, ǰ ջ, ġ ղ, x խ, ṙ ռ, ə ը, ē է, ō օ) to ts dz ch j gh kh.
    "hy": [("ʻ", ""), ("č", "CH"), ("ǰ", "J"), ("c", "ts"), ("j", "dz"),
           ("ġ", "gh"), ("x", "kh"), ("š", "sh"), ("ž", "zh"), ("ə", "e")],
    # Persian (â ā long a, x خ, ġ and q غ ق, š č ž, ' for ع and ء) to
    # Finglish's kh gh sh ch zh.
    "fa": [("x", "kh"), ("q", "gh"), ("ġ", "gh"), ("š", "sh"), ("č", "ch"),
           ("ž", "zh"), ("'", ""), ("ʼ", ""), ("ʿ", "")],
    # Pashto (x̌ or ṣ̌ ښ, ǵ or ẓ̌ ږ, ś څ, ź ځ, ǧ غ, x خ, the retroflexes with
    # a dot, ə and ë its schwa, accents for stress) to sh zh ts dz gh kh.
    "ps": [("x̌", "sh"), ("ṣ̌", "sh"), ("ẓ̌", "zh"), ("ǵ", "zh"),
           ("ś", "ts"), ("ź", "dz"), ("ǧ", "gh"), ("x", "kh"), ("č", "ch"),
           ("š", "sh"), ("ž", "zh"), ("ë", "a"), ("ə", "a")],
    # Mongolian (j for я ю е ё's y, ž ж, č ч, š ш, ö ө, ü ү, ʹ ь, ʺ ъ) to
    # y j ch sh o u, as its table reads them (x х and c ц stay).
    "mn": [("j", "y"), ("ž", "j"), ("č", "ch"), ("š", "sh"), ("ʹ", ""), ("ʺ", "")],
    # Santali (ô ᱚ, ă ᱟᱹ, ’ a checked consonant, ṅ ᱝ, ñ ᱧ, the retroflexes
    # with a dot, a tilde for nasalization) to o a ' ng ny, the tilde as n.
    "sat": [("’", "'"), ("ṅ", "ng"), ("ñ", "ny"), ("̃", "n")],
}
OK = re.compile(r"[a-z']+")


def fold(latin, lang):
    s = unicodedata.normalize("NFD", latin.strip().lower())
    for a, b in FOLDS.get(lang, []):
        s = s.replace(a, b)
    s = "".join(c for c in s if not unicodedata.combining(c))
    return unicodedata.normalize("NFC", s).lower().replace("’", "'")


def native(form):
    # "plural כלבים": the form without its label, which is in Latin letters
    words = [w for w in form.split() if not w.isascii()]
    return unicodedata.normalize("NFC", words[0]) if len(words) == 1 else None


def in_script(word, script):
    letters = [c for c in word if c.isalpha()]
    return bool(letters) and all(unicodedata.name(c, "").startswith(script) for c in letters)


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--fold", choices=sorted(FOLDS))
    ap.add_argument("--headword", action="store_true", help="the headword and its first romanization only")
    ap.add_argument("--last", action="store_true", help="with --headword, its last romanization")
    ap.add_argument("--script", help="only words of this script")
    ap.add_argument("extract")
    opts = ap.parse_args()
    script = opts.script.replace("_", " ") if opts.script else None
    seen = set()
    out = sys.stdout
    for line in gzip.open(opts.extract, "rt", encoding="utf-8"):
        e = json.loads(line)
        forms = e.get("forms", [])
        can = [native(f["form"]) for f in forms if "canonical" in f.get("tags", [])]
        rom = [f["form"] for f in forms if f.get("tags") == ["romanization"]]
        if not rom:
            continue
        if opts.headword:
            pairs = [(native(e["word"]), rom[-1] if opts.last else rom[0])]
        elif len(can) == len(rom):
            # the forms and their romanizations are in the same order
            pairs = zip(can, rom)
        else:
            # only the first romanization is sure: the headword's
            pairs = [(native(e["word"]), rom[0])]
        for word, latin in pairs:
            if not word or (script and not in_script(word, script)):
                continue
            latin = fold(latin, opts.fold)
            if OK.fullmatch(latin) and (word, latin) not in seen:
                seen.add((word, latin))
                out.write(f"{word}\t{latin}\t1\n")


if __name__ == "__main__":
    main()
