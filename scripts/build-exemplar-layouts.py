#!/usr/bin/env python3
"""build-exemplar-layouts.py — keyboards for languages no text here
predicts, their letters from CLDR's exemplar characters.

CLDR lists the letters each language writes, its main exemplar
characters: in common/main/<lang>.xml for the languages it covers, and in
exemplars/main/<lang>.xml, unconfirmed, for the others. Each language here
is typed on the US keys, as build-derived-layouts.py's are, and each
letter those lack is first on the long press of the letter it is made
from, in CLDR's order, the menu's others after them: by its canonical
decomposition (á, ą́ on a; ẑ on z), by its Unicode name (ɓ, B WITH HOOK,
on b; ɨ, I WITH STROKE, on i; ʋ on v), or by the vowel or consonant it
writes (ɛ ə ǝ on e, ɔ on o, ŋ on n, ɩ on i, ʉ on u, ʌ on a). Its capital
is on the Shift view's. A run of letters typed one by one (Lakota's čh,
Navajo's tł) needs no key of its own. A glottal stop written as an
apostrophe-like letter (Aborlan's and Muna's ꞌ, Lendu's ʼ, Cebaara's ˀ)
has a key at the home row's end, or where that is full, a place on the
full stop's long press (Lakota's ʼ).

What the list could not say, a language's own text did where any could
be read (FineWeb-2's or the Wikimedia Incubator's, 900 to 41 000 words;
scripts/fineweb-text.py's sources): the letters it writes that the list
lacks, after the list's (Mundang's ẽ; Ngambay's ə ɓ ŋ ɗ, its tones and
its creaky vowels ḛ ḭ a̰ o̰; Fang's é è ô, as its Jehovah's Witnesses' and
Bible pages spell; Berom's tones), the apostrophe its words write (an
apostrophe key for Duri, Gor, Ngambay, Mundang, Serer, Soga, Kiga, Zarma,
Betawi and Lojban; Navajo's ʼ for the list's ’), and keys of their own
for the letters its words write most of those only a long press would
type: those a fifth of its words write at the home row's end, and where
long presses would still be in a quarter of its words, the next most
written there and then in place of q and x, which it hardly writes (q and
x on those keys' long presses), as build-derived-layouts.py's are:
Navajo's ʼ í at the home row's end and á é on q and x (long presses in
78.5% of its words, then 37.1%), Lakota's ŋ á, č é (71.4%, 37.6%),
Temne's ɔ ŋ, ə ɛ (61.4%, 18.3%), Gor's ɨ and apostrophe, ə ɔ (60.4%,
2.0%), Ngambay's ə and apostrophe, é ɔ (59.2%, 29.3%) and Mundang's ǝ and
apostrophe (29.8%, 20.0%). None has a model: suggestions and glides are
off.

Each language's CLDR file is fetched from unicode-org/cldr at the
release-49 commit and checked against its sha256 into $CLDR_DIR (default
$TMPDIR/ki-osk-sources/cldr); a layout's id is its language's tag.

    scripts/build-exemplar-layouts.py [--check]

--check fails if a written file would differ, as a test of the tree
(it needs the files, or the network).
"""
import hashlib
import importlib.util
import json
import os
import pathlib
import re
import sys
import unicodedata
import urllib.request
import xml.etree.ElementTree as ET

ROOT = pathlib.Path(__file__).resolve().parent.parent
REPO = "https://raw.githubusercontent.com/unicode-org/cldr"
COMMIT = "96e54b273d2da4ec6fbad350ba19ff23fa24a588"  # release-49

_spec = importlib.util.spec_from_file_location("derived", ROOT / "scripts" / "build-derived-layouts.py")
derived = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(derived)

COMMON, EXEMPLARS = "common/main", "exemplars/main"

