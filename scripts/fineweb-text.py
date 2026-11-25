#!/usr/bin/env python3
"""fineweb-text.py — a language's FineWeb-2 web text as sentences, one a
line: what its model learns from, for languages with no Wikipedia of
their own.

FineWeb-2 (Hugging Face; ODC-By 1.0, subject to Common Crawl's Terms of
Use) is web text from Common Crawl, sorted by language with the GlotLID
classifier, which errs most between close languages: its Dogri holds
Punjabi poetry, its Angika is Hindi news. So a document is kept only
when it reads as its language:

- a page of bible.is (or a host under it, live.bible.is) is never read,
  in any language, from any source that says where a document was read
  (FineWeb-2's url, GlotCC's warc-target-uri): its Bibles are its
  holders' and its terms allow personal use only. It was most of
  FineWeb-2's text of dozens of small languages (Garo's in the Bengali
  script all but one page), and fourteen are left with too little to
  learn from (too few words or kinds of word, too little held-out text,
  or too little saved), so their files are not fetched and they have no
  model: Buhid, Ntcham, Northern Grebo, Gogo, Haya, Itawit,
  Xaasongaxango, Kafa, Moba, Nande, Nyoro, Sebat Bet Gurage, Dan and Lahu
  (their settings below are kept; Farefare, Gulay, Koronadal Blaan,
  Lampung Api and Sasak, without a model in data 0.84.0, learn from
  what is left again); a test split none of whose documents is kept
  holds out the train split's instead (`--hold`);

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
Amharic and Tigrinya, which GlotLID mistakes for each other, have each
the other's own function words against it (Amharic's ነው እና ላይ, Tigrinya's
እዩ ኣብ ናይ), their words cut at the Ethiopic wordspace and punctuation
too (`words`) and their sentences ended at the Ethiopic full stop ። and
question mark ፧ or the two wordspaces ፡፡ older text writes for the stop,
a space after them or not (`ends`); every 12th of Amharic's row groups is
read (75 million characters kept) and every 2nd of Tigrinya's (63).
Sebat Bet Gurage's was read so too, whole, against Amharic's function
words (its own ሰብ "person", ኸማ "as", ዝኽ "this"…): its New Testament
(bible.is, never read now), a story and a few articles, and an Amharic
page, which goes; its test split is five documents, so every 10th
document is held out.
Shan, Mon, Sgaw Karen and Pa'O are written in the Myanmar script too,
with no spaces between words, and read whole as Burmese is (Shan's 48
million characters, Mon's 10, Karen's 8, Pa'O's 1.7), their ၀ for ဝ
read as Burmese's is. Their letters are where Zawgyi put its shapes (U+1060
to U+1097), so only ေ before its consonant and ္ ending a syllable mark
a document as Zawgyi's (a third of Pa'O's, Burmese blogs; under 0.3% of
the others'); a sentence whose syllables are more than 4% Burmese's own
(သည် များ တွင် ၏ ပြီး…) is Burmese, and left out. A third of FineWeb-2's
Pa'O is Burmese and Zawgyi's: a Pa'O document writes its tones ႏ and ꩻ,
2% of its letters and more. Karen's older fonts' letters are read as
Unicode's, as its keyboard types them: ၦ as ပှ (ၦၤ, ပှၤ), ၥ as ာ (လံၥ်,
လံာ်), and Shan's tones ႉ and ႋ, written for its full stop, and ႇ, for
its comma, as . and , (a fifth of its sentences). Mon writes its own
nga, ၚ, which its keyboard types, as Burmese's င nearly a quarter of
the time (more than half of the time in its wiki): read as ၚ.
Four of East Asia's languages in Latin letters are read whole: Southern
Min in Pe̍h-ōe-jī, Hakka in Pha̍k-fa-sṳ, Amis, and Standard Zhuang, which
FineWeb-2 files as Yongbei Zhuang's (zyb_Latn). Their Han runs (Hàn-lô
text, a wiki's menus) are left out (`script_only`), Southern Min's ⁿ
being a Latin letter there (`script_also`: Unicode names it a
superscript); a document is kept where its own words are 1% of its
words and more than English's (and Indonesian's, for Hakka, beside
which Indonesian Wiktionary's pages and spam sites are left out; a
parallel corpus's markup beside Amis). Southern Min's i written as the
dotless ı under a tone mark (ı̄ ı̍ ı̀) is read as i, its o͘ written o· as
o͘; Amis' glottal stop, written ’ and ˈ ' ‘ ` ´ ʼ ^, as ’. Hakka's, Amis'
and Zhuang's test splits are small (1 300 to 4 200 words): every 10th
document is held out.
Twenty-eight of Africa's long tail that FineWeb-2 has 0.4 to 0.9 MB of
(Zande, Alur, Chopi, Sakalava Malagasy, Akoose, Mende…) are read whole,
the Bible's and Jehovah's Witnesses' text in the main: a sentence of
English is left out, and of the state's language (French's, by its own
words une pour dans avec, not le; Portuguese's; beside East Africa's
Bantu languages Swahili's own katika kwamba lakini…, not the kwa ni wa
ya they write too); every 10th document is held out. Fante's ͻ (a Greek
look-alike) ɑ ε are read as ɔ a ɛ, Lugbara's ʉ as the ᴜ its text writes
forty times as often, a scan's ﬁ as fi.
Twenty-three more of the long tail are read so too: Southeast Asia's,
the Americas' and the Pacific's that FineWeb-2 has 0.4 to 1.2 MB of
(Niuean, Hawaiian Pidgin, Saint Lucian Creole, Maranao, Ngaju, Wayuu,
Lampung, Sasak, Gorontalo, Itawit, Buhid, Blaan, South Bolivian
Quechua, Guianan Creole) and Africa's with 0.2 to 0.4 MB
(Tumbuka, Tetela, Gusii, Makonde, Nyakyusa, Maasai, Mandinka, Meru,
Dagaare), each against English and its state's language (Indonesian's,
Tagalog's, Spanish's, French's, Portuguese's, Swahili's own words);
Maranao's Arabic (sermons, the Quran) is left out by script, and so
are the LOLcat Bible beside Hawaiian Pidgin and a book
pirate's pages beside Itawit; Meru's ĩ written ῖ is read as ĩ.
Coptic is read in Bohairic's documents, a document being Bohairic's
where its own words (ⲟⲩⲟϩ ⲛⲉⲙ ϧⲉⲛ ⲫϯ…) are 1% of its words and more than
Sahidic's (ⲁⲩⲱ ⲙⲛ̅ ϩⲛ̅…), which write Bohairic's words with the stroke
where Bohairic writes the jinkim; its runs of Coptic letters alone, line
by line (the Arabic, English and French beside them left out). Its
jinkim, written ⳿ or ` or a lone grave before its letter or a dot over
it, is read as the grave after it, its stroke written as the macron as
the overline, ⳪ as ϭ̅ⲥ̅ and ⳾ as a full stop; a number written in letters
under the stroke is left out, and so is a sentence whose Coptic letters
are those of one read before (`same`: a verse's copies on other sites,
in other editions). Lahu's tones are letters of its words
(`script_also`: Unicode names them modifier letters), its ꞈ written as
a caret, ‸, read as ꞈ. Ten more of the long tail with 0.15 to 0.3 MB
(Hunsrik, Plains and Swampy Cree, Ottawa, Tokelauan, Kankanaey, Mamasa,
Livonian, Ambonese Malay, Aja) are read as those before them, against
English (beside the Crees and Ottawa its commonest words too) and their
state's language; Hunsrik's documents in its standard spelling (fon fer
tas…) and not German's; Plains Cree's long vowels written with the
circumflex read with the macron; FineWeb-2's Livonian's Kwak'wala and
Kurdish books, Ambonese Malay's chord sites and Aja's Ewe localisation
of a browser left out.
Thirteen more with 0.1 to 1.4 MB are read so too: Nenets and Dungan in
Cyrillic letters, against Russian's sentences, and Tausug, Ewondo,
Tooro, Makhuwa-Meetto, Kanuri, Giryama, Lango, Hanunoo, Toraja-Sa'dan,
Ledo Kaili and Sassarese against English's and their state's language's
(Tausug's Arabic pages left out by script). A Nenets sentence writes
ӈ or a glottal stop (`letters_some`): its newspaper's Russian
translations write neither. Nenets' glottal stops, written ’ and ’’
(and ʼ, ʼʼ, ”), are read as ’ and ˮ, the letter its keyboard types for
the second: two apostrophes are one to the corrector, which puts back
an apostrophe a word was typed without. Kanuri's ǝ, ә and the digit 3
beside a letter (a radio's site: d3, k3re) are read as ə, Dungan's
Latin ə ǝ as ә, Tausug's saltillo ꞌ as ’, Ewondo's ǝ as ə. Sites in
another tongue are left out: Tooro's magazine scans read by OCR,
Kenyan radios' Kikuyu beside Giryama, another Mangyan language's
recordings beside Hanunoo, Indonesian Wiktionary's Toraja-Sa'dan word
list, English songs in Cyrillic letters beside Dungan.
Vlax and Balkan Romani are each read from both of FineWeb-2's Romani
files, for each holds the other's documents (a fifth of its Balkan
Romani is Vlax) beside Spanish spam and Macedonian in Latin letters (two
thirds and three fifths of their text). A document is the dialect whose
own words it writes more of (`own` against `rival`: Vlax's sas naj ande
kodo…, Balkan's sine taro hem isi…), the spam and the Macedonian writing
neither, and is kept in the spelling of the international standard,
which Serbia's, North Macedonia's and Sweden's Romani text writes, more
of č š ž than of sh ch zh (`foreign`): not the digraphs of Sweden's
radio (6 MB of Vlax) nor Romanian's, Hungarian's, German's or Albanian's
letters, each a country's own; the national languages' sentences inside
it are left out by their words. Northwestern Ojibwe is read in
syllabics, mostly its Bible: a syllable's w-dot written apart before it
(ᐧᐃ) and a syllable with the dot on its right (West Cree's ᐏ) are read
as the one syllable with the dot on its left (ᐎ), as its keyboard types
it and the Bible writes, and its full stop ᙮ ends a sentence.

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

Twenty-six of Europe's regional languages are read whole (Asturian
every other row group, Nynorsk every 28th): Nynorsk, Asturian,
Aragonese, Ladino, Alemannic, Bavarian, Low German, Palatine German,
Kölsch, Limburgish, Upper and Lower Sorbian, Neapolitan, Sicilian,
Sardinian, Friulian, Piedmontese, Ligurian, Walloon, Latgalian, Võro,
Kashubian, Silesian, Crimean Tatar, Scots and Rusyn. Their text mixes
with their national language's, which writes its own words for theirs
(Bokmål's ikke jeg hun, Nynorsk's ikkje eg ho; German's ist nicht,
Bavarian's is ned): a sentence more than 15% the national language's is
left out, or English's, and for the dialects mixed with it most
(Neapolitan and Sicilian with Italian, the German ones with German,
Limburgish with Dutch, Scots with English) a document whose own words
(Neapolitan's nun pe ca chesto, Low German's dat vun nich hett) are
under 1% of its words or not more than the national language's. Much
of their text is their wikis' pages (Asturian's 46%, Low German's 33%),
whose section-editing links ([editar],
[ännern | Bornkood ännern]) go, as does every short span in brackets.
Neapolitan's has Roman's songs and comments in it (er pane, vojo): a
sentence with Roman's article er or quer is left out; Võro's has a
dictionary's entries (s_nimi sg.nom.), and Crimean Tatar's escort spam.
FineWeb-2's Zeelandic is Middle Dutch from text archives, its West
Flemish half Dutch Low Saxon's wiki, its Picard a dictionary's glosses
in French, its Lombard half Ladin, its Gagauz Kazakh and Cyrillic
letters for Latin ones, its Venetian spam: none is read yet.

Fifteen of Russia's and the north's languages are read whole (Sakha's
and Chuvash's every other row group): Sakha, Chuvash, Northern Sami,
Meadow and Hill Mari, Udmurt, Tuvan, Buryat, Komi-Zyrian, Komi-Permyak,
Erzya, Moksha, Kalmyk, Karelian, Livvi and Veps; and twenty-three more
languages that had a keyboard and no model: Greenlandic, Oromo, Romansh,
Kirundi, Luganda, Lingala, Fijian, Ewe, Akan (Twi's text), Tsonga,
Sepedi, Setswana, Tongan, Wolof, Tahitian, Venda, Southern Ndebele,
Aymara, Swati, Quechua (Ayacucho's), Bambara and Inuktitut. A sentence
of the big language written beside them is left out: Russian's beside
Russia's, Norwegian's beside Sami, Danish's beside Greenlandic, German's
beside Romansh, French's beside Lingala, Ewe, Wolof, Tahitian and
Bambara (not la or de, which Wolof and Bambara write), Spanish's beside
Aymara and Quechua, and English's. FineWeb-2 has chat sites' machine-made
pages in the Maris' and Udmurt's name (mrj.videochat.date: half of Hill
Mari's documents): a document read at such an address is left out
(`hosts_out`). Letters typed for others are read as the keyboard types
them: Chuvash's Latin ă ĕ ç ÿ as ӑ ӗ ҫ ӳ (вăл, вӑл), the Komis' ö and i as
ӧ and і, the Maris' ä ö ÿ as ӓ ӧ ӱ, Buryat's and Kalmyk's fita ѳ and Latin
schwas as ө and ә, ӊ as ң, Bambara's Greek ε as ɛ; and a Polynesian glottal
stop written as an apostrophe or a quote before a vowel (fa'a, ‘o) as ʻ,
in Tongan, Tahitian, Hawaiian and Samoan alike. Akan's and Inuktitut's
sentences are cut where a word of another script stands, and a document
of Akan's typed in an old font's encoding, ɛ on j and ɔ on c (yj, sj for
yɛ, sɛ: c and j more than 1% of its letters, which Twi writes neither of),
is left out, as Zawgyi's Burmese is. FineWeb-2's
Nahuatl is three quarters one variety's Bible (Eastern Huasteca's): not
read for the keyboard of Nahuatl at large.

Nineteen of Iran's and the Caucasus' are read whole: South Azerbaijani,
Mazandarani, Kirmanjki and Zazaki, Talysh, Karakalpak (in Latin letters,
its alphabet since 2016), Chechen, Ingush, Avar, Kabardian, Adyghe,
Lezgian, Lak, Tabasaran, Dargwa, Kumyk, Karachay-Balkar, Abkhaz and
Mingrelian. A sentence of the big language written beside each is left
out: Russian's beside the Caucasus', Persian's and Arabic's (in Persian
letters) beside South Azerbaijani and Mazandarani, Turkish's and
Kurmanji's beside the Zazakis (with their viagra spam), Azerbaijani's
beside Talysh, Georgian's beside Mingrelian; Lak's and Tabasaran's news
sites' Russian page furniture (читать далее, опубликовано) is left out,
and a religious-books site (download.ghbook.ir) that FineWeb-2 files under
South Azerbaijani and Mazandarani, mostly Arabic and Persian. The
palochka, Ӏ, which the Caucasus' alphabets write and keyboards without it
type as 1, I, i, l, |, і or ι (д1а, гьечiо), is read as Ӏ beside a
Cyrillic letter: a digit would otherwise cut the word in two. Karakalpak's
letters as its 1994 alphabet wrote them (ha'm) are read as 2016's (hám),
Abkhaz's ҧ and Latin ə as ԥ and ә. FineWeb-2's Southern Kurdish is
Sorani's news, its Gilaki Persian shops' spam and its Gorani mostly
Sorani: none is read.

Twenty-four of Africa's, those of which FineWeb-2 has 2 MB or more, are
read whole: Seychellois Creole, Northern Ndebele, Ndonga, Efik, Sango,
Kwanyama, Tshiluba, Kiluba, Cape Verdean Creole, Mossi, Réunion Creole,
Kikongo, Wolaytta, Dholuo, Tiv, Isoko, Bemba, Fon, Acholi, Baoulé, Jula,
Kikuyu, Kimbundu and Umbundu. A sentence of English is left out, of
French beside those where it is the state's language (not pour, which
Seychellois writes), of Portuguese beside Kimbundu, Umbundu and Cape
Verdean, and of Spanish beside Cape Verdean (not es, its "they"). Most of
their text is the Bible and Watchtower (Kiluba's, Isoko's and Efik's
nearly all), Seychellois Creole's its National Assembly's records, and
Kikuyu's Kenya's news.

Eighteen of the Americas', the Pacific's, and the classical and
constructed languages' are read whole (Latin's every 44th row group,
Papiamento's every 5th): Latin, Papiamento, Classical Syriac (for the
Syriac keyboard, whose keymap is its), Yucatec Maya, Kʼicheʼ, Qʼeqchiʼ,
Cusco Quechua, Kichwa, Sranan Tongo, Guadeloupean Creole, Bislama,
Interlingua, Tok Pisin, Kiribati, Tuvaluan, Hiri Motu, Chamorro and
Jamaican Patois. A sentence of English is left out, of Spanish beside
the Mayan languages and the Quechuas, of Spanish and Dutch beside
Papiamento (not un no por su, which it writes too), of Dutch beside Sranan,
of French beside Guadeloupean Creole. A third of FineWeb-2's Latin is Lorem
ipsum, web pages' placeholder: a document holding it is left out. A Mayan
or Quechuan glottal stop or ejective written as an apostrophe, a quote or
the saltillo after a letter (k'o, saˈ) is read as ʼ, which their keyboards
type; Syriac's vowel points, which its Bibles write and few type, are left
out (not the seyame, which marks a plural). FineWeb-2's Kichwa is a Bible
site's pages, a dozen languages' verses among them: a document is kept
where Kichwa's own words (mana alli chai ima) are 3% of its words, and a
sentence where e and o are no more than a quarter of its vowels (Kichwa
writes a i u), none of Twi's ɛ ɔ is written, and the other languages'
words are under 15%. FineWeb-2's Fiji Hindi is Hindi in Latin letters
(Hinglish, hi-Latn's): not read. Nor is its Lojban, whose short words
differ by a letter (lo le la, ku ke ka): the keyboard's autocorrect
changed 1.5% of the words typed right, with every tap true.

Four keyboards that had no model are read whole too (Serbian's every
18th row group): Serbian in Latin letters, sr-Latn, a document of 300
letters written without č ć š ž đ left out, and a sentence with a
Cyrillic letter or with è æ ð ţ ĉ (an old code page's č ć đ read as
another's); Guaraní, its nasal vowels and g̃ written with a circumflex
or a diaeresis (peteî, peteï, haĝua) read as the tilde its keyboard
types, ´ as ’, and a sentence of Spanish left out (not y, la, para,
which it writes too); Fula, Pular's, Maasina's and the Fulfuldes' text
together, a font's å ð for ɓ ɗ and ň for ñ read as theirs, a sentence of
French left out; and Tamazight in Tifinagh, Central Atlas and Standard
Moroccan Tamazight's text, a run of another script's words cut out and a
sentence with ⵧ ⴵ ⵒ, which its keyboard does not type, left out.

Tibetan and Dzongkha are read whole too (Tibetan's every 4th row group),
a line or a sentence ended by a shad (། ༎) each, a run of another
script's words (Latin, Chinese) cut out, the non-breaking tsheg read as
the tsheg; two fifths of FineWeb-2's Dzongkha is Tibetan (its Tibetan
news sites'), so a document is kept where Dzongkha's own words, counted
in syllables (ཚུ ལུ ཨིན ཨིནམ ཡོདཔ འབད), outnumber Tibetan's (ཚོ རེད འདུག
བྱེད).

Twenty of Africa's and four of Southeast Asia's that FineWeb-2 has 1 to
2 MB of are read whole too, Kabiye's, Minangkabau's and Waray's GlotCC
text beside it: a sentence of English is left out, and of French,
Portuguese, Swahili, Indonesian or Tagalog beside those whose state
writes it, and a wiki's table line (its cells between | and |); Waray's
and Minangkabau's wikis, a bot's, are left out, and Dagbani's spam and
Moba's Romanian and English pages; letters written for others are read
as theirs (Kabiye's ʋ as ʊ, Moba's ɑ as a, Nias' õ as ö, Sidamo's ˈ as
', Kamba's ī ū as ĩ ũ, Dagbani's ε as ɛ, a Yao font's Ć Ǝ as Ŵ);
Dagbani's and Yao's test splits hold a hundred words each, so every 10th
of their documents is held out instead.

Eleven of Europe's regional languages are read whole too (Cornish,
Manx, Extremaduran, Picard, Samogitian, Pontic Greek, Venetian, West
Flemish, Lombard, Ladin, Gagauz): a sentence of English is left out, and
of the state's language beside them by the words they write otherwise
(Spanish's como pero desde, which Extremaduran writes comu peru dendi;
French's, Lithuanian's, standard Greek's, Italian's, Dutch's, German's).
A third of FineWeb-2's Lombard is Ladin, five times what it files as
Ladin: a document is Ladin's where Ladin's words (y dla dl ie nia tl…)
are 2% of its words and more than Lombard's (el del anca minga hinn…),
Lombard's the other way round. Two thirds of its Gagauz is Turkish
typed with Cyrillic letters among the Latin, Azerbaijani and Kyrgyz: a
document is kept where Gagauz's words (hem ani taa nicä deyni…) are 1%
of its words and more than theirs (ve vä menen dep üçün…), and a
sentence with a Cyrillic letter is left out. Sites in another tongue are
left out: Extremaduran's Cantabrian, Leonese, Murcian and A Fala's,
Manx's Voynich manuscript and an English book's scan, Venetian's spam
and old Italian, West Flemish's Low Saxon, Brabantian and old Dutch,
Picard's Norman, Lombard's other Gallo-Italic tongues. Letters written
for others are read as theirs: Gagauz's ȇ ț as ê ţ, Venetian's ƚ as ł,
Pontic's ќ as κ, a scan's ﬁ ﬂ as fi fl.

Each kept document is cut into sentences after a full stop, danda,
shad, question or exclamation mark; a sentence of fewer than three words, of
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
document kept is held out instead (`--hold`). Krio comes from MADLAD-400
too: half of what it files as Krio is Australian Kriol's Bible, so a
document is kept where Krio's own words (dɛn fɔ wetin gɛt) are 3% of its
words and more than Kriol's (langa blanga detlot); ε and כ, written for
ɛ ɔ, are read as them, a parallel corpus's markup lines are left out,
and every 10th document is held out (`hold`). Newari comes from GlotCC
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


END = re.compile(r"(?<=[.!?।॥۔؟།༎])\s+")
# Khmer's and Burmese's ends of sentences, often with no space after them.
UNSPACED_END = re.compile(r"(?<=[។៕။])\s*")
# What marks Burmese text as Zawgyi's: ေ before its consonant, ္ ending a
# syllable, and the shapes Zawgyi put at U+1060 to U+1097.
ZAWGYI = re.compile(r"(?:^|[^\u1000-\u109F])\u1031|\u1039(?![\u1000-\u1021])|[\u1060-\u1097]")
MYANMAR = re.compile(r"[\u1000-\u109F]")
# Myanmar as Shan, Mon, Sgaw Karen and Pa'O write it, Extended-A and -B
# too, and what marks their text as Zawgyi's: ေ before its consonant, ္
# not before one (Zawgyi's asat). Zawgyi's shapes at U+1060 to U+1097 are
# their letters, so they mark nothing. A run of their letters is what
# osk_data::wordbreak cuts (not the digits, Shan's, Tai Laing's, nor ၊ ။).
MYANMAR_ALL = re.compile("[\u1000-\u109F\uA9E0-\uA9FF\uAA60-\uAA7F]")
OWN_ZAWGYI = re.compile("(?:^|[^\u1000-\u109F\uA9E0-\uA9FF\uAA60-\uAA7F])\u1031"
                        "|\u1039(?![\u1000-\u1021\u1050-\u1055\u105A-\u105D\u1061\u1065\u1066"
                        "\u106E-\u1070\u1075-\u1081\u108E])")
MYANMAR_RUN = re.compile("[\u1000-\u103F\u104C-\u108F\u109A-\u109F\uA9E0-\uA9EF\uA9FA-\uA9FF"
                         "\uAA60-\uAA7F\u200C\u200D]+")
# Burmese's own syllables: more than 4% of a sentence's (most Burmese
# sentences' are 8% and more; Shan's, Mon's and Karen's under 0.3%).
BURMESE_ONLY = set("သည် များ တွင် ၏ ပြီး သော ဖြစ် ရှိ နှင့် ခဲ့ လည်း မည် ရန် သို့ တို့ သည့် ကြောင့်".split())
STEERING = re.compile("[‎‏‪-‮⁦-⁩﻿]")
MARKUP = re.compile(r"\[\[|\]\]|\{\{|\}\}")
# A short span in brackets: in a wiki page's text, the link that edits a
# section ([editar], [ännern | Bornkood ännern]).
BRACKETED = re.compile(r"\[[^\[\]\n]{1,60}\]")
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
# Europe's regional languages and their national languages' words, which
# they write their own for (Nynorsk's ikkje eg ho frå for Bokmål's ikke
# jeg hun fra, Bavarian's ned hom for German's nicht haben…): a sentence
# more than 15% the national language's is left out, and for the
# dialects that text mixes most with it, a document whose own words are
# under 1% of its words or not more than the national language's.
BOKMAL_ONLY = set("ikke jeg hun fra hva hvor noen mye hvordan bare".split())
SPANISH_ONLY = set("es está están hay muy también desde sus".split())
ARAGONESE_RIVAL = SPANISH_ONLY | set("los las del".split())
LADINO_RIVAL = set("que con como y porque cuando quien muy".split())
GERMAN_ONLY = set("ist nicht eine einen einem haben auch werden".split())
SORBIAN_RIVAL = set("und die der das ist nicht mit von für sie eine auf".split())
DUTCH_ONLY = set("voor niet naar ook zijn wordt worden hebben heeft".split())
ITALIAN_ONLY = set("il della del per una sono questo anche gli nel alla perché molto".split())
FRENCH_ONLY = set("une pour dans avec nous cette qui".split())
LATVIAN_ONLY = set("arī kā bija vai viņš viņa tas pie ko viņi šo".split())
ESTONIAN_ONLY = set("on ning see oli kui aga või mis seda".split())
POLISH_ONLY = set("się nie jest oraz który która które tylko bardzo też".split())
RUSYN_RIVAL = set("що це від також який яка які что это как или был была".split())
TURKISH_ONLY = set("için olarak değil gibi çok kadar sonra şey bunu olan".split())
SCOTS_RIVAL = set("of with was from have not all were would".split())
SCOTS = set("o tae wi wis frae hae nae aw ken yin twa efter fowk awa doon oot aboot thair maist".split())
NEAPOLITAN = set("nun pe ca na nu cu chesto chillo chella isso essa pecché comme sti stu nuje vuje addò mo tene stanno".split())
SICILIAN = set("lu li ca cu pi nu comu sunnu nun chistu iddu idda ccà ddu dda picchì puru".split())
LIMBURGISH = set("veur vaan neet det nao ouch mit ich waat dao gein ós".split())
BAVARIAN = set("is ned net hod hob hom mia oiso scho aa oan oa nimma dea vo bissl gwen gwesn hoid wos wia".split())
ALEMANNIC = set("isch nid nöd het hei dr vo au uf si gsi üs öppis chunt chönd".split())
LOW_GERMAN = set("dat vun nich hett ok wat weer un sünd ik maakt wedder över ünner".split())
PALATINE = set("unn mer net hot vun uff gewest nix awwer hawwe emol".split())
KOLSCH = set("dat et dä mer op och nit jet ens wat jo sin hät".split())
# English's words these write too, out of what marks English.
ENGLISH_NOT_NORSE = ENGLISH_ONLY - {"for", "by"}
ENGLISH_NOT_GERMANIC = ENGLISH_ONLY - {"was"}
# The big languages written beside the north's and Africa's, Oceania's
# and the Andes': Norwegian's beside Northern Sami, Danish's beside
# Greenlandic, French's beside Lingala, Wolof, Bambara, Ewe and Tahitian
# (not la and de, which Wolof and Bambara write), Spanish's beside Aymara
# and Quechua.
NORWEGIAN_ONLY = set("og ikke jeg er på til som av med det".split())
DANISH_ONLY = set("og er til det af med på som ikke for at en den".split())
FRENCH_SET = set("le les et des une pour dans avec est que qui du au sont pas".split())
SPANISH_SET = set("de la y en el que los del las se por para con un una es".split())
# Sites of machine-made chat-room pages, which FineWeb-2 has in the Maris'
# and Udmurt's name (mrj.videochat.date: two fifths of Hill Mari's text).
CHAT_SPAM = re.compile(r"video-?chat")
# Latin letters typed for Cyrillic ones: Chuvash's ă ĕ ç ÿ for ӑ ӗ ҫ ӳ (вăл,
# вӑл), Komi's ö for ӧ, and i for і within a Cyrillic word.
CHUVASH_FOLD = str.maketrans("ăĂĕĔçÇÿŸ", "ӑӐӗӖҫҪӳӲ")
# And the Maris' ä ö ÿ ü for ӓ ӧ ӱ; the old fita ѳ and Latin schwas ǝ ə
# for Buryat's and Kalmyk's ө ә, and ӊ (en with a hook) for their and
# Tuvan's ң; Greek's ε and Cyrillic's є for Bambara's ɛ; the modifier ˈ
# for Tahitian's ʻ.
MARI_FOLD = str.maketrans("äÄöÖÿŸüÜ", "ӓӒӧӦӱӰӱӰ")
MONGOLIC_FOLD = str.maketrans("ѳѲǝƎəƏӊӉ", "өӨәӘәӘңҢ")
BAMBARA_FOLD = str.maketrans("εΕєЄ", "ɛƐɛƐ")
KOMI_I = re.compile(r"(?<=[\u0400-\u04FF])[iI]|[iI](?=[\u0400-\u04FF])")
# A Polynesian glottal stop written as an apostrophe or a quote mark
# before a vowel (fa'a, fa’a, ‘o for faʻa, ʻo): one letter, ʻ (U+02BB),
# which the keyboard types. A quote before a consonant (’s, ‘Hello) is
# left as it is.
GLOTTAL = re.compile("(?:^|(?<=[\\s(\\[«\"“\\-—–/])|(?<=[^\\W\\d_]))[‘’'`](?=[aeiouāēīōūAEIOUĀĒĪŌŪ])")
# The Caucasus' and Iran's: Persian's own words (not این از در, which
# Mazandarani writes too), Arabic's in Persian letters (فی علی for في على),
# Turkish's beside Zazaki, Azerbaijani's beside Talysh, Georgian's beside
# Mingrelian, and spam's beside Zazaki.
PERSIAN_ONLY = set("است را شده شد بود هستند آنها برای".split())
ARABIC_IN_PERSIAN = set("فی علی أن الذی التی هذا هذه عن إلی الی کان قال".split())
AZERBAIJANI_ONLY = set("üçün ilə olan edir deyil olub".split())
GEORGIAN_ONLY = set("და არის რომ ეს იყო მისი ასევე".split())
SPAM = set("viagra cialis http https www url casino porn".split())
# The palochka, Ӏ (U+04C0), which the Caucasus' Cyrillic alphabets write
# (Chechen's дӀа, Avar's гьечӀо) and keyboards without it type as 1, I, i,
# l, |, і or Greek's ι (д1а, гьечiо): beside a Cyrillic letter, it is Ӏ. A
# digit would otherwise cut the word in two.
PALOCHKA = re.compile("(?<=[\u0400-\u04FF])[1Iil|\u0406\u0456\u04CF\u03B9](?=[\u0400-\u04FF])"
                      "|(?<![\\w])[Ii|\u0406\u0456\u04CF\u03B9](?=[\u0430-\u044F\u0451])"
                      "|(?<=[\u0400-\u04FF])[Iil|\u0406\u0456\u04CF\u03B9](?![\\w])")
# Kurmanji's words that Zazaki writes its own for (ji li ku for ra de ke):
# a sentence of Kurmanji in Zazaki's text. Not di, Zazaki's two.
KURMANJI_ONLY = set("ji li ku jî dike dikin heye hene bû".split())
# A site of religious books in Arabic and Persian, which FineWeb-2 files
# under South Azerbaijani and Mazandarani as well (download.ghbook.ir).
RELIGIOUS_BOOKS = re.compile(r"ghbook")
# Abkhaz's pe written as Cyrillic's ҧ, its schwa as Latin's ə.
ABKHAZ_FOLD = str.maketrans("ҧҦəƏ", "ԥԤәӘ")
# Karakalpak's letters as its 1994 alphabet wrote them, a letter and an
# apostrophe (ha'm, a', g', n'), as its alphabet of 2016 writes them (hám);
# not an English possessive's 's.
KARAKALPAK_LETTERS = {"a": "á", "o": "ó", "u": "ú", "g": "ǵ", "n": "ń", "i": "ı",
                      "A": "Á", "O": "Ó", "U": "Ú", "G": "Ǵ", "N": "Ń", "I": "I"}
KARAKALPAK_OLD = re.compile("([aouginAOUGIN])['’ʼ‘`](?=[^\\W\\d_])(?!s\\b)")
# Africa's: Spanish's words that Cape Verdean Creole does not write (not
# es, its "they"), beside Portuguese's.
CAPE_VERDE_RIVAL = PORTUGUESE_ONLY | set("el los las del con por una para que".split())
# The Americas', the Pacific's and the classical languages': Spanish's
# and Dutch's words that Papiamento writes its own for (y, el, que, het
# for i, e, ku, e; not un no por su, which it writes too); Lorem ipsum,
# which web pages hold for text to come, in FineWeb-2's Latin.
PAPIAMENTO_RIVAL = set("y el los las del que con para una es het niet zijn een van voor".split())
LOREM = set("lorem consectetur adipiscing eiusmod incididunt".split())
# Kichwa's own words: FineWeb-2's Kichwa is a Bible site's pages, some of
# them in other languages (Twi's, Swedish's, Portuguese's, Spanish's,
# Swahili's),
# which have none; a sentence with a letter Kichwa does not write (Twi's ɛ
# ɔ), more than 15% the other languages' words, or e and o more than a
# quarter of its vowels (Kichwa writes a i u: Italian's di che da) is
# another's too.
SWEDISH_ONLY = set("och att som det är jag inte för med på".split())
SWAHILI_ONLY = set("kwa ni wa ya kuwa kama yake za hii".split())
KICHWA = set("mana alli chai ima chashna shina cai ñuca pai tucui jatun taita shuc can cashpa nishpa".split())
# A Mayan or Quechuan glottal stop or ejective written after a letter as
# an apostrophe, a quote or the saltillo ꞌ (k'o, saˈ, ti’): one letter, ʼ
# (U+02BC), which
# the keyboard types.
EJECTIVE = re.compile("(?<=[^\\W\\d_])['’ˈ‘`\uA78C]")
# Syriac's vowel points and the marks beside them (U+0730 to U+074A), which
# its Bibles write and few type; not the seyame (U+0308), which marks a
# plural in unpointed text too.
SYRIAC_POINTS = re.compile("[\u0730-\u074A]")
# Serbian in Latin letters: the letters of its alphabet that ASCII has
# not, which a document written without them lacks ("shaved" Latin,
# kuca for kuća); what an old code page's č ć đ read as another's are (è
# æ ð, ţ ĉ); and Cyrillic, which a sentence of the Latin keyboard's text
# does not write.
SERBIAN_DIACRITICS = set("čćšžđČĆŠŽĐ")
MOJIBAKE = set("èæðţĉÈÆÐŢĈ")
CYRILLIC_LETTERS = {chr(c) for c in range(0x0400, 0x0530)}
# Guaraní's nasal vowels and g̃, which its text writes with a tilde, a
# circumflex or a diaeresis (peteĩ, peteî, peteï; hag̃ua, haĝua), read as
# the tilde, as its keyboard types them; its glottal stop written ´ read
# as ’. Spanish's words that Guaraní does not write (not y, water, la, which
# its spoken Spanish lends it, para, sea), what marks a sentence of Spanish.
GUARANI_FOLD = str.maketrans({
    "â": "ã", "ê": "ẽ", "î": "ĩ", "ô": "õ", "û": "ũ", "ŷ": "ỹ",
    "ä": "ã", "ë": "ẽ", "ï": "ĩ", "ö": "õ", "ÿ": "ỹ",
    "Â": "Ã", "Ê": "Ẽ", "Î": "Ĩ", "Ô": "Õ", "Û": "Ũ", "Ŷ": "Ỹ",
    "Ä": "Ã", "Ë": "Ẽ", "Ï": "Ĩ", "Ö": "Õ", "Ÿ": "Ỹ",
    "ĝ": "g\u0303", "Ĝ": "G\u0303", "\u00B4": "\u2019",
})
GUARANI_RIVAL = (SPANISH_SET - {"y", "la", "para"}) | ENGLISH_ONLY
# Fula's: a Christian site's font that puts ɓ ɗ on å ð, and ñ written ň.
FULA_FOLD = str.maketrans("åÅðÐňŇ", "ɓƁɗƊñÑ")
# Krio's own words, and Australian Kriol's, whose Bible MADLAD-400 files
# with Krio's text (langa blanga detlot: as much as Krio's); ε and כ
# written for ɛ ɔ. Some of its documents are a parallel corpus's lines,
# their markup (src)="7"> and (trg)="14"> a sentence's words src and trg.
KRIO = set("dɛn fɔ wetin gɛt bɔt nɔ ɛn pipul mɔ dɔn tɛl lɛ fɔs sɛf ɛvri bɛtɛ".split())
KRIOL = set("langa blanga detlot garra olabat melabat imbin deibin yumob mela gudbala".split())
KRIO_FOLD = str.maketrans("εΕכ", "ɛƐɔ")
# MADLAD-400's Tandroy-Mahafaly Malagasy, Hawu, Kituba and Ayacucho
# Quechua: their Bibles and Jehovah's Witnesses' pages, and in their noisy
# files as much spam and English. A document is kept where its own words
# are a share of its words: Tandroy's ty amy zay laha lafa (standard
# Malagasy's ny amin'ny izay raha rehefa), Hawu's do nga ri ddau dhara,
# Kituba's yandi mono yina mpi sambu (Lingala's ezali mpe oyo bato kasi),
# Ayacucho Quechua's mana chay hinaspa nispa (with no ejective, which it
# does not write). Hawu's glottal stop is the saltillo ꞌ (U+A78C), which
# its Bible writes; an apostrophe written between letters is read as it.
TANDROY = set("ty amy tamy zay laha lafa olo avao baka iaby".split())
MALAGASY_STANDARD = set("ny amin'ny tamin'ny izay raha rehefa olona avy rehetra ireo".split())
HAWU = set("do nga ri ddau dhara jhe dhèu harre dènge lii lua".split())
KITUBA = set("yandi mono yina mpi sambu beno nde vandaka ikele kele bantu ntangu yonso kansi".split())
LINGALA = set("ezali mpe oyo bato kasi lokola elongo nyonso".split())
AYACUCHO = set("mana chay hinaspa nispa manam allin nirqa ichaqa kay tukuy chaymi hina huk kaq".split())
SALTILLO = re.compile(r"(?<=\w)['’](?=\w)")
# Tibetan and Dzongkha: a syllable, the run of letters and marks between
# its tsheg and the one before (what the own and rival words are counted
# in); Dzongkha's own words that Tibetan writes otherwise (its plural ཚུ,
# Tibetan's ཚོ; its dative ལུ, Tibetan's ལ; its copulas ཨིན ཨིནམ, Tibetan's
# རེད; ཡོདཔ, Tibetan's ཡོད་པ; འབད, do, Tibetan's བྱེད), and Tibetan's, which
# a third of FineWeb-2's Dzongkha is (its Tibetan news sites); and the
# non-breaking tsheg, read as the tsheg.
SYLLABLE = re.compile("[\u0F40-\u0FBC]+")
# Africa's and Southeast Asia's long tail. Letters written for others:
# Kabiye's ʋ for its ʊ (its Wikipedia writes ʊ, jw.org ʋ), Moba's ɑ for a
# (its Bibles write the same words with either: nba and nbɑ), Nias' õ for
# ö, Sidamo's ˈ for its glottal stop ', Kamba's ī ū for ĩ ũ, Dagbani's ε for
# ɛ, and a Yao font's Ć and Ǝ for Ŵ (achiĆalimu, achiŵalimu). Hosts whose
# pages are another language's: Dagbani's spam, Moba's Romanian and
# English documents; and the wikis a bot wrote, Waray's and
# Minangkabau's, their species and places in one sentence each. A
# table's line in a wiki's text (its cells between | and |) is no
# sentence.
LONG_TAIL_FOLD = {
    "kbp": str.maketrans("ʋƲ", "ʊƱ"),
    "mfq": str.maketrans("ɑⱭ", "aA"),
    "nia": str.maketrans("õÕ", "öÖ"),
    "sid": str.maketrans({"\u02C8": "'"}),
    "kam": str.maketrans("īūĪŪ", "ĩũĨŨ"),
    "dag": str.maketrans("εΕ", "ɛƐ"),
    "yao": str.maketrans("ćĆǝƎ", "ŵŴŵŴ"),
    "gag": str.maketrans("ȇȆțȚ", "êÊţŢ"),
    "vec": str.maketrans("ƚȽ", "łŁ"),
    "pnt": str.maketrans("ќЌ", "κΚ"),
    # Fante's ɔ written as a Greek look-alike, ͻ, its a as ɑ, its ɛ as ε;
    # Lugbara's ʉ, the vowel its text writes ᴜ forty times more often.
    "fat": str.maketrans("ͻϽɑⱭεΕ", "ɔƆaAɛƐ"),
    "lgg": str.maketrans({"\u0289": "\u1D1C"}),
    # Meru's ĩ written as Greek's ῖ.
    "mer": str.maketrans("ῖ", "ĩ"),
    # Plains Cree's long vowels, written with the macron fifteen times as
    # often as with the circumflex, with it; the hyphen after its preverbs
    # written as the non-breaking one (ē‑wī-). Livonian's ț written with the
    # cedilla, Aja's ʒ as Cyrillic's ӡ.
    "crk": str.maketrans({"â": "ā", "ê": "ē", "î": "ī", "ô": "ō", "Â": "Ā", "Ê": "Ē", "Î": "Ī", "Ô": "Ō",
                          "‑": "-"}),
    "csw": str.maketrans({"‑": "-"}),
    "liv": str.maketrans("ţŢ", "țȚ"),
    "ajg": str.maketrans("ӡӠ", "ʒƷ"),
    # Tausug's glottal stop, the Bible's saltillo ꞌ, as the ’ its other
    # sites write; Ewondo's ə written as the turned e ǝ.
    "tsg": str.maketrans("\uA78C\uA78B", "\u2019\u2019"),
    "ewo": str.maketrans("\u01DD\u018E", "\u0259\u018F"),
    # A scanned magazine's ligatures.
    "skg": str.maketrans({"\uFB01": "fi", "\uFB02": "fl"}),
    "lld": str.maketrans({"\uFB01": "fi", "\uFB02": "fl"}),
    "lmo": str.maketrans({"\uFB01": "fi", "\uFB02": "fl"}),
}
SPAM_HOSTS = re.compile(r"ghanabuys|chegg|statoperator|lyrics")
MOBA_OUT = re.compile(r"scribd|newspaperarchive|etudier|buenastareas")
BOT_WIKI = re.compile(r"wikipedia\.org")
# Europe's long tail. Ladin's words and Lombard's: FineWeb-2's Lombard
# is a third Ladin (La Usc di Ladins on issuu, the province's pages).
LADIN = set("y dla dl ie nia tl tla plu nce sce chësc chësta dles fova unì ince vën iel sciche pra danter "
            "coche ova lingaz".split())
LOMBARD = set("el del anca minga staa pussee hinn ona ghe gh quell chì faa ind soa dent tucc poeu sò alter "
              "quand".split())
# Lombard's sites in other Gallo-Italic tongues (Trentino's, Emilian's,
# the Marche's, Piedmont's Occitan) and junk.
LOMBARD_OUT = re.compile(r"prourbino|televignole|albarnardon|tuenetto|dialettando|chambradoc|"
                         r"mapandwordsstartingwithf|docplayer")
# Gagauz's words, and those of Turkish, Azerbaijani and Kyrgyz that it
# does not write.
GAGAUZ = set("hem ani taa nicä deyni lääzım onnar diil angısı angı bölä büük eni hepsi sora".split())
GAGAUZ_RIVAL = set("ve vä və menen dep ele jana üçün bolup emes değil olarak ilə".split())
# The state's language's words that the regional one writes otherwise:
# standard Greek's beside Pontic's σο σην έν' κ' ντο, Lithuanian's beside
# Samogitian's ėr īr ka bova, Spanish's beside Extremaduran's comu peru
# esti pa mu tamién dendi, French's beside Picard's el in dins su aveuc.
GREEK_ONLY = set("του της στο στην στη στον στα στις στους είναι δεν που αυτό αυτή όπως μπορεί πολύ".split())
LITHUANIAN_ONLY = set("ir yra kad buvo su kaip taip jis jie kuris kuri apie labai arba".split())
EXTREMADURAN_RIVAL = set("como pero todo este para muy también desde cuando porque donde otro mismo ellos puede "
                         "sobre".split())
PICARD_RIVAL = set("le une en je ce dans sur avec son sa elle cette être très aussi même bien rien".split())
# Italian's, less the words Venetian and Lombard write as it does.
VENETIAN_RIVAL = ITALIAN_ONLY - {"per", "del", "una"}
# Sites in another tongue, or none: Cantabrian's, Leonese's, Asturian's,
# Murcian's, Andalusian's and A Fala's (Galician-Portuguese), and
# scans, beside Extremaduran; a Voynich
# manuscript's transcription and an English book's scan beside Manx;
# Norman beside Picard; spam, song sites, old Italian, Ligurian and
# tables beside Venetian; the Netherlands' sites (Low Saxon, old Dutch),
# Brabantian and the Dutch Low Saxon wiki beside West Flemish.
EXTREMADURAN_OUT = re.compile(r"cantabria|iventia|cicuiro|cabreires|corienses|vivirasturias|murciano|"
                              r"lalenguadeandalucia|sierradegatadigital|valverdidufresnu|scribd")
MANX_OUT = re.compile(r"archive\.org|fmjlang")
PICARD_OUT = re.compile(r"bmlisieux")
VENETIAN_OUT = re.compile(r"cerveceria-victoria|eurekamovil|acornandoaktree|wanderlog|firstratetailgate|araldica|"
                          r"classicitaliani|unipi|italytravelescape|open-closed|tripadvisor|easyviaggio|istat|"
                          r"books\.google|lij\.wikisource|uni-giessen|radio\.uol|angolotesti|lyrics")
FLEMISH_OUT = re.compile(r"^https?://[^/]*\.nl(?:[:/]|$)|nds-nl\.|dbnl|kuleuven|books\.google|docstoc|"
                         r"bruxelloises|leveninleuven|seniorennet|aentwaereps")
DZONGKHA = set("ཚུ ལུ ཨིན ཨིནམ ཡོདཔ འབད མེདཔ བཏུབ ནིའི འབདཝ".split())
TIBETAN = set("ཚོ རེད འདུག བྱེད".split())
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


def myanmar_syllables(text):
    """The syllables of `text`'s runs of Myanmar letters, cut as
    osk_data::wordbreak cuts Shan's, Mon's and Pa'O's, and Burmese's
    sentences, which they are counted to find, are cut: one starts on a
    letter, but on one stacked under the one before (after ္) or one the
    asat after it ends a syllable with."""
    out = []
    for m in MYANMAR_RUN.finditer(text):
        run, start = m.group(), 0
        for i in range(1, len(run)):
            after = run[i + 1] if i + 1 < len(run) else ""
            if (unicodedata.category(run[i])[0] != "M" and run[i - 1] != "\u1039"
                    and run[i] not in "\u200c\u200d" and after != "\u103a"):
                out.append(run[start:i])
                start = i
        out.append(run[start:])
    return out


# East Asia's in Latin letters: Southern Min's words in Pe̍h-ōe-jī,
# Hakka's in Pha̍k-fa-sṳ, Amis', Zhuang's (Standard Zhuang, which
# FineWeb-2 files as Yongbei Zhuang's).
SOUTHERN_MIN = set("ê sī ū tī lâng lâi bô kóng kap lán beh koh lí chit góa lóng tùi chiū kā teh kàu mā iā".split())
HAKKA = set("ke he chhai lâu kì yû kóng oi ngài ngì voi yung tên chhiu pûn tui".split())
AMIS = set("ko ato saka ira ci itini ano caay sowal kitakit tamdaw miheca nina namo iso mato".split())
ZHUANG = set("gij caeuq aen dwg miz bae aeu guh mbouj youq ndaej haenx neix vunz daengz ndaw boux hawj "
             "ndei".split())
# Southern Min's i, written as the dotless ı under its tone marks (ı̄ ı̍
# ı̀), and its o͘ written o·.
DOTLESS_UNDER_A_MARK = re.compile("\u0131(?=[\u0300-\u036F])")
# Amis' glottal stop, written ’ most of the time, ˈ ' ‘ ` ´ ʼ and ^ (the
# official spelling's epiglottal, written for the same words) the rest.
AMIS_STOP = re.compile(r"(?<=[^\W\d_])[\u02C8'\u2018`\u00B4\u02BC^]|[\u02C8\u2018`\u00B4\u02BC^](?=[^\W\d_])")
HAKKA_OUT = re.compile(r"wiktionary|sio\.si|bashgmu|politicalfriendster")
AMIS_OUT = re.compile(r"opus\.nlpl")


def southern_min(text):
    text = DOTLESS_UNDER_A_MARK.sub("i", text).replace("o\u00B7", "o\u0358").replace("O\u00B7", "O\u0358")
    return unicodedata.normalize("NFC", text)


def hakka(text):
    return unicodedata.normalize("NFC", text)


def amis(text):
    return AMIS_STOP.sub("\u2019", unicodedata.normalize("NFC", text))


# Swahili's own words, which the Bantu languages beside it do not write
# (its kwa, ni, wa and ya they do).
SWAHILI_RIVAL = set("katika kwamba lakini pamoja baada sana hii yake".split())


# Hawaiian Pidgin's English look-alikes: the LOLcat Bible and its kin.
LOLCAT_HOSTS = re.compile(r"lolcat|catsrgreen|layonara")
BOOK_PIRATES = re.compile(r"b-ok\.")


# Coptic's letters (its own block's and the seven of the Greek block's,
# ϣ to ϯ). Its jinkim, the grave over a letter that is a syllable of its
# own (ⲛ̀ⲧⲉ), is written as Unicode writes it, after its letter: the
# Bohairic New Testament on the web writes it ⳿ before the letter (⳿ⲛⲧⲉ),
# others a backtick or a lone grave before it, or a dot over it; the
# supralinear stroke (ⲛ̅, Ⲓⲏ̅ⲥ̅), the overline 300 times as often as the
# macron, as the overline. A number written in letters under the stroke
# (ⲁ̅, ⲓ̅ⲃ̅: the Bible's verses) is no word, and the full stop ⳾ is one.
# The Bohairic New Testament's ⳪, the sign for ϭⲟⲓⲥ (Lord: Ⲡ⳪), is a
# symbol to Unicode, which a word does not hold: it is read as the nomen
# sacrum the others write, ϭ̅ⲥ̅.
COPTIC_LETTERS = "\u2C80-\u2CE4\u03E2-\u03EF"
JINKIM_BEFORE = re.compile(f"(?:[\u2CFF`]|(?<![{COPTIC_LETTERS}\u0300-\u036F])\u0300)([{COPTIC_LETTERS}])")
DOT_JINKIM = re.compile(f"(?<=[{COPTIC_LETTERS}])\u0307")
COPTIC_NUMBER = re.compile(f"(?<!\\S)(?:[{COPTIC_LETTERS}]\u0305)+(?!\\S)")
# The Gospels' Bohairic and Sahidic side by side in a font's letters (Ǹ
# for ⲛ̀), papyri's and dictionaries' text.
COPTIC_OUT = re.compile(r"cld\.bz|papyri\.info|coptic-dictionary|cenob\.org|multipal")
# Sylheti in Syloti Nagri: FineWeb-2 files it as und_Sylo, a news site's
# pages (surmafarorkhobor.com, 3 404 of its 4 881 documents) and the
# Incubator's copies of the Sylheti wiki, whose own text its model learns
# from already (and whose held-out articles they would leak), and the
# character tables beside them: those left out. Its sentences end with ⁕
# and its poetry marks as well as the danda.
SYLOTI_OUT = re.compile(r"wikimedia\.org|unicode|keyboard\.cool|forvo")
# Nuosu's Yi: its stops, Chinese's (no space after them); a syllable, each
# a word as osk_data::wordbreak cuts it; and its commonest syllables, of
# the twenty commonest in both its Incubator wiki's text and People's
# Daily's Yi pages. A page of the web's character tables (keyboard.cool,
# unicode-table.com, the fancy-text sites) or of syllables strung at
# random writes 1 to 3% of its syllables with them, a page of Nuosu 15 to
# 30% (People's Daily's 21 to 26%). The Incubator's pages are its wiki's
# copies, whose held-out articles they would leak.
YI_END = re.compile(r"(?<=[。！？])\s*")
YI_SYLLABLE = re.compile("[\ua000-\ua48c]")
NUOSU = set("ꃅ ꌠ ꇬ ꆀ ꄉ ꏦ ꇩ ꂷ ꀋ ꆏ ꅉ ꍏ".split())
SYLOTI_END = re.compile(r"(?<=[⁕꠨꠩꠪꠫])\s*")
# Bohairic's own words (and, with, in, God) and Sahidic's, by which a
# document is Bohairic's: Sahidic writes the words Bohairic does (ⲛ̀ⲧⲉ,
# ⲛ̅ⲧⲉ) with its stroke where Bohairic writes the jinkim, and beside it
# autocorrect changed one Bohairic word typed right in 27.
BOHAIRIC = set("ⲟⲩⲟϩ ⲛⲉⲙ ϧⲉⲛ ⲫϯ ⲉⲑⲃⲉ ⲛⲓⲃⲉⲛ ⲛ̀ⲧⲉ ⲉ̀ⲃⲟⲗ ⲉⲃⲟⲗϧⲉⲛ".split())
SAHIDIC = set("ⲁⲩⲱ ⲙⲛ ϩⲛ ϩⲙ ⲙⲛ̅ ϩⲛ̅ ϩⲙ̅ ⲡⲛⲟⲩⲧⲉ ⲉⲧⲃⲉ ⲛ̅ⲧⲉ".split())


def coptic(text):
    text = JINKIM_BEFORE.sub("\\1\u0300", unicodedata.normalize("NFC", text))
    text = DOT_JINKIM.sub("\u0300", text).replace("\u0304", "\u0305").replace("\u2CFE", ".")
    # A jinkim written both ways at once (`\u2C9B\u0300) is one.
    text = re.sub("\u0300{2,}", "\u0300", text)
    return COPTIC_NUMBER.sub(" ", text).replace("\u2CEA", "\u03ED\u0305\u2CA5\u0305")


def coptic_letters(s):
    """The Coptic letters of `s` in lower case: what a verse's copies on two
    sites, or in two editions, share where their marks, spaces and numbers
    differ."""
    return "".join(c for c in s.lower() if c.isalpha() and "COPTIC" in unicodedata.name(c, ""))


# Lahu's tones, written after a syllable as letters Unicode names
# modifier letters (ˬ ˇ ˍ ˆ ˉ ꞈ), the low circumflex ꞈ as a caret, ‸, on
# Bible.is (nine tenths of its text).
LAHU_TONES = frozenset("\u02EC\u02C7\u02CD\u02C6\u02C9\uA788")


def lahu(text):
    return text.replace("\u2038", "\uA788")


# The long tail's last: Hunsrik's word lists; FineWeb-2's "Livonian" from
# a Kwak'wala prayer book and a Kurdish one; Ambonese Malay's song pages,
# their chords between its words; Ewe's localisation of a browser beside
# Aja.
MEMRISE = re.compile(r"memrise")
# Hunsrik is written two ways: in its standard spelling, ESCRITHU's
# (fon, fer, tas, sayn, mënxe, ix), and in German's (von, unn, ich, schun),
# a blog's; the standard's documents are read. Portuguese's sentences
# among them are left out, by its words Hunsrik does not write.
HUNSRIK = set("fon fer tas ti sayn wayl hon kot tuu woo uns aach keen nët ix".split())
HUNSRIK_GERMAN = set("von unn nit sinn isch gewest schun nix schon ich sich mich dich nicht und".split())
HUNSRIK_RIVAL = (PORTUGUESE_ONLY - {"do"}) | set("para com foi ao mas como era foram muito também ele ela eu "
                                                  "meu nós você mais isso este esta".split())
LIVONIAN_OUT = re.compile(r"anglican|wikibooks")
CHORDS = re.compile(r"chord")
MOZILLA = re.compile(r"mozilla")


# The long tail's rest, those with 0.1 to 1.4 MB of FineWeb-2's text.
# Nenets writes its two glottal stops after their letter, ʼ and ˮ (поʼ,
# ненэцяˮ), mostly as ’ and ’’, and as ” too: read as ’ and ˮ, the
# letter its keyboard types for the second (two apostrophes are one to
# the corrector, which puts back an apostrophe a word was typed without:
# beside ’’ it changed one Nenets word typed right in 80, ненэцие’ to
# ненэцие’’). Its ӈ written as Kazakh's ң is read as ӈ. Its newspaper
# puts a Russian translation after each article, left out by Russian's
# words (not по: Nenets' по’, cut at its apostrophe, is one) and by its
# writing neither ӈ nor a glottal stop; a news site writes its ӈ as ?,
# and Forest Nenets' text (its ԓ) is another's.
NENETS_STOPS = re.compile("(?<=[\u0400-\u04FF])(?:\u201D|[\u2019\u02BC']{2})")
NENETS_FOLD = str.maketrans("\u04A3\u04A2", "\u04C8\u04C7")
NENETS_OUT = re.compile(r"nao-news")
# What nearly every Nenets sentence writes and Russian's never do: a
# glottal stop or ӈ (a sentence with none is Russian 49 times in 50).
NENETS_MARKS = frozenset("\u2019\u02EE\u04C8\u04C7")


def nenets(text):
    return NENETS_STOPS.sub("\u02EE", text).replace("\u02BC", "\u2019").translate(NENETS_FOLD)


# Kanuri's ə, written as the turned e ǝ, Cyrillic's ә, and on a radio's
# site as the digit 3 beside a letter (d3, k3re): as ə. Dungan's ә
# written as Latin's ə or ǝ; a site's English songs written in Cyrillic
# letters beside it.
KANURI_THREE = re.compile("(?<=[A-Za-z\u0259\u01DD])3|3(?=[A-Za-z\u0259\u01DD])")
KANURI_FOLD = str.maketrans("\u01DD\u018E\u04D9\u04D8", "\u0259\u018F\u0259\u018F")
DUNGAN_FOLD = str.maketrans("\u0259\u01DD\u018F\u018E", "\u04D9\u04D9\u04D8\u04D8")
SONG_SCRIPTS = re.compile(r"transkriptsiya")


def kanuri(text):
    return KANURI_THREE.sub("\u0259", text).translate(KANURI_FOLD)


def dungan(text):
    return text.translate(DUNGAN_FOLD)


# Romani's two, Vlax (rmy) and Balkan (rmn), are written in each country's
# spelling: with č š ž (the international standard's, which Serbia's,
# North Macedonia's and Sweden's Romani text writes), with digraphs as
# English and Swedish spell (sh ch zh), with Romanian's, Hungarian's,
# German's or Albanian's letters. A document is kept in the first: more
# of č š ž than of sh, ch and zh. FineWeb-2's file of each holds the
# other's documents too (Vlax Romani is a fifth of its Balkan Romani's),
# so both files are read for each and a document is the dialect whose own
# words it writes more of: Vlax's sas (was), naj (is not), ande ando andi
# (in the), kodo kado kava (this); Balkan's sine (was), taro tari tare
# (from the), hem (and), isi (there is), nane (is not), keda (when),
# odova adava akava (this). The spam and the Macedonian beside them write
# neither; the national languages' sentences inside a document are left
# out by their words.
ROMANI_VLAX = set("sas naj ande ando andi kodo kado kava".split())
ROMANI_BALKAN = set("sine taro tari tare hem isi nane keda odova adava akava".split())
ROMANI_CARONS = re.compile("[čšžČŠŽ]")
ROMANI_DIGRAPHS = re.compile("(?i)sh|ch|zh")
MACEDONIAN_LATIN = set("vo se za od deka kako toa gi go ili koj koja koi site mnogu tie ovaa kade što sum nie".split())
SERBIAN_LATIN = set("je u kao koji što su biti bio nije sve".split())
ROMANIAN_ONLY = set("și şi în cu să nu este sunt care pentru".split())
SPANISH_SPAM = SPANISH_ONLY | set("el en los con para por que una prostitutas putas porno videos sexo gratis".split())
ROMANI_RIVAL = (ENGLISH_ONLY | SWEDISH_ONLY | MACEDONIAN_LATIN | SERBIAN_LATIN | ROMANIAN_ONLY
                | SPANISH_SPAM | GERMAN_ONLY)


def romani_digraphs(text):
    """A Romani document spelled otherwise than with č š ž."""
    return len(ROMANI_CARONS.findall(text)) <= len(ROMANI_DIGRAPHS.findall(text))


# Northwestern Ojibwe's syllabics (ojb_Cans: the Bible, most of it), each
# syllable as one character, as its keyboard types it: the w-dot written
# apart before its syllable (ᐧᐃ, U+1427) and a syllable with the dot on
# its right (West Cree's ᐏ) read as the syllable with the dot on its
# left (ᐎ), as the Bible writes; its full stop ᙮ ends a sentence, as a
# full stop does.
def _syllable(name):
    try:
        return unicodedata.lookup("CANADIAN SYLLABICS " + name)
    except KeyError:
        return None


def _w_dotted():
    dotted, west = {}, {}
    for c in map(chr, range(0x1400, 0x1680)):
        name = unicodedata.name(c, "").removeprefix("CANADIAN SYLLABICS ")
        if name.startswith("WEST-CREE ") and _syllable(name.removeprefix("WEST-CREE ")):
            west[c] = _syllable(name.removeprefix("WEST-CREE "))
        m = re.fullmatch(r"(P|T|K|C|M|N|S|SH|Y|L|R)?(E|I|II|O|OO|A|AA)", name)
        if m and _syllable((m.group(1) or "") + "W" + m.group(2)):
            dotted["\u1427" + c] = _syllable((m.group(1) or "") + "W" + m.group(2))
    return dotted, west


OJIBWE_DOTTED, OJIBWE_WEST = _w_dotted()
OJIBWE_DOT = re.compile("\u1427[\u1400-\u167F]")
OJIBWE_WEST_FOLD = str.maketrans(OJIBWE_WEST)


def ojibwe(text):
    text = OJIBWE_DOT.sub(lambda m: OJIBWE_DOTTED.get(m.group(0), m.group(0)), text)
    return text.translate(OJIBWE_WEST_FOLD).replace("\u166E", ".")


# Pages in another tongue: Tooro's magazine scans read by OCR, Kenyan
# radios' Kikuyu beside Giryama, another Mangyan language's recordings
# and a Tagalog blog beside Hanunoo, Indonesian Wiktionary's word list of
# Toraja-Sa'dan.
SCANS = re.compile(r"issuu")
KENYAN_RADIOS = re.compile(r"radiokimuri|rmsradio")
HANUNOO_OUT = re.compile(r"globalrecordings|stephencuyos")
WIKTIONARY = re.compile(r"wiktionary")


# Sgaw Karen's older fonts' letters, as Unicode writes them.
KAREN_FOLD = str.maketrans({"\u1066": "\u1015\u103E", "\u1065": "\u102C",
                            "\u1089": ".", "\u108B": ".", "\u1087": ","})


def karen(text):
    return burmese(text).translate(KAREN_FOLD)


def mon(text):
    # Mon's nga, written as Burmese's nearly a quarter of the time.
    return burmese(text).replace("\u1004", "\u105A")


def own_zawgyi(text):
    """Shan's, Mon's, Karen's or Pa'O's text in Zawgyi's encoding."""
    return len(OWN_ZAWGYI.findall(text)) > 0.003 * max(1, len(MYANMAR_ALL.findall(text)))


def not_pao(text):
    """Zawgyi's text, or Burmese: Pa'O's tones ႏ and ꩻ under 2% of its
    letters."""
    tones = text.count("\u108F") + text.count("\uAA7B")
    return own_zawgyi(text) or tones < 0.02 * max(1, len(MYANMAR_ALL.findall(text)))


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


# Gorani (Hawrami): FineWeb-2's hac_Arab is mostly Sorani and Badini
# Kurdish and Persian; a document is Hawrami where its own function
# words, those its sites' pages write and Sorani's do not (hewraman.net,
# horam.info, hawramannews.com and ANF's Hawrami pages against Sorani
# news: جە in 93% of them and 26%, پەی 73% and 18%, ئی 60% and 13%),
# outnumber Sorani's, Badini's and Persian's (لە in 77% of Sorani's
# pages and 6% of Hawrami's).
HAWRAMI = set("جە پەی چەنی ئی بیەن پەنە دماو فرە وێش کەردەن ئانەی لاو".split())
SORANI_WORDS = set("لە لەسەر لەو ئەم بۆ دەکات دەکرێت بوو دەبێت لەگەڵ".split())
BADINI_WORDS = set("ل ژ د دا ب یێ ئەڤ ڤێ ژی هەیە".split())


def hawrami(text):
    """Hawrami as Keyman's horami keyboard types it: the soft d written
    with a ghunna mark (د٘) as ڎ, as most of its pages write it; ۊ as ۉ,
    the keyboard's (its pages write ۊ 18 times as often); and Arabic's kaf,
    yeh, alef maksura and heh doachashmee as Kurdish keyboards type them.
    A sentence in another spelling, whose vowels its keys do not type (ٙ ٚ
    ۍ ې: a twentieth of its words), is left out (`letters_never`)."""
    return text.replace("\u062f\u0658", "\u068e").replace("\u06ca", "\u06c9").translate(SORANI_FOLD).replace("\u0640", "")


def yi_syllables(text):
    """`text`'s Yi syllables, which a sentence is counted in."""
    return YI_SYLLABLE.findall(text)


