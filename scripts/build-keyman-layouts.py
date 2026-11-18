#!/usr/bin/env python3
"""build-keyman-layouts.py — layouts from Keyman's touch keyboards, for
the languages xkeyboard-config has no keymap of.

Sgaw Karen's is Keyman's sil_sgaw_karen (SIL Global, MIT), laid out for
a phone: three rows of letters a view, the signs Karen writes most on
keys of their own, some of them two or three letters (ၣ် ာ် ၢ် ၢၣ်…), and
whole words on long presses (လီၤ, ဒီး). Its touch layout's default and
shift layers become the base and shift views' letter rows, each key
typing what Keyman's rules (its .kmn) type for it, which is its label:
but the shift layer's ၢၤ, whose rule types ", types its label, as its
label says (ၢၤ is in one Karen syllable in fifty). A key's long presses
are its own. On the base view a key types one letter, as the corrector
takes a tap to (a key typing two, ၣ်, was no letter key to it, and on
Keyman's layout autocorrect changed one Karen word typed right in
fifteen): the keys of a sign and the asat, ၣ် ာ် ၢ်, type the sign, and
the asat, ်, the commonest of Karen's letters, has a key of its own at
the home row's end; the shift view keeps Keyman's keys of two or three.
Northwestern Ojibwe's is FirstVoices' fv_ojibwa (MIT), its syllabics
for a tablet: the i and a syllables of each consonant on the default
layer, with the finals, and the e and o syllables on the shift layer.
Keyman types a long vowel's syllable and a w-dotted one with keys of
their own after or before the syllable (ᑭ˙ ᑮ, ᐧᑭ ᑶ), by rules with a
context, which a key that types one letter has none of: on each
syllable's long press are its long vowel and its w-dotted forms first,
as Unicode names them (ᑭ's ᑮ ᑶ ᑸ, ᑫ's ᑴ), and on ᒃ's the final ᕽ
(hk), which the keyboard lacks and one Ojibwe word in twenty writes; its
full stop ᙮, which its shift layer has, is on the long press of the
full stop. The views of symbols and the control row are the US
template's, as scripts/build-layout.py's layouts' are; every key types
text, so no keymap is needed (xkb_layout names the script's, Burmese's
mm and Canada's ca, for the keys the control row types by keycode).

Each keyboard's files are fetched from keymanapp/keyboards at a pinned
commit and checked against their sha256 into $KEYMAN_DIR (default
$TMPDIR/ki-osk-sources/keyman); a layout's id is its language's tag.

    scripts/build-keyman-layouts.py [--check]

--check fails if a written file would differ, as a test of the tree
(it needs the files, or the network).
"""
import hashlib
import json
import os
import pathlib
import re
import sys
import unicodedata
import urllib.request

ROOT = pathlib.Path(__file__).resolve().parent.parent
REPO = "https://raw.githubusercontent.com/keymanapp/keyboards"

# The layout made, its name, the keyboard (its path in keymanapp/keyboards
# and the commit), and the sha256 of its touch layout, rules and licence.
KEYBOARDS = [
    ("ksw", "ကညီ", "release/sil/sil_sgaw_karen", "7225701d383cf8317f595bfd73481fcd87524f44", {
        "source/sil_sgaw_karen.keyman-touch-layout":
            "7ea0e10ec4dbcc92a7d0ed700cd48f9dc4f5405513f6413d61bd64277d9a19fb",
        "source/sil_sgaw_karen.kmn": "915740a85216b9553abae0b7dec0bfa3ecbbc18d922e00bd9c3d1bf6212461bd",
        "LICENSE.md": "dbb1d1f49159d4713f308d293eb2d3a201ae495332600f729cea6f22453bf78a",
    }),
    ("ojb", "ᐊᓂᔑᓇᐯᒧᐎᓐ", "release/fv/fv_ojibwa", "7225701d383cf8317f595bfd73481fcd87524f44", {
        "source/fv_ojibwa.keyman-touch-layout": "79ff1490dee2a14f54472730ff6f762709e05970bb9b16d35df35c585ace723d",
        "source/fv_ojibwa.kmn": "7b3a98522842d6061158ae8f6c05c6f3f1d6d62b998c55976c7a6e7e63e68764",
        "LICENSE.md": "d0ca606a218e493ce0f0e8374e6972d35a783cbcd8aa15b0ef0f91998f56dca3",
    }),
]

