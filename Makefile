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
# in scripts/fetch-corpora.sh's MAP too, but for those that learn from
# Wikipedia text alone: Dakshina's (DAKSHINA_ONLY_LANGS) or the wikis' own
# (WIKIPEDIA_LANGS), below. Nine (kab to
# th) were missing until 2026-11-08, so `make predict-assets` silently
# skipped models that were already committed.
PREDICT_LANGS ?= en de fr es it nl pt sv pl no da fi cs tr vi el ru uk ar he fa ro hu sk hr et is ca eo bg sr mk be id fil az ku ug hi bn hy ka lt lv kab \
                 oc gl br eu tt mr as ms os th gu kn ml pa sd si ta te ur \
                 bho mai mn ne or pnb ps sa sat yi kok ks mni awa skr tcy \
                 brx doi hne mag brh sd-Deva mni-Beng lo gbm raj trp wsg lus kha khw \
                 dty anp new km my \
                 sw af sl zu \
                 sq kk uz bs ky tg cy so ga mt ha lb mg rw gd fy jv tk ht ba \
                 xh fo ceb mi su co ny sm sn ig st haw yo dv \
                 ckb pcm \
                 mww cnh kac hil ilo bcl pag pam krj iba bbc ace btx bts bjn dtp ban tet ium \
                 nn ast an lad gsw bar nds pfl ksh li hsb dsb nap scn sc fur pms lij wa ltg vro csb szl crh sco rue \
                 sah cv sme mhr mrj udm tyv bxr kpv koi myv mdf xal krl olo vep \
                 kl om rm rn lg ln fj ee ak ts nso tn to wo ty ve nr ay ss qu bm iu \
                 azb mzn kiu diq tly kaa ce inh av kbd ady lez lbe tab dar kum krc ab xmf \
                 crs nd ng efi sg kj lua lu kea mos rcf kg wal luo tiv iso bem fon ach bci dyu ki kmb umb \
                 la pap syr yua quc kek quz qug srn gcf bi ia tpi gil tvl ho ch jam \
                 sr-Latn gn ff ber kri \
                 bo dz \
                 kbp dag seh ada nyn kwy sid koo kam sop vmw ndc bum bin teo sgc yao dik \
                 min war nia mad \
                 cor glv ext pcd sgs pnt vec vls lmo lld gag \
                 shn mnw ksw blk \
                 nan hak ami za \
                 zne alz cce skg bss men lgg xon heh gux gmv suk \
                 spp bas ngl chw fat tsc \
                 niu hwc acf mrw nij guc gor \
                 quh gcr tum tll guz kde nyy mas mnk mer dga \
                 cop hrx crk csw otw tkl kne mqj liv abs ajg \
                 yrk dng tsg ewo ttj mgh knc nyf laj hnn sda lew sdc \
                 rmy rmn ojb \
                 cnr \
                 bto jax mfp xmm mui mak pmf mfa ibb vmf sdh bgn \
                 rkt \
                 cdo hif egl abr cu \
                 tdx hvn ktu quy \
                 zea \
                 btm gpe igl rki vel \
                 bew nup glk bpy \
                 am ti \
                 myx tuv \
                 hlb kfq nah \
                 ahr bfy bgq bns kfy kru mjl noe sgj sjp unr xnr grt dje grt-Beng \
                 vai tdd \
                 gax mkw yaf \
                 syl-Sylo hac ii kjp

PREDICT_CORPORA_DIR = predict/corpora
PREDICT_OUT_DIR = predict

# Wikipedia text of twelve South Asian languages, from the Dakshina
# dataset (scripts/fetch-dakshina.sh, the archive pinned by its sha256;
# CC BY-SA 4.0). It is the whole corpus of the nine Tatoeba has too few
# sentences of (DAKSHINA_ONLY_LANGS, the last nine of PREDICT_LANGS), and
# joins Tatoeba's for Hindi, Bengali and Marathi, whose lexicons it widens:
# their transliteration tables (translit/) then find far more of the
# words typed. Tatoeba's sentences count TATOEBA_WEIGHT times, so that
# everyday text still leads the strip: words the lexicon lacks fell from
# 4.0% to 1.6% (hi). At 30 (from 10) keystroke savings on Tatoeba's
# held-out text rise 2.3 points (hi), 0.9 (bn), 0.6 (mr) and 2.0 (as,
# beside FineWeb-2's text), for transliteration's first reading on
# Dakshina's test sentences 0.3, 0.2 and 0.5 points less often right and
# Assamese's savings on the web's text 0.6 points lower; at 100 Hindi's
# rise 0.7 more and its transliteration falls 0.4 more. The keystrokes
# transliteration saves on Dakshina's test sentences fell with it: Hindi
# 0.6 points, net of the single letters' 0.4 gain, Bengali 0.2, Marathi
# 1.7, whose Tatoeba text then counts ten times still
# (TATOEBA_LIGHT_LANGS).
DAKSHINA_LANGS ?= bn gu hi kn ml mr pa sd si ta te ur
DAKSHINA_ONLY_LANGS ?= gu kn ml pa sd si ta te ur
DAKSHINA_DIR = $(SRC_DIR)/dakshina
TATOEBA_WEIGHT ?= 30
TATOEBA_LIGHT_LANGS ?= mr
TATOEBA_LIGHT_WEIGHT ?= 10

# Wikipedia's own text, the whole corpus of languages Tatoeba and Dakshina
# have too little of: Bhojpuri, Maithili, Mongolian, Nepali, Odia, Western
# Punjabi, Pashto, Sanskrit, Santali, Yiddish, Konkani, Kashmiri, Meitei,
# Awadhi, Saraiki and Tulu (scripts/fetch-wikipedia.sh, the wikis'
# CirrusSearch dumps pinned by sha256, CC BY-SA 4.0), every 20th article
# held out for eval/<lang>.txt. And the Wikimedia Incubator's test wikis
# of languages with no Wikipedia yet: Adilabad Gondi's, Khowar's and
# Montenegrin's, their whole corpus, and Magahi's, beside its FineWeb-2
# text; and twelve small ones (17 000 to 89 000 words), each its whole
# corpus, every 10th article held out and their bots' stubs left out:
# Rinconada Bikol, five Malays (Jambi, Makassar, Manado, Palembang's
# Musi, Kelantan-Pattani), Makassarese, Pamona, Ibibio, East Franconian,
# Southern Kurdish and Western Balochi; and Rangpuri's, the same way
# (27 000 words, in the Bengali script as Assamese writes it, ৰ for র),
# and Abron's (18 000 words, its sentences that write ɛ or ɔ: a third
# of its wiki's are English). And three wikis no model learned from
# before: Eastern Min's, in Bàng-uâ-cê (170 000 words, its Han left
# out), Fiji Hindi's (620 000, its English references left out) and the
# Emilian–Romagnol wiki's Emilian (2.2 million, its Romagnol left out),
# their bots' stubs left out as the small wikis' are; and Church
# Slavonic's (60 000 words, the sentences that write its own letters);
# and Zeelandic's (460 000 words, its bots' French communes and calendar
# days left out). And five more no round had used: Batak Mandailing's
# (146 000 words), Ghanaian Pidgin's (2.6 million) and Igala's (330 000),
# the sentences that write their own words (their references English,
# some Indonesian), Arakanese's, in the Myanmar script (680 000 words
# between its spaces, cut into syllables as Pa'O's are), and Veluws',
# the Dutch Low Saxon wiki's articles in it (229 000). And four whose
# keyboards had no model: Betawi's (270 000 words), Nupe's (115 000) and
# Gilaki's (271 000, read as Persian's keys type it), the sentences that
# write their own words, and Bishnupriya Manipuri's (103 000), their
# bots' stubs left out. And the Incubator's Vai, in its own script, its
# Latin words left out (200 000 words), whose keyboard offers the model's
# words through its transliteration table, and Tai Nüa's, in Tai Le (87 000
# words, its Chinese and Latin words left out, every 10th article held
# out). And Sylheti's, in Syloti Nagri (85 000 words; syl-Sylo, as
# Sylheti's keyboard in the Bengali script has no model), every 10th
# article held out. And the Incubator's Nuosu, in Yi (84 000 syllables,
# each a word: Yi writes no space between its words), its lists of links
# and its bots' stubs left out, every 10th article held out, beside its
# FineWeb-2 text; its keyboard offers the model's syllables through its
# transliteration table, as Vai's does. And the Incubator's Eastern Pwo
# Karen, in the Myanmar script (194 000 characters), cut into syllables
# with the asat its initials write too (လ်ု), every 10th article held out.
WIKIPEDIA_LANGS ?= bho mai mn ne or pnb ps sa sat yi kok ks mni awa skr tcy dty anp new mag wsg khw cnr \
                   bto jax mfp xmm mui mak pmf mfa ibb vmf sdh bgn rkt abr cdo hif egl cu zea \
                   btm gpe igl rki vel bew nup glk bpy vai tdd syl-Sylo ii kjp
WIKIPEDIA_DIR = $(SRC_DIR)/wikipedia
# And beside a Tatoeba corpus: Armenian and Persian, whose lexicons lacked
# four fifths and half of Wiktionary's words, the first 400 000 sentences
# of their wikis beside Tatoeba's, which count TATOEBA_WIKI_WEIGHT times
# (their wikis' text is eight and thirteen times Tatoeba's). Their
# held-out text stays Tatoeba's (eval/<lang>.txt); the wiki's held out
# goes to $(WIKIPEDIA_DIR)/eval, uncommitted. With 80 000 words
# (PREDICT_LARGE_LANGS): keystroke savings on Tatoeba's held-out text
# 46.6% -> 48.2% (Armenian), 41.0% -> 41.9% (Persian), its words unknown
# 4.8% -> 2.4% and 3.6% -> 2.3%, Wiktionary's words the lexicons have
# 20% -> 50% and 46% -> 77%; counted ten times, as Hindi's were, the
# savings fell two points. Their sentences are cut where a Latin word
# stands (wikipedia-text.py --script-only): a tenth of their commonest
# words were the references' Latin.
WIKIPEDIA_PLUS_LANGS ?= hy fa
TATOEBA_WIKI_WEIGHT ?= 100

