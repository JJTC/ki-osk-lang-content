#!/usr/bin/env python3
"""fineweb-text.py — a language's FineWeb-2 web text as sentences, one a
line: what its model learns from, for languages with no Wikipedia of
their own.

FineWeb-2 (Hugging Face; ODC-By 1.0, subject to Common Crawl's Terms of
Use) is web text from Common Crawl, sorted by language with the GlotLID
classifier, which errs most between close languages: its Dogri holds
Punjabi poetry, its Angika is Hindi news. So a document is kept only
when it reads as its language:

- a junk document is left out: one whose words are mostly single
  characters or digits (scanned books read by OCR), or whose letters are
  more than a fifth combining marks;
- so is one in which a big neighbour's own function words (Hindi's है का
  में को…, Urdu's ہے کا میں کو…, words the language itself does not use)
  are more than 6% of the words;
- and for a language with RIVAL words in the table below, one whose own
  function words (OWN) are not more than the rival's and Hindi's
  together, or are under 1% of its words (Dogri's ऐ गी कन्नै नेई against
  Punjabi's नूं तों विच नाल).

Sindhi in Devanagari (sd-Deva) shares में and का with Hindi, so Hindi's
other words mark a document as Hindi; and its implosives, written with
the letters Unicode has for them (ॻ ॼ ॾ ॿ) or, more often on the web,
as ग ज ड द ब with the underline ॒ (U+0952, after any vowel sign), are
read as those letters, which the keyboard types. Meitei in the Bengali
script (mni-Beng) has Bengali's function words (এবং করে হয়…) against it.
Lao, written without spaces between words, ends a sentence at a line's
end too, its sentences being as often unpunctuated, and a short one is
one of fewer than 15 letters rather than 3 words; FineWeb-2 has far more
of it than a model needs (780 million characters), so every 12th of its
row groups is read (`every`), 70 million characters. Its sentences are
cut where a word in another script stands (English names, mostly: half
the words of a model learned with them were English, each rare), and
its letters written two ways are written as the keyboard types them
(ໍາ as ຳ, ຫນ ຫມ ຫລ as ໜ ໝ ຫຼ; osk_data::wordbreak::lao_fold).
Kokborok (trp), written in Latin letters, has English against it, its
words counted in lower case, and a sentence of it whose words are more
than 15% English's (`sentence_rival`) is left out (39 of the 4 843 of
the documents kept). Mizo is read so too, every third row group of it
(100 MB in all). Khasi has Pnar (Jaintia) against it as well, which
FineWeb-2 files under Khasi (wyrta.com, two fifths of its documents):
Khasi writes ba jong bad kiba kaba, Pnar wa ya kiwa kawa ko wow.
Khmer and Burmese, written without spaces between words, are read as
Lao is, a sentence ending at ។ ៕ and ။ as well, a space after them or
not; every 40th row group of each (some 3 000 and 7 000 million
characters in all). Burmese's web was written in Zawgyi, a font's
encoding of its letters at other code points, until 2019 (two fifths of
FineWeb-2's Burmese text, most of it before 2020; 9% of 2024's): a
document that reads as Zawgyi, ေ before its consonant, ္ ending a
syllable or the letters of U+1060 to U+1097 being more than 0.3% of its
Burmese characters (Unicode's Burmese has none; Zawgyi's 5% and more),
is left out, as the keyboard and ICU's word list are Unicode's, and so
is one whose Zawgyi was read as Unicode (ျ for ်, ွ for ြ: ငျ, which
Burmese never writes, twice or more; 3% of the text kept before); and
the digit ၀ written for the letter ဝ, which older text does, is read as
ဝ where a letter or a sign follows it and no digit comes before it.

Thirty-eight languages that had a keyboard and no model, FineWeb-2 has
hundreds of millions of characters of: Afrikaans, Albanian, Bashkir,
Bosnian, Cebuano, Chichewa, Corsican, Dhivehi, Faroese, Frisian,
Haitian Creole, Hausa, Hawaiian, Igbo, Irish, Javanese, Kazakh,
Kinyarwanda, Kyrgyz, Luxembourgish, Malagasy, Maltese, Māori, Samoan,
Scottish Gaelic, Sesotho, Shona, Slovenian, Somali, Sundanese, Swahili,
Tajik, Turkmen, Uzbek, Welsh, Xhosa, Yoruba and Zulu. Of each, the first
file of its train split is read, every Nth row group (some 45 MB of the
parquet, 100 million characters or so), and a sentence of English is
left out, one whose words are more than 15% English's own (the, and,
that…: not in, is, of, which Afrikaans writes too), or for Kazakh,
Kyrgyz, Tajik and Bashkir of Russian (в на что…). Their classifier's
neighbours do not show: a sample's commonest words are each language's
own (Frisian's fan it yn, Faroese's í á við sum, Cebuano's nga ug usa,
Corsican's u è chì).

Central Kurdish (Sorani) is read every 10th row group. A third of its
web text is in the old Arabic-keyboard spelling, which writes the vowel
ە as ه (ئهنجلۆس, له), ه being h as well: a document in it is left out,
one more of whose words begin ئه or ده and a letter (ئهو, دهکات) than ئە
or دە (ئەو, دەکات), or in which ه ends more words than ە is written, and
so is a sentence of it by how its words begin; so is a document whose
letters are under 4% Sorani's own (ە ێ ۆ ڵ ڕ: Persian or Arabic), and a
sentence more than 15% of whose words are Arabic's (فی علی الذی…).
The letters Arabic and Persian keyboards typed for Sorani's (ك ي ى ھ)
are read as those its keyboard types (ک ی ی ه), a word's last ه as ە
but after ا (شاه, گوناه) and in الله (half the words ending in ه have
their ە form commoner: ە typed as the old keyboards had it, لاپەڕه,
ئیسلامەوه), and the tatweel that stretches a word goes. Nigerian Pidgin's documents are
kept where its own words (di dey wey dem na don…) are at least 5% of
their words and more than English's (the is was are…: the rest of what
FineWeb-2 and GlotCC file under it is poster shops, porn and English),
and a sentence of English is left out. Its GlotCC text is read after its
FineWeb-2 text (`--exclude`): a sentence that has is neither written
again nor held out, so that the held-out text of each is in neither
corpus; FineWeb-2's test split of it is six documents, GlotCC's every
20th held out.

Nineteen languages of Southeast Asia are read whole (Hmong Daw every
6th row group): Hmong Daw, Hakha Chin, Jingpho, Hiligaynon, Iloko,
Central Bikol, Pangasinan, Kapampangan, Kinaray-a, Iban, Batak Toba,
Acehnese, Karo, Simalungun, Banjar, Kadazandusun, Balinese, Tetum and Iu
Mien. A sentence of the big languages written beside them is left out:
of English's, and of Tagalog's in the Philippines' (ng ay hindi, where
Hiligaynon writes sang indi), Indonesian's or Malay's in Indonesia's and
Borneo's (dengan untuk tidak, where Iban writes enggau ngagai enda),
Portuguese's and Indonesian's in Tetum, Vietnamese's in Iu Mien (whose
Bible site is Vietnamese's beside it). Four fifths of Balinese's is a
dictionary site's entries, whose labels (In English, In Balinese) leave
a sentence out, and Indonesian's yang dan ini itu dari, for which
Balinese writes sane lan puniki punika saking, count against it too.
Tetum's saltillo (ꞌ), a letter, written for the apostrophe of ne'e, is
read as the apostrophe. FineWeb-2's Betawi is everyday Indonesian, its
Musi porn spam, its Banjar in the Arabic script Malaysia's Jawi and its
Chavacano Spanish lyrics: none is read.

Each kept document is cut into sentences after a full stop, danda,
question or exclamation mark; a sentence of fewer than three words, of
more than 1 000 characters, with wiki markup, or fewer than half of whose
letters are the language's script is left out, and each sentence is
written once. The corpus is the `train` split's; the held-out text, at
most EVAL sentences of the `test` split (which FineWeb-2 keeps apart for
measuring) that the corpus does not have, is what eval-predict measures
the model on. Neither is committed: the text is the web's.

Garhwali and Rajasthani come from MADLAD-400 (Google; Common Crawl's
text, audited by language), its JSON lines read the same way: a
document is kept where Hindi's function words are under 6% of its words
and its own (Garhwali's मा च छौ ह्वे कु…, Rajasthani's री सूं मांय रो…)
are at least 1% and more than Hindi's, and for Garhwali Nepali's too
(पनि भएको हुन्छ…: its casino spam shares मा and नि). With no test split, every 20th
document kept is held out instead (`--hold`). Newari comes from GlotCC
(Common Crawl's text sorted for minority languages), its parquet files'
text in "content", a document kept where its own words (दु खः नं थ्व
जूगु…) are at least 1% and Hindi's and Nepali's under 6%, every 20th held
out as MADLAD-400's are.

    fineweb-text.py --lang L [--eval N] OUT EVAL --train F... [--test F... | --hold N]
                    [--exclude F...]

Needs pyarrow for parquet. The files are read in the order given, so
the same files always give the same text.
"""

