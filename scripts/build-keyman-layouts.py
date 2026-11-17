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
The views of symbols and the control row are the US template's, as
scripts/build-layout.py's layouts' are; every key types text, so no
keymap is needed (xkb_layout names Burmese's, mm, for the keys the
control row types by keycode).

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
]

# Keys whose rule types other than their label, which type their label.
TYPES_ITS_LABEL = {("ksw", "shift", "K_QUOTE")}

# The layers whose keys type one letter each, a key of several typing its
# first, and a key added at the end of a layer's home row.
ONE_LETTER = {("ksw", "default")}
HOME_ROW_END = {("ksw", "default"): "\u103A"}

# A rule with no context: `+ [SHIFT K_X] > U+1062 U+103a`.
RULE = re.compile(r"^\+ \[((?:SHIFT )?)(K_\w+)\] > ((?:U\+[0-9A-Fa-f]+\s*)+|nul)\s*$")
SPECIAL = {"K_SHIFT", "K_BKSP", "K_ENTER", "K_SPACE", "K_NUMLOCK", "K_LOPT", "K_LOWER"}


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
            text = "" if m.group(3) == "nul" else "".join(chr(int(u[2:], 16)) for u in m.group(3).split())
            out[(bool(m.group(1)), m.group(2))] = text
    return out


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
            if k["id"] in SPECIAL or k.get("sp") in ("1", "2"):
                continue
            text = typed(lang, layer["id"], k, kmn)
            label = k["text"]
            if (lang, layer["id"]) in ONE_LETTER:
                text = label = text[0]
            key = {"width_units": 1, "label": label, "output": {"kind": "text", "text": text}}
            if k.get("sk"):
                key["long_press"] = [{"kind": "text", "text": typed(lang, layer["id"], s, kmn)} for s in k["sk"]]
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
    layers = {layer["id"]: layer for layer in touch["phone"]["layer"]}
    layout = json.loads((ROOT / "layouts" / "us.json").read_text(encoding="utf-8"))
    layout["id"] = lang
    layout["name"] = name
    layout["xkb_layout"] = "mm"
    layout.pop("xkb_variant", None)
    layout["direction"] = "ltr"
    layout["swipe_enabled"] = False
    if (ROOT / "predict" / f"{lang}.fst").exists():
        layout["prediction_enabled"] = True
        layout["prediction_lang"] = lang
    else:
        layout["prediction_enabled"] = False
        layout.pop("prediction_lang", None)
    layout["font_script"] = "myanmar"
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
