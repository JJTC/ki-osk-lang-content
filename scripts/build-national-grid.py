#!/usr/bin/env python3
"""Rebuild the base+shift letter rows of national layouts that need
dedicated keys for letters the US-QWERTY 26-key grid can't hold.

Most clone layouts (see relabel-national.py) only *relabel* the existing
US grid, which is enough for scripts whose alphabet fits 26 keys (Greek,
Cyrillic) or that already reuse the US letters (Italian, Polish). But
German, the Nordic languages, and Turkish add letters the US grid has no
slot for (ä ö ü, å ø æ, ğ ş ç …). On a physical national keyboard those
sit on the keys US keyboards use for punctuation — evdev codes 26/27 next
to the top row, 39/40 next to the home row, 51/52 next to the bottom row.

This adds exactly those keys, wherever the national xkb layout maps them
to an alphabetic character, producing a wider staggered grid like a
mobile keyboard. Everything is read from libxkbcommon, so nothing is
hand-transcribed — including the QWERTZ y/z swap and the Turkish dotted
vs dotless i. Existing long-press menus are preserved; a few layouts get
an extra long-press entry for a letter with no grid home (German ß lives
on keycode 12 on the number row, so it rides the `s` key's long-press,
exactly as Gboard does).

    python3 scripts/build-national-grid.py [layouts_dir]

Run on any host with libxkbcommon + xkeyboard-config.
"""
import ctypes
import json
import sys

# (layout id, xkb layout, {keycode: [extra long-press KeyOutputs to add]})
# `de` is intentionally absent: it already ships a full QWERTZ grid with
# dedicated ä ö ü AND a ß key, so the generator (which can't place ß —
# it lives on the number row, keycode 12) would only regress it.
TARGETS = [
    ("no", "no", {}),
    ("dk", "dk", {}),
    ("se", "se", {}),
    ("fi", "fi", {}),
    ("tr", "tr", {}),  # already in this shape; regenerating is a no-op
]

# US letter rows, and the adjacent punctuation-position codes a national
# layout may reuse for letters (row 0 top, row 1 home, row 2 bottom).
BASE_ROWS = [list(range(16, 26)), list(range(30, 39)), list(range(44, 51))]
EXTRA_ROWS = [[26, 27], [39, 40], [51, 52]]
KEY_LEFTSHIFT = 42  # evdev


class _Rmlvo(ctypes.Structure):
    _fields_ = [
        ("rules", ctypes.c_char_p),
        ("model", ctypes.c_char_p),
        ("layout", ctypes.c_char_p),
        ("variant", ctypes.c_char_p),
        ("options", ctypes.c_char_p),
    ]


def _lib():
    x = ctypes.CDLL("libxkbcommon.so.0")
    x.xkb_context_new.restype = ctypes.c_void_p
    x.xkb_keymap_new_from_names.restype = ctypes.c_void_p
    x.xkb_keymap_new_from_names.argtypes = [ctypes.c_void_p, ctypes.POINTER(_Rmlvo), ctypes.c_int]
    x.xkb_state_new.restype = ctypes.c_void_p
    x.xkb_state_new.argtypes = [ctypes.c_void_p]
    x.xkb_state_key_get_utf8.restype = ctypes.c_int
    x.xkb_state_key_get_utf8.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_char_p, ctypes.c_size_t]
    x.xkb_state_update_key.restype = ctypes.c_int
    x.xkb_state_update_key.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_int]
    return x


def maps_for(layout):
    """Return ({code: base_char}, {code: shift_char}) for an xkb layout."""
    x = _lib()
    ctx = x.xkb_context_new(0)
    rmlvo = _Rmlvo(b"", b"", layout.encode(), b"", b"")
    km = x.xkb_keymap_new_from_names(ctx, ctypes.byref(rmlvo), 0)
    if not km:
        raise SystemExit(f"libxkbcommon could not compile layout {layout!r}")
    st = x.xkb_state_new(km)
    codes = [c for row in BASE_ROWS + EXTRA_ROWS for c in row]

    def ch(code):
        buf = ctypes.create_string_buffer(32)
        x.xkb_state_key_get_utf8(st, code + 8, buf, 32)  # evdev -> xkb keycode
        return buf.value.decode("utf-8")

    base = {c: ch(c) for c in codes}
    x.xkb_state_update_key(st, KEY_LEFTSHIFT + 8, 1)
    shift = {c: ch(c) for c in codes}
    x.xkb_state_update_key(st, KEY_LEFTSHIFT + 8, 0)
    return base, shift


def rebuild(path, xlayout, add_lp):
    base, shift = maps_for(xlayout)
    # Structure (which extra keys exist) comes from the base level so the
    # base and shift views stay column-aligned.
    extras = [[c for c in row if base[c].isalpha()] for row in EXTRA_ROWS]

    with open(path, encoding="utf-8") as f:
        layout = json.load(f)

    for view in layout.get("views", []):
        vid = view.get("id")
        if vid == "base":
            chars, add = base, add_lp
        elif vid == "shift":
            chars, add = shift, {}  # extra long-press (ß) rides the base level only
        else:
            continue
        rows = view["rows"]

        # Preserve any existing long-press menus, keyed by keycode.
        old_lp = {}
        for row in rows:
            for key in row.get("keys", []):
                out = key.get("output", {})
                if out.get("kind") == "keycode" and key.get("long_press"):
                    old_lp[out["code"]] = key["long_press"]

        # A fresh half-key spacer (staggers the home row). Built explicitly
        # rather than copied from the layout: some clones (e.g. de) have no
        # leading spacer, and copying their first *letter* key would double
        # it. Shift/backspace bookend row 2 and are reused as-is.
        def spacer():
            return {"width_units": 0.5, "label": "", "output": {"kind": "none"}}

        shift_key = rows[2]["keys"][0]
        backspace = rows[2]["keys"][-1]

        def kc(code):
            key = {"width_units": 1, "label": chars[code], "output": {"kind": "keycode", "code": code}}
            lp = add.get(code) or old_lp.get(code)
            if lp:
                key["long_press"] = lp
            return key

        rows[0]["keys"] = [kc(c) for c in BASE_ROWS[0] + extras[0]]
        rows[1]["keys"] = [spacer()] + [kc(c) for c in BASE_ROWS[1] + extras[1]] + [spacer()]
        rows[2]["keys"] = [shift_key] + [kc(c) for c in BASE_ROWS[2] + extras[2]] + [backspace]

    with open(path, "w", encoding="utf-8") as f:
        json.dump(layout, f, ensure_ascii=False, indent=2)
        f.write("\n")
    return extras


def main():
    base_dir = sys.argv[1] if len(sys.argv) > 1 else "layouts"
    for lid, xlayout, add_lp in TARGETS:
        base, _ = maps_for(xlayout)
        extras = rebuild(f"{base_dir}/{lid}.json", xlayout, add_lp)
        added = " ".join(base[c] for row in extras for c in row)
        print(f"{lid} ({xlayout}): dedicated keys = {added or '(none)'}")


if __name__ == "__main__":
    main()
