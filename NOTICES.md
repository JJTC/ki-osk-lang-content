# Notices — ki-osk language content

Attributions and licences for the datasets in this repository and the
files built from them. Paths are relative to this repository; ki-osk
fetches the release it pins into its `lang/`, and an installed system has
the same files under `/usr/share/ki-osk/`.

**Several of these are share-alike.** Redistributing the derived `.bin`
files carries their terms.

The code here — the Makefile, the Cargo workspace and `crates/`, the
scripts, the workflows and `packs.toml` — is MIT licensed (`LICENSE`), as
are the text rules in `rules/` and the transliteration tables in
`translit/`, which were written here (thirty tables' costs then fitted
to the Dakshina and Aksharantar datasets' romanizations, and Gondi's and
Angika's taken from Telugu's and Hindi's, below).
The data keeps its original licence, below. That includes the seeds in
`seeds/`, which are excerpts of the datasets they are listed under. Add an entry here whenever a
new dataset is pulled in.

Exactly which upstream file each committed output was built from is
recorded, by sha256, in `sources.lock`.

The keyboard layouts, the emoji data, the offensive-word lists, the
Mandarin syllable inventory and the Double-Pinyin scheme tables came here
from the ki-osk repository (ki-osk 54672eb), which holds only its MIT
code since. The fonts the Wayland keyboard draws with are in
ki-osk-wayland's `NOTICES.md`.

## Dictionaries

- **CC-CEDICT** (Mandarin, Pinyin → Simplified Hanzi) — CC BY-SA 4.0.
  `pinyin/cedict.bin` is built from the **full CC-CEDICT dataset** (the
  MDBG export; the committed bundle is from the 2026-08-02 export,
  124 770 lines → 86 400 Pinyin keys, ≤9 candidates per key) via
  `make pinyin-assets`. Share-alike applies to the resulting `.bin`. A
  ~150-headword HSK-1/2 seed (`seeds/pinyin/seed.cedict.txt`,
  `build-pinyin --seed`) remains as the no-download fallback; it is
  CC-CEDICT data under the same licence. Source:
  https://www.mdbg.net/chinese/dictionary?page=cc-cedict
  The Zhuyin / Traditional-Chinese IME reuses the same CC-CEDICT source:
  `zhuyin/cedict.bin` (`make zhuyin-assets`) keys the **Traditional**
  column by toneless Bopomofo — 86 388 keys, ~2.8 MB. Same CC BY-SA 4.0
  share-alike.
- **CC-Canto Cantonese readings for CC-CEDICT** (Cantonese, Jyutping →
  Traditional Hanzi) — **CC BY-SA 3.0**, © 2015 Pleco Software
  Incorporated, with CC-CEDICT's own portions as above.
  `jyutping/cedict.bin` is built from the readings file dated
  2015-09-23 (`cccedict-canto-readings-150923.txt`, 105 862 entries →
  84 531 toneless-Jyutping keys, ~1.9 MB) via `make jyutping-assets`.
  This is the readings file, not the separate CC-Canto dictionary
  (`cccanto-*.zip`) published beside it. Jyutping readings **cannot** be
  derived from the Mandarin Pinyin table (你好 is `nei5 hou2`, not
  `ni3 hao3`), so this is a distinct dataset. Share-alike applies to the
  resulting `.bin`. The ~150-key seed (`seeds/jyutping/seed.canto.txt`,
  `build-jyutping --seed`) is the same readings data, same licence.
  Source: https://cantonese.org/download.html
- **FrequencyWords** (Chinese word frequencies, used only to ORDER the
  Pinyin, Zhuyin and Jyutping candidates) — MIT, © 2016 Hermit Dave. The
  list is derived from OpenSubtitles, which ki-osk's CLAUDE.md §4.8 names
  as an acceptable source. `content/2018/zh_cn/zh_cn_50k.txt` (the
  Simplified list ranks all three IMEs), fetched by
  `scripts/fetch-cjk-sources.sh`. No part of the list is redistributed —
  only the resulting candidate ORDER inside the `.bin` files, which carry
  their dictionary's own share-alike. Source:
  https://github.com/hermitdave/FrequencyWords
- **mozc kanji dict** (Japanese kana → kanji) — 3-clause BSD.
  `kana/mozc-kanji.bin` is built from the **mozc OSS system dictionary**
  (`src/data/dictionary_oss/dictionary00–09.txt` @ master; the committed
  bundle from the 2026-07-19 fetch, 1 289 076 entries) via
  `make kana-assets` (`--max-cost 7000 --max-readings 80000`): entries
  above mozc cost 7000 (rare compounds, most proper nouns) are dropped,
  then the 80 000 most frequent readings are kept (≤9 candidates each,
  ~4.4 MB). Each candidate carries its mozc **word cost and POS ids**
  (`lid`/`rid`) for the context-aware conversion lattice. The BSD license
  applies to the resulting `.bin`. A ~190-headword JLPT N5/N4 seed
  (`seeds/kana/seed.mozc.txt`, `build-kana --seed`, POS-less) remains
  as the no-download fallback, under the same BSD licence.
  Source: https://github.com/google/mozc
- **mozc connection matrix** (kana → kanji context model) — 3-clause BSD.
  `kana/mozc-connection.bin` is built from mozc's
  `src/data/dictionary_oss/connection_single_column.txt` (the 2672×2672
  POS-bigram transition-cost matrix) via `make kana-assets`: the 14 MB
  `i16` matrix is zlib-deflated to ~1.9 MB and inflated once at load. It
  drives the Viterbi (`word_cost + connection(prev.rid, cur.lid)`),
  turning にほんごをべんきょうする into 日本語を勉強する. Absent → the dict
  falls back to word-cost-only conversion.

## Prediction corpora

The committed `predict/<lang>.fst` + `predict/<lang>.ngram.bin` are
built from
per-language sentence subsets — the first 120 000 sentences in id order,
or all available where fewer — of the **Tatoeba** per-language exports
(<https://downloads.tatoeba.org/exports/per_language/>), filtered to the
matching ISO-639-3 code. Held out of every subset are each 20th sentence
by id and any sentence with the same text as one of those; the first
1 500 held out per language are committed as `eval/<lang>.txt`, the text
ki-osk's `toolbox eval-predict` measures prediction on. 54 were built
from the exports of 2026-11-08, Thai from those of 2026-11-12. Thai is
written without spaces between words: its sentences are cut into words
with its word list (see [Word lists](#word-lists)).

| ki-osk | code | ki-osk | code | ki-osk | code | ki-osk | code |
|--|--|--|--|--|--|--|--|
| en | eng | pt | por | ro | ron | id | ind |
| de | deu | sv | swe | hu | hun | fil | tgl |
| fr | fra | pl | pol | sk | slk | az | aze |
| es | spa | no | nob | hr | hrv | ku | kmr |
| it | ita | da | dan | et | est | ug | uig |
| nl | nld | fi | fin | is | isl | hi | hin |
| ru | rus | cs | ces | ca | cat | bn | ben |
| uk | ukr | tr | tur | eo | epo | hy | hye |
| el | ell | vi | vie | bg | bul | ka | kat |
| ar | ara | he | heb | sr | srp | lt | lit |
| fa | pes | mk | mkd | be | bel | lv | lvs |
| — | — | — | — | kab | kab | — | — |
| oc | oci | gl | glg | br | bre | eu | eus |
| tt | tat | mr | mar | as | asm | ms | zsm |
| os | oss | th | tha | — | — | — | — |

Size tiers are recorded in the Makefile: 32 languages
(`PREDICT_COMPACT_LANGS`) keep 18 000 words and 30 000 n-grams, those
whose lexicons Wikipedia or web text widens (below) 30 000 and 60 000,
Bengali, Marathi, Assamese, Magahi, Newari, Armenian and Persian 80 000
and 90 000 (`PREDICT_LARGE_LANGS`), the
Dravidian languages 200 000 and 90 000 (`PREDICT_WIDER_LANGS`), the
others 20 000 and 60 000.

### Wikipedia text (the Dakshina dataset)

Twelve South Asian languages' models are built from Wikipedia text too,
as the **Dakshina dataset** v1.0 (Google Research,
<https://github.com/google-research-datasets/dakshina>) gives it: the
first 300 000 lines (all, where fewer) of its filtered Wikipedia text
(`<lang>.wiki-filt.train.text.shuf.txt.gz`). It is the whole corpus of
Gujarati, Kannada, Malayalam, Punjabi, Sindhi, Sinhala, Tamil, Telugu and
Urdu, of which Tatoeba has too few sentences, and is added to Tatoeba's
for Hindi, Bengali and Marathi, Tatoeba's counted thirty times (Marathi's
ten). For those
nine, the first 500 lines of its held-out text
(`<lang>.wiki-filt.valid.text.shuf.txt.gz`) are `eval/<lang>.txt`.
`scripts/fetch-dakshina.sh` reads the archive, pinned by its sha256;
`sources.lock` records the sha256 of each language's text as
`dakshina/<lang>.txt`. The sentences it romanizes are in that text too,
a few hundred to 4 500 lines a language: `scripts/fetch-dakshina.sh`
leaves them out, so that no model knows a word from the sentences the
tables are measured on. Its romanization lexicons' training words fit the
costs of twelve transliteration tables (`make fit-translit`, below), and
its test words and the test half of its romanized sentences measure them
(`make eval-translit`); none of them is shipped.

Hinglish (`predict/hi-Latn.*`, the `hi-Latn` pack) is Hindi's corpora —
Tatoeba's sentences and Dakshina's text — respelled in Latin letters
(`scripts/build-hinglish.py`): each word in one of the spellings
Dakshina's romanization lexicons attest, as often as each was typed, any
other by rule. `eval/hi-Latn.txt` is the first 500 of Dakshina's romanized
Hindi test sentences, as people typed them, which the Wikipedia text it
learns from leaves out.

### Wikipedia text (the wikis' own)

Thirty-three languages' models learn from their own Wikipedia alone:
Bhojpuri, Maithili, Mongolian, Nepali, Odia, Western Punjabi, Pashto,
Sanskrit, Santali, Yiddish, Konkani, Kashmiri, Meitei, Awadhi, Saraiki,
Tulu, Dotyali, Angika, Eastern Min (in Bàng-uâ-cê, the Han its sentences
write beside left out), Fiji Hindi (its sentences that write its own
function words: its references are English) and Emilian (the
Emilian–Romagnol wiki's, its Romagnol articles left out, each article
without the name of its dialect), Church Slavonic (its sentences that
write a letter of its own, ѣ є ꙑ ѥ…: its references are Russian),
Zeelandic, Batak Mandailing, Ghanaian Pidgin and Igala (their sentences
that write their own words: their references are English; and those
that show a failed template's name), Arakanese (in the Myanmar script,
cut into syllables), Veluws (the Dutch Low Saxon wiki's articles it
files as written in Veluws), Betawi, Nupe and Gilaki (their sentences
that write their own words: their references are English, some of
Betawi's Indonesian, Gilaki's village articles Persian; Gilaki's read
as Persian's keys type it), Bishnupriya Manipuri and Tai Nuea (in Tai
Le, its Chinese and Latin words left out, every 10th article held
out), the last fifteen's bots' stubs left out but three of a shape;
Newari's from its
own beside its news
(GlotCC's, below); and Armenian's and Persian's from theirs beside their
Tatoeba sentences, the first 400 000 sentences of each, whose held-out
articles are not committed (Tatoeba's held-out text measures them). The text is each wiki's CirrusSearch dump of 2026-11-08
(Wikimedia's `cirrus_search_index`, an article's plain text a JSON
field), pinned by sha256 in `scripts/fetch-wikipedia.sh`, cut into
sentences by `scripts/wikipedia-text.py`, and where a word in another
script stands, that word left out (the references' and English
passages' Latin): at most 400 000 a language.
Every 20th article is held out of it, and the first 500 sentences of
those are `eval/<lang>.txt`. Nineteen more learn from the Wikimedia
Incubator's test wikis, in its dump of the same date (each article's
title starting with its wiki's prefix, `Wp/mag/`): Adilabad Gondi's,
Khowar's and Montenegrin's alone (Montenegrin's Latin sentences, its few
Cyrillic ones left out), and fourteen small wikis' alone, every 10th of
their articles held out and their bots' stubs left out but three of a
shape (Rinconada Bikol's; Jambi, Makassar, Manado, Palembang and
Kelantan-Pattani Malay's; Makassarese's, Pamona's, Ibibio's, East
Franconian's, Southern Kurdish's, Western Balochi's, Rangpuri's and
Abron's, its sentences that write its ɛ or ɔ), Vai's, in its own
script, its Latin words left out and every 10th article held out,
Magahi's beside its FineWeb-2 text; an article or sentence heavy with Hindi's, Urdu's,
Persian's, Indonesian's or English's own words is left out.
**Wikipedia's text, and the Incubator's, is CC BY-SA 4.0**, share-alike:
the packs of those fifty-five languages, their models and that held-out
text carry those terms, with attribution to the wikis' contributors (each article's
history, at its wiki, names them).

### FineWeb-2 (web text)

Contains information from [FineWeb-2](https://huggingface.co/datasets/HuggingFaceFW/fineweb-2)
which is made available under the [ODC Attribution License](https://opendatacommons.org/licenses/by/1-0/).

Three hundred and twenty-three models learn from FineWeb-2's web text alone: those of Bodo,
Dogri, Chhattisgarhi, Brahui, Kokborok, Mizo and Khasi, which have no
Wikipedia of their own, of Sindhi in Devanagari (`sd-Deva`) and Meitei
in the Bengali script (`mni-Beng`), whose wikis are in their other
scripts, of Lao, Khmer and Burmese, whose wikis are small or written as
few type, and of thirty-eight languages that had a keyboard and no
model (Afrikaans, Albanian, Bashkir, Bosnian, Cebuano, Chichewa,
Corsican, Dhivehi, Faroese, Frisian, Haitian Creole, Hausa, Hawaiian,
Igbo, Irish, Javanese, Kazakh, Kinyarwanda, Kyrgyz, Luxembourgish,
Malagasy, Maltese, Māori, Samoan, Scottish Gaelic, Sesotho, Shona,
Slovenian, Somali, Sundanese, Swahili, Tajik, Turkmen, Uzbek, Welsh,
Xhosa, Yoruba and Zulu), of Central Kurdish (Sorani), and of nineteen
languages of Southeast Asia (Acehnese, Balinese, Banjar, Batak Toba,
Central Bikol, Hakha Chin, Hiligaynon, Hmong Daw, Iban, Iloko, Iu Mien,
Jingpho, Kadazandusun, Kapampangan, Karo, Kinaray-a, Pangasinan,
Simalungun and Tetum), and of twenty-six of Europe's regional languages
(Alemannic, Aragonese, Asturian, Bavarian, Crimean Tatar, Friulian,
Kashubian, Kölsch, Ladino, Latgalian, Ligurian, Limburgish, Low German,
Lower Sorbian, Neapolitan, Norwegian Nynorsk, Palatine German,
Piedmontese, Rusyn, Sardinian, Scots, Sicilian, Silesian, Upper Sorbian,
Võro and Walloon), of fifteen of Russia's and the north's (Buryat, Chuvash,
Erzya, Hill and Meadow Mari, Kalmyk, Karelian, Komi-Permyak, Komi-Zyrian,
Livvi, Moksha, Northern Sami, Sakha, Tuvan and Veps), and of twenty-three
more that had a keyboard and no model (Akan, Aymara, Bambara, Ewe, Fijian,
Greenlandic, Inuktitut, Kirundi, Lingala, Luganda, Oromo, Quechua, Romansh,
Sepedi, Setswana, Southern Ndebele, Swati, Tahitian, Tongan, Tsonga,
Udmurt, Venda and Wolof), and of nineteen of Iran's and the Caucasus'
(Abkhaz, Adyghe, Avar, Chechen, Dargwa, Ingush, Kabardian,
Karachay-Balkar, Karakalpak, Kirmanjki, Kumyk, Lak, Lezgian, Mazandarani,
Mingrelian, South Azerbaijani, Tabasaran, Talysh and Zazaki), and of
twenty-four of Africa's (Acholi, Baoulé, Bemba, Cape Verdean Creole,
Efik, Fon, Isoko, Jula, Kikongo, Kikuyu, Kimbundu, Kiluba, Kwanyama,
Mossi, Ndonga, Northern Ndebele, Dholuo, Réunion Creole, Sango,
Seychellois Creole, Tiv, Tshiluba, Umbundu and Wolaytta), and of eighteen
of the Americas', the Pacific's and the classical and constructed (Bislama,
Chamorro, Classical Syriac, Cusco Quechua, Guadeloupean Creole, Hiri Motu,
Interlingua, Jamaican Patois, Kʼicheʼ, Kichwa, Kiribati, Latin, Papiamento,
Qʼeqchiʼ, Sranan Tongo, Tok Pisin, Tuvaluan and Yucatec Maya), and of six
more that had a keyboard and no model (Dzongkha, Fula, Guaraní, Serbian
in Latin letters, Tamazight in Tifinagh and Tibetan), and of twenty-one
of Africa's and Southeast Asia's long tail (Bulu, Dagbani, Dangme, Dinka,
Edo, Farefare, Kamba, Kipsigis, Konzo, Koongo, Madurese, Makhuwa, Moba,
Ndau, Nias, Nkore, Sena, Sidamo, Songe, Teso and Yao), and of eleven
more of Europe's regional languages (Cornish, Extremaduran, Gagauz,
Ladin, Lombard, Manx, Picard, Pontic Greek, Samogitian, Venetian and
West Flemish), and of four of Myanmar's (Mon, Pa'O, Sgaw Karen and
Shan), and of four of East Asia's (Amis, Hakka, Southern Min and
Zhuang), and of twenty-eight more of Africa's long tail (Akoose, Alur,
Basaa, Chopi, Chuwabu, Dan, Fante, Gamo, Gogo, Gourmanché, Gulay, Haya,
Hehe, Kafa, Konkomba, Lomwe, Lugbara, Mende, Nande, Northern Grebo,
Ntcham, Nyoro, Sakalava Malagasy, Sukuma, Supyire, Tswa, Xaasongaxango
and Zande), and of twenty-three more of the long tail (Buhid, Dagaare,
Gorontalo, Guianan Creole, Gusii, Hawaiian Pidgin, Itawit, Koronadal
Blaan, Lampung Api, Maasai, Makonde, Mandinka, Maranao, Meru, Ngaju,
Niuean, Nyakyusa, Saint Lucian Creole, Sasak, South Bolivian Quechua,
Tetela, Tumbuka and Wayuu), and of twelve more (Aja, Ambonese Malay,
Coptic, Hunsrik, Kankanaey, Lahu, Livonian, Mamasa, Ottawa, Plains Cree,
Swampy Cree and Tokelauan), and of thirteen more (Dungan, Ewondo, Giryama,
Hanunoo, Kanuri, Lango, Ledo Kaili, Makhuwa-Meetto, Nenets, Sassarese,
Tausug, Tooro and Toraja-Sa'dan), and of three more (Balkan Romani,
Northwestern Ojibwe and Vlax Romani), and of Amharic, Tigrinya and Sebat
Bet Gurage, which the Ge'ez composer offers the words of (Sebat Bet
Gurage's text in FineWeb-2 is mostly its New Testament, the Bible Society
of Ethiopia's translation as bible.is gives it); and Assamese's from it beside its Tatoeba sentences, Magahi's beside its Incubator text, and
Nigerian Pidgin's, Kabiye's, Minangkabau's and Waray's beside GlotCC's
(below).
FineWeb-2 (Hugging Face; Penedo et al., "FineWeb2: One
Pipeline to Scale Them All", arXiv 2506.20920) is text from Common
Crawl's crawls, sorted by language; **it is ODC-By 1.0, and its use is
subject to [Common Crawl's Terms of Use](https://commoncrawl.org/terms-of-use)**.
Its parquet files, at one revision of the dataset, are pinned by sha256
in `scripts/fetch-fineweb.sh`; `scripts/fineweb-text.py` keeps the
documents that read as their language. The models made from it (word
lists and the counts of words and word pairs and triples) are what this
repository and its packs hold; the text itself is not, neither the
corpus nor the held-out text: it is the web's, whose pages keep their
own rights. Each pack with such a model names FineWeb-2 among its
sources, with this licence, as ODC-By asks of what is made from it.

### MADLAD-400 (web text)

Garhwali's, Rajasthani's, Krio's, Tandroy-Mahafaly Malagasy's, Hawu's,
Kituba's and Ayacucho Quechua's models learn from MADLAD-400's web text
alone, which neither Wikipedia nor FineWeb-2 has of them: MADLAD-400
(Google; Kudugunta et al., "MADLAD-400: A Multilingual And
Document-Level Large Audited Dataset", arXiv 2309.04662; at Hugging Face
as `allenai/MADLAD-400`), text from Common Crawl sorted by language and
audited. **Its card releases it under CC BY 4.0** (its metadata says
ODC-By 1.0), **its use subject to Common Crawl's Terms of Use**. Its
files, at one revision, are pinned by sha256 in `scripts/fetch-madlad.sh`;
`scripts/fineweb-text.py` keeps the documents that read as their
language. As with FineWeb-2, the models made from it are what this
repository and its packs hold, never the text; each pack with such a
model names MADLAD-400 among its sources, with that licence.

### The African Storybook (stories)

Masaaba's and Turkana's models learn from the African Storybook's
stories alone, which no wiki or web text here has of them: picture books
for children written and translated by teachers and writers across
Africa and published at [africanstorybook.org](https://www.africanstorybook.org/)
(an initiative of Saide), each under a Creative Commons licence. They
are read as the Global African Storybook Project's
[asp-raw-db](https://github.com/global-asp/asp-raw-db) keeps them, its
archive at one commit pinned by sha256 in `scripts/fetch-storybooks.sh`,
and only the stories under **CC BY 3.0 or CC BY 4.0** are read (those
under CC BY-NC are left out): 90 of Masaaba's (Lumasaaba) and 151 of
Turkana's (Ng'aturkana). Each story's author (who wrote or translated
it) and illustrator are named in its file there and on its page at
africanstorybook.org, with its licence. The models made from them
(word lists and the counts of words and word pairs and triples) are what
this repository and its packs hold, never the stories; each pack with
such a model names the African Storybook among its sources, with those
licences. Bukusu's and Malawi Lomwe's keyboards take their letters from
their CC BY stories there (11 and 19).

### StoryWeaver (stories)

Halbi's, Korku's and Nahuatl's models learn from StoryWeaver's stories
alone, which no wiki or web text here has of them: picture books for
children in hundreds of languages, written, translated and illustrated
by its community and by publishers, and published at
[storyweaver.org.in](https://storyweaver.org.in/) by Pratham Books,
**each under CC BY 4.0** (its terms of use; the Read Along videos alone
are under CC BY-NC-ND, and none is read here). Its reader shows a
story's pages to a signed-in account, and they were read through it: the
stories each model learns from are named in
`scripts/storyweaver-stories.tsv` by their id and slug, each pinned by
the sha256 of its text, and every one's attribution page says CC BY 4.0
(177 of Halbi's, 105 of Korku's, 419 of Nahuatl's). Each story's page,
`https://storyweaver.org.in/en/stories/<slug>`, names its author,
translator, illustrator and publisher, the story it was translated
from, and its licence. The models made from them (word lists and the
counts of words and word pairs and triples) are what this repository
and its packs hold, never the stories; each pack with such a model names
StoryWeaver among its sources, with that licence.

### SMOL (translations)

Fourteen models learn from SMOL's translations alone, which no wiki or
web text here has of them: Ahirani's, Bagheli's, Bagri's, Bundeli's,
Kumaoni's, Kurukh's, Mandeali's, Nimadi's, Surgujia's, Surjapuri's,
Mundari's and Kangri's, in Devanagari, Garo's in Latin letters, and
Zarma's. SMOL (Caswell et al., "SMOL: Professionally translated parallel
data for 115 under-represented languages", 2025; at Hugging Face as
[google/smol](https://huggingface.co/datasets/google/smol)) is English
text translated by professional translators, a document a sentence at
a time (SmolDoc) and single sentences (SmolSent), **released by Google
Research under CC BY 4.0**. Its files are read at one revision, pinned
by sha256 in `scripts/fetch-smol.sh`. The models made from them (word
lists and the counts of words and word pairs and triples) are what this
repository and its packs hold, never the translations; each pack with
such a model names SMOL among its sources, with that licence. Pulaar's
keyboard takes its letters from SMOL's Pulaar sentences, as CLDR's Fula
lists them.

### GlotCC (web text)

Newari's model learns from GlotCC's web text beside its wiki's, whose
articles are half village stubs a bot wrote, and Nigerian Pidgin's,
Kabiye's, Minangkabau's and Waray's beside FineWeb-2's, the sentences that
lacks: GlotCC v1.0 (Kargaran, Yvon
and Schütze, "GlotCC: An Open Broad-Coverage CommonCrawl Corpus and
Pipeline for Minority Languages", NeurIPS 2024; at Hugging Face as
`cis-lmu/GlotCC-V1`), text from Common Crawl sorted by language for a
thousand minority languages. **Its card licenses its packaging, metadata
and annotations CC0 1.0, the text itself being under [Common Crawl's
Terms of Use](https://commoncrawl.org/terms-of-use)**. Its files, at one
revision, are pinned by sha256 in `scripts/fetch-glotcc.sh`;
`scripts/fineweb-text.py` keeps the documents that read as their
language. As with FineWeb-2, the models made from it are what this
repository and its packs hold, never the text; each pack names GlotCC
among its sources.

**The Dakshina dataset is CC BY-SA 4.0**, share-alike: the packs of
those twelve languages, their models and the held-out text in `eval/`
carry those terms, with attribution, besides Tatoeba's where they use
it, and so do Hinglish's pack and its held-out text. Its authors ask that it be cited: Brian Roark, Lawrence
Wolf-Sonkin, Christo Kirov, Sabrina J. Mielke, Cibu Johny, Işın
Demirşahin and Keith Hall, "Processing South Asian Languages Written in
the Latin Script: the Dakshina Dataset", LREC 2020.

**The corpora are not committed.** `scripts/fetch-corpora.sh` (which
holds the id→code map above) downloads them on demand; `sources.lock`
records the sha256 of the corpus every committed model was built from;
and every release made by the refresh workflow attaches those exact
corpora (`ki-osk-lang-corpora-<version>.tar.xz`).

**Tatoeba sentences are CC BY 2.0 FR.** The subsets, the held-out text
in `eval/`, and the derived n-gram tables and lexicons inherit those
terms; attribution to the Tatoeba project must travel with any
redistribution.

To enlarge coverage, append from any of the following into
`predict/corpora/<lang>.txt` and re-run `make predict-assets`:

- **Wikipedia dumps** (per-language text extracts) — CC BY-SA 3.0 / CC BY-SA 4.0.
- **Project Gutenberg** — public domain in the US; per-country status varies.
- **OpenSubtitles** — per-language check required; some languages excluded.
- **Additional Tatoeba sentences** beyond the first 120 000 per
  language — same CC BY 2.0 FR terms.

Adding share-alike sources (Wikipedia in particular) downgrades the
redistributable license of the resulting binaries to the strongest
copyleft in the mix. A hand-enlarged corpus is also overwritten by the
next `make fetch-corpora`.

## Transliteration tables

`translit/<lang>.txt` say how the Latin letters typed for a language
become its script (`osk_data::translit`): Assamese, Bengali, Bhojpuri,
Greek, Gujarati, Hindi, Armenian, Kannada, Maithili, Malayalam, Marathi,
Mongolian, Nepali, Odia, Punjabi (Gurmukhi and Shahmukhi), Persian,
Pashto, Sanskrit, Santali, Sindhi (Perso-Arabic and Devanagari),
Sinhala, Tamil, Telugu, Urdu, Yiddish, Awadhi, Kashmiri, Konkani, Meitei
(Meetei Mayek and the Bengali script), Saraiki, Tulu, Bodo, Dogri,
Chhattisgarhi, Magahi, Brahui, Garhwali and Rajasthani. They were
written here, MIT licensed,
each joining its language's pack; what they choose among, the
language's lexicon chooses. The costs of the twelve the Dakshina dataset
romanizes (Bengali, Gujarati, Hindi, Kannada, Malayalam, Marathi,
Punjabi, Sindhi, Sinhala, Tamil, Telugu and Urdu) were then fitted to how
people romanized its lexicons' training words (`make fit-translit`,
`crates/osk-data-tools/src/bin/fit_translit.rs`): the numbers after the
colons moved, each held to the one written here, and nothing of the
dataset's words in the tables. Ten moved (Gujarati's and Kannada's did
no better so, and stayed as written); their packs name the dataset for
it, beside the Wikipedia text their models learn from (CC BY-SA 4.0).
The costs of the ten AI4Bharat's Aksharantar dataset measures (below)
were fitted the same way, to four fifths of the words its train and
valid splits romanized by hand and of the names it took from Wikidata,
chosen by the other fifth, its test words left out; seven moved
(Assamese's, Nepali's and Odia's stayed as written), and their packs
name it (its card: the manually collected data CC BY, the mined and
existing data CC0; `packs.toml`'s `[[translit.fitted]]`). And thirteen
tables of languages neither dataset has (Awadhi, Bhojpuri, Brahui,
Chhattisgarhi, Dotyali, Garhwali, Khowar, Magahi, Newari, Rajasthani,
Western Punjabi, Saraiki, Tulu) were fitted to Dakshina's romanizations
of the words their lexicons share with Hindi's, Urdu's or Kannada's, and
are measured so; Adilabad Gondi's and Angika's are Telugu's and Hindi's
tables and fitted costs, measured on the words their lexicons share with
Telugu's and Hindi's. Their packs name the dataset,
adding no licence (Chhattisgarhi's, Brahui's, Garhwali's and
Rajasthani's models are not otherwise share-alike).
The tables
are measured on romanizations nothing here ships: the Dakshina
dataset's test sentences and words, and for those ten Aksharantar's
test words ("Aksharantar: Towards Building Open Transliteration Tools
for the Next Billion Users", arXiv 2205.03018), which
`scripts/fetch-aksharantar.sh` downloads to measure and fit, and for
Greek AUEB's real-world Greeklish (Toumazatos, Pavlopoulos,
Androutsopoulos and Vassos, "Still All Greeklish to Me: Greeklish to
Greek Transliteration", LREC-COLING 2024; github.com/nlpaueb/greeklish,
Apache-2.0), which `scripts/fetch-greeklish.sh` downloads to measure,
and for Yiddish, Armenian, Persian, Pashto, Mongolian and Santali
Wiktionary's words and their romanizations as kaikki.org extracts them
(CC BY-SA 4.0), which `scripts/fetch-wiktionary-romanized.sh` downloads
to measure.
Their layouts, `layouts/<lang>-abc.json`, are the US layout's keys
(`scripts/build-translit-layouts.py`), under the keyboard layouts'
notices below. Vai's and Nuosu's tables, `translit/vai.txt` and
`translit/ii.txt`, were not written here: they are what Keyman's
keyboards for the two languages type each syllable as, found by running
the keyboards' rules, under the keyboards' notices (with the keyboard
layouts, below), and the two languages' own layouts type through them.

### English words (CMUdict)

`translit/<lang>.english.txt` are the English words a language with a
table writes in its own script (computer: कंप्यूटर), which people type as
English spells them, not as the table reads its own sounds; the table
reads each such word typed whole as those words too
(`osk_data::translit::Scheme::with_english`). They are made from the
Carnegie Mellon University Pronouncing Dictionary, CMUdict
(github.com/cmusphinx/cmudict, at commit 74790861f652, which
`scripts/fetch-cmudict.sh` downloads and pins by sha256): how American
English says each word, spelled in Latin letters as the language's sounds
are typed, in the few ways this repository's
`crates/osk-data-tools/src/bin/build_english.rs` gives, and read by the
language's table; the likeliest reading that is a word of the language's
lexicon is kept, where the table does not read the English spelling so
already, and, for a word English's own lexicon (`predict/en.fst`) lacks,
where the table reads it as no word of the language (`make
english-assets`). Each joins its language's pack, under CMUdict's
notice:

```text
Copyright (C) 1993-2015 Carnegie Mellon University. All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:

1. Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.
   The contents of this file are deemed to be source code.

2. Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in
   the documentation and/or other materials provided with the
   distribution.

This work was supported in part by funding from the Defense Advanced
Research Projects Agency, the Office of Naval Research and the National
Science Foundation of the United States of America, and by member
companies of the Carnegie Mellon Sphinx Speech Consortium. We acknowledge
the contributions of many volunteers to the expansion and improvement of
this dictionary.

THIS SOFTWARE IS PROVIDED BY CARNEGIE MELLON UNIVERSITY ``AS IS'' AND
ANY EXPRESSED OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL CARNEGIE MELLON UNIVERSITY
NOR ITS EMPLOYEES BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

## Word lists

`wordbreak/th.txt`, `lo.txt`, `km.txt` and `my.txt`, the word lists that
cut Thai, Lao, Khmer and Burmese text into words (none is written with
spaces between them), in the trainer, the keyboard and `eval-predict`
alike, are **ICU's Thai, Lao, Khmer and Burmese word-break
dictionaries**, `icu4c/source/data/brkitr/dictionaries/thaidict.txt`,
`laodict.txt`, `khmerdict.txt` and `burmesedict.txt` of ICU 79.1,
committed as ICU has them, their copyright lines with them
(`scripts/fetch-word-lists.sh` pins them by sha256). Thai's says:
"Copyright (C) 2016 and later: Unicode, Inc. and others. License & terms
of use: http://www.unicode.org/copyright.html. Copyright (c) 2006-2015
International Business Machines Corporation, Apple Inc., and others. All
Rights Reserved." Lao's: "Copyright (C) 2016 and later: Unicode, Inc. and
others. License & terms of use: http://www.unicode.org/copyright.html.
Copyright (c) 2015 International Business Machines Corporation and
others. All Rights Reserved.", and that it is the Lao Word List of Brian
Eugene Wilson and Robert Martin Campbell, below. Khmer's: "Copyright (C)
2016 and later: Unicode, Inc. and others. License & terms of use:
http://www.unicode.org/copyright.html. Copyright (c) 2011-2015
International Business Machines Corporation and others. All Rights
Reserved." Burmese's: "Copyright (C) 2016 and later: Unicode, Inc. and
others. License & terms of use: http://www.unicode.org/copyright.html.
Copyright (c) 2015 International Business Machines Corporation and
others. All Rights Reserved.", and that it is LeRoy Benjamin Sharon's
word list, below. Thai's model (`predict/th.*`) is built from Tatoeba's
sentences cut with its list, Lao's, Khmer's and Burmese's (`predict/lo.*`,
`km.*`, `my.*`) from FineWeb-2's text cut with theirs. ICU's licence, for
the files, from ICU's `LICENSE`:

```text
UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 1991-2026 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
Data Files or Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, and/or sell
copies of the Data Files or Software, and to permit persons to whom the
Data Files or Software are furnished to do so, provided that either (a)
this copyright and permission notice appear with all copies of the Data
Files or Software, or (b) this copyright and permission notice appear in
associated Documentation.

THE DATA FILES AND SOFTWARE ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY
KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF
THIRD PARTY RIGHTS.

IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS INCLUDED IN THIS NOTICE
BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES,
OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THE DATA
FILES OR SOFTWARE.

Except as contained in this notice, the name of a copyright holder shall
not be used in advertising or otherwise to promote the sale, use or other
dealings in these Data Files or Software without prior written
authorization of the copyright holder.

SPDX-License-Identifier: Unicode-3.0
```

and, for what was written before ICU 58:

```text
ICU License - ICU 1.8.1 to ICU 57.1

COPYRIGHT AND PERMISSION NOTICE

Copyright (c) 1995-2016 International Business Machines Corporation and others
All rights reserved.

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, and/or sell copies of the Software, and to permit persons
to whom the Software is furnished to do so, provided that the above
copyright notice(s) and this permission notice appear in all copies of
the Software and that both the above copyright notice(s) and this
permission notice appear in supporting documentation.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT
OF THIRD PARTY RIGHTS. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
HOLDERS INCLUDED IN THIS NOTICE BE LIABLE FOR ANY CLAIM, OR ANY
SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES WHATSOEVER
RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF
CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

Except as contained in this notice, the name of a copyright holder
shall not be used in advertising or otherwise to promote the sale, use
or other dealings in this Software without prior written authorization
of the copyright holder.

All trademarks and registered trademarks mentioned herein are the
property of their respective owners.
```

Source: https://github.com/unicode-org/icu/blob/release-79.1/icu4c/source/data/brkitr/dictionaries/thaidict.txt

Lao's list is ICU's copy of the **Lao Word List** of Brian Eugene Wilson
and Robert Martin Campbell, "with special thanks to Erik Mundall, Arlyta
Keosamone, and Bualong Nyoukmai" (its version of 2020-11-22, as ICU's
file says), whose notice, in that file's header, goes with it:

```text
Copyright (C) 2013 Brian Eugene Wilson, Robert Martin Campbell.
All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

	Redistributions of source code must retain the above copyright notice, this
	list of conditions and the following disclaimer. Redistributions in binary
	form must reproduce the above copyright notice, this list of conditions and
	the following disclaimer in the documentation and/or other materials
	provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

Source: https://github.com/unicode-org/icu/blob/release-79.1/icu4c/source/data/brkitr/dictionaries/laodict.txt

Burmese's list is ICU's copy of LeRoy Benjamin Sharon's Burmese word
list, "with special thanks to Robert Martin Campbell" (from the my-l10n
project and NCSoft's Myanmar-English dictionary, its header says; part of
github.com/kanyawtech/myanmar-karen-word-lists), whose notice, in that
file's header and in ICU's `LICENSE`, goes with it:

```text
Copyright (c) 2013, LeRoy Benjamin Sharon
All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

  Redistributions of source code must retain the above copyright notice, this
  list of conditions and the following disclaimer.

  Redistributions in binary form must reproduce the above copyright notice, this
  list of conditions and the following disclaimer in the documentation and/or
  other materials provided with the distribution.

  Neither the name Myanmar Karen Word Lists, nor the names of its
  contributors may be used to endorse or promote products derived from
  this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

Source: https://github.com/unicode-org/icu/blob/release-79.1/icu4c/source/data/brkitr/dictionaries/burmesedict.txt

`wordbreak/shn.txt`, `mnw.txt`, `ksw.txt` and `blk.txt`, which cut Shan,
Mon, Sgaw Karen and Pa'O text, have no list of words behind them: each is
the syllables its language's FineWeb-2 text writes twice or more,
listed by `build-syllables` (`make syllable-lists`). They are made from
FineWeb-2, under its ODC-By 1.0 as above. `wordbreak/rki.txt`, which
cuts Arakanese text, is the syllables its Wikipedia's text writes twice
or more, listed the same way: it is made from that text, under its
CC BY-SA 4.0 as above, share-alike.

## Dialect flags

`predict/en.dialects.bin` (which varieties of English write a word, and
its spellings across them) is built by `build-dialects` from two sources,
fetched by `scripts/fetch-dialect-sources.sh` and recorded by sha256 in
`sources.lock`:

- **English Speller Database (ESDB, formerly SCOWLv2)** by Kevin
  Atkinson, <https://github.com/en-wl/wordlist>, `data/scowl-pre.txt` at
  commit `c107d90`: its spelling codes and variant levels. Its notice,
  which goes with the file:

  > Copyright 2000-2026 by Kevin Atkinson
  >
  > Permission to use, copy, modify, distribute, and sell any part of the
  > English Speller Database (ESDB, previously known as SCOWLv2), or word
  > lists created from it, is hereby granted without fee, provided that
  > the above copyright notice appears in all copies and that both the
  > above copyright notice and this notice appear in supporting
  > documentation. Kevin Atkinson makes no representations about the
  > suitability of this database for any purpose. It is provided "as is"
  > without express or implied warranty.

- **Harper's English dictionary** (`dictionary.dict` of the `harper-core`
  3.0.0 crate, <https://github.com/automattic/harper>), Apache License
  2.0 (<https://www.apache.org/licenses/LICENSE-2.0>): the words it marks
  as Indian English alone.

`predict/pt.dialects.bin` and `predict/es.dialects.bin` (Brazil's and
Portugal's, Spain's and Latin America's spellings and words) are built
by `build-dialects` from **Wiktionary**'s Portuguese and Spanish entries
(its English edition, <https://en.wiktionary.org/>), by the Wiktionary
contributors, as **kaikki.org** extracts them with wiktextract
(<https://kaikki.org/dictionary/Portuguese/>,
<https://kaikki.org/dictionary/Spanish/>; Tatu Ylonen, "Wiktextract:
Wiktionary as Machine-Readable Structured Data", LREC 2022). Wiktionary's
text is under the Creative Commons Attribution-ShareAlike 4.0 licence
(<https://creativecommons.org/licenses/by-sa/4.0/>), and so are these two
tables, which are adaptations of it: **share-alike**. They are fetched by
`scripts/fetch-dialect-sources.sh`, and `sources.lock` records the sha256
of the extract each was built from. Hunspell's Portuguese and Spanish
dictionaries were used to check them, not as a source; nothing of them
is in the tables.

## Keyboard layouts

`layouts/<id>.json`: which keys a layout has and what each types. The
national layouts were generated from **xkeyboard-config**'s keymaps,
read through libxkbcommon (`scripts/build-layout.py`,
`build-national-grid.py`, `relabel-layout.py`, `relabel-national.py`):
each key's letters are the ones the keymap a layout names in
`xkb_layout` produces. The layout structure around them — rows, widths,
symbol pages, the control row — was written for ki-osk, under MIT. The
layouts of Bhojpuri, Maithili, Sanskrit, Awadhi, Bodo, Dogri,
Chhattisgarhi, Magahi and Sindhi in Devanagari are Hindi's, Konkani's
Marathi's, Tulu's Kannada's, Meitei's in the Bengali script Bengali's,
Western Punjabi's, Saraiki's, Kashmiri's and Brahui's Urdu's and
Yiddish's Hebrew's, renamed, with the letters each language adds on long
presses (`scripts/build-derived-layouts.py`); Pa'O's is Burmese's, its
tones ꩻ and ႏ in place of ဩ and ၏, and Eastern Pwo Karen's too, its
ႋ ၜ ၠ in place of ဩ ၏ ၒ; Rangpuri's is Assamese's, Amdo and Khams
Tibetan's Tibetan's, Sadri's Hindi's, Maharashtrian Konkani's Marathi's,
Kutchi's Gujarati's, Tày's Vietnamese's and Northern Thai's in the Thai
script Thai's; and Sebat Bet Gurage's is Amharic's, ʷ and ʸ in place of
v and the hyphen, its letters (which the Ge'ez composer types) and its
name those SIL's Locale Data Repository lists for it (SLDR,
`sldr/s/sgw.xml`, https://github.com/silnrsi/sldr, under the MIT
License: Copyright (c) 2014-2023 SIL International). Sgaw Karen's, Northwestern Ojibwe's, the thirteen of a
script of their own, Chakma's to Osage's, and five more, Gorani's to
Northern Thai's in Tai Tham, are Keyman's (below), and Vai's and Nuosu's
the Latin letters Keyman's keyboards type their syllables with.
xkeyboard-config's copyright and permission notices, which go with every
copy (its `COPYING`, as xkeyboard-config 2.48 ships it):

```text
Copyright 1996 by Joseph Moss
Copyright (C) 2002-2007 Free Software Foundation, Inc.
Copyright (C) Dmitry Golubev <lastguru@mail.ru>, 2003-2004
Copyright (C) 2004, Gregory Mokhin <mokhin@bog.msu.ru>
Copyright (C) 2006 Erdal Ronahî

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation, and that the name of the copyright holder(s) not be used in
advertising or publicity pertaining to distribution of the software without
specific, written prior permission.  The copyright holder(s) makes no
representations about the suitability of this software for any purpose.  It
is provided "as is" without express or implied warranty.

THE COPYRIGHT HOLDER(S) DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
EVENT SHALL THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY SPECIAL, INDIRECT OR
CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THIS SOFTWARE.


Copyright (c) 1996  Digital Equipment Corporation

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be included
in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL DIGITAL EQUIPMENT CORPORATION BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR
THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of the Digital Equipment
Corporation shall not be used in advertising or otherwise to promote
the sale, use or other dealings in this Software without prior written
authorization from Digital Equipment Corporation.


Copyright 1996, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall
not be used in advertising or otherwise to promote the sale, use or
other dealings in this Software without prior written authorization
from The Open Group.


Copyright 2004-2005 Sun Microsystems, Inc.  All rights reserved.

Permission is hereby granted, free of charge, to any person obtaining a
copy of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice (including the next
paragraph) shall be included in all copies or substantial portions of the
Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.


Copyright (c) 1996 by Silicon Graphics Computer Systems, Inc.

Permission to use, copy, modify, and distribute this
software and its documentation for any purpose and without
fee is hereby granted, provided that the above copyright
notice appear in all copies and that both that copyright
notice and this permission notice appear in supporting
documentation, and that the name of Silicon Graphics not be
used in advertising or publicity pertaining to distribution
of the software without specific prior written permission.
Silicon Graphics makes no representation about the suitability
of this software for any purpose. It is provided "as is"
without any express or implied warranty.

SILICON GRAPHICS DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS
SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
AND FITNESS FOR A PARTICULAR PURPOSE. IN NO EVENT SHALL SILICON
GRAPHICS BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL
DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE
OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION  WITH
THE USE OR PERFORMANCE OF THIS SOFTWARE.


Copyright (c) 1996  X Consortium

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE X CONSORTIUM BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of the X Consortium shall
not be used in advertising or otherwise to promote the sale, use or
other dealings in this Software without prior written authorization
from the X Consortium.


Copyright (C) 2004, 2006 Ævar Arnfjörð Bjarmason <avarab@gmail.com>

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of a copyright holder shall
not be used in advertising or otherwise to promote the sale, use or
other dealings in this Software without prior written authorization of
the copyright holder.


Copyright (C) 1999, 2000, 2007, 2009 by Anton Zinoviev <anton@lml.bas.bg>

This software may be used, modified, copied, distributed, and sold,
both in source and binary form provided that the above copyright
notice and these terms are retained. The name of the author may not
be used to endorse or promote products derived from this software
without prior permission. THIS SOFTWARE IS PROVIDED "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES ARE DISCLAIMED. IN NO EVENT
SHALL THE AUTHOR BE LIABLE FOR ANY DAMAGES ARISING IN ANY WAY OUT
OF THE USE OF THIS SOFTWARE.


Copyright (C) 2002-2004 Soós Péter <sp@osb.hu>
Copyright (C) 2010 Andras Timar <timar@fsf.hu>

Permission is granted to anyone to use, distribute and modify
this file in any way, provided that the above copyright notice
is left intact and the author of the modification summarizes
the changes in this header.

This file is distributed without any expressed or implied warranty.
```

Source: https://gitlab.freedesktop.org/xkeyboard-config/xkeyboard-config

`layouts/ksw.json`, Sgaw Karen's, is **Keyman's sil_sgaw_karen** keyboard
(1.1.3, SIL Global), its touch layout's letter rows as its rules type
them (`scripts/build-keyman-layouts.py`, its files pinned by sha256),
under the MIT licence it carries:

```text
The MIT License (MIT)

Copyright © 2020-2025 SIL Global

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

Source: https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/sil/sil_sgaw_karen

`layouts/ojb.json`, Northwestern Ojibwe's, is **Keyman's fv_ojibwa**
keyboard (10.1.1, FirstVoices, SIL Global and the First Peoples'
Cultural Foundation), its touch layout's letter rows as its rules type
them, each syllable's long vowel and w-dotted forms on its long press
(`scripts/build-keyman-layouts.py`, its files pinned by sha256), under
the MIT licence it carries:

```text
The MIT License (MIT)

Copyright (c) 2015-2025 FirstVoices, SIL Global, 2015 First Peoples' Cultural Foundation

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

Source: https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/fv/fv_ojibwa

Thirteen layouts of a script of their own, Chakma's to Osage's, are
**Keyman's keyboards** for them, their touch layouts' rows of letters as
their rules type them (`scripts/build-keyman-layouts.py`, their files
pinned by sha256), each under the MIT licence it carries: the same text,
each with its own copyright line.

| Layout | Keyman keyboard | Copyright | Source |
|---|---|---|---|
| `layouts/ccp.json`, Chakma's | easy_chakma 2.0.1 | Copyright (c) 2015-2025 Bivuti Chakma | https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/e/easy_chakma |
| `layouts/rhg.json`, Rohingya's | hanifi_rohingya 1.0 | Copyright © 2024 SIL International | https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/h/hanifi_rohingya |
| `layouts/blt.json`, Tai Dam's | sil_tai_dam 2.3 | Copyright (c) 2007-2024 SIL International | https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/sil/sil_tai_dam |
| `layouts/kyu.json`, Western Kayah's | sil_kayah_kali 1.3.1 | Copyright (c) 2007-2025 SIL Global | https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/sil/sil_kayah_kali |
| `layouts/lis.json`, Lisu's | sil_lisu_standard 1.1.1 | Copyright (c) 2018-2024 SIL Global | https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/sil/sil_lisu_standard |
| `layouts/lep.json`, Lepcha's | sil_lepcha 1.1.1 | Copyright (c) 2010-2025 SIL Global | https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/sil/sil_lepcha |
| `layouts/kjg.json`, Khmu's | sil_kmhmu 1.8 | Copyright (c) 2008-2021 SIL International | https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/sil/sil_kmhmu |
| `layouts/saz.json`, Saurashtra's | saurashtra_inscript 1.0 | Copyright © 2024 SIL International | https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/s/saurashtra_inscript |
| `layouts/bax.json`, Bamum's | bamum 1.0 | Copyright © 2024 SIL International | https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/b/bamum |
| `layouts/cjm.json`, Eastern Cham's | sil_cham_phonetic 1.3 | Copyright © 2024-2025 SIL Global | https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/sil/sil_cham_phonetic |
| `layouts/tdd.json`, Tai Nuea's | tainua 1.0.5 | Copyright (c) 2016-2023 Sai Ai Wong | https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/t/tainua |
| `layouts/khb.json`, Tai Lue's | ntl_onekey 1.4 | Copyright (c) 2015-2019 SIL International | https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/n/ntl_onekey |
| `layouts/osa.json`, Osage's | osage_nation 1.2.4 | Copyright (c) 2018-2023 Osage Nation | https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/o/osage_nation |

```text
The MIT License (MIT)

Copyright (c) 2015-2025 Bivuti Chakma
Copyright © 2024 SIL International
Copyright (c) 2007-2024 SIL International
Copyright (c) 2007-2025 SIL Global
Copyright (c) 2018-2024 SIL Global
Copyright (c) 2010-2025 SIL Global
Copyright (c) 2008-2021 SIL International
Copyright © 2024 SIL International
Copyright © 2024 SIL International
Copyright © 2024-2025 SIL Global
Copyright (c) 2016-2023 Sai Ai Wong
Copyright (c) 2015-2019 SIL International
Copyright (c) 2018-2023 Osage Nation

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

Six more layouts are **Keyman's keyboards** read at a later commit, the
same way (`scripts/build-keyman-layouts.py`, their files pinned by
sha256), each under the MIT licence it carries: the same text, each with
its own copyright line. Northern Thai's is in Keyman's experimental
keyboards, which its maintainers have not reviewed; the International
Phonetic Alphabet's is laid out on the US keys, its long presses and
symbol views Keyman's.

| Layout | Keyman keyboard | Copyright | Source |
|---|---|---|---|
| `layouts/hac.json`, Gorani's | horami 2.0.2 | Copyright © 2024-2026 SIL Global | https://github.com/keymanapp/keyboards/tree/c9a2da84df94cc0acb5b0a684615e5e91ca95a0a/release/h/horami |
| `layouts/psi.json`, Southeastern Pashayi's | pashai 1.1.1 | Copyright © 2024-2026 SIL Global | https://github.com/keymanapp/keyboards/tree/c9a2da84df94cc0acb5b0a684615e5e91ca95a0a/release/p/pashai |
| `layouts/hnd.json`, Southern Hindko's | rac_hindko 1.0.1 | Copyright (c) 2018-2026 Rehmat Aziz Chitrali | https://github.com/keymanapp/keyboards/tree/c9a2da84df94cc0acb5b0a684615e5e91ca95a0a/release/rac/rac_hindko |
| `layouts/cld.json`, Chaldean Neo-Aramaic's | east_syriac_qwerty 2.0 | Copyright © 2025 Esho Marcus | https://github.com/keymanapp/keyboards/tree/c9a2da84df94cc0acb5b0a684615e5e91ca95a0a/release/e/east_syriac_qwerty |
| `layouts/nod.json`, Northern Thai's | tai_lanna_thai 1.1 | Copyright © 2025-2026 Jayasiri Jayasāro Bhikkhu | https://github.com/keymanapp/keyboards/tree/c9a2da84df94cc0acb5b0a684615e5e91ca95a0a/experimental/t/tai_lanna_thai |
| `layouts/und-fonipa.json`, the International Phonetic Alphabet's | sil_ipa 2.0.2 | Copyright (c) 2002-2025 SIL Global | https://github.com/keymanapp/keyboards/tree/c9a2da84df94cc0acb5b0a684615e5e91ca95a0a/release/sil/sil_ipa |

```text
The MIT License (MIT)

Copyright © 2024-2026 SIL Global
Copyright © 2024-2026 SIL Global
Copyright (c) 2018-2026 Rehmat Aziz Chitrali
Copyright © 2025 Esho Marcus
Copyright © 2025-2026 Jayasiri Jayasāro Bhikkhu
Copyright (c) 2002-2025 SIL Global

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

Vai's and Nuosu's keyboards, `layouts/vai.json` and `layouts/ii.json`,
and their transliteration tables, `translit/vai.txt` and
`translit/ii.txt`, are made from **Keyman's keyboards** that type the two
scripts' syllables as Latin letters (`scripts/build-keyman-composers.py`,
their files pinned by sha256): each table what each syllable is typed as,
found by running the keyboard's rules; each layout the Latin letters
they read and, Nuosu's, the punctuation its rules type. Each is under
the MIT licence its keyboard carries: the same text, each with its own
copyright line.

| Files | Keyman keyboard | Copyright | Source |
|---|---|---|---|
| `layouts/vai.json` and `translit/vai.txt`, Vai's | athinkra_vai 1.3 | Copyright © 2008-2024 Athinkra, LLC | https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/athinkra/athinkra_vai |
| `layouts/ii.json` and `translit/ii.txt`, Nuosu's | sil_yi 1.3.1 | Copyright (C) 2000-2022 SIL International | https://github.com/keymanapp/keyboards/tree/7225701d383cf8317f595bfd73481fcd87524f44/release/sil/sil_yi |

```text
The MIT License (MIT)

Copyright © 2008-2024 Athinkra, LLC
Copyright (C) 2000-2022 SIL International

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

Sebat Bet Gurage's keyboard (`layouts/sgw.json`, above) takes its letters
and its name from SIL's Locale Data Repository, `sldr/s/sgw.xml` at
https://github.com/silnrsi/sldr/tree/6ce8c0702dc61f395ada7a0dd098d420e59ec07d,
whose notice (its `LICENSE`) is:

```text
The MIT License (MIT)

Copyright (c) 2014-2023 SIL International

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Emoji

`emoji/catalogue.json` (the search catalogue: every fully-qualified emoji
with its CLDR short name, keywords, category and skin-tone variants) and
`layouts/emoji.json` (the picker's grid, a curated subset) are built by
`scripts/build-emoji.py` and `scripts/gen-emoji-layout.py` from the
**Unicode emoji data** (`emoji-test.txt`, Emoji 16.0) and the **CLDR**
English annotations (`annotations/en.xml`, `annotationsDerived/en.xml`),
both under the Unicode License v3:

```text
UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 1991-2023 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
Data Files or Software without restriction, including without limitation
the rights to use, copy, modify, merge, publish, distribute, and/or sell
copies of the Data Files or Software, and to permit persons to whom the
Data Files or Software are furnished to do so, provided that either (a)
this copyright and permission notice appear with all copies of the Data
Files or Software, or (b) this copyright and permission notice appear in
associated Documentation.

THE DATA FILES AND SOFTWARE ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY
KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF
THIRD PARTY RIGHTS.

IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS INCLUDED IN THIS NOTICE
BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES,
OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THE DATA
FILES OR SOFTWARE.

Except as contained in this notice, the name of a copyright holder shall
not be used in advertising or otherwise to promote the sale, use or other
dealings in these Data Files or Software without prior written
authorization of the copyright holder.
```

Sources: https://unicode.org/Public/emoji/16.0/emoji-test.txt,
https://github.com/unicode-org/cldr

## Keyboards from CLDR's exemplar characters

Fifty-seven layouts (made by `scripts/build-exemplar-layouts.py`) take
their letters from the **CLDR**'s exemplar characters, release 49
(`common/main/<lang>.xml`, and `exemplars/main/<lang>.xml` for the
languages below its basic coverage), under the Unicode License v3, whose
notice is above: thirty-two in Latin letters, Afar's to Zarma's
(`layouts/aa.json` … `layouts/xog.json`) and Adamawa Fulfulde's and
Bench's (`layouts/fub-latn.json`, `layouts/bcq-latn.json`, from their
Latin sets, `exemplars/main/fub_Latn.xml` and `bcq_Latn.xml`), and twenty-five on the Arabic
script's, Devanagari's and Bengali's keys, Balti's to Sylheti's
(`layouts/bft.json` … `layouts/syl.json`). Southern Balochi's
(`layouts/bcc.json`) is Balochi's with three letters more, and Dari's
(`layouts/prs.json`) Persian's keys with the Pashto letters CLDR's Dari
(`common/main/fa_AF.xml`) lists among its auxiliary exemplar characters.

Source: https://github.com/unicode-org/cldr/tree/96e54b273d2da4ec6fbad350ba19ff23fa24a588

## Offensive-word lists

`offensive/<lang>.txt`, 26 lists, used ONLY to keep words off a
suggestion strip (ki-osk's `--filter-offensive`, off by default); they
never restrict what can be typed.

- **LDNOOBW** ("List of Dirty, Naughty, Obscene and Otherwise Bad
  Words"), © 2012–2020 Shutterstock, Inc. — Creative Commons
  Attribution 4.0 International
  (https://creativecommons.org/licenses/by/4.0/). The 26 lists are its
  lists for those languages; `en.txt` adds the words below. Source:
  https://github.com/LDNOOBW/List-of-Dirty-Naughty-Obscene-and-Otherwise-Bad-Words
- **AOSP LatinIME wordlist** (English `flags=offensive` entries only) —
  Apache License 2.0 (https://www.apache.org/licenses/LICENSE-2.0),
  © 2008 The Android Open Source Project. 190 words merged into
  `offensive/en.txt`. Taken from the FUTO Keyboard checkout, whose own
  code is under the non-free FUTO Source First License — but its `NOTICE`
  scopes that license to changes AFTER commit
  `d847619a2b48945465f840b8d81644fa455cc115`, and every commit touching
  `dictionaries/` is an ancestor of it, authored by AOSP engineers. Only
  the Apache-2.0 AOSP portion is used here; no FUTO-licensed material is
  included. Source:
  https://android.googlesource.com/platform/packages/inputmethods/LatinIME/

## Pinyin tables

Written here, and MIT licensed with the code; listed for transparency
about where they came from.

- **Mandarin syllable inventory** (`pinyin/syllables.txt`) — derived
  from CC-CEDICT (see Dictionaries): the 413 toneless Pinyin syllables
  obtained by taking each CC-CEDICT entry's bracketed reading, dropping
  tone digits, and keeping what splits cleanly into a standard initial +
  final. The result is the phonological inventory of the language — a
  set of facts, not an excerpt — and carries no dictionary content: no
  headword, gloss or Hanzi from CC-CEDICT is reproduced by it.
- **Double-Pinyin scheme tables** (`pinyin/double-pinyin.json`) — the
  published key mappings of 小鹤双拼, 微软双拼, 自然码, 智能ABC, 紫光拼音 and
  拼音加加. These are layout definitions in the same category as an xkb
  symbols file: which key carries which initial or final, as specified by
  each scheme's own authors. Transcribed from the schemes' published
  definitions as tabulated by libpinyin (GPL) and then **independently
  verified** by an exhaustive encode/decode round trip over the syllable
  inventory above (`crates/osk-data/tests/pinyin.rs`) — no libpinyin
  code, structure or generated file is copied or linked.
