# Recipes for pulling and regenerating this repository's contents.
#
# The split of responsibility, which is deliberate and worth
# understanding before moving anything across it:
#
#   THIS repo owns the RECIPE   — which corpora, which languages, which
#                                 upstream URLs, which tuning caps.
#   ki-osk owns the ENCODER     — `crates/toolbox`, which writes the
#                                 `KIOS`-headered files through
#                                 `osk_core::asset_codec`.
#
# The encoder deliberately did NOT move here. It writes a format that
# `osk-core` decodes, and the two must agree: a decoder that disagrees
# with its encoder does not crash, it makes the loader return `None`, and
# the daemon silently degrades to seed dictionaries with quietly worse
# prediction. Keeping both sides in one repository is what lets
# `asset_codec::tests::every_committed_asset_decodes` be a real guard.
# Moving `toolbox` here would also make this repo depend on `osk-core`
# while ki-osk carries this repo as a submodule — a cycle.
#
# So these targets shell out to a ki-osk checkout, found automatically in
# either of the two places this repository is normally sitting:
#
#   Projects/wlosk/ki-osk                          <- ../ki-osk
#   Projects/wlosk/ki-osk-lang-content
#
#   ki-osk/crates/osk-core/assets/lang             <- ../../../..
#     (the submodule checkout, which is where ki-osk's README sends you)
#
# Both matter: a plain `../ki-osk` default is wrong from inside the
# submodule, which is exactly the path the documentation recommends.
# Detection requires a [workspace] Cargo.toml so it cannot latch onto
# some unrelated crate. Override with `make KI_OSK=/path/to/ki-osk`.

KI_OSK ?= $(shell for d in ../../../.. ../ki-osk .. ; do \
              if [ -f "$$d/Cargo.toml" ] && grep -q '^\[workspace\]' "$$d/Cargo.toml" 2>/dev/null; then \
                  echo "$$d"; break; \
              fi; \
          done)
CARGO  ?= cargo

# Run a toolbox binary out of the ki-osk checkout.
TOOLBOX = $(CARGO) run --manifest-path $(KI_OSK)/Cargo.toml -p toolbox

PREDICT_LANGS ?= en de fr es it nl pt sv pl no da fi cs tr vi el ru uk ar he fa ro hu sk hr et is ca eo bg sr mk be id fil az ku ug hi bn hy ka lt lv kab
PREDICT_CORPORA_DIR = predict/corpora
PREDICT_OUT_DIR = predict
NEURAL_OUT_DIR = neural

# Caps tuned for the Tatoeba-100k corpora (Swedish caps at 56.5 k since
# Tatoeba does not have more sv). Larger corpora lift these; the only
# constraint is repo size. Override at the make-CLI level.
PREDICT_VOCAB ?= 20000
PREDICT_NGRAM ?= 60000

.PHONY: help
help:
	@echo "targets:"
	@echo "  fetch-corpora    download Tatoeba corpora into predict/corpora/"
	@echo "  predict-assets   rebuild predict/*.fst + *.ngram.bin from corpora/"
	@echo "  cjk-assets       rebuild the CJK dicts from the upstream datasets"
	@echo "  cjk-assets-seed  rebuild from seeds (offline; DOWNGRADES, needs CONFIRM=yes)"
	@echo "  neural-assets    (re)train the per-language char-LMs"
	@echo "  validate-neural  validate a bundle: make validate-neural BUNDLE=<dir>"
	@echo ""
	@echo "  all of these drive ki-osk's toolbox; set KI_OSK=<path> if it"
	@echo "  is not at $(KI_OSK)"

# Fail with something readable rather than cargo's error about a
# manifest that is not there.
.PHONY: require-ki-osk
require-ki-osk:
	@test -n "$(KI_OSK)" && test -f $(KI_OSK)/Cargo.toml || { \
	  echo "no ki-osk checkout found (looked at ../../../.. and ../ki-osk)"; \
	  echo "these targets need it for the encoder (crates/toolbox)."; \
	  echo "pass one: make KI_OSK=/path/to/ki-osk <target>"; exit 1; }

.PHONY: fetch-corpora
fetch-corpora:
	./scripts/fetch-corpora.sh

# Rebuild every language's prefix lexicon and n-gram table. Drop a
# bigger corpus at predict/corpora/<lang>.txt and rerun to refresh one
# language without touching anything else.
.PHONY: predict-assets
predict-assets: require-ki-osk
	@for lang in $(PREDICT_LANGS); do \
		echo "build-predict $$lang"; \
		$(TOOLBOX) --release --quiet --bin build-predict -- \
			--lang $$lang \
			--corpus $(CURDIR)/$(PREDICT_CORPORA_DIR)/$$lang.txt \
			--out $(CURDIR)/$(PREDICT_OUT_DIR) \
			--vocab-size $(PREDICT_VOCAB) \
			--ngram-keep $(PREDICT_NGRAM) \
			|| exit 1; \
	done

# Regenerate the CJK dictionaries from the full upstream datasets — what
# the committed .bins are built from. See NOTICES.md for provenance and
# licences; several are share-alike. Downloads go to /tmp; only the
# .bins are committed. The kana filters keep the bundle on its size
# budget (mozc's full dump is ~750 k readings).
CEDICT_URL = https://www.mdbg.net/chinese/export/cedict/cedict_1_0_ts_utf-8_mdbg.txt.gz
MOZC_RAW   = https://raw.githubusercontent.com/google/mozc/master/src/data/dictionary_oss
# Word frequencies for ranking the Pinyin candidates. CC-CEDICT carries
# no frequency data at all, and without this the order is CC-CEDICT's own
# (≈ codepoint order), which puts archaic variants ahead of everyday
# characters — see ki-osk's crates/osk-core/tests/pinyin_ranking.rs,
# which fails against a bundle built without it.
ZHFREQ_URL = https://raw.githubusercontent.com/hermitdave/FrequencyWords/master/content/2018/zh_cn/zh_cn_50k.txt