import argparse
import gzip
import json
import re
import sys
import unicodedata


END = re.compile(r"(?<=[.!?।॥۔؟])\s+")
# Khmer's and Burmese's ends of sentences, often with no space after them.
UNSPACED_END = re.compile(r"(?<=[។៕။])\s*")
# What marks Burmese text as Zawgyi's: ေ before its consonant, ္ ending a
# syllable, and the shapes Zawgyi put at U+1060 to U+1097.
ZAWGYI = re.compile(r"(?:^|[^\u1000-\u109F])\u1031|\u1039(?![\u1000-\u1021])|[\u1060-\u1097]")
MYANMAR = re.compile(r"[\u1000-\u109F]")
STEERING = re.compile("[‎‏‪-‮⁦-⁩﻿]")
MARKUP = re.compile(r"\[\[|\]\]|\{\{|\}\}")
WORDS = re.compile(r"[\s.,;:!?।॥۔؟،\"'()\[\]{}<>«»“”‘’\-–—/|*#@&%$+=~`^]+")

HINDI = set("है हैं था थी थे का की में को ने नहीं यह वह और लिए".split())
URDU = set("ہے ہیں تھا تھی تھے کا کی میں کو نے نہیں یہ وہ اور لیے کے سے".split())
PUNJABI = set("नूं तों विच नाल वी सी लई हुण".split())
DOGRI = set("ऐ गी कन्नै कन्ने नेई बी लेई एह् ओह् च".split())
GARHWALI = set("मा च छौ छन छा ह्वे कु बि नि तैं".split())
# Nepali's, which Garhwali does not use (it shares मा and नि; its and is
# अर, Nepali's र), and the words of the Nepali casino spam MADLAD-400's
# Garhwali holds.
NEPALI = set("पनि भएको गरेको हुन्छ थियो गर्न छन् हुने गरी र छैन तपाईंको तपाइँको "
             "गर्नुहोस् मार्फत क्यासिनो कैसीनो बोनस".split())
