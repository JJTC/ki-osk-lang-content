# Recipes for pulling and regenerating this repository's contents.
#
# Everything needed lives here: the recipe (which corpora, which
# languages, which upstream URLs, which caps), the builders
# (crates/osk-data-tools) and the format they write through
# (crates/osk-data), which ki-osk's osk-core reads with. An encoder and a
# decoder that share one crate cannot disagree about the format, and
# building the data needs no ki-osk checkout.
#
# The builders run on this repository's pinned toolchain
# (rust-toolchain.toml), which cargo picks up because the recipes run
# here.

CARGO  ?= cargo
PYTHON ?= python3

# Run one of the builders.
TOOLS = $(CARGO) run --release --quiet -p osk-data-tools --bin

# Which revision of this repository's builders wrote a file. Recorded in
# sources.lock for provenance only; it does not make anything stale.
ENCODER = $(shell git describe --always --dirty --abbrev=12 2>/dev/null)

# Every language with a committed prediction model. Each needs an entry
# in scripts/fetch-corpora.sh's MAP too, but for the last nine, which learn
# from Wikipedia text alone (DAKSHINA_ONLY_LANGS, below). Nine (kab to
# th) were missing until 2026-11-08, so `make predict-assets` silently
# skipped models that were already committed.
PREDICT_LANGS ?= en de fr es it nl pt sv pl no da fi cs tr vi el ru uk ar he fa ro hu sk hr et is ca eo bg sr mk be id fil az ku ug hi bn hy ka lt lv kab \
                 oc gl br eu tt mr as ms os th gu kn ml pa sd si ta te ur

PREDICT_CORPORA_DIR = predict/corpora
PREDICT_OUT_DIR = predict

# Wikipedia text of twelve South Asian languages, from the Dakshina
# dataset (scripts/fetch-dakshina.sh, the archive pinned by its sha256;
# CC BY-SA 4.0). It is the whole corpus of the nine Tatoeba has too few
# sentences of (DAKSHINA_ONLY_LANGS, the last nine of PREDICT_LANGS), and
# joins Tatoeba's for Hindi, Bengali and Marathi, whose lexicons it widens:
# their transliteration tables (translit/) then find far more of the
# words typed. Tatoeba's sentences count TATOEBA_WEIGHT times, so that
# everyday text still leads the strip: keystroke savings on Tatoeba's
# held-out text moved by -0.5 (hi), -0.1 (bn) and -0.3 (mr) points, words
# the lexicon lacks fell from 4.0% to 1.6% (hi).
DAKSHINA_LANGS ?= bn gu hi kn ml mr pa sd si ta te ur
DAKSHINA_ONLY_LANGS ?= gu kn ml pa sd si ta te ur
DAKSHINA_DIR = $(SRC_DIR)/dakshina
TATOEBA_WEIGHT ?= 10

# Where the CJK upstream downloads go (scripts/fetch-cjk-sources.sh).
# Only the .bins built from them are committed.
SRC_DIR ?= $(or $(TMPDIR),/tmp)/ki-osk-sources

# sources.lock records, for every committed output, the sha256 of each
# input and the recipe values it was built with (scripts/lock.py). With
# ONLY_STALE=1 the build targets skip every output whose inputs, recipe
# and file still match the lock — what `make refresh` and the scheduled
# workflow run. Without it they rebuild everything. Either way, each
# successful build updates its lock entry.
LOCK ?= sources.lock
ONLY_STALE ?=
LOCKTOOL = $(PYTHON) scripts/lock.py --lock $(LOCK)
# Shell prelude for the locked targets: `stale NAME ARGS...` is true when
# NAME must be built; `record NAME ARGS...` writes its lock entry.
LOCKED = stale() { [ -z "$(ONLY_STALE)" ] || ! $(LOCKTOOL) fresh "$$@"; }; \
         record() { $(LOCKTOOL) record "$$@" --encoder "$(ENCODER)"; }

