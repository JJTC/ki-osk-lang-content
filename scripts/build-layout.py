#!/usr/bin/env python3
"""Generate a full keyboard layout for a language from its xkb layout.

Clones the universal `us.json` template — the base/shift letter grid, the
shared symbol views, and the control row — then rebuilds the base+shift
*letter* rows with the target language's letters, read from libxkbcommon
(native keycaps, nothing hand-transcribed). Letters the US 26-key grid
can't hold are added at their real keycodes — the punctuation-position
keys (41 top-left, 26/27 top row, 39/40/43 home, 51/52/53 bottom, 86 the
ISO `<>` key) — plus any surplus on the number row (lifted to a top row,
see `extra_top`). Together these reach every base-level letter of the
supported national keymaps, and a per-view dedup keeps the grid clean.

The number row and symbols are text-output (universal digits/punctuation),
so only the keycode-based letter rows are localised; nothing else breaks
regardless of the national keymap.

Layout-only: no prediction/swipe assets are produced (those need per-
language corpora). Scripts DejaVu already renders — Latin, Cyrillic,
Greek — need no extra font; every other script sets `font_script` so the
keyboard loads the matching Noto face on demand. Run on a host with
libxkbcommon + xkeyboard-config.

    python3 scripts/build-layout.py [layouts_dir] [id,id,...]

With an id list, only those layouts are (re)generated — used to add a new
tier without rewriting the already-committed ones.

Complex-script note: for abugidas (Devanagari, Tamil, …) InScript keeps
the alphabet on the letter block and digits on the number row, so cloning
the three letter rows captures the script; ki-osk's digits live in the
symbol view, so the ASCII number row is untouched. Composed multi-glyph
clusters (conjuncts, matra reordering) shape correctly at render time via
ki-osk-wayland's fontations engine; these layouts still ship
layout-only (no bundled prediction/swipe assets) pending per-language
corpora.
"""
import ctypes
import json
import os
import sys
import unicodedata

# Scripts DejaVu already covers — no fallback font, and (Latin only) the
# accent long-press menus / all-keycodes-are-letters treatment.
DEJAVU_SCRIPTS = {"latin", "cyrillic", "greek"}

