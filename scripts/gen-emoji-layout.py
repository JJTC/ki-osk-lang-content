#!/usr/bin/env python3
"""Generate the ki-osk emoji picker layout from a Unicode emoji-test.txt.

Produces layouts/emoji.json: four category
views (Smileys / People+Nature / Food+Travel+Activities / Objects+
Symbols+Flags), each a 10-wide grid, ZWJ sequences included as single
entries, and skin-tone-capable emoji carrying a long_press list of the
five toned variants. A control row (backspace, 4 category tabs, ABC,
space, enter) is appended to every view — matching the hand-written
layout it replaces.

Usage: gen-emoji-layout.py <emoji-test.txt> <out.json> [per_view]
"""
import json
import sys
from collections import OrderedDict

TONES = ["light skin tone", "medium-light skin tone", "medium skin tone",
         "medium-dark skin tone", "dark skin tone"]

# 4 picker views → Unicode groups. Tab labels match the control row.
VIEWS = [
    ("base", ["Smileys & Emotion"]),
    ("shift", ["People & Body", "Animals & Nature"]),
    ("symbol1", ["Food & Drink", "Travel & Places", "Activities"]),
    ("symbol2", ["Objects", "Symbols", "Flags"]),
]
COLS = 10


def parse(path):
    """Return (group -> [ (glyph, name) base emoji ]) and skin-tone map."""
    groups = OrderedDict()
    tone_map = {}       # base_name -> {tone_name: glyph}
    base_glyph = {}     # base_name -> glyph
    cur_group = None
    with open(path, encoding="utf-8") as f:
        for line in f:
            if line.startswith("# group:"):
                cur_group = line.split(":", 1)[1].strip()
                continue
            if line.startswith("#") or ";" not in line:
                continue
            code_part, rest = line.split(";", 1)
            status, _, comment = rest.partition("#")
            if status.strip() != "fully-qualified":
                continue
            comment = comment.strip()
            # comment is "<glyph> E<ver> <name>"
            glyph = comment.split(" ", 1)[0]
            # name is after the version token
            parts = comment.split(" ", 2)
            name = parts[2] if len(parts) == 3 else parts[-1]
            if ": " in name and name.rsplit(": ", 1)[1] in TONES:
                base_name = name.rsplit(": ", 1)[0]
                tone = name.rsplit(": ", 1)[1]
                tone_map.setdefault(base_name, {})[tone] = glyph
                continue
            base_glyph[name] = glyph
            groups.setdefault(cur_group, []).append((glyph, name))
    return groups, tone_map


def key(glyph, tones=None):
    k = {"width_units": 1, "label": glyph,
         "output": {"kind": "text", "text": glyph}}
    if tones:
        k["long_press"] = [{"kind": "text", "text": t} for t in tones]
    return k


def control_row():
    return {"height_units": 1, "keys": [
        {"width_units": 1, "label": "⌫", "output": {"kind": "none"}, "action": "backspace"},
        {"width_units": 1, "label": "\U0001f600", "output": {"kind": "none"}, "action": {"switch_view": "base"}},
        {"width_units": 1, "label": "\U0001f9d1", "output": {"kind": "none"}, "action": {"switch_view": "shift"}},
        {"width_units": 1, "label": "\U0001f354", "output": {"kind": "none"}, "action": {"switch_view": "symbol1"}},
        {"width_units": 1, "label": "⚙", "output": {"kind": "none"}, "action": {"switch_view": "symbol2"}},
        {"width_units": 1, "label": "\U0001f50d", "output": {"kind": "none"}, "action": {"switch_view": "search"}},
        {"width_units": 1, "label": "ABC", "output": {"kind": "none"}, "action": "globe"},
        {"width_units": 2, "label": "space", "output": {"kind": "keycode", "code": 57}, "action": "space"},
        {"width_units": 1, "label": "⏎", "output": {"kind": "keycode", "code": 28}, "action": "enter"},
    ]}


def search_view():
    """A QWERTY page that feeds the emoji search query. Letters are text
    keys the daemon intercepts (in ViewId::Search) into the query rather
    than committing; ☺ returns to the picker, ⌫ edits the query."""
    def t(ch):
        return {"width_units": 1, "label": ch, "output": {"kind": "text", "text": ch}}

    def spacer(w):
        return {"width_units": w, "label": "", "output": {"kind": "none"}}

    bksp = {"width_units": 1.5, "label": "⌫", "output": {"kind": "none"}, "action": "backspace"}
    return {"id": "search", "rows": [
        {"height_units": 1, "keys": [t(c) for c in "qwertyuiop"]},
        {"height_units": 1, "keys": [spacer(0.5)] + [t(c) for c in "asdfghjkl"] + [spacer(0.5)]},
        {"height_units": 1, "keys": [spacer(1.5)] + [t(c) for c in "zxcvbnm"] + [bksp]},
        {"height_units": 1, "keys": [
            {"width_units": 2, "label": "\U0001f600", "output": {"kind": "none"}, "action": {"switch_view": "base"}},
            {"width_units": 8, "label": "space", "output": {"kind": "text", "text": " "}},
        ]},
    ]}


def build_view(vid, glyphs, tone_map, per_view):
    keys = []
    for glyph, name in glyphs[:per_view]:
        tones = None
        if name in tone_map:
            ordered = [tone_map[name][t] for t in TONES if t in tone_map[name]]
            if ordered:
                tones = ordered
        keys.append(key(glyph, tones))
    rows = []
    for i in range(0, len(keys), COLS):
        rows.append({"height_units": 1, "keys": keys[i:i + COLS]})
    rows.append(control_row())
    return {"id": vid, "rows": rows}


def main():
    src, out = sys.argv[1], sys.argv[2]
    per_view = int(sys.argv[3]) if len(sys.argv) > 3 else 80
    groups, tone_map = parse(src)
    views = []
    for vid, grp_names in VIEWS:
        glyphs = []
        for g in grp_names:
            glyphs.extend(groups.get(g, []))
        views.append(build_view(vid, glyphs, tone_map, per_view))
    views.append(search_view())
    layout = OrderedDict([
        ("id", "emoji"), ("name", "Emoji picker"),
        ("swipe_enabled", False), ("prediction_enabled", False),
        ("views", views),
    ])
    with open(out, "w", encoding="utf-8") as f:
        json.dump(layout, f, ensure_ascii=False, indent=2)
        f.write("\n")
    total = sum(sum(len(r["keys"]) for r in v["rows"][:-1]) for v in views)
    print(f"wrote {out}: {len(views)} views, {total} emoji")


if __name__ == "__main__":
    main()