# Caps tuned for the Tatoeba corpora (≤120 000 sentences per language;
# Swedish has ~56.5 k). Larger corpora lift these; the only constraint is
# repo size. Override at the make-CLI level.
PREDICT_VOCAB ?= 20000
PREDICT_NGRAM ?= 60000
# The COMPACT tier: the 35 languages added in f73b244 ("Tier 4") plus
# kab were built with these smaller caps, by hand — this Makefile never
# said so. Established on 2026-11-08 by rebuilding: with these caps every
# one of their .fst files is byte-identical to the committed one (and
# each .ngram.bin within 0.35% of its size; those are not byte-stable,
# see scripts/lock.py), with the caps above none of the large ones are.
# Recording the tier keeps a refresh from silently growing 36 n-gram
# tables by up to 2x. Moving a language to the full tier is a size
# decision: take it out of this list. Hindi left it for the WIKI tier
# (below) when Wikipedia text joined its corpus.
PREDICT_COMPACT_LANGS ?= no da fi cs tr vi el ru uk ar he fa ro hu sk hr et is ca eo bg sr mk be \
                         id fil az ku ug hy ka lt lv kab
PREDICT_COMPACT_VOCAB ?= 18000
PREDICT_COMPACT_NGRAM ?= 30000
# The LARGE tier: languages whose words, kept whole (osk_data::words), are
# too many for the full tier's caps. Marathi has 25 k word types in its
# corpus once viramas no longer cut them; it and the Dravidian languages,
# whose words take their endings with them, learn their lexicons from
# Wikipedia text (DAKSHINA_LANGS), where 40 000 words still leave a quarter
# of Tamil's held-out words unknown.
PREDICT_LARGE_LANGS ?= mr kn ml ta te
PREDICT_LARGE_VOCAB ?= 40000
PREDICT_LARGE_NGRAM ?= 90000
# The WIKI tier: the other languages whose lexicons come from Wikipedia
# text too (DAKSHINA_LANGS).
PREDICT_WIKI_LANGS ?= hi bn gu pa sd si ur
PREDICT_WIKI_VOCAB ?= 30000
PREDICT_WIKI_NGRAM ?= 60000
# The bigrams and trigrams that start a sentence, kept apart from the
# tiers' n-gram caps (each kept to this many). Measured on en fr de es ru
# ar hi tr: 5 000 gains 1.30 points of keystroke savings over none, for
# n-gram tables 8-19% larger; 20 000 gains 0.07 more, for up to 67%.
PREDICT_START ?= 5000
# The generation of the word splitter the models are trained with
# (`osk_data::words::GENERATION`). A model built by another generation
# reads its words differently, so it is part of every predict output's
# recipe, and a new generation makes every one of them stale.
WORDS_GENERATION = $(shell sed -n 's/^pub const GENERATION: u32 = \([0-9]*\);/\1/p' crates/osk-data/src/words.rs)

.PHONY: help
help:
	@echo "targets:"
	@echo "  refresh          fetch everything, rebuild only what changed, check"
	@echo "                   (what the scheduled GitHub workflow runs)"
	@echo "  fetch-corpora    download Tatoeba corpora into predict/corpora/"
	@echo "  fetch-dakshina   Wikipedia text and romanizations of 12 South Asian"
	@echo "                   languages (the Dakshina dataset) into SRC_DIR"
	@echo "  word-lists       check (or fetch) ICU's word lists in wordbreak/"
	@echo "  cjk-sources      download the CJK upstream datasets into SRC_DIR"
	@echo "  predict-assets   rebuild predict/*.fst + *.ngram.bin from corpora/"
	@echo "  dialect-assets   rebuild predict/<lang>.dialects.bin (dialect flags)"
	@echo "                   from ESDB and Harper (English), Wiktionary (pt, es)"
	@echo "  own-corpus       TAG=<lang> CORPUS=<file> OUT=<dir>: one language's"
	@echo "                   prediction files from a corpus of your own (README)"
	@echo "  cjk-assets       rebuild all five CJK dicts from the upstream datasets"
	@echo "                   (pinyin-assets zhuyin-assets jyutping-assets kana-assets)"
	@echo "  cjk-assets-seed  rebuild from seeds (offline; DOWNGRADES, needs CONFIRM=yes)"
	@echo "  eval-translit    how often each transliteration table puts the word"
	@echo "                   meant first (after fetch-dakshina)"
	@echo "  translit-layouts write layouts/<lang>-abc.json for translit/<lang>.txt"
	@echo "  hinglish-assets  rebuild predict/hi-Latn.* (Hindi typed in Latin letters)"
	@echo "                   from Hindi's corpora respelled (after fetch-dakshina)"
	@echo "  check            compare the tree against HEAD: missing, shrunk or"
	@echo "                   broken outputs fail (scripts/check-assets.py)"
	@echo "  lock-verify      check every output still matches sources.lock"
	@echo "  packs            build the language packs of a release in dist/packs/"
	@echo "  release          every asset of the release, in dist/release/"
	@echo "  version          print the release version (Cargo.toml)"
	@echo "  bump-version     PART=patch (data) | minor (format, API, packs) | major"
	@echo "  check-version    the version is not released, or the tree is its content"
	@echo ""
	@echo "  ONLY_STALE=1     build only outputs whose inputs/recipe changed"