# (layout id, display name, xkb layout, xkb variant, script). `script`
# decides: the fallback font (`font_script`, unless DejaVu covers it),
# writing direction (rtl for arabic), whether accent long-press menus are
# kept (latin only), and the keycap filter (Latin keeps all 26 base codes;
# other scripts drop empty/stray-ASCII codes but keep combining matras).
LANGS = [
    # --- European, Latin script ---
    ("pt", "Português", "pt", "", "latin"),
    ("ro", "Română", "ro", "std", "latin"),
    ("hu", "Magyar", "hu", "", "latin"),
    ("sk", "Slovenčina", "sk", "", "latin"),
    ("sl", "Slovenščina", "si", "", "latin"),
    ("hr", "Hrvatski", "hr", "", "latin"),
    ("bs", "Bosanski", "ba", "", "latin"),
    ("sr-latn", "Srpski (latinica)", "rs", "latin", "latin"),
    ("et", "Eesti", "ee", "", "latin"),
    # lv (Latvian) + lt (Lithuanian) deferred: their diacritics live on
    # dead-keys / the number row, outside the six keycodes this tool adds,
    # so they'd ship a-z only. They need bespoke number-row handling.
    ("ga", "Gaeilge", "ie", "", "latin"),
    ("mt", "Malti", "mt", "", "latin"),
    ("is", "Íslenska", "is", "", "latin"),
    ("fo", "Føroyskt", "fo", "", "latin"),
    ("sq", "Shqip", "al", "", "latin"),
    ("ca", "Català", "es", "cat", "latin"),
    ("eo", "Esperanto", "epo", "", "latin"),
    # --- European, Cyrillic script ---
    ("bg", "Български", "bg", "", "cyrillic"),
    ("sr", "Српски", "rs", "", "cyrillic"),
    ("mk", "Македонски", "mk", "", "cyrillic"),
    ("be", "Беларуская", "by", "", "cyrillic"),
    # --- Tier 2: Asian, in already-renderable scripts ---
    # Latin
    ("id", "Bahasa Indonesia", "id", "", "latin"),
    ("fil", "Filipino", "ph", "", "latin"),
    ("az", "Azərbaycan", "az", "", "latin"),
    ("ku", "Kurdî", "iq", "ku", "latin"),
    # Arabic script (RTL). Central-Asian Cyrillic (kk/ky/mn/tg) is
    # deferred: like Lithuanian, its special letters sit on the number
    # row, outside the six extra keycodes this tool adds.
    ("ur", "اردو", "pk", "", "arabic"),
    ("ps", "پښتو", "af", "ps", "arabic"),
    ("sd", "سنڌي", "pk", "snd", "arabic"),
    ("ug", "ئۇيغۇرچە", "cn", "ug", "arabic"),
    # --- Tier 3: complex Asian / Caucasus scripts (bundled Noto faces) ---
    # Alphabetic — fit the grid cleanly, no combining marks:
    ("ka", "ქართული", "ge", "", "georgian"),
    ("hy", "Հայերեն", "am", "", "armenian"),
    # Abugidas (InScript-derived letter block; one layout per script —
    # Marathi/Nepali reuse the Devanagari layout, etc.):
    ("hi", "हिन्दी", "in", "hin-wx", "devanagari"),
    ("bn", "বাংলা", "in", "ben", "bengali"),
    ("ta", "தமிழ்", "in", "tam", "tamil"),
    ("te", "తెలుగు", "in", "tel", "telugu"),
    ("kn", "ಕನ್ನಡ", "in", "kan", "kannada"),
    ("ml", "മലയാളം", "in", "mal", "malayalam"),
    ("gu", "ગુજરાતી", "in", "guj", "gujarati"),
    ("pa", "ਪੰਜਾਬੀ", "in", "guru", "gurmukhi"),
    ("or", "ଓଡ଼ିଆ", "in", "ori", "oriya"),
    ("si", "සිංහල", "lk", "sin_phonetic", "sinhala"),
    ("km", "ខ្មែរ", "kh", "", "khmer"),
    ("lo", "ລາວ", "la", "", "lao"),
    ("my", "မြန်မာ", "mm", "", "myanmar"),
    # --- Central-Asian / Baltic: extra letters on the NUMBER ROW ---
    # These alphabets overflow the 32-key letter grid, so the national
    # xkb layout parks the surplus letters on the number-row keycodes.
    # `build()` lifts those into a centred top row (digits stay in the
    # symbol view). All DejaVu-drawable — no font_script.
    #   kk 9 extras (ә і ң ғ ү ұ қ ө һ), lt 9 (ą č ę ė į š ų ū ž),
    #   ky/tg 2 each (ө ң / ғ ӯ). mn needs none — its ө/ү are already in
    #   the grid, so it generates as a plain Cyrillic layout.
    ("kk", "Қазақша", "kz", "", "cyrillic"),
    ("ky", "Кыргызча", "kg", "", "cyrillic"),
    ("mn", "Монгол", "mn", "", "cyrillic"),
    ("tg", "Тоҷикӣ", "tj", "", "cyrillic"),
    ("lt", "Lietuvių", "lt", "", "latin"),
    # Latvian composes its diacritics through a DEAD KEY in xkb (' + a →
    # ā), which the keycode reader can't see, so its letters were missing.
    # On a touch keyboard the mobile answer is long-press, not a dead-key
    # state machine — see DIACRITIC_LONGPRESS below.
    ("lv", "Latviešu", "lv", "", "latin"),
    # --- Middle East / North Africa / South America ---
    # Syriac (RTL, Suriyaya): needs Noto Sans Syriac. Tamazight in Tifinagh
    # (LTR): needs Noto Sans Tifinagh. Kabyle is written in Latin — an `us`
    # base plus its ten extra letters on long-press (DIACRITIC_LONGPRESS).
    ("syr", "ܣܘܪܝܝܐ", "sy", "syc", "syriac"),
    ("ber", "ⵜⴰⵎⴰⵣⵉⵖⵜ", "ma", "tifinagh", "tifinagh"),
    ("kab", "Taqbaylit", "us", "", "latin"),
    # South American: Latin on the Spanish (`es`) base for ñ; Guaraní adds
    # its nasal vowels + g̃, Aymara its diaeresis vowels, on long-press.
    ("qu", "Runa Simi", "es", "", "latin"),
    ("gn", "Avañe'ẽ", "es", "", "latin"),
    ("ay", "Aymar aru", "es", "", "latin"),
    # --- Phase A: African + more Asian, Latin script ---
    # Built on the `us` (or `es`, for ñ) base like Kabyle/Guaraní — the
    # touch-keyboard convention is QWERTY + long-press, not a national
    # physical layout. Clean a–z ones inherit the US accent long-press
    # menus (a harmless mobile bonus); the rest add their own letters via
    # DIACRITIC_LONGPRESS below. All Latin → DejaVu-drawable, swipe on.
    ("sw", "Kiswahili", "us", "", "latin"),
    ("ms", "Bahasa Melayu", "us", "", "latin"),
    ("so", "Soomaali", "us", "", "latin"),
    ("zu", "isiZulu", "us", "", "latin"),
    ("xh", "isiXhosa", "us", "", "latin"),
    ("af", "Afrikaans", "us", "", "latin"),
    ("jv", "Basa Jawa", "us", "", "latin"),
    ("su", "Basa Sunda", "us", "", "latin"),
    ("om", "Afaan Oromoo", "us", "", "latin"),
    ("ceb", "Cebuano", "es", "", "latin"),   # ñ from the es base
    ("ha", "Hausa", "us", "", "latin"),      # hooked ɓ ɗ ƙ ƴ
    ("yo", "Yorùbá", "us", "", "latin"),     # dot-below + tone
    ("ig", "Igbo", "us", "", "latin"),       # dot-below
    ("uz", "Oʻzbekcha", "us", "", "latin"),  # oʻ gʻ
    ("tk", "Türkmençe", "us", "", "latin"),  # ä ç ž ň ö ş ü ý
    # --- Phase B: split-out Devanagari / Bengali (fonts already loaded) ---
    # Marathi/Nepali get their own name + prediction slot on the Devanagari
    # face; Assamese rides the Bangla layout with its two distinct letters
    # (ৰ ৱ) on long-press.
    ("mr", "मराठी", "in", "marathi", "devanagari"),
    ("ne", "नेपाली", "np", "", "devanagari"),
    ("as", "অসমীয়া", "in", "ben", "bengali"),
    # --- Tier C: additional minority scripts (each its own Noto face;
    # shaping handles Tibetan stacking / Thaana joining) ---
    ("bo", "བོད་ཡིག", "cn", "tib", "tibetan"),     # Tibetan
    ("dv", "ދިވެހި", "mv", "", "thaana"),           # Dhivehi (Thaana, RTL)
    ("sat", "ᱥᱟᱱᱛᱟᱲᱤ", "in", "sat", "olchiki"),      # Santali (Ol Chiki)
    # --- Tier E: European minority / regional ---
    # Latin on the us/es base; grave/acute/circumflex/umlaut for aeiou are
    # inherited from the us long-press menus, so only letters BEYOND them get
    # a DIACRITIC_LONGPRESS entry (which then replaces the inherited menu).
    # The Cyrillic minorities of Russia use their national xkb — the generator
    # lifts alphabet-overflow letters to a centred top row, as for Kazakh.
    ("cy", "Cymraeg", "us", "", "latin"),        # Welsh: ŵ ŷ + âêîôû
    ("gd", "Gàidhlig", "us", "", "latin"),       # Scottish Gaelic: àèìòù (inherited)
    ("eu", "Euskara", "es", "", "latin"),        # Basque: ñ from es
    ("gl", "Galego", "es", "", "latin"),         # Galician: ñ + áéíóú
    ("oc", "Occitan", "us", "", "latin"),        # Occitan: ç + àáèéíïòóúü
    ("br", "Brezhoneg", "us", "", "latin"),      # Breton: ñ ç + âêîôûù
    ("co", "Corsu", "us", "", "latin"),          # Corsican: àèìòù (inherited)
    ("lb", "Lëtzebuergesch", "us", "", "latin"), # Luxembourgish: ä ë é ï ö ü
    ("rm", "Rumantsch", "us", "", "latin"),      # Romansh: àèéìòù (inherited)
    ("fy", "Frysk", "us", "", "latin"),          # Frisian: âêëîïôûú
    ("tt", "Татарча", "ru", "tt", "cyrillic"),        # Tatar: ә ө ү җ ң һ
    ("ba", "Башҡортса", "ru", "bak", "cyrillic"),     # Bashkir: ө ҡ ғ ҫ ҙ һ ә ң
    ("os", "Ирон", "ru", "os_winkeys", "cyrillic"),   # Ossetian: ӕ
    ("udm", "Удмурт", "ru", "udm", "cyrillic"),       # Udmurt: ӝ ӧ ӵ ӥ ӟ
    # --- Tier F: African expansion (Latin, us base) ---
    # QWERTY + long-press, the touch-keyboard convention. Clean a–z ones
    # (Kinyarwanda, Kirundi, Shona, Sesotho, Tsonga, Swati, Ndebele) inherit
    # the harmless US accent menus; the rest add their alphabet's extra
    # letters (open vowels ɛ ɔ, hooked ɓ ɗ ƴ, velar ŋ, etc.) via
    # DIACRITIC_LONGPRESS.
    ("wo", "Wolof", "us", "", "latin"),          # à é ë ñ ŋ ó
    ("ln", "Lingála", "us", "", "latin"),        # ɛ ɔ
    ("rw", "Ikinyarwanda", "us", "", "latin"),   # clean a–z
    ("rn", "Ikirundi", "us", "", "latin"),       # clean a–z
    ("sn", "chiShona", "us", "", "latin"),       # clean a–z
    ("ny", "Chichewa", "us", "", "latin"),       # ŵ
    ("mg", "Malagasy", "us", "", "latin"),       # ñ ô
    ("st", "Sesotho", "us", "", "latin"),        # clean a–z
    ("tn", "Setswana", "us", "", "latin"),       # š
    ("nso", "Sepedi", "us", "", "latin"),        # š ê ô
    ("lg", "Luganda", "us", "", "latin"),        # ŋ ɲ
    ("ak", "Akan", "us", "", "latin"),           # ɛ ɔ
    ("ee", "Eʋegbe", "us", "", "latin"),         # ɖ ɛ ƒ ɣ ɔ ʋ ŋ
    ("bm", "Bamanankan", "us", "", "latin"),     # ɛ ɔ ɲ ŋ
    ("ff", "Fulfulde", "us", "", "latin"),       # ɓ ɗ ŋ ƴ
    ("ts", "Xitsonga", "us", "", "latin"),       # clean a–z
    ("ve", "Tshivenḓa", "us", "", "latin"),      # ḓ ḽ ṋ ṱ
    ("ss", "siSwati", "us", "", "latin"),        # clean a–z
    ("nr", "isiNdebele", "us", "", "latin"),     # clean a–z
    # --- Tier G: Austronesian / Pacific (→ langs-pacific) + Americas
    # (→ langs-american). Latin on the us base; the Polynesian macron long
    # vowels ride long-press (the ʻokina is typed from the symbols view's
    # apostrophe). Display names use macron-only endonyms, no okina, so the
    # picker never shows tofu.
    ("mi", "Māori", "us", "", "latin"),
    ("haw", "Ōlelo Hawaii", "us", "", "latin"),
    ("sm", "Gagana Sāmoa", "us", "", "latin"),
    ("to", "Lea faka-Tonga", "us", "", "latin"),
    ("fj", "Vosa Vakaviti", "us", "", "latin"),
    ("ty", "Reo Tahiti", "us", "", "latin"),
    ("ht", "Kreyòl Ayisyen", "us", "", "latin"),   # è ò à inherited
    ("nah", "Nāhuatl", "us", "", "latin"),         # macrons
    ("kl", "Kalaallisut", "us", "", "latin"),      # clean a–z
    # --- Tier H: new scripts (each its own Noto face, loaded lazily). All
    # LTR. Cherokee/Inuktitut/Meitei are syllabaries → keycaps carry whole
    # syllables; Dzongkha is the Tibetan script (reuses the Tibetan face).
    ("chr", "ᏣᎳᎩ", "us", "chr", "cherokee"),           # Cherokee syllabary
    ("iu", "ᐃᓄᒃᑎᑐᑦ", "ca", "ike", "canadian_aboriginal"),  # Inuktitut (UCAS)
    ("mni", "ꯃꯤꯇꯩ ꯂꯣꯟ", "in", "mni", "meetei_mayek"),      # Meitei (Meetei Mayek)
    ("dz", "རྫོང་ཁ", "bt", "", "tibetan"),                # Dzongkha (Tibetan script)
    # Deferred new-script candidates — wanted, but their xkb symbols aren't
    # in this host's xkeyboard-config (libxkbcommon can't compile them), so
    # they'd generate an ASCII-only grid. Add when the layouts land upstream
    # (or hand-author the JSON): N'Ko `us(nqo)` (nqo, RTL, Noto Sans NKo),
    # Yi `us(yi)` (ii, Noto Sans Yi), Adlam, Vai, Osage.
]