# FineWeb-2's web text, the whole corpus of languages with no Wikipedia of
# their own: Bodo, Dogri, Chhattisgarhi, Brahui, Kokborok, Mizo and Khasi
# (Magahi's beside its Incubator text), of Sindhi in Devanagari and Meitei
# in the Bengali script, of Lao, Khmer and Burmese, whose wikis are
# small or few write as people type, of thirty-eight languages that
# had a keyboard and no model, from Swahili to Zulu, of Central
# Kurdish (Sorani) and Nigerian Pidgin (beside GlotCC's, below), and of
# nineteen languages of Southeast Asia, from Hmong Daw to Iu Mien, of
# twenty-six of Europe's regional languages, from Nynorsk to Rusyn, of
# fifteen of Russia's and the north's, from Sakha to Veps, and of the
# twenty-three more that had a keyboard and no model, from Udmurt and
# Greenlandic to Inuktitut, of nineteen of Iran's and the Caucasus', from
# South Azerbaijani to Mingrelian, of twenty-four of Africa's, from
# Seychellois Creole to Umbundu, of eighteen of the Americas', the
# Pacific's and the classical and constructed, from Latin to Jamaican
# Patois, and of six more that had a keyboard and no model: Serbian in
# Latin letters, Guaraní, Fula, Tamazight in Tifinagh, Tibetan and
# Dzongkha, and of twenty of Africa's and four of Southeast Asia's that it
# has 1 to 2 MB of, from Kabiye to Madurese, of eleven more of Europe's
# regional languages, from Cornish to Gagauz, of four of Myanmar's, Shan,
# Mon, Sgaw Karen and Pa'O, of four of East Asia's in Latin letters, of
# fifty-one of the long tail, from Zande to Dagaare, of twelve more:
# Coptic, Lahu, and ten that it has 0.15 to 0.3 MB of, from Hunsrik to Aja,
# and of thirteen more of the long tail, from Nenets to Sassarese
# (scripts/fetch-fineweb.sh, the dataset's parquet files at one revision
# pinned by sha256; ODC-By 1.0, subject to Common Crawl's Terms of Use).
# scripts/fineweb-text.py keeps the documents that read as their language;
# the held-out text is FineWeb-2's test split, in $(FINEWEB_DIR)/eval, and
# like the corpus it is not committed. Reading parquet needs pyarrow:
# FINEWEB_PYTHON names a Python that has it.
FINEWEB_LANGS ?= brx doi hne mag brh sd-Deva mni-Beng lo trp lus kha km my \
                 sw af sl zu \
                 sq kk uz bs ky tg cy so ga mt ha lb mg rw gd fy jv tk ht ba \
                 xh fo ceb mi su co ny sm sn ig st haw yo dv \
                 ckb pcm \
                 mww cnh kac hil ilo bcl pag pam krj iba bbc ace btx bts bjn dtp ban tet ium \
                 nn ast an lad gsw bar nds pfl ksh li hsb dsb nap scn sc fur pms lij wa ltg vro csb szl crh sco rue \
                 sah cv sme mhr mrj udm tyv bxr kpv koi myv mdf xal krl olo vep \
                 kl om rm rn lg ln fj ee ak ts nso tn to wo ty ve nr ay ss qu bm iu \
                 azb mzn kiu diq tly kaa ce inh av kbd ady lez lbe tab dar kum krc ab xmf \
                 crs nd ng efi sg kj lua lu kea mos rcf kg wal luo tiv iso bem fon ach bci dyu ki kmb umb \
                 la pap syr yua quc kek quz qug srn gcf bi ia tpi gil tvl ho ch jam \
                 sr-Latn gn ff ber bo dz \
                 kbp dag seh ada nyn kwy sid koo kam sop vmw ndc bum bin teo sgc yao dik \
                 min war nia mad \
                 cor glv ext pcd sgs pnt vec vls lmo lld gag \
                 shn mnw ksw blk \
                 nan hak ami za \
                 zne alz cce skg bss men lgg xon heh gux gmv suk \
                 spp bas ngl chw fat tsc \
                 niu hwc acf mrw nij guc gor \
                 quh gcr tum tll guz kde nyy mas mnk mer dga \
                 cop hrx crk csw otw tkl kne mqj liv abs ajg \
                 yrk dng tsg ewo ttj mgh knc nyf laj hnn sda lew sdc \
                 rmy rmn ojb \
                 am ti \
                 syl-Sylo hac ii
# (Amharic's, Tigrinya's and Sebat Bet Gurage's: what the Ge'ez composer
# offers words from. And Sylheti's in Syloti Nagri, a news site's pages,
# beside its wiki's text, and Gorani's, the Hawrami among FineWeb-2's
# hac_Arab, whose keyboard had no model. And Nuosu's, the pages of
# FineWeb-2's und_Yiii that read as Nuosu, beside its wiki's text.)
# A language with web text and a wiki's (Magahi, whose Incubator text is
# twice its FineWeb-2 text; Newari, below) counts the web's WEB_WEIGHT
# times: the web's text, nearer what is typed than a wiki's stubs, leads
# the strip. At 5 Magahi's keystroke savings on FineWeb-2's held-out text
# are as before (31.4%), on its Incubator's held-out articles 38.7% (from
# 17.8%); at 1, 29.8% and 42.4%.
WEB_WEIGHT ?= 5
# But Sylheti's in Syloti Nagri, whose wiki's articles are prose, not
# stubs, counts its news site's pages once (WEB_EVEN_LANGS): at 1 its
# wiki's held-out articles are saved 29.8% of their keystrokes (30.1%
# with the wiki alone, 27.7% at 5, its right words changed 0.73%) and its
# news's 49.8% (24.3% with the wiki alone; 51.8% at 2 and 5). So does
# Nuosu's, whose web text is half its wiki's: its wiki's held-out articles
# are saved 38.5% of their keystrokes at 1, 37.2% at 5.
WEB_EVEN_LANGS ?= syl-Sylo ii
# And beside a small Tatoeba corpus, counted TATOEBA_WEIGHT times as
# Hindi's is beside Dakshina's text: Assamese (14 000 sentences).
FINEWEB_PLUS_LANGS ?= as
FINEWEB_DIR = $(SRC_DIR)/fineweb

# MADLAD-400's web text (scripts/fetch-madlad.sh, its files at one revision
# pinned by sha256; CC BY 4.0 by its card, subject to Common Crawl's Terms
# of Use), the whole corpus of languages neither Wikipedia nor FineWeb-2
# has: Garhwali, Rajasthani and Krio, and Tandroy-Mahafaly Malagasy,
# Hawu, Kituba and Ayacucho Quechua (their Bibles and Jehovah's
# Witnesses' pages, a document kept where its own words are 3-5% of its
# words). scripts/fineweb-text.py keeps the documents that read as their
# language and holds out every 20th (Krio's and Hawu's every 10th), in
# $(MADLAD_DIR)/eval; nothing of it is committed.
MADLAD_LANGS ?= gbm raj kri tdx hvn ktu quy
MADLAD_DIR = $(SRC_DIR)/madlad

# The African Storybook's stories (scripts/fetch-storybooks.sh, the
# asp-raw-db repository's archive at one commit pinned by sha256; each
# story CC BY 3.0 or 4.0, the CC BY-NC ones left out), the whole corpus of
# languages that no web text or wiki has: Masaaba and Turkana (20 000 and
# 23 000 words). scripts/storybook-text.py leaves out the pages another
# language's and holds out every 10th story, in $(STORYBOOK_DIR)/eval;
# nothing of it is committed.
STORYBOOK_LANGS ?= myx tuv
STORYBOOK_DIR = $(SRC_DIR)/storybooks

# StoryWeaver's stories (scripts/fetch-storyweaver.sh, the stories
# scripts/storyweaver-stories.tsv names, each pinned by the sha256 of its
# text, read through the site's reader with an account; each CC BY 4.0),
# the whole corpus of languages that no web text or wiki has: Halbi,
# Korku and Nahuatl (33 000, 30 000 and 40 000 words).
# scripts/storyweaver-text.py leaves out the pages another language's and
# holds out every 10th story, in $(STORYWEAVER_DIR)/eval; nothing of it is
# committed. Without an account (STORYWEAVER_EMAIL, STORYWEAVER_PASSWORD)
# and the stories not read before, fetch-storyweaver reads none, and
# predict-assets keeps these languages' models as they are.
STORYWEAVER_LANGS ?= hlb kfq nah
STORYWEAVER_DIR = $(SRC_DIR)/storyweaver

# SMOL's professional translations (scripts/fetch-smol.sh, google/smol's
# files at one revision pinned by sha256; CC BY 4.0, Google Research), the
# whole corpus of languages no web text or wiki has: twelve of India's in
# Devanagari (Ahirani, Bagheli, Bagri, Bundeli, Kumaoni, Kurukh,
# Mandeali, Nimadi, Surgujia, Surjapuri, Mundari, Kangri), Garo in Latin
# letters and Zarma (55 000 to 93 000 words each, Zarma's 35 000).
# scripts/smol-text.py holds out every 10th document, in $(SMOL_DIR)/eval;
# nothing of it is committed.
SMOL_LANGS ?= ahr bfy bgq bns kfy kru mjl noe sgj sjp unr xnr grt dje
SMOL_DIR = $(SRC_DIR)/smol
# Garo in the Bengali script, which no open text is written in: SMOL's
# Garo, in Latin letters, spelled in the Bengali script by its table
# (translit/grt-Beng.txt, crates/osk-data-tools/src/bin/respell.rs), each
# word as the table reads it; make respelled-text writes
# $(SMOL_DIR)/grt-Beng.txt and its held-out half, after fetch-smol.
RESPELLED_LANGS ?= grt-Beng

