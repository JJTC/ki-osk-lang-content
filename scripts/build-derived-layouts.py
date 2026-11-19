#!/usr/bin/env python3
"""build-derived-layouts.py — the layouts that are another language's
keys, renamed, for the languages that write with them.

Bhojpuri, Maithili, Sanskrit, Awadhi, Bodo, Dogri, Chhattisgarhi, Magahi
and Angika are written in Devanagari as Hindi is, and typed on Hindi's
InScript keys (xkeyboard-config's `in`), Dotyali and Newari on Nepali's
(`np`): their layouts are Hindi's,
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
â ê î ô û and ṭ, Khasi's ï and ñ; Nigerian Pidgin, which the web writes
in the 26 letters, on them as they are. Nineteen languages of Southeast
Asia are typed on them too, some with their letters first on the long
presses: Hakha Chin's ṭ; Hiligaynon's, Iloko's, Central Bikol's,
Pangasinan's, Kapampangan's, Kinaray-a's and Tetum's ñ, which their
Spanish and Portuguese words write; Acehnese's vowels é ë è ô ö;
Balinese's é; and Hmong Daw, Jingpho, Iban, Batak Toba, Karo, Simalungun,
Banjar, Kadazandusun and Iu Mien as they are. Twenty-six of Europe's
regional languages are typed on their national language's keys, their
own letters first on the long presses: Nynorsk on Norwegian's;
Asturian, Aragonese and Ladino on Spanish's; Alemannic (è é à ì ò ù),
Bavarian (å), Low German, Palatine German, Kölsch and the two Sorbians
(Upper's ć č ě ł ń ó ř š ž ź, Lower's ć č ě ł ń ó ŕ ś š ž ź) on
German's; Limburgish on Dutch's (è ë ö ó first); Neapolitan, Sicilian
(â ê î ô û after the graves), Sardinian, Friulian (â ê î ô û ç),
Piedmontese (ë, ò ö, ù ü) and Ligurian (â æ ç…) on Italian's; Walloon
on French's (å); Latgalian on Latvian's (ō); Võro on Estonian's (š ž);
Kashubian (ë ò ô ã ù) and Silesian (ō ŏ ô ů ã) on Polish's; Crimean
Tatar on Turkish's (ñ â); Scots on the UK's; and Rusyn on Ukrainian's
(ы on и, ъ on ь, ё on е). Russia's languages written in Cyrillic are
typed on Russian's keys, their letters on the long presses of the
letters xkeyboard-config's keymaps put them over (Chuvash's ӑ ӗ ҫ ӳ on а
е с у, Sakha's ҕ ҥ ө һ ү, the Komis' і ӧ, the Maris' ӓ ӧ ӱ and ҥ or ӹ,
Tuvan's, Buryat's and Kalmyk's ң ө ү һ ә җ), Erzya and Moksha on them as
they are; Karelian and Livvi on Finnish's keys, Veps on Estonian's, each
with č š ž. Kumyk and Karachay-Balkar are typed on Russian's keys as they
are, Abkhaz on them with its letters on the long presses of those they
are made from (ә on а, ԥ on п, ҭ on т, ҟ қ on к…); South Azerbaijani and
Mazandarani on Persian's keys, Azerbaijani's ۆ ۇ on و and ې on ی;
Karakalpak on the US keys with á ǵ ı ń ó ú, the two Zazakis on Turkish's
with ê î û, Talysh on Azerbaijani's, and Mingrelian on Georgian's with ჷ
on ე, ჲ on ი, ჸ on ყ. Twenty-four of Africa's are typed on the US keys,
their letters first on the long presses: Efik's and Isoko's ọ ẹ ụ ị,
Sango's ë â ê ô û ï, Kiluba's ā ū ō ē ñ, Cape Verdean's á ã é ê ó ç,
Mossi's ã ẽ ɛ ɩ ʋ õ, Réunion Creole's é è, Tiv's ô é, Fon's ɔ ɛ ɖ and
tones, Acholi's ŋ, Baoulé's and Jula's ɛ ɔ ɲ, Kikuyu's ĩ ũ, Umbundu's
nasal vowels and ñ; Seychellois Creole, Northern Ndebele, Ndonga,
Kwanyama, Tshiluba, Kikongo, Wolaytta, Dholuo, Bemba and Kimbundu as
they are. The Americas' and the Pacific's, and Latin and Interlingua,
are typed on the US keys too: Papiamentu's and Kichwa's ñ,
Guadeloupean Creole's é è ò, Tuvaluan's macrons and Chamorro's å ñ first on
the long presses; and Krio, its ɛ ɔ ŋ first on them. Twenty of Africa's
long tail and four of Southeast Asia's are typed on them too: Kabiye's ɛ
ɩ ɔ ʊ ɖ ɣ ŋ ñ, Dagbani's ɛ ɔ ŋ ɣ ʒ, Dangme's, Farefare's and Moba's ɛ ɔ
(ŋ), Kamba's ĩ ũ, Bulu's é ô ñ, Edo's ẹ ọ, Teso's ŋ, Yao's ŵ, Dinka's ä ë ï
ö ɛ ɔ ɛ̈ ɔ̈ ŋ ɣ, Nias' ö ŵ and Madurese's â è é ḍ ṭ first on the long
presses, the others as they are. Eleven more of Europe's regional
languages are typed on their national language's keys, their own
letters first on the long presses: Cornish (à â ê ë è î ò ô ù û ü ÿ) and
Manx (ç á é í ó ú) on the UK's; Extremaduran on Spanish's (ç); Picard on
French's (ò); Samogitian on Lithuanian's (ā ē ī ō); Pontic Greek on
Greek's; Venetian (ł), Lombard (ò ö ó, ü ù) and Ladin (ë é è ê, ö ò ó ô,
ü ù, ć, à á, ś) on Italian's; West Flemish on Dutch's (ê è ë é, ô); and
Gagauz on Turkish's (ä ê ţ). Pa'O is typed on Burmese's keys, its
tones ꩻ and ႏ (an eighth of its letters) on keys of their own in place
of ဩ and ၏, which it hardly writes. Zhuang is typed on the US keys as they
are; Southern Min (Pe̍h-ōe-jī) and Hakka (Pha̍k-fa-sṳ) on them with their
toned vowels first on the long presses, the commonest first: Southern
Min's á â ā à a̍…, o's ó ō o̍ ò ô and o͘ in its tones, ⁿ first on n;
Hakka's â à á a̍…, and ṳ in its tones on u. Fifty-one of the long tail
(Zande, Alur, Akoose… Wayuu, Sasak… Maasai, Dagaare) are typed on the US
keys, their letters and tones first on the long presses, and those that
write an apostrophe inside their words (Alur's ng'wa, Gusii's ng'a) or ʼ
(Akoose's ʼɛ, South Bolivian Quechua's q'ala) with a key for it at the
home row's end, Wayuu with ü beside it; and so are ten more, Livonian on
Latvian's keys: Hunsrik, Plains and Swampy Cree, Ottawa, Tokelauan,
Kankanaey, Mamasa, Ambonese Malay and Aja, the letters a fifth to a third
of their words write with a key of its own (Plains Cree's ā ē, Swampy
Cree's â ê, Livonian's õ, Aja's ɔ ɛ), Ottawa's and Mamasa's apostrophe
too. Lahu writes a tone after most of its syllables (ˬ ˇ ꞈ ˍ ˆ ˉ: three
words in four end in one) and no q, x or z: ꞈ, its apostrophe and ˍ are
typed on those keys, ˆ and ˉ on the long presses of ꞈ and ˍ, and ˬ and ˇ,
the commonest, at the home row's end, where its desktop keyboard has them
(on ; and Shift+;). Thirteen more of the long tail: Nenets on Russian's keys, ӈ in place of
the ъ it hardly writes (ъ stays on ь's long press) and its two glottal
stops, which nearly half its words write (поʼ, ненэцяˮ), at the home
row's end: the apostrophe and ˮ; Dungan on them too, ә in place of ъ, җ
at the home row's end and ү ў first on у; Tausug, Ewondo, Tooro,
Makhuwa-Meetto, Kanuri, Giryama, Lango, Hanunoo, Toraja-Sa'dan and Ledo
Kaili on the US keys and Sassarese on Italian's, their letters first on
the long presses, with an apostrophe key for those whose words write one
(Tausug's and Toraja-Sa'dan's glottal stop, Giryama's at'u) and Kanuri's
ə, in a quarter of its words, at the home row's end. Vlax Romani is
typed on the US keys, its č ć š ž đ first on the long presses of c s z
d, and Balkan Romani on Serbian's Latin keys, which have them all; Ge'ez
on Amharic's, its letters composed from Latin ones as Amharic's are,
with no model; and so are Gedeo, Hadiyya, Kambaata and Silt'e, whose
letters CLDR's exemplar characters list in the Ethiopic script and
Amharic's composer now types (its ejectives ጠ ጨ ጰ as T C P, ዸ as D, the
-wa forms, ኧ), named in Latin letters, as they also write their names. Kabiye, Akoose and Grebo, whose long presses typed more
than a quarter of their letters, and Nande, whose long presses were in a
quarter of its words, have keys for the letters their words write most
of those: two at the home row's end (Kabiye's ɛ ɩ, Akoose's é á, Grebo's
ɛ ɔ, Nande's ĩ beside its apostrophe), and in place of q and x, which
they hardly write (q and x on those keys' long presses), Kabiye's ʊ ɔ,
Akoose's ɛ and glottal stop (which moves there from the home row's end),
Grebo's ǒ ě and Nande's ũ. Montenegrin is typed on Serbian's Latin keys
with ś ź first on the long presses of s z, and on its Cyrillic ones,
with no model, with с́ з́ on с з. Twelve of the Incubator's test wikis'
languages: Rinconada Bikol (ñ), the Malays of Jambi, Makassar and Manado
as the US keys are, Palembang's Musi (é è ó), Kelantan-Pattani Malay
(è é ó ũ) and Ibibio (ọ ñ n̄ ị ẹ ụ ʌ) with their letters first on the
long presses, Makassarese and Pamona with an apostrophe key for their
glottal stop (se're, ince'emo: a seventh of Pamona's words); East
Franconian on German's keys with â ô; Southern Kurdish on Sorani's, ۊ on
و; and Western Balochi on Persian's, with Urdu's ے ں ٹ ڈ ڑ ہ ھ and
Pashto's ګ on the long presses of ی ن ت د ر ه گ. Rangpuri is typed on
Assamese's keys, Bengali's digits on the symbol view's, and Abron on
Akan's; Eastern Min (Bàng-uâ-cê) on the US keys with its toned vowels
first on the long presses, the commonest first, as Southern Min's (á â ă
ā à, then those with the diaeresis below: ā̤ â̤…), Fiji Hindi on them as
they are, and Emilian on Italian's, its dialects' letters first on the
long presses (ś ṡ ş ṣ, ṅ, ō ē ū, ć ṿ ł ǵ ż). With no model: Amdo and
Khams Tibetan on Tibetan's keys, Sadri on Hindi's, Maharashtrian Konkani
on Marathi's, Northern Thai in the Thai script on Thai's, Kutchi on
Gujarati's, Tày on Vietnamese's with the macron below and the double
grave first on each vowel's long press, and Eastern Pwo Karen on
Burmese's, ႋ ၜ ၠ in place of ဩ ၏ ၒ, its letters on long presses; and
with the letters their test wikis write, Northern Min (Kienning) on the
US keys with its toned vowels, the double overline its seventh tone,
Siberian Tatar on Bashkir's with ё on е, Isan on Thai's, Maguindanao,
Capiznon, Buginese and Rejang (ê first on e) on the US keys, Kedah Malay
on Malay's, Qashqai on South Azerbaijani's with ێ on ی, Ghomala' on the
US keys with ə ɛ ɔ ʉ ŋ ƴ and its tones on the long presses, Laki on
Sorani's, and Southern Balochi on Balochi's, ۏ ݔ ګ on و ی گ. A key at
the home row's end types its
capital on the Shift view (Ā, Ɔ, Җ), and has the long press its letter is
given (Ngambay's ə, ə́ on its). A letter only a long press types turns the
gesture on for the layout (`long_press_letters`). A layout said to have no
model (build-exemplar-layouts.py's, which build them through this) has
neither suggestions nor glides. A layout's id is
its language's tag in lower case (sd-deva), its prediction language the
tag.

    scripts/build-derived-layouts.py [--check]

--check fails if a written file would differ, as a test of the tree.
"""
import json
import pathlib
import sys
import unicodedata