# Per-language diacritics surfaced via long-press on the base letter, for
# scripts whose national letters live on an xkb dead key (unreadable off
# key levels). For these ids the base a–z grid is kept and ONLY these keys
# get a long-press menu — the inherited US accent menus are dropped, so the
# layout shows exactly the language's own letters and nothing spurious.
# base letter → list of national letters offered on long-press. Lists
# (not single chars) so a base key can carry several (Kabyle g → ǧ, ɣ) and
# so multi-codepoint marks survive (Guaraní g̃ = g + combining tilde).
DIACRITIC_LONGPRESS = {
    # Latvian: macron vowels (ā ē ī ū), háčeks (č š ž), cedillas (ģ ķ ļ ņ).
    "lv": {
        "a": ["ā"], "e": ["ē"], "i": ["ī"], "u": ["ū"], "c": ["č"],
        "s": ["š"], "z": ["ž"], "g": ["ģ"], "k": ["ķ"], "l": ["ļ"],
        "n": ["ņ"],
    },
    # Kabyle (Latin orthography): the letters beyond a–z — háčeks (č ǧ),
    # the two IPA letters (ɣ ɛ), and the emphatic dot-belows (ḍ ḥ ṛ ṣ ṭ ẓ).
    "kab": {
        "c": ["č"], "g": ["ǧ", "ɣ"], "e": ["ɛ"], "d": ["ḍ"], "h": ["ḥ"],
        "r": ["ṛ"], "s": ["ṣ"], "t": ["ṭ"], "z": ["ẓ"],
    },
    # Guaraní: the six nasal vowels + nasal g̃ (ñ is already on the es base).
    "gn": {
        "a": ["ã"], "e": ["ẽ"], "i": ["ĩ"], "o": ["õ"], "u": ["ũ"],
        "y": ["ỹ"], "g": ["g̃"],
    },
    # Aymara: the three diaeresis (long) vowels.
    "ay": {"a": ["ä"], "i": ["ï"], "u": ["ü"]},
    # Hausa: the hooked consonants (ɓ ɗ ƙ) and hooked y (ƴ).
    "ha": {"b": ["ɓ"], "d": ["ɗ"], "k": ["ƙ"], "y": ["ƴ"]},
    # Yoruba: dot-below ẹ/ọ/ṣ plus acute/grave tone marks on the vowels.
    "yo": {
        "e": ["ẹ", "é", "è"], "o": ["ọ", "ó", "ò"], "s": ["ṣ"],
        "a": ["á", "à"], "i": ["í", "ì"], "u": ["ú", "ù"], "n": ["ń", "ǹ"],
    },
    # Igbo: dot-below vowels and dotted ṅ.
    "ig": {"i": ["ị"], "o": ["ọ"], "u": ["ụ"], "n": ["ṅ"]},
    # Uzbek Latin: oʻ and gʻ (modifier-letter turned comma, U+02BB).
    "uz": {"o": ["oʻ"], "g": ["gʻ"]},
    # Turkmen Latin: its eight letters beyond a–z.
    "tk": {
        "a": ["ä"], "c": ["ç"], "z": ["ž"], "n": ["ň"], "o": ["ö"],
        "s": ["ş"], "u": ["ü"], "y": ["ý"],
    },
    # Assamese: the two letters that distinguish it from Bengali — Assamese
    # ra ৰ (long-press Bengali ra র) and wa ৱ (long-press Bengali ba ব).
    "as": {"র": ["ৰ"], "ব": ["ৱ"]},
    # --- Tier E: European minority (letters beyond the inherited us menus) ---
    # Welsh: the circumflex ("to bach") set — incl. ŵ ŷ on the semivowels.
    "cy": {
        "a": ["â"], "e": ["ê"], "i": ["î"], "o": ["ô"], "u": ["û"],
        "w": ["ŵ"], "y": ["ŷ"],
    },
    # Occitan: ç plus its grave/acute vowels and ï ü.
    "oc": {
        "a": ["à", "á"], "c": ["ç"], "e": ["è", "é"], "i": ["í", "ï"],
        "o": ["ò", "ó"], "u": ["ú", "ü"],
    },
    # Breton: ñ, ç, the circumflex vowels and ù.
    "br": {
        "a": ["â"], "e": ["ê"], "i": ["î"], "o": ["ô"], "u": ["û", "ù"],
        "n": ["ñ"], "c": ["ç"],
    },
    # Luxembourgish: umlauts + é.
    "lb": {"a": ["ä"], "e": ["ë", "é"], "i": ["ï"], "o": ["ö"], "u": ["ü"]},
    # Frisian: circumflex + diaeresis vowels.
    "fy": {"a": ["â"], "e": ["ê", "ë"], "i": ["î", "ï"], "o": ["ô"], "u": ["û", "ú"]},
    # --- Tier F: African (alphabet letters beyond a–z; a DIACRITIC entry
    # replaces the inherited us menu, so standard accents that the language
    # also uses are listed here too, e.g. Wolof à/é) ---
    "wo": {"a": ["à"], "e": ["é", "ë"], "n": ["ñ", "ŋ"], "o": ["ó"]},
    "ln": {"e": ["ɛ"], "o": ["ɔ"]},
    "ny": {"w": ["ŵ"]},
    "mg": {"n": ["ñ"], "o": ["ô"]},
    "tn": {"s": ["š"]},
    "nso": {"s": ["š"], "e": ["ê"], "o": ["ô"]},
    "lg": {"n": ["ŋ", "ɲ"]},
    "ak": {"e": ["ɛ"], "o": ["ɔ"]},
    "ee": {
        "d": ["ɖ"], "e": ["ɛ"], "f": ["ƒ"], "g": ["ɣ"], "o": ["ɔ"],
        "v": ["ʋ"], "n": ["ŋ"],
    },
    "bm": {"e": ["ɛ"], "o": ["ɔ"], "n": ["ɲ", "ŋ"]},
    "ff": {"b": ["ɓ"], "d": ["ɗ"], "y": ["ƴ"], "n": ["ŋ"]},
    "ve": {"d": ["ḓ"], "l": ["ḽ"], "n": ["ṋ"], "t": ["ṱ"]},
    # --- Tier G: Pacific macron long vowels + Nahuatl macrons ---
    "mi": {"a": ["ā"], "e": ["ē"], "i": ["ī"], "o": ["ō"], "u": ["ū"]},
    "haw": {"a": ["ā"], "e": ["ē"], "i": ["ī"], "o": ["ō"], "u": ["ū"]},
    "sm": {"a": ["ā"], "e": ["ē"], "i": ["ī"], "o": ["ō"], "u": ["ū"]},
    "to": {"a": ["ā"], "e": ["ē"], "i": ["ī"], "o": ["ō"], "u": ["ū"]},
    "fj": {"a": ["ā"], "e": ["ē"], "i": ["ī"], "o": ["ō"], "u": ["ū"]},
    "ty": {"a": ["ā"], "e": ["ē"], "i": ["ī"], "o": ["ō"], "u": ["ū"]},
    "nah": {"a": ["ā"], "e": ["ē"], "i": ["ī"], "o": ["ō"]},
}

