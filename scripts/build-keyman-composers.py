#!/usr/bin/env python3
"""build-keyman-composers.py — the keyboards of Vai and Nuosu (Yi), whose
Keyman keyboards type each syllable as Latin letters.

Keyman's athinkra_vai (Athinkra, MIT) types a Vai syllable as its
consonant and its vowel, k then a ꕪ and a ~ after it the nasal ꕫ, its
rules changing what is before the caret as each key comes, through
letters of their own between (s then h is ʃ, then a ꕣ). SIL's sil_yi
(MIT) types a Nuosu syllable as its Yi pinyin and a space (bit, then
the space: ꀖ). Neither is keys that type the script, so each is made a
transliteration table (translit/<lang>.txt, osk_data::translit), which
ki-osk's composer reads the letters typed with, a word at a time, and a
layout of the Latin letters that types into it (`ime: translit`). The
table is what each syllable is typed as, found by running the
keyboard's rules on keys typed from nothing (a search over the text
before the caret), a syllable a rule; a word is read as the fewest
syllables its letters spell, as typed.

Vai's: Keyman's capitals, E O N T D J, are also read as ɛ ɔ ŋ þ ð ʒ, the
vowels they type and the letters its rules write between, which the
layout's keys type: a capital is a letter of its own (T is THA, t TA),
and a Shift view would capitalize a field's first letter. Keyman's hE
and hE~ type HEN and WE, a place past HE and HEN: they are read as HE
and HEN, as Unicode names them. NDOLE MA and NDOLE DO, which it lacks,
and FAA and DO-O, whose rule its DO rule comes before, are typed as its
other Ndole forms are, a hyphen first (-ma, -dlɔ, -fa~, -dlɔ~): so every
syllable of CLDR's exemplar characters, main and auxiliary, is typed,
and every word of Vai's Incubator wiki, spelled so, reads back as
written. Its layout is the US keys but Shift: ɛ in q's place (Vai writes
no q), ɔ at the home row's end and ~ in Shift's, for Vai's ɛ is in 12%
of its syllables, its ɔ in 10% and its nasals in 5% (on its Incubator
wiki); ŋ þ ð ʒ on the long presses of n t d j and the lengthener ꘌ on
x's, Vai's comma, full stop and question mark on those of , . ?, its
digits on the symbol view's.

Nuosu's: each syllable is its pinyin without the space, and the table
says `unspaced`, for Nuosu writes no space between its words (on its
Incubator wiki, a space between two syllables once in 140): Space takes
the syllables typed without one. Where a tone letter (t, x, p) could end
a syllable or begin the next (bita: bit a, bi ta), a syllable of a vowel
alone costs a little more, so the reading has the other (bi ta), as
Nuosu writes more often: of its Incubator wiki's words typed whole,
96.7% read back as written so, 75% the other way, the rest mostly a
vowel that ends a syllable or is one (gu o, guo); typed a syllable and
Space at a time, as Keyman's keyboard has them typed, all of them. Its
layout is the US keys but Shift (Yi pinyin has no capitals): 、 in
Shift's place, ， and 。 for , and ., as Keyman's rules type them, the
full stop's long presses Keyman's phone layout's (！ ？ ： ；), and the
symbol views' punctuation as its rules type it (fullwidth).

Each keyboard's files are fetched from keymanapp/keyboards at the
pinned commit and checked against their sha256 into $KEYMAN_DIR, as
scripts/build-keyman-layouts.py's are.

    scripts/build-keyman-composers.py [--check]

--check fails if a written file would differ, as a test of the tree
(it needs the files, or the network).
"""
import importlib.util
import json
import pathlib
import re
import sys
import unicodedata

ROOT = pathlib.Path(__file__).resolve().parent.parent

_spec = importlib.util.spec_from_file_location("keyman", ROOT / "scripts" / "build-keyman-layouts.py")
keyman = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(keyman)

