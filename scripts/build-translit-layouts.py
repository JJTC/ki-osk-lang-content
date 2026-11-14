#!/usr/bin/env python3
"""build-translit-layouts.py — the transliteration layouts, `<lang>-abc`.

Each is the US layout's keys typing their Latin letters into the
language's transliteration composer (`ime: translit`): the table
`translit/<lang>.txt` reads them, the language's lexicon chooses among
its readings, and the strip offers them (osk_data::translit). The views
are the US layout's (Shift, the digits and symbols); a letter key types
its letter as text rather than a keycode, since the composer reads it,
and keeps no accent menu (é is no letter of a transliteration). Each is
named in its own script, after the language's own layout, and draws the
strip's words in that layout's font.

  scripts/build-translit-layouts.py [--check]

Writes layouts/<lang>-abc.json for every table in translit/, each
language having a layout of its own (scripts/build-derived-layouts.py
makes those of the languages typed on another's keys); a table of a
language in a second script (translit/sd-Deva.txt) is its tag's, and its
layouts' ids are the tag in lower case (sd-deva, sd-deva-abc). --check
fails if a written file would differ, as a test of the tree.

It writes the layouts of the languages typed in Latin letters and left in
them too, so with no composer but the US layout as it is (its keys, its
accent menus, glide typing) suggesting from the language's model:
Hinglish, hi-latn, whose model is Hindi's text respelled as people type
it (scripts/build-hinglish.py), and Kokborok, trp, written in Latin
letters.
"""
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent

# Where the language's own layout names it in English only, its name in
# its script.
NAMES = {
    "el": "Ελληνικά",
    "fa": "فارسی",
}

def letter(label):
    return len(label) == 1 and label.isascii() and label.isalpha()


def build(lang, us):
    own = json.loads((ROOT / "layouts" / f"{lang.lower()}.json").read_text(encoding="utf-8"))
    name = NAMES.get(lang, own["name"])
    out = {
        "id": f"{lang.lower()}-abc",
        "name": f"{name} (abc)",
        "ime": "translit",
    }
    if own.get("font_script"):
        out["font_script"] = own["font_script"]
    out["swipe_enabled"] = False
    out["prediction_enabled"] = False
    out["prediction_lang"] = lang
    views = []
    for view in us["views"]:
        rows = []
        for row in view["rows"]:
            keys = []
            for key in row["keys"]:
                key = dict(key)
                if letter(key["label"]) and not key.get("action"):
                    key["output"] = {"kind": "text", "text": key["label"]}
                    key.pop("long_press", None)
                keys.append(key)
            rows.append({"height_units": row["height_units"], "keys": keys})
        views.append({"id": view["id"], "rows": rows})
    out["views"] = views
    return json.dumps(out, ensure_ascii=False, indent=2) + "\n"


# The languages typed on the US keys as they are: layout id, name, model.
LATIN = [("hi-latn", "Hinglish", "hi-Latn"), ("trp", "Kokborok", "trp")]


def latin(us, layout_id, name, lang):
    out = dict(us)
    out["id"] = layout_id
    out["name"] = name
    out["prediction_lang"] = lang
    return json.dumps(out, ensure_ascii=False, indent=2) + "\n"


def main():
    check = "--check" in sys.argv[1:]
    us = json.loads((ROOT / "layouts" / "us.json").read_text(encoding="utf-8"))
    stale = []
    outputs = [(f"{t.stem.lower()}-abc", build(t.stem, us)) for t in sorted((ROOT / "translit").glob("*.txt"))]
    outputs += [(layout_id, latin(us, layout_id, name, lang)) for layout_id, name, lang in LATIN]
    for name, text in outputs:
        path = ROOT / "layouts" / f"{name}.json"
        if check:
            if not path.exists() or path.read_text(encoding="utf-8") != text:
                stale.append(path.name)
        else:
            path.write_text(text, encoding="utf-8")
            print(f"wrote layouts/{path.name}")
    if stale:
        sys.exit("out of date (run scripts/build-translit-layouts.py): " + " ".join(stale))


if __name__ == "__main__":
    main()