def three_vowels(s):
    """Whether `s` reads as a language of three vowels, a i u (Kichwa's):
    e and o, which only its Spanish loans write (dios), are not more than
    a quarter of its vowels, as they are in Spanish's, Italian's,
    Portuguese's and Swedish's sentences."""
    vowels = [c for c in s.lower() if c in "aeiouáéíóú"]
    return sum(c in "eoéó" for c in vowels) <= 0.25 * len(vowels)


def palochka(text):
    return PALOCHKA.sub("\u04C0", text)


def karakalpak(text):
    return KARAKALPAK_OLD.sub(lambda m: KARAKALPAK_LETTERS[m.group(1)], text)


def abkhaz(text):
    return text.translate(ABKHAZ_FOLD)


def ejective(text):
    return EJECTIVE.sub("\u02BC", text)


def syriac(text):
    return SYRIAC_POINTS.sub("", text)


def long_tail(lang):
    fold = LONG_TAIL_FOLD[lang]
    return lambda text: text.translate(fold)


def tibetan(text):
    return text.replace("\u0F0C", "\u0F0B")


def guarani(text):
    return unicodedata.normalize("NFC", text).translate(GUARANI_FOLD)


def fula(text):
    return text.translate(FULA_FOLD)


def hawu(text):
    return SALTILLO.sub("\uA78C", text)