# The layout and table made, the language's name (CLDR's, in its
# script), the keyboard (its path in keymanapp/keyboards and the commit)
# and the sha256 of its rules, touch layout and licence.
COMPOSERS = [
    ("vai", "ꕙꔤ", "release/athinkra/athinkra_vai", keyman.COMMIT, {
        "source/athinkra_vai.kmn": "8818ee7a97417cfb414decc41b729b3ef6daac2cf6418d0eb8400833b048070e",
        "source/athinkra_vai.keyman-touch-layout": "6063c6dcdcafd4925491a6b47d7495ad7c0f31fa3fb19dadafd45c92d5fe6212",
        "LICENSE.md": "8a26fe15421eb80297aebb99bab679201acfaf5441ffec6655ad7c2743db2cb0",
    }),
    ("ii", "ꆈꌠꉙ", "release/sil/sil_yi", keyman.COMMIT, {
        "source/sil_yi.kmn": "69792d7ccbfdcbeac53d297bdef962c86763f3695917a257bbc7e23a045100fe",
        "source/sil_yi.keyman-touch-layout": "87ef52bebf71c4122236d1a3811eff2e909fdecb0fe511c9434de4032b178d21",
        "LICENSE.md": "eaa17ceee65d09212dffe4535f6f3be1d46241d5a31515596f5d6f35c7203929",
    }),
]

# The script's own characters: what a typing is searched for.
OWN = {
    "vai": lambda c: 0xA500 <= ord(c) <= 0xA62B and unicodedata.category(c)[0] == "L",
    "ii": lambda c: 0xA000 <= ord(c) <= 0xA48C,
}
# The keys typed in the search, and how long a typing may be.
KEYS = {
    "vai": "abcdefghijklmnopqrstuvwxyzOETDJNX~-",
    "ii": "abcdefghijklmnopqrstuvwxyz ",
}
LONGEST = {"vai": 6, "ii": 7}
# Keyman's capitals, and the letters they are also read as.
ALSO = {"vai": {"E": "ɛ", "O": "ɔ", "N": "ŋ", "T": "þ", "D": "ð", "J": "ʒ"}}
# What a typing types where Keyman's does not (see the docstring).
FIXED = {
    "vai": {"hE": "ꗤ", "hE~": "ꗥ", "-ma": "ꘪ", "-dlO": "ꘫ",
            "-fa~": "ꘘ", "-dlO~": "ꘞ"},
}
# A syllable of a vowel alone, where a tone letter could end the one
# before it or begin it: its cost.
VOWEL_FIRST = {"ii": 0.1}
VOWELS = "aeiouɛɔ"
FONT_SCRIPT = {"vai": "vai", "ii": "yi"}
UNSPACED = {"ii"}
NAMES = {"vai": "Vai", "ii": "Nuosu"}