.PHONY: cjk-assets
cjk-assets: require-ki-osk
	curl -fsSL --retry 5 --retry-all-errors $(CEDICT_URL) | gunzip > /tmp/cedict.txt
	curl -fsSL --retry 5 --retry-all-errors $(ZHFREQ_URL) > /tmp/zh_cn_50k.txt
	rm -f /tmp/mozc-dict.tsv; for i in 0 1 2 3 4 5 6 7 8 9; do \
		curl -fsSL --retry 5 --retry-all-errors $(MOZC_RAW)/dictionary0$$i.txt \
			>> /tmp/mozc-dict.tsv; done
	curl -fsSL --retry 5 --retry-all-errors $(MOZC_RAW)/connection_single_column.txt \
		> /tmp/mozc-connection.txt
	$(TOOLBOX) --release --quiet --bin build-pinyin -- \
		--cedict /tmp/cedict.txt --freq /tmp/zh_cn_50k.txt \
		--out $(CURDIR)/pinyin/
	$(TOOLBOX) --release --quiet --bin build-kana -- \
		--mozc /tmp/mozc-dict.tsv --max-cost 7000 --max-readings 80000 \
		--out $(CURDIR)/kana/
	$(TOOLBOX) --release --quiet --bin build-connection -- \
		--matrix /tmp/mozc-connection.txt --out $(CURDIR)/kana/

# Offline fallback: rebuild both dictionaries from the tiny curated
# seeds. Those seeds live in ki-osk, not here — `toolbox` embeds them
# with unconditional include_str!, which is precisely why they could not
# move with the rest of the data.
#
# DESTRUCTIVE: this OVERWRITES the committed full bundles with the seeds.
# mozc-kanji.bin goes from 4.4 MB / 80 000 readings to 4 KB / 191. That
# is the point of the target — it exists so a machine with no network can
# still produce a working dictionary — but running it by accident and
# committing the result would quietly gut CJK input. Check `git diff
# --stat` before committing after this.
.PHONY: cjk-assets-seed
cjk-assets-seed: require-ki-osk
	@test "$(CONFIRM)" = "yes" || { \
	  echo "cjk-assets-seed REPLACES the committed dictionaries with tiny seeds:"; \
	  echo ""; \
	  echo "    pinyin/cedict.bin     86 400 keys     ->  ~150"; \
	  echo "    kana/mozc-kanji.bin   80 000 readings ->   191"; \
	  echo ""; \
	  echo "It exists so a machine with no network can still produce a working"; \
	  echo "dictionary. If that is genuinely what you want:"; \
	  echo ""; \
	  echo "    make cjk-assets-seed CONFIRM=yes"; \
	  echo ""; \
	  echo "Otherwise you probably want 'make cjk-assets' (full, needs network)."; \
	  exit 1; }
	$(TOOLBOX) --quiet --bin build-pinyin -- --seed --out $(CURDIR)/pinyin/
	$(TOOLBOX) --quiet --bin build-kana  -- --seed --out $(CURDIR)/kana/
	@echo ""
	@echo "NOTE: the full bundles are now seeds. ki-osk's"
	@echo "      crates/osk-core/tests/bundled_dicts.rs will fail until you"
	@echo "      restore them (git checkout) or rebuild with 'make cjk-assets'."


# Train the per-language char-LMs. Released packages that want neural
# rescoring install these to /usr/share/ki-osk/models/<lang>/. The
# committed weights are trained on the Tatoeba corpora and are small;
# drop a bigger corpus on <lang>.txt for more useful ones.
NEURAL_EPOCHS     ?= 5
NEURAL_N_LAYER    ?= 2
NEURAL_N_HEAD     ?= 2
NEURAL_N_EMBD     ?= 64
NEURAL_BLOCK_SIZE ?= 64
NEURAL_BATCH      ?= 32
NEURAL_MAX_EX     ?= 20000

.PHONY: neural-assets
neural-assets: require-ki-osk
	@for lang in $(PREDICT_LANGS); do \
		echo "train-char-lm $$lang"; \
		$(TOOLBOX) --release --features neural --bin train-char-lm -- \
			--corpus $(CURDIR)/$(PREDICT_CORPORA_DIR)/$$lang.txt \
			--out  $(CURDIR)/$(NEURAL_OUT_DIR)/$$lang \
			--epochs $(NEURAL_EPOCHS) \
			--batch-size $(NEURAL_BATCH) \
			--max-examples $(NEURAL_MAX_EX) \
			--n-layer $(NEURAL_N_LAYER) --n-head $(NEURAL_N_HEAD) \
			--n-embd $(NEURAL_N_EMBD) --block-size $(NEURAL_BLOCK_SIZE) \
			|| exit 1; \
	done

# BYOW workflow: validate a neural bundle before installing it under
# /usr/share/ki-osk/models/. Pairs with scripts/hf-to-kiosk.py.
.PHONY: validate-neural
validate-neural: require-ki-osk
	@if [ -z "$(BUNDLE)" ]; then \
	    echo "usage: make validate-neural BUNDLE=/path/to/neural/bundle"; exit 1; \
	fi
	$(TOOLBOX) --release --features neural --bin validate-neural -- "$(BUNDLE)"