# Everything, incrementally: what the monthly workflow does before it
# runs ki-osk's tests and opens a PR. Safe to run locally — it builds
# only what the lock says is out of date, then refuses a result that
# lost a file or shrank one.
.PHONY: refresh
refresh: fetch-corpora fetch-dakshina
	$(MAKE) predict-assets hinglish-assets cjk-assets dialect-assets ONLY_STALE=1
	$(MAKE) check

.PHONY: fetch-corpora
fetch-corpora:
	./scripts/fetch-corpora.sh

.PHONY: fetch-dakshina
fetch-dakshina:
	OUT=$(DAKSHINA_DIR) ./scripts/fetch-dakshina.sh

# The word lists of the languages written without spaces between words
# (wordbreak/<lang>.txt, committed): ICU's word-break dictionaries, pinned
# by release and sha256 in the script. build-predict cuts their text into
# words with them, and so do the keyboard and eval-predict.
.PHONY: word-lists
word-lists:
	./scripts/fetch-word-lists.sh

# Rebuild every language's prefix lexicon and n-gram table. Drop a
# bigger corpus at predict/corpora/<lang>.txt and rerun to refresh one
# language without touching anything else. A language with a word list
# (wordbreak/<lang>.txt) is cut with it, and the list is an input.
.PHONY: predict-assets
predict-assets:
	@$(LOCKED); for lang in $(PREDICT_LANGS); do \
		case " $(PREDICT_COMPACT_LANGS) " in \
			*" $$lang "*) vocab=$(PREDICT_COMPACT_VOCAB); ngram=$(PREDICT_COMPACT_NGRAM) ;; \
			*)            vocab=$(PREDICT_VOCAB);         ngram=$(PREDICT_NGRAM) ;; \
		esac; \
		case " $(PREDICT_LARGE_LANGS) " in \
			*" $$lang "*) vocab=$(PREDICT_LARGE_VOCAB); ngram=$(PREDICT_LARGE_NGRAM) ;; \
		esac; \
		case " $(PREDICT_WIKI_LANGS) " in \
			*" $$lang "*) vocab=$(PREDICT_WIKI_VOCAB); ngram=$(PREDICT_WIKI_NGRAM) ;; \
		esac; \
		corpus=; corpus_in=; weight=; \
		case " $(DAKSHINA_ONLY_LANGS) " in \
			*" $$lang "*) ;; \
			*) case " $(DAKSHINA_LANGS) " in \
				*" $$lang "*) corpus="--corpus $(TATOEBA_WEIGHT):$(CURDIR)/$(PREDICT_CORPORA_DIR)/$$lang.txt"; \
				              weight="--recipe tatoeba_weight=$(TATOEBA_WEIGHT)" ;; \
				*) corpus="--corpus $(CURDIR)/$(PREDICT_CORPORA_DIR)/$$lang.txt" ;; \
			   esac; \
			   corpus_in="--input corpora/$$lang.txt=$(PREDICT_CORPORA_DIR)/$$lang.txt" ;; \
		esac; \
		case " $(DAKSHINA_LANGS) " in \
			*" $$lang "*) corpus="$$corpus --corpus $(DAKSHINA_DIR)/$$lang.txt"; \
			              corpus_in="$$corpus_in --input dakshina/$$lang.txt=$(DAKSHINA_DIR)/$$lang.txt" ;; \
		esac; \
		words=; words_in=; \
		if [ -f wordbreak/$$lang.txt ]; then \
			words="--word-list $(CURDIR)/wordbreak/$$lang.txt"; \
			words_in="--input wordbreak/$$lang.txt=wordbreak/$$lang.txt"; \
		fi; \
		test -n "$(WORDS_GENERATION)" || { echo "no GENERATION in crates/osk-data/src/words.rs"; exit 1; }; \
		set -- predict/$$lang \
			$$corpus_in $$words_in $$weight \
			--recipe vocab_size=$$vocab --recipe ngram_keep=$$ngram \
			--recipe start_keep=$(PREDICT_START) \
			--recipe words=$(WORDS_GENERATION) \
			--output $(PREDICT_OUT_DIR)/$$lang.fst --output $(PREDICT_OUT_DIR)/$$lang.ngram.bin \
			--output $(PREDICT_OUT_DIR)/$$lang.forms.fst; \
		stale "$$@" || continue; \
		echo "build-predict $$lang (vocab $$vocab, n-grams $$ngram)"; \
		$(TOOLS) build-predict -- \
			--lang $$lang \
			$$corpus \
			--out $(CURDIR)/$(PREDICT_OUT_DIR) \
			--vocab-size $$vocab \
			--ngram-keep $$ngram \
			--start-keep $(PREDICT_START) \
			$$words \
			|| exit 1; \
		record "$$@" || exit 1; \
	done