# The layout made, its CLDR file (its directory and sha256), its name
# where CLDR's file gives none, and what the language's text decided: the
# letters it writes that CLDR's list lacks ("also"), its apostrophe in place
# of the list's ("apostrophe"), the keys at the home row's end ("home") and
# those in place of q and x ("swap").
LANGS = [
    ("aa", COMMON, "29f534a799a1eab1a85a3011cdbe6a3713d037667be8cdebc6c77fc1f40de281", {}),
    ("bew", COMMON, "3b5410451427220daf07103db75af0097a451ba0cfea9d5c7816f1fd15fd4d20", {"home": ["'"]}),
    ("bom", EXEMPLARS, "cf19accebdff85b00f8e91b30a1591f42c5838c7bf1892c25b40dd42193c3291", {
        "name": "Berom", "also": "é è ê ē ě á à â ā í î ī ó ò ô ō û ɛ́ ɛ̀ ɛ̂ ɛ̄ ɔ́ ɔ̀ ɔ̂".split()}),
    ("cbk", EXEMPLARS, "8ec668d2e1fc32cf3c680a9bdab7e9bc928a12ee9857e8a9c7df250566882a26", {
        "name": "Chavacano", "also": "á é í ó ú".split()}),
    ("cgg", COMMON, "0fa08540a9df2cfba8c6aa0810f71a70e712e2d6cb6fa625a8088fce5e036f32", {"home": ["'"]}),
    ("cho", COMMON, "f320ba041fcb6513fc490f7dc843453b77a99c4290a0a634e7ce601f06f3478c", {"also": "u̱ á í ó".split()}),
    ("dje", COMMON, "8ea46dafca04001146dbbbd9b1e55b3b5109ab8a3b5476b0c75edc4622351e95", {"home": ["'"]}),
    ("dzg", EXEMPLARS, "2253992d6d58d2eecf1bdcb27745f2f3be7d4bbd77f2d937b3ced28a3d1967aa", {"name": "Dazaga"}),
    ("fan", EXEMPLARS, "d23e151f45a32909aac36f39133281341f1bb2856fbc3f39d22b14e183e62655", {
        "name": "Fang", "also": "é è ê ô ò â à î ï".split()}),
    ("gqr", EXEMPLARS, "49305b084b3887401ab77a44ecefefe147874e27df5b71e464fb468fa989f1bb", {
        "name": "Gor", "home": ["ɨ", "'"], "swap": {"q": "ə", "x": "ɔ"}}),
    ("igb", EXEMPLARS, "fa5606db2064c8614b142f51574496480034bbfc6c1fdafd504dd7c717810697", {"name": "Ebira"}),
    ("jbo", COMMON, "03802ba4da788d16eb2224a0ab2160ef3b8e6854fb489781f45b2e1e0f5f57c1", {"name": "Lojban", "home": ["'"]}),
    ("las", EXEMPLARS, "e4e4725167d89d9eaa66183f35607043f6779bac20a52a149865cfc973a20843", {"name": "Lama"}),
    ("led", EXEMPLARS, "525e16aa2a48a3c198ef52fabcb5e686daefbffe5a15208fa2d54486b4cebb45", {"name": "Lendu"}),
    ("lkt", COMMON, "b9856438b70066d5a5476b8d8e601682c72e6a8b7a0430128cc3eab727fb0797", {
        "home": ["ŋ", "á"], "swap": {"q": "č", "x": "é"}}),
    ("mkw", EXEMPLARS, "970f38106d733771b8af0298cff28b50a1c78b94cf53ec638938dd1f451adfb0", {"name": "Kituba"}),
    ("mnb", EXEMPLARS, "d108db60c852d6c13adf49c11fc55c569ba68ef5747f11a1080b57ca906773e1", {"name": "Muna"}),
    ("mua", COMMON, "9279d796249e709becdefb6de27f8ee9314c5aadfe562bd062cc8aa41cfb1e2f", {
        "name": "Mundaŋ", "also": ["ẽ"], "home": ["ǝ", "'"]}),
    ("mvp", EXEMPLARS, "ebcdfa6022e798e58d36cd4cdcee195ef80c186791367a1446c7fb43d7bd12ac", {"name": "Duri", "home": ["'"]}),
    ("nup", EXEMPLARS, "ae1678809e9bb5b106b1b39502daffad14a9572141ea09205c1ee365fda8894a", {"name": "Nupe"}),
    ("nv", COMMON, "70582769bccb4bb3d90cc66e4a3565f173771c85578b36b2975b304d52f58988", {
        "apostrophe": "ʼ", "also": ["ń"], "home": ["ʼ", "í"], "swap": {"q": "á", "x": "é"}}),
    ("pmy", EXEMPLARS, "025a901e5884b1507a08d66993449b54b4b687c7f06e73d15357f1fb7cf42380", {"name": "Melayu Papua"}),
    ("pov", EXEMPLARS, "259ddc05a744a24635c0fa4daa40ff90be68beca16a48ec49d9b994b0859efa1", {"name": "Kriol", "also": ["ç", "õ"]}),
    ("sba", EXEMPLARS, "a57b3d81b2a5313f90f94b867cdc470b2fdc4a265d6e9a55997635999883d801", {
        "name": "Ngambay", "also": "ə ə́ ə̰ ɓ ɗ ŋ é á í ó è à ḛ ḭ a̰ o̰".split(),
        "home": ["ə", "'"], "swap": {"q": "é", "x": "ɔ"}}),
    ("sef", EXEMPLARS, "108f604f6eaa14fec984476513095b6c0a764148c7f2ac5370464836fd9c163e", {"name": "Cebaara"}),
    ("snk", EXEMPLARS, "1f10bd67944c347c485b3231be6582f4af20655cbc2c46b5d91596d76644b9e2", {"name": "Sooninkanxanne"}),
    ("srr", EXEMPLARS, "61e10aa67302107638440635d4ec44f1ad4e0a4243373307e6022116ca6ce7c0", {
        "name": "Seereer", "also": ["é"], "home": ["'"]}),
    ("tbw", EXEMPLARS, "ac656976c8b1f064a779cfe21e490c975a098b85f46c356ea9d865e2edc8c39d", {"name": "Tagbanwa"}),
    ("tem", EXEMPLARS, "f36e539e83ba41b2fd75075d0f898bc9988ac142fd62a1cc3c932dcbcf7bd9db", {
        "name": "Temne", "also": ["ȧ", "å"], "home": ["ɔ", "ŋ"], "swap": {"q": "ə", "x": "ɛ"}}),
    ("xog", COMMON, "6ba4b7c0c0c17310f4d42958d91e3ecb98d5e477314983e3495b47a846a078e9", {"home": ["'"]}),
]