def krio(text):
    return text.translate(KRIO_FOLD)


def shaved(text):
    """A Serbian document written without its letters ASCII has not: 300
    letters and none of č ć š ž đ."""
    return sum(c.isalpha() for c in text) >= 300 and SERBIAN_DIACRITICS.isdisjoint(text)


def lorem(text):
    """A document holding Lorem ipsum, the placeholder Latin of web pages."""
    lowered = text.lower()
    return "lorem ipsum" in lowered or "consectetur adipiscing" in lowered


def chuvash(text):
    return text.translate(CHUVASH_FOLD)


def komi(text):
    return KOMI_I.sub(lambda m: "\u0456" if m.group() == "i" else "\u0406", text.replace("ö", "\u04E7").replace("Ö", "\u04E6"))


def polynesian(text):
    return GLOTTAL.sub("\u02BB", text)


def tahitian(text):
    return polynesian(text.replace("\u02C8", "\u02BB"))


def mari(text):
    return text.translate(MARI_FOLD)


def mongolic(text):
    return text.translate(MONGOLIC_FOLD)


def bambara(text):
    return text.translate(BAMBARA_FOLD)


def twi_font(text):
    """A document typed in an old Ghanaian font's encoding, which put
    Twi's ɛ on j and ɔ on c (yj, sj, asomafoc for yɛ, sɛ, asomafoɔ), as
    Zawgyi does Burmese's letters: Twi writes neither c nor j, so they are
    more than 1% of such a document's letters."""
    letters = sum(c.isalpha() for c in text)
    return sum(c in "cjCJ" for c in text) > 0.01 * max(1, letters)


