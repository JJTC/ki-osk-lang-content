# ki-osk language content

Keyboard layouts, dictionaries, prediction models and input-method tables
for [ki-osk](https://github.com/jjtc/ki-osk), an on-screen keyboard, as
language packs: one per language, 149 in all, and three every language
shares: the text rules, the emoji picker and the offensive-word lists.
Everything ki-osk knows about a language is here; ki-osk itself is code.
Beside the data are the builders that produce it
(`crates/osk-data-tools`) and the format crate both they and ki-osk use
(`crates/osk-data`), so a reader and the writer of the same file cannot
disagree. `LANGUAGES.md` lists every layout, the pack it is in and what
it can do.

## Licences

`LICENSE` (MIT) covers the code and tooling only, and the text rules,
which were written here. The data keeps the licences of the datasets it
was built from, listed in `NOTICES.md`, and **several are share-alike**
(CC BY-SA): the Chinese and Cantonese dictionaries, and the Portuguese
and Spanish dialect flags. Whoever redistributes those files, or anything
built from them, takes on those terms. The layouts carry
xkeyboard-config's MIT-style notices, the emoji data Unicode's licence.

## Layout

| Path | What | Built by |
|---|---|---|
| `predict/<lang>.fst` | Prefix-completion lexicon (54 languages) | `make predict-assets` |
| `predict/<lang>.ngram.bin` | Bigram/trigram scores, with those of the words that start sentences (after `<s>`, `osk_data::words::SENTENCE_START`) kept apart | `make predict-assets` |
| `predict/<lang>.forms.fst` | Each word's usual capitals, where it is not written in lower case (`Haus`, `I`, `E-Mail`) | `make predict-assets` |
| `predict/<lang>.dialects.bin` | Dialect flags, where a source has them (English, Portuguese, Spanish): which varieties use a word, and its spellings across them (`color` and `colour`, `fato` and `facto`) | `make dialect-assets` |
| `rules/all.weir`, `rules/<lang>.weir` | Text rules: what typing a key does to the text before the caret beyond typing it (the space after a strip word giving way to punctuation, French spacing), and where autocorrect acts besides space, Weir-style, with their tests | by hand |
| `predict/corpora/<lang>.txt` | Training text (Tatoeba) — **not committed** | `make fetch-corpora` |
| `eval/<lang>.txt` | Held-out Tatoeba text no model is trained on: what keystroke savings are measured on | `make fetch-corpora` |
| `pinyin/cedict.bin` | Pinyin → Simplified Hanzi | `make pinyin-assets` |
| `zhuyin/cedict.bin` | Bopomofo → Traditional | `make zhuyin-assets` |
| `jyutping/cedict.bin` | Jyutping → Traditional | `make jyutping-assets` |
| `kana/mozc-kanji.bin` | Kana → kanji | `make kana-assets` |
| `kana/mozc-connection.bin` | mozc POS-bigram matrix | `make kana-assets` |
| `sources.lock` | What each output was built from | every build target |
| `pinyin/syllables.txt` | The 413 Mandarin syllables in toneless Pinyin | by hand |
| `pinyin/double-pinyin.json` | The six double-Pinyin schemes: which key carries which initial and finals | by hand |
| `layouts/<id>.json` | Keyboard layouts (154): the keys of each and what each types, the national ones read from xkeyboard-config's keymaps | `scripts/build-layout.py` and the relabel scripts |
| `emoji/catalogue.json` | Every fully-qualified emoji with its CLDR name and keywords: the emoji search | `scripts/build-emoji.py` |
| `offensive/<lang>.txt` | Words a keyboard can keep off its strip (LDNOOBW, AOSP) | by hand |
| `seeds/` | Tiny Pinyin, Jyutping and kana seeds (offline fallback) | by hand |
| `packs.toml` | Which files make up each pack, with licences and sources | by hand |
| `crates/osk-data/` | The format: the KIOS codec, the serialized types, packs; and the word splitter the models are trained with | — |
| `crates/osk-data-tools/` | The builders (`build-predict`, `build-pinyin`, …, `make-packs`) | — |

Every `.bin` carries a `KIOS` magic and version header and is written
through `osk_data::codec`, so a stale or foreign file is rejected on
sight rather than mis-decoded.

## Packs and releases

A pack is one language's files: its keyboard layouts, and an `.fst`
lexicon and an `.ngram.bin` table for each of the 54 prediction
languages; `zh-Hans` (Pinyin), `zh-Hant` (Zhuyin), `yue` (Jyutping), `ja`
(kana–kanji) and `ko` (Hangul) with their layouts; and a pack of its
layouts alone for each of the 90 other languages with a layout and no
model yet. Three packs every language shares: `rules`, the text rules,
which are written by hand, Weir-style (after the rule language of the
Harper grammar checker): each rule a pattern over the end of the text
before the caret, what it becomes, and its tests, in the same file
(`cargo test` runs them); `emoji`, the picker's layout and its search
catalogue; and `offensive`, the lists only a keyboard keeping words off
its strip needs. `packs.toml` says which file is in which pack, and
`make-packs` refuses a file in none.

A release is `vX.Y.Z`, the version in `Cargo.toml`, one number for the
format crate and its data: **patch** when the format and the crate's API
stay as they are (new data, fixes, documentation), **minor** when either
changes or the set of packs does. Each release carries:

| Asset | What |
|---|---|
| `packs.json` | The index: every pack's manifest — its files with size and sha256, its licences, its upstream sources — and how each file is served |
| `<path>.gz` | Each file, gzipped: `predict.en.fst.gz` is `predict/en.fst` |
| `NOTICES.md` | The licence notices, which go with any pack |
| `ki-osk-lang-X.Y.Z.tar.xz` | Every pack, installed into one tree |
| `ki-osk-lang-corpora-X.Y.Z.tar.xz` | The text the prediction models were trained on (data releases) |
| `SHA256SUMS` | |

Every pack at once:

```sh
tar -xJf ki-osk-lang-X.Y.Z.tar.xz -C /usr/share/ki-osk
```

Some packs: read `packs.json`, download their files, and check each one
twice, the download against the index before decompressing it and the
result against the pack's manifest after (`osk_data::pack` does both,
with the `install` feature). An install tree holds each file at its
path, each pack's manifest at `packs/<id>.json` and the notices at
`NOTICES.md`. Since `packs.json` holds every checksum, keeping it pins a
release byte for byte; ki-osk keeps one as `lang.lock`.