RAJASTHANI = set("री सूं मांय रो रै नीं पण नै रा".split())
ENGLISH = set("the of and to in is that for on with as was are by this it from be at an or".split())
# English's words that no language read beside it writes too (Afrikaans
# has in, is, of and op; Swahili and Zulu a; Slovenian to and on): what
# marks a sentence of theirs as English.
ENGLISH_ONLY = set("the and that for with was are by this from which have".split())
# Russian's words that Kazakh, Kyrgyz, Tajik and Bashkir do not write (not
# не, which Kazakh's is "what"): what marks a sentence of theirs as Russian.
RUSSIAN_ONLY = set("в на что с по это как для от из но или так же все был при".split())
KOKBOROK = set("tei ni no bai khe bo kha kaisa tongo bini hwnwi wngkha twi nwng ang".split())
MIZO = set("chu leh hi kan te hian ah pawh tur chuan ni ang tak tih awm".split())
KHASI = set("ba jong bad kiba kaba uba ia ïa".split())
NEWARI = set("दु खः नं थ्व जूगु याःगु यायेगु धकाः यानाः कथं".split())
# Pnar (Jaintia), Khasi's neighbour, which FineWeb-2 files under Khasi.
PNAR = set("wa ya kiwa kawa uwa ko wow".split())
BENGALI = set("এবং করে হয় এই যে থেকে জন্য করা তার আমি আমাদের হয়েছে একটি সঙ্গে করেন হবে".split())
# Nigerian Pidgin's own words, and English's that it does not write (it
# writes di, dey, wey, na, don, dis, dat; for, and, of and with as English
# does): a document of it has more of its own, a sentence of English more
# than 15% of English's.
NAIJA = set("di dey wey dem na don una wetin sey pipo dis im fit wan bin dat dia oda tok".split())
NAIJA_RIVAL = set("the is was are this that which have were".split())
# The words of the big languages that Southeast Asia's write beside them,
# which they do not write themselves (Iban's enggau ngagai enda for
# Indonesian's and Malay's dengan kepada tidak, Hiligaynon's sang indi for
# Tagalog's ng hindi): what marks a sentence of them in their text.
INDONESIAN_ONLY = set("dengan untuk tidak adalah akan bahwa bahawa oleh kepada tersebut".split())
TAGALOG_ONLY = set("ng ay hindi ito nang kanyang ating".split())
PORTUGUESE_ONLY = set("que não uma os do da em dos das pelo".split())
VIETNAMESE_ONLY = set("và của là có không được những người một các".split())
# And Indonesian's that Balinese writes its own for (sane, lan, puniki,
# punika, saking: yang, dan, ini, itu, dari), and the words of the
# Balinese dictionary site's labels, which four fifths of FineWeb-2's
# Balinese is (In English, In Balinese, Balinese-English Dictionary):
# Balinese writes basa Inggris, basa Bali.
BALINESE_RIVAL = INDONESIAN_ONLY | set("yang dan ini itu dari pada dalam".split())
LABELS = set("english balinese indonesian".split())
# Arabic's words, after Sorani's letters are folded (فی for في), that
# Sorani does not write: what marks a sentence of Arabic in its text.
ARABIC_ONLY = set("فی علی إلی الی الذی التی هذا هذه عن".split())
# Sorani's own letters, which Persian and Arabic do not write.
SORANI = set("ەێۆڵڕ")
# Sorani's letters as Arabic and Persian keyboards wrote them: Arabic's
# kaf and yeh, alef maksura for a final yeh, and the doachashmee heh,
# each read as the letter the keyboard types.
SORANI_FOLD = str.maketrans({"\u0643": "\u06A9", "\u064A": "\u06CC", "\u0649": "\u06CC", "\u06BE": "\u0647"})
# ه ending a word: in the old spelling, ە mostly. In text that writes the
# rest as Unicode's, it is ە typed as the old keyboards had it (لاپەڕه,
# ئیسلامەوه, له), but after ا (شاه, گوناه, ئاگاه) and in الله.
FINAL_HEH = re.compile("\u0647(?![\u0600-\u06FF])")
# A word begun with ئه or ده and a letter: the old spelling's ئهو دهکات,
# Unicode's ئەو دەکات.
OLD_START = re.compile("(?<![\u0600-\u06FF])[\u0626\u062F]\u0647(?=[\u0600-\u06FF])")
NEW_START = re.compile("(?<![\u0600-\u06FF])[\u0626\u062F]\u06D5")
FINAL_AE = re.compile("(?<=[\u0620-\u06FF])(?<!\u0627)(?<!\u0627\u0644\u0644)\u0647(?![\u0600-\u06FF])")
# Where a sentence of text ends, as `sentences` cuts it.
SENTENCE_GAP = re.compile(r"((?<=[.!?\u061F])\s+|\n)")

