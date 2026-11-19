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
full stop.

Thirteen more are keyboards of a script of their own, made the same way
for languages no text here predicts: Chakma's (easy_chakma), Rohingya's
in the Hanifi script (hanifi_rohingya, right to left), Tai Dam's
(sil_tai_dam), Western Kayah's in Kayah Li (sil_kayah_kali), Lisu's
(sil_lisu_standard), Lepcha's (sil_lepcha), Khmu's in the Lao script
(sil_kmhmu), Saurashtra's (saurashtra_inscript), Bamum's (bamum),
Eastern Cham's (sil_cham_phonetic), Tai Nuea's in Tai Le
(tainua), Tai Lue's in New Tai Lue (ntl_onekey) and Osage's
(osage_nation). Their keys keep Keyman's rows and places: a key that
types nothing of the script (an ASCII digit, a Latin letter, a symbol
the symbol views have) is a gap, a row of gaps on both views goes, and
a row of the script's own on one view only is on both (Chakma's
digits); every row is as wide as the widest, its keys as wide as the
others, Shift and Backspace taking the last letter row's rest. A key's
label is what it types (Khmu's ກຸ labels ຸ). What a key types under
AltGr (Cham's ꨒ, on ꨗ's key) is first on its long press, and a letter
Keyman types by tapping a key again or after another (Tai Dam's ꪂ, ꪀ
twice; Lepcha's ᰅ, ᰍ then g) is on the long press of the first key. A
letter of the bottom row (Bamum's two marks beside the space bar,
Osage's acute) is at the last letter row's end; the script's own
digits are on the long presses of the symbol view's (Saurashtra's ꣑ on
1), and its punctuation no key keeps (Saurashtra's danda ꣎) on the full
stop's. A long press offering a letter no tap types turns the gesture
on for the layout (`long_press_letters`): on a kiosk, whose long-press
menus are off by default, Ojibwe's long vowels could not be typed
without it.

The views of symbols and the control row are the US template's, as
scripts/build-layout.py's layouts' are; every key types text, so no
keymap is needed (xkb_layout names the script's, Burmese's mm and
Canada's ca, for the keys the control row types by keycode; the scripts
with no keymap name none, as Amharic's layout).

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
COMMIT = "7225701d383cf8317f595bfd73481fcd87524f44"
# The keyboards added since are read at a later commit (2026-11-17), each
# pinned as it is.
LATER = "c9a2da84df94cc0acb5b0a684615e5e91ca95a0a"

# The layout made, its name, the keyboard (its path in keymanapp/keyboards
# and the commit), and the sha256 of its touch layout, rules and licence.
KEYBOARDS = [
    ("ksw", "ကညီ", "release/sil/sil_sgaw_karen", COMMIT, {
        "source/sil_sgaw_karen.keyman-touch-layout":
            "7ea0e10ec4dbcc92a7d0ed700cd48f9dc4f5405513f6413d61bd64277d9a19fb",
        "source/sil_sgaw_karen.kmn": "915740a85216b9553abae0b7dec0bfa3ecbbc18d922e00bd9c3d1bf6212461bd",
        "LICENSE.md": "dbb1d1f49159d4713f308d293eb2d3a201ae495332600f729cea6f22453bf78a",
    }),
    ("ojb", "ᐊᓂᔑᓇᐯᒧᐎᓐ", "release/fv/fv_ojibwa", COMMIT, {
        "source/fv_ojibwa.keyman-touch-layout": "79ff1490dee2a14f54472730ff6f762709e05970bb9b16d35df35c585ace723d",
        "source/fv_ojibwa.kmn": "7b3a98522842d6061158ae8f6c05c6f3f1d6d62b998c55976c7a6e7e63e68764",
        "LICENSE.md": "d0ca606a218e493ce0f0e8374e6972d35a783cbcd8aa15b0ef0f91998f56dca3",
    }),
    # Named as CLDR's files name the languages in their own scripts
    # (ccp blt rhg osa); the others, which CLDR has no file of, in English.
    ("ccp", "𑄌𑄋𑄴𑄟𑄳𑄦", "release/e/easy_chakma", COMMIT, {
        "source/easy_chakma.keyman-touch-layout": "65c13617196aa9bae3a701de2ac19b3042de4c92904f9ce5cf49a92b0b3ff91b",
        "source/easy_chakma.kmn": "1a2d27e484af789a05f786a6c0ce8de7b14dbc19c26221cf029dc943bbf22ddb",
        "LICENSE.md": "e403bbb9529c2b988c9ba9acb947f27beb42f5f2c9361067961314cb03b72dee",
    }),
    ("rhg", "𐴌𐴗𐴥𐴝𐴙𐴚𐴒𐴙𐴝", "release/h/hanifi_rohingya", COMMIT, {
        "source/hanifi_rohingya.keyman-touch-layout": "021022f798c2f983441656852ec0e4e067c3000341bfe44492f076110f00ad00",
        "source/hanifi_rohingya.kmn": "e6bf13d0fa9b8d603003ffa684cc3481fe8c5a06f5b0581d5574f8064a51c72d",
        "LICENSE.md": "c4426616dbcfc315ae1282aa53b7d45829491c00d760b4b0dc44199f158976a5",
    }),
    ("blt", "ꪼꪕꪒꪾ", "release/sil/sil_tai_dam", COMMIT, {
        "source/sil_tai_dam.keyman-touch-layout": "8b5bc20da3cfa23b856235c170418236ff58e3fae229f66a25d96099cf3c321a",
        "source/sil_tai_dam.kmn": "37c063db38add5021a90373da617fda525f6eea118b3fae8627d014c7e55e8af",
        "LICENSE.md": "2de14ee73522a3da860ac0ab57bc580e958ecbd012226e4bff2d0a662d11020a",
    }),
    ("kyu", "Western Kayah", "release/sil/sil_kayah_kali", COMMIT, {
        "source/sil_kayah_kali.keyman-touch-layout": "a71bd818cd70d257a4a196c32403add0bbadbfdb6e8b13e50ce22bf19eac3e3b",
        "source/sil_kayah_kali.kmn": "03ce41be9904c3c9accc0d5c62bc3d419dbd33aa3b57428661fc72a47e26359c",
        "LICENSE.md": "de1f60a8d68b17fa13d2bd81cdd594563d01fad4c816cf3a5ad3c613dc16cacd",
    }),
    ("lis", "Lisu", "release/sil/sil_lisu_standard", COMMIT, {
        "source/sil_lisu_standard.keyman-touch-layout": "942b28d2add5fbb6c44adb7e27747c5204cf22c5729f65428c79735d5c21bf0b",
        "source/sil_lisu_standard.kmn": "24f30e55c5a3a975f7c04234fccdd8a585587aa3f84a5b6251e2b461c4284d37",
        "LICENSE.md": "7946ae1f9d06d99e81a383950ea9a1581c23f9bd46ba695c4e2f898a89013628",
    }),
    ("lep", "Lepcha", "release/sil/sil_lepcha", COMMIT, {
        "source/sil_lepcha.keyman-touch-layout": "dd5a1999e984c0a78b6abc2d98dadd58e2fa52c8c2df7b27ceb0d19de28cb0bc",
        "source/sil_lepcha.kmn": "a7131e200fdeb3de0eb3ef3c350f297830bf3a88d5d015f287c8249aa97ec458",
        "LICENSE.md": "d73a3eaa0e2841721ccff36a4f1c394b4f812e75e5a8661683e76cf432e4302b",
    }),
    ("kjg", "Khmu", "release/sil/sil_kmhmu", COMMIT, {
        "source/sil_kmhmu.keyman-touch-layout": "5e9e9ea194315c5b863c781f796f38e29f1d374279affdcad9a01414e91234b1",
        "source/sil_kmhmu.kmn": "5e4b2f0a2ec82b1d9f14b798959124ed1a42190e42630b25516d5f62f3da1199",
        "LICENSE.md": "05e4b063c1bf783a0af3456e1fd2821aae4b23cf332a1b9a2b79c79bccadb366",
    }),
    ("saz", "Saurashtra", "release/s/saurashtra_inscript", COMMIT, {
        "source/saurashtra_inscript.keyman-touch-layout": "448bb781628d4b880e6f40ccd84594d8f1d08132ece6b16489e5db200cdebcff",
        "source/saurashtra_inscript.kmn": "464e4bf53e1cda23b2816c5eabe02ba18cdaaa87544ca8db5c3c986d6bf4350e",
        "LICENSE.md": "c4426616dbcfc315ae1282aa53b7d45829491c00d760b4b0dc44199f158976a5",
    }),
    ("bax", "Bamum", "release/b/bamum", COMMIT, {
        "source/bamum.keyman-touch-layout": "0ba7c43b542d6a3ec04eb358fa96d092a09d09469a38bfe5b7da85955b743c89",
        "source/bamum.kmn": "f0e139a4b7eecf707a479689d0c4ff18b2248f2665268f469d3e404d81473842",
        "LICENSE.md": "c4426616dbcfc315ae1282aa53b7d45829491c00d760b4b0dc44199f158976a5",
    }),
    ("cjm", "Eastern Cham", "release/sil/sil_cham_phonetic", COMMIT, {
        "source/sil_cham_phonetic.keyman-touch-layout": "7d03fa84f8bd3c9e9b6f49bca50a460a943ffa4235c16e19aae49f1c43dff046",
        "source/sil_cham_phonetic.kmn": "a4fa86b67fe02a3f8888e9d45d23fe62ab684cfc50bb754a6ff849e47b78615b",
        "LICENSE.md": "3b311195cd667f204af1c7015abbd4a23c5eed9703121ddab1cac6b803899d7e",
    }),
    ("tdd", "Tai Nuea", "release/t/tainua", COMMIT, {
        "source/tainua.keyman-touch-layout": "c5868fcb5528270cbdf315a93b9d20b0895e71b5f72c875e2c223233b7d5816d",
        "source/tainua.kmn": "a7111eacd05af03306810a6ab2c7f790f823cef9650c7f4aef98d31b49cd8472",
        "LICENSE.md": "1a246d375041755584108ef3e7d18189b05105506efae7168f6fed8d5f6e000a",
    }),
    ("khb", "Tai Lue", "release/n/ntl_onekey", COMMIT, {
        "source/ntl_onekey.keyman-touch-layout": "19fa290c236a0801b91d092c9ff8822497a7cf9976d60b358245de20edec2d01",
        "source/ntl_onekey.kmn": "ff73811a868c0188e8e306d1c3b5cc246bcd38061a00c58d356c130448c9dbae",
        "LICENSE.md": "576a6be3a3a735a8e5aade28137b061ce6091e3c3b1f650eb5c927efa53d2bf9",
    }),
    ("osa", "𐓏𐓘𐓻𐓘𐓻𐓟", "release/o/osage_nation", COMMIT, {
        "source/osage_nation.keyman-touch-layout": "34161264a6a2dc4762b8efe6629946c5e480ee3374fcacfeb580f1116162625a",
        "source/osage_nation.kmn": "ae199fd0401d0f63711a68f787099f15f18a5ef688e7cab7e2fce2a195e6729c",
        "LICENSE.md": "ab1714c0b4775087a6a8bcebf28913557704d1105d54993dd8fb2a7aec27d052",
    }),
    # In English, as CLDR's files of them name none.
    ("hac", "Gorani", "release/h/horami", LATER, {
        "source/horami.keyman-touch-layout": "391280eaf8f08fefeb4755ee9e39a81b2890566d6c7be348e2a5aa08299cdabf",
        "source/horami.kmn": "09395069fb2afd85d9eba4fca4de343212fe8a42b77d0a84ac880d2306a21846",
        "LICENSE.md": "81d375a5237167bac4afe074c26f77b1ba01c88a5385c42195ce3ac4cf597fb1",
    }),
    ("psi", "Southeastern Pashayi", "release/p/pashai", LATER, {
        "source/pashai.keyman-touch-layout": "a7616312262ff5468015e15f8ba31169a4211b10109e76bec3b19dfb7dda21ed",
        "source/pashai.kmn": "576a9c3460d42f75c99e23414c449513b05ef79391deba4ca5176c54735fa77d",
        "LICENSE.md": "bda30053d5709bc4c750bd7ace4e734f3747498d65e0af1f0d9d5b3ec26444b9",
    }),
    ("hnd", "Southern Hindko", "release/rac/rac_hindko", LATER, {
        "source/rac_hindko.keyman-touch-layout": "7eac5c11e53da5d4364b97e0d3ca34b1aabc596629043fd81ecc21627157d6c1",
        "source/rac_hindko.kmn": "de04d79e73b3643b1485cc5bf0af45505dead0dd2b8d1079c78cb3c83ca4ecd9",
        "LICENSE.md": "47252b585b03e63562dea34963c52edb8944a2b78b15f06f1689d4496098113c",
    }),
    ("cld", "Chaldean Neo-Aramaic", "release/e/east_syriac_qwerty", LATER, {
        "source/east_syriac_qwerty.keyman-touch-layout":
            "4635aa584ba7b0697338da15cb5d63b9dca12545c8e63f1ddaaa929804f931f9",
        "source/east_syriac_qwerty.kmn": "d51b1c130caff5f69d87085a3623fe7d9263d9ba4ae1ea15fa6a9cf7d75d6cf6",
        "LICENSE.md": "5d40eccf69f560666b8428d786a1da70e336e8b8d2ee0b3ee64ea15a1f053d44",
    }),
    ("nod", "Northern Thai", "experimental/t/tai_lanna_thai", LATER, {
        "source/tai_lanna_thai.keyman-touch-layout": "7e5178041d9a3d1d3f0b51b45106ffa01d43035f5c6997113a4e3b5e9e59762a",
        "source/tai_lanna_thai.kmn": "06241564b16f0fd214b37a4f12b235d8a11eee3d8f07ef436712b08e87e32590",
        "LICENSE.md": "6e391bf5b8db2d0a31e1b4227e4edb92dc314253389fd21ca7ab1dbfe8322d14",
    }),
]

# Each keyboard's touch platform, the keymap its control row types
# through (none for a script with no keymap), and its script's font.
PLATFORM = {"ksw": "phone", "ojb": "tablet", "ccp": "tablet", "rhg": "phone", "blt": "tablet", "kyu": "tablet",
            "lis": "tablet", "lep": "tablet", "kjg": "tablet", "saz": "phone", "bax": "phone", "cjm": "tablet",
            "tdd": "phone", "khb": "phone", "osa": "phone", "hac": "phone", "psi": "tablet", "hnd": "tablet",
            "cld": "phone", "nod": "tablet"}
XKB_LAYOUT = {"ksw": "mm", "ojb": "ca", "kjg": "la", "hac": "iq", "psi": "af", "hnd": "pk", "cld": "sy"}
FONT_SCRIPT = {"ksw": "myanmar", "ojb": "canadian_aboriginal", "ccp": "chakma", "rhg": "hanifi_rohingya",
               "blt": "tai_viet", "kyu": "kayah_li", "lis": "lisu", "lep": "lepcha", "kjg": "lao",
               "saz": "saurashtra", "bax": "bamum", "cjm": "cham", "tdd": "tai_le", "khb": "new_tai_lue",
               "osa": "osage", "hac": "arabic", "psi": "arabic", "hnd": "arabic", "cld": "syriac",
               "nod": "tai_tham"}
RTL = {"rhg", "hac", "psi", "hnd", "cld"}

# The layer a keyboard's Shift key turns to, where it is not `shift`:
# East Syriac's vowel points and marks.
SHIFT_LAYER = {"cld": "ES-diac"}

# The keyboards laid out as Keyman's rows and places are (see above): the
# others, Sgaw Karen's and Ojibwe's, take their first three rows of keys
# as they are.
PLACED = {lang for lang, *_ in KEYBOARDS} - {"ksw", "ojb"}

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
LONG_PRESS_ADDED = {("ojb", "\u1483"): ["\u157D"],
                    # Rohingya's kinna wa, in its own name and on no key of
                    # Keyman's phone layout, as kinna ya is on a long press.
                    ("rhg", "\U00010D16"): ["\U00010D17"],
                    # Hindko's retroflex nasal, which its keyboard types on a
                    # layer of symbols, on noon's, as Punjabi's and Saraiki's.
                    ("hnd", "\u0646"): ["\u0768"],
                    # Tai Tham's high ya, which CLDR's exemplar characters
                    # list and the keyboard lacks, on low ya's.
                    ("nod", "\u1A3F"): ["\u1A40"]}
FULL_STOP_LONG_PRESS = {"ojb": "\u166E"}
# Layers a keyboard has that the language does not write with: East
# Syriac's keyboard's West Syriac vowels, which Chaldean does not use.
UNUSED_LAYERS = {("cld", "WS-diac")}

SPECIAL = {"K_SHIFT", "K_BKSP", "K_ENTER", "K_SPACE", "K_NUMLOCK", "K_LOPT", "K_LOWER", "K_ROPT", "K_TAB", "K_CAPS"}
# Touch keys that type nothing: Keyman's special keys and its spacers.
NO_LETTER = {"1", "2", "10"}
# Keyman's blank keys and spacers, which hold a place in a row.
GAP = {"9", "10"}

# The US keys' characters, unshifted and shifted, for the rules that name
# a key by what it types on them (+ 'q' > ..., store(k) "qwerty").
US = {" ": ("", "K_SPACE")}
for _c in "abcdefghijklmnopqrstuvwxyz":
    US[_c] = ("", "K_" + _c.upper())
    US[_c.upper()] = ("SHIFT", "K_" + _c.upper())
for _c, _s in zip("1234567890", "!@#$%^&*()"):
    US[_c] = ("", "K_" + _c)
    US[_s] = ("SHIFT", "K_" + _c)
for _a, _b, _k in [("`", "~", "K_BKQUOTE"), ("-", "_", "K_HYPHEN"), ("=", "+", "K_EQUAL"),
                   ("[", "{", "K_LBRKT"), ("]", "}", "K_RBRKT"), ("\\", "|", "K_BKSLASH"),
                   (";", ":", "K_COLON"), ("'", '"', "K_QUOTE"), (",", "<", "K_COMMA"),
                   (".", ">", "K_PERIOD"), ("/", "?", "K_SLASH")]:
    US[_a] = ("", _k)
    US[_b] = ("SHIFT", _k)

# A touch layer's modifiers, as its rules name them.
LAYER_MODS = {"default": "", "shift": "SHIFT", "rightalt": "RALT", "rightalt-shift": "RALT SHIFT",
              # East Syriac's layer of marks, whose keys type as with none
              # (its K_1, labelled ◌ܲ, as + [K_1] > U+0732 types).
              "ES-diac": ""}

TOKEN = re.compile(r"""
    '(?P<sq>[^']*)' | "(?P<dq>[^"]*)"
  | U\+(?P<u>[0-9A-Fa-f]{1,6})
  | \[(?P<mods>(?:(?:L|R)?(?:SHIFT|CTRL|ALT|CAPS|NCAPS)\s+)*)(?P<key>[KTU]_\w+)\]
  | (?P<fn>any|index|outs|deadkey|dk|context|nul|beep|use|notany|if|set|reset|platform|layer|baselayout|save|call)
    (?:\((?P<arg>[^)]*)\))?
  | d(?P<d>\d+) | x(?P<x>[0-9A-Fa-f]+)
  | (?P<range>\.\.) | (?P<plus>\+) | (?P<gt>>)
""", re.X)


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


def uncomment(line):
    """A line of rules without its comment (c and a space, outside quotes)."""
    quote = None
    for i, ch in enumerate(line):
        if quote:
            if ch == quote:
                quote = None
        elif ch in "'\"":
            quote = ch
        elif ch == "c" and (i == 0 or line[i - 1].isspace()) and (i + 1 == len(line) or line[i + 1].isspace()):
            return line[:i].strip()
    return line.strip()


def tokens(text):
    out, pos = [], 0
    while pos < len(text):
        if text[pos].isspace():
            pos += 1
            continue
        m = TOKEN.match(text, pos)
        if not m:
            return None
        pos = m.end()
        if m.group("sq") is not None or m.group("dq") is not None:
            out.append(("str", m.group("sq") if m.group("sq") is not None else m.group("dq")))
        elif m.group("u"):
            out.append(("str", chr(int(m.group("u"), 16))))
        elif m.group("key"):
            out.append(("key", (" ".join(sorted(m.group("mods").split())), m.group("key"))))
        elif m.group("fn"):
            out.append((m.group("fn"), (m.group("arg") or "").strip()))
        elif m.group("d"):
            out.append(("str", chr(int(m.group("d")))))
        elif m.group("x"):
            out.append(("str", chr(int(m.group("x"), 16))))
        else:
            out.append((m.lastgroup, None))
    return out


class Rules:
    """A .kmn's stores and the rules of its keys group: what a key types
    with no context (`free`), and the letters a key types after another
    (`after`: the letter before, the key, what it becomes)."""

    def __init__(self, kmn):
        text = re.sub(r"\\\s*\n", " ", kmn.decode("utf-8-sig"))
        lines = [uncomment(line) for line in text.splitlines()]
        self.stores, self.free, self.after = {}, {}, []
        for line in lines:
            m = re.match(r"store\(&?(\w+)\)\s*(.*)$", line, re.I)
            if m:
                self.stores[m.group(1)] = self.members(tokens(m.group(2)) or [])
        keys_group = False
        for line in lines:
            m = re.match(r"group\(\w+\)\s*(using keys)?", line, re.I)
            if m:
                keys_group = bool(m.group(1))
                continue
            toks = tokens(line) if keys_group and ">" in line else None
            if not toks or ("plus", None) not in toks or ("gt", None) not in toks:
                continue
            plus, gt = toks.index(("plus", None)), toks.index(("gt", None))
            if gt != plus + 2:
                continue
            context, key, output = toks[:plus], toks[plus + 1], toks[gt + 1:]
            if key[0] == "any":
                keyed = list(enumerate(self.stores.get(key[1], [])))
            elif key[0] == "key":
                keyed = [(None, key[1])]
            elif key[0] == "str" and len(key[1]) == 1:
                keyed = [(None, key[1])]
            else:
                continue
            for index, k in keyed:
                k = k if isinstance(k, tuple) else US.get(k)
                if k is None:
                    continue
                if not context:
                    text_out = self.output(output, index)
                    if text_out is not None:
                        self.free.setdefault(k, text_out)
                elif len(context) == 1 and context[0][0] == "str" and len(context[0][1]) == 1:
                    text_out = self.output(output, index)
                    if text_out is not None and len(text_out) == 1:
                        self.after.append((context[0][1], k, text_out))
                elif len(context) == 1 and context[0][0] == "any" and index is None:
                    # A letter after any of a store's (Lepcha's ᰀ then h,
                    # ᰂ): index(store, 1) names the context's place.
                    for i, before in enumerate(self.stores.get(context[0][1], [])):
                        text_out = self.output(output, i)
                        if isinstance(before, str) and text_out is not None and len(text_out) == 1:
                            self.after.append((before, k, text_out))

    def members(self, toks):
        items, i = [], 0
        while i < len(toks):
            kind, val = toks[i]
            if kind == "str":
                if i + 2 < len(toks) and toks[i + 1][0] == "range" and len(val) == 1:
                    items.extend(chr(c) for c in range(ord(val), ord(toks[i + 2][1]) + 1))
                    i += 3
                    continue
                items.extend(val)
            elif kind == "key":
                items.append(val)
            elif kind == "outs":
                items.extend(self.stores.get(val, []))
            i += 1
        return items

    def output(self, output, index):
        out = []
        for kind, val in output:
            if kind == "str":
                out.append(val)
            elif kind == "index":
                items = self.stores.get(val.partition(",")[0].strip(), [])
                if index is None or index >= len(items) or not isinstance(items[index], str):
                    return None
                out.append(items[index])
            elif kind in ("nul", "beep"):
                continue
            else:
                return None
        return "".join(out)

    def typed(self, layer, key):
        """What a touch key types with no context, on its layer: None if
        no rule types it."""
        kid = key.get("id", "")
        if kid.startswith("U_"):
            return "".join(chr(int(h, 16)) for h in kid[2:].split("_"))
        mods = LAYER_MODS.get(key.get("layer", layer))
        if mods is None:
            return None
        return self.free.get((" ".join(sorted(mods.split())), kid))


def own(c):
    """A character of the keyboard's script, as against what the symbol
    views have: no ASCII, nor Latin-1's, general punctuation's and the
    symbol blocks' signs (Keyman's “ ” ÷ ★ beside the letters); a sign of
    the script's own block is (New Tai Lue's ᧞)."""
    if ord(c) < 0x80 or "\u2000" <= c <= "\u2bff" or c in "\u200b\u200c\u200d\u2060\ufeff":
        return False
    cat = unicodedata.category(c)
    if cat[0] in "LMN":
        return True
    if cat[0] == "P":
        return not "\u00a0" <= c <= "\u00bf"
    return cat[0] == "S" and ord(c) >= 0x300 and not ("\u3000" <= c <= "\u303f" or "\ufe30" <= c <= "\uffef")


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


def typed(lang, layer, key, rules):
    """What a touch key (or a long press) types, checked against its label."""
    kid, label = key["id"], key.get("text", "")
    text = rules.typed(layer, key)
    if text is None:
        sys.exit(f"{lang}: {layer} {kid}: no rule")
    if text != label:
        if (lang, layer, kid) not in TYPES_ITS_LABEL:
            sys.exit(f"{lang}: {layer} {kid}: labelled {label!r}, types {text!r}")
        text = label
    return text


def letter_rows(lang, layer, rules):
    """A touch layer's three rows of letters, as text keys."""
    rows = []
    for row in layer["row"][:3]:
        keys = []
        for k in row["key"]:
            if k["id"] in SPECIAL or str(k.get("sp")) in NO_LETTER:
                continue
            text = typed(lang, layer["id"], k, rules)
            label = k["text"]
            if (lang, layer["id"]) in ONE_LETTER:
                text = label = text[0]
            key = {"width_units": 1, "label": label, "output": {"kind": "text", "text": text}}
            long_press = (syllabic_forms(text) if lang in SYLLABICS else []) + LONG_PRESS_ADDED.get((lang, text), [])
            long_press += [typed(lang, layer["id"], s, rules) for s in k.get("sk", [])]
            if long_press:
                key["long_press"] = [{"kind": "text", "text": t} for t in long_press]
            keys.append(key)
        rows.append(keys)
    end = HOME_ROW_END.get((lang, layer["id"]))
    if end:
        rows[1].append({"width_units": 1, "label": end, "output": {"kind": "text", "text": end}})
    return rows


def gap(width=1):
    return {"width_units": width, "label": "", "output": {"kind": "none"}}


def text_key(text, long_press=()):
    key = {"width_units": 1, "label": text, "output": {"kind": "text", "text": text}}
    if long_press:
        key["long_press"] = [{"kind": "text", "text": t} for t in long_press]
    return key


def own_text(layer, key, rules):
    """What a key types of the script, or None."""
    text = rules.typed(layer, key)
    if text and any(own(c) for c in text):
        return text.strip()
    return None


def placed_rows(layers, rules, shift_layer="shift"):
    """The base and shift views' letter rows, Keyman's places kept: each
    a list of cells, a key typing the script's (its text, its long
    presses) or a gap (None); and the letters of the bottom row."""
    views = {}
    for view, layer_id in (("base", "default"), ("shift", shift_layer)):
        # A keyboard of one layer of letters (Khmu's) has the same on Shift.
        layer_id = layer_id if layer_id in layers else "default"
        layer = layers[layer_id]
        rows = []
        for row in layer["row"][:-1]:
            cells = []
            for k in row["key"]:
                if k.get("id") in SPECIAL or str(k.get("sp")) in ("1", "2"):
                    continue
                text = None if str(k.get("sp")) in GAP else own_text(layer_id, k, rules)
                if text is None:
                    cells.append(None)
                    continue
                held = [t for t in (own_text(layer_id, s, rules) for s in k.get("sk", [])) if t and t != text]
                cells.append([text, held, k.get("id")])
            rows.append(cells)
        bottom, after_space = [], False
        for k in layer["row"][-1]["key"]:
            after_space |= k.get("id") == "K_SPACE"
            if k.get("id") in SPECIAL or str(k.get("sp")) in ("1", "2", "9", "10"):
                continue
            text = own_text(layer_id, k, rules)
            if text and not all(unicodedata.category(c)[0] == "P" for c in text):
                bottom.append([text, [], k.get("id"), after_space])
        views[view] = (rows, bottom)
    return views


def build_placed(lang, layout, layers, rules):
    """The base and shift views' letter rows for a keyboard laid out as
    Keyman's rows and places are; the letters on no tap."""
    views = placed_rows(layers, rules, SHIFT_LAYER.get(lang, "shift"))
    base_rows, base_bottom = views["base"]
    shift_rows, shift_bottom = views["shift"]
    # AltGr's letters first on the long press of the key in its place; its
    # digits (Tai Tham's Hora digits) on the symbol view's, as below.
    for alt_layer, rows in (("rightalt", base_rows), ("rightalt-shift", shift_rows)):
        if alt_layer not in layers:
            continue
        by_id = {cell[2]: cell for row in rows for cell in row if cell}
        for row in layers[alt_layer]["row"]:
            for k in row["key"]:
                text = own_text(alt_layer, k, rules)
                cell = by_id.get(k.get("id"))
                if text and all(unicodedata.category(c) == "Nd" for c in text):
                    continue
                if text and cell and text != cell[0] and text not in cell[1]:
                    cell[1].insert(0, text)
    # A row of gaps on one view takes the other's; on both it goes.
    n = max(len(base_rows), len(shift_rows))
    base_rows += [[]] * (n - len(base_rows))
    shift_rows += [[]] * (n - len(shift_rows))
    rows = []
    for b, s in zip(base_rows, shift_rows):
        b_empty, s_empty = not any(b), not any(s)
        if b_empty and s_empty:
            continue
        rows.append(([list(c) if c else None for c in s] if b_empty else b,
                     [list(c) if c else None for c in b] if s_empty else s))
    # The views' rows keep each other's places: a place that is a gap on
    # both at a row's end goes, and the shorter row ends in gaps.
    for b, s in rows:
        while (b or s) and (not b or b[-1] is None) and (not s or s[-1] is None):
            b[-1:] = []
            s[-1:] = []
        while b and s and b[0] is None and s[0] is None:
            del b[0], s[0]
        b += [None] * (len(s) - len(b))
        s += [None] * (len(b) - len(s))
    base_rows, shift_rows = [r[0] for r in rows], [r[1] for r in rows]
    # The bottom row's letters no other key types, beside the space bar.
    tapped = {c[0] for row in base_rows + shift_rows for c in row if c}
    base_bottom = [c for c in base_bottom if c[0] not in tapped]
    shift_bottom = [c for c in shift_bottom if c[0] not in tapped]
    for row in base_rows + shift_rows + [base_bottom, shift_bottom]:
        for cell in row:
            if cell:
                cell[1] += [t for t in LONG_PRESS_ADDED.get((lang, cell[0]), []) if t not in cell[1]]
    # A letter typed by a key after another's is on that one's long press.
    tapped = {c[0] for row in base_rows + shift_rows for c in row if c}
    for before, _key, after in rules.after:
        if after in tapped or not own(after):
            continue
        for row in base_rows + shift_rows:
            for cell in row:
                if cell and cell[0] == before and after not in cell[1]:
                    cell[1].append(after)
    return base_rows, shift_rows, base_bottom, shift_bottom


def beside_space(view, cells):
    """The bottom row's letters (Bamum's two marks, Osage's acute) on each
    side of the space bar as Keyman has them, taking its width."""
    row = view["rows"][-1]["keys"]
    i = next(n for n, k in enumerate(row) if k.get("action") == "space")
    before = [text_key(c[0], c[1]) for c in cells if not c[3]]
    after = [text_key(c[0], c[1]) for c in cells if c[3]]
    row[i] = {**row[i], "width_units": max(2, row[i]["width_units"] - len(cells))}
    row[i + 1:i + 1] = after
    row[i:i] = before


def lay_out(view, rows, width):
    """A view's letter rows, as wide as `width`: gaps at a short row's
    ends, Shift and Backspace taking the last row's rest."""
    template = view["rows"]
    shift, backspace = template[2]["keys"][0], template[2]["keys"][-1]
    out = []
    for i, cells in enumerate(rows):
        keys = [gap() if c is None else text_key(c[0], c[1]) for c in cells]
        last = i == len(rows) - 1
        rest = width - len(keys) - (0 if not last else 0)
        if last:
            side = max(1.5, (width - len(keys)) / 2)
            keys = [{**shift, "width_units": side}] + keys + [{**backspace, "width_units": side}]
        elif rest > 0:
            keys = [gap(rest / 2)] + keys + [gap(rest / 2)]
        out.append({**template[0], "keys": keys})
    view["rows"] = out + template[3:]


def offers_untapped(layout):
    """Whether a long press offers a letter no tap types."""
    taps, held = set(), set()
    for view in layout["views"]:
        for row in view["rows"]:
            for key in row["keys"]:
                taps.update(key.get("label", ""))
                if key.get("output", {}).get("kind") == "text":
                    taps.update(key["output"].get("text", ""))
                for o in key.get("long_press", []):
                    if o.get("kind") == "text":
                        held.update(o.get("text", ""))
    return any(c not in taps and (unicodedata.category(c)[0] in "LM" or unicodedata.category(c) == "Nl")
               for c in held)


def everything_typed(lang, layers, rules):
    """Every text a key or a long press of the keyboard types, any layer
    the language writes with."""
    out = []
    for layer in layers.values():
        if (lang, layer["id"]) in UNUSED_LAYERS:
            continue
        for row in layer["row"]:
            for k in row["key"]:
                for key in [k] + k.get("sk", []):
                    text = rules.typed(layer["id"], key)
                    label = key.get("text", "")
                    # A digit of a layer of its own no rule types is what
                    # its label shows (Saurashtra's ꣑ on K_1).
                    if text is None and len(label) == 1 and unicodedata.category(label) == "Nd":
                        text = label
                    if text:
                        out.append(text)
    return out


def build(lang, name, path, commit, files):
    got = fetch(path, commit, files)
    touch = json.loads(next(v for k, v in got.items() if k.endswith(".keyman-touch-layout")).decode("utf-8-sig"))
    rules = Rules(next(v for k, v in got.items() if k.endswith(".kmn")))
    layers = {layer["id"]: layer for layer in touch[PLATFORM[lang]]["layer"]}
    layout = json.loads((ROOT / "layouts" / "us.json").read_text(encoding="utf-8"))
    layout["id"] = lang
    layout["name"] = name
    if lang in XKB_LAYOUT:
        layout["xkb_layout"] = XKB_LAYOUT[lang]
    else:
        layout.pop("xkb_layout", None)
    layout.pop("xkb_variant", None)
    layout["direction"] = "rtl" if lang in RTL else "ltr"
    layout["swipe_enabled"] = False
    if (ROOT / "predict" / f"{lang}.fst").exists():
        layout["prediction_enabled"] = True
        layout["prediction_lang"] = lang
    else:
        layout["prediction_enabled"] = False
        layout.pop("prediction_lang", None)
    layout["font_script"] = FONT_SCRIPT[lang]
    views = {view["id"]: view for view in layout["views"]}
    if lang in PLACED:
        base_rows, shift_rows, base_bottom, shift_bottom = build_placed(lang, layout, layers, rules)
        width = max([len(r) for rows in (base_rows, shift_rows) for r in rows[:-1]]
                    + [len(rows[-1]) + 3 for rows in (base_rows, shift_rows)])
        lay_out(views["base"], base_rows, width)
        lay_out(views["shift"], shift_rows, width)
        beside_space(views["base"], base_bottom)
        beside_space(views["shift"], shift_bottom)
        tapped = set()
        for view in (views["base"], views["shift"]):
            for row in view["rows"]:
                for key in row["keys"]:
                    if key.get("output", {}).get("kind") == "text":
                        tapped.add(key["output"]["text"])
                    tapped.update(o.get("text") for o in key.get("long_press", []))
        everything = everything_typed(lang, layers, rules)
        # The script's digits on the long presses of the symbol view's.
        digits = {}
        for text in everything:
            for c in text:
                if unicodedata.category(c)[0] == "N" and own(c) and c not in tapped \
                        and unicodedata.numeric(c, -1) in range(10) and c not in digits.get(int(unicodedata.numeric(c)), []):
                    digits.setdefault(int(unicodedata.numeric(c)), []).append(c)
        for row in views["symbol1"]["rows"]:
            for key in row["keys"]:
                t = key.get("output", {}).get("text", "")
                if len(t) == 1 and t.isdigit() and int(t) in digits:
                    key["long_press"] = [{"kind": "text", "text": d} for d in digits[int(t)]]
                    tapped.update(digits[int(t)])
        # Its stops and commas no key keeps on the full stop's, and the
        # letters only its symbol layer has (Lao's ຯ and ໆ).
        marks = []
        for text in everything:
            if len(text) == 1 and unicodedata.category(text)[0] in "PLM" and unicodedata.category(text) not in \
                    ("Ps", "Pe", "Pi", "Pf", "Pd", "Pc") and own(text) and text not in tapped and text not in marks:
                marks.append(text)
        for key in views["base"]["rows"][-1]["keys"] if marks else []:
            if key.get("output") == {"kind": "text", "text": "."}:
                key["long_press"] = [{"kind": "text", "text": m} for m in marks]
    else:
        for view in layout["views"]:
            layer = {"base": "default", "shift": "shift"}.get(view["id"])
            if layer is None:
                continue
            rows = view["rows"]
            shift, backspace = rows[2]["keys"][0], rows[2]["keys"][-1]
            r0, r1, r2 = letter_rows(lang, layers[layer], rules)
            rows[0]["keys"] = r0
            rows[1]["keys"] = r1
            rows[2]["keys"] = [shift] + r2 + [backspace]
            stop = FULL_STOP_LONG_PRESS.get(lang)
            for key in rows[3]["keys"] if stop else []:
                if key.get("output") == {"kind": "text", "text": "."}:
                    key["long_press"] = [{"kind": "text", "text": stop}] + key.get("long_press", [])
    # A right-to-left layout is stored in reading order, as
    # scripts/build-layout.py's: the renderer mirrors each row, so the
    # keys are drawn where Keyman has them.
    if lang in RTL:
        for view in layout["views"]:
            for row in view["rows"]:
                row["keys"].reverse()
    # A letter only a long press types turns the gesture on for the layout.
    if offers_untapped(layout):
        flagged = {}
        for k, v in layout.items():
            if k == "views":
                flagged["long_press_letters"] = True
            flagged[k] = v
        layout = flagged
    # The template's keys first, then the layout's own fields as
    # build-layout.py writes them.
    return json.dumps(layout, ensure_ascii=False, indent=2) + "\n"


def main():
    check = "--check" in sys.argv[1:]
    only = {a for a in sys.argv[1:] if not a.startswith("--")}
    stale = []
    for lang, name, path, commit, files in KEYBOARDS:
        if only and lang not in only:
            continue
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