class Keyboard:
    """A .kmn's keys group, run on keys typed: each rule a context (the
    text it needs before the caret), a key and what it writes in the
    context's place, the longest context first, then the file's order,
    as Keyman matches them."""

    def __init__(self, kmn):
        text = re.sub(r"\\\s*\n", " ", kmn.decode("utf-8-sig"))
        lines = [keyman.uncomment(line) for line in text.splitlines()]
        reader = keyman.Rules.__new__(keyman.Rules)
        reader.stores = {}
        for line in lines:
            m = re.match(r"store\(&?(\w+)\)\s*(.*)$", line, re.I)
            if m:
                reader.stores[m.group(1)] = reader.members(keyman.tokens(m.group(2)) or [])
        self.reader = reader
        self.by_key = {}
        self.contexts = set()
        rules = []
        keys_group = False
        for line in lines:
            m = re.match(r"group\s*\(\w+\)\s*(using keys)?", line, re.I)
            if m:
                keys_group = bool(m.group(1))
                continue
            if not keys_group or ">" not in line:
                continue
            toks = keyman.tokens(line)
            if not toks or ("plus", None) not in toks or ("gt", None) not in toks:
                continue
            plus, gt = toks.index(("plus", None)), toks.index(("gt", None))
            if gt != plus + 2 or any(kind != "str" for kind, _ in toks[:plus]):
                continue
            context = "".join(val for _, val in toks[:plus])
            kind, val = toks[plus + 1]
            if kind == "str" and len(val) == 1:
                keyed = [(val, None)]
            elif kind == "any":
                keyed = [(item, i) for i, item in enumerate(reader.stores.get(val, [])) if isinstance(item, str)]
            else:
                continue  # a key named by its code: AltGr's, not typed here
            for typed, index in keyed:
                out = reader.output(toks[gt + 1:], index)
                if out is not None:
                    rules.append((len(context), len(rules), typed, context, out))
            if context:
                self.contexts.add(context)
        for _, _, typed, context, out in sorted(rules, key=lambda r: (-r[0], r[1])):
            self.by_key.setdefault(typed, []).append((context, out))

    def press(self, before, key):
        for context, out in self.by_key.get(key, []):
            if before.endswith(context):
                return before[:len(before) - len(context)] + out
        return before + key

    def typings(self, keys, own, longest):
        """Every way each own character is typed from nothing, in keys up
        to `longest`: a search over the text before the caret, carried on
        from a text of one or two pieces each a context begun, every
        typing that reaches it (ŋ is N, and n then g)."""
        begun = {c[:i] for c in self.contexts for i in range(1, len(c) + 1)}

        def carried(text):
            return text in begun or any(text[:i] in begun and text[i:] in begun for i in range(1, len(text)))

        found = {}
        frontier = {"": [""]}
        for _ in range(longest):
            nxt = {}
            for before, typed in frontier.items():
                for key in keys:
                    after = self.press(before, key)
                    reached = [t + key for t in typed]
                    if len(after) == 1 and own(after):
                        found.setdefault(after, []).extend(reached)
                    if carried(after):
                        nxt.setdefault(after, []).extend(reached)
            frontier = nxt
        return found


def table(lang, keyboard, path):
    found = keyboard.typings(KEYS[lang], OWN[lang], LONGEST[lang])
    rules = {}
    for char, typings in found.items():
        for typed in typings:
            rules.setdefault(typed.strip(), char)
    rules.update(FIXED.get(lang, {}))
    for typed, char in list(rules.items()):
        also = "".join(ALSO.get(lang, {}).get(c, c) for c in typed)
        if also != typed:
            rules.setdefault(also, char)
    commit = keyman.COMMIT
    lines = [
        f"# How the Latin letters typed for {NAMES[lang]} are its syllables, made by",
        "# scripts/build-keyman-composers.py from Keyman's keyboard's rules",
        f"# (keymanapp/keyboards, {path} at {commit[:12]};",
        "# its MIT licence and copyright in NOTICES.md): a syllable each way it is typed.",
        "script alphabet",
    ]
    if lang in UNSPACED:
        lines.append("unspaced")
    vowel_first = VOWEL_FIRST.get(lang)
    for section, pick in (("consonants", lambda t: t[0].lower() not in VOWELS),
                          ("vowels", lambda t: t[0].lower() in VOWELS)):
        lines.append(f"[{section}]")
        for typed, char in sorted(rules.items(), key=lambda kv: (kv[1], len(kv[0]), kv[0])):
            if not pick(typed):
                continue
            cost = f":{vowel_first}" if vowel_first and section == "vowels" else ""
            lines.append(f"{typed}  {char}{cost}")
    return "\n".join(lines) + "\n"


def text_key(text, long_press=(), width=1):
    key = {"width_units": width, "label": text, "output": {"kind": "text", "text": text}}
    if long_press:
        key["long_press"] = [{"kind": "text", "text": t} for t in long_press]
    return key


