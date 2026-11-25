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
-wa forms, ኧ), named in Latin letters, as they also write their names;
and Sebat Bet Gurage, whose letters SIL's SLDR lists, with ʷ in place of
the v it does not write and ʸ in place of the hyphen (on ?123 still),
which the composer reads after m b f p as their labialised syllables
(Ethiopic Supplement's ᎀ…) and after q k x g as their palatalised ones
(Ethiopic Extended's ⷀ…), named in its script, as SLDR names it. Kabiye, Akoose and Grebo, whose long presses typed more
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
Capiznon, Buginese (é first on e) and Rejang (ê first on e) on the US
keys, Kedah Malay on Malay's (å first on a), Qashqai on South Azerbaijani's with ێ on ی, Ghomala' on the
US keys with ə ɛ ɔ ʉ ŋ ƴ and its tones on the long presses, Laki on
Sorani's, and Southern Balochi on Balochi's, ۏ ݔ ګ on و ی گ; from smaller
test wikis still and FineWeb-2's few pages, Ahirani on Marathi's keys,
Harauti and Kannauji on Hindi's, Chittagonian on Bengali's with ৱ ৰ on ব
র, Khorasani Turkic on South Azerbaijani's, Northern Khmer in the Thai
script on Thai's, and Hani, Nandi (ñ ā ē ĩ ō, an apostrophe key), North
Moluccan Malay (its accented vowels), Bangka Malay, Banggai, Cuyonon
(ñ), Youjiang Zhuang, Mongondow (ḷ ḻ, an apostrophe key), Pokot,
Manggarai (é first, a ’ key) and Ma'anyan on the US keys. Church
Slavonic, with its model, is typed on Russian's keys with є ѣ ꙑ ꙁ ꙗ in
place of the е э ы з я it hardly writes, ѥ and its accent marks at the
home row's end, the rest of its letters on long presses, and Noto
Sans's Cyrillic for the Extended-B letters DejaVu Sans lacks. And, with
no text to take letters from but a source naming their script, whose
national keys type all of it: Dari on Persian's keys with the Pashto
letters CLDR's Dari (fa_AF) adds on long presses, Valencian on
Catalan's, the Devanagari languages CLDR's language data or Wikidata's
writing systems name (Bhili, Kumaoni, Kurukh, Mewari, Nimadi,
Shekhawati, Wagdi, Mundari; Bundeli, Dhundhari, Korku, Malvi, Merwari,
Mahasu Pahari, Halbi) on Hindi's, Lambadi on Telugu's, Southern Thai on
Thai's, Dakhini on Urdu's, and Bishnupriya Manipuri on Bengali's with
ৱ on ব (its wiki has given it a model since). MADLAD-400's four, with their models: Tandroy-Mahafaly Malagasy
on Malagasy's keys with à è é ò ì ỳ first on the long presses, Hawu on
the US keys with è first on e and its saltillo ꞌ at the home row's end,
Kituba on them as they are, and Ayacucho Quechua on Quechua's with á é
first. From the Bibles eBible.org has of them, which have given them
models since: Southern Oromo (Guji) on Oromo's keys, whose apostrophe
key types its glottal stop (in 8% of its words), and Yaka on the US keys with
its high tones á é í ó ú first on the long presses. Zeelandic, with its model, from its Wikipedia, is
typed on Dutch's keys with ê ô first on the long presses of e and o and
an apostrophe key (its 't and 'n and the words it drops an h or ge-
from, 'Olland, 'ebouwd: one word in sixteen). With no model, from the
Universal Declaration of Human Rights in their languages: Maay on
Somali's keys and Nyamwezi on the US keys, each with an apostrophe key
(two words in a hundred write one), and Chokwe and Northern Qiandong
Miao, whose tones are letters, on the US keys as they are; from
Wiktionary's entries: Garo on them with a key for its raised dot
(A·chik: a quarter of its words), and in the Bengali script, which
CLDR's language data names for it, on Bengali's keys; Mandar and Bouyei
on them as they are, Muong on Vietnamese's keys, and Kui on Odia's with
the overline Unicode encodes for it (U+0B55) at the home row's end; and
from their Incubator test wikis, Central Malay, Komering and Kutai Malay
on the US keys as they are. From the wikis no round had used, Batak
Mandailing and Ghanaian Pidgin on the US keys as they are; Igala on them
with ọ and ẹ first on o's and e's long presses (each in one word in
twenty), its tones after them and on the other vowels, ñ on n's, and an
apostrophe key (ny’ẹ́yọ́, l'amẹnwu: two words in a hundred); Arakanese on
Burmese's keys; and Veluws on Dutch's, ö first on o, ä å on a and ü on u,
and an apostrophe key (its 't and 'n: two words in a hundred). With no
model, Godwari, Mandeali and Surjapuri on Hindi's keys, Varhadi on
Marathi's, Vasavi on Hindi's and Gujarati's and Kachi Koli on
Gujarati's: the scripts LinguaMeta names for them, each named in its
own words as LinguaMeta writes them; and Urum in Cyrillic, which
LinguaMeta names, on Russian's keys, the ӧ ӱ ґ Wiktionary's entries
write on о's, у's and г's long presses and a key for the ʼ it writes
where Russian writes ь (one entry in thirteen). From the African
Storybook's stories, Masaaba and Turkana, with their models, on the US
keys with an apostrophe key (Turkana's ng' is in a sixth of its words),
Masaaba's ŋ first on n's long press, and from their letters there,
Bukusu and Malawi Lomwe, with no model, the same way. SMOL's
translations have given models to fourteen of these, on the keys they
had: Ahirani, Bundeli, Kumaoni, Kurukh, Mandeali, Nimadi, Surjapuri,
Mundari and Garo here, and Bagheli, Bagri, Kangri, Surgujia and Zarma,
build-exemplar-layouts.py's; Kurukh with ' first on its full stop's long
press (र'ईर: one word in twenty-six), Hindi's home row having no spacers,
and Nimadi with its avagraha ऽ on ः's (करनऽ: one word in twenty), as
Sanskrit's. StoryWeaver's stories have given models to two more, Halbi
and Korku, on Hindi's keys; Kui, with no model, has Odia's keys and
their long presses (ୟ on ଯ, ୱ on ଵ). Pulaar, with no model, is typed on Fula's keys with ñ first
on n's long press: the letters SMOL's Pulaar translations write, as
CLDR's Fula, whose name for itself is Pulaar, lists them. With no
model, from the letters of FineWeb-2's few pages of their Bibles, six
more on the US keys: Northwest Gbaya with ɛ, in a quarter of its words,
and an apostrophe at the home row's end, ɔ in place of x and the
cedilla its nasal vowels write (a̧ ɔ̧, in an eighth of its words) in
place of q, its tones first on the vowels' long presses (ɛ́ ɛ̀ on ɛ's),
ŋ ɓ ɗ on n b d (long presses had been in 58% of its words, now 26%);
Sar with ə and ɔ in place of q and x and its tones' marks, the macron
and the acute (in two words in five and a third), at the home row's end,
the tilde below on the acute's long press, ɓ on b's (77% → 9%); Masana
with ɗ ŋ ɓ ɦ first on d n b h and an apostrophe key (di'e, kulo'o);
Marba with â î á ï ê ä ô û first on the vowels' and a ’ key, its
implosive d’ (in a ninth of its words); Kinyaturu with ë, in more than
a quarter of its words, and an apostrophe (ng') at the home row's end,
ü first on u; and Eastern Oromo on Oromo's keys, its apostrophe key
typing ’. Southern
Luri, with no model, is typed on Persian's keys with the letters of its
test wiki's two spellings, Persian's and one that writes every vowel (ة
for a, in two words in five, ە ݖ, and ۿ for h): those four in place of
ث ذ ظ ض, which it hardly writes (each on its letter's long press: ة's ث),
ۊ and ڤ first on و and ف (each in a tenth of its words), ڍ ڌ on د, ڲ on
گ, ݩ on ن, ھ on ه and the mark ٛ on the full stop's (long presses had
been in half its words, now a quarter); Noto Sans Arabic draws them, which
DejaVu Sans does not all. A key at the
home row's end types its capital on the Shift view (Ā, Ɔ, Җ), and has the long press its letter is
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
# its apostrophe), "font_script" for one whose letters the default face
# lacks (Church Slavonic's ꙁ ꙃ ꙋ: Noto Sans's Cyrillic), "model": False
# for a language with none, and "composer_model" for one whose composer
# offers its model's words where the base predicts nothing itself (Sebat
# Bet Gurage's, through Amharic's Ge'ez composer).
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
    ("gur", "us", "Gurenɛ", {"e": ["ɛ"], "o": ["ɔ"], "n": ["ŋ"], "E": ["Ɛ"], "O": ["Ɔ"], "N": ["Ŋ"]}, {"model": False}),
    ("bum", "us", "Bulu", {"e": ["é"], "o": ["ô"], "n": ["ñ"], "E": ["É"], "O": ["Ô"], "N": ["Ñ"]}),
    ("bin", "us", "Ẹdo", {"e": ["ẹ"], "o": ["ọ"], "E": ["Ẹ"], "O": ["Ọ"]}),
    ("teo", "us", "Ateso", {"n": ["ŋ"], "N": ["Ŋ"]}),
    ("sgc", "us", "Kipsigis", {}),
    ("mfq", "us", "Moba", {"e": ["ɛ"], "o": ["ɔ"], "n": ["ŋ"], "E": ["Ɛ"], "O": ["Ɔ"], "N": ["Ŋ"]}, {"model": False}),
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
    ("dnj", "us", "Dan", {"e": ["ë", "ɛ"], "E": ["Ë", "Ɛ"], "o": ["ö", "ɔ"], "O": ["Ö", "Ɔ"], "u": ["ü"], "U": ["Ü"], "v": ["ʋ̈", "ʋ"], "V": ["Ʋ̈", "Ʋ"], "i": ["ɩ"], "I": ["Ɩ"]}, {"model": False}),
    ("gbo", "us", "Grebo", {"a": ["à", "ǎ", "ä"], "A": ["À", "Ǎ", "Ä"], "e": ["ɛ", "ě", "è", "ɛ̌", "ɛ̀", "ɛ̈"], "E": ["Ɛ", "Ě", "È", "Ɛ̌", "Ɛ̀", "Ɛ̈"], "i": ["ǐ", "ì", "ï"], "I": ["Ǐ", "Ì", "Ï"], "o": ["ɔ", "ǒ", "ò", "ɔ̀", "ɔ̌", "ɔ̈"], "O": ["Ɔ", "Ǒ", "Ò", "Ɔ̀", "Ɔ̌", "Ɔ̈"], "u": ["ù", "ǔ", "ü"], "U": ["Ù", "Ǔ", "Ü"],
                           "ǒ": ["q"], "Ǒ": ["Q"], "ě": ["x"], "Ě": ["X"]},
     {"swap": {"q": "ǒ", "Q": "Ǒ", "x": "ě", "X": "Ě"}, "home": ["ɛ", "ɔ"], "model": False}),
    ("heh", "us", "Kihehe", {}),
    ("gux", "us", "Gulmancema", {"n": ["ŋ", "ñ"], "N": ["Ŋ", "Ñ"]}),
    ("gmv", "us", "Gamo", {}),
    ("suk", "us", "Kisukuma", {"i": ["ī"], "I": ["Ī"], "u": ["ū"], "U": ["Ū"]}, {"home": "'"}),
    ("spp", "us", "Supyire", {"a": ["à", "á", "â"], "A": ["À", "Á", "Â"], "e": ["ɛ", "è", "ɛ̀", "é", "ê", "ɛ̂", "ɛ́"], "E": ["Ɛ", "È", "Ɛ̀", "É", "Ê", "Ɛ̂", "Ɛ́"], "i": ["í", "ì", "î"], "I": ["Í", "Ì", "Î"], "n": ["ŋ", "ɲ"], "N": ["Ŋ", "Ɲ"], "o": ["ɔ", "ɔ̀", "ò", "ɔ́", "ó", "ɔ̂", "ô"], "O": ["Ɔ", "Ɔ̀", "Ò", "Ɔ́", "Ó", "Ɔ̂", "Ô"], "u": ["ù", "ú", "û"], "U": ["Ù", "Ú", "Û"]}, {"home": "'"}),
    ("bas", "us", "Basaa", {"e": ["é", "è"], "E": ["É", "È"], "n": ["ñ"], "N": ["Ñ"], "o": ["ô"], "O": ["Ô"], "a": ["à"], "A": ["À"]}),
    ("hay", "us", "Ekihaya", {}, {"home": "'", "model": False}),
    ("kbr", "us", "Kafa", {}, {"home": "'", "model": False}),
    ("ngl", "us", "Elomwe", {"a": ["á", "à", "ã"], "A": ["Á", "À", "Ã"], "e": ["é"], "E": ["É"], "o": ["ó", "ò"], "O": ["Ó", "Ò"]}),
    ("gog", "us", "Cigogo", {}, {"model": False}),
    ("chw", "us", "Echuwabo", {"a": ["á", "ã"], "A": ["Á", "Ã"], "c": ["ç"], "C": ["Ç"], "e": ["é"], "E": ["É"], "i": ["í"], "I": ["Í"], "n": ["ñ"], "N": ["Ñ"], "o": ["ó"], "O": ["Ó"]}),
    ("fat", "us", "Mfantse", {"e": ["ɛ"], "E": ["Ɛ"], "o": ["ɔ", "õ"], "O": ["Ɔ", "Õ"], "a": ["ã"], "A": ["Ã"], "u": ["ũ"], "U": ["Ũ"]}),
    ("tsc", "us", "Xitshwa", {}),
    ("bud", "us", "Ncam", {"a": ["á", "à", "ā"], "A": ["Á", "À", "Ā"], "b": ["b́", "b̀"], "B": ["B́", "B̀"], "e": ["é", "è", "ē"], "E": ["É", "È", "Ē"], "i": ["í", "ì", "ī"], "I": ["Í", "Ì", "Ī"], "l": ["l̄", "l̀", "ĺ"], "L": ["L̄", "L̀", "Ĺ"], "m": ["ḿ", "m̀", "m̄"], "M": ["Ḿ", "M̀", "M̄"], "n": ["ŋ", "ǹ", "ń", "n̄"], "N": ["Ŋ", "Ǹ", "Ń", "N̄"], "o": ["ɔ", "ɔ́", "ò", "ɔ̃", "ɔ̄"], "O": ["Ɔ", "Ɔ́", "Ò", "Ɔ̃", "Ɔ̄"], "u": ["ú", "ū", "ù"], "U": ["Ú", "Ū", "Ù"]}, {"model": False}),
    ("kao", "us", "Xaasongaxango", {"n": ["ɲ", "ŋ", "ń"], "N": ["Ɲ", "Ŋ", "Ń"], "i": ["í"], "I": ["Í"], "e": ["é"], "E": ["É"]}, {"home": "'", "model": False}),
    ("gvl", "us", "Gulay", {"e": ["è", "é", "ê"], "E": ["È", "É", "Ê"], "a": ["à", "á"], "A": ["À", "Á"], "b": ["ɓ"], "B": ["Ɓ"], "d": ["ɗ"], "D": ["Ɗ"], "m": ["m̂"], "M": ["M̂"], "o": ["ɔ", "ò", "ó"], "O": ["Ɔ", "Ò", "Ó"], "u": ["ú"], "U": ["Ú"]}, {"home": "'", "model": False}),
    ("nnb", "us", "Kinande", {"i": ["ĩ", "í", "į"], "I": ["Ĩ", "Í", "Į"], "u": ["ũ", "ú", "ų"], "U": ["Ũ", "Ú", "Ų"], "a": ["á"], "A": ["Á"], "e": ["é"], "E": ["É"], "o": ["ó"], "O": ["Ó"], "ũ": ["q"], "Ũ": ["Q"]},
     {"swap": {"q": "ũ", "Q": "Ũ"}, "home": ["ĩ", "'"], "model": False}),
    ("nyo", "us", "Runyoro", {}, {"home": "'", "model": False}),
    ("niu", "us", "Vagahau Niuē", {"a": ["ā"], "A": ["Ā"], "e": ["ē"], "E": ["Ē"], "o": ["ō"], "O": ["Ō"], "u": ["ū"], "U": ["Ū"]}),
    ("hwc", "us", "Pidgin", {}),
    ("acf", "us", "Kwéyòl", {"e": ["é", "è"], "E": ["É", "È"], "o": ["ò"], "O": ["Ò"]}),
    ("mrw", "us", "Meranaw", {"a": ["ā"], "A": ["Ā"], "u": ["ū"], "U": ["Ū"], "i": ["ī"], "I": ["Ī"]}),
    ("nij", "us", "Ngaju", {}),
    ("guc", "us", "Wayuunaiki", {"u": ["ü", "ú"], "U": ["Ü", "Ú"], "a": ["á"], "A": ["Á"], "e": ["é"], "E": ["É"], "i": ["í"], "I": ["Í"], "o": ["ó"], "O": ["Ó"], "n": ["ñ"], "N": ["Ñ"]}, {"home": ["ü", "'"]}),
    ("ljp", "us", "Lampung", {}, {"model": False}),
    ("sas", "us", "Basa Sasak", {"e": ["ẽ"], "E": ["Ẽ"]}, {"model": False}),
    ("gor", "us", "Hulontalo", {}, {"home": "'"}),
    ("itv", "us", "Itawit", {}, {"model": False}),
    ("bku", "us", "Buhid", {}, {"model": False}),
    ("bpr", "us", "Blaan", {"a": ["à"], "A": ["À"], "e": ["é", "è", "ê"], "E": ["É", "È", "Ê"], "i": ["ì"], "I": ["Ì"], "o": ["ò"], "O": ["Ò"], "u": ["ù"], "U": ["Ù"]}, {"model": False}),
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
     {"swap": {"q": "ꞈ", "Q": "ꞈ", "x": "'", "X": "'", "z": "ˍ", "Z": "ˍ"}, "home": ["ˬ", "ˇ"], "font_script": "latin", "model": False}),
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
    ("bgn", "fa", "بلوچی رخشانی", {"ی": ["ے"], "ن": ["ں"], "ت": ["ٹ"], "د": ["ڈ"], "ر": ["ڑ"], "ه": ["ہ", "ھ"], "گ": ["ګ"]},
     {"font_script": "arabic"}),
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
     {"swap": {"ဩ": "ႋ", "၏": "ၜ", "ၒ": "ၠ"}}),
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
    ("meo", "ms", "Kedah Malay", {"a": ["å"], "A": ["Å"]}, {"model": False}),
    ("qxq", "azb", "قشقایی", {"ی": ["ێ"]}, {"model": False}),
    ("rej", "us", "Baso Jang", {"e": ["ê"], "E": ["Ê"]}, {"model": False}),
    ("bbj", "us", "Ghomala'",
     with_capitals({"e": ["é", "ə", "ə́", "è", "ə̂", "ê", "ɛ́", "ə̌", "ə̀", "ɛ̂", "ɛ̀", "ɛ", "ě", "ɛ̌"],
                    "a": ["à", "á", "â", "ǎ"], "o": ["ɔ", "ɔ́", "ǒ", "ó", "ô", "ɔ̌", "ò", "ɔ̂", "ɔ̀"],
                    "u": ["ú", "ʉ́", "ù", "ʉ", "û", "ǔ", "ʉ̂"], "i": ["í", "î", "ï", "ì", "ǐ"],
                    "n": ["ŋ", "ŋ́"], "y": ["ƴ"]}),
     {"model": False}),
    ("bug", "us", "Basa Ugi", {"e": ["é"], "E": ["É"]}, {"model": False}),
    ("lki", "ckb", "لەکی", {}, {"model": False}),
    ("bcc", "bal", "بلۏچی", {"و": ["ۏ"], "ی": ["ݔ"], "گ": ["ګ"]}, {"model": False}),
    ("ahr", "mr", "अहिराणी", {}),
    ("hoj", "hi", "हाड़ौती", {}, {"model": False}),
    ("bjj", "hi", "कन्नौजी", {}, {"model": False}),
    ("ctg", "bn", "চাটগাঁইয়া", {"ব": ["ৱ"], "র": ["ৰ"]}, {"model": False}),
    ("hni", "us", "Haqniqdoq", {}, {"model": False}),
    ("niq", "us", "Nandi", with_capitals({"n": ["ñ"], "a": ["ā"], "e": ["ē"], "i": ["ĩ"], "o": ["ō"]}),
     {"home": "'", "model": False}),
    ("max", "us", "North Moluccan Malay",
     with_capitals({"a": ["à", "á"], "e": ["é", "è"], "i": ["í"], "o": ["ò", "ó"], "u": ["ù", "ú", "ŭ"]}), {"model": False}),
    ("mfb", "us", "Bangka Malay", {}, {"model": False}),
    ("bgz", "us", "Banggai", {}, {"model": False}),
    ("cyo", "us", "Cuyonon", {"n": ["ñ"], "N": ["Ñ"]}, {"model": False}),
    ("zyj", "us", "Youjiang Zhuang", {}, {"model": False}),
    ("mog", "us", "Mongondow", with_capitals({"l": ["ḷ", "ḻ"]}), {"home": "'", "model": False}),
    ("pko", "us", "Pokot", {}, {"model": False}),
    ("kmz", "azb", "Khorasani Turkic", {}, {"model": False}),
    ("kxm", "th", "Northern Khmer (Thai script)", {}, {"model": False}),
    ("mqy", "us", "Manggarai", {"e": ["é"], "E": ["É"]}, {"home": "’", "model": False}),
    ("mhy", "us", "Ma'anyan", {}, {"model": False}),
    ("cu", "ru", "Словѣньскъ",
     {**with_capitals({"є": ["е"], "ѣ": ["э"], "ꙑ": ["ы"], "ꙁ": ["з", "ѕ", "ꙃ"], "ꙗ": ["я", "ѧ", "ѩ"],
                       "и": ["ї", "і", "ѵ", "ѷ"], "у": ["ѫ", "ꙋ", "ѹ", "ў"], "ю": ["ѭ"], "о": ["ѡ", "ѻ", "ѿ"],
                       "ф": ["ѳ"], "к": ["ѯ"], "п": ["ѱ"]}),
      "\u0301": ["\u0483", "\u0484", "\u0486", "\u0300", "\u0311"]},
     {"swap": {"е": "є", "Е": "Є", "э": "ѣ", "Э": "Ѣ", "ы": "ꙑ", "Ы": "Ꙑ", "з": "ꙁ", "З": "Ꙁ", "я": "ꙗ", "Я": "Ꙗ"},
      "home": ["ѥ", "\u0301"], "font_script": "cyrillic"}),
    ("prs", "fa", "دری", {"ت": ["ټ"], "ح": ["ځ", "څ"], "د": ["ډ"], "ر": ["ړ", "ږ"], "س": ["ښ"], "گ": ["ګ"], "ن": ["ڼ"],
                          "ی": ["ئ", "ي", "ى"]}, {"model": False}),
    ("bhb", "hi", "Bhili", {}, {"model": False}),
    ("kfy", "hi", "Kumaoni", {}),
    ("kru", "hi", "Kurukh", {"।": ["'"]}),
    ("mtr", "hi", "Mewari", {}, {"model": False}),
    ("noe", "hi", "Nimadi", {"ः": ["ऽ"]}),
    ("swv", "hi", "Shekhawati", {}, {"model": False}),
    ("wbr", "hi", "Wagdi", {}, {"model": False}),
    ("unr", "hi", "Mundari", {}),
    ("lmn", "te", "Lambadi", {}, {"model": False}),
    ("sou", "th", "Southern Thai", {}, {"model": False}),
    ("dcc", "ur", "Dakhini", {}, {"model": False}),
    ("bpy", "bn", "বিষ্ণুপ্রিয়া মণিপুরী", {"ব": ["ৱ"]}),
    ("bns", "hi", "Bundeli", {}),
    ("dhd", "hi", "Dhundhari", {}, {"model": False}),
    ("kfq", "hi", "Korku", {}),
    ("mup", "hi", "Malvi", {}, {"model": False}),
    ("wry", "hi", "Merwari", {}, {"model": False}),
    ("bfz", "hi", "Mahasu Pahari", {}, {"model": False}),
    ("hlb", "hi", "Halbi", {}),
    ("ca-valencia", "ca", "Valencià", {}, {"model": False}),
    ("tdx", "mg", "Tandroy", with_capitals({"a": ["à"], "e": ["è", "é"], "o": ["ò"], "i": ["ì"], "y": ["ỳ"]})),
    ("hvn", "us", "Hawu", with_capitals({"e": ["è"]}), {"home": "\ua78c"}),
    ("ktu", "us", "Kituba", {}),
    ("quy", "qu", "Runa Simi (Ayacucho)", with_capitals({"a": ["á"], "e": ["é"]})),
    ("gax", "om", "Southern Oromo", {}),
    ("yaf", "us", "Yaka", with_capitals({"a": ["á"], "e": ["é"], "i": ["í"], "o": ["ó"], "u": ["ú"]})),
    ("zea", "nl", "Zeêuws", with_capitals({"e": ["ê", "è", "ë", "é"], "o": ["ô", "ò", "ó", "ö"]}), {"home": "'"}),
    ("ymm", "so", "Maay", {}, {"home": "'", "model": False}),
    ("nym", "us", "Kinyamwezi", {}, {"home": "'", "model": False}),
    ("cjk", "us", "Chokwe", {}, {"model": False}),
    ("hea", "us", "Hmu", {}, {"model": False}),
    ("grt", "us", "A·chik", {}, {"home": "·"}),
    # Its model learns from SMOL's Garo spelled in the Bengali script
    # (make respelled-text); the raka, in one word in four, a key of its
    # own, ঋ's, which its Bible never writes.
    ("grt-Beng", "bn", "গারো", {}, {"swap": {"ঋ": "·"}}),
    ("mdr", "us", "Mandar", {}, {"model": False}),
    ("pcc", "us", "Haausqyaix", {}, {"model": False}),
    ("mtq", "vi", "Thiểng Mường", {}, {"model": False}),
    ("kxu", "or", "Kui", {}, {"home": "୕", "model": False}),
    # Ten of Gboard's languages that had no keyboard, their letters those
    # Wiktionary's entries (en.wiktionary.org's lemmas) and their Incubator
    # test wikis' few pages write: Nauruan (127 entries: â and ñ, one each),
    # Central Melanau (76) and Sabah Malay (19) on Malay's keys as they are,
    # Guyanese Creole (36, and its wiki's Creolese) as they are, Parauk
    # (its wiki writes the colon inside 18 of its 115 words, cia:, cao:;
    # Wiktionary's entries tones on its vowels, í ó á é ī ā ē), Gayo (é è
    # ö: its wiki's 492 words, Wiktionary's unggör), Izon on Igbo's keys
    # (its name Ịzọn, its entries' tones bení, kúkúndùkú), Balkan Gagauz
    # Turkish on Turkish's (its wiki's Rumelice), Lari in the Arabic script
    # on Persian's (its wiki's اچمی) and Batak Alas-Kluet on Indonesian's.
    ("nau", "us", "Dorerin Naoero", with_capitals({"a": ["â"], "n": ["ñ"]}), {"model": False}),
    ("mel", "ms", "Central Melanau", {}, {"model": False}),
    ("msi", "ms", "Sabah Malay", {}, {"model": False}),
    ("gyn", "us", "Creolese", {}, {"model": False}),
    ("prk", "us", "Lox Vax",
     with_capitals({"a": ["á", "ā"], "e": ["é", "ē"], "i": ["í", "ī"], "o": ["ó", "ō"], "u": ["ú", "ū"]}),
     {"home": ":", "model": False}),
    ("gay", "id", "Gayo", with_capitals({"e": ["é", "è"], "o": ["ö"]}), {"model": False}),
    ("ijc", "ig", "Ịzọn",
     with_capitals({"a": ["á", "à"], "e": ["ẹ", "é", "è"], "i": ["ị", "í", "ì"], "o": ["ọ", "ó", "ò"],
                    "u": ["ụ", "ú", "ù"]}),
     {"model": False}),
    ("bgx", "tr", "Rumelice", {}, {"model": False}),
    ("lrl", "fa", "اچمی", {}, {"model": False}),
    ("btz", "id", "Batak Alas-Kluet", {}, {"model": False}),
    # Fifteen more of Gboard's languages that had no keyboard, on the keys
    # of the language whose letters they write, their own letters first on
    # the long presses, as their texts write them: Wiktionary's entries
    # (Nung's 55, in SIL's Vietnamese-letter spelling, chéht, slíhp;
    # Kohistani Shina's 316 in the Arabic script, ڦ ڱ and the rising tone's
    # ٗ among them; Southwestern Fars' 115, Persian's letters; Cameroon
    # Pidgin's 22; Western Cham's 30 in Latin letters, â ă; Xong's, its
    # tones letters), Wikipedia's articles (Kohistani Shina's alphabet, ڦ
    # and ڱ its own; Wanetsi's poem and sentences in the Pashto alphabet,
    # ۀ, and ں for its nasal vowels; Kuhmareyi's forms in Persian
    # Wikipedia's, ڌ ۊ ٱ ٛ ۆ ێ ؽ ݩ ۉ; Takestani's ten words in Persian
    # Wikipedia's; Mehri's unmodified Arabic alphabet, the commonest way to
    # write it; Xong's 1956 alphabet; Western Cham's name in the Cham
    # script, ꨌꩌ ꨚꨭꩉ; Bangala's bangála, ɓisu; Upper Saxon's German
    # spellings; Pashai's one alphabet, drawn up since 2003), and Eastern
    # Balochi's test wiki (243 words in the Arabic script, all on Balochi's
    # keys). Named as Wikipedia's articles and LinguaMeta (Google
    # Research's language metadata) write their names, in English
    # otherwise (Southwestern Pashayi, as Southeastern Pashayi's keys are).
    ("nut", "vi", "Nùng", {}, {"model": False}),
    ("plk", "scl", "ݜݨیاٗ کستِین٘و", {"ف": ["ڦ"], "گ": ["ڱ"], "ُ": ["ٗ"]}, {"model": False}),
    ("fay", "fa", "کوهمَره‌ای",
     {"د": ["ڌ"], "و": ["ۊ", "ۆ", "ۉ"], "ا": ["ٱ"], "ی": ["ێ", "ؽ"], "ن": ["ݩ"], ".": ["ٛ"]},
     {"model": False, "font_script": "arabic"}),
    ("wne", "ps", "وڼېڅي", {"ه": ["ۀ"], "ن": ["ں"]}, {"model": False}),
    ("gdq", "ar", "مهريّت", {}, {"model": False}),
    ("tks", "fa", "تاتی", {}, {"model": False}),
    ("bgp", "bal", "بلوچی", {}, {"model": False}),
    ("psh", "psi", "Southwestern Pashayi", {}, {"model": False}),
    ("cja", "cjm", "ꨌꩌ ꨚꨭꩉ", {}, {"model": False}),
    ("cja-Latn", "us", "Cam pur", with_capitals({"a": ["â", "ă"]}), {"model": False}),
    ("mmr", "us", "Dut Xongb", {}, {"model": False}),
    ("wes", "us", "Kamtok", {}, {"model": False}),
    ("bah", "us", "Bahamian", {}, {"model": False}),
    ("trf", "us", "Trinidadian Creole", {}, {"model": False}),
    ("sxu", "de", "Obersächsisch", {}, {"model": False}),
    ("bxg", "ln", "Ngala", with_capitals({"a": ["á"], "b": ["ɓ"]}), {"model": False}),
    # The last four of Gboard's languages that had no keyboard. Anaang on
    # the US keys with ñ ö ü first on n's, o's and u's long presses: the
    # letters of its alphabet besides the 26 (and the digraphs ch gh gw kp
    # kw ny ñw, doubled vowels long), as Owulette's Annang-English
    # dictionary (2019, French Wikipedia's article), Omniglot's chart and
    # the 30 letters of Michael and Ijioma's pronouncing dictionary (2019)
    # list them; named Anaañ, as it calls itself. Ha on the US keys with an
    # apostrophe key, for the vowel it drops before the next word
    # (nʼumunsi, chʼamanza), as PanLex's 1,684 entries (CC0) write it, with
    # bh ch sh and l beside r; named Igiha, as Kirundi's keyboard is
    # Ikirundi. Northern and Southern Betsimisaraka Malagasy on Malagasy's
    # keys, its apostrophe key among them (amin’ny: one word in seventeen of
    # each), as JW.org's brochure in each writes them (6,300 words each):
    # Northern's velar nasal n̈ (zain̈y, ten̈a: nearly one word in five)
    # first on n's long press, before ñ, and ỳ à ê ì ï (Vakỳ, lalàn̈a,
    # folêra, marìny, naïvan̈a); Southern's à é ỳ ì (koà, maé, tatỳ,
    # fanìny), its velar nasal gn (fôgn), as Wikipedia's articles' tables
    # write both dialects' (agnaragnanazy, etoagna); named as JW.org and
    # Wikipedia name them.
    ("anw", "us", "Anaañ", with_capitals({"n": ["ñ"], "o": ["ö"], "u": ["ü"]}), {"model": False}),
    ("haq", "us", "Igiha", {}, {"home": "'", "model": False}),
    ("bmm", "mg", "Betsimisaraka Avaratra",
     with_capitals({"n": ["n̈"], "y": ["ỳ"], "a": ["à"], "e": ["ê"], "i": ["ì", "ï"]}), {"model": False}),
    ("bzc", "mg", "Betsimisaraka Atsimo", with_capitals({"a": ["à"], "e": ["é"], "y": ["ỳ"], "i": ["ì"]}),
     {"model": False}),
    ("pse", "us", "Baso Melayu Tenga", {}, {"model": False}),
    ("kge", "us", "Basa Kumoring", {}, {"model": False}),
    ("vkt", "us", "Kutai Malay", {}, {"model": False}),
    ("btm", "us", "Saro Mandailing", {}),
    ("gpe", "us", "Ghanaian Pidgin", {}),
    ("igl", "us", "Igala", with_capitals({"e": ["ẹ", "é", "è", "ẹ́", "ẹ̀"], "o": ["ọ", "ó", "ò", "ọ́", "ọ̀"],
                                         "i": ["í", "ì", "î"], "a": ["á", "à"], "u": ["ú", "ù"], "n": ["ñ", "ń"]}),
     {"home": "'"}),
    ("rki", "my", "ရခိုင်", {}),
    ("vel", "nl", "Veluws", with_capitals({"o": ["ö"], "a": ["ä", "å"], "u": ["ü"]}), {"home": "'"}),
    ("gdx", "hi", "गोड़वाड़ी", {}, {"model": False}),
    ("mjl", "hi", "मंडयाली", {}),
    ("sjp", "hi", "सुरजापुरी", {}),
    ("vah", "mr", "वऱ्हाडी", {}, {"model": False}),
    ("vas", "hi", "वासवी", {}, {"model": False}),
    ("vas-Gujr", "gu", "વસાવી", {}, {"model": False}),
    ("gjk", "gu", "કચ્છી કોળી", {}, {"model": False}),
    ("uum", "ru", "Урум", with_capitals({"о": ["ӧ"], "у": ["ӱ"], "г": ["ґ"]}), {"home": "ʼ", "model": False}),
    ("myx", "us", "Lumasaaba", with_capitals({"n": ["ŋ"]}), {"home": "'"}),
    ("tuv", "us", "Ng’aturkana", {}, {"home": "'"}),
    ("bxk", "us", "Lubukusu", {}, {"home": "'", "model": False}),
    ("lon", "us", "Ellomwe", {}, {"home": "'", "model": False}),
    ("fuc", "ff", "Pulaar", with_capitals({"n": ["ñ"]}), {"model": False}),
    ("sgw", "am", "ሰባት ቤት ጉራጌ", {}, {"swap": {"v": "ʷ", "V": "ʷ", "-": "ʸ"}, "model": False}),
    ("gya", "us", "Northwest Gbaya",
     {**with_capitals({"a": ["á", "à"], "e": ["ɛ", "é"], "i": ["í", "ì"], "o": ["ɔ", "ó", "ò"], "u": ["ú", "ù"],
                       "n": ["ŋ"], "b": ["ɓ"], "d": ["ɗ"], "ɛ": ["ɛ́", "ɛ̀"], "ɔ": ["ɔ́", "ɔ̀", "x"]}),
      "̧": ["q"]},
     {"swap": {"x": "ɔ", "X": "Ɔ", "q": "̧", "Q": "̧"}, "home": ["ɛ", "'"], "model": False}),
    ("mwm", "us", "Sar", {**with_capitals({"b": ["ɓ"], "ə": ["q"], "ɔ": ["x"]}), "́": ["̰"]},
     {"swap": {"q": "ə", "Q": "Ə", "x": "ɔ", "X": "Ɔ"}, "home": ["̄", "́"], "model": False}),
    ("mcn", "us", "Masana", with_capitals({"d": ["ɗ"], "n": ["ŋ"], "b": ["ɓ"], "h": ["ɦ"]}), {"home": "'", "model": False}),
    ("mpg", "us", "Marba",
     with_capitals({"a": ["â", "á", "ä", "à"], "i": ["î", "ï", "ì"], "e": ["ê"], "o": ["ô"], "u": ["û"]}),
     {"home": "’", "model": False}),
    ("rim", "us", "Kinyaturu", with_capitals({"e": ["ë"], "u": ["ü"]}), {"home": ["ë", "'"], "model": False}),
    ("hae", "om", "Eastern Oromo", {}, {"swap": {"'": "’"}, "model": False}),
    ("luz", "fa", "Southern Luri",
     {"ة": ["ث"], "ە": ["ذ"], "ݖ": ["ظ"], "ۿ": ["ض"], "و": ["ۊ", "ۋ", "ۏ"], "ف": ["ڤ"], "د": ["ڍ", "ڌ"],
      "گ": ["ڲ"], "ن": ["ݩ"], "ه": ["ھ"], ".": ["\u065b"]},
     {"swap": {"ث": "ة", "ذ": "ە", "ظ": "ݖ", "ض": "ۿ"}, "model": False, "font_script": "arabic"}),
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
    if extras.get("font_script"):
        out["font_script"] = extras["font_script"]
    # A language no text predicts (build-exemplar-layouts.py's) has neither
    # suggestions nor glides, which would find no word.
    if extras.get("model") is False:
        out["prediction_enabled"] = False
        out["swipe_enabled"] = False
        out.pop("prediction_lang", None)
    # A layout with no model has no prediction language: not its base's,
    # whose composer may read one (Amharic's: Ge'ez, Gedeo… have none). One
    # whose composer reads its own language's (Sebat Bet Gurage's) names it.
    if out.get("prediction_enabled", True) or extras.get("composer_model"):
        out["prediction_lang"] = lang
    elif out.get("prediction_lang") != lang:
        out.pop("prediction_lang", None)
    homes = [] if not home else [home] if isinstance(home, str) else home
    added = set()
    swapped = set()
    for view in out["views"]:
        for row in view["rows"]:
            for i, key in enumerate(row["keys"]):
                # A swap is a letter's: the symbol views keep their keys
                # (Amharic's hyphen, which Sebat Bet Gurage's ʸ takes).
                if key.get("label") in swap and not view["id"].startswith("symbol"):
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