# Sindhi's implosives: the letter, or (on the web) its plain consonant
# with the underline after any vowel signs.
IMPLOSIVE = {"ग": "ॻ", "ज": "ॼ", "ड": "ॾ", "द": "ॾ", "ब": "ॿ"}
UNDERLINED = re.compile("([गजडदब])([\u0900-\u0903\u093a-\u094c\u094e\u094f\u0955-\u0957\u0962\u0963]*)\u0952")


def lao(text):
    return (text.replace("\u0ecd\u0eb2", "\u0eb3").replace("\u0eab\u0e99", "\u0edc")
            .replace("\u0eab\u0ea1", "\u0edd").replace("\u0eab\u0ea5", "\u0eab\u0ebc"))


def burmese(text):
    # The digit zero for the letter wa, where a letter or a sign follows
    # it and no digit comes before it (၀င်း, not ၁၀ကြိမ်).
    return re.sub("(?<![\u1040-\u1049])\u1040(?=[\u1000-\u103F])", "\u101D", text)


def zawgyi(text):
    # Or Zawgyi's letters read as Unicode's, ျ for ် and ွ for ြ: ငျ, which
    # Burmese never writes, for the ever-present င်.
    return (len(ZAWGYI.findall(text)) > 0.003 * max(1, len(MYANMAR.findall(text)))
            or text.count("\u1004\u103B") >= 2)