def build_layout(lang, name, keyboard, touch):
    us = json.loads((ROOT / "layouts" / "us.json").read_text(encoding="utf-8"))
    layout = {
        "id": lang,
        "name": name,
        "ime": "translit",
        "swipe_enabled": False,
        "prediction_enabled": False,
        "font_script": FONT_SCRIPT[lang],
    }
    views = {v["id"]: v for v in us["views"]}
    base = views["base"]
    letters = [[k["label"] for k in row["keys"] if len(k["label"]) == 1 and k["label"].isalpha()]
               for row in base["rows"][:3]]
    backspace = base["rows"][2]["keys"][-1]
    bottom = base["rows"][3]["keys"]
    if lang == "vai":
        long_press = {"n": ["ŋ"], "t": ["þ"], "d": ["ð"], "j": ["ʒ"], "x": ["ꘌ"]}
        row0 = [text_key("ɛ")] + [text_key(c, long_press=long_press.get(c, ())) for c in letters[0][1:]]
        row1 = [text_key(c, long_press=long_press.get(c, ())) for c in letters[1]] + [text_key("ɔ")]
        row2 = [text_key("~", width=1.5)] + [text_key(c, long_press=long_press.get(c, ())) for c in letters[2]]
        stops = {",": ["꘍"], ".": ["꘎"], "?": ["꘏"]}
        digits = {str(d): [chr(0xA620 + d)] for d in range(10)}
    else:
        row0 = [text_key(c) for c in letters[0]]
        row1 = [{"width_units": 0.5, "label": "", "output": {"kind": "none"}}] + [text_key(c) for c in letters[1]] \
            + [{"width_units": 0.5, "label": "", "output": {"kind": "none"}}]
        row2 = [text_key(keyboard.press("", "\\"), width=1.5)] + [text_key(c) for c in letters[2]]
        digits = {}
        # Keyman's phone layout's full stop, its long presses.
        full_stop = keyboard.press("", ".")
        sk = []
        for layer in touch["phone"]["layer"]:
            for row in layer["row"]:
                for key in row["key"]:
                    if key.get("text") == full_stop:
                        sk = [s["text"] for s in key.get("sk", []) if s.get("text")]
        # Not those a key of the base view types (， 、).
        tapped = {k["output"]["text"] for k in row2} | {keyboard.press("", k["output"]["text"])
                                                        for k in bottom if k["output"].get("kind") == "text"}
        stops = {full_stop: [s for s in sk if s not in tapped]}
    base["rows"][0]["keys"] = row0
    base["rows"][1]["keys"] = row1
    base["rows"][2]["keys"] = row2 + [backspace]
    out_views = []
    for view in us["views"]:
        if view["id"] == "shift":
            continue
        for row in view["rows"]:
            for key in row["keys"]:
                text = key.get("output", {}).get("text")
                if key.get("output", {}).get("kind") != "text" or text is None:
                    continue
                if lang == "ii" and len(text) == 1 and text.isascii() and not text.isalnum():
                    typed = keyboard.press("", text)
                    if len(typed) == 1 and typed != text:
                        key["label"] = typed
                        key["output"] = {"kind": "text", "text": typed}
                        text = typed
                extra = (stops.get(text, []) if view["id"] in ("base", "symbol1") else []) \
                    + (digits.get(text, []) if view["id"] == "symbol1" else [])
                if extra:
                    key["long_press"] = [{"kind": "text", "text": t} for t in extra] + key.get("long_press", [])
        out_views.append(view)
    layout["views"] = out_views
    if lang == "vai":
        flagged = {}
        for k, v in layout.items():
            if k == "views":
                flagged["long_press_letters"] = True
            flagged[k] = v
        layout = flagged
    return json.dumps(layout, ensure_ascii=False, indent=2) + "\n"


def main():
    check = "--check" in sys.argv[1:]
    stale = []
    for lang, name, path, commit, files in COMPOSERS:
        got = keyman.fetch(path, commit, files)
        keyboard = Keyboard(next(v for k, v in got.items() if k.endswith(".kmn")))
        touch = json.loads(next(v for k, v in got.items() if k.endswith(".keyman-touch-layout")).decode("utf-8-sig"))
        outputs = [(ROOT / "translit" / f"{lang}.txt", table(lang, keyboard, path)),
                   (ROOT / "layouts" / f"{lang}.json", build_layout(lang, name, keyboard, touch))]
        for out, body in outputs:
            if check:
                if not out.exists() or out.read_text(encoding="utf-8") != body:
                    stale.append(out.name)
            else:
                out.write_text(body, encoding="utf-8")
                print(f"wrote {out.relative_to(ROOT)}")
    if stale:
        sys.exit("out of date (run scripts/build-keyman-composers.py): " + " ".join(stale))


if __name__ == "__main__":
    main()