ROOT = pathlib.Path(__file__).resolve().parent.parent

def with_capitals(extra):
    """`extra` and its capitals': a's á ā… and A's Á Ā…."""
    return {**extra, **{k.upper(): [unicodedata.normalize("NFC", t.upper()) for t in v] for k, v in extra.items()}}


# The layout made, the one it is made from, its name, and what its keys
# gain on a long press (a key's label → the letters, first in its menu),
# on every view; and, for some, keys that type another letter instead
# ("swap": a key's label → the letter), or keys at the home row's end
# in place of its spacers ("home": what one types, the apostrophe many of
# Africa's languages write inside their words, or a list: Wayuu's ü and
# its apostrophe), and "model": False for a language with none.
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
    ("anp", "hi", "अंगिका", {}),
    ("dty", "ne", "डोटेली", {}),
    ("new", "ne", "नेपाल भाषा", {}),
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
    ("pcm", "us", "Naijá", {}),
    ("mww", "us", "Hmoob Dawb", {}),
    ("cnh", "us", "Laiholh", {"t": ["ṭ"], "T": ["Ṭ"]}),
    ("kac", "us", "Jinghpaw", {}),
    ("hil", "us", "Hiligaynon", {"n": ["ñ"], "N": ["Ñ"]}),
    ("ilo", "us", "Ilokano", {"n": ["ñ"], "N": ["Ñ"]}),
    ("bcl", "us", "Bikol Sentral", {"n": ["ñ"], "N": ["Ñ"]}),
    ("pag", "us", "Pangasinan", {"n": ["ñ"], "N": ["Ñ"]}),
    ("pam", "us", "Kapampangan", {"n": ["ñ"], "N": ["Ñ"]}),
    ("krj", "us", "Kinaray-a", {"n": ["ñ"], "N": ["Ñ"]}),
    ("iba", "us", "Jaku Iban", {}),
    ("bbc", "us", "Hata Batak Toba", {}),
    ("btx", "us", "Cakap Karo", {}),
    ("bts", "us", "Sahap Simalungun", {}),
    ("ace", "us", "Bahsa Acèh", {"e": ["é", "ë", "è"], "o": ["ô", "ö"], "E": ["É", "Ë", "È"], "O": ["Ô", "Ö"]}),
    ("ban", "us", "Basa Bali", {"e": ["é"], "E": ["É"]}),
    ("bjn", "us", "Bahasa Banjar", {}),
    ("dtp", "us", "Kadazandusun", {}),
    ("tet", "us", "Tetun", {"n": ["ñ"], "N": ["Ñ"]}),
    ("ium", "us", "Iu Mienh", {}),
    ("nn", "no", "Norsk nynorsk", {}),
    ("ast", "es", "Asturianu", {}),
    ("an", "es", "Aragonés", {}),
    ("lad", "es", "Ladino", {}),
    ("gsw", "de", "Alemannisch", {"e": ["è", "é"], "a": ["à"], "i": ["ì"], "o": ["ò"], "u": ["ù"], "E": ["È", "É"], "A": ["À"], "I": ["Ì"], "O": ["Ò"], "U": ["Ù"]}),
    ("bar", "de", "Boarisch", {"a": ["å"], "A": ["Å"]}),
    ("nds", "de", "Plattdüütsch", {}),
    ("pfl", "de", "Pälzisch", {}),
    ("ksh", "de", "Kölsch", {}),
    ("li", "nl", "Limburgs", {"e": ["è", "ë", "é", "ê"], "o": ["ö", "ó", "ô", "ò"], "E": ["È", "Ë", "É", "Ê"], "O": ["Ö", "Ó", "Ô", "Ò"]}),
    ("hsb", "de", "Hornjoserbsce", {"c": ["ć", "č"], "e": ["ě"], "l": ["ł"], "n": ["ń"], "o": ["ó"], "r": ["ř"], "s": ["š"], "z": ["ž", "ź"], "C": ["Ć", "Č"], "E": ["Ě"], "L": ["Ł"], "N": ["Ń"], "O": ["Ó"], "R": ["Ř"], "S": ["Š"], "Z": ["Ž", "Ź"]}),
    ("dsb", "de", "Dolnoserbski", {"c": ["ć", "č"], "e": ["ě"], "l": ["ł"], "n": ["ń"], "o": ["ó"], "r": ["ŕ"], "s": ["ś", "š"], "z": ["ž", "ź"], "C": ["Ć", "Č"], "E": ["Ě"], "L": ["Ł"], "N": ["Ń"], "O": ["Ó"], "R": ["Ŕ"], "S": ["Ś", "Š"], "Z": ["Ž", "Ź"]}),
    ("nap", "it", "Napulitano", {}),
    ("scn", "it", "Sicilianu", {"a": ["à", "â"], "e": ["è", "ê"], "i": ["ì", "î"], "o": ["ò", "ô"], "u": ["ù", "û"], "A": ["À", "Â"], "E": ["È", "Ê"], "I": ["Ì", "Î"], "O": ["Ò", "Ô"], "U": ["Ù", "Û"]}),
    ("sc", "it", "Sardu", {}),
    ("fur", "it", "Furlan", {"a": ["â", "à"], "e": ["ê", "è"], "i": ["î", "ì"], "o": ["ô", "ò"], "u": ["û", "ù"], "c": ["ç"], "A": ["Â", "À"], "E": ["Ê", "È"], "I": ["Î", "Ì"], "O": ["Ô", "Ò"], "U": ["Û", "Ù"], "C": ["Ç"]}),
    ("pms", "it", "Piemontèis", {"e": ["ë", "é", "è", "ė"], "o": ["ò", "ö"], "u": ["ù", "ü"], "E": ["Ë", "É", "È", "Ė"], "O": ["Ò", "Ö"], "U": ["Ù", "Ü"]}),
    ("lij", "it", "Ligure", {"a": ["â", "à", "æ"], "e": ["é", "è", "ê"], "c": ["ç"], "o": ["ò", "ô", "ö"], "u": ["ù", "û", "ü"], "A": ["Â", "À", "Æ"], "E": ["É", "È", "Ê"], "C": ["Ç"], "O": ["Ò", "Ô", "Ö"], "U": ["Ù", "Û", "Ü"]}),
    ("wa", "fr-azerty", "Walon", {"a": ["å", "â", "à"], "A": ["Å", "Â", "À"]}),
    ("ltg", "lv", "Latgalīšu", {"o": ["ō"], "O": ["Ō"]}),
    ("vro", "et", "Võro", {"s": ["š"], "z": ["ž"], "S": ["Š"], "Z": ["Ž"]}),
    ("csb", "pl", "Kaszëbsczi", {"e": ["ë", "é", "ę"], "o": ["ò", "ô", "ó"], "a": ["ã", "ą"], "u": ["ù"], "E": ["Ë", "É", "Ę"], "O": ["Ò", "Ô", "Ó"], "A": ["Ã", "Ą"], "U": ["Ù"]}),
    ("szl", "pl", "Ślōnski", {"o": ["ō", "ŏ", "ô", "ó"], "u": ["ů"], "a": ["ã"], "O": ["Ō", "Ŏ", "Ô", "Ó"], "U": ["Ů"], "A": ["Ã"]}),
    ("crh", "tr", "Qırımtatarca", {"n": ["ñ"], "a": ["â"], "N": ["Ñ"], "A": ["Â"]}),
    ("sco", "gb", "Scots", {}),
    ("rue", "uk", "Русиньскый", {"и": ["ы"], "ь": ["ъ"], "е": ["ё"], "И": ["Ы"], "Ь": ["Ъ"], "Е": ["Ё"]}),
    ("cv", "ru", "Чӑвашла", {"а": ["ӑ"], "е": ["ӗ"], "с": ["ҫ"], "у": ["ӳ"], "А": ["Ӑ"], "Е": ["Ӗ"], "С": ["Ҫ"], "У": ["Ӳ"]}),
    ("sah", "ru", "Саха тыла", {"г": ["ҕ"], "н": ["ҥ"], "о": ["ө"], "х": ["һ"], "у": ["ү"], "Г": ["Ҕ"], "Н": ["Ҥ"], "О": ["Ө"], "Х": ["Һ"], "У": ["Ү"]}),
    ("kpv", "ru", "Коми кыв", {"и": ["і"], "о": ["ӧ"], "И": ["І"], "О": ["Ӧ"]}),
    ("koi", "ru", "Перым коми кыв", {"и": ["і"], "о": ["ӧ"], "И": ["І"], "О": ["Ӧ"]}),
    ("mhr", "ru", "Олык марий", {"а": ["ӓ"], "н": ["ҥ"], "о": ["ӧ"], "у": ["ӱ"], "А": ["Ӓ"], "Н": ["Ҥ"], "О": ["Ӧ"], "У": ["Ӱ"]}),
    ("mrj", "ru", "Кырык мары", {"а": ["ӓ"], "о": ["ӧ"], "у": ["ӱ"], "ы": ["ӹ"], "А": ["Ӓ"], "О": ["Ӧ"], "У": ["Ӱ"], "Ы": ["Ӹ"]}),
    ("tyv", "ru", "Тыва дыл", {"н": ["ң"], "о": ["ө"], "у": ["ү"], "Н": ["Ң"], "О": ["Ө"], "У": ["Ү"]}),
    ("bxr", "ru", "Буряад хэлэн", {"о": ["ө"], "у": ["ү"], "х": ["һ"], "О": ["Ө"], "У": ["Ү"], "Х": ["Һ"]}),
    ("xal", "ru", "Хальмг келн", {"а": ["ә"], "ж": ["җ"], "н": ["ң"], "о": ["ө"], "у": ["ү"], "х": ["һ"], "А": ["Ә"], "Ж": ["Җ"], "Н": ["Ң"], "О": ["Ө"], "У": ["Ү"], "Х": ["Һ"]}),
    ("myv", "ru", "Эрзянь кель", {}),
    ("mdf", "ru", "Мокшень кяль", {}),
    ("krl", "fi", "Karjala", {"c": ["č"], "s": ["š"], "z": ["ž"], "C": ["Č"], "S": ["Š"], "Z": ["Ž"]}),
    ("olo", "fi", "Livvi", {"c": ["č"], "s": ["š"], "z": ["ž"], "C": ["Č"], "S": ["Š"], "Z": ["Ž"]}),
    ("vep", "et", "Vepsän kel’", {"c": ["č"], "s": ["š"], "z": ["ž"], "C": ["Č"], "S": ["Š"], "Z": ["Ž"]}),
    ("kum", "ru", "Къумукъ тил", {}),
    ("krc", "ru", "Къарачай-малкъар тил", {}),
    ("mzn", "fa", "مازرونی", {}),
    ("kaa", "us", "Qaraqalpaqsha", {"a": ["á"], "g": ["ǵ"], "i": ["ı"], "n": ["ń"], "o": ["ó"], "u": ["ú"], "A": ["Á"], "G": ["Ǵ"], "N": ["Ń"], "O": ["Ó"], "U": ["Ú"]}),
    ("kiu", "tr", "Kirmanckî", {"e": ["ê"], "i": ["î"], "u": ["û"], "E": ["Ê"], "I": ["Î"], "U": ["Û"]}),
    ("diq", "tr", "Zazakî", {"e": ["ê"], "i": ["î"], "u": ["û"], "E": ["Ê"], "I": ["Î"], "U": ["Û"]}),
    ("tly", "az", "Tolışə zıvon", {}),
    ("ab", "ru", "Аԥсшәа", {"а": ["ә"], "к": ["қ", "ҟ"], "п": ["ԥ"], "т": ["ҭ"], "х": ["ҳ"], "ц": ["ҵ"], "ч": ["ҿ", "ҽ", "ҷ"], "о": ["ҩ"], "з": ["ӡ"], "д": ["џ"], "г": ["ҕ", "ӷ"], "А": ["Ә"], "К": ["Қ", "Ҟ"], "П": ["Ԥ"], "Т": ["Ҭ"], "Х": ["Ҳ"], "Ц": ["Ҵ"], "Ч": ["Ҿ", "Ҽ", "Ҷ"], "О": ["Ҩ"], "З": ["Ӡ"], "Д": ["Џ"], "Г": ["Ҕ", "Ӷ"]}),
    ("azb", "fa", "تۆرکجه", {"و": ["ۆ", "ۇ"], "ی": ["ې"]}),
    ("xmf", "ka", "მარგალური", {"ე": ["ჷ"], "ი": ["ჲ"], "ყ": ["ჸ"]}),
    ("crs", "us", "Kreol Seselwa", {}),
    ("nd", "us", "Sindebele", {}),
    ("ng", "us", "Oshindonga", {}),
    ("efi", "us", "Efịk", {"o": ["ọ"], "e": ["ẹ"], "u": ["ụ"], "i": ["ị"], "O": ["Ọ"], "E": ["Ẹ"], "U": ["Ụ"], "I": ["Ị"]}),
    ("sg", "us", "Sängö", {"e": ["ë", "ê", "é"], "a": ["â", "ä"], "o": ["ô", "ö"], "u": ["û"], "i": ["ï", "î"], "E": ["Ë", "Ê", "É"], "A": ["Â", "Ä"], "O": ["Ô", "Ö"], "U": ["Û"], "I": ["Ï", "Î"]}),
    ("kj", "us", "Oshikwanyama", {}),
    ("lua", "us", "Tshiluba", {}),
    ("lu", "us", "Kiluba", {"a": ["ā"], "u": ["ū"], "o": ["ō"], "e": ["ē"], "n": ["ñ"], "A": ["Ā"], "U": ["Ū"], "O": ["Ō"], "E": ["Ē"], "N": ["Ñ"]}),
    ("kea", "us", "Kabuverdianu", {"a": ["á", "ã", "â", "à"], "e": ["é", "ê"], "o": ["ó", "ô"], "c": ["ç"], "A": ["Á", "Ã", "Â", "À"], "E": ["É", "Ê"], "O": ["Ó", "Ô"], "C": ["Ç"]}),
    ("mos", "us", "Mòoré", {"a": ["ã"], "e": ["ẽ", "ɛ"], "i": ["ɩ", "ĩ"], "u": ["ʋ", "ũ"], "o": ["õ"], "A": ["Ã"], "E": ["Ẽ", "Ɛ"], "I": ["Ɩ", "Ĩ"], "U": ["Ʋ", "Ũ"], "O": ["Õ"]}),
    ("rcf", "us", "Kréol rényoné", {"e": ["é", "è"], "E": ["É", "È"]}),
    ("kg", "us", "Kikongo", {}),
    ("wal", "us", "Wolayttatto", {}),
    ("luo", "us", "Dholuo", {}),
    ("tiv", "us", "Tiv", {"o": ["ô"], "e": ["é"], "O": ["Ô"], "E": ["É"]}),
    ("iso", "us", "Isoko", {"o": ["ọ"], "e": ["ẹ"], "O": ["Ọ"], "E": ["Ẹ"]}),
    ("bem", "us", "Ichibemba", {}),
    ("fon", "us", "Fɔngbè", {"o": ["ɔ", "ó", "ò"], "e": ["ɛ", "é", "è", "ě"], "d": ["ɖ"], "i": ["í", "ǐ"], "u": ["ú", "ǔ"], "a": ["á", "ǎ"], "O": ["Ɔ", "Ó", "Ò"], "E": ["Ɛ", "É", "È", "Ě"], "D": ["Ɖ"], "I": ["Í", "Ǐ"], "U": ["Ú", "Ǔ"], "A": ["Á", "Ǎ"]}),
    ("ach", "us", "Acoli", {"n": ["ŋ"], "N": ["Ŋ"]}),
    ("bci", "us", "Wawle", {"e": ["ɛ"], "o": ["ɔ"], "n": ["ɲ"], "E": ["Ɛ"], "O": ["Ɔ"], "N": ["Ɲ"]}),
    ("dyu", "us", "Julakan", {"e": ["ɛ"], "o": ["ɔ"], "n": ["ɲ", "ŋ"], "E": ["Ɛ"], "O": ["Ɔ"], "N": ["Ɲ", "Ŋ"]}),
    ("ki", "us", "Gĩkũyũ", {"i": ["ĩ"], "u": ["ũ"], "I": ["Ĩ"], "U": ["Ũ"]}),
    ("kmb", "us", "Kimbundu", {}),
    ("umb", "us", "Umbundu", {"a": ["ã"], "e": ["ẽ"], "i": ["ĩ"], "u": ["ũ"], "o": ["õ"], "n": ["ñ"], "A": ["Ã"], "E": ["Ẽ"], "I": ["Ĩ"], "U": ["Ũ"], "O": ["Õ"], "N": ["Ñ"]}),
    ("la", "us", "Latina", {}),
    ("pap", "us", "Papiamentu", {"n": ["ñ"], "N": ["Ñ"]}),
    ("qug", "us", "Kichwa", {"n": ["ñ"], "N": ["Ñ"]}),
    ("srn", "us", "Sranantongo", {}),
    ("gcf", "us", "Kréyòl gwadloupéyen", {"e": ["é", "è"], "o": ["ò"], "E": ["É", "È"], "O": ["Ò"]}),
    ("bi", "us", "Bislama", {}),
    ("ia", "us", "Interlingua", {}),
    ("tpi", "us", "Tok Pisin", {}),
    ("gil", "us", "Taetae ni Kiribati", {}),
    ("tvl", "us", "Te Ggana Tuuvalu", {"a": ["ā"], "e": ["ē"], "i": ["ī"], "o": ["ō"], "u": ["ū"], "A": ["Ā"], "E": ["Ē"], "I": ["Ī"], "O": ["Ō"], "U": ["Ū"]}),
    ("ho", "us", "Hiri Motu", {}),
    ("ch", "us", "Finuʼ Chamoru", {"a": ["å"], "n": ["ñ"], "A": ["Å"], "N": ["Ñ"]}),
    ("jam", "us", "Patwa", {}),
    ("kri", "us", "Krio", {"e": ["ɛ"], "o": ["ɔ"], "n": ["ŋ"], "E": ["Ɛ"], "O": ["Ɔ"], "N": ["Ŋ"]}),
    ("kbp", "us", "Kabɩyɛ", {"e": ["ɛ"], "i": ["ɩ"], "o": ["ɔ"], "u": ["ʊ"], "d": ["ɖ"], "g": ["ɣ"], "n": ["ŋ", "ñ"], "E": ["Ɛ"], "I": ["Ɩ"], "O": ["Ɔ"], "U": ["Ʊ"], "D": ["Ɖ"], "G": ["Ɣ"], "N": ["Ŋ", "Ñ"],
                             "ʊ": ["q"], "Ʊ": ["Q"], "ɔ": ["x"], "Ɔ": ["X"]},
     {"swap": {"q": "ʊ", "Q": "Ʊ", "x": "ɔ", "X": "Ɔ"}, "home": ["ɛ", "ɩ"]}),
    ("dag", "us", "Dagbanli", {"e": ["ɛ"], "o": ["ɔ"], "n": ["ŋ"], "g": ["ɣ"], "z": ["ʒ"], "E": ["Ɛ"], "O": ["Ɔ"], "N": ["Ŋ"], "G": ["Ɣ"], "Z": ["Ʒ"]}),
    ("seh", "us", "Cisena", {}),
    ("ada", "us", "Dangme", {"e": ["ɛ"], "o": ["ɔ"], "E": ["Ɛ"], "O": ["Ɔ"]}),
    ("nyn", "us", "Runyankore", {}),
    ("kwy", "us", "Kisikongo", {}),
    ("sid", "us", "Sidaamu Afoo", {}),
    ("koo", "us", "Olukonzo", {}),
    ("kam", "us", "Kĩkamba", {"i": ["ĩ"], "u": ["ũ"], "I": ["Ĩ"], "U": ["Ũ"]}),
    ("sop", "us", "Kisonge", {}),
    ("vmw", "us", "Emakhuwa", {}),
    ("ndc", "us", "ChiNdau", {}),
    ("gur", "us", "Gurenɛ", {"e": ["ɛ"], "o": ["ɔ"], "n": ["ŋ"], "E": ["Ɛ"], "O": ["Ɔ"], "N": ["Ŋ"]}),
    ("bum", "us", "Bulu", {"e": ["é"], "o": ["ô"], "n": ["ñ"], "E": ["É"], "O": ["Ô"], "N": ["Ñ"]}),
    ("bin", "us", "Ẹdo", {"e": ["ẹ"], "o": ["ọ"], "E": ["Ẹ"], "O": ["Ọ"]}),
    ("teo", "us", "Ateso", {"n": ["ŋ"], "N": ["Ŋ"]}),
    ("sgc", "us", "Kipsigis", {}),
    ("mfq", "us", "Moba", {"e": ["ɛ"], "o": ["ɔ"], "n": ["ŋ"], "E": ["Ɛ"], "O": ["Ɔ"], "N": ["Ŋ"]}),
    ("yao", "us", "Chiyao", {"w": ["ŵ"], "W": ["Ŵ"]}),
    ("dik", "us", "Thuɔŋjäŋ", {"a": ["ä"], "e": ["ë", "ɛ", "ɛ̈"], "i": ["ï"], "o": ["ö", "ɔ", "ɔ̈"], "n": ["ŋ"], "g": ["ɣ"], "A": ["Ä"], "E": ["Ë", "Ɛ", "Ɛ̈"], "I": ["Ï"], "O": ["Ö", "Ɔ", "Ɔ̈"], "N": ["Ŋ"], "G": ["Ɣ"]}),
    ("min", "us", "Baso Minang", {}),
    ("war", "us", "Winaray", {}),
    ("nia", "us", "Li Niha", {"o": ["ö"], "w": ["ŵ"], "O": ["Ö"], "W": ["Ŵ"]}),
    ("mad", "us", "Basa Madhura", {"a": ["â"], "e": ["è", "é"], "d": ["ḍ"], "t": ["ṭ"], "A": ["Â"], "E": ["È", "É"], "D": ["Ḍ"], "T": ["Ṭ"]}),    ("cor", "gb", "Kernewek", {"a": ["à", "â"], "e": ["ê", "ë", "è"], "i": ["î"], "o": ["ò", "ô"], "u": ["ù", "û", "ü"], "y": ["ÿ"], "A": ["À", "Â"], "E": ["Ê", "Ë", "È"], "I": ["Î"], "O": ["Ò", "Ô"], "U": ["Ù", "Û", "Ü"], "Y": ["Ÿ"]}),
    ("glv", "gb", "Gaelg", {"c": ["ç"], "a": ["á"], "e": ["é"], "i": ["í"], "o": ["ó"], "u": ["ú"], "C": ["Ç"], "A": ["Á"], "E": ["É"], "I": ["Í"], "O": ["Ó"], "U": ["Ú"]}),
    ("ext", "es", "Estremeñu", {"c": ["ç"], "C": ["Ç"]}),
    ("pcd", "fr-azerty", "Picard", {"o": ["ò"], "O": ["Ò"]}),
    ("sgs", "lt", "Žemaitėška", {"a": ["ā"], "e": ["ē"], "i": ["ī"], "o": ["ō"], "A": ["Ā"], "E": ["Ē"], "I": ["Ī"], "O": ["Ō"]}),
    ("pnt", "el", "Ποντιακά", {}),
    ("vec", "it", "Vèneto", {"l": ["ł"], "L": ["Ł"]}),
    ("vls", "nl", "West-Vlams", {"e": ["ê", "è", "ë", "é"], "o": ["ô"], "E": ["Ê", "È", "Ë", "É"], "O": ["Ô"]}),
    ("lmo", "it", "Lumbard", {"o": ["ò", "ö", "ó"], "u": ["ü", "ù"], "O": ["Ò", "Ö", "Ó"], "U": ["Ü", "Ù"]}),
    ("lld", "it", "Ladin", {"e": ["ë", "é", "è", "ê"], "o": ["ö", "ò", "ó", "ô"], "u": ["ü", "ù"], "c": ["ć"], "a": ["à", "á"], "s": ["ś"], "E": ["Ë", "É", "È", "Ê"], "O": ["Ö", "Ò", "Ó", "Ô"], "U": ["Ü", "Ù"], "C": ["Ć"], "A": ["À", "Á"], "S": ["Ś"]}),
    ("gag", "tr", "Gagauzça", {"a": ["ä"], "e": ["ê"], "t": ["ţ"], "A": ["Ä"], "E": ["Ê"], "T": ["Ţ"]}),
    ("blk", "my", "ပအိုဝ်ႏ", {}, {"swap": {"ဩ": "ꩻ", "၏": "ႏ"}}),
    ("za", "us", "Vahcuengh", {}),
    ("nan", "us", "Bân-lâm-gú", {"a": ["á", "â", "ā", "à", "a̍"], "A": ["Á", "Â", "Ā", "À", "A̍"], "e": ["ê", "ē", "è", "é", "e̍"], "E": ["Ê", "Ē", "È", "É", "E̍"], "i": ["ī", "í", "i̍", "ì", "î"], "I": ["Ī", "Í", "I̍", "Ì", "Î"], "o": ["ó", "ō", "o̍", "ò", "ô", "ō͘", "ó͘", "o͘", "ò͘", "ô͘"], "O": ["Ó", "Ō", "O̍", "Ò", "Ô", "Ō͘", "Ó͘", "O͘", "Ò͘", "Ô͘"], "u": ["ū", "ú", "ù", "û", "u̍"], "U": ["Ū", "Ú", "Ù", "Û", "U̍"], "m": ["m̄"], "M": ["M̄"], "n": ["ⁿ", "n̄", "n̂", "ǹ", "ń"], "N": ["N̄", "N̂", "Ǹ", "Ń"]}),
    ("hak", "us", "Hak-kâ-ngî", {"a": ["â", "à", "á", "a̍"], "A": ["Â", "À", "Á", "A̍"], "e": ["ê", "è", "é", "e̍"], "E": ["Ê", "È", "É", "E̍"], "i": ["î", "ì", "í", "i̍"], "I": ["Î", "Ì", "Í", "I̍"], "o": ["ô", "ò", "ó", "o̍"], "O": ["Ô", "Ò", "Ó", "O̍"], "u": ["û", "ú", "ù", "ṳ", "u̍", "ṳ̂", "ṳ́", "ṳ̀", "ṳ̍"], "U": ["Û", "Ú", "Ù", "Ṳ", "U̍", "Ṳ̂", "Ṳ́", "Ṳ̀", "Ṳ̍"], "m": ["m̀"], "M": ["M̀"], "n": ["ń", "ǹ"], "N": ["Ń", "Ǹ"]}),
    ("zne", "us", "Pazande", {"a": ["ã", "á"], "A": ["Ã", "Á"], "e": ["ẽ"], "E": ["Ẽ"], "i": ["ĩ", "í"], "I": ["Ĩ", "Í"], "o": ["õ"], "O": ["Õ"], "u": ["ũ", "ú"], "U": ["Ũ", "Ú"]}),
    ("alz", "us", "Alur", {}, {"home": "'"}),
    ("cce", "us", "Chopi", {"n": ["ñ"], "N": ["Ñ"]}),
    ("skg", "us", "Sakalava", {"a": ["à"], "A": ["À"], "e": ["è", "é"], "E": ["È", "É"], "i": ["ì"], "I": ["Ì"], "o": ["ò"], "O": ["Ò"]}),
    ("bss", "us", "Akoose", {"a": ["á", "â", "ǎ", "ā"], "A": ["Á", "Â", "Ǎ", "Ā"], "e": ["é", "ɛ", "ɛ́", "ē", "ê", "ə", "ə́", "ě", "ə̄", "ɛ̄", "ɛ̂", "ɛ̌", "ə̌", "ə̂", "è"], "E": ["É", "Ɛ", "Ɛ́", "Ē", "Ê", "Ə", "Ə́", "Ě", "Ə̄", "Ɛ̄", "Ɛ̂", "Ɛ̌", "Ə̌", "Ə̂", "È"], "i": ["í", "ī", "ǐ", "î"], "I": ["Í", "Ī", "Ǐ", "Î"], "m": ["ḿ"], "M": ["Ḿ"], "n": ["ŋ", "ń"], "N": ["Ŋ", "Ń"], "o": ["ɔ́", "ó", "ɔ", "ǒ", "ō", "ɔ̄", "ô", "ɔ̂", "ɔ̌"], "O": ["Ɔ́", "Ó", "Ɔ", "Ǒ", "Ō", "Ɔ̄", "Ô", "Ɔ̂", "Ɔ̌"], "u": ["ú", "ū", "ǔ", "û"], "U": ["Ú", "Ū", "Ǔ", "Û"],
                            "ɛ": ["q"], "Ɛ": ["Q"], "ʼ": ["x", "X"]},
     {"swap": {"q": "ɛ", "Q": "Ɛ", "x": "ʼ", "X": "ʼ"}, "home": ["é", "á"]}),
    ("men", "us", "Mɛnde", {"e": ["ɛ"], "E": ["Ɛ"], "o": ["ɔ"], "O": ["Ɔ"], "n": ["ŋ"], "N": ["Ŋ"]}),
    ("lgg", "us", "Lugbarati", {"u": ["ᴜ", "ᴜ́", "ú"], "U": ["Ú"], "i": ["ɨ", "ɨ́", "ɨ̀", "í", "ì"], "I": ["Ɨ", "Ɨ́", "Ɨ̀", "Í", "Ì"], "a": ["á", "à"], "A": ["Á", "À"], "e": ["è", "é"], "E": ["È", "É"], "o": ["ó", "ò"], "O": ["Ó", "Ò"]}, {"home": "'"}),
    ("xon", "us", "Konkomba", {"o": ["ɔ"], "O": ["Ɔ"], "n": ["ŋ"], "N": ["Ŋ"]}),
    ("dnj", "us", "Dan", {"e": ["ë", "ɛ"], "E": ["Ë", "Ɛ"], "o": ["ö", "ɔ"], "O": ["Ö", "Ɔ"], "u": ["ü"], "U": ["Ü"], "v": ["ʋ̈", "ʋ"], "V": ["Ʋ̈", "Ʋ"], "i": ["ɩ"], "I": ["Ɩ"]}),
    ("gbo", "us", "Grebo", {"a": ["à", "ǎ", "ä"], "A": ["À", "Ǎ", "Ä"], "e": ["ɛ", "ě", "è", "ɛ̌", "ɛ̀", "ɛ̈"], "E": ["Ɛ", "Ě", "È", "Ɛ̌", "Ɛ̀", "Ɛ̈"], "i": ["ǐ", "ì", "ï"], "I": ["Ǐ", "Ì", "Ï"], "o": ["ɔ", "ǒ", "ò", "ɔ̀", "ɔ̌", "ɔ̈"], "O": ["Ɔ", "Ǒ", "Ò", "Ɔ̀", "Ɔ̌", "Ɔ̈"], "u": ["ù", "ǔ", "ü"], "U": ["Ù", "Ǔ", "Ü"],
                           "ǒ": ["q"], "Ǒ": ["Q"], "ě": ["x"], "Ě": ["X"]},
     {"swap": {"q": "ǒ", "Q": "Ǒ", "x": "ě", "X": "Ě"}, "home": ["ɛ", "ɔ"]}),
    ("heh", "us", "Kihehe", {}),
    ("gux", "us", "Gulmancema", {"n": ["ŋ", "ñ"], "N": ["Ŋ", "Ñ"]}),
    ("gmv", "us", "Gamo", {}),
    ("suk", "us", "Kisukuma", {"i": ["ī"], "I": ["Ī"], "u": ["ū"], "U": ["Ū"]}, {"home": "'"}),
    ("spp", "us", "Supyire", {"a": ["à", "á", "â"], "A": ["À", "Á", "Â"], "e": ["ɛ", "è", "ɛ̀", "é", "ê", "ɛ̂", "ɛ́"], "E": ["Ɛ", "È", "Ɛ̀", "É", "Ê", "Ɛ̂", "Ɛ́"], "i": ["í", "ì", "î"], "I": ["Í", "Ì", "Î"], "n": ["ŋ", "ɲ"], "N": ["Ŋ", "Ɲ"], "o": ["ɔ", "ɔ̀", "ò", "ɔ́", "ó", "ɔ̂", "ô"], "O": ["Ɔ", "Ɔ̀", "Ò", "Ɔ́", "Ó", "Ɔ̂", "Ô"], "u": ["ù", "ú", "û"], "U": ["Ù", "Ú", "Û"]}, {"home": "'"}),
    ("bas", "us", "Basaa", {"e": ["é", "è"], "E": ["É", "È"], "n": ["ñ"], "N": ["Ñ"], "o": ["ô"], "O": ["Ô"], "a": ["à"], "A": ["À"]}),
    ("hay", "us", "Ekihaya", {}, {"home": "'"}),
    ("kbr", "us", "Kafa", {}, {"home": "'"}),
    ("ngl", "us", "Elomwe", {"a": ["á", "à", "ã"], "A": ["Á", "À", "Ã"], "e": ["é"], "E": ["É"], "o": ["ó", "ò"], "O": ["Ó", "Ò"]}),
    ("gog", "us", "Cigogo", {}),
    ("chw", "us", "Echuwabo", {"a": ["á", "ã"], "A": ["Á", "Ã"], "c": ["ç"], "C": ["Ç"], "e": ["é"], "E": ["É"], "i": ["í"], "I": ["Í"], "n": ["ñ"], "N": ["Ñ"], "o": ["ó"], "O": ["Ó"]}),
    ("fat", "us", "Mfantse", {"e": ["ɛ"], "E": ["Ɛ"], "o": ["ɔ", "õ"], "O": ["Ɔ", "Õ"], "a": ["ã"], "A": ["Ã"], "u": ["ũ"], "U": ["Ũ"]}),
    ("tsc", "us", "Xitshwa", {}),
    ("bud", "us", "Ncam", {"a": ["á", "à", "ā"], "A": ["Á", "À", "Ā"], "b": ["b́", "b̀"], "B": ["B́", "B̀"], "e": ["é", "è", "ē"], "E": ["É", "È", "Ē"], "i": ["í", "ì", "ī"], "I": ["Í", "Ì", "Ī"], "l": ["l̄", "l̀", "ĺ"], "L": ["L̄", "L̀", "Ĺ"], "m": ["ḿ", "m̀", "m̄"], "M": ["Ḿ", "M̀", "M̄"], "n": ["ŋ", "ǹ", "ń", "n̄"], "N": ["Ŋ", "Ǹ", "Ń", "N̄"], "o": ["ɔ", "ɔ́", "ò", "ɔ̃", "ɔ̄"], "O": ["Ɔ", "Ɔ́", "Ò", "Ɔ̃", "Ɔ̄"], "u": ["ú", "ū", "ù"], "U": ["Ú", "Ū", "Ù"]}),
    ("kao", "us", "Xaasongaxango", {"n": ["ɲ", "ŋ", "ń"], "N": ["Ɲ", "Ŋ", "Ń"], "i": ["í"], "I": ["Í"], "e": ["é"], "E": ["É"]}, {"home": "'"}),
    ("gvl", "us", "Gulay", {"e": ["è", "é", "ê"], "E": ["È", "É", "Ê"], "a": ["à", "á"], "A": ["À", "Á"], "b": ["ɓ"], "B": ["Ɓ"], "d": ["ɗ"], "D": ["Ɗ"], "m": ["m̂"], "M": ["M̂"], "o": ["ɔ", "ò", "ó"], "O": ["Ɔ", "Ò", "Ó"], "u": ["ú"], "U": ["Ú"]}, {"home": "'"}),
    ("nnb", "us", "Kinande", {"i": ["ĩ", "í", "į"], "I": ["Ĩ", "Í", "Į"], "u": ["ũ", "ú", "ų"], "U": ["Ũ", "Ú", "Ų"], "a": ["á"], "A": ["Á"], "e": ["é"], "E": ["É"], "o": ["ó"], "O": ["Ó"], "ũ": ["q"], "Ũ": ["Q"]},
     {"swap": {"q": "ũ", "Q": "Ũ"}, "home": ["ĩ", "'"]}),
    ("nyo", "us", "Runyoro", {}, {"home": "'"}),
    ("niu", "us", "Vagahau Niuē", {"a": ["ā"], "A": ["Ā"], "e": ["ē"], "E": ["Ē"], "o": ["ō"], "O": ["Ō"], "u": ["ū"], "U": ["Ū"]}),
    ("hwc", "us", "Pidgin", {}),
    ("acf", "us", "Kwéyòl", {"e": ["é", "è"], "E": ["É", "È"], "o": ["ò"], "O": ["Ò"]}),
    ("mrw", "us", "Meranaw", {"a": ["ā"], "A": ["Ā"], "u": ["ū"], "U": ["Ū"], "i": ["ī"], "I": ["Ī"]}),
    ("nij", "us", "Ngaju", {}),
    ("guc", "us", "Wayuunaiki", {"u": ["ü", "ú"], "U": ["Ü", "Ú"], "a": ["á"], "A": ["Á"], "e": ["é"], "E": ["É"], "i": ["í"], "I": ["Í"], "o": ["ó"], "O": ["Ó"], "n": ["ñ"], "N": ["Ñ"]}, {"home": ["ü", "'"]}),
    ("ljp", "us", "Lampung", {}),
    ("sas", "us", "Basa Sasak", {"e": ["ẽ"], "E": ["Ẽ"]}),
    ("gor", "us", "Hulontalo", {}, {"home": "'"}),
    ("itv", "us", "Itawit", {}),
    ("bku", "us", "Buhid", {}),
    ("bpr", "us", "Blaan", {"a": ["à"], "A": ["À"], "e": ["é", "è", "ê"], "E": ["É", "È", "Ê"], "i": ["ì"], "I": ["Ì"], "o": ["ò"], "O": ["Ò"], "u": ["ù"], "U": ["Ù"]}),
    ("quh", "us", "Qhichwa", {"n": ["ñ"], "N": ["Ñ"], "a": ["á"], "A": ["Á"], "e": ["é"], "E": ["É"], "i": ["í"], "I": ["Í"], "o": ["ó"], "O": ["Ó"], "u": ["ú"], "U": ["Ú"]}, {"home": "ʼ"}),
    ("gcr", "us", "Kriyòl", {"e": ["é", "è"], "E": ["É", "È"], "o": ["ò"], "O": ["Ò"], "a": ["à"], "A": ["À"], "i": ["ï"], "I": ["Ï"]}),
    ("tum", "us", "Chitumbuka", {"w": ["ŵ"], "W": ["Ŵ"]}),
    ("tll", "us", "Otetela", {"e": ["ɛ"], "E": ["Ɛ"], "o": ["ɔ"], "O": ["Ɔ"]}, {"home": "'"}),
    ("guz", "us", "Ekegusii", {}, {"home": "'"}),
    ("kde", "us", "Chimakonde", {"a": ["á", "à", "â"], "A": ["Á", "À", "Â"], "e": ["é", "ê", "è"], "E": ["É", "Ê", "È"], "i": ["í", "î", "ì"], "I": ["Í", "Î", "Ì"], "o": ["ô"], "O": ["Ô"], "u": ["ú", "ù"], "U": ["Ú", "Ù"]}),
    ("nyy", "us", "Ikinyakyusa", {"i": ["ị", "í", "ị̄", "ī"], "I": ["Ị", "Í", "Ị̄", "Ī"], "u": ["ụ", "ū", "ụ̄", "ú"], "U": ["Ụ", "Ū", "Ụ̄", "Ú"], "a": ["ā"], "A": ["Ā"], "e": ["ē"], "E": ["Ē"], "o": ["ō"], "O": ["Ō"]}),
    ("mas", "us", "Maa", {"a": ["á", "â"], "A": ["Á", "Â"], "e": ["é", "ê"], "E": ["É", "Ê"], "i": ["í"], "I": ["Í"], "o": ["ó", "ô"], "O": ["Ó", "Ô"], "u": ["ú"], "U": ["Ú"]}),
    ("mnk", "us", "Mandinka", {"n": ["ŋ", "ñ", "ǹ"], "N": ["Ŋ", "Ñ", "Ǹ"], "i": ["ì"], "I": ["Ì"], "a": ["à"], "A": ["À"], "e": ["é"], "E": ["É"], "m": ["m̀"], "M": ["M̀"]}),
    ("mer", "us", "Kĩmĩĩrũ", {"i": ["ĩ"], "I": ["Ĩ"], "u": ["ũ"], "U": ["Ũ"]}),
    ("dga", "us", "Dagaare", {"e": ["ɛ", "ē", "é", "ɛ̃"], "E": ["Ɛ", "Ē", "É", "Ɛ̃"], "n": ["ŋ"], "N": ["Ŋ"], "o": ["ɔ", "õ", "ɔ̃"], "O": ["Ɔ", "Õ", "Ɔ̃"], "a": ["ã"], "A": ["Ã"], "u": ["ŭ", "ū", "ũ"], "U": ["Ŭ", "Ū", "Ũ"]}),
    ("lhu", "us", "Laˇhuˍ", {"ꞈ": ["ˆ"], "ˍ": ["ˉ"]},
     {"swap": {"q": "ꞈ", "Q": "ꞈ", "x": "'", "X": "'", "z": "ˍ", "Z": "ˍ"}, "home": ["ˬ", "ˇ"]}),
    ("hrx", "us", "Hunsrik", {"e": ["ë"], "E": ["Ë"], "a": ["ã", "á"], "A": ["Ã", "Á"], "o": ["ó"], "O": ["Ó"], "c": ["ç"], "C": ["Ç"]}),
    ("crk", "us", "nēhiyawēwin", {"a": ["ā"], "A": ["Ā"], "e": ["ē"], "E": ["Ē"], "i": ["ī"], "I": ["Ī"], "o": ["ō"], "O": ["Ō"]},
     {"home": ["ā", "ē"]}),
    ("csw", "us", "Ininîmowin", {"a": ["â"], "A": ["Â"], "e": ["ê"], "E": ["Ê"], "i": ["î"], "I": ["Î"], "o": ["ô"], "O": ["Ô"],
                                "s": ["š"], "S": ["Š"]}, {"home": ["â", "ê"]}),
    ("otw", "us", "Nishnaabemwin", {}, {"home": "'"}),
    ("tkl", "us", "Gagana Tokelau", {"a": ["ā"], "A": ["Ā"], "e": ["ē"], "E": ["Ē"], "i": ["ī"], "I": ["Ī"], "o": ["ō"], "O": ["Ō"],
                                    "u": ["ū"], "U": ["Ū"]}),
    ("kne", "us", "Kankanaey", {}),
    ("mqj", "us", "Mamasa", {}, {"home": "'"}),
    ("liv", "lv", "Līvõ kēļ", {"o": ["õ", "ō", "ȯ", "ȭ", "ȱ"], "O": ["Õ", "Ō", "Ȯ", "Ȭ", "Ȱ"], "a": ["ā", "ä", "ǟ"], "A": ["Ā", "Ä", "Ǟ"],
                               "d": ["ḑ"], "D": ["Ḑ"], "t": ["ț"], "T": ["Ț"], "r": ["ŗ"], "R": ["Ŗ"]}, {"home": "õ"}),
    ("abs", "us", "Bahasa Ambon", {}),
    ("ajg", "us", "Ajagbe", {"e": ["ɛ", "é", "è"], "E": ["Ɛ", "É", "È"], "o": ["ɔ", "ó"], "O": ["Ɔ", "Ó"], "d": ["ɖ"], "D": ["Ɖ"],
                             "n": ["ŋ"], "N": ["Ŋ"], "i": ["í", "ì"], "I": ["Í", "Ì"], "z": ["ʒ"], "Z": ["Ʒ"], "a": ["à", "á"],
                             "A": ["À", "Á"]}, {"home": ["ɔ", "ɛ"]}),
    ("yrk", "ru", "Ненэцяˮ вада", {}, {"swap": {"ъ": "ӈ", "Ъ": "Ӈ"}, "home": ["'", "ˮ"]}),
    ("dng", "ru", "Хуэйзў йүян", {"у": ["ү", "ў"], "У": ["Ү", "Ў"]}, {"swap": {"ъ": "ә", "Ъ": "Ә"}, "home": "җ"}),
    ("tsg", "us", "Bahasa Sūg", {"i": ["ī"], "I": ["Ī"], "a": ["ā"], "A": ["Ā"], "u": ["ū"], "U": ["Ū"]}, {"home": "'"}),
    ("ewo", "us", "Ewondo", {"e": ["é", "ë", "è", "ə", "ê"], "E": ["É", "Ë", "È", "Ə", "Ê"], "a": ["á", "à"], "A": ["Á", "À"],
                             "o": ["ò", "ó", "ô", "ɔ"], "O": ["Ò", "Ó", "Ô", "Ɔ"], "i": ["í", "ï"], "I": ["Í", "Ï"],
                             "u": ["ú"], "U": ["Ú"], "n": ["ñ", "ŋ", "ṅ"], "N": ["Ñ", "Ŋ", "Ṅ"]}),
    ("ttj", "us", "Rutooro", {"n": ["ŋ"], "N": ["Ŋ"]}, {"home": "'"}),
    ("mgh", "us", "Emeetto", {}),
    ("knc", "us", "Kanuri", {"e": ["ə"], "E": ["Ə"]}, {"home": "ə"}),
    ("nyf", "us", "Kigiryama", {"v": ["v̱"], "V": ["V̱"]}, {"home": "'"}),
    ("laj", "us", "Lëblaŋo", {"n": ["ŋ"], "N": ["Ŋ"]}),
    ("hnn", "us", "Hanunuo", {"e": ["ê"], "E": ["Ê"]}),
    ("sda", "us", "Basa Toraya", {}, {"home": "'"}),
    ("lew", "us", "Kaili Ledo", {}),
    ("sdc", "it", "Sassaresu", {}, {"home": "'"}),
    ("rmy", "us", "Romani čhib (Vlax)", {"s": ["š"], "S": ["Š"], "c": ["č", "ć"], "C": ["Č", "Ć"], "z": ["ž"], "Z": ["Ž"],
                                         "d": ["đ"], "D": ["Đ"]}),
    ("rmn", "sr-latn", "Romani čhib (Balkan)", {}),
    ("gez", "am", "ግዕዝ", {}),
    ("drs", "am", "Gedeo", {}),
    ("hdy", "am", "Hadiyya", {}),
    ("ktb", "am", "Kambaata", {}),
    ("stv", "am", "Silt'e", {}),
    ("cnr", "sr-latn", "Crnogorski", {"s": ["ś"], "S": ["Ś"], "z": ["ź"], "Z": ["Ź"]}),
    ("cnr-Cyrl", "sr", "Црногорски", {"с": ["с́"], "С": ["С́"], "з": ["з́"], "З": ["З́"]}, {"model": False}),
    ("bto", "us", "Rinconada Bikol", {"n": ["ñ"], "N": ["Ñ"]}),
    ("jax", "us", "Bahaso Jambi", {}),
    ("mfp", "us", "Bahasa Melayu Makassar", {}),
    ("xmm", "us", "Bahasa Manado", {}),
    ("mui", "us", "Baso Pelémbang", {"e": ["é", "è"], "E": ["É", "È"], "o": ["ó"], "O": ["Ó"]}),
    ("mak", "us", "Basa Mangkasarak", {}, {"home": "'"}),
    ("pmf", "us", "Basa Pamona", {}, {"home": "'"}),
    ("mfa", "us", "Kècèk Nnayu Klatae-Ttaning", {"e": ["è", "é"], "E": ["È", "É"], "o": ["ó"], "O": ["Ó"], "u": ["ũ"], "U": ["Ũ"]}),
    ("ibb", "us", "Ibibio", {"o": ["ọ"], "O": ["Ọ"], "n": ["ñ", "n̄"], "N": ["Ñ", "N̄"], "i": ["ị"], "I": ["Ị"], "e": ["ẹ"], "E": ["Ẹ"],
                            "u": ["ụ"], "U": ["Ụ"], "a": ["ʌ"], "A": ["Ʌ"]}),
    ("vmf", "de", "Fränggisch", {"a": ["â"], "A": ["Â"], "o": ["ô"], "O": ["Ô"]}),
    ("sdh", "ckb", "کوردی خوارین", {"و": ["ۊ"]}),
    ("bgn", "fa", "بلوچی رخشانی", {"ی": ["ے"], "ن": ["ں"], "ت": ["ٹ"], "د": ["ڈ"], "ر": ["ڑ"], "ه": ["ہ", "ھ"], "گ": ["ګ"]}),
    ("adx", "bo", "Amdo Tibetan", {}, {"model": False}),
    ("khg", "bo", "Khams Tibetan", {}, {"model": False}),
    ("sck", "hi", "Sadri", {}, {"model": False}),
    ("knn", "mr", "Maharashtrian Konkani", {}, {"model": False}),
    ("nod-Thai", "th", "Northern Thai (Thai)", {}, {"model": False}),
    ("kfr", "gu", "Kutchi", {}, {"model": False}),
    ("tyz", "vi", "Tày",
     {v: [unicodedata.normalize("NFC", v + "\u0331"), unicodedata.normalize("NFC", v + "\u030F")]
      for v in "aăâeêioôơuưyAĂÂEÊIOÔƠUƯY"},
     {"model": False}),
    ("rkt", "as", "Rangpuri", {str(d): [chr(0x09E6 + d)] for d in range(10)}),
    ("kjp", "my", "Eastern Pwo Karen",
     {"ဏ": ["ၮ"], "ယ": ["ၯ"], "ဃ": ["ၰ"], "န": ["ၞ"], "မ": ["ၟ"], "င": ["ၚ"], "ု": ["ဴ"], "ႋ": ["ႉ"],
      "1": ["၁"], "2": ["၂"], "3": ["၃"], "4": ["၄"], "5": ["၅"], "6": ["၆"], "7": ["၇"], "8": ["၈"], "9": ["၉"],
      "0": ["၀"]},
     {"swap": {"ဩ": "ႋ", "၏": "ၜ", "ၒ": "ၠ"}, "model": False}),
    ("cdo", "us", "Mìng-dĕ̤ng-ngṳ̄",
     with_capitals({"a": ["á", "â", "ă", "ā", "à", "ā̤", "â̤", "à̤", "ă̤", "á̤", "a̤"],
                    "e": ["ê", "é", "è", "ĕ", "ē", "e̤", "è̤", "é̤", "ĕ̤", "ê̤", "ē̤"],
                    "i": ["ì", "ĭ", "ī"],
                    "o": ["ŏ", "ó", "ô", "ò", "ō", "ó̤", "ô̤", "ò̤", "ō̤", "ŏ̤", "o̤"],
                    "u": ["ŭ", "ṳ̄", "ṳ̆", "ū", "ù", "ṳ", "ṳ̀"]})),
    ("hif", "us", "Fiji Hindi", {}),
    ("abr", "ak", "Bono", {}),
    ("egl", "it", "Emilian",
     with_capitals({"e": ["è", "é", "ê", "ē", "ë"], "a": ["à", "â", "ä", "å", "á"], "o": ["ò", "ó", "ō", "ô", "ö"],
                    "i": ["ì", "î", "í", "ī"], "u": ["ù", "û", "ū", "ü", "ú"], "s": ["ś", "ṡ", "ş", "ṣ"],
                    "n": ["ṅ"], "c": ["ć"], "v": ["ṿ"], "l": ["ł"], "g": ["ǵ"], "z": ["ż"]})),
    ("mnp", "us", "Mâing-bă̤-ngṳ̌",
     with_capitals({"a": ["â̤", "á", "a̿", "ă", "ǎ", "ā", "â", "à", "ă̤", "ǎ̤", "à̤", "ā̤"],
                    "e": ["ê", "ě", "é", "ē", "ě̤", "è", "e̿", "ĕ", "é̤", "ê̤", "e̤̿", "ē̤", "è̤", "ĕ̤"],
                    "i": ["ī", "ǐ", "i̿", "ì", "í", "ĭ", "î"],
                    "o": ["ŏ̤", "ŏ", "ó̤", "ō", "ó", "ò̤", "ǒ", "ô̤", "ǒ̤", "ō̤", "o̿", "o̤̿", "ò", "ô"],
                    "u": ["ǔ", "ú", "u̿", "ṳ̌", "ṳ̂", "ū", "ŭ", "ṳ̄", "ṳ̀", "ṳ", "ṳ́", "ù", "ṳ̆", "ṳ̿", "û"]}),
     {"model": False}),
    ("sty", "ba", "Себер татар", {"е": ["ё"], "Е": ["Ё"]}, {"model": False}),
    ("tts", "th", "อีสาน", {}, {"model": False}),
    ("mdh", "us", "Magindanawn", {}, {"model": False}),
    ("cps", "us", "Capiznon", {}, {"model": False}),
    ("meo", "ms", "Kedah Malay", {}, {"model": False}),
    ("qxq", "azb", "قشقایی", {"ی": ["ێ"]}, {"model": False}),
    ("rej", "us", "Baso Jang", {"e": ["ê"], "E": ["Ê"]}, {"model": False}),
    ("bbj", "us", "Ghomala'",
     with_capitals({"e": ["é", "ə", "ə́", "è", "ə̂", "ê", "ɛ́", "ə̌", "ə̀", "ɛ̂", "ɛ̀", "ɛ", "ě", "ɛ̌"],
                    "a": ["à", "á", "â", "ǎ"], "o": ["ɔ", "ɔ́", "ǒ", "ó", "ô", "ɔ̌", "ò", "ɔ̂", "ɔ̀"],
                    "u": ["ú", "ʉ́", "ù", "ʉ", "û", "ǔ", "ʉ̂"], "i": ["í", "î", "ï", "ì", "ǐ"],
                    "n": ["ŋ", "ŋ́"], "y": ["ƴ"]}),
     {"model": False}),
    ("bug", "us", "Basa Ugi", {}, {"model": False}),
    ("lki", "ckb", "لەکی", {}, {"model": False}),
    ("bcc", "bal", "بلۏچی", {"و": ["ۏ"], "ی": ["ݔ"], "گ": ["ګ"]}, {"model": False}),
]


