# ki-osk language content

Dictionaries, prediction models and input-method tables for
[ki-osk](https://github.com/jjtc/ki-osk), an on-screen keyboard, as
language packs: one per language, 58 in all, and one of the text rules
of every language. Beside the data are the builders that produce it
(`crates/osk-data-tools`) and the format crate both they and ki-osk use
(`crates/osk-data`), so a reader and the writer of the same file cannot
disagree.

## Licences

`LICENSE` (MIT) covers the code and tooling only, and the text rules,
which were written here. The data keeps the licences of the datasets it
was built from, listed in `NOTICES.md`, and **several are share-alike**
(CC BY-SA): the Chinese and Cantonese dictionaries. Whoever
redistributes those files, or anything built from them, takes on those
terms.

## Layout

| Path | What | Built by |
|---|---|---|
| `predict/<lang>.fst` | Prefix-completion lexicon (54 languages) | `make predict-assets` |
| `predict/<lang>.ngram.bin` | Bigram/trigram scores | `make predict-assets` |
| `predict/<lang>.forms.fst` | Each word's usual capitals, where it is not written in lower case (`Haus`, `I`, `E-Mail`) | `make predict-assets` |
| `rules/all.weir`, `rules/<lang>.weir` | Text rules: what typing a key does to the text before the caret beyond typing it (the space after a strip word giving way to punctuation, French spacing), and where autocorrect acts besides space, Weir-style, with their tests | by hand |
| `predict/corpora/<lang>.txt` | Training text (Tatoeba) — **not committed** | `make fetch-corpora` |
| `eval/<lang>.txt` | Held-out Tatoeba text no model is trained on: what keystroke savings are measured on | `make fetch-corpora` |
| `pinyin/cedict.bin` | Pinyin → Simplified Hanzi | `make pinyin-assets` |
| `zhuyin/cedict.bin` | Bopomofo → Traditional | `make zhuyin-assets` |
| `jyutping/cedict.bin` | Jyutping → Traditional | `make jyutping-assets` |
| `kana/mozc-kanji.bin` | Kana → kanji | `make kana-assets` |
| `kana/mozc-connection.bin` | mozc POS-bigram matrix | `make kana-assets` |
| `sources.lock` | What each output was built from | every build target |
| `seeds/` | Tiny Pinyin, Jyutping and kana seeds (offline fallback) | by hand |
| `packs.toml` | Which files make up each pack, with licences and sources | by hand |
| `crates/osk-data/` | The format: the KIOS codec, the serialized types, packs; and the word splitter the models are trained with | — |
| `crates/osk-data-tools/` | The builders (`build-predict`, `build-pinyin`, …, `make-packs`) | — |

Every `.bin` carries a `KIOS` magic and version header and is written
through `osk_data::codec`, so a stale or foreign file is rejected on
sight rather than mis-decoded.

## Packs and releases

A pack is one language's files: an `.fst` lexicon and an `.ngram.bin`
table for each of the 54 prediction languages, and `zh-Hans` (Pinyin),
`zh-Hant` (Zhuyin), `yue` (Jyutping) and `ja` (kana–kanji); and `rules`,
the text rules of every language, which are written by hand, Weir-style
(after the rule language of the Harper grammar checker): each rule a
pattern over the end of the text before the caret, what it becomes, and
its tests, in the same file. `cargo test` runs them.

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
