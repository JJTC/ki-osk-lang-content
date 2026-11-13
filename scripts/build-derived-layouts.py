#!/usr/bin/env python3
"""build-derived-layouts.py — the layouts that are another language's
keys, renamed, for the languages that write with them.

Bhojpuri, Maithili, Sanskrit and Awadhi are written in Devanagari as
Hindi is, and typed on Hindi's InScript keys (xkeyboard-config's `in`):
their layouts are Hindi's, named in their own words and predicting from
their own models, Sanskrit's with its avagraha ऽ on the long press of ः
and ॐ on ओ's. Konkani is typed on Marathi's keys (`in(marathi)`), ळ
among them, and Tulu on Kannada's. Western Punjabi, Saraiki and Kashmiri
in Perso-Arabic script are typed on Urdu's keys (`pk`), with the letters
Urdu lacks on the long presses of those they are made from: Punjabi's
and Saraiki's ݨ on ن, Saraiki's implosives ٻ ڄ ݙ ڳ on ب ج د گ,
Kashmiri's vowel letters ٲ ۄ ۆ ؠ ۍ on ا و ی and its vowel marks
ٕ ٟ ٖ ٗ ٚ ٛ on Urdu's. Yiddish is typed on Hebrew's keys (`il`), its
pointed letters (אַ אָ פּ פֿ וּ יִ …) and the ligatures װ ױ ײ on the long
presses of the letters they are made from. A letter only a long press
types turns the gesture on for the layout (`long_press_letters`).

    scripts/build-derived-layouts.py [--check]

--check fails if a written file would differ, as a test of the tree.
"""
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent

# The layout made, the one it is made from, its name, and what its keys
# gain on a long press (a key's label → the letters), on every view.
DERIVED = [
    ("bho", "hi", "भोजपुरी", {}),
    ("mai", "hi", "मैथिली", {}),
    ("pnb", "ur", "پنجابی", {"ن": ["ݨ"]}),
    ("sa", "hi", "संस्कृतम्", {"ः": ["ऽ"], "ओ": ["ॐ"]}),
    ("awa", "hi", "अवधी", {}),
    ("kok", "mr", "कोंकणी", {}),
    ("tcy", "kn", "ತುಳು", {}),
    ("skr", "ur", "سرائیکی", {"ب": ["ٻ"], "ج": ["ڄ"], "د": ["ݙ"], "گ": ["ڳ"], "ن": ["ݨ"]}),
    (
        "ks",
        "ur",
        "کٲشُر",
        {
            "ا": ["ٲ"],
            "و": ["ۄ", "ۆ"],
            "ی": ["ؠ", "ۍ"],
            "ٔ": ["ٕ", "ٟ"],
            "ِ": ["ٖ"],
            "ُ": ["ٗ"],
            "َ": ["ٚ", "ٛ"],
        },
    ),
    (
        "yi",
        "he",
        "ייִדיש",
        {
            "א": ["אַ", "אָ"],
            "ב": ["בֿ"],
            "ו": ["וּ", "װ", "ױ"],
            "י": ["יִ", "ײ", "ײַ"],
            "כ": ["כּ"],
            "פ": ["פּ", "פֿ"],
            "ש": ["שׂ"],
            "ת": ["תּ"],
        },
    ),
]


def build(lang, base, name, extra):
    layout = json.loads((ROOT / "layouts" / f"{base}.json").read_text(encoding="utf-8"))
    out = {}
    for k, v in layout.items():
        if k == "views" and extra:
            # A letter only a long press types: the gesture is on for it.
            out["long_press_letters"] = True
        out[k] = v
    out["id"] = lang
    out["name"] = name
    out["prediction_lang"] = lang
    added = set()
    for view in out["views"]:
        for row in view["rows"]:
            for key in row["keys"]:
                letters = extra.get(key.get("label"))
                if letters:
                    key["long_press"] = key.get("long_press", []) + [
                        {"kind": "text", "text": t} for t in letters
                    ]
                    added.add(key.get("label"))
    missing = set(extra) - added
    if missing:
        sys.exit(f"{lang}: no key {' '.join(sorted(missing))} on {base}'s layout")
    return json.dumps(out, ensure_ascii=False, indent=2) + "\n"


def main():
    check = "--check" in sys.argv[1:]
    stale = []
    for lang, base, name, extra in DERIVED:
        text = build(lang, base, name, extra)
        path = ROOT / "layouts" / f"{lang}.json"
        if check:
            if not path.exists() or path.read_text(encoding="utf-8") != text:
                stale.append(path.name)
        else:
            path.write_text(text, encoding="utf-8")
            print(f"wrote layouts/{path.name}")
    if stale:
        sys.exit("out of date (run scripts/build-derived-layouts.py): " + " ".join(stale))


if __name__ == "__main__":
    main()