TEMPLATE = "us"  # universal base: letter grid + symbol views + control row
NUM_ROW = list(range(2, 14))  # evdev number-row keycodes (1…= )
BASE_ROWS = [list(range(16, 26)), list(range(30, 39)), list(range(44, 51))]
# Extra keycodes appended to each letter row for letters the US 26-key
# grid can't hold — the punctuation-position keys various layouts strand a
# letter on: 41 (`\`~``, top-left), 43 (`\`, right of home), 53 (right of
# bottom), 86 (the ISO `<>` key). Catches Icelandic þ (53), Hungarian ű
# (43), Maltese ċ (41) / ż (86), Belarusian/Kyrgyz/Tajik ё (41), and ~20
# more. Non-letters on these keys are dropped by the per-script filter, and
# a per-view dedup below removes any that merely repeat a base-grid letter.
EXTRA_ROWS = [[26, 27, 41], [39, 40, 43], [51, 52, 53, 86]]
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
    x = _lib()
    ctx = x.xkb_context_new(0)
    rmlvo = _Rmlvo(b"", b"", layout.encode(), variant.encode(), b"")
    km = x.xkb_keymap_new_from_names(ctx, ctypes.byref(rmlvo), 0)
    if not km:
        raise SystemExit(f"libxkbcommon could not compile {layout!r}/{variant!r}")
    st = x.xkb_state_new(km)
    codes = NUM_ROW + [c for row in BASE_ROWS + EXTRA_ROWS for c in row]

    def ch(code):
        buf = ctypes.create_string_buffer(32)
        x.xkb_state_key_get_utf8(st, code + 8, buf, 32)
        return buf.value.decode("utf-8")

    base = {c: ch(c) for c in codes}
    x.xkb_state_update_key(st, KEY_LEFTSHIFT + 8, 1)
    shift = {c: ch(c) for c in codes}
    x.xkb_state_update_key(st, KEY_LEFTSHIFT + 8, 0)
    return base, shift