def old_start(text):
    """Text that writes ە as ه within words, the old Arabic-keyboard
    spelling, in which ه is h as well and cannot be read back: more of its
    words begin ئه or ده and a letter than ئە or دە."""
    return len(OLD_START.findall(text)) > len(NEW_START.findall(text))


def old_sorani(text):
    """A document not written as Unicode's Sorani: one whose letters are
    under 4% Sorani's own (ە ێ ۆ ڵ ڕ: Persian or Arabic), or in the old
    spelling, by how its words begin or by ه ending more of them than ە
    is written (each test finds old documents the other does not)."""
    letters = sum(c.isalpha() for c in text)
    return (sum(c in SORANI for c in text) < 0.04 * max(1, letters) or old_start(text)
            or len(FINAL_HEH.findall(text)) > text.count("\u06D5"))


def sorani(text):
    # A sentence in the old spelling, which a document in Unicode's quotes
    # or a comment under it writes, is left out; the rest is read as its
    # keyboard types it, the tatweel that stretches a word's letters gone.
    parts = SENTENCE_GAP.split(text)
    text = "".join("" if i % 2 == 0 and old_start(p) else p for i, p in enumerate(parts))
    return FINAL_AE.sub("\u06D5", text.translate(SORANI_FOLD)).replace("\u0640", "")


def tetum(text):
    # The saltillo, a letter, written for the apostrophe of ne'e, ha'u.
    return text.replace("\uA78C", "'")


def sindhi(text):
    text = UNDERLINED.sub(lambda m: IMPLOSIVE[m.group(1)] + m.group(2), text)
    return text.replace("\u0952", "")


