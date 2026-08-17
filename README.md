# ki-osk language content

Dictionaries, prediction models and IME tables for
[ki-osk](../ki-osk), the on-screen keyboard for cog-in-cage kiosks.

This is a **data** repository. It contains no code. It was split out of
the ki-osk tree on 2026-08-17, with history preserved, for two reasons:

- **Churn.** These are generated binaries. `make predict-assets`
  rewrites ~43 MB of `.fst` and `.ngram.bin` wholesale, and binaries do
  not delta — every regeneration added its full weight to the code
  repository, permanently.
- **Licensing.** ki-osk is MIT. Most of what is here is not: CC-CEDICT
  and CC-Canto are CC BY-SA 4.0, whose share-alike reaches the `.bin`
  files built from them. Keeping the boundary structural rather than
  documentary makes it much harder to get wrong.

See `NOTICES.md` for the per-dataset licences and attributions. **The
share-alike terms are real** — if you redistribute the CC BY-SA-derived
files, or anything built from them, those terms apply to you.

## Layout

| Path | What | Built by |
|---|---|---|
| `predict/<lang>.fst` | Prefix-completion lexicon | `toolbox build-predict` |
| `predict/<lang>.ngram.bin` | Bigram/trigram scores | `toolbox build-predict` |
| `predict/corpora/<lang>.txt` | Training text (Tatoeba) | `scripts/fetch-corpora.sh` |
| `neural/<lang>/` | Optional char-LM bundles | `scripts/hf-to-kiosk.py` |
| `pinyin/cedict.bin` | Pinyin → Hanzi | `toolbox build-pinyin` |
| `zhuyin/cedict.bin` | Bopomofo → Traditional | `toolbox build-zhuyin` |
| `jyutping/cedict.bin` | Jyutping → Traditional | `toolbox build-jyutping` |
| `kana/mozc-kanji.bin` | Kana → kanji | `toolbox build-kana` |
| `kana/mozc-connection.bin` | mozc POS-bigram matrix | `toolbox build-connection` |

Every `.bin` carries a `KIOS` magic + version header and is written
through `osk_core::asset_codec`, so a stale or foreign file is rejected
by inspection rather than silently mis-decoded.

## What deliberately stayed behind

The small text **seeds** (`pinyin/seed.cedict.txt`,
`jyutping/seed.canto.txt`, `kana/seed.mozc.txt`) and the
offensive-word lists remain in the ki-osk repository: the toolbox
builders and `osk_core::offensive` embed them with unconditional
`include_str!`, so moving them would have made the code repository
unable to compile on its own. They are a few tens of kilobytes.

Keyboard **layouts** also stayed. They are language content by subject
but code by structure — schema-versioned JSON compiled in via
`include_str!` behind the `langs-*` features, and validated by tests.

## Use

ki-osk consumes this as a git submodule at
`crates/osk-core/assets/lang`. At runtime the daemon does not use this
checkout at all: it loads from `/usr/share/ki-osk` (staged by
`make dist-data`), or from an `embed-dicts` build that compiled these
files in.
