# Notices — ki-osk language content

Attributions and licences for the datasets in this repository. Extracted
from the ki-osk tree's `NOTICES.md` on 2026-08-17; that file remains
authoritative for the fonts, emoji data and word lists that stayed with
the code.

**Several of these are share-alike.** Redistributing the derived `.bin`
files carries their terms.

The ki-osk source tree is MIT licensed. Bundled *data* keeps its original
license. Add an entry here whenever a new asset is pulled in.

## Fonts

- **DejaVu Sans** — Bitstream Vera / DejaVu license (permissive,
  MIT-compatible). Source: https://dejavu-fonts.github.io/
- **Noto Color Emoji** — SIL Open Font License 1.1. **Not** embedded in
  the binary (it is ~10 MB); loaded at runtime from a system font or from
  `/usr/share/ki-osk/fonts/NotoColorEmoji.ttf` (staged by `make
  dist-data`). Ship the OFL license text alongside the installed font.
  Source: https://github.com/googlefonts/noto-emoji

## Dictionaries

- **CC-CEDICT** (Mandarin Pinyin → Hanzi) — CC BY-SA 4.0.
  `crates/osk-core/assets/pinyin/cedict.bin` is built from the
  **full CC-CEDICT dataset** (MDBG export dated 2026-08-02,
  124 770 lines → 86 400 pinyin keys, ≤9 candidates per key) via
  `build-pinyin --cedict <path> --freq <path>`. Share-alike applies
  to the resulting `.bin`. A ~150-headword HSK-1/2 seed
  (`seed.cedict.txt`, `build-pinyin --seed`) remains as the
  no-download fallback. Source:
  https://www.mdbg.net/chinese/dictionary?page=cc-cedict
  The Zhuyin/Traditional-Chinese IME reuses the same CC-CEDICT source:
  `assets/zhuyin/cedict.bin` (`build-zhuyin`) keys the **Traditional**
  column by toneless Bopomofo. Same CC BY-SA 4.0 share-alike.
- **FrequencyWords** (Chinese word frequencies, used only to ORDER the
  CC-CEDICT candidates) — MIT, © 2016 Hermit Dave. The list is derived
  from OpenSubtitles, which CLAUDE.md §4.8 names as an acceptable
  source. `content/2018/zh_cn/zh_cn_50k.txt`, fetched by
  `make cjk-assets`. No part of the list is redistributed — only the
  resulting candidate ORDER inside `cedict.bin`, which carries
  CC-CEDICT's own share-alike. Source:
  https://github.com/hermitdave/FrequencyWords
- **Mandarin syllable inventory** — derived from CC-CEDICT (above).
  `pinyin::ALL_SYLLABLES` is the 413 toneless Pinyin syllables obtained
  by taking each CC-CEDICT entry's bracketed reading, dropping tone
  digits, and keeping what splits cleanly into a standard initial +
  final. The result is the phonological inventory of the language — a
  set of facts, not an excerpt — and carries no dictionary content:
  no headword, gloss or Hanzi from CC-CEDICT is reproduced by it.
  Listed here for transparency about where it came from.
- **Double-Pinyin scheme tables** (`ime::double_pinyin`) — the published
  key mappings of 小鹤双拼, 微软双拼, 自然码, 智能ABC, 紫光拼音 and
  拼音加加. These are layout definitions in the same category as an xkb
  symbols file: which key carries which initial or final, as specified
  by each scheme's own authors. Transcribed from the schemes' published
  definitions as tabulated by libpinyin (GPL) and then **independently
  verified** by an exhaustive encode/decode round trip over the syllable
  inventory above — no libpinyin code, structure or generated file is
  copied or linked, and ki-osk takes no GPL dependency.