def _spacer(width):
    """A non-emitting gap key of the given width (in key units)."""
    return {"width_units": width, "label": "", "output": {"kind": "none"}}


def kept_codes(base_codes, extra_codes, chars, script):
    """Keycodes to emit for a row, given this view's character map.

    Latin keeps all 26 base codes (always ASCII letters) and adds the
    extra codes that are letters (dropping punctuation). Every other
    script drops empty and stray-ASCII codes but keeps non-ASCII ones —
    which retains Indic combining matras (not `.isalpha()`) while pruning
    Tamil's blank slots and xkb fall-through ASCII (e.g. Bengali's `z`)."""
    if script == "latin":
        # `.isalpha()` is True for the Spanish ordinal indicators º/ª (the
        # es keymap parks them on keycode 41), but they aren't alphabet
        # letters — drop them so es-base layouts (eu gl ceb qu …) don't get
        # a stray top-left key. They remain reachable via the symbol view.
        return list(base_codes) + [
            c for c in extra_codes if chars[c].isalpha() and chars[c] not in ("º", "ª")
        ]
    # Non-Latin: keep Unicode letters (L*), marks (M*, i.e. Indic matras /
    # Syriac points), and punctuation (P*, e.g. Devanagari danda ।, Arabic
    # ؛ ، — which caseless scripts put on their shift level). Drop ASCII,
    # empty, and the non-textual categories — numbers (Tifinagh ²), symbols,
    # and format chars incl. zero-width joiners (Malayalam ZWJ = Cf) — that
    # would otherwise render as junk or blank caps.
    def keep(c):
        g = chars[c]
        return bool(g) and not g.isascii() and unicodedata.category(g[0])[0] in ("L", "M", "P")

    return [c for c in base_codes if keep(c)] + [c for c in extra_codes if keep(c)]