# eBible.org's Bibles (scripts/fetch-ebible.sh, each translation's verses
# pinned by sha256; CC BY-SA 4.0, each under its own copyright), the whole
# corpus of languages whose keyboards have no other open text: Southern
# Oromo's (Guji's Bible, 542 000 words), Kituba of the Congo Republic's
# (803 000) and Yaka's (its New Testament, 127 000). And Gamo's Bible and
# New Testament (the latter CC BY 4.0; 584 000 words) beside its web
# text, which bible.is's pages were more than half of (data 0.84.0): the
# web's counted WEB_WEIGHT times, as beside a wiki's, its savings on its
# held-out web text 44.3% (40.2% without the Bibles; 42.5% with the Bible
# alone; 44.0% with the web's counted 3 times, 45.0% 10 times), 38.9% with
# taps astray (31.6%), 5.2% of its words unknown (14.0%). Gourmanché's
# two New Testaments are left out: FineWeb-2's text of it is half one of
# them, and the other, of Togo's Gourmanché, saved 0.1% more of the
# held-out words that are neither. scripts/ebible-text.py holds out every
# 10th chapter, in $(EBIBLE_DIR)/eval; nothing of it is committed. A
# translation changed since its pin is not read, and predict-assets keeps
# its language's model.
EBIBLE_LANGS ?= gax mkw yaf gmv
EBIBLE_DIR = $(SRC_DIR)/ebible

# GlotCC's web text (scripts/fetch-glotcc.sh, its files at one revision
# pinned by sha256; its packaging CC0 1.0, its text under Common Crawl's
# Terms of Use), for languages whose wiki is mostly a bot's stubs: Newari,
# beside its wiki's text (WEB_WEIGHT); and for Nigerian Pidgin, Kabiye,
# Minangkabau and Waray beside FineWeb-2's text, whose sentences it leaves
# out (fetch-fineweb first).
# scripts/fineweb-text.py keeps the documents that read as their language
# and holds out every 20th, in $(GLOTCC_DIR)/eval; nothing of it is
# committed. Reading parquet needs pyarrow (FINEWEB_PYTHON).
GLOTCC_LANGS ?= new pcm kbp min war
GLOTCC_DIR = $(SRC_DIR)/glotcc
FINEWEB_PYTHON ?= python3

# Words romanized by hand, from the test split of AI4Bharat's Aksharantar
# dataset (scripts/fetch-aksharantar.sh, each zip pinned by sha256): what
# eval-translit measures the tables Dakshina has nothing for on. Never
# committed or shipped.
AKSHARANTAR_LANGS ?= as mai ne or sa kok ks mni brx doi
AKSHARANTAR_DIR = $(SRC_DIR)/aksharantar
# Meitei's words in the Bengali script, for its table there
# (translit/mni-Beng.txt), which no open data romanizes: Aksharantar's
# Meetei Mayek words respelled in the Bengali script as its FineWeb-2 text
# spells them (scripts/meetei-to-bengali.py; make aksharantar-respelled,
# after fetch-aksharantar and fetch-fineweb), a quarter of them; measured
# and fitted as the others. Never committed or shipped.
AKSHARANTAR_RESPELLED ?= mni-Beng

# Greek typed in Latin letters ("Greeklish"), word by word: AUEB's
# real-world test set (scripts/fetch-greeklish.sh, its files pinned by
# sha256; Apache-2.0), what eval-translit measures Greek's table on. Never
# committed or shipped.
GREEKLISH_DIR = $(SRC_DIR)/greeklish

# Words and their romanizations from Wiktionary, as kaikki.org extracts
# them (scripts/fetch-wiktionary-romanized.sh; CC BY-SA 4.0): what
# eval-translit measures Yiddish's table on, Wiktionary's romanization of
# Yiddish being YIVO's, the one people type; and Armenian's, Persian's,
# Pashto's, Mongolian's and Santali's, their scholarly romanizations
# folded to the letters each table reads (scripts/wiktionary-romanized.py):
# how often the table and lexicon find a word spelled as the table
# expects, not how people spell it. kaikki.org keeps only its latest
# extract, so the measure moves with it. Never committed or shipped.
WIKTIONARY_ROMANIZED_LANGS ?= yi hy fa ps mn sat lo
WIKTIONARY_ROMANIZED_DIR = $(SRC_DIR)/wiktionary

# CMUdict, how American English says its words (scripts/fetch-cmudict.sh,
# pinned by sha256; BSD-2-Clause-style): what english-assets spells and
# reads with each language's table. Only the English words made from it
# are committed, translit/<lang>.english.txt.
CMUDICT_DIR = $(SRC_DIR)/cmudict

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
# NAME must be built; `record NAME ARGS...` writes its lock entry;
# `words_gen LANG` prints the splitter generation LANG's words are read by.
LOCKED = stale() { [ -z "$(ONLY_STALE)" ] || ! $(LOCKTOOL) fresh "$$@"; }; \
         record() { $(LOCKTOOL) record "$$@" --encoder "$(ENCODER)"; }; \
         words_gen() { g=$(WORDS_GENERATION); for p in $(WORDS_LATER); do \
           if [ "$${p%%=*}" = "$${1%%-*}" ]; then g=$$(echo $$p | cut -d= -f2); fi; done; echo $$g; }

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
PREDICT_COMPACT_LANGS ?= no da fi cs tr vi el ru uk ar he ro hu sk hr et is ca eo bg sr mk be \
                         id fil az ku ug ka lt lv kab
PREDICT_COMPACT_VOCAB ?= 18000
PREDICT_COMPACT_NGRAM ?= 30000
# The LARGE tier: languages whose words, kept whole (osk_data::words), are
# too many for the full tier's caps. Marathi has 25 k word types in its
# Tatoeba corpus once viramas no longer cut them; with Wikipedia text
# (DAKSHINA_LANGS) its and Bengali's lexicons keep growing: from 40 000
# words (Marathi) and 30 000 (Bengali) to 80 000, transliteration's first
# reading is right 3.8 and 2.0 points more often, and keystroke savings on
# Tatoeba's held-out text rise 0.5 and 1.9 points. Assamese, Bengali's
# neighbour, with FineWeb-2's text (FINEWEB_PLUS_LANGS), too; and Magahi,
# with its Incubator text beside FineWeb-2's (72 000 words; at 30 000 its
# keystroke savings on FineWeb-2's held-out text fall 1.1 points more);
# and Newari, with its news beside its wiki (at 30 000 words its savings
# on the news held out are 38.7%, at 80 000 42.1%, its words unknown
# 6.6%); and the languages whose web text FineWeb-2 has plenty of (at
# 30 000 words Zulu saved 39.3% of the keystrokes of its held-out text,
# Slovenian 37.1%; at 80 000 42.4% and 39.8%, their words unknown 20.1%
# -> 12.9% and 8.8% -> 3.3%); and Southeast Asia's nineteen, whose
# lexicons at 80 000 save 0.3 (Kadazandusun) to 2.5 points (Iloko) more
# than at 30 000, the small corpora's every word (36 000 to 70 000);
# and Europe's twenty-six regional languages, Russia's fifteen, the
# twenty-three more that had a keyboard and no model, Iran's and the
# Caucasus' nineteen, Africa's twenty-four, the Americas', the Pacific's
# and the classical and constructed eighteen, and Serbian in Latin
# letters, Guaraní, Fula, Tamazight in Tifinagh, Krio, Tibetan and
# Dzongkha. Tibetan's and Dzongkha's words are syllables, 49 300 and
# 13 700 of them, so their n-grams do the work: 180 000 save 0.9 and 0.6
# points more than 90 000, 300 000 1.6 and 1.0, for 1.5 and 3.4 MB more
# each, more than a point a megabyte is worth: they keep 90 000. And
# twenty of Africa's and four of Southeast Asia's long tail, whose corpora
# of 160 000 to 1.2 million words hold 6 000 to 84 000 kinds of word:
# Nkore's alone more than 80 000, and at 120 000 it saves 0.2 points more.
# And eleven of Europe's regional languages, whose corpora hold 255 000
# (Pontic Greek) to 4.1 million words (Lombard, Ladin): Cornish and Gagauz
# save no more at 120 000 (0.0 and 0.1 points).
# Shan, Mon, Sgaw Karen and Pa'O are cut into syllables, 3 000 to 9 000
# kinds of them: their lexicons keep them all, at any tier.
# Southern Min, Hakka, Amis and Zhuang, whose corpora hold 200 000 to
# 710 000 words, keep all their 8 400 to 34 000 kinds of word.
# So do the twenty-eight of Africa's long tail with 0.4 to 0.9 MB of
# FineWeb-2's text (2 600 to 25 000 kinds).
# And the twenty-three after them (Niuean… Dagaare), the twelve after
# those (Coptic, Lahu, Hunsrik… Aja: 1 700 to 35 000 kinds), the
# thirteen after those (Nenets… Sassarese), and the two Romanis and
# Northwestern Ojibwe (29 000 to 88 000 kinds: Balkan Romani's saves 0.2
# points more at 120 000). And Montenegrin, from its Incubator test wiki
# (900 000 words, 106 000 kinds), which is WIDE. And Eastern Min, Fiji
# Hindi, Emilian and Zeelandic, from their wikis (Eastern Min's
# syllables, as Southern Min's; Fiji Hindi's 55 000 kinds; Emilian's
# 144 000 and Zeelandic's 60 000, their dialects each spelled their own
# way). And MADLAD-400's Tandroy-Mahafaly Malagasy, Hawu, Kituba and
# Ayacucho Quechua. And Batak Mandailing, Igala and Veluws from their
# wikis, which keep all their 21 000 to 37 000 kinds of word, Arakanese's
# syllables, and Ghanaian Pidgin's, 80 000 of its 113 000 kinds (98.7% of
# its words written; the rest its articles' names, most of them). And
# Masaaba and Turkana from the African Storybook's stories, which keep all
# their kinds of word, as do Halbi, Korku and Nahuatl from StoryWeaver's,
# the fourteen from SMOL's translations and the three from eBible.org's
# Bibles (Southern Oromo's 36 000 kinds; 20 000 would hold 97.0% of its
# words). And Sylheti's in Syloti Nagri, from its wiki: all its 21 000
# kinds, 30.1% of its held-out keystrokes saved (29.6% at 20 000).
PREDICT_LARGE_LANGS ?= bn mr as mag new hy fa \
                       sw af sl zu \
                       sq kk uz bs ky tg cy so ga mt ha lb mg rw gd fy jv tk ht ba \
                       xh fo ceb mi su co ny sm sn ig st haw yo dv \
                       mww cnh kac hil ilo bcl pag pam krj iba bbc ace btx bts bjn dtp ban tet ium \
                       nn ast an lad gsw bar nds pfl ksh li hsb dsb nap scn sc fur pms lij wa ltg vro csb szl crh sco rue \
                       sah cv sme mhr mrj udm tyv bxr kpv koi myv mdf xal krl olo vep \
                       kl om rm rn lg ln fj ee ak ts nso tn to wo ty ve nr ay ss qu bm iu \
                       azb mzn kiu diq tly kaa ce inh av kbd ady lez lbe tab dar kum krc ab xmf \
                       crs nd ng efi sg kj lua lu kea mos rcf kg wal luo tiv iso bem fon ach bci dyu ki kmb umb \
                       la pap syr yua quc kek quz qug srn gcf bi ia tpi gil tvl ho ch jam \
                       sr-Latn gn ff ber kri bo dz \
                       kbp dag seh ada nyn kwy sid koo kam sop vmw ndc bum bin teo sgc yao dik \
                       min war nia mad \
                       cor glv ext pcd sgs pnt vec vls lmo lld gag \
                       shn mnw ksw blk \
                       nan hak ami za \
                       zne alz cce skg bss men lgg xon heh gux gmv suk \
                       spp bas ngl chw fat tsc \
                       niu hwc acf mrw nij guc gor \
                       quh gcr tum tll guz kde nyy mas mnk mer dga \
                       cop hrx crk csw otw tkl kne mqj liv abs ajg \
                       yrk dng tsg ewo ttj mgh knc nyf laj hnn sda lew sdc \
                       rmy rmn ojb \
                       cnr \
                       cdo hif egl zea \
                       tdx hvn ktu quy \
                       btm gpe igl rki vel \
                       bew nup glk bpy \
                       myx tuv \
                       hlb kfq nah \
                       ahr bfy bgq bns kfy kru mjl noe sgj sjp unr xnr grt dje grt-Beng \
                       gax mkw yaf \
                       syl-Sylo