# The dialect flags: which varieties of a language use a word, and its
# spellings across them (osk_data::dialects), for the languages with a
# source. English's come from the English Speller Database (SCOWLv2) and
# Harper's dictionary (its Indian English words); Portuguese and Spanish
# from Wiktionary, as kaikki.org extracts it, read with the language's
# usual forms so that a name (`Tony`) is not taken for a regional word. A
# table keeps what the language's lexicon can use, so the lexicon is one
# of its inputs. French is left out: Wiktionary marks no French spelling
# as regional, and too few words for a table worth its share-alike terms.
DIALECT_LANGS ?= en pt es

.PHONY: dialect-sources dialect-assets
dialect-sources:
	OUT=$(SRC_DIR) ./scripts/fetch-dialect-sources.sh

dialect-assets: dialect-sources
	@$(LOCKED); for lang in $(DIALECT_LANGS); do \
		test -n "$(WORDS_GENERATION)" || { echo "no GENERATION in crates/osk-data/src/words.rs"; exit 1; }; \
		case $$lang in \
			en) inputs="--input esdb-scowl-pre.txt=$(SRC_DIR)/esdb-scowl-pre.txt \
			            --input harper-dictionary.dict=$(SRC_DIR)/harper-dictionary.dict"; \
			    flags="--esdb $(SRC_DIR)/esdb-scowl-pre.txt --harper $(SRC_DIR)/harper-dictionary.dict" ;; \
			pt|es) inputs="--input kaikki-$$lang.jsonl.gz=$(SRC_DIR)/kaikki-$$lang.jsonl.gz \
			               --input forms=$(PREDICT_OUT_DIR)/$$lang.forms.fst"; \
			    flags="--kaikki $(SRC_DIR)/kaikki-$$lang.jsonl.gz \
			           --forms $(CURDIR)/$(PREDICT_OUT_DIR)/$$lang.forms.fst" ;; \
			*)  echo "no dialect source for $$lang"; exit 1 ;; \
		esac; \
		set -- dialects/$$lang $$inputs --input lexicon=$(PREDICT_OUT_DIR)/$$lang.fst \
			--recipe words=$(WORDS_GENERATION) \
			--output $(PREDICT_OUT_DIR)/$$lang.dialects.bin; \
		stale "$$@" || continue; \
		echo "build-dialects $$lang"; \
		$(TOOLS) build-dialects -- --lang $$lang $$flags \
			--lexicon $(CURDIR)/$(PREDICT_OUT_DIR)/$$lang.fst \
			--out $(CURDIR)/$(PREDICT_OUT_DIR)/$$lang.dialects.bin \
			|| exit 1; \
		record "$$@" || exit 1; \
	done

