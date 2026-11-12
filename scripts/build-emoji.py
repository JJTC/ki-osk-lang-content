#!/usr/bin/env python3
"""Build the full emoji *search* catalogue from Unicode data.

The picker grid (emoji.json) is a curated 320-emoji subset built by
gen-emoji-layout.py. This produces the separate, complete keyword index
the search bar queries — every fully-qualified emoji with its CLDR short
name, CLDR keywords, category, and skin-tone variants — so a search for
"burrito" or "shrug" finds emoji that never fit on the picker tabs.

Inputs:
  - Unicode emoji-test.txt          (glyph, group, name, skin tones)
  - CLDR annotations  en.xml        (tts short name + keywords)
  - CLDR annotationsDerived en.xml  (names/keywords for flags, families)

Usage:
  scripts/build-emoji.py emoji-test.txt en.xml enDerived.xml emoji/catalogue.json
"""
import json
import sys
import xml.etree.ElementTree as ET
from collections import OrderedDict

TONES = ["light skin tone", "medium-light skin tone", "medium skin tone",
         "medium-dark skin tone", "dark skin tone"]

# emoji-test.txt group -> EmojiCategory serde (kebab-case). "Component"
# (the bare skin-tone modifiers) is intentionally dropped.
CATEGORY = {
    "Smileys & Emotion": "smileys-emotion",
    "People & Body": "people-body",
    "Animals & Nature": "animals-nature",
    "Food & Drink": "food-drink",
    "Travel & Places": "travel-places",
    "Activities": "activities",
    "Objects": "objects",
    "Symbols": "symbols",
    "Flags": "flags",
}


def parse_emoji_test(path):
    """Return an ordered list of (glyph, name, category, [tone glyphs])."""
    entries = OrderedDict()   # base_name -> [glyph, category, {tone: glyph}]
    group = None
    with open(path, encoding="utf-8") as f:
        for line in f:
            if line.startswith("# group:"):
                group = line.split(":", 1)[1].strip()
                continue
            if line.startswith("#") or ";" not in line:
                continue
            _, rest = line.split(";", 1)
            status, _, comment = rest.partition("#")
            if status.strip() != "fully-qualified":
                continue
            cat = CATEGORY.get(group)
            if cat is None:
                continue
            comment = comment.strip()
            glyph, _, tail = comment.partition(" ")     # "<glyph> E<ver> <name>"
            name = tail.split(" ", 1)[1] if " " in tail else tail
            if ": " in name and name.rsplit(": ", 1)[1] in TONES:
                base, tone = name.rsplit(": ", 1)
                if base in entries:
                    entries[base][2][tone] = glyph
                continue
            entries[name] = [glyph, cat, {}]
    return entries


def parse_annotations(*paths):
    """Merge CLDR annotation files: glyph -> (tts_name, [keywords])."""
    tts, kw = {}, {}
    for path in paths:
        root = ET.parse(path).getroot()
        ns = root.find("annotations")
        for a in (ns.iter("annotation") if ns is not None else []):
            cp = a.get("cp")
            if not cp:
                continue
            text = (a.text or "").strip()
            if a.get("type") == "tts":
                tts[cp] = text
            else:
                kw[cp] = [k.strip() for k in text.split("|") if k.strip()]
    return tts, kw


def _variants(glyph):
    """A glyph plus its FE0F-stripped form, for robust CLDR lookup."""
    yield glyph
    stripped = glyph.replace("️", "")
    if stripped != glyph:
        yield stripped


def main():
    et_path, en_path, der_path, out = sys.argv[1:5]
    entries = parse_emoji_test(et_path)
    tts, kw = parse_annotations(en_path, der_path)

    out_list = []
    named = 0
    for name, (glyph, cat, tone_map) in entries.items():
        cldr_name, keywords = None, []
        for g in _variants(glyph):
            if g in tts:
                cldr_name = tts[g]
            if g in kw:
                keywords = kw[g]
            if cldr_name or keywords:
                break
        if cldr_name:
            named += 1
        tones = [tone_map[t] for t in TONES if t in tone_map]
        out_list.append({
            "glyph": glyph,
            "name": cldr_name or name,
            "keywords": " ".join(keywords),
            "category": cat,
            "skin_tones": tones,
        })

    with open(out, "w", encoding="utf-8") as f:
        json.dump(out_list, f, ensure_ascii=False, separators=(",", ":"))
        f.write("\n")
    print(f"wrote {len(out_list)} emoji ({named} with CLDR names, "
          f"{sum(1 for e in out_list if e['skin_tones'])} skin-tone-capable) to {out}")


if __name__ == "__main__":
    main()