PREDICT_LARGE_VOCAB ?= 80000
PREDICT_LARGE_NGRAM ?= 90000
# The WIDE tier: the Dravidian languages, whose words take their endings
# with them. From 40 000 words to 120 000, the share of Tamil's held-out
# words its lexicon lacks falls from 25.6% to 17.0% (Malayalam's from
# 33.2% to 25.4%), keystroke savings rise 2.7 points (3.0), and
# transliteration's first reading is right 3.4 points more often (4.4),
# for lexicons of 1.4 to 1.7 MB; all four are WIDER now. And the
# languages FineWeb-2's text gives the most word forms, whose lexicons
# lacked 8% to 12% of the held-out words at 80 000 (Kazakh, Bosnian,
# Slovenian, Uzbek, Xhosa, Zulu, Shona, Luxembourgish, Dhivehi, Irish,
# Bashkir): at 120 000 they lack 1 to 2 points fewer, and save 0.3 to 0.8
# points more keystrokes, 0.7 to 1.9 with taps astray (Zulu 43.1% ->
# 43.9%, Kazakh 35.2% -> 35.9%). And Central Kurdish (Sorani), which
# writes its prepositions and endings onto its words: at 80 000 its
# lexicon lacks 12.2% of the held-out words and saves 33.2% of their
# keystrokes, at 120 000 10.4% and 33.6%. And Iloko and Balinese, whose
# lexicons at 80 000 lack 5.7% and 9.7% of their held-out words: at
# 120 000 4.7% and 7.9%, 0.5 and 0.4 points more saved (1.4 and 1.0 with
# taps astray). And nineteen of Europe's regional languages, many of them
# written as each writer speaks (the German and Italian dialects, Kashubian,
# Silesian, Rusyn…), whose lexicons at 80 000 lack 6.9% (Nynorsk) to 21.2%
# (Kashubian) of their held-out words: at 120 000 0.3 to 0.9 points more
# saved, 0.6 to 2.2 with taps astray.
# And fourteen of Russia's, the north's and Africa's whose lexicons at
# 80 000 lack 7.8% (Kirundi) to 17.7% (Quechua) of their held-out words:
# 0.3 to 0.9 points more saved at 120 000. And ten of Iran's and the
# Caucasus', whose lexicons at 80 000 lack 8.1% (Lak) to 15.5% (South
# Azerbaijani): 0.3 to 0.8 points more. And Northern Ndebele and Cape
# Verdean Creole, which lack 11.1% and 12.0% at 80 000: 0.9 and 0.5 points
# more at 120 000. And Cusco Quechua, which lacks 14.9%: 1.3 points more.
# And Guaraní, Fula and Serbian in Latin letters, which lack 8.8%, 8.4%
# and 2.9% at 80 000: at 120 000 7.7%, 7.3% and 2.0%, 0.5, 0.3 and 0.2
# points more saved (1.0, 0.7 and 0.9 with taps astray). And Picard,
# Extremaduran, Venetian, Lombard and West Flemish, which lack 5.1%, 9.1%,
# 6.7%, 11.2% and 10.0% at 80 000: at 120 000 4.5%, 8.4%, 5.5%, 9.5% and
# 8.3%, 0.3 to 0.9 points more saved (0.3 to 2.1 with taps astray).
# And Montenegrin, which lacks 10.4% at 80 000: at 120 000 8.6%, 0.4
# points more saved (1.0 with taps astray).
PREDICT_WIDE_LANGS ?= kk bs sl uz xh zu sn lb dv ga ba ckb ilo ban \
                      nn gsw bar nds pfl ksh li hsb dsb nap scn sc lij ltg vro csb szl sco rue \
                      sah sme lg vep qu krl olo udm bxr mdf myv tyv kpv rn \
                      ab ady lbe krc kaa inh kiu azb tly diq nd kea quz \
                      gn ff sr-Latn \
                      pcd ext vec lmo vls \
                      cnr
PREDICT_WIDE_VOCAB ?= 120000
PREDICT_WIDE_NGRAM ?= 90000
# The WIDER tier: the Dravidian languages, whose lexicons still lack a
# sixth to a quarter of the words typed at 120 000. At 200 000 Malayalam's
# (3.0 MB, from 1.7) lacks 22.8% of its held-out words (from 25.4%), its
# keystroke savings rise a point (30.7%), and transliteration's first
# reading is right 1.4 points more often; Tamil's, Telugu's and
# Kannada's first readings 0.8-1.1 points, for a megabyte more each
# (packs fetched as a language is wanted, none in the Android app). And
# the languages that build words from many parts whose web text is
# small: Greenlandic, Inuktitut, Swati, Aymara and Southern Ndebele, whose
# lexicons at 80 000 lack 14.6% (Ndebele) to 36.0% (Inuktitut) of their
# held-out words; at 200 000 10.9% to 32.4%, 0.7 to 1.8 points more saved
# and 2.2 to 6.5 with taps astray (Greenlandic 1.7% -> 8.2%). And
# Kabardian and Mingrelian, which lack 22.3% and 21.9% at 80 000: at
# 200 000 17.5% and 18.3%, 1.4 and 1.0 points more saved. And Latin,
# whose words take many forms: at 80 000 it lacks 15.2% of its held-out
# words, at 200 000 8.7%, and saves 1.7 points more (4.6 with taps astray).
# And Samogitian, whose 620 000 words hold 126 500 kinds: at 80 000 it
# lacks 18.1% of its held-out words, at 200 000 15.2%, and saves 0.8
# points more (2.3 with taps astray; at 120 000 0.7 and 2.1). And
# Amharic and Tigrinya, whose words take their prefixes and suffixes with
# them: at 120 000 they lack 9.3% and 11.4% of their held-out words, at
# 200 000 7.5% and 9.0%, and the Ge'ez composer's strip saves 0.5 and 0.6
# points more of their taps (1.6 MB each, from 1.0). And Gorani
# (Hawrami), whose 770 000 words have 146 000 kinds: at 80 000 it lacks
# 12.6% of its held-out words, at 200 000 10.3%, and saves 0.5 points
# more (1.4 with taps astray; 1.2 MB, from 0.65).
PREDICT_WIDER_LANGS ?= kn ml ta te kl iu ss ay nr kbd xmf la sgs am ti hac
PREDICT_WIDER_VOCAB ?= 200000
PREDICT_WIDER_NGRAM ?= 90000
# The WIKI tier: the other languages whose lexicons come from Wikipedia
# text too (DAKSHINA_LANGS); twice as many words gain Hindi's
# transliteration under a point. And the small web corpora: Nigerian
# Pidgin's 540 000 words have 26 565 kinds (at 20 000 it saves 43.5% of
# its held-out keystrokes, with all of them 44.0%).
PREDICT_WIKI_LANGS ?= hi gu pa sd si ur bho mai mn ne or pnb ps sa sat yi kok ks mni awa skr tcy \
                      brx doi hne brh sd-Deva mni-Beng lo gbm raj trp wsg lus kha khw dty anp km my \
                      pcm
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
# recipe, and a new generation makes every one of them stale. A language
# whose words a later generation changed alone has that one
# (`LATER_GENERATIONS`, read here as lang=generation, `words_gen` in
# LOCKED), and only its models go stale.
WORDS_GENERATION = $(shell sed -n 's/^pub const GENERATION: u32 = \([0-9]*\);/\1/p' crates/osk-data/src/words.rs)
WORDS_LATER = $(shell sed -n '/^pub const LATER_GENERATIONS/,/];/p' crates/osk-data/src/words.rs \
	| grep -o '("[a-zA-Z-]*", [0-9]*)' | tr -d '("),' | tr ' ' '=')