def tetum(text):
    # The saltillo, a letter, written for the apostrophe of ne'e, ha'u.
    return text.replace("\uA78C", "'")


def sindhi(text):
    text = UNDERLINED.sub(lambda m: IMPLOSIVE[m.group(1)] + m.group(2), text)
    return text.replace("\u0952", "")


# Amharic's and Tigrinya's words end at the Ethiopic wordspace and
# punctuation too (፡ ። ፣ ፤ ፥ ፦ ፧ ፨), and their sentences at its full stop
# and question mark, or the two wordspaces older text writes for the stop,
# a space after them or not.
ETHIOPIC_WORDS = re.compile(r"[\s\u1361-\u1368.,;:!?\"'()\[\]{}<>«»“”‘’\-–—/|*#@&%$+=~`^]+")
ETHIOPIC_END = re.compile(r"(?<=[።፧፨])\s*|(?<=፡፡)\s*")


def ethiopic_words(text):
    return [w for w in ETHIOPIC_WORDS.split(text) if w]


# Amharic's own function words and Tigrinya's, which the other does not
# write (Amharic's ነው "is", እና "and", ላይ "on"; Tigrinya's እዩ "is", ኣብ
# "in", ናይ "of").
AMHARIC = set("ነው እና ላይ ውስጥ ወደ ግን ይህ ነበር ጋር ብቻ ደግሞ ናቸው እንደ ሲሆን ይሆናል".split())
TIGRINYA = set("እዩ ኣብ ናይ ምስ ከም እዚ ኢዩ ኣሎ እዮም ነይሩ ኣይ ብዙሕ ኢዮም እያ እዩ".split())
# Sebat Bet Gurage's, which Amharic writes otherwise (ሰብ "person", Amharic's
# ሰው; ኸማ "as", እንደ; ዝኽ "this", ይህ; ቃር "word", ቃል).
GURAGE = set("ሰብ ኸማ ዝኽ ቃር በኽ ዝም ኽኖም ቢኸርም".split())