# Each language: its script (Unicode's name for it), the neighbour whose
# function words mark a document as not its own, for a language that a
# rival is mistaken for, its own words and the rival's (or its own letter
# and the rival's, which it must write more often), and how its text is
# read where the web spells it two ways.
LANGS = {
    "brx": {"script": "DEVANAGARI", "neighbour": HINDI},
    "doi": {"script": "DEVANAGARI", "neighbour": HINDI, "own": DOGRI, "rival": PUNJABI},
    "hne": {"script": "DEVANAGARI", "neighbour": HINDI},
    "mag": {"script": "DEVANAGARI", "neighbour": HINDI},
    "brh": {"script": "ARABIC", "neighbour": URDU},
    "sd-Deva": {"script": "DEVANAGARI", "neighbour": HINDI - {"में", "का"}, "normalize": sindhi},
    "mni-Beng": {"script": "BENGALI", "neighbour": BENGALI},
    # Assamese shares Bengali's script and এই, যে, হয়; its r is ৰ where
    # Bengali's is র, which a document must write more often.
    "as": {"script": "BENGALI", "neighbour": BENGALI - {"এই", "যে", "হয়"}, "letters": ("ৰ", "র"),
           "every": 6},
    # MADLAD-400's (no test split: every 20th document held out).
    "gbm": {"script": "DEVANAGARI", "neighbour": HINDI, "own": GARHWALI, "rival": NEPALI},
    "raj": {"script": "DEVANAGARI", "neighbour": HINDI, "own": RAJASTHANI, "rival": set()},
    # Kokborok, in Latin letters: a document whose own words outnumber
    # English's, the English sentences in it left out.
    "trp": {"script": "LATIN", "neighbour": set(), "own": KOKBOROK, "rival": ENGLISH,
            "sentence_rival": ENGLISH},
    # Mizo, in Latin letters: as Kokborok; every 3rd row group.
    "lus": {"script": "LATIN", "neighbour": set(), "own": MIZO, "rival": ENGLISH,
            "sentence_rival": ENGLISH, "every": 3},
    # Khasi: a document whose own words outnumber Pnar's and English's.
    "kha": {"script": "LATIN", "neighbour": set(), "own": KHASI, "rival": PNAR | ENGLISH,
            "sentence_rival": ENGLISH},
    # GlotCC's (no test split: every 20th document held out). Newari: its
    # own words, and neither Hindi's nor Nepali's.
    "new": {"script": "DEVANAGARI", "neighbour": HINDI | NEPALI, "own": NEWARI, "rival": set()},
    "lo": {"script": "LAO", "neighbour": set(), "lines": True, "unspaced": True, "every": 12,
           "normalize": lao, "script_only": True},
    # The languages with a keyboard and no model before, whose web text
    # FineWeb-2 has plenty of: every Nth row group, for some 100 million
    # characters; a sentence of English left out.
    "sw": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 30},
    "af": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 80},
    "sl": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 100},
    "zu": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 5},
    "sq": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 108},
    "kk": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "every": 108},
    "uz": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 45},
    "bs": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 108},
    "ky": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "every": 33},
    "tg": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "every": 27},
    "cy": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 26},
    "so": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 22},
    "ga": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 22},
    "mt": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 16},
    "ha": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 13},
    "lb": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 11},
    "mg": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 9},
    "rw": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 8},
    "gd": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 8},
    "fy": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 7},
    "jv": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 7},
    "tk": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 7},
    "ht": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 7},
    "ba": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "every": 6},
    "xh": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 6},
    "fo": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 6},
    "ceb": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 6},
    "mi": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 5},
    "su": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 4},
    "co": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 4},
    "ny": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 4},
    "sm": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 4},
    "sn": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 3},
    "ig": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 3},
    "st": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 3},
    "haw": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 3},
    "yo": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 3},
    "dv": {"script": "THAANA", "neighbour": set(), "every": 8},
    "km": {"script": "KHMER", "neighbour": set(), "lines": True, "unspaced": True, "every": 40,
           "script_only": True},
    "my": {"script": "MYANMAR", "neighbour": set(), "lines": True, "unspaced": True, "every": 40,
           "normalize": burmese, "script_only": True, "foreign": zawgyi},
    "ckb": {"script": "ARABIC", "neighbour": set(), "foreign": old_sorani, "normalize": sorani,
            "sentence_rival": ARABIC_ONLY, "every": 10},
    "pcm": {"script": "LATIN", "neighbour": set(), "own": NAIJA, "rival": NAIJA_RIVAL, "own_least": 0.05,
            "sentence_rival": NAIJA_RIVAL},
    "mww": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 6},
    "cnh": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY},
    "kac": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY},
    "hil": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | TAGALOG_ONLY},
    "ilo": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | TAGALOG_ONLY},
    "bcl": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | TAGALOG_ONLY},
    "pag": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | TAGALOG_ONLY},
    "pam": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | TAGALOG_ONLY},
    "krj": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | TAGALOG_ONLY},
    "iba": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | INDONESIAN_ONLY},
    "bbc": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | INDONESIAN_ONLY},
    "ace": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | INDONESIAN_ONLY},
    "btx": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | INDONESIAN_ONLY},
    "bts": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | INDONESIAN_ONLY},
    "bjn": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | INDONESIAN_ONLY},
    "dtp": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | INDONESIAN_ONLY},
    "ban": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | BALINESE_RIVAL,
            "sentence_never": LABELS},
    "tet": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | PORTUGUESE_ONLY | INDONESIAN_ONLY,
            "normalize": tetum},
    "ium": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | VIETNAMESE_ONLY},
}


def words(text):
    return [w for w in WORDS.split(text) if w]


def share(ws, vocab):
    return sum(w in vocab for w in ws) / len(ws) if ws else 0.0


def junk(text, ws, script):
    if not ws:
        return True
    if sum(len(w) == 1 or w.isdigit() for w in ws) > 0.5 * len(ws):
        return True
    if script != "ARABIC":
        # An abugida's vowel signs are marks: only Arabic's are suspect.
        return False
    letters = [c for c in text if c.isalpha() or unicodedata.category(c).startswith("M")]
    marks = sum(unicodedata.category(c).startswith("M") for c in letters)
    return marks > 0.2 * len(letters)


def kept(text, conf):
    # In lower case: Latin's function words start sentences too.
    ws = words(text.lower())
    if junk(text, ws, conf["script"]):
        return False
    if "foreign" in conf and conf["foreign"](text):
        return False
    if share(ws, conf["neighbour"]) > 0.06:
        return False
    if "letters" in conf:
        own, rival = conf["letters"]
        if text.count(own) <= text.count(rival):
            return False
    if "own" in conf:
        own = share(ws, conf["own"])
        if own < conf.get("own_least", 0.01) or own <= share(ws, conf["rival"]) + share(ws, conf["neighbour"]):
            return False
    return True