.PHONY: help
help:
	@echo "targets:"
	@echo "  refresh          fetch everything, rebuild only what changed, check"
	@echo "                   (what the scheduled GitHub workflow runs)"
	@echo "  fetch-corpora    download Tatoeba corpora into predict/corpora/"
	@echo "  fetch-dakshina   Wikipedia text and romanizations of 12 South Asian"
	@echo "                   languages (the Dakshina dataset) into SRC_DIR"
	@echo "  fetch-wikipedia  the wikis' own text, for WIKIPEDIA_LANGS, into SRC_DIR"
	@echo "  fetch-aksharantar  romanized words for eval-translit, into SRC_DIR"
	@echo "  fetch-greeklish  Greek typed in Latin letters, for eval-translit"
	@echo "  fetch-wiktionary-romanized  Wiktionary's romanized words of yi hy"
	@echo "                   fa ps mn sat, for eval-translit"
	@echo "  fetch-cmudict    CMUdict, English's pronunciations, for english-assets"
	@echo "  fetch-fineweb    FineWeb-2's web text, for FINEWEB_LANGS, into SRC_DIR"
	@echo "                   (FINEWEB_PYTHON: a Python with pyarrow)"
	@echo "  fetch-madlad     MADLAD-400's web text, for MADLAD_LANGS, into SRC_DIR"
	@echo "  fetch-storybooks  the African Storybook's stories, for STORYBOOK_LANGS,"
	@echo "                   into SRC_DIR"
	@echo "  fetch-storyweaver  StoryWeaver's stories, for STORYWEAVER_LANGS, into"
	@echo "                   SRC_DIR (STORYWEAVER_EMAIL, STORYWEAVER_PASSWORD: an"
	@echo "                   account, for stories not read before)"
	@echo "  fetch-smol       SMOL's translations, for SMOL_LANGS, into SRC_DIR"
	@echo "  respelled-text   Garo's SMOL text spelled in the Bengali script, into SRC_DIR"
	@echo "  fetch-ebible     eBible.org's Bibles, for EBIBLE_LANGS, into SRC_DIR"
	@echo "  fetch-glotcc     GlotCC's web text, for GLOTCC_LANGS, into SRC_DIR"
	@echo "                   (FINEWEB_PYTHON: a Python with pyarrow)"
	@echo "  word-lists       check (or fetch) ICU's word lists in wordbreak/"
	@echo "  syllable-lists   SYLLABLE_LANGS' word lists, their syllables, in"
	@echo "                   wordbreak/ (after fetch-fineweb)"
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
	@echo "                   meant first (after fetch-dakshina, fetch-aksharantar,"
	@echo "                   fetch-greeklish, fetch-wiktionary-romanized)"
	@echo "  fit-translit     fit the Dakshina and Aksharantar languages' tables'"
	@echo "                   costs to their training romanizations (after"
	@echo "                   fetch-dakshina, fetch-aksharantar)"
	@echo "  fit-ranker       how much a learned ranking of the readings would do"
	@echo "                   (after fetch-dakshina; writes nothing)"
	@echo "  translit-layouts write layouts/<lang>-abc.json for translit/<lang>.txt"
	@echo "  derived-layouts  write the layouts made from another language's keys"
	@echo "  exemplar-layouts write the layouts made from CLDR's exemplar characters"
	@echo "  keyman-layouts   write the layouts made from Keyman's touch keyboards"
	@echo "  keyman-composers write Vai's and Nuosu's tables and layouts, from Keyman's"
	@echo "                   keyboards' rules"
	@echo "  hinglish-assets  rebuild predict/hi-Latn.* (Hindi typed in Latin letters)"
	@echo "                   from Hindi's corpora respelled (after fetch-dakshina)"
	@echo "  english-assets   rebuild translit/<lang>.english.txt, the English words"
	@echo "                   each table's language writes (after fetch-cmudict)"
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
refresh: fetch-corpora fetch-dakshina fetch-wikipedia fetch-fineweb fetch-madlad fetch-storybooks fetch-storyweaver fetch-smol fetch-ebible fetch-glotcc fetch-cmudict
	$(MAKE) predict-assets hinglish-assets cjk-assets dialect-assets ONLY_STALE=1
	$(MAKE) english-assets ONLY_STALE=1
	$(MAKE) check

.PHONY: fetch-corpora
fetch-corpora:
	./scripts/fetch-corpora.sh

.PHONY: fetch-dakshina
fetch-dakshina:
	OUT=$(DAKSHINA_DIR) ./scripts/fetch-dakshina.sh

.PHONY: fetch-wikipedia
fetch-wikipedia:
	OUT=$(WIKIPEDIA_DIR) ./scripts/fetch-wikipedia.sh $(WIKIPEDIA_LANGS)
	OUT=$(WIKIPEDIA_DIR) EVAL_OUT=$(WIKIPEDIA_DIR)/eval ./scripts/fetch-wikipedia.sh $(WIKIPEDIA_PLUS_LANGS)

.PHONY: fetch-aksharantar
fetch-aksharantar:
	OUT=$(AKSHARANTAR_DIR) ./scripts/fetch-aksharantar.sh $(AKSHARANTAR_LANGS)

.PHONY: respelled-text
respelled-text:
	@test -f $(SMOL_DIR)/grt.txt || { echo "no $(SMOL_DIR)/grt.txt: run make fetch-smol"; exit 1; }
	$(TOOLS) respell -- --table translit/grt-Beng.txt \
		--join-dots --text $(SMOL_DIR)/grt.txt --out $(SMOL_DIR)/grt-Beng.txt --show 0
	$(TOOLS) respell -- --table translit/grt-Beng.txt \
		--join-dots --text $(SMOL_DIR)/eval/grt.txt --out $(SMOL_DIR)/eval/grt-Beng.txt --show 0

.PHONY: aksharantar-respelled
aksharantar-respelled:
	$(PYTHON) -I scripts/meetei-to-bengali.py $(AKSHARANTAR_DIR) $(FINEWEB_DIR)/mni-Beng.txt

.PHONY: fetch-greeklish
fetch-greeklish:
	OUT=$(GREEKLISH_DIR) ./scripts/fetch-greeklish.sh

.PHONY: fetch-wiktionary-romanized
fetch-wiktionary-romanized:
	OUT=$(WIKTIONARY_ROMANIZED_DIR) ./scripts/fetch-wiktionary-romanized.sh $(WIKTIONARY_ROMANIZED_LANGS)

.PHONY: fetch-fineweb
fetch-fineweb:
	OUT=$(FINEWEB_DIR) PYTHON=$(FINEWEB_PYTHON) ./scripts/fetch-fineweb.sh $(FINEWEB_LANGS) $(FINEWEB_PLUS_LANGS)

.PHONY: fetch-cmudict
fetch-cmudict:
	OUT=$(CMUDICT_DIR) ./scripts/fetch-cmudict.sh

.PHONY: fetch-madlad
fetch-madlad:
	OUT=$(MADLAD_DIR) ./scripts/fetch-madlad.sh $(MADLAD_LANGS)

.PHONY: fetch-storybooks
fetch-storybooks:
	OUT=$(STORYBOOK_DIR) ./scripts/fetch-storybooks.sh $(STORYBOOK_LANGS)

.PHONY: fetch-storyweaver
fetch-storyweaver:
	OUT=$(STORYWEAVER_DIR) ./scripts/fetch-storyweaver.sh $(STORYWEAVER_LANGS)

.PHONY: fetch-smol
fetch-smol:
	OUT=$(SMOL_DIR) ./scripts/fetch-smol.sh $(SMOL_LANGS)

.PHONY: fetch-ebible
fetch-ebible:
	OUT=$(EBIBLE_DIR) ./scripts/fetch-ebible.sh $(EBIBLE_LANGS)

.PHONY: fetch-glotcc
fetch-glotcc:
	OUT=$(GLOTCC_DIR) FINEWEB_OUT=$(FINEWEB_DIR) PYTHON=$(FINEWEB_PYTHON) ./scripts/fetch-glotcc.sh $(GLOTCC_LANGS)