## Building

The builders need only this repository and its pinned toolchain.
`make help` lists everything.

```sh
make fetch-corpora     # Tatoeba -> predict/corpora/ and eval/
make predict-assets    # corpora -> predict/*.fst + *.ngram.bin
make cjk-assets        # upstream -> pinyin, zhuyin, jyutping, kana
make refresh           # all three, rebuilding only what changed, then check
make check             # refuse a tree that lost, broke or shrank a file
make release           # every asset of the release -> dist/release/
make bump-version      # PART=patch | minor | major
make dialect-assets    # ESDB + Harper, Wiktionary -> predict/{en,pt,es}.dialects.bin
make own-corpus        # TAG=<lang> CORPUS=<file> OUT=<dir>: your own corpus
```

Every 20th Tatoeba sentence by id, and any sentence with the same text,
is held out of training. The first 1 500 per language are `eval/<lang>.txt`,
where ki-osk's `toolbox eval-predict` measures keystroke savings through
its real suggestion engine. Since no model saw that text, the number is
honest, and it is what the monthly refresh is held to.

`build-predict` finds words with `osk_data::words`, the splitter ki-osk
reads the text before the caret with and `eval-predict` types held-out
text with, so the words the models know are the words the keyboard
reads: whole, with their apostrophes, hyphens and marks (`don't`,
`что-то`, `क्या`), and an elided article apart from its word (`l'`,
`homme`). Its generation is part of every model's recipe, so a change to
its rules rebuilds them all.