# Bring your own corpus: one language's prediction files from a corpus of
# your own, built as the release builds that language (its tier's caps,
# the sentence start's), into $(OUT)/predict/, with the text rules beside
# them in $(OUT)/rules/: a data tree ki-osk's eval-predict measures, and
# whose predict/ the keyboard reads. Not recorded in sources.lock, and
# nothing in this tree changes. TAG names the language, as LANG would but
# for being the locale's. See README, "Your own corpus".
.PHONY: own-corpus
own-corpus:
	@test -n "$(TAG)" && test -n "$(CORPUS)" && test -n "$(OUT)" || \
		{ echo "usage: make own-corpus TAG=<lang> CORPUS=<text file> OUT=<dir>"; exit 2; }
	@test -f "$(CORPUS)" || { echo "own-corpus: no file $(CORPUS)"; exit 2; }
	@lang=$(TAG); \
	case " $(PREDICT_COMPACT_LANGS) " in \
		*" $$lang "*) vocab=$(PREDICT_COMPACT_VOCAB); ngram=$(PREDICT_COMPACT_NGRAM) ;; \
		*)            vocab=$(PREDICT_VOCAB);         ngram=$(PREDICT_NGRAM) ;; \
	esac; \
	case " $(PREDICT_LARGE_LANGS) " in \
		*" $$lang "*) vocab=$(PREDICT_LARGE_VOCAB); ngram=$(PREDICT_LARGE_NGRAM) ;; \
	esac; \
	case " $(PREDICT_WIKI_LANGS) " in \
		*" $$lang "*) vocab=$(PREDICT_WIKI_VOCAB); ngram=$(PREDICT_WIKI_NGRAM) ;; \
	esac; \
	words=; \
	if [ -f wordbreak/$$lang.txt ]; then \
		words="--word-list $(CURDIR)/wordbreak/$$lang.txt"; \
		mkdir -p "$(OUT)/wordbreak" && cp wordbreak/$$lang.txt "$(OUT)/wordbreak/" || exit 1; \
	fi; \
	mkdir -p "$(OUT)/predict" "$(OUT)/rules" && \
	echo "build-predict $$lang from $(CORPUS) (vocab $$vocab, n-grams $$ngram) -> $(OUT)/predict" && \
	$(TOOLS) build-predict -- --lang $$lang --corpus "$(abspath $(CORPUS))" \
		--out "$(abspath $(OUT))/predict" --vocab-size $$vocab --ngram-keep $$ngram \
		--start-keep $(PREDICT_START) $$words && \
	cp rules/*.weir "$(OUT)/rules/"

# Regenerate the CJK dictionaries from the full upstream datasets — what
# the committed .bins are built from. See NOTICES.md for provenance and
# licences; several are share-alike. The kana filters keep the bundle on
# its size budget (mozc's full dump is ~750 k readings).
.PHONY: cjk-sources
cjk-sources:
	OUT=$(SRC_DIR) ./scripts/fetch-cjk-sources.sh

# Lock arguments per dictionary: inputs by stable name, recipe, outputs.
ZHFREQ_IN  = --input zh_cn_50k.txt=$(SRC_DIR)/zh_cn_50k.txt
CEDICT_IN  = --input cedict.txt=$(SRC_DIR)/cedict.txt
PINYIN_LOCK   = pinyin $(CEDICT_IN) $(ZHFREQ_IN) --output pinyin/cedict.bin
ZHUYIN_LOCK   = zhuyin $(CEDICT_IN) $(ZHFREQ_IN) --output zhuyin/cedict.bin
JYUTPING_LOCK = jyutping --input cccedict-canto-readings.txt=$(SRC_DIR)/cccedict-canto-readings.txt \
                $(ZHFREQ_IN) --output jyutping/cedict.bin
KANA_MAX_COST     ?= 7000
KANA_MAX_READINGS ?= 80000
KANJI_LOCK   = kana/mozc-kanji --input mozc-dict.tsv=$(SRC_DIR)/mozc-dict.tsv \
               --recipe max_cost=$(KANA_MAX_COST) --recipe max_readings=$(KANA_MAX_READINGS) \
               --output kana/mozc-kanji.bin
CONNECT_LOCK = kana/mozc-connection --input mozc-connection.txt=$(SRC_DIR)/mozc-connection.txt \
               --output kana/mozc-connection.bin

.PHONY: cjk-assets pinyin-assets zhuyin-assets jyutping-assets kana-assets
cjk-assets: pinyin-assets zhuyin-assets jyutping-assets kana-assets

pinyin-assets: cjk-sources
	@$(LOCKED); set -- $(PINYIN_LOCK); stale "$$@" || exit 0; \
	$(TOOLS) build-pinyin -- \
		--cedict $(SRC_DIR)/cedict.txt --freq $(SRC_DIR)/zh_cn_50k.txt \
		--out $(CURDIR)/pinyin/ \
	&& record "$$@"

# The Traditional twin of pinyin: same CC-CEDICT source, Traditional
# column, keyed by toneless Bopomofo.
zhuyin-assets: cjk-sources
	@$(LOCKED); set -- $(ZHUYIN_LOCK); stale "$$@" || exit 0; \
	$(TOOLS) build-zhuyin -- \
		--cedict $(SRC_DIR)/cedict.txt --freq $(SRC_DIR)/zh_cn_50k.txt \
		--out $(CURDIR)/zhuyin/ \
	&& record "$$@"

# Same story as zhuyin. Verified: this recipe rebuilds the committed
# jyutping/cedict.bin from f664e39 byte-for-byte.
jyutping-assets: cjk-sources
	@$(LOCKED); set -- $(JYUTPING_LOCK); stale "$$@" || exit 0; \
	$(TOOLS) build-jyutping -- \
		--canto $(SRC_DIR)/cccedict-canto-readings.txt --freq $(SRC_DIR)/zh_cn_50k.txt \
		--out $(CURDIR)/jyutping/ \
	&& record "$$@"

kana-assets: cjk-sources
	@$(LOCKED); set -- $(KANJI_LOCK); stale "$$@" || exit 0; \
	$(TOOLS) build-kana -- \
		--mozc $(SRC_DIR)/mozc-dict.tsv \
		--max-cost $(KANA_MAX_COST) --max-readings $(KANA_MAX_READINGS) \
		--out $(CURDIR)/kana/ \
	&& record "$$@"
	@$(LOCKED); set -- $(CONNECT_LOCK); stale "$$@" || exit 0; \
	$(TOOLS) build-connection -- \
		--matrix $(SRC_DIR)/mozc-connection.txt --out $(CURDIR)/kana/ \
	&& record "$$@"

# Offline fallback: rebuild both dictionaries from the tiny curated
# seeds in seeds/, which the builders embed.
#
# DESTRUCTIVE: this OVERWRITES the committed full bundles with the seeds.
# mozc-kanji.bin goes from 4.4 MB / 80 000 readings to 4 KB / 191. That
# is the point of the target — it exists so a machine with no network can
# still produce a working dictionary — but running it by accident and
# committing the result would quietly gut CJK input. Check `git diff
# --stat` before committing after this.
#
# It does not touch the lock, so `make lock-verify` flags both files and
# the next `make refresh` rebuilds them from the real sources.
.PHONY: cjk-assets-seed
cjk-assets-seed:
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
	$(TOOLS) build-pinyin -- --seed --out $(CURDIR)/pinyin/
	$(TOOLS) build-kana -- --seed --out $(CURDIR)/kana/
	@echo ""
	@echo "NOTE: the full bundles are now seeds. ki-osk's"
	@echo "      crates/osk-core/tests/bundled_dicts.rs will fail until you"
	@echo "      restore them (git checkout) or rebuild with 'make cjk-assets'."


# Gate a rebuilt tree before it is committed: every output still there,
# with a valid header, none shrunk by more than CHECK_MAX_SHRINK.
# CHECK_BASE is the git revision to compare against.
CHECK_BASE         ?= HEAD
CHECK_MAX_SHRINK   ?= 0.10
CHECK_REPORT       ?=
# Hinglish: Hindi as it is typed in Latin letters, the hi-Latn model.
# scripts/build-hinglish.py respells Hindi's corpora word by word in the
# spellings Dakshina's romanization lexicons attest, as often as each was
# typed (the rest by rule), leaving out the romanized test sentences whose
# Latin side is eval/hi-Latn.txt; build-predict then counts them as Hindi's
# are counted (Tatoeba's ten times). Its inputs are the corpora, the
# lexicons and the script. After fetch-corpora and fetch-dakshina.
HINGLISH_DIR = $(DAKSHINA_DIR)/hinglish
.PHONY: hinglish-assets
hinglish-assets:
	@$(LOCKED); d=$(DAKSHINA_DIR)/hi; \
	test -d $$d || { echo "no $$d: run make fetch-dakshina"; exit 1; }; \
	set -- predict/hi-Latn \
		--input corpora/hi.txt=$(PREDICT_CORPORA_DIR)/hi.txt \
		--input dakshina/hi.txt=$(DAKSHINA_DIR)/hi.txt \
		--input dakshina/hi.translit.sampled.train.tsv=$$d/lexicons/hi.translit.sampled.train.tsv \
		--input dakshina/hi.translit.sampled.dev.tsv=$$d/lexicons/hi.translit.sampled.dev.tsv \
		--input dakshina/hi.translit.sampled.test.tsv=$$d/lexicons/hi.translit.sampled.test.tsv \
		--input dakshina/hi.romanized.rejoined.test.native.txt=$$d/romanized/hi.romanized.rejoined.test.native.txt \
		--input scripts/build-hinglish.py=scripts/build-hinglish.py \
		--recipe tatoeba_weight=$(TATOEBA_WEIGHT) \
		--recipe vocab_size=$(PREDICT_WIKI_VOCAB) --recipe ngram_keep=$(PREDICT_WIKI_NGRAM) \
		--recipe start_keep=$(PREDICT_START) --recipe words=$(WORDS_GENERATION) \
		--output $(PREDICT_OUT_DIR)/hi-Latn.fst --output $(PREDICT_OUT_DIR)/hi-Latn.ngram.bin \
		--output $(PREDICT_OUT_DIR)/hi-Latn.forms.fst; \
	stale "$$@" || exit 0; \
	mkdir -p $(HINGLISH_DIR) && \
	$(PYTHON) scripts/build-hinglish.py --lexicons $$d/lexicons \
		$(PREDICT_CORPORA_DIR)/hi.txt $(HINGLISH_DIR)/tatoeba.txt && \
	$(PYTHON) scripts/build-hinglish.py --lexicons $$d/lexicons \
		--drop $$d/romanized/hi.romanized.rejoined.test.native.txt \
		$(DAKSHINA_DIR)/hi.txt $(HINGLISH_DIR)/wikipedia.txt && \
	echo "build-predict hi-Latn (vocab $(PREDICT_WIKI_VOCAB), n-grams $(PREDICT_WIKI_NGRAM))" && \
	$(TOOLS) build-predict -- --lang hi-Latn \
		--corpus $(TATOEBA_WEIGHT):$(HINGLISH_DIR)/tatoeba.txt --corpus $(HINGLISH_DIR)/wikipedia.txt \
		--out $(CURDIR)/$(PREDICT_OUT_DIR) --vocab-size $(PREDICT_WIKI_VOCAB) \
		--ngram-keep $(PREDICT_WIKI_NGRAM) --start-keep $(PREDICT_START) && \
	record "$$@"

# How often each transliteration table (translit/<lang>.txt), with its
# language's lexicon, puts the word meant first, as the composer's strip
# offers them: on Dakshina's romanized Wikipedia sentences (every word
# once) and on its romanization lexicon's test words (each romanization as
# often as it was typed). After `make fetch-dakshina`.
TRANSLIT_EVAL_LANGS ?= bn gu hi kn ml mr pa ta te ur
.PHONY: eval-translit
eval-translit:
	@for lang in $(TRANSLIT_EVAL_LANGS); do \
		d=$(DAKSHINA_DIR)/$$lang; \
		test -d $$d || { echo "no $$d: run make fetch-dakshina"; exit 1; }; \
		echo "$$lang, romanized sentences:"; \
		$(TOOLS) eval-translit -- --table translit/$$lang.txt --lexicon predict/$$lang.fst \
			--pairs $$d/romanized/$$lang.romanized.rejoined.aligned.cased_nopunct.tsv | tail -n 1 || exit 1; \
		echo "$$lang, romanization lexicon (test):"; \
		$(TOOLS) eval-translit -- --table translit/$$lang.txt --lexicon predict/$$lang.fst \
			--pairs $$d/lexicons/$$lang.translit.sampled.test.tsv | tail -n 1 || exit 1; \
	done

# The transliteration layouts (layouts/<lang>-abc.json), one for each
# table in translit/: the US keys typing into the language's composer.
.PHONY: translit-layouts
translit-layouts:
	$(PYTHON) scripts/build-translit-layouts.py

.PHONY: check
check:
	$(PYTHON) scripts/build-translit-layouts.py --check
	$(PYTHON) scripts/check-assets.py --base $(CHECK_BASE) \
		--max-shrink $(CHECK_MAX_SHRINK) \
		$(if $(CHECK_REPORT),--report $(CHECK_REPORT))

.PHONY: lock-verify
lock-verify:
	$(LOCKTOOL) verify

# One release of language packs (packs.toml) in dist/packs/, every file a
# release asset: packs.json (each pack's manifest — paths, sizes, sha256,
# licences, upstream inputs from sources.lock — and how each file is
# served), each file gzipped, and NOTICES.md. Named after the crates'
# version. Refuses a data file that no pack claims, then reads the
# release back as a client would.
PACKS_OUT ?= dist/packs
.PHONY: packs
packs:
	$(TOOLS) make-packs -- --out $(PACKS_OUT)

# The release version: one number for the format crate and the data,
# tagged v$(VERSION) when it reaches the default branch (release.yml).
# PART=patch when the format and the crate's API stay as they are -- new
# data, the monthly refresh's bump; fixes; documentation -- and minor when
# either changes or the set of packs does. See scripts/version.py.
VERSION = $(shell python3 scripts/version.py get)
PART   ?= patch
.PHONY: version bump-version
version:
	@python3 scripts/version.py get
bump-version:
	python3 scripts/version.py bump $(PART)

# A released version names one content: refuse a tree that changes what
# v$(VERSION) contains while that release exists (ci.yml runs this).
.PHONY: check-version
check-version:
	python3 scripts/version.py check

# Every asset of release v$(VERSION), in dist/release/: the packs
# (packs.json, each file gzipped, NOTICES.md), the all-languages bundle
# ki-osk-lang-$(VERSION).tar.xz and SHA256SUMS. The bundle is every pack
# installed from those assets as a client installs them:
# `tar -xJf ki-osk-lang-$(VERSION).tar.xz -C /usr/share/ki-osk`. The same
# commit gives the same bytes; the bundle's entries carry its time.
RELEASE_OUT       ?= dist/release
SOURCE_DATE_EPOCH ?= $(shell git log -1 --format=%ct 2>/dev/null || echo 0)
.PHONY: release
release:
	$(TOOLS) make-packs -- --out $(RELEASE_OUT) --bundle --mtime $(SOURCE_DATE_EPOCH)
	xz -9 -T1 -f $(RELEASE_OUT)/ki-osk-lang-$(VERSION).tar
	@$(MAKE) -s release-sums
	@echo "release v$(VERSION): $$(ls $(RELEASE_OUT) | wc -l) assets in $(RELEASE_OUT)"

# SHA256SUMS over every other asset in $(RELEASE_OUT), in C-locale name
# order so it does not depend on whose machine wrote it. Run again after
# adding an asset (release.yml adds a refresh's corpora).
.PHONY: release-sums
release-sums:
	cd $(RELEASE_OUT) && find . -maxdepth 1 -type f ! -name 'SHA256SUMS*' -printf '%P\0' \
	  | LC_ALL=C sort -z | xargs -0 sha256sum -- > SHA256SUMS.tmp && mv SHA256SUMS.tmp SHA256SUMS

# `make -s print-PREDICT_LANGS` — lets the workflows read a variable
# instead of keeping a second copy of it.
print-%:
	@echo $($*)