- **LDNOOBW** ("List of Dirty, Naughty, Obscene and Otherwise Bad
  Words") — CC BY 4.0. `crates/osk-core/assets/offensive/<lang>.txt`
  holds the 26 bundled language lists, used ONLY to suppress words from
  the suggestion strip under `--filter-offensive` (off by default). They
  never restrict what can be typed. Source:
  https://github.com/LDNOOBW/List-of-Dirty-Naughty-Obscene-and-Otherwise-Bad-Words
- **AOSP LatinIME wordlist** (English `flags=offensive` entries only) —
  Apache 2.0, © 2008 The Android Open Source Project. 190 words merged
  into `assets/offensive/en.txt`. Taken from the FUTO Keyboard checkout,
  whose own code is under the non-free FUTO Source First License — but
  its `NOTICE` scopes that license to changes AFTER commit
  `d847619a2b48945465f840b8d81644fa455cc115`, and every commit touching
  `dictionaries/` is an ancestor of it, authored by AOSP engineers. Only
  the Apache-2.0 AOSP portion is used here; no FUTO-licensed material is
  included. Source: https://android.googlesource.com/platform/packages/inputmethods/LatinIME/
- **CC-Canto** (Cantonese Jyutping → Traditional Hanzi) — CC BY-SA 4.0.
  The committed `crates/osk-core/assets/jyutping/cedict.bin` is the
  ~150-key HSK-common seed (`seed.canto.txt`, `build-jyutping --seed`,
  the same headwords as the Pinyin seed carrying their Cantonese
  readings) so the OSK ships a usable Cantonese dict with no download.
  A full bundle (**CC-Canto Cantonese readings**, 105 862 entries →
  84 531 toneless-Jyutping keys, ~3.8 MB) drops in via
  `build-jyutping --canto <cccedict-canto-readings.txt>` and is loaded
  at runtime from `$KI_OSK_DATA_DIR/jyutping/cedict.bin` without a
  rebuild. Jyutping readings **cannot** be derived from the Mandarin
  Pinyin table (你好 is `nei5 hou2`, not `ni3 hao3`), so this is a
  distinct dataset. Share-alike applies to the resulting `.bin`.
  Source: https://cantonese.org/download.html
- **mozc kanji dict** (Japanese kana → kanji) — 3-clause BSD.
  `crates/osk-core/assets/kana/mozc-kanji.bin` is built from the
  **mozc OSS system dictionary**
  (`src/data/dictionary_oss/dictionary00–09.txt` @ master, fetched
  2026-07-19; 1 289 076 entries) via
  `build-kana --mozc <path> --max-cost 7000 --max-readings 80000`:
  entries above mozc cost 7000 (rare compounds, most proper nouns)
  are dropped, then the 80 000 most frequent readings are kept
  (≤9 candidates each, ~7.1 MB). Each candidate now carries its mozc
  **word cost and POS ids** (`lid`/`rid`) for the context-aware
  conversion lattice. The BSD license applies to the resulting `.bin`.
  A ~190-headword JLPT N5/N4 seed (`seed.mozc.txt`, `build-kana --seed`,
  POS-less) remains as the no-download fallback.
- **mozc connection matrix** (kana → kanji context model) — 3-clause BSD.
  `crates/osk-core/assets/kana/mozc-connection.bin` is built from
  mozc's `src/data/dictionary_oss/connection_single_column.txt` (the
  2672×2672 POS-bigram transition-cost matrix) via
  `build-connection --matrix <path>`: the 14 MB `i16` matrix is
  zlib-deflated to ~1.9 MB and inflated once at load. It drives the
  Viterbi (`word_cost + connection(prev.rid, cur.lid)`), turning
  にほんごをべんきょうする into 日本語を勉強する. Absent → the dict falls
  back to word-cost-only conversion. Source:
  https://github.com/google/mozc

## Prediction corpora

The committed `crates/osk-core/assets/predict/<lang>.fst` +
`<lang>.ngram.bin` (and `assets/neural/<lang>/model.safetensors`)
files are trained from per-language sentence subsets (first
~120 000 in id order, or all available where fewer) from the
**Tatoeba** per-language exports
(<https://downloads.tatoeba.org/exports/per_language/>), filtered
to the matching ISO-639-3 code:

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

**The corpora are not committed.** Only the built `.fst`/`.ngram.bin`
artifacts are; the source text is regenerated on demand by
`scripts/fetch-corpora.sh` (which holds the id→code map above). The
nine legacy corpora (`en de fr es it nl pt sv pl`) predate this and
remain committed under `corpora/`.

**Tatoeba sentences are CC BY 2.0 FR.** The subsets, derived n-gram
tables, and trained char-LM weights inherit those terms; attribution
to the Tatoeba project must travel with any redistribution.

To enlarge coverage, append from any of the following into
`corpora/<lang>.txt` and re-run `make predict-assets`
(plus `make neural-assets` for the char-LMs):

- **Wikipedia dumps** (per-language text extracts) — CC BY-SA 3.0 / CC BY-SA 4.0.
- **Project Gutenberg** — public domain in the US; per-country status varies.
- **OpenSubtitles** — per-language check required; some languages excluded.
- **Additional Tatoeba sentences** beyond the first 30 k per
  language — same CC BY 2.0 FR terms.

Adding share-alike sources (Wikipedia in particular) downgrades
the redistributable license of the resulting binaries to the
strongest copyleft in the mix.

## Emoji metadata

- **Unicode emoji data** (`emoji-test.txt`, `emoji-sequences.txt`) — Unicode
  license (https://www.unicode.org/license.txt). Permissive.
- **CLDR annotations** — Unicode/CLDR license. Used for emoji search keywords.