# Each language: its script (Unicode's name for it), the neighbour whose
# function words mark a document as not its own, for a language that a
# rival is mistaken for, its own words and the rival's (or its own letter
# and the rival's, which it must write more often), and how its text is
# read where the web spells it two ways; with no test split, how many of
# its documents to keep for each held out (`hold`, else --hold).
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
    "sm": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 4, "normalize": polynesian},
    "sn": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 3},
    "ig": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 3},
    "st": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 3},
    "haw": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 3, "normalize": polynesian},
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
    "nn": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_NOT_NORSE | BOKMAL_ONLY, "every": 28, "brackets": True},
    "ast": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | SPANISH_ONLY, "every": 2, "brackets": True},
    "an": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | ARAGONESE_RIVAL, "brackets": True},
    "lad": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | LADINO_RIVAL, "brackets": True},
    "gsw": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_NOT_GERMANIC | GERMAN_ONLY, "own": ALEMANNIC, "rival": GERMAN_ONLY, "brackets": True},
    "bar": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_NOT_GERMANIC | GERMAN_ONLY, "own": BAVARIAN, "rival": GERMAN_ONLY, "brackets": True},
    "nds": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_NOT_GERMANIC | GERMAN_ONLY, "own": LOW_GERMAN, "rival": GERMAN_ONLY, "brackets": True},
    "pfl": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_NOT_GERMANIC | GERMAN_ONLY, "own": PALATINE, "rival": GERMAN_ONLY, "brackets": True},
    "ksh": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_NOT_GERMANIC | GERMAN_ONLY, "own": KOLSCH, "rival": GERMAN_ONLY, "brackets": True},
    "li": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_NOT_GERMANIC | DUTCH_ONLY, "own": LIMBURGISH, "rival": DUTCH_ONLY, "brackets": True},
    "hsb": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | SORBIAN_RIVAL, "brackets": True},
    "dsb": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | SORBIAN_RIVAL, "brackets": True},
    "nap": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | ITALIAN_ONLY, "own": NEAPOLITAN, "rival": ITALIAN_ONLY, "brackets": True, "sentence_never": {"er", "quer"}},
    "scn": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | (ITALIAN_ONLY - {"una"}), "own": SICILIAN, "rival": ITALIAN_ONLY - {"una"}, "brackets": True},
    "sc": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | (ITALIAN_ONLY - {"una"}), "brackets": True},
    "fur": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | (ITALIAN_ONLY - {"il"}), "brackets": True},
    "pms": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | ITALIAN_ONLY, "brackets": True},
    "lij": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | (ITALIAN_ONLY - {"per"}), "brackets": True},
    "wa": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_ONLY, "brackets": True},
    "ltg": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | LATVIAN_ONLY, "brackets": True},
    "vro": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | ESTONIAN_ONLY, "brackets": True, "sentence_never": {"sg"}},
    "csb": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | POLISH_ONLY, "brackets": True},
    "szl": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | POLISH_ONLY, "brackets": True},
    "crh": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | TURKISH_ONLY, "sentence_never": {"escort"}, "brackets": True},
    "sco": {"script": "LATIN", "neighbour": set(), "sentence_rival": SCOTS_RIVAL, "own": SCOTS, "rival": SCOTS_RIVAL, "brackets": True},
    "rue": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSYN_RIVAL, "brackets": True},
    "sah": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "every": 2, "brackets": True},
    "cv": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "every": 2, "normalize": chuvash, "brackets": True},
    "sme": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | NORWEGIAN_ONLY, "brackets": True},
    "mhr": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "hosts_out": CHAT_SPAM, "brackets": True, "normalize": mari},
    "mrj": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "hosts_out": CHAT_SPAM, "brackets": True, "normalize": mari},
    "udm": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "hosts_out": CHAT_SPAM, "brackets": True},
    "tyv": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "brackets": True, "normalize": mongolic},
    "bxr": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "brackets": True, "normalize": mongolic},
    "kpv": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "normalize": komi, "brackets": True},
    "koi": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "normalize": komi, "brackets": True},
    "myv": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "brackets": True},
    "mdf": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "brackets": True},
    "xal": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "brackets": True, "normalize": mongolic},
    "krl": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "olo": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "vep": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "kl": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | DANISH_ONLY, "brackets": True},
    "om": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    # Amharic and Tigrinya, which FineWeb-2's classifier mistakes for each
    # other, each against the other's own words; every 12th of Amharic's
    # row groups (75 million characters kept), every 2nd of Tigrinya's (63).
    "am": {"script": "ETHIOPIC", "neighbour": set(), "own": AMHARIC, "rival": TIGRINYA,
           "words": ethiopic_words, "ends": ETHIOPIC_END, "every": 12},
    "ti": {"script": "ETHIOPIC", "neighbour": set(), "own": TIGRINYA, "rival": AMHARIC,
           "words": ethiopic_words, "ends": ETHIOPIC_END, "every": 2},
    # Sebat Bet Gurage, against Amharic's words, read whole; its test split
    # is five documents: every 10th is held out.
    "sgw": {"script": "ETHIOPIC", "neighbour": set(), "own": GURAGE, "rival": AMHARIC,
            "words": ethiopic_words, "ends": ETHIOPIC_END, "hold": 10},
    # Gorani (Hawrami), against Sorani's, Badini's and Persian's words.
    "hac": {"script": "ARABIC", "neighbour": set(), "own": HAWRAMI,
            "rival": SORANI_WORDS | BADINI_WORDS | PERSIAN_ONLY, "normalize": hawrami, "brackets": True,
            "script_only": True, "letters_never": set("\u0659\u065a\u06cd\u06d0")},
    # Sylheti in Syloti Nagri: its news site's pages, beside its wiki's text
    # (SYLOTI_OUT); its Latin words (the site's menus) cut out. It has no
    # test split: every 20th document is held out.
    "syl-Sylo": {"script": "SYLOTI NAGRI", "neighbour": set(), "script_only": True, "ends": SYLOTI_END,
                 "hosts_out": SYLOTI_OUT, "brackets": True},
    # Nuosu, in Yi, which writes no space between its words: a page whose
    # syllables are a tenth its commonest, its lines and sentences of
    # three syllables or more, the Incubator's pages left out.
    "ii": {"script": "YI", "neighbour": set(), "units": yi_syllables, "own": NUOSU, "rival": set(),
           "own_least": 0.1, "least": 3, "script_only": True, "lines": True, "ends": YI_END,
           "hosts_out": re.compile(r"wikimedia\.org"), "brackets": True},
    "rm": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | GERMAN_ONLY, "brackets": True},
    "rn": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "lg": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "ln": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET, "brackets": True},
    "fj": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "ee": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET, "brackets": True},
    "ak": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "script_only": True, "brackets": True,
           "foreign": twi_font},
    "ts": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "nso": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "tn": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "to": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "normalize": polynesian, "brackets": True},
    "wo": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET, "brackets": True},
    "ty": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET, "normalize": tahitian, "brackets": True},
    "ve": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "nr": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "ay": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | SPANISH_SET, "brackets": True},
    "ss": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "qu": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | SPANISH_SET, "brackets": True},
    "bm": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET, "brackets": True, "normalize": bambara},
    "iu": {"script": "CANADIAN SYLLABICS", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "script_only": True, "brackets": True},
    "azb": {"script": "ARABIC", "neighbour": set(), "sentence_rival": PERSIAN_ONLY | ARABIC_IN_PERSIAN, "brackets": True,
            "hosts_out": RELIGIOUS_BOOKS},
    "mzn": {"script": "ARABIC", "neighbour": set(), "sentence_rival": PERSIAN_ONLY | ARABIC_IN_PERSIAN, "brackets": True,
            "hosts_out": RELIGIOUS_BOOKS},
    "kiu": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | TURKISH_ONLY | KURMANJI_ONLY, "sentence_never": SPAM, "brackets": True},
    "diq": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | TURKISH_ONLY | KURMANJI_ONLY, "sentence_never": SPAM, "brackets": True},
    "tly": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | AZERBAIJANI_ONLY, "brackets": True},
    "kaa": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "normalize": karakalpak, "brackets": True},
    "ce": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "normalize": palochka, "brackets": True},
    "inh": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "normalize": palochka, "brackets": True},
    "av": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "normalize": palochka, "brackets": True},
    "kbd": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "normalize": palochka, "brackets": True},
    "ady": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "normalize": palochka, "brackets": True},
    "lez": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "normalize": palochka, "brackets": True},
    "lbe": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "normalize": palochka, "sentence_never": {"читать", "далее"}, "brackets": True},
    "tab": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "normalize": palochka, "sentence_never": {"опубликовано", "просмотров"}, "brackets": True},
    "dar": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "normalize": palochka, "brackets": True},
    "kum": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "brackets": True},
    "krc": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "brackets": True},
    "ab": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY, "normalize": abkhaz, "brackets": True},
    "xmf": {"script": "GEORGIAN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | GEORGIAN_ONLY, "brackets": True},
    "crs": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | (FRENCH_SET - {"pour"}), "brackets": True},
    "nd": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "ng": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "efi": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "sg": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET, "brackets": True},
    "kj": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "lua": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET, "brackets": True},
    "lu": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET, "brackets": True},
    "kea": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | CAPE_VERDE_RIVAL, "brackets": True},
    "mos": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET, "brackets": True},
    "rcf": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET, "brackets": True},
    "kg": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET, "brackets": True},
    "wal": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "luo": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "tiv": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "iso": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "bem": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "fon": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET, "brackets": True},
    "ach": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "bci": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET, "brackets": True},
    "dyu": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET, "brackets": True},
    "ki": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "kmb": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | PORTUGUESE_ONLY, "brackets": True},
    "umb": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | PORTUGUESE_ONLY, "brackets": True},
    "la": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 44, "foreign": lorem, "sentence_never": LOREM, "brackets": True},
    "pap": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | PAPIAMENTO_RIVAL, "every": 5, "brackets": True},
    "syr": {"script": "SYRIAC", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "normalize": syriac, "brackets": True},
    "yua": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | SPANISH_SET, "normalize": ejective, "brackets": True},
    "quc": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | SPANISH_SET, "normalize": ejective, "brackets": True},
    "kek": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | SPANISH_SET, "normalize": ejective, "brackets": True},
    "quz": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | SPANISH_SET, "normalize": ejective, "brackets": True},
    "qug": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | SPANISH_SET | PORTUGUESE_ONLY | SWEDISH_ONLY | SWAHILI_ONLY,
            "script_only": True, "brackets": True,
            "own": KICHWA, "rival": set(), "own_least": 0.03, "letters_never": set("ɛɔĩũšž"), "three_vowels": True},
    "srn": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | DUTCH_ONLY, "brackets": True},
    "gcf": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET, "brackets": True},
    "bi": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "ia": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "tpi": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "gil": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "tvl": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "ho": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "ch": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "jam": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "brackets": True},
    "sr-Latn": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY, "every": 18,
                "foreign": shaved, "letters_never": MOJIBAKE | CYRILLIC_LETTERS, "brackets": True},
    "gn": {"script": "LATIN", "neighbour": set(), "sentence_rival": GUARANI_RIVAL, "normalize": guarani,
           "brackets": True},
    "ff": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET,
           "normalize": fula, "brackets": True},
    "ber": {"script": "TIFINAGH", "neighbour": set(), "script_only": True, "letters_never": set("ⵧⴵⵒ"),
            "brackets": True},
    "bo": {"script": "TIBETAN", "neighbour": set(), "unspaced": True, "lines": True, "normalize": tibetan,
           "script_only": True, "every": 4, "brackets": True},
    "dz": {"script": "TIBETAN", "neighbour": set(), "unspaced": True, "lines": True, "normalize": tibetan,
           "script_only": True, "syllables": True, "own": DZONGKHA, "rival": TIBETAN, "own_least": 0.005,
           "brackets": True},
    "kbp": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET,
           "letters_never": set("|"), "normalize": long_tail("kbp"), "brackets": True},
    "dag": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY,
           "letters_never": set("|"), "normalize": long_tail("dag"), "hosts_out": SPAM_HOSTS,
           "brackets": True, "hold": 10},
    "seh": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | PORTUGUESE_ONLY,
           "letters_never": set("|"), "brackets": True},
    "ada": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY,
           "letters_never": set("|"), "brackets": True},
    "nyn": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY,
           "letters_never": set("|"), "brackets": True},
    "kwy": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | PORTUGUESE_ONLY | FRENCH_SET,
           "letters_never": set("|"), "brackets": True},
    "sid": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY,
           "letters_never": set("|"), "normalize": long_tail("sid"), "brackets": True},
    "koo": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET,
           "letters_never": set("|"), "brackets": True},
    "kam": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | SWAHILI_ONLY,
           "letters_never": set("|"), "normalize": long_tail("kam"), "brackets": True},
    "sop": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET,
           "letters_never": set("|"), "brackets": True},
    "vmw": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | PORTUGUESE_ONLY,
           "letters_never": set("|"), "brackets": True},
    "ndc": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | PORTUGUESE_ONLY,
           "letters_never": set("|"), "brackets": True},
    "gur": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET,
           "letters_never": set("|"), "brackets": True},
    "bum": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET,
           "letters_never": set("|"), "brackets": True},
    "bin": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY,
           "letters_never": set("|"), "brackets": True},
    "teo": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY,
           "letters_never": set("|"), "brackets": True},
    "sgc": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | SWAHILI_ONLY,
           "letters_never": set("|"), "brackets": True},
    "mfq": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | FRENCH_SET,
           "letters_never": set("|"), "normalize": long_tail("mfq"), "hosts_out": MOBA_OUT, "brackets": True},
    "yao": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | PORTUGUESE_ONLY,
           "letters_never": set("|"), "normalize": long_tail("yao"),
           "brackets": True, "hold": 10},
    "dik": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY,
           "letters_never": set("|"), "brackets": True},
    "min": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | INDONESIAN_ONLY,
           "letters_never": set("|"), "hosts_out": BOT_WIKI, "brackets": True},
    "war": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | TAGALOG_ONLY,
           "letters_never": set("|"), "hosts_out": BOT_WIKI, "brackets": True},
    "nia": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | INDONESIAN_ONLY,
           "letters_never": set("|"), "normalize": long_tail("nia"), "brackets": True},
    "mad": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | INDONESIAN_ONLY,
           "letters_never": set("|"), "brackets": True},
    "kri": {"script": "LATIN", "neighbour": set(), "own": KRIO, "rival": KRIOL, "own_least": 0.03,
            "sentence_rival": ENGLISH_ONLY, "sentence_never": {"src", "trg"}, "normalize": krio,
            "brackets": True, "hold": 10},
    "tdx": {"script": "LATIN", "neighbour": set(), "own": TANDROY, "rival": MALAGASY_STANDARD, "own_least": 0.05,
            "sentence_rival": ENGLISH_ONLY | FRENCH_ONLY, "brackets": True},
    "hvn": {"script": "LATIN", "neighbour": set(), "own": HAWU, "rival": INDONESIAN_ONLY, "own_least": 0.05,
            "sentence_rival": ENGLISH_ONLY | INDONESIAN_ONLY, "normalize": hawu, "brackets": True,
            "hold": 10},
    "ktu": {"script": "LATIN", "neighbour": set(), "own": KITUBA, "rival": LINGALA | FRENCH_ONLY, "own_least": 0.05,
            "sentence_rival": ENGLISH_ONLY | FRENCH_ONLY, "brackets": True},
    "quy": {"script": "LATIN", "neighbour": set(), "own": AYACUCHO, "rival": set(), "own_least": 0.03,
            "sentence_rival": ENGLISH_ONLY | SPANISH_SET, "brackets": True},
    "cor": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY,
            "letters_never": set("|"), "brackets": True},
    "glv": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY,
            "letters_never": set("|"), "hosts_out": MANX_OUT, "brackets": True},
    "ext": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | EXTREMADURAN_RIVAL,
            "letters_never": set("|"), "hosts_out": EXTREMADURAN_OUT, "brackets": True},
    "pcd": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | PICARD_RIVAL,
            "letters_never": set("|"), "hosts_out": PICARD_OUT, "brackets": True},
    "sgs": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | LITHUANIAN_ONLY,
            "letters_never": set("|"), "brackets": True},
    "pnt": {"script": "GREEK", "neighbour": set(), "sentence_rival": GREEK_ONLY,
            "letters_never": set("|"), "normalize": long_tail("pnt"), "brackets": True},
    "vec": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | VENETIAN_RIVAL,
            "letters_never": set("|"), "normalize": long_tail("vec"), "hosts_out": VENETIAN_OUT,
            "brackets": True},
    "vls": {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_NOT_GERMANIC | DUTCH_ONLY,
            "letters_never": set("|ſ"), "hosts_out": FLEMISH_OUT, "brackets": True},
    "lmo": {"script": "LATIN", "neighbour": set(), "own": LOMBARD, "rival": LADIN,
            "sentence_rival": ENGLISH_ONLY | VENETIAN_RIVAL, "letters_never": set("|"),
            "normalize": long_tail("lmo"), "hosts_out": LOMBARD_OUT, "brackets": True},
    "lld": {"script": "LATIN", "neighbour": set(), "own": LADIN, "rival": LOMBARD, "own_least": 0.02,
            "sentence_rival": ENGLISH_ONLY | (ITALIAN_ONLY - {"per"}) | GERMAN_ONLY,
            "letters_never": set("|"), "normalize": long_tail("lld"), "brackets": True},
    "gag": {"script": "LATIN", "neighbour": set(), "own": GAGAUZ, "rival": GAGAUZ_RIVAL,
            "sentence_rival": ENGLISH_ONLY | GAGAUZ_RIVAL, "letters_never": set("|") | CYRILLIC_LETTERS,
            "normalize": long_tail("gag"), "brackets": True},
    # East Asia's in Latin letters, their Han runs (Hàn-lô text, a wiki's
    # menus) left out.
    **{lang: {"script": "LATIN", "neighbour": set(), "own": own, "rival": ENGLISH_ONLY | rival,
              "sentence_rival": ENGLISH_ONLY | rival, "script_only": True, "letters_never": set("|"),
              "brackets": True, **more}
       for lang, own, rival, more in (
           ("nan", SOUTHERN_MIN, set(), {"normalize": southern_min, "script_also": frozenset("\u207F")}),
           ("hak", HAKKA, INDONESIAN_ONLY, {"normalize": hakka, "hosts_out": HAKKA_OUT, "hold": 10}),
           ("ami", AMIS, set(), {"normalize": amis, "hosts_out": AMIS_OUT, "hold": 10}),
           ("za", ZHUANG, set(), {"hold": 10}))},
    # Africa's long tail, those with 0.4 to 0.9 MB of FineWeb-2's text (the
    # Bible's and Jehovah's Witnesses' in the main): the state's language's
    # sentences left out, every 10th document held out (their test splits
    # hold a few hundred words).
    **{lang: {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | rival,
              "letters_never": set("|"), "brackets": True, "hold": 10,
              **({"normalize": long_tail(lang)} if lang in LONG_TAIL_FOLD else {})}
       for lang, rival in (
           ("zne", FRENCH_ONLY), ("alz", SWAHILI_RIVAL), ("cce", PORTUGUESE_ONLY), ("skg", FRENCH_ONLY),
           ("bss", FRENCH_ONLY), ("men", set()), ("lgg", SWAHILI_RIVAL), ("xon", FRENCH_ONLY), ("dnj", FRENCH_ONLY),
           ("gbo", set()), ("heh", SWAHILI_RIVAL), ("gux", FRENCH_ONLY), ("gmv", set()), ("suk", SWAHILI_RIVAL),
           ("spp", FRENCH_ONLY), ("bas", FRENCH_ONLY), ("hay", SWAHILI_RIVAL), ("kbr", set()),
           ("ngl", PORTUGUESE_ONLY), ("gog", SWAHILI_RIVAL), ("chw", PORTUGUESE_ONLY), ("fat", set()),
           ("tsc", PORTUGUESE_ONLY), ("bud", FRENCH_ONLY), ("kao", FRENCH_ONLY), ("gvl", FRENCH_ONLY),
           ("nnb", FRENCH_ONLY | SWAHILI_RIVAL), ("nyo", SWAHILI_RIVAL))},
    # The long tail of Southeast Asia, the Americas, the Pacific (0.4 to 1.2
    # MB of FineWeb-2's text) and of Africa (0.2 to 0.4 MB): every 10th
    # document held out; Maranao's Arabic (sermons, the Quran) left out by
    # script. Makassarese's is not used: it writes its glottal stop ' in
    # the Bible and q in its chronicles, and saved 27% (8% astray).
    **{lang: {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | rival,
              "letters_never": set("|"), "brackets": True, "hold": 10,
              **({"normalize": long_tail(lang)} if lang in LONG_TAIL_FOLD else {}), **more}
       for lang, rival, more in (
           ("niu", set(), {}), ("hwc", set(), {"hosts_out": LOLCAT_HOSTS}), ("acf", FRENCH_ONLY, {}),
           ("mrw", set(), {"script_only": True}), ("nij", INDONESIAN_ONLY, {}), ("guc", SPANISH_SET, {}),
           ("ljp", INDONESIAN_ONLY, {}), ("sas", INDONESIAN_ONLY, {}),
           ("gor", INDONESIAN_ONLY, {}),
           ("itv", TAGALOG_ONLY, {"hosts_out": BOOK_PIRATES}), ("bku", TAGALOG_ONLY, {}), ("bpr", TAGALOG_ONLY, {}),
           ("quh", SPANISH_SET, {}), ("gcr", FRENCH_ONLY, {}), ("tum", set(), {}), ("tll", FRENCH_ONLY, {}),
           ("guz", SWAHILI_RIVAL, {}), ("kde", PORTUGUESE_ONLY | SWAHILI_RIVAL, {}), ("nyy", SWAHILI_RIVAL, {}),
           ("mas", SWAHILI_RIVAL, {}), ("mnk", FRENCH_ONLY, {}), ("mer", SWAHILI_RIVAL, {}), ("dga", set(), {}))},
    # Myanmar's other languages, cut into syllables.
    **{lang: {"script": "MYANMAR", "neighbour": set(), "lines": True, "unspaced": True,
              "normalize": normalize, "script_only": True, "foreign": foreign,
              "units": myanmar_syllables, "sentence_rival": BURMESE_ONLY, "rival_most": 0.04,
              "brackets": True, **more}
       for lang, normalize, foreign, more in (
           ("shn", burmese, own_zawgyi, {}), ("mnw", mon, own_zawgyi, {}),
           ("ksw", karen, own_zawgyi, {}), ("blk", burmese, not_pao, {"hold": 10}))},
    # Coptic, Bohairic's (the New Testament, the liturgy, the Fathers, a
    # wiki): its runs of Coptic alone (the Arabic, English and French beside
    # them left out), a sentence whose letters are those of one read before
    # left out too (`same`: a verse's copies on other sites), every 10th
    # document held out.
    "cop": {"script": "COPTIC", "neighbour": set(), "own": BOHAIRIC, "rival": SAHIDIC,
            "normalize": coptic, "script_only": True, "lines": True,
            "same": coptic_letters, "letters_never": set("|"), "brackets": True, "hosts_out": COPTIC_OUT,
            "hold": 10},
    # Lahu, its tones letters of its words (`script_also`).
    "lhu": {"script": "LATIN", "neighbour": set(), "normalize": lahu, "script_only": True,
            "script_also": LAHU_TONES, "sentence_rival": ENGLISH_ONLY, "letters_never": set("|"),
            "brackets": True, "hold": 10},
    # The long tail's last, those with 0.15 to 0.3 MB of FineWeb-2's text
    # (the Bible, Jehovah's Witnesses', Canada's and New Zealand's
    # governments', Livonian's wiki and institute): the state's language's
    # sentences left out, and Swampy Cree's syllabics; beside the Crees and
    # Ottawa, English's sentences by its commonest words as well (in, is,
    # of, on: none of theirs).
    **{lang: {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | rival,
              "letters_never": set("|"), "brackets": True, "hold": 10,
              **({"normalize": long_tail(lang)} if lang in LONG_TAIL_FOLD else {}), **more}
       for lang, rival, more in (
           ("hrx", HUNSRIK_RIVAL | GERMAN_ONLY, {"own": HUNSRIK, "rival": HUNSRIK_GERMAN, "hosts_out": MEMRISE}),
           ("crk", ENGLISH, {}), ("csw", ENGLISH, {"script_only": True}), ("otw", ENGLISH | FRENCH_ONLY, {}),
           ("tkl", set(), {}),
           ("kne", TAGALOG_ONLY, {}), ("mqj", INDONESIAN_ONLY, {}),
           ("liv", (LATVIAN_ONLY - {"kā"}) | (ESTONIAN_ONLY - {"mis", "kui"}), {"hosts_out": LIVONIAN_OUT}),
           ("abs", INDONESIAN_ONLY, {"hosts_out": CHORDS}), ("ajg", FRENCH_ONLY, {"hosts_out": MOZILLA}))},
    # The long tail's rest, with 0.1 to 1.4 MB of FineWeb-2's text: Nenets'
    # and Dungan's in Cyrillic letters, Russian's sentences left out; of
    # the others, English's and their state's language's (and Tausug's
    # Arabic pages, by script). Lango holds out every 9th document: its
    # 10th, the Gospel of John, is two fifths of its text.
    "yrk": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY - {"по"},
            "normalize": nenets, "letters_never": set("|ԓԒ"), "letters_some": NENETS_MARKS,
            "hosts_out": NENETS_OUT, "brackets": True, "hold": 10},
    "dng": {"script": "CYRILLIC", "neighbour": set(), "sentence_rival": RUSSIAN_ONLY,
            "normalize": dungan, "letters_never": set("|"), "hosts_out": SONG_SCRIPTS, "brackets": True,
            "hold": 10},
    **{lang: {"script": "LATIN", "neighbour": set(), "sentence_rival": ENGLISH_ONLY | rival,
              "letters_never": set("|"), "brackets": True, "hold": 10,
              **({"normalize": long_tail(lang)} if lang in LONG_TAIL_FOLD else {}), **more}
       for lang, rival, more in (
           ("tsg", TAGALOG_ONLY, {"script_only": True}), ("ewo", FRENCH_SET, {}),
           ("ttj", set(), {"hosts_out": SCANS}), ("mgh", PORTUGUESE_ONLY, {}),
           ("knc", set(), {"normalize": kanuri}), ("nyf", SWAHILI_RIVAL, {"hosts_out": KENYAN_RADIOS}),
           ("laj", SWAHILI_RIVAL, {"hold": 9}), ("hnn", TAGALOG_ONLY, {"hosts_out": HANUNOO_OUT}),
           ("sda", BALINESE_RIVAL, {"hosts_out": WIKTIONARY}), ("lew", BALINESE_RIVAL, {}),
           ("sdc", ITALIAN_ONLY - {"una"}, {}))},
    # Romani, Vlax and Balkan: each from both FineWeb-2 files, by its
    # dialect's words and in č š ž.
    **{lang: {"script": "LATIN", "neighbour": set(), "own": own, "rival": rival,
              "foreign": romani_digraphs, "sentence_rival": ROMANI_RIVAL,
              "letters_never": CYRILLIC_LETTERS | set("|"), "brackets": True, "hold": 10}
       for lang, own, rival in (("rmy", ROMANI_VLAX, ROMANI_BALKAN), ("rmn", ROMANI_BALKAN, ROMANI_VLAX))},
    # Northwestern Ojibwe in syllabics, a syllable a character; English's
    # names cut out of its sentences.
    "ojb": {"script": "CANADIAN SYLLABICS", "neighbour": set(), "sentence_rival": ENGLISH_ONLY,
            "normalize": ojibwe, "script_only": True, "brackets": True, "hold": 10},
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
    ws = conf.get("words", words)(text.lower())
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
        if conf.get("syllables"):
            ws = SYLLABLE.findall(text)
        elif "units" in conf:
            ws = conf["units"](text)
        own = share(ws, conf["own"])
        if own < conf.get("own_least", 0.01) or own <= share(ws, conf["rival"]) + share(ws, conf["neighbour"]):
            return False
    return True


