#!/usr/bin/env python3
"""Relabel national keycode-based layouts with native keycaps.

The clone layouts emit the right characters (the daemon uploads an xkb
keymap from `xkb_layout`, so an evdev keycode produces the locale
character) but for non-Latin scripts the keycaps still read US QWERTY.
This computes the *actual* per-keycode characters from the real xkb
layout via `libxkbcommon` and writes them onto the keycaps, for both the
base view (level 1) and the shift view (level 2). It also drops the
US-Latin accent long-press menus, which don't apply to these scripts.

Because it asks libxkbcommon directly, the caps are guaranteed to match
what the key produces — no hand-transcribed Greek/Arabic/Thai/Persian/
Ukrainian tables to get wrong. Run on any host with libxkbcommon +
xkeyboard-config (rules/symbols data), e.g. a dev Fedora box.

    python3 scripts/relabel-national.py [layouts_dir]
"""
import ctypes
import json
import sys

# (layout id, xkb layout, xkb variant). ru/he are already localized by
# the older hand-transcribed relabel-layout.py; these are the ones that
# still showed Latin caps. Thai was one, until build-layout.py made it
# with every key of its keymap: the 26 US letter keys this relabels held
# only 47 of its 74 characters.
TARGETS = [
    ("el", "gr", ""),
    ("ar", "ara", ""),
    ("fa", "ir", ""),
    ("uk", "ua", ""),
]

# Latin-script locales that reuse the US-QWERTY grid. Their base alphabet
# already matches the US caps, so relabelling only touches the handful of
# keys the national xkb layout genuinely remaps: Czech QWERTZ (y<->z) and
# Turkish (dotless 'i' on the US-'i' key). The other Latin locales
# (it/nl/no/dk/se/fi/pl/pt-br/vi) come out unchanged — their accented
# letters live on long-press, not the base grid. Unlike the scripts
# above, these long-press accent menus (a -> á à â ä …) DO apply here, so
# `keep_long_press` leaves them in place.
LATIN_TARGETS = [
    ("cz", "cz", ""),
    ("tr", "tr", ""),
]

# evdev keycodes on the three letter rows (+ the adjacent punctuation the
# national layouts reuse): q..] , a..' , z../
CODES = list(range(16, 28)) + list(range(30, 41)) + list(range(44, 54))
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


def maps_for(layout, variant):
    """Return ({code: base_char}, {code: shift_char}) for an xkb layout."""
    x = _lib()
    ctx = x.xkb_context_new(0)
    rmlvo = _Rmlvo(b"", b"", layout.encode(), variant.encode(), b"")
    km = x.xkb_keymap_new_from_names(ctx, ctypes.byref(rmlvo), 0)
    if not km:
        raise SystemExit(f"libxkbcommon could not compile layout {layout!r}")
    st = x.xkb_state_new(km)

    def ch(code):
        buf = ctypes.create_string_buffer(32)
        x.xkb_state_key_get_utf8(st, code + 8, buf, 32)  # evdev -> xkb keycode
        return buf.value.decode("utf-8")

    base = {c: ch(c) for c in CODES}
    x.xkb_state_update_key(st, KEY_LEFTSHIFT + 8, 1)  # hold shift
    shift = {c: ch(c) for c in CODES}
    x.xkb_state_update_key(st, KEY_LEFTSHIFT + 8, 0)
    return base, shift


def relabel(path, base, shift, keep_long_press=False):
    with open(path, encoding="utf-8") as f:
        layout = json.load(f)
    changed = 0
    for view in layout.get("views", []):
        vid = view.get("id")
        chars = base if vid == "base" else shift if vid == "shift" else None
        for row in view.get("rows", []):
            for key in row.get("keys", []):
                if not keep_long_press:
                    key.pop("long_press", None)  # US-Latin accent menus don't apply
                out = key.get("output", {})
                if chars is not None and out.get("kind") == "keycode":
                    label = chars.get(out.get("code"), "")
                    if label.strip():
                        key["label"] = label
                        changed += 1
    with open(path, "w", encoding="utf-8") as f:
        json.dump(layout, f, ensure_ascii=False, indent=2)
        f.write("\n")
    return changed

# Laid out by hand now, each as its language's phone keyboard (AOSP's
# arrangement) with every letter on a key: regenerating them would cut
# them back to the US grid, as these scripts first made them.
HAND_MADE = {"ar", "fa", "uk"}


def main():
    base_dir = sys.argv[1] if len(sys.argv) > 1 else "layouts"
    for lid, xlayout, xvariant in TARGETS:
        if lid in HAND_MADE:
            continue
        base, shift = maps_for(xlayout, xvariant)
        n = relabel(f"{base_dir}/{lid}.json", base, shift)
        row0 = " ".join(base[c] for c in range(16, 26))
        print(f"{lid} ({xlayout}): {n} caps relabelled; base q..p = {row0}")
    for lid, xlayout, xvariant in LATIN_TARGETS:
        base, shift = maps_for(xlayout, xvariant)
        n = relabel(f"{base_dir}/{lid}.json", base, shift, keep_long_press=True)
        row0 = " ".join(base[c] for c in range(16, 26))
        print(f"{lid} ({xlayout}): {n} caps set, long-press kept; base q..p = {row0}")


if __name__ == "__main__":
    main()