`sources.lock` records, for every output, the sha256 of each input, the
recipe and the output itself. With `ONLY_STALE=1`, as `make refresh`
runs, only outputs whose inputs, recipe or file changed are rebuilt, and
`make lock-verify` checks the tree against the lock. The `.fst` files and
the Jyutping and kana tables rebuild byte for byte from their inputs.
The committed `.ngram.bin` tables predate deterministic builds and will
change on their next rebuild. `pinyin` and `zhuyin` come from CC-CEDICT,
which changes daily. A release builds byte for byte from its commit.

`make cjk-assets-seed` builds the CJK tables from `seeds/` instead:
offline, and **destructive**, since it replaces the full dictionaries.
It asks for `CONFIRM=yes`.

The layouts and the emoji data are rebuilt rarely, by their scripts:

```sh
python3 scripts/build-layout.py layouts kk,ky   # national layouts, from this host's
                                                # xkeyboard-config (libxkbcommon)
python3 scripts/build-emoji.py emoji-test.txt en.xml enDerived.xml emoji/catalogue.json
python3 scripts/gen-emoji-layout.py emoji-test.txt layouts/emoji.json
```

`cargo test` checks every layout with the parser ki-osk reads them with
(`osk_data::layout`), and the Pinyin tables exhaustively: every syllable
encoded and decoded back in every double-Pinyin scheme.

## Your own corpus

A language's predictions can be fitted to your own text: a product's
names and jargon, a domain's words, or a bigger corpus than Tatoeba's.

1. **The text**: plain UTF-8, a sentence or a paragraph per line. The
   splitter reads a line's end as a sentence's end, so unwrap wrapped
   text first. Keep a tenth of it out, as `<dir>/<lang>.txt`, to
   measure on.
2. **The corpus**: to add your text to the general one rather than
   replace it, append it to Tatoeba's (`make fetch-corpora` fetches that
   into `predict/corpora/`), a few times over to weight it up:

   ```sh
   cat predict/corpora/en.txt mine.txt mine.txt mine.txt > corpus.txt
   ```

3. **Build**: `make own-corpus TAG=en CORPUS=corpus.txt OUT=my-data`
   writes `my-data/predict/en.fst`, `en.ngram.bin` and `en.forms.fst`,
   built as the release builds that language (its caps, and the sentence
   start's), with the text rules in `my-data/rules/`. `TAG` names the
   language; nothing in this tree changes.
4. **Measure** with ki-osk's `toolbox eval-predict`, on your held-out
   text and on `eval/`, so as to see that general typing did not get
   worse, against this tree's data (`--data` this repository) for the
   release's:

   ```sh
   cargo run --release -p toolbox --features eval-layouts --bin eval-predict -- \
     measure --data my-data --eval <dir> --langs en
   ```

   With a novel (*Alice's Adventures in Wonderland*, a tenth of its
   sentences held out) appended three times to Tatoeba's English,
   keystroke savings on its held-out sentences rose from 42.4% to 49.7%,
   and on Tatoeba's held-out English went from 52.0% to 51.7%.
5. **Install** the three files under `/usr/share/ki-osk/predict/` (or
   `$KI_OSK_DATA_DIR/predict/`): the keyboard reads them at its next
   layout switch.

Models built from your text carry its licence, and Tatoeba's (CC BY 2.0
FR, `NOTICES.md`) when it went into them.

## Workflows

- **`ci.yml`**, on every push and pull request: fmt, clippy and the tests
  (every committed file decodes as its reader expects), the lock, a
  release built and read back, the version, and the workflows. The version
  check (`make check-version`) refuses a change to what a released version
  contains, and a version below the latest release.
- **`refresh.yml`** re-fetches every source, rebuilds what changed, runs
  the checks and ki-osk's tests over the result, and opens a pull request
  that bumps the patch version, with a draft release holding the corpora.
  It refuses a result in which any language's keystroke savings on
  `eval/` fell by more than a point.
  It is monthly, but its schedule is off until ki-osk is on GitHub, since
  it checks ki-osk out.
- **`release.yml`** publishes the version on the default branch when it
  has no release yet: a merged refresh's draft, or a new release.