# Each keyboard's touch platform, the keymap its control row types
# through, and its script's font.
PLATFORM = {"ksw": "phone", "ojb": "tablet"}
XKB_LAYOUT = {"ksw": "mm", "ojb": "ca"}
FONT_SCRIPT = {"ksw": "myanmar", "ojb": "canadian_aboriginal"}

# Keys whose rule types other than their label, which type their label.
TYPES_ITS_LABEL = {("ksw", "shift", "K_QUOTE")}

# The layers whose keys type one letter each, a key of several typing its
# first, and a key added at the end of a layer's home row.
ONE_LETTER = {("ksw", "default")}
HOME_ROW_END = {("ksw", "default"): "\u103A"}

# The keyboards whose syllables take their long vowel and w-dotted forms
# on their long presses (syllabic_forms), the letters a key gains on its
# long press, and the letter the control row's full stop gains on its.
SYLLABICS = {"ojb"}
LONG_PRESS_ADDED = {("ojb", "\u1483"): ["\u157D"]}
FULL_STOP_LONG_PRESS = {"ojb": "\u166E"}

# A rule with no context: `+ [SHIFT K_X] > U+1062 U+103a`, `+ [T_Q] > 'ᐃ'`.
RULE = re.compile(r"^\+ \[((?:SHIFT )?)([KT]_\w+)\] > ((?:U\+[0-9A-Fa-f]+\s*)+|nul|'[^']+')\s*$")
SPECIAL = {"K_SHIFT", "K_BKSP", "K_ENTER", "K_SPACE", "K_NUMLOCK", "K_LOPT", "K_LOWER"}
# Touch keys that type nothing: Keyman's special keys and its spacers.
NO_LETTER = {"1", "2", "10"}


def fetch(path, commit, files):
    """The keyboard's files, fetched where they are not kept, checked."""
    cache = pathlib.Path(os.environ.get("KEYMAN_DIR")
                         or pathlib.Path(os.environ.get("TMPDIR", "/tmp")) / "ki-osk-sources" / "keyman")
    out = {}
    for name, sha in files.items():
        local = cache / commit / path / name
        if not local.exists() or hashlib.sha256(local.read_bytes()).hexdigest() != sha:
            local.parent.mkdir(parents=True, exist_ok=True)
            req = urllib.request.Request(f"{REPO}/{commit}/{path}/{name}", headers={"User-Agent": "ki-osk-lang-content"})
            with urllib.request.urlopen(req) as r:
                data = r.read()
            got = hashlib.sha256(data).hexdigest()
            if got != sha:
                sys.exit(f"{path}/{name}: sha256 {got}, not {sha}")
            local.write_bytes(data)
        out[name] = local.read_bytes()
    return out


def rules(kmn):
    """What each key types, by (shifted, key), from the rules with no context."""
    out = {}
    for line in kmn.decode("utf-8-sig").splitlines():
        m = RULE.match(line.strip())
        if m:
            if m.group(3) == "nul":
                text = ""
            elif m.group(3).startswith("'"):
                text = m.group(3)[1:-1]
            else:
                text = "".join(chr(int(u[2:], 16)) for u in m.group(3).split())
            out[(bool(m.group(1)), m.group(2))] = text
    return out


def syllabic_forms(text):
    """A syllable's long vowel and w-dotted forms, as Unicode's syllabics
    block names them (ᑭ's ᑮ ᑶ ᑸ; ᑫ's ᑴ, e having no short vowel), which
    Keyman types after its ˙ key and before its ᐧ: not the Extended
    block's (ᓂ's ᣆ), which it does not type."""
    m = re.fullmatch(r"(P|T|K|C|M|N|S|SH|Y)?(E|I|O|A)", unicodedata.name(text, "").removeprefix("CANADIAN SYLLABICS "))
    if len(text) != 1 or not m:
        return []
    c, v = m.group(1) or "", m.group(2)
    names = [c + "WE"] if v == "E" else [c + v + v, c + "W" + v, c + "W" + v + v]
    forms = []
    for n in names:
        try:
            form = unicodedata.lookup("CANADIAN SYLLABICS " + n)
        except KeyError:
            continue
        if "\u1400" <= form <= "\u167F":
            forms.append(form)
    return forms


