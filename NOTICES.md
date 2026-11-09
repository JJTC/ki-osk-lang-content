# Notices — ki-osk language content

Attributions and licences for the datasets in this repository and the
files built from them. Paths are relative to this repository; inside
ki-osk the same files sit under `crates/osk-core/assets/lang/`, and an
installed system has them under `/usr/share/ki-osk/`.

**Several of these are share-alike.** Redistributing the derived `.bin`
files carries their terms.

The code here — the Makefile, the Cargo workspace and `crates/`, the
scripts, the workflows and `packs.toml` — is MIT licensed (`LICENSE`), as
are the text rules in `rules/`, which were written here.
The data keeps its original licence, below. That includes the seeds in
`seeds/`, which are excerpts of the datasets they are listed under. Add an entry here whenever a
new dataset is pulled in.

Exactly which upstream file each committed output was built from is
recorded, by sha256, in `sources.lock`.

Fonts, emoji data, the offensive-word lists, the Mandarin syllable
inventory and the Double-Pinyin scheme tables stayed in the ki-osk
repository with the code that embeds them; ki-osk's own `NOTICES.md`
covers those.

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
ki-osk's `toolbox eval-predict` measures prediction on. All 54 were built
from the exports of 2026-11-08.

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
| os | oss | — | — | — | — | — | — |

Two size tiers are in use, both recorded in the Makefile: 36 languages
(`PREDICT_COMPACT_LANGS`) keep 18 000 words and 30 000 n-grams, the
other 18 keep 20 000 and 60 000.

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