def units(s, conf):
    """`s`'s words, as the sentence filters count them: or its syllables,
    for a language cut into them."""
    return conf["units"](s) if "units" in conf else words(s.lower())


def in_script(s, script):
    letters = [c for c in s if c.isalpha()]
    own = sum(unicodedata.name(c, "").startswith(script) for c in letters)
    return 2 * own >= len(letters)


def long_enough(s, conf):
    if "least" in conf:
        return len(units(s, conf)) >= conf["least"]
    if conf.get("unspaced"):
        return sum(c.isalpha() for c in s) >= 15
    return len(s.split()) >= 3


def sentences(text, conf):
    script = conf["script"]
    text = STEERING.sub("", text)
    if conf.get("brackets"):
        text = BRACKETED.sub(" ", text)
    if "normalize" in conf:
        text = conf["normalize"](text)
    pieces = text.split("\n") if conf.get("lines") else [text]
    for piece in pieces:
        if conf.get("unspaced"):
            ends = UNSPACED_END.split(piece)
        else:
            ends = conf["ends"].split(piece) if "ends" in conf else [piece]
        for s in (s for e in ends for s in END.split(e)):
            for s in (script_runs(s, script, conf.get("script_also", frozenset())) if conf.get("script_only")
                      else [" ".join(s.split())]):
                if ("sentence_rival" in conf
                        and share(units(s, conf), conf["sentence_rival"]) > conf.get("rival_most", 0.15)):
                    continue
                if "sentence_never" in conf and not conf["sentence_never"].isdisjoint(words(s.lower())):
                    continue
                if "letters_never" in conf and any(c in conf["letters_never"] for c in s):
                    continue
                if "letters_some" in conf and conf["letters_some"].isdisjoint(s):
                    continue
                if "three_vowels" in conf and not three_vowels(s):
                    continue
                if len(s) <= 1000 and long_enough(s, conf) and not MARKUP.search(s) and in_script(s, script):
                    yield s