def in_script(s, script):
    letters = [c for c in s if c.isalpha()]
    own = sum(unicodedata.name(c, "").startswith(script) for c in letters)
    return 2 * own >= len(letters)


def long_enough(s, conf):
    if conf.get("unspaced"):
        return sum(c.isalpha() for c in s) >= 15
    return len(s.split()) >= 3


def sentences(text, conf):
    script = conf["script"]
    text = STEERING.sub("", text)
    if "normalize" in conf:
        text = conf["normalize"](text)
    pieces = text.split("\n") if conf.get("lines") else [text]
    for piece in pieces:
        ends = UNSPACED_END.split(piece) if conf.get("unspaced") else [piece]
        for s in (s for e in ends for s in END.split(e)):
            for s in script_runs(s, script) if conf.get("script_only") else [" ".join(s.split())]:
                if "sentence_rival" in conf and share(words(s.lower()), conf["sentence_rival"]) > 0.15:
                    continue
                if "sentence_never" in conf and not conf["sentence_never"].isdisjoint(words(s.lower())):
                    continue
                if len(s) <= 1000 and long_enough(s, conf) and not MARKUP.search(s) and in_script(s, script):
                    yield s


def script_runs(s, script):
    """`s` cut where a word has letters of another script, those left out."""
    runs, run = [], []
    for w in s.split():
        if all(unicodedata.name(c, "").startswith(script) for c in w if c.isalpha()):
            run.append(w)
        elif run:
            runs.append(" ".join(run))
            run = []
    if run:
        runs.append(" ".join(run))
    return runs


def documents(paths, every=1):
    for path in paths:
        if path.endswith(".jsonl.gz"):
            # MADLAD-400's: a JSON object a line, its text in "text".
            with gzip.open(path, "rt", encoding="utf-8") as f:
                for line in f:
                    if line.strip():
                        # Its line breaks are written as the two
                        # characters \ and n.
                        yield json.loads(line)["text"].replace("\\n", "\n")
            continue
        import pyarrow.parquet as pq

        f = pq.ParquetFile(path)
        # FineWeb-2's text is in "text", GlotCC's in "content".
        column = "text" if "text" in f.schema_arrow.names else "content"
        for g in range(0, f.metadata.num_row_groups, every):
            yield from f.read_row_group(g, columns=[column]).column(column).to_pylist()


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--lang", required=True, choices=sorted(LANGS))
    ap.add_argument("--eval", type=int, default=500, help="held-out sentences written at most")
    ap.add_argument("out")
    ap.add_argument("eval_out")
    ap.add_argument("--train", nargs="+", required=True, help="the train split's files")
    ap.add_argument("--test", nargs="+", help="the test split's files")
    ap.add_argument("--hold", type=int, default=20,
                    help="with no test split, every HOLDth document kept is held out")
    ap.add_argument("--exclude", nargs="+", default=[],
                    help="another source's sentences of the language (its corpus and held-out "
                         "text): neither written nor held out")
    opts = ap.parse_args()
    train, test = opts.train, opts.test or []
    conf = LANGS[opts.lang]
    seen = set()
    for path in opts.exclude:
        with open(path, encoding="utf-8") as f:
            seen.update(line.rstrip("\n") for line in f)
    excluded = len(seen)
    docs = dropped = 0
    held_docs = []
    with open(opts.out, "w", encoding="utf-8") as out:
        for text in documents(train, conf.get("every", 1)):
            docs += 1
            if not kept(text, conf):
                dropped += 1
                continue
            if not test and (docs - dropped) % opts.hold == 0:
                held_docs.append(text)
                continue
            for s in sentences(text, conf):
                if s not in seen:
                    seen.add(s)
                    out.write(s + "\n")
    held = {}
    for text in held_docs or (t for t in documents(test) if kept(t, conf)):
        for s in sentences(text, conf):
            if s not in seen:
                held.setdefault(s, None)
    held = list(held)[: opts.eval]
    with open(opts.eval_out, "w", encoding="utf-8") as ev:
        for s in held:
            ev.write(s + "\n")
    if len(seen) == excluded or not held:
        sys.exit(f"fineweb-text: {opts.lang}: no text")
    print(f"{opts.out}: {len(seen) - excluded} sentences from {docs - dropped} of {docs} documents, {len(held)} held out")


if __name__ == "__main__":
    main()