# The letters no letter with a mark is made from, on the key of the
# vowel or consonant they write.
NEAREST = {"ɛ": "e", "ə": "e", "ǝ": "e", "ɔ": "o", "ŋ": "n", "ɩ": "i", "ʊ": "u", "ʉ": "u",
           "ʌ": "a", "ɣ": "g", "ʒ": "z", "ʃ": "s"}
# A glottal stop or apostrophe written as a letter.
APOSTROPHES = {"'", "’", "ʼ", "ꞌ", "ˀ", "ʔ"}
# Where an apostrophe-like letter goes when the home row has no room.
FULL_STOP = "."


def fetch(lang, where, sha):
    """The language's CLDR file, fetched where it is not kept, checked."""
    cache = pathlib.Path(os.environ.get("CLDR_DIR")
                         or pathlib.Path(os.environ.get("TMPDIR", "/tmp")) / "ki-osk-sources" / "cldr")
    local = cache / COMMIT / where / f"{lang}.xml"
    if not local.exists() or hashlib.sha256(local.read_bytes()).hexdigest() != sha:
        local.parent.mkdir(parents=True, exist_ok=True)
        req = urllib.request.Request(f"{REPO}/{COMMIT}/{where}/{lang}.xml", headers={"User-Agent": "ki-osk-lang-content"})
        with urllib.request.urlopen(req) as r:
            data = r.read()
        got = hashlib.sha256(data).hexdigest()
        if got != sha:
            sys.exit(f"{where}/{lang}.xml: sha256 {got}, not {sha}")
        local.write_bytes(data)
    return local.read_bytes()


def unicode_set(text):
    """The members of an exemplar set, CLDR's UnicodeSet syntax: single
    characters, ranges (a-z), escapes (\\u0301) and {strings}."""
    body = text.strip()
    if not (body.startswith("[") and body.endswith("]")):
        sys.exit(f"not a set: {text!r}")
    body, out, i = body[1:-1], [], 0
    while i < len(body):
        c = body[i]
        if c == "{":
            j = body.index("}", i)
            out.append(body[i + 1:j])
            i = j + 1
        elif c == "\\":
            if body[i + 1] == "u":
                out.append(chr(int(body[i + 2:i + 6], 16)))
                i += 6
            else:
                out.append(body[i + 1])
                i += 2
        elif c.isspace():
            i += 1
        elif i + 2 < len(body) and body[i + 1] == "-" and not body[i + 2].isspace():
            out.extend(chr(k) for k in range(ord(c), ord(body[i + 2]) + 1))
            i += 3
        else:
            out.append(c)
            i += 1
    # A letter and its marks are one member ({ą́}, or a̱ written out).
    merged = []
    for m in (unicodedata.normalize("NFC", m) for m in out):
        if merged and unicodedata.category(m[0]).startswith("M"):
            merged[-1] = unicodedata.normalize("NFC", merged[-1] + m)
        else:
            merged.append(m)
    return merged