def typed(lang, layer, key, kmn):
    """What a touch key (or a long press) types, checked against its label."""
    kid, label = key["id"], key.get("text", "")
    if kid.startswith("U_"):
        text = "".join(chr(int(h, 16)) for h in kid[2:].split("_"))
    else:
        text = kmn.get((key.get("layer", layer) == "shift", kid))
        if text is None:
            sys.exit(f"{lang}: {layer} {kid}: no rule")
    if text != label:
        if (lang, layer, kid) not in TYPES_ITS_LABEL:
            sys.exit(f"{lang}: {layer} {kid}: labelled {label!r}, types {text!r}")
        text = label
    return text


def letter_rows(lang, layer, kmn):
    """A touch layer's three rows of letters, as text keys."""
    rows = []
    for row in layer["row"][:3]:
        keys = []
        for k in row["key"]:
            if k["id"] in SPECIAL or str(k.get("sp")) in NO_LETTER:
                continue
            text = typed(lang, layer["id"], k, kmn)
            label = k["text"]
            if (lang, layer["id"]) in ONE_LETTER:
                text = label = text[0]
            key = {"width_units": 1, "label": label, "output": {"kind": "text", "text": text}}
            long_press = (syllabic_forms(text) if lang in SYLLABICS else []) + LONG_PRESS_ADDED.get((lang, text), [])
            long_press += [typed(lang, layer["id"], s, kmn) for s in k.get("sk", [])]
            if long_press:
                key["long_press"] = [{"kind": "text", "text": t} for t in long_press]
            keys.append(key)
        rows.append(keys)
    end = HOME_ROW_END.get((lang, layer["id"]))
    if end:
        rows[1].append({"width_units": 1, "label": end, "output": {"kind": "text", "text": end}})
    return rows


def build(lang, name, path, commit, files):
    got = fetch(path, commit, files)
    touch = json.loads(next(v for k, v in got.items() if k.endswith(".keyman-touch-layout")).decode("utf-8-sig"))
    kmn = rules(next(v for k, v in got.items() if k.endswith(".kmn")))
    layers = {layer["id"]: layer for layer in touch[PLATFORM[lang]]["layer"]}
    layout = json.loads((ROOT / "layouts" / "us.json").read_text(encoding="utf-8"))
    layout["id"] = lang
    layout["name"] = name
    layout["xkb_layout"] = XKB_LAYOUT[lang]
    layout.pop("xkb_variant", None)
    layout["direction"] = "ltr"
    layout["swipe_enabled"] = False
    if (ROOT / "predict" / f"{lang}.fst").exists():
        layout["prediction_enabled"] = True
        layout["prediction_lang"] = lang
    else:
        layout["prediction_enabled"] = False
        layout.pop("prediction_lang", None)
    layout["font_script"] = FONT_SCRIPT[lang]
    for view in layout["views"]:
        layer = {"base": "default", "shift": "shift"}.get(view["id"])
        if layer is None:
            continue
        rows = view["rows"]
        shift, backspace = rows[2]["keys"][0], rows[2]["keys"][-1]
        r0, r1, r2 = letter_rows(lang, layers[layer], kmn)
        rows[0]["keys"] = r0
        rows[1]["keys"] = r1
        rows[2]["keys"] = [shift] + r2 + [backspace]
        stop = FULL_STOP_LONG_PRESS.get(lang)
        for key in rows[3]["keys"] if stop else []:
            if key.get("output") == {"kind": "text", "text": "."}:
                key["long_press"] = [{"kind": "text", "text": stop}] + key.get("long_press", [])
    # The template's keys first, then the layout's own fields as
    # build-layout.py writes them.
    return json.dumps(layout, ensure_ascii=False, indent=2) + "\n"


def main():
    check = "--check" in sys.argv[1:]
    stale = []
    for lang, name, path, commit, files in KEYBOARDS:
        text = build(lang, name, path, commit, files)
        out = ROOT / "layouts" / f"{lang}.json"
        if check:
            if not out.exists() or out.read_text(encoding="utf-8") != text:
                stale.append(out.name)
        else:
            out.write_text(text, encoding="utf-8")
            print(f"wrote layouts/{out.name}")
    if stale:
        sys.exit("out of date (run scripts/build-keyman-layouts.py): " + " ".join(stale))


if __name__ == "__main__":
    main()
