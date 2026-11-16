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
they are. A letter only a long press types turns
the gesture on for the layout (`long_press_letters`). A layout's id is
its language's tag in lower case (sd-deva), its prediction language the
tag.

    scripts/build-derived-layouts.py [--check]

--check fails if a written file would differ, as a test of the tree.
"""
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent

# The layout made, the one it is made from, its name, and what its keys
# gain on a long press (a key's label → the letters, first in its menu),
# on every view.
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
]


def build(lang, base, name, extra):
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
    out["prediction_lang"] = lang
    added = set()
    for view in out["views"]:
        for row in view["rows"]:
            for key in row["keys"]:
                letters = extra.get(key.get("label"))
                if letters:
                    rest = [k for k in key.get("long_press", []) if k.get("text") not in letters]
                    key["long_press"] = [{"kind": "text", "text": t} for t in letters] + rest
                    added.add(key.get("label"))
    missing = set(extra) - added
    if missing:
        sys.exit(f"{lang}: no key {' '.join(sorted(missing))} on {base}'s layout")
    return json.dumps(out, ensure_ascii=False, indent=2) + "\n"


def main():
    check = "--check" in sys.argv[1:]
    stale = []
    for lang, base, name, extra in DERIVED:
        text = build(lang, base, name, extra)
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