def exemplars(lang, xml):
    """The language's main exemplar characters and its own name for itself."""
    root = ET.fromstring(xml)
    main = [e.text for e in root.iter("exemplarCharacters") if not e.get("type")]
    if len(main) != 1:
        sys.exit(f"{lang}: {len(main)} main exemplar sets")
    name = None
    for e in root.iter("language"):
        if e.get("type") == lang and not e.get("alt"):
            name = e.text
    return unicode_set(main[0]), name


def host(letter, keys):
    """The key whose long press types `letter`: the letter it is made from."""
    base = unicodedata.normalize("NFD", letter)[0]
    if base in keys:
        return base
    m = re.match(r"(LATIN (?:SMALL|CAPITAL) LETTER \w+) WITH ", unicodedata.name(base, ""))
    if m:
        try:
            made_from = unicodedata.lookup(m.group(1))
        except KeyError:
            made_from = None
        if made_from in keys:
            return made_from
    if base in NEAREST:
        return NEAREST[base]
    sys.exit(f"no key for {letter!r} ({unicodedata.name(base, '?')})")


def capital(text):
    return derived.shifted(text)


def plan(lang, where, sha, options):
    """The layout's name, what its keys gain on a long press, its keys at
    the home row's end and those in place of q and x."""
    letters, cldr_name = exemplars(lang, fetch(lang, where, sha))
    name = options.get("name") or cldr_name
    if not name:
        sys.exit(f"{lang}: CLDR gives no name; name it")
    base = json.loads((ROOT / "layouts" / "us.json").read_text(encoding="utf-8"))
    keys = {k["label"] for r in base["views"][0]["rows"][:3] for k in r["keys"]
            if len(k.get("label", "")) == 1 and k["label"].isalpha()}
    apostrophe = options.get("apostrophe")
    wanted = []
    for m in letters + list(options.get("also", [])):
        # A run of letters typed one by one (Lakota's čh, Navajo's kʼ):
        # only its apostrophe, which is typed alone, may need a key.
        run = len(m) > 1 and not unicodedata.category(m[1]).startswith("M")
        for c in ([c for c in m if c in APOSTROPHES] if run else [m]):
            if apostrophe and c in APOSTROPHES:
                c = apostrophe
            if c not in keys and c not in wanted:
                wanted.append(c)
    home = list(options.get("home", []))
    swap = dict(options.get("swap", {}))
    own = set(home) | set(swap.values())
    for m in wanted:
        if m in APOSTROPHES and m not in own and len(home) < 2:
            home.append(m)
            own.add(m)
    extra = {}

    def put(on, letter):
        extra.setdefault(on, []).append(letter)
        if capital(on) != on:
            extra.setdefault(capital(on), []).append(capital(letter))

    for m in wanted:
        if m in APOSTROPHES:
            if m not in own:
                put(FULL_STOP, m)
        elif m in own:
            # A letter with a key of its own stays on the long press of the
            # one it is made from too (Navajo's á on a).
            put(host(m, keys), m)
        else:
            # Made from a letter with a key of its own, on that key (Ngambay's
            # ə́ on ə).
            put(host(m, keys | own), m)
    # The letters q and x typed, on the long press of the keys in their place.
    full_swap = {}
    for k, letter in swap.items():
        full_swap[k] = letter
        extra.setdefault(letter, []).insert(0, k)
        if capital(k) != k:
            full_swap[capital(k)] = capital(letter)
            extra.setdefault(capital(letter), []).insert(0, capital(k))
    return name, extra, home, full_swap


def build(lang, where, sha, options):
    name, extra, home, swap = plan(lang, where, sha, options)
    return derived.build(lang, "us", name, extra, {"home": home, "swap": swap, "model": False})


def main():
    check = "--check" in sys.argv[1:]
    stale = []
    for lang, where, sha, options in LANGS:
        text = build(lang, where, sha, options)
        out = ROOT / "layouts" / f"{lang}.json"
        if check:
            if not out.exists() or out.read_text(encoding="utf-8") != text:
                stale.append(out.name)
        else:
            out.write_text(text, encoding="utf-8")
            print(f"wrote layouts/{out.name}")
    if stale:
        sys.exit("out of date (run scripts/build-exemplar-layouts.py): " + " ".join(stale))


if __name__ == "__main__":
    main()