# The word lists of the languages written without spaces between words
# (wordbreak/<lang>.txt, committed): ICU's word-break dictionaries, pinned
# by release and sha256 in the script. build-predict cuts their text into
# words with them, and so do the keyboard and eval-predict.
.PHONY: word-lists
word-lists:
	./scripts/fetch-word-lists.sh

# The languages cut into syllables (osk_data::wordbreak), no list of whose
# words is to be had: Shan, Mon, Sgaw Karen, Pa'O, Arakanese and Eastern
# Pwo Karen. Their word lists (wordbreak/<lang>.txt, committed) are the
# syllables their FineWeb-2 text writes twice or more (Arakanese's and
# Eastern Pwo Karen's, their wikis': SYLLABLE_WIKI_LANGS), made from it
# (fetch-fineweb, fetch-wikipedia) before their models are.
SYLLABLE_LANGS ?= shn mnw ksw blk rki kjp
SYLLABLE_WIKI_LANGS ?= rki kjp

.PHONY: syllable-lists
syllable-lists:
	@for lang in $(SYLLABLE_LANGS); do \
		dir=$(FINEWEB_DIR); \
		case " $(SYLLABLE_WIKI_LANGS) " in *" $$lang "*) dir=$(WIKIPEDIA_DIR) ;; esac; \
		$(TOOLS) build-syllables -- --lang $$lang --corpus $$dir/$$lang.txt \
			--out $(CURDIR)/wordbreak/$$lang.txt || exit 1; \
	done

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
		case " $(PREDICT_WIDE_LANGS) " in \
			*" $$lang "*) vocab=$(PREDICT_WIDE_VOCAB); ngram=$(PREDICT_WIDE_NGRAM) ;; \
		esac; \
		case " $(PREDICT_WIDER_LANGS) " in \
			*" $$lang "*) vocab=$(PREDICT_WIDER_VOCAB); ngram=$(PREDICT_WIDER_NGRAM) ;; \
		esac; \
		corpus=; corpus_in=; weight=; \
		case " $(DAKSHINA_ONLY_LANGS) $(WIKIPEDIA_LANGS) $(FINEWEB_LANGS) $(MADLAD_LANGS) $(STORYBOOK_LANGS) $(STORYWEAVER_LANGS) $(SMOL_LANGS) $(RESPELLED_LANGS) $(EBIBLE_LANGS) $(GLOTCC_LANGS) " in \
			*" $$lang "*) ;; \
			*) tw=$(TATOEBA_WEIGHT); case " $(TATOEBA_LIGHT_LANGS) " in \
				*" $$lang "*) tw=$(TATOEBA_LIGHT_WEIGHT) ;; \
			   esac; \
			   case " $(WIKIPEDIA_PLUS_LANGS) " in \
				*" $$lang "*) tw=$(TATOEBA_WIKI_WEIGHT) ;; \
			   esac; \
			   case " $(DAKSHINA_LANGS) $(FINEWEB_PLUS_LANGS) $(WIKIPEDIA_PLUS_LANGS) " in \
				*" $$lang "*) corpus="--corpus $$tw:$(CURDIR)/$(PREDICT_CORPORA_DIR)/$$lang.txt"; \
				              weight="--recipe tatoeba_weight=$$tw" ;; \
				*) corpus="--corpus $(CURDIR)/$(PREDICT_CORPORA_DIR)/$$lang.txt" ;; \
			   esac; \
			   corpus_in="--input corpora/$$lang.txt=$(PREDICT_CORPORA_DIR)/$$lang.txt" ;; \
		esac; \
		case " $(DAKSHINA_LANGS) " in \
			*" $$lang "*) corpus="$$corpus --corpus $(DAKSHINA_DIR)/$$lang.txt"; \
			              corpus_in="$$corpus_in --input dakshina/$$lang.txt=$(DAKSHINA_DIR)/$$lang.txt" ;; \
		esac; \
		case " $(WIKIPEDIA_LANGS) $(WIKIPEDIA_PLUS_LANGS) " in \
			*" $$lang "*) corpus="$$corpus --corpus $(WIKIPEDIA_DIR)/$$lang.txt"; \
			              corpus_in="$$corpus_in --input wikipedia/$$lang.txt=$(WIKIPEDIA_DIR)/$$lang.txt" ;; \
		esac; \
		fw=; case " $(WIKIPEDIA_LANGS) $(EBIBLE_LANGS) " in \
			*" $$lang "*) case " $(FINEWEB_LANGS) $(FINEWEB_PLUS_LANGS) $(GLOTCC_LANGS) " in \
				*" $$lang "*) ww=$(WEB_WEIGHT); case " $(WEB_EVEN_LANGS) " in *" $$lang "*) ww=1 ;; esac; \
				              fw="$$ww:"; weight="$$weight --recipe web_weight=$$ww" ;; \
			   esac ;; \
		esac; \
		case " $(FINEWEB_LANGS) $(FINEWEB_PLUS_LANGS) " in \
			*" $$lang "*) corpus="$$corpus --corpus $$fw$(FINEWEB_DIR)/$$lang.txt"; \
			              corpus_in="$$corpus_in --input fineweb/$$lang.txt=$(FINEWEB_DIR)/$$lang.txt" ;; \
		esac; \
		case " $(MADLAD_LANGS) " in \
			*" $$lang "*) corpus="$$corpus --corpus $(MADLAD_DIR)/$$lang.txt"; \
			              corpus_in="$$corpus_in --input madlad/$$lang.txt=$(MADLAD_DIR)/$$lang.txt" ;; \
		esac; \
		case " $(STORYBOOK_LANGS) " in \
			*" $$lang "*) corpus="$$corpus --corpus $(STORYBOOK_DIR)/$$lang.txt"; \
			              corpus_in="$$corpus_in --input storybooks/$$lang.txt=$(STORYBOOK_DIR)/$$lang.txt" ;; \
		esac; \
		case " $(STORYWEAVER_LANGS) " in \
			*" $$lang "*) if [ ! -f $(STORYWEAVER_DIR)/$$lang.txt ]; then \
			                  echo "predict $$lang: keeps its model (no StoryWeaver stories: fetch-storyweaver needs an account)"; \
			                  continue; \
			              fi; \
			              corpus="$$corpus --corpus $(STORYWEAVER_DIR)/$$lang.txt"; \
			              corpus_in="$$corpus_in --input storyweaver/$$lang.txt=$(STORYWEAVER_DIR)/$$lang.txt" ;; \
		esac; \
		case " $(SMOL_LANGS) $(RESPELLED_LANGS) " in \
			*" $$lang "*) corpus="$$corpus --corpus $(SMOL_DIR)/$$lang.txt"; \
			              corpus_in="$$corpus_in --input smol/$$lang.txt=$(SMOL_DIR)/$$lang.txt" ;; \
		esac; \
		case " $(RESPELLED_LANGS) " in \
			*" $$lang "*) corpus_in="$$corpus_in --input translit/$$lang.txt=translit/$$lang.txt" ;; \
		esac; \
		case " $(EBIBLE_LANGS) " in \
			*" $$lang "*) if [ ! -f $(EBIBLE_DIR)/$$lang.txt ]; then \
			                  echo "predict $$lang: keeps its model (its Bible not read: fetch-ebible)"; \
			                  continue; \
			              fi; \
			              corpus="$$corpus --corpus $(EBIBLE_DIR)/$$lang.txt"; \
			              corpus_in="$$corpus_in --input ebible/$$lang.txt=$(EBIBLE_DIR)/$$lang.txt" ;; \
		esac; \
		case " $(GLOTCC_LANGS) " in \
			*" $$lang "*) corpus="$$corpus --corpus $$fw$(GLOTCC_DIR)/$$lang.txt"; \
			              corpus_in="$$corpus_in --input glotcc/$$lang.txt=$(GLOTCC_DIR)/$$lang.txt" ;; \
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
			--recipe words=$$(words_gen $$lang) \
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
			--recipe words=$$(words_gen $$lang) \
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
	case " $(PREDICT_WIDE_LANGS) " in \
		*" $$lang "*) vocab=$(PREDICT_WIDE_VOCAB); ngram=$(PREDICT_WIDE_NGRAM) ;; \
	esac; \
	case " $(PREDICT_WIDER_LANGS) " in \
		*" $$lang "*) vocab=$(PREDICT_WIDER_VOCAB); ngram=$(PREDICT_WIDER_NGRAM) ;; \
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
# Latin side is eval/hi-Latn.txt; build-predict then counts Tatoeba's
# HINGLISH_TATOEBA_WEIGHT times: ten, as Hindi's were (at 30, as Hindi's
# are now, its savings on those sentences fall a point, 31.8% -> 30.8%,
# they being Wikipedia's). Its inputs are the corpora, the lexicons and
# the script. After fetch-corpora and fetch-dakshina.
HINGLISH_DIR = $(DAKSHINA_DIR)/hinglish
HINGLISH_TATOEBA_WEIGHT ?= 10
# Spelled many ways, Hinglish's words are many: from 30 000 to 60 000, the
# share of the held-out words its lexicon lacks falls from 16.2% to 13.4%.
HINGLISH_VOCAB ?= 60000
HINGLISH_NGRAM ?= 60000
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
		--recipe tatoeba_weight=$(HINGLISH_TATOEBA_WEIGHT) \
		--recipe vocab_size=$(HINGLISH_VOCAB) --recipe ngram_keep=$(HINGLISH_NGRAM) \
		--recipe start_keep=$(PREDICT_START) --recipe words=$$(words_gen hi-Latn) \
		--output $(PREDICT_OUT_DIR)/hi-Latn.fst --output $(PREDICT_OUT_DIR)/hi-Latn.ngram.bin \
		--output $(PREDICT_OUT_DIR)/hi-Latn.forms.fst; \
	stale "$$@" || exit 0; \
	mkdir -p $(HINGLISH_DIR) && \
	$(PYTHON) scripts/build-hinglish.py --lexicons $$d/lexicons \
		$(PREDICT_CORPORA_DIR)/hi.txt $(HINGLISH_DIR)/tatoeba.txt && \
	$(PYTHON) scripts/build-hinglish.py --lexicons $$d/lexicons \
		--drop $$d/romanized/hi.romanized.rejoined.test.native.txt \
		$(DAKSHINA_DIR)/hi.txt $(HINGLISH_DIR)/wikipedia.txt && \
	echo "build-predict hi-Latn (vocab $(HINGLISH_VOCAB), n-grams $(HINGLISH_NGRAM))" && \
	$(TOOLS) build-predict -- --lang hi-Latn \
		--corpus $(HINGLISH_TATOEBA_WEIGHT):$(HINGLISH_DIR)/tatoeba.txt --corpus $(HINGLISH_DIR)/wikipedia.txt \
		--out $(CURDIR)/$(PREDICT_OUT_DIR) --vocab-size $(HINGLISH_VOCAB) \
		--ngram-keep $(HINGLISH_NGRAM) --start-keep $(PREDICT_START) && \
	record "$$@"