def build(base_dir, lid, name, xlayout, xvariant, script):
    base, shift = maps_for(xlayout, xvariant)

    with open(f"{base_dir}/{TEMPLATE}.json", encoding="utf-8") as f:
        layout = json.load(f)
    layout["id"] = lid
    layout["name"] = name
    layout["xkb_layout"] = xlayout
    if xvariant:
        layout["xkb_variant"] = xvariant
    else:
        layout.pop("xkb_variant", None)
    layout["direction"] = "rtl" if script in ("arabic", "syriac", "thaana") else "ltr"
    # Prediction is data-gated: enable it iff a committed n-gram model for
    # this id exists (`assets/lang/predict/<id>.fst`, built by `make
    # predict-assets` from a permissively-licensed corpus). Self-healing —
    # a regen re-derives the flag, so model-backed layouts keep prediction
    # and the rest stay layout-only. (Country-code core layouts whose
    # prediction_lang differs from the id — us→en, se→sv — are hand-authored,
    # not generated here.)
    fst = os.path.join(base_dir, "..", "..", "..", "assets", "predict", f"{lid}.fst")
    if os.path.exists(fst):
        layout["prediction_enabled"] = True
        layout["prediction_lang"] = lid
    else:
        layout["prediction_enabled"] = False
        layout.pop("prediction_lang", None)
    layout["swipe_enabled"] = script == "latin"
    # Scripts DejaVu can't draw need their Noto face loaded on demand.
    if script in DEJAVU_SCRIPTS:
        layout.pop("font_script", None)
    else:
        layout["font_script"] = script

    # Letters the national layout parks on the number row because the
    # alphabet overflows the 32-key letter grid (Kazakh `ә і ң ғ …`,
    # Lithuanian `ą č ę …`). Skip any already reachable in the grid, so
    # Mongolian — whose number row is ordinary `е`/`щ` — gets no extra
    # row and generates as a plain Cyrillic layout.
    grid_letters = set()
    for br, er in zip(BASE_ROWS, EXTRA_ROWS):
        grid_letters.update(base[c] for c in kept_codes(br, er, base, script))
    extra_top = [c for c in NUM_ROW if base[c].isalpha() and base[c] not in grid_letters]

    keep_lp = script == "latin"
    for view in layout["views"]:
        vid = view["id"]
        if vid == "base":
            chars = base
        elif vid == "shift":
            chars = shift
        else:
            continue
        rows = view["rows"]
        old_lp = {}
        for row in rows:
            for key in row.get("keys", []):
                out = key.get("output", {})
                if out.get("kind") == "keycode" and key.get("long_press"):
                    old_lp[out["code"]] = key["long_press"]

        shift_key = rows[2]["keys"][0]
        backspace = rows[2]["keys"][-1]

        def spacer():
            return {"width_units": 0.5, "label": "", "output": {"kind": "none"}}

        diac = DIACRITIC_LONGPRESS.get(lid)

        def kc(code):
            label = chars[code]
            key = {"width_units": 1, "label": label, "output": {"kind": "keycode", "code": code}}
            if diac is not None:
                # National letters on long-press only, cased to the key
                # (a→ā, A→Ā); no inherited US accent menu.
                marks = diac.get(label.lower())
                if marks:
                    key["long_press"] = [
                        {"kind": "text", "text": m.upper() if label.isupper() else m}
                        for m in marks
                    ]
            elif keep_lp and code in old_lp:
                key["long_press"] = old_lp[code]
            return key

        r0 = kept_codes(BASE_ROWS[0], EXTRA_ROWS[0], chars, script)
        r1 = kept_codes(BASE_ROWS[1], EXTRA_ROWS[1], chars, script)
        r2 = kept_codes(BASE_ROWS[2], EXTRA_ROWS[2], chars, script)
        rows[0]["keys"] = [kc(c) for c in r0]
        rows[1]["keys"] = [spacer()] + [kc(c) for c in r1] + [spacer()]
        rows[2]["keys"] = [shift_key] + [kc(c) for c in r2] + [backspace]

        # Prepend the overflow letters as a top row, centred to the top
        # letter row's width with edge spacers (a 2-key row for ky/tg
        # then sits as two normal keys, not two half-screen ones).
        if extra_top:
            pad = max(0.0, (len(r0) - len(extra_top)) / 2.0)
            top = [kc(c) for c in extra_top]
            if pad > 0:
                top = [_spacer(pad)] + top + [_spacer(pad)]
            view["rows"].insert(0, {"height_units": rows[0].get("height_units", 1.0), "keys": top})

        # Drop duplicate letter keys within this view (a letter reachable
        # from two keycodes — e.g. Lao matras that also sit on 43/53). The
        # first, earlier-row occurrence wins, so the base grid keeps the
        # letter and only the redundant extra key is removed; no letter is
        # ever lost. Non-keycode keys (space, shift, spacers) are untouched.
        seen = set()
        for r in view["rows"]:
            deduped = []
            for k in r["keys"]:
                if k.get("output", {}).get("kind") == "keycode":
                    if k["label"] in seen:
                        continue
                    seen.add(k["label"])
                deduped.append(k)
            r["keys"] = deduped

    # A right-to-left layout is stored in reading order, its rows' keys
    # right to left: the renderer mirrors each of its rows
    # (osk_core::layout::geometry::key_rects), so the first key is drawn
    # rightmost. Stored as the left-to-right grid above, it was drawn the
    # wrong way round, Backspace at the left.
    if layout["direction"] == "rtl":
        for view in layout["views"]:
            for r in view["rows"]:
                r["keys"].reverse()

    path = f"{base_dir}/{lid}.json"
    with open(path, "w", encoding="utf-8") as f:
        json.dump(layout, f, ensure_ascii=False, indent=2)
        f.write("\n")
    row0 = " ".join(base[c] for c in kept_codes(BASE_ROWS[0], EXTRA_ROWS[0], base, script))
    tail = f"  +toprow=[{' '.join(base[c] for c in extra_top)}]" if extra_top else ""
    return f"{lid:8} ({xlayout}/{xvariant or '-'}): row0=[{row0}]{tail}"

# Laid out by hand now, each as its language's phone keyboard (AOSP's
# arrangement) with every letter on a key: regenerating them would cut
# them back to the US grid, as these scripts first made them.
HAND_MADE = {"hi"}


def main():
    base_dir = sys.argv[1] if len(sys.argv) > 1 else "layouts"
    only = set(sys.argv[2].split(",")) if len(sys.argv) > 2 else None
    for lid, name, xlayout, xvariant, script in LANGS:
        if (only and lid not in only) or lid in HAND_MADE:
            continue
        print(build(base_dir, lid, name, xlayout, xvariant, script))


if __name__ == "__main__":
    main()