def script_runs(s, script, also=frozenset()):
    """`s` cut where a word has letters of another script, those left out;
    the letters `also` are the script's (Southern Min's ⁿ, which Unicode
    names a superscript)."""
    runs, run = [], []
    for w in s.split():
        if all(unicodedata.name(c, "").startswith(script) or c in also for c in w if c.isalpha()):
            run.append(w)
        elif run:
            runs.append(" ".join(run))
            run = []
    if run:
        runs.append(" ".join(run))
    return runs


# The hosts no document is read from, in any language (the docstring).
NEVER_HOSTS = re.compile(r"(^|\.)bible\.is$", re.I)


def never(url):
    """Whether `url` is on a host no document is read from."""
    host = re.sub(r"^[a-z][a-z0-9+.-]*://", "", url, flags=re.I).split("/", 1)[0].split(":", 1)[0]
    return bool(NEVER_HOSTS.search(host))


def documents(paths, every=1):
    """Each document's text and the address it was read at ("" where a
    source does not say); none from a host never read (NEVER_HOSTS)."""
    for path in paths:
        if path.endswith(".jsonl.gz"):
            # MADLAD-400's: a JSON object a line, its text in "text".
            with gzip.open(path, "rt", encoding="utf-8") as f:
                for line in f:
                    if line.strip():
                        # Its line breaks are written as the two
                        # characters \ and n.
                        yield json.loads(line)["text"].replace("\\n", "\n"), ""
            continue
        import pyarrow.parquet as pq

        f = pq.ParquetFile(path)
        names = f.schema_arrow.names
        # FineWeb-2's text is in "text" and its address in "url", GlotCC's
        # in "content" and "warc-target-uri".
        column = "text" if "text" in names else "content"
        where = next((n for n in ("url", "warc-target-uri") if n in names), None)
        for g in range(0, f.metadata.num_row_groups, every):
            group = f.read_row_group(g, columns=[column] + ([where] if where else []))
            texts = group.column(column).to_pylist()
            urls = group.column(where).to_pylist() if where else [""] * len(texts)
            yield from ((t, u or "") for t, u in zip(texts, urls) if not never(u or ""))


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--lang", required=True, choices=sorted(LANGS))
    ap.add_argument("--eval", type=int, default=500, help="held-out sentences written at most")
    ap.add_argument("out")
    ap.add_argument("eval_out")
    ap.add_argument("--train", nargs="+", required=True, help="the train split's files")
    ap.add_argument("--test", nargs="+", help="the test split's files")
    ap.add_argument("--hold", type=int, default=20,
                    help="with no test split (or none of its documents kept), every HOLDth "
                         "document kept is held out")
    ap.add_argument("--exclude", nargs="+", default=[],
                    help="another source's sentences of the language (its corpus and held-out "
                         "text): neither written nor held out")
    opts = ap.parse_args()
    train, test = opts.train, opts.test or []
    conf = LANGS[opts.lang]

    def keep(text, url):
        return kept(text, conf) and not ("hosts_out" in conf and conf["hosts_out"].search(url))

    # A test split none of whose documents is kept (all bible.is's, as
    # some small languages' are) holds out the train split's instead.
    if test and not any(keep(t, url) for t, url in documents(test)):
        test = []
    # What makes two sentences one: their text, or (`same`) what a
    # language's copies of one share.
    same = conf.get("same", lambda s: s)
    seen = set()
    for path in opts.exclude:
        with open(path, encoding="utf-8") as f:
            seen.update(same(line.rstrip("\n")) for line in f)
    excluded = len(seen)
    docs = dropped = 0
    held_docs = []
    with open(opts.out, "w", encoding="utf-8") as out:
        for text, url in documents(train, conf.get("every", 1)):
            docs += 1
            if not keep(text, url):
                dropped += 1
                continue
            if not test and (docs - dropped) % conf.get("hold", opts.hold) == 0:
                held_docs.append(text)
                continue
            for s in sentences(text, conf):
                if same(s) not in seen:
                    seen.add(same(s))
                    out.write(s + "\n")
    held = {}
    held_test = (t for t, url in documents(test) if keep(t, url))
    for text in held_docs or held_test:
        for s in sentences(text, conf):
            if same(s) not in seen:
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