# The English words each language with a transliteration table writes in
# its script, which people type as English spells them
# (translit/<lang>.english.txt, osk_data::translit::Scheme::with_english):
# CMUdict's pronunciations spelled in Latin letters as the language's
# sounds are typed, read by its table, the likeliest reading its lexicon
# has kept (crates/osk-data-tools/src/bin/build_english.rs); a word
# English's lexicon lacks (a name, or a word of another language CMUdict
# says too: hain, yah) is left out where the table reads it as a word of
# the language already. Made anew where CMUdict, the table, the lexicons
# or the tool changed, so after predict-assets; after `make fetch-cmudict`.
# Not Greek's, Persian's, Armenian's, Mongolian's, Santali's or Yiddish's:
# their measures (Greeklish, Wiktionary's headwords) hold no English words
# to gain on, and lost up to 0.8 points to the readings they added; nor
# Lao's or Garo's in the Bengali script, which nothing measures. Nor
# Vai's or Nuosu's, which have no lexicon to keep a reading.
ENGLISH_NOT_LANGS ?= el fa hy mn sat yi vai ii lo grt-Beng
ENGLISH_LANGS ?= $(filter-out $(ENGLISH_NOT_LANGS),$(sort $(patsubst translit/%.txt,%,$(filter-out %.english.txt,$(wildcard translit/*.txt)))))
.PHONY: english-assets
english-assets:
	@$(LOCKED); test -f $(CMUDICT_DIR)/cmudict.dict || { echo "no $(CMUDICT_DIR)/cmudict.dict: run make fetch-cmudict"; exit 1; }; \
	for lang in $(ENGLISH_LANGS); do \
		set -- english/$$lang \
			--input cmudict.dict=$(CMUDICT_DIR)/cmudict.dict \
			--input translit/$$lang.txt=translit/$$lang.txt \
			--input predict/$$lang.fst=$(PREDICT_OUT_DIR)/$$lang.fst \
			--input predict/en.fst=$(PREDICT_OUT_DIR)/en.fst \
			--input build_english.rs=crates/osk-data-tools/src/bin/build_english.rs \
			--output translit/$$lang.english.txt; \
		stale "$$@" || continue; \
		$(TOOLS) build-english -- --table translit/$$lang.txt \
			--lexicon $(PREDICT_OUT_DIR)/$$lang.fst --cmudict $(CMUDICT_DIR)/cmudict.dict \
			--english-words $(PREDICT_OUT_DIR)/en.fst --out translit/$$lang.english.txt \
			--source "The English words $$lang writes in its script, as its table reads them (make english-assets): from CMUdict (cmusphinx/cmudict; its notice in NOTICES.md) and predict/$$lang.fst." && \
		record "$$@" || exit 1; \
	done

# How often each transliteration table (translit/<lang>.txt), with its
# language's lexicon, puts the word meant first, as the composer's strip
# offers them: on the test half of Dakshina's romanized Wikipedia
# sentences (every word once; scripts/dakshina-halves.py cuts the halves)
# and on its romanization lexicon's test words (each romanization as often
# as it was typed); and the keystrokes the strip saves, typing the test
# sentences' first 20 000 words a letter at a time, each sentence's words
# ranked after the words before them with the language's n-grams, as the
# keyboard ranks them. After `make fetch-dakshina`; the tables' costs are
# fitted on what these leave out (fit-translit, below). Then FIT_BORROWED's
# tables, on the related language's test sentences and words, of the words
# the two share. Then the tables Dakshina has nothing for, on Aksharantar's
# words romanized by hand (mostly rare words and names, so what the lexicon
# has is the measure): after `make fetch-aksharantar`. And Greek's, on
# people's Greeklish: after `make fetch-greeklish`. And Yiddish's, on
# Wiktionary's words and their romanizations, a reading right when it is
# the word but for its points (eval-translit --unpointed): after `make
# fetch-wiktionary-romanized`. Each table is read with the English words
# its language writes, where it has them (translit/<lang>.english.txt,
# english-assets).
TRANSLIT_EVAL_LANGS ?= bn gu hi kn ml mr pa sd si ta te ur
DAKSHINA_TEST = romanized/$$lang.romanized.rejoined.aligned.cased_nopunct.test.tsv
TRANSLIT_ENGLISH = $$(test -f translit/$$lang.english.txt && echo --english translit/$$lang.english.txt)
.PHONY: eval-translit
eval-translit:
	@for lang in $(TRANSLIT_EVAL_LANGS); do \
		d=$(DAKSHINA_DIR)/$$lang; \
		test -d $$d || { echo "no $$d: run make fetch-dakshina"; exit 1; }; \
		test -f $$d/$(DAKSHINA_TEST) || $(PYTHON) scripts/dakshina-halves.py $(DAKSHINA_DIR) $$lang || exit 1; \
		echo "$$lang, romanized sentences (test):"; \
		$(TOOLS) eval-translit -- --table translit/$$lang.txt $(TRANSLIT_ENGLISH) --lexicon predict/$$lang.fst \
			--ngrams predict/$$lang.ngram.bin --pairs $$d/$(DAKSHINA_TEST) | tail -n 1 || exit 1; \
		echo "$$lang, romanization lexicon (test):"; \
		$(TOOLS) eval-translit -- --table translit/$$lang.txt $(TRANSLIT_ENGLISH) --lexicon predict/$$lang.fst \
			--pairs $$d/lexicons/$$lang.translit.sampled.test.tsv | tail -n 1 || exit 1; \
		echo "$$lang, typed a letter at a time:"; \
		$(TOOLS) eval-translit -- --table translit/$$lang.txt $(TRANSLIT_ENGLISH) --lexicon predict/$$lang.fst \
			--ngrams predict/$$lang.ngram.bin --pairs $$d/$(DAKSHINA_TEST) \
			--keystrokes --limit 20000 | tail -n 1 || exit 1; \
	done
	@for pair in $(FIT_BORROWED); do \
		lang=$${pair%%:*}; from=$${pair##*:}; d=$(DAKSHINA_DIR)/$$from; \
		test -d $$d || { echo "no $$d: run make fetch-dakshina"; exit 1; }; \
		echo "$$lang, $$from's romanized sentences (test), the words they share:"; \
		$(TOOLS) eval-translit -- --known --table translit/$$lang.txt $(TRANSLIT_ENGLISH) --lang $$lang \
			--lexicon predict/$$lang.fst --ngrams predict/$$lang.ngram.bin \
			--pairs $$d/romanized/$$from.romanized.rejoined.aligned.cased_nopunct.test.tsv \
			| tail -n 1 || exit 1; \
		echo "$$lang, $$from's romanization lexicon (test), the words they share:"; \
		$(TOOLS) eval-translit -- --known --table translit/$$lang.txt $(TRANSLIT_ENGLISH) --lang $$lang \
			--lexicon predict/$$lang.fst --pairs $$d/lexicons/$$from.translit.sampled.test.tsv \
			| tail -n 1 || exit 1; \
	done
	@for lang in $(AKSHARANTAR_LANGS) $(AKSHARANTAR_RESPELLED); do \
		p=$(AKSHARANTAR_DIR)/$$lang.tsv; \
		test -f $$p || { echo "no $$p: run make fetch-aksharantar (and aksharantar-respelled)"; exit 1; }; \
		echo "$$lang, Aksharantar's words:"; \
		$(TOOLS) eval-translit -- --table translit/$$lang.txt $(TRANSLIT_ENGLISH) --lexicon predict/$$lang.fst \
			--pairs $$p | tail -n 1 || exit 1; \
	done
	@lang=el; p=$(GREEKLISH_DIR)/el.tsv; \
	test -f $$p || { echo "no $$p: run make fetch-greeklish"; exit 1; }; \
	echo "el, AUEB's Greeklish:"; \
	$(TOOLS) eval-translit -- --table translit/el.txt $(TRANSLIT_ENGLISH) --lexicon predict/el.fst \
		--pairs $$p | tail -n 1
	@for lang in $(WIKTIONARY_ROMANIZED_LANGS); do \
		p=$(WIKTIONARY_ROMANIZED_DIR)/$$lang.tsv; \
		test -f $$p || { echo "no $$p: run make fetch-wiktionary-romanized"; exit 1; }; \
		case $$lang in yi) u=--unpointed; but=", the words but for their points" ;; *) u=; but= ;; esac; \
		echo "$$lang, Wiktionary's romanized words$$but:"; \
		$(TOOLS) eval-translit -- $$u --table translit/$$lang.txt $(TRANSLIT_ENGLISH) \
			--lexicon predict/$$lang.fst --pairs $$p | tail -n 1 || exit 1; \
	done

# The costs of the Dakshina languages' tables (each way's after its
# colon, and an abugida's cluster, schwa, final and final-virama), fitted
# to the romanizations of its lexicons' training words, each held to the
# table's own (crates/osk-data-tools/src/bin/fit_translit.rs); of the
# rounds (the table's own costs the first), the one that puts the most
# of its dev words and dev sentences' words first is written into
# translit/<lang>.txt. Then the Aksharantar languages' tables, fitted to
# four fifths of the words its train and valid splits romanized by hand
# and of Wikidata's names, chosen by the other fifth. Then the tables of
# languages neither has, each fitted to a related language's Dakshina
# romanizations of the words the two share (FIT_BORROWED, lang:from;
# fit-translit --known). eval-translit's test sentences and words are
# left out. After `make fetch-dakshina` and `make fetch-aksharantar`. A
# table fitted before starts from its fitted costs.
FIT_TRANSLIT_LANGS ?= $(DAKSHINA_LANGS)
FIT_AKSHARANTAR_LANGS ?= $(AKSHARANTAR_LANGS) $(AKSHARANTAR_RESPELLED)
FIT_BORROWED ?= bho:hi awa:hi mag:hi hne:hi sd-Deva:hi gbm:hi raj:hi anp:hi dty:hi new:hi pnb:ur skr:ur brh:ur khw:ur tcy:kn wsg:te \
	ahr:mr bfy:hi bgq:hi bns:hi hlb:hi kfq:hi kfy:hi kru:hi mjl:hi noe:hi sjp:hi rkt:bn bpy:bn
DAKSHINA_DEV = romanized/$$lang.romanized.rejoined.aligned.cased_nopunct.dev.tsv
.PHONY: fit-translit
fit-translit:
	@for lang in $(FIT_TRANSLIT_LANGS); do \
		d=$(DAKSHINA_DIR)/$$lang; \
		test -d $$d || { echo "no $$d: run make fetch-dakshina"; exit 1; }; \
		test -f $$d/$(DAKSHINA_DEV) || $(PYTHON) scripts/dakshina-halves.py $(DAKSHINA_DIR) $$lang || exit 1; \
		$(TOOLS) fit-translit -- --table translit/$$lang.txt --lexicon predict/$$lang.fst \
			--train $$d/lexicons/$$lang.translit.sampled.train.tsv \
			--dev $$d/lexicons/$$lang.translit.sampled.dev.tsv --dev $$d/$(DAKSHINA_DEV) \
			--write || exit 1; \
	done
	@for lang in $(FIT_AKSHARANTAR_LANGS); do \
		p=$(AKSHARANTAR_DIR)/$$lang; \
		test -f $$p.fit.tsv || { echo "no $$p.fit.tsv: run make fetch-aksharantar (and aksharantar-respelled)"; exit 1; }; \
		$(TOOLS) fit-translit -- --table translit/$$lang.txt --lexicon predict/$$lang.fst \
			--train $$p.fit.tsv --dev $$p.choose.tsv --write || exit 1; \
	done
	@for pair in $(FIT_BORROWED); do \
		lang=$${pair%%:*}; from=$${pair##*:}; d=$(DAKSHINA_DIR)/$$from; \
		test -d $$d || { echo "no $$d: run make fetch-dakshina"; exit 1; }; \
		test -f $$d/romanized/$$from.romanized.rejoined.aligned.cased_nopunct.dev.tsv || \
			$(PYTHON) scripts/dakshina-halves.py $(DAKSHINA_DIR) $$from || exit 1; \
		$(TOOLS) fit-translit -- --known --table translit/$$lang.txt --lang $$lang \
			--lexicon predict/$$lang.fst --train $$d/lexicons/$$from.translit.sampled.train.tsv \
			--dev $$d/lexicons/$$from.translit.sampled.dev.tsv \
			--dev $$d/romanized/$$from.romanized.rejoined.aligned.cased_nopunct.dev.tsv \
			--write || exit 1; \
	done

# How much a learned ranking of the readings would do
# (crates/osk-data-tools/src/bin/fit_ranker.rs, osk_data::translit::Ranker:
# a weighed sum of each reading's likelihood, cost, context, ... and a few
# tanh units of them, held to the default): for each Dakshina language,
# how often the word meant is a reading at all, how often the default
# ranking puts it first, and how often a ranker learned from its training
# words and its even dev sentences does, chosen by its dev words and odd
# dev sentences. Nothing is written (fit-ranker --write would put it in
# the table's [ranker] section). Measured 2026-11-15: the default ranking
# was within 0.2 points of the learned one on those, and on the test
# sentences the learned one lost keystrokes (Urdu 24.0% -> 23.3%), so no
# table has one: of the words meant, 6% to 14% of those the lexicons have
# are no reading at all, which a ranker cannot help, and the readings'
# ranking misses only 2.5 to 7 points more, mostly homophones (Urdu's wa:
# و or وہ). After `make fetch-dakshina`.
FIT_RANKER_LANGS ?= $(DAKSHINA_LANGS)
.PHONY: fit-ranker
fit-ranker:
	@for lang in $(FIT_RANKER_LANGS); do \
		d=$(DAKSHINA_DIR)/$$lang; \
		test -d $$d || { echo "no $$d: run make fetch-dakshina"; exit 1; }; \
		test -f $$d/$(DAKSHINA_DEV) || $(PYTHON) scripts/dakshina-halves.py $(DAKSHINA_DIR) $$lang || exit 1; \
		$(TOOLS) fit-ranker -- --table translit/$$lang.txt --lexicon predict/$$lang.fst \
			--ngrams predict/$$lang.ngram.bin --train $$d/lexicons/$$lang.translit.sampled.train.tsv \
			--dev $$d/lexicons/$$lang.translit.sampled.dev.tsv --halve $$d/$(DAKSHINA_DEV) || exit 1; \
	done

# The transliteration layouts (layouts/<lang>-abc.json), one for each
# table in translit/: the US keys typing into the language's composer.
.PHONY: translit-layouts
translit-layouts:
	$(PYTHON) scripts/build-translit-layouts.py

# The layouts that are another language's keys under its own name
# (layouts/bho.json, mai.json and sa.json are Hindi's, pnb.json Urdu's
# and ݨ, yi.json Hebrew's and Yiddish's pointed letters).
.PHONY: derived-layouts
derived-layouts:
	$(PYTHON) scripts/build-derived-layouts.py

# The layouts made from Keyman's touch keyboards, for the languages
# xkeyboard-config has no keymap of (layouts/ksw.json, Sgaw Karen's;
# ojb.json, Northwestern Ojibwe's; thirteen of a script of their own,
# ccp.json to osa.json; hac.json to nod.json, Gorani's to Northern
# Thai's; and und-fonipa.json, the IPA's, on the US keys): the
# keyboards' files fetched at a pinned commit, sha256-checked.
.PHONY: keyman-layouts
keyman-layouts:
	$(PYTHON) scripts/build-keyman-layouts.py

# Vai's and Nuosu's keyboards, whose Keyman keyboards type each syllable
# as Latin letters: their transliteration tables (translit/vai.txt,
# translit/ii.txt), what each syllable is typed as, found by running the
# keyboards' rules, and the layouts that type through them
# (layouts/vai.json, ii.json); the files fetched at a pinned commit,
# sha256-checked.
.PHONY: keyman-composers
keyman-composers:
	$(PYTHON) scripts/build-keyman-composers.py

# The layouts of languages no text here predicts, their letters from
# CLDR's exemplar characters (layouts/nv.json, Navajo's): each language's
# CLDR file fetched at the release-49 commit, sha256-checked.
.PHONY: exemplar-layouts
exemplar-layouts:
	$(PYTHON) scripts/build-exemplar-layouts.py

.PHONY: check
check:
	$(PYTHON) scripts/build-translit-layouts.py --check
	$(PYTHON) scripts/build-derived-layouts.py --check
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
# (packs.json, each pack's files in one archive, pack.<id>.tar.gz, and
# NOTICES.md), the all-languages bundle ki-osk-lang-$(VERSION).tar.xz and
# SHA256SUMS. The bundle is every pack installed from those assets as a
# client installs them:
# `tar -xJf ki-osk-lang-$(VERSION).tar.xz -C /usr/share/ki-osk`. The same
# commit gives the same bytes; the bundle's entries carry its time. GitHub
# keeps at most RELEASE_MAX_ASSETS in a release, and refuses the upload
# of one more, so a release with more is refused here, before any upload
# (a refresh adds its corpora, one asset more).
RELEASE_OUT       ?= dist/release
RELEASE_MAX_ASSETS = 1000
SOURCE_DATE_EPOCH ?= $(shell git log -1 --format=%ct 2>/dev/null || echo 0)
.PHONY: release
release:
	$(TOOLS) make-packs -- --out $(RELEASE_OUT) --bundle --mtime $(SOURCE_DATE_EPOCH)
	xz -9 -T1 -f $(RELEASE_OUT)/ki-osk-lang-$(VERSION).tar
	@$(MAKE) -s release-sums
	@n=$$(ls $(RELEASE_OUT) | wc -l); \
	if [ $$((n + 1)) -gt $(RELEASE_MAX_ASSETS) ]; then \
		echo "release v$(VERSION): $$n assets, and a refresh's corpora one more:" \
		     "GitHub keeps at most $(RELEASE_MAX_ASSETS) in a release"; exit 1; \
	fi; \
	echo "release v$(VERSION): $$n assets in $(RELEASE_OUT)"

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