def shifted(text):
    """What a key typing `text` types on the Shift view: its capital (ā's
    Ā, ɔ's Ɔ), or the same where it has none (an apostrophe, Lahu's
    tones)."""
    upper = text.upper()
    return upper if len(upper) == len(text) else text


def build(lang, base, name, extra, extras=None):
    extras = extras or {}
    swap = extras.get("swap", {})
    home = extras.get("home")
    layout = json.loads((ROOT / "layouts" / f"{base}.json").read_text(encoding="utf-8"))
    out = {}
    for k, v in layout.items():
        # The base's variety of its own language (English's GB) is not
        # one of this language's.
        if k == "dialect":
            continue
        if k == "views" and extra:
            # A letter only a long press types: the gesture is on for it.
            out["long_press_letters"] = True
        out[k] = v
    out["id"] = lang.lower()
    out["name"] = name
    # A language no text predicts (build-exemplar-layouts.py's) has neither
    # suggestions nor glides, which would find no word.
    if extras.get("model") is False:
        out["prediction_enabled"] = False
        out["swipe_enabled"] = False
        out.pop("prediction_lang", None)
    # A layout with no model has no prediction language (Ge'ez, as Amharic).
    if out.get("prediction_enabled", True):
        out["prediction_lang"] = lang
    homes = [] if not home else [home] if isinstance(home, str) else home
    added = set()
    swapped = set()
    for view in out["views"]:
        for row in view["rows"]:
            for i, key in enumerate(row["keys"]):
                if key.get("label") in swap:
                    letter = swap[key["label"]]
                    swapped.add(key["label"])
                    row["keys"][i] = key = {"width_units": key["width_units"], "label": letter,
                                            "output": {"kind": "text", "text": letter}}
                letters = extra.get(key.get("label"))
                if letters:
                    rest = [k for k in key.get("long_press", []) if k.get("text") not in letters]
                    key["long_press"] = [{"kind": "text", "text": t} for t in letters] + rest
                    added.add(key.get("label"))
    # A key at the home row's end has the long press its letter is given
    # (Ngambay's ə, ə́ on its).
    added |= set(homes) | {shifted(h) for h in homes}
    missing = (set(extra) - added) | (set(swap) - swapped)
    if missing:
        sys.exit(f"{lang}: no key {' '.join(sorted(missing))} on {base}'s layout")
    if homes:
        # The home row (the second of letters), its spacers out and the
        # key at its end, on the base and Shift views alike.
        for view in out["views"]:
            if view["id"] not in ("base", "shift"):
                continue
            row = view["rows"][1]["keys"]
            if row[0].get("output", {}).get("kind") != "none" or row[-1].get("output", {}).get("kind") != "none":
                sys.exit(f"{lang}: {base}'s home row has no spacers to give way")
            keys = []
            for h in (shifted(h) if view["id"] == "shift" else h for h in homes):
                key = {"width_units": 1, "label": h, "output": {"kind": "text", "text": h}}
                if extra.get(h):
                    key["long_press"] = [{"kind": "text", "text": t} for t in extra[h]]
                keys.append(key)
            view["rows"][1]["keys"] = row[1:-1] + keys
    return json.dumps(out, ensure_ascii=False, indent=2) + "\n"


def main():
    check = "--check" in sys.argv[1:]
    stale = []
    for lang, base, name, extra, *extras in DERIVED:
        text = build(lang, base, name, extra, *extras)
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
