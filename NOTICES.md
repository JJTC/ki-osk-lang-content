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
`translit/`, which were written here.
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

Size tiers are recorded in the Makefile: 34 languages
(`PREDICT_COMPACT_LANGS`) keep 18 000 words and 30 000 n-grams, those
whose lexicons Wikipedia text widens (below) 30 000 and 60 000, or 40 000
and 90 000 (`PREDICT_LARGE_LANGS`: Marathi and the Dravidian languages),
the others 20 000 and 60 000.

### Wikipedia text (the Dakshina dataset)

Twelve South Asian languages' models are built from Wikipedia text too,
as the **Dakshina dataset** v1.0 (Google Research,
<https://github.com/google-research-datasets/dakshina>) gives it: the
first 300 000 lines (all, where fewer) of its filtered Wikipedia text
(`<lang>.wiki-filt.train.text.shuf.txt.gz`). It is the whole corpus of
Gujarati, Kannada, Malayalam, Punjabi, Sindhi, Sinhala, Tamil, Telugu and
Urdu, of which Tatoeba has too few sentences, and is added to Tatoeba's
for Hindi, Bengali and Marathi, Tatoeba's counted ten times. For those
nine, the first 500 lines of its held-out text
(`<lang>.wiki-filt.valid.text.shuf.txt.gz`) are `eval/<lang>.txt`.
`scripts/fetch-dakshina.sh` reads the archive, pinned by its sha256;
`sources.lock` records the sha256 of each language's text as
`dakshina/<lang>.txt`. Its romanization lexicons and romanized sentences
measure the transliteration tables (`make eval-translit`); none of them
is shipped.

Hinglish (`predict/hi-Latn.*`, the `hi-Latn` pack) is Hindi's corpora —
Tatoeba's sentences and Dakshina's text — respelled in Latin letters
(`scripts/build-hinglish.py`): each word in one of the spellings
Dakshina's romanization lexicons attest, as often as each was typed, any
other by rule. `eval/hi-Latn.txt` is the first 500 of Dakshina's romanized
Hindi test sentences, as people typed them, which the Wikipedia text it
learns from leaves out.

### Wikipedia text (the wikis' own)

Sixteen languages' models learn from their own Wikipedia alone:
Bhojpuri, Maithili, Mongolian, Nepali, Odia, Western Punjabi, Pashto,
Sanskrit, Santali, Yiddish, Konkani, Kashmiri, Meitei, Awadhi, Saraiki
and Tulu. The text is each wiki's CirrusSearch dump of 2026-11-08
(Wikimedia's `cirrus_search_index`, an article's plain text a JSON
field), pinned by sha256 in `scripts/fetch-wikipedia.sh`, cut into
sentences by `scripts/wikipedia-text.py`: at most 400 000 a language.
Every 20th article is held out of it, and the first 500 sentences of
those are `eval/<lang>.txt`. **Wikipedia's text is CC BY-SA 4.0**,
share-alike: the packs of those sixteen languages, their models and that
held-out text carry those terms, with attribution to Wikipedia's
contributors (each article's history, at its wiki, names them).

### FineWeb-2 (web text)

Contains information from [FineWeb-2](https://huggingface.co/datasets/HuggingFaceFW/fineweb-2)
which is made available under the [ODC Attribution License](https://opendatacommons.org/licenses/by/1-0/).

Seven models learn from FineWeb-2's web text alone: those of Bodo,
Dogri, Chhattisgarhi, Magahi and Brahui, which have no Wikipedia of
their own, and of Sindhi in Devanagari (`sd-Deva`) and Meitei in the
Bengali script (`mni-Beng`), whose wikis are in their other scripts.
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
Chhattisgarhi, Magahi and Brahui. They were written here, MIT licensed,
each joining its language's pack; what they choose among, the
language's lexicon chooses. They are measured on romanizations nothing
here ships: the Dakshina dataset's, and for ten of them the test words of AI4Bharat's Aksharantar dataset (manually
collected data CC BY, mined data CC0; "Aksharantar: Towards Building
Open Transliteration Tools for the Next Billion Users", arXiv
2205.03018), which `scripts/fetch-aksharantar.sh` downloads to measure.
Their layouts, `layouts/<lang>-abc.json`, are the US layout's keys
(`scripts/build-translit-layouts.py`), under the keyboard layouts'
notices below.

## Word lists

`wordbreak/th.txt`, the word list that cuts Thai text into words (Thai is
written without spaces between them), in the trainer, the keyboard and
`eval-predict` alike, is **ICU's Thai word-break dictionary**,
`icu4c/source/data/brkitr/dictionaries/thaidict.txt` of ICU 79.1,
committed as ICU has it, its copyright lines with it
(`scripts/fetch-word-lists.sh` pins it by sha256). It says: "Copyright (C)
2016 and later: Unicode, Inc. and others. License & terms of use:
http://www.unicode.org/copyright.html. Copyright (c) 2006-2015
International Business Machines Corporation, Apple Inc., and others. All
Rights Reserved." Thai's model (`predict/th.*`) is built from Tatoeba's
sentences cut with it. ICU's licence, for the file, from ICU's `LICENSE`:

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
presses (`scripts/build-derived-layouts.py`).
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
