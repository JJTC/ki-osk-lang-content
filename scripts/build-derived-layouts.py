#!/usr/bin/env python3
"""build-derived-layouts.py — the layouts that are another language's
keys, renamed, for the languages that write with them.

Bhojpuri, Maithili, Sanskrit, Awadhi, Bodo, Dogri, Chhattisgarhi and
Magahi are written in Devanagari as Hindi is, and typed on Hindi's
InScript keys (xkeyboard-config's `in`): their layouts are Hindi's,
named in their own words and predicting from their own models,
Sanskrit's with its avagraha ऽ on the long press of ः and ॐ on ओ's.
Konkani is typed on Marathi's keys (`in(marathi)`), ळ among them, and
Tulu on Kannada's. Western Punjabi, Saraiki, Kashmiri and Brahui in
Perso-Arabic script are typed on Urdu's keys (`pk`), with the letters
Urdu lacks on the long presses of those they are made from: Punjabi's
and Saraiki's ݨ on ن, Saraiki's implosives ٻ ڄ ݙ ڳ on ب ج د گ, Brahui's
ۺ on ش, Kashmiri's vowel letters ٲ ۄ ۆ ؠ ۍ on ا و ی and its vowel marks
ٕ ٟ ٖ ٗ ٚ ٛ on Urdu's. Yiddish is typed on Hebrew's keys (`il`), its
pointed letters (אַ אָ פּ פֿ וּ יִ …) and the ligatures װ ױ ײ on the
long presses of the letters they are made from. Sindhi in Devanagari
(sd-Deva) is typed on Hindi's keys, its implosives ॻ ॼ ॾ ॿ on the long
presses of ग ज ड ब, and Meitei in the Bengali script (mni-Beng) on
Bengali's, with ৱ on ব's. Khowar is typed on Urdu's keys, its own
letters on the long presses of those that sound nearest: ځ ݮ on ج, څ ݯ on
چ, ݰ on ش, ݱ on ز. Adilabad Gondi is typed on Telugu's keys. Mizo
and Khasi, in Latin letters, are typed on the US keys with their own
letters first on the long presses, the menu's others after them: Mizo's
â ê î ô û and ṭ, Khasi's ï and ñ. A letter only a long press types turns
the gesture on for the layout (`long_press_letters`). A layout's id is
its language's tag in lower case (sd-deva), its prediction language the
tag.

    scripts/build-derived-layouts.py [--check]

--check fails if a written file would differ, as a test of the tree.
"""
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent

# The layout made, the one it is made from, its name, and what its keys
# gain on a long press (a key's label → the letters, first in its menu),
# on every view.
DERIVED = [
    ("bho", "hi", "भोजपुरी", {}),
    ("mai", "hi", "मैथिली", {}),
    ("pnb", "ur", "پنجابی", {"ن": ["ݨ"]}),
    ("sa", "hi", "संस्कृतम्", {"ः": ["ऽ"], "ओ": ["ॐ"]}),
    ("awa", "hi", "अवधी", {}),
    ("brx", "hi", "बड़ो", {}),
    ("doi", "hi", "डोगरी", {}),
    ("hne", "hi", "छत्तीसगढ़ी", {}),
    ("mag", "hi", "मगही", {}),
    ("brh", "ur", "براہوئی", {"ش": ["ۺ"]}),
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
    ("sd-Deva", "hi", "सिंधी", {"ग": ["ॻ"], "ज": ["ॼ"], "ड": ["ॾ"], "ब": ["ॿ"]}),
    ("gbm", "hi", "गढ़वाली", {"ल": ["ळ"]}),
    ("raj", "hi", "राजस्थानी", {"ल": ["ळ"]}),
    ("mni-Beng", "bn", "মণিপুরী", {"ব": ["ৱ"]}),
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
    ("wsg", "te", "గోండి", {}),
    (
        "lus",
        "us",
        "Mizo",
        {
            "a": ["â"], "e": ["ê"], "i": ["î"], "o": ["ô"], "u": ["û"], "t": ["ṭ"],
            "A": ["Â"], "E": ["Ê"], "I": ["Î"], "O": ["Ô"], "U": ["Û"], "T": ["Ṭ"],
        },
    ),
    ("kha", "us", "Khasi", {"i": ["ï"], "n": ["ñ"], "I": ["Ï"], "N": ["Ñ"]}),
    ("khw", "ur", "کھوار", {"ج": ["ځ", "ݮ"], "چ": ["څ", "ݯ"], "ش": ["ݰ"], "ز": ["ݱ"]}),
]


def build(lang, base, name, extra):
    layout = json.loads((ROOT / "layouts" / f"{base}.json").read_text(encoding="utf-8"))
    out = {}
    for k, v in layout.items():
        if k == "views" and extra:
            # A letter only a long press types: the gesture is on for it.
            out["long_press_letters"] = True
        out[k] = v
    out["id"] = lang.lower()
    out["name"] = name
    out["prediction_lang"] = lang
    added = set()
    for view in out["views"]:
        for row in view["rows"]:
            for key in row["keys"]:
                letters = extra.get(key.get("label"))
                if letters:
                    rest = [k for k in key.get("long_press", []) if k.get("text") not in letters]
                    key["long_press"] = [{"kind": "text", "text": t} for t in letters] + rest
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
        path = ROOT / "layouts" / f"{lang.lower()}.json"
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
