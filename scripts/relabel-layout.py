#!/usr/bin/env python3
"""Relabel a keycode-based layout's keycaps for a national layout.

The 18 "clone" layouts already emit the right characters (the daemon
uploads an xkb keymap from `xkb_layout`, so an evdev keycode maps to the
locale letter). Only the keycaps (JSON `label`s) were wrong — US QWERTY.
This rewrites the letter labels by evdev keycode, keeping the keycodes,
control keys, and structure untouched, so what the user SEES matches
what the key produces.

Maps are the standard national arrangements. evdev keycodes:
  row1 q..p = 16..25, [ ] = 26 27
  row2 a..l = 30..38, ; ' = 39 40
  row3 z..m = 44..50, , . / = 51 52 53
"""
import json
import sys

MAPS = {
    # Russian ЙЦУКЕН (SI standard).
    "ru": {
        16: "й", 17: "ц", 18: "у", 19: "к", 20: "е", 21: "н", 22: "г",
        23: "ш", 24: "щ", 25: "з", 26: "х", 27: "ъ",
        30: "ф", 31: "ы", 32: "в", 33: "а", 34: "п", 35: "р", 36: "о",
        37: "л", 38: "д", 39: "ж", 40: "э",
        44: "я", 45: "ч", 46: "с", 47: "м", 48: "и", 49: "т", 50: "ь",
        51: "б", 52: "ю",
    },
    # Hebrew (SI-1452 standard). Layout is RTL (direction set in metadata).
    "he": {
        16: "/", 17: "'", 18: "ק", 19: "ר", 20: "א", 21: "ט", 22: "ו",
        23: "ן", 24: "ם", 25: "פ",
        30: "ש", 31: "ד", 32: "ג", 33: "כ", 34: "ע", 35: "י", 36: "ח",
        37: "ל", 38: "ך", 39: "ף", 40: ",",
        44: "ז", 45: "ס", 46: "ב", 47: "ה", 48: "נ", 49: "מ", 50: "צ",
        51: "ת", 52: "ץ",
    },
}

# Laid out by hand now, each as its language's phone keyboard (AOSP's
# arrangement) with every letter on a key: regenerating them would cut
# them back to the US grid, as these scripts first made them.
HAND_MADE = {"he", "ru"}


def relabel(path, mapping):
    with open(path, encoding="utf-8") as f:
        layout = json.load(f)
    changed = 0
    for view in layout.get("views", []):
        for row in view.get("rows", []):
            for key in row.get("keys", []):
                out = key.get("output", {})
                if out.get("kind") == "keycode":
                    code = out.get("code")
                    if code in mapping:
                        key["label"] = mapping[code]
                        changed += 1
    with open(path, "w", encoding="utf-8") as f:
        json.dump(layout, f, ensure_ascii=False, indent=2)
        f.write("\n")
    return changed


def main():
    base = sys.argv[1] if len(sys.argv) > 1 else \
        "layouts"
    for lang, mapping in MAPS.items():
        if lang in HAND_MADE:
            continue
        path = f"{base}/{lang}.json"
        n = relabel(path, mapping)
        print(f"{lang}: relabelled {n} keycaps")


if __name__ == "__main__":
    main()
