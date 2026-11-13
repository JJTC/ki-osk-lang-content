//! Every data file this repository commits decodes as its reader expects.
//!
//! The format guard: a decoder that disagrees with its encoder does not
//! crash — a loader returns `None` and the language quietly has no
//! suggestions. This walks the committed tree, not a sample, and
//! an unrecognised `.bin` fails rather than going unchecked.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use osk_data::cedict::CedictTable;
use osk_data::codec::decode;
use osk_data::kana::{ConnectionBlob, KanjiTable};
use osk_data::ngram::NgramModel;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn files(dir: &str, ext: &str) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = std::fs::read_dir(root().join(dir))
        .unwrap_or_else(|e| panic!("{dir}/: {e}"))
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.to_string_lossy().ends_with(ext))
        .collect();
    out.sort();
    out
}

fn name(p: &Path) -> String {
    p.strip_prefix(root()).unwrap_or(p).display().to_string()
}

#[test]
fn every_prediction_pair_decodes() {
    let (forms, lexicons): (Vec<PathBuf>, Vec<PathBuf>) = files("predict", ".fst")
        .into_iter()
        .partition(|p| p.to_string_lossy().ends_with(".forms.fst"));
    let tables = files("predict", ".ngram.bin");
    let langs = |v: &[PathBuf], suffix: &str| -> BTreeSet<String> {
        v.iter()
            .map(|p| {
                p.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .trim_end_matches(suffix)
                    .to_string()
            })
            .collect()
    };
    assert_eq!(
        langs(&lexicons, ".fst"),
        langs(&tables, ".ngram.bin"),
        "every language needs both its lexicon and its n-gram table"
    );
    assert_eq!(
        langs(&lexicons, ".fst"),
        langs(&forms, ".forms.fst"),
        "every language needs its usual forms, empty where its script has no case"
    );
    assert!(
        lexicons.len() >= 54,
        "expected the full set, found {}",
        lexicons.len()
    );

    for p in &lexicons {
        let bytes = std::fs::read(p).unwrap();
        let map = fst::Map::new(bytes).unwrap_or_else(|e| panic!("{}: {e}", name(p)));
        assert!(map.len() > 1000, "{}: only {} words", name(p), map.len());
    }
    for p in &forms {
        fst::Map::new(std::fs::read(p).unwrap()).unwrap_or_else(|e| panic!("{}: {e}", name(p)));
    }
    for p in &tables {
        let m = NgramModel::from_bytes(&std::fs::read(p).unwrap())
            .unwrap_or_else(|e| panic!("{}: {e}", name(p)));
        // A wrong configuration can decode to a plausible empty model
        // rather than an error.
        assert!(
            m.bigram_len() > 0 && m.trigram_len() > 0,
            "{}: decoded but empty",
            name(p)
        );
    }
}

#[test]
fn every_dialect_table_decodes_and_is_sound() {
    let tables = files("predict", ".dialects.bin");
    assert!(!tables.is_empty(), "no dialect table is committed");
    for p in &tables {
        let file = p.file_name().unwrap().to_string_lossy().to_string();
        let lang = file.trim_end_matches(".dialects.bin").to_string();
        let lexicon = root().join(format!("predict/{lang}.fst"));
        assert!(lexicon.is_file(), "{}: no lexicon beside it", name(p));
        let d = osk_data::dialects::Dialects::from_bytes(&std::fs::read(p).unwrap())
            .unwrap_or_else(|e| panic!("{}: {e}", name(p)));
        let problems = d.problems(&lang);
        assert!(problems.is_empty(), "{}: {problems:?}", name(p));
        assert!(
            !d.groups.is_empty(),
            "{}: no spellings across varieties",
            name(p)
        );
    }
}

#[test]
fn every_input_method_table_decodes() {
    for dir in ["pinyin", "zhuyin", "jyutping"] {
        let p = root().join(dir).join("cedict.bin");
        let t: CedictTable =
            decode(&std::fs::read(&p).unwrap()).unwrap_or_else(|e| panic!("{}: {e}", name(&p)));
        // The full bundles hold tens of thousands of readings; the seeds
        // that once replaced them by accident hold ~150.
        assert!(
            t.len() > 10_000,
            "{}: only {} readings — a seed?",
            name(&p),
            t.len()
        );
    }
    let p = root().join("kana/mozc-kanji.bin");
    let t: KanjiTable =
        decode(&std::fs::read(&p).unwrap()).unwrap_or_else(|e| panic!("{}: {e}", name(&p)));
    assert!(
        t.len() > 10_000,
        "{}: only {} readings — a seed?",
        name(&p),
        t.len()
    );

    let p = root().join("kana/mozc-connection.bin");
    let blob: ConnectionBlob =
        decode(&std::fs::read(&p).unwrap()).unwrap_or_else(|e| panic!("{}: {e}", name(&p)));
    assert!(
        blob.n > 0 && !blob.deflated.is_empty(),
        "{}: empty matrix",
        name(&p)
    );
}

#[test]
fn no_bin_goes_unchecked() {
    // A new kind of `.bin` must get a decode rule above, not slip past.
    let known = ["predict", "pinyin", "zhuyin", "jyutping", "kana"];
    for entry in std::fs::read_dir(root()).unwrap().flatten() {
        let p = entry.path();
        let dir = p.file_name().unwrap().to_string_lossy().to_string();
        let not_data = ["target", "crates", "dist", "seeds"];
        if !p.is_dir() || dir.starts_with('.') || not_data.contains(&dir.as_str()) {
            continue;
        }
        let has_bin = std::fs::read_dir(&p)
            .unwrap()
            .flatten()
            .any(|e| e.path().extension().is_some_and(|x| x == "bin"));
        assert!(
            !has_bin || known.contains(&dir.as_str()),
            "{dir}/ holds .bin files this test does not decode"
        );
    }
}

#[test]
fn the_emoji_catalogue_parses() {
    let p = root().join("emoji/catalogue.json");
    let json = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {e}", name(&p)));
    let entries = osk_data::emoji::parse(&json).unwrap_or_else(|e| panic!("{e}"));
    assert!(entries.len() >= 2000, "only {} emoji", entries.len());
}

#[test]
fn every_offensive_list_has_words() {
    let lists = files("offensive", ".txt");
    assert!(lists.len() >= 26, "only {} lists", lists.len());
    for p in lists {
        let text = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {e}", name(&p)));
        assert!(
            osk_data::offensive::words(&text).next().is_some(),
            "{}: no words",
            name(&p)
        );
    }
}

/// Each word list parses, belongs to a language written without spaces
/// between words, and cuts its held-out text into words of the list.
#[test]
fn every_word_list_cuts_its_language() {
    use std::sync::Arc;

    use osk_data::wordbreak::WordList;
    use osk_data::words::Splitter;

    let lists = files("wordbreak", ".txt");
    assert!(!lists.is_empty(), "no word list in wordbreak/");
    for p in lists {
        let lang = p.file_stem().unwrap().to_string_lossy().to_string();
        let text = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {e}", name(&p)));
        let list = Arc::new(WordList::parse(&text).unwrap_or_else(|e| panic!("{}: {e}", name(&p))));
        assert!(
            list.len() >= 10_000,
            "{}: only {} words",
            name(&p),
            list.len()
        );
        let splitter = Splitter::new(&lang);
        assert!(
            splitter.wants_word_list(),
            "{}: {lang} is written with spaces",
            name(&p)
        );
        assert!(
            root().join(format!("predict/{lang}.fst")).is_file(),
            "{}: no model for {lang}",
            name(&p)
        );
        let splitter = splitter.with_word_list(list.clone());
        let eval = std::fs::read_to_string(root().join(format!("eval/{lang}.txt")))
            .unwrap_or_else(|e| panic!("eval/{lang}.txt: {e}"));
        let (mut words, mut known) = (0usize, 0usize);
        for line in eval.lines() {
            for w in splitter.words(line) {
                words += 1;
                known += usize::from(list.contains(&w.text));
            }
        }
        // Names and loanwords are in no list; most words are.
        assert!(
            known * 10 >= words * 9,
            "{lang}: {known} of {words} words of eval/{lang}.txt in the list"
        );
    }
}

#[test]
fn thai_is_cut_as_written() {
    use std::sync::Arc;

    use osk_data::wordbreak::WordList;
    use osk_data::words::Splitter;

    let text = std::fs::read_to_string(root().join("wordbreak/th.txt")).unwrap();
    let list = WordList::parse(&text).unwrap();
    let splitter = Splitter::new("th").with_word_list(Arc::new(list));
    let words: Vec<String> = splitter
        .words("ผมต้องไปธนาคาร ฉันไปโรงเรียน")
        .into_iter()
        .map(|w| w.text)
        .collect();
    assert_eq!(words, ["ผม", "ต้อง", "ไป", "ธนาคาร", "ฉัน", "ไป", "โรงเรียน"]);
}

/// A language's lexicon, read as the keyboard reads it, for a table's
/// readings.
struct FstLexicon {
    map: fst::Map<Vec<u8>>,
    log_total: f32,
    unknown: f32,
}

impl FstLexicon {
    fn open(path: &Path) -> Self {
        let map = fst::Map::new(std::fs::read(path).unwrap()).unwrap();
        let mut stream = map.stream();
        let (mut total, mut rarest) = (0u64, u64::MAX);
        while let Some((_, v)) = fst::Streamer::next(&mut stream) {
            total += v;
            rarest = rarest.min(v.max(1));
        }
        #[allow(clippy::cast_precision_loss)]
        let log_total = (total.max(1) as f32).ln();
        #[allow(clippy::cast_precision_loss)]
        let unknown = (rarest as f32).ln() - log_total - osk_data::translit::UNKNOWN_BELOW_RAREST;
        Self {
            map,
            log_total,
            unknown,
        }
    }
}

impl osk_data::translit::Lexicon for FstLexicon {
    fn has_prefix(&self, prefix: &str) -> bool {
        use fst::{IntoStreamer, Streamer, automaton::Str};
        let aut = fst::Automaton::starts_with(Str::new(prefix));
        self.map.search(aut).into_stream().next().is_some()
    }
    fn log_prob(&self, word: &str) -> Option<f32> {
        #[allow(clippy::cast_precision_loss)]
        self.map
            .get(word)
            .map(|c| (c.max(1) as f32).ln() - self.log_total)
    }
    fn unknown_log_prob(&self) -> f32 {
        self.unknown
    }
}

/// Each transliteration table parses, has the layout that types through it,
/// and reads everyday words of its language first with its lexicon.
#[test]
fn every_translit_table_reads_its_language() {
    use osk_data::layout::{ImeKind, LayoutDef};
    use osk_data::translit::Scheme;

    let words: &[(&str, &[(&str, &str)])] = &[
        ("as", &[("moi", "মই"), ("tumi", "তুমি"), ("axom", "অসম")]),
        ("bn", &[("ami", "আমি"), ("tumi", "তুমি"), ("bhalo", "ভালো")]),
        (
            "el",
            &[
                ("kalimera", "καλημέρα"),
                ("nai", "ναι"),
                ("Ellada", "Ελλάδα"),
            ],
        ),
        ("fa", &[("salam", "سلام"), ("man", "من"), ("iran", "ایران")]),
        ("gu", &[("che", "છે"), ("gujarat", "ગુજરાત")]),
        (
            "hi",
            &[
                ("namaste", "नमस्ते"),
                ("kya", "क्या"),
                ("hai", "है"),
                ("bharat", "भारत"),
            ],
        ),
        ("hy", &[("barev", "բարեվ"), ("hayastan", "հայաստան")]),
        ("kn", &[("kannada", "ಕನ್ನಡ"), ("naanu", "ನಾನು")]),
        ("ml", &[("malayalam", "മലയാളം"), ("ente", "എന്റെ")]),
        ("mr", &[("aahe", "आहे"), ("maharashtra", "महाराष्ट्र")]),
        ("pa", &[("punjab", "ਪੰਜਾਬ"), ("vich", "ਵਿੱਚ")]),
        (
            "sd",
            &[
                ("sindh", "سنڌ"),
                ("pakistan", "پاڪستان"),
                ("karachi", "ڪراچي"),
            ],
        ),
        (
            "si",
            &[("mama", "මම"), ("sinhala", "සිංහල"), ("lankawa", "ලංකාව")],
        ),
        ("ta", &[("tamil", "தமிழ்"), ("enna", "என்ன")]),
        ("te", &[("telugu", "తెలుగు"), ("nenu", "నేను")]),
        ("ur", &[("kya", "کیا"), ("pakistan", "پاکستان")]),
        (
            "bho",
            &[("bhojpuri", "भोजपुरी"), ("ham", "हम"), ("bihar", "बिहार")],
        ),
        (
            "mai",
            &[("maithili", "मैथिली"), ("achhi", "अछि"), ("bhasha", "भाषा")],
        ),
        (
            "mn",
            &[
                ("mongol", "монгол"),
                ("ulaanbaatar", "улаанбаатар"),
                ("sain", "сайн"),
            ],
        ),
        (
            "ne",
            &[
                ("namaste", "नमस्ते"),
                ("nepal", "नेपाल"),
                ("ramro", "राम्रो"),
                ("timi", "तिमी"),
            ],
        ),
        ("or", &[("odia", "ଓଡ଼ିଆ"), ("bhala", "ଭଲ"), ("odisha", "ଓଡ଼ିଶା")]),
        (
            "pnb",
            &[("punjab", "پنجاب"), ("vich", "وچ"), ("pakistan", "پاکستان")],
        ),
        (
            "ps",
            &[
                ("kabul", "کابل"),
                ("afghanistan", "افغانستان"),
                ("pashto", "پښتو"),
            ],
        ),
        (
            "sa",
            &[
                ("samskritam", "संस्कृतम्"),
                ("asti", "अस्ति"),
                ("ramah", "रामः"),
                ("kritva", "कृत्वा"),
            ],
        ),
        (
            "sat",
            &[
                ("santari", "ᱥᱟᱱᱛᱟᱲᱤ"),
                ("menaka", "ᱢᱮᱱᱟᱜᱼᱟ"),
                ("tahen", "ᱛᱟᱦᱮᱸ"),
                ("lagit", "ᱞᱟᱹᱜᱤᱫ"),
            ],
        ),
        (
            "yi",
            &[
                ("yidish", "יידיש"),
                ("oyf", "אויף"),
                ("vos", "וואס"),
                ("sholem", "שלום"),
            ],
        ),
        ("awa", &[("aur", "अउर"), ("kay", "कय"), ("ekthu", "एक्ठु")]),
        (
            "kok",
            &[
                ("ani", "आनी"),
                ("asa", "आसा"),
                ("hem", "हें"),
                ("konkani", "कोंकणी"),
            ],
        ),
        ("ks", &[("manz", "مَنٛز"), ("teh", "تہٕ"), ("chhu", "چھُ")]),
        (
            "mni",
            &[
                ("amasung", "ꯑꯃꯁꯨꯡ"),
                ("masing", "ꯃꯁꯤꯡ"),
                ("henna", "ꯍꯦꯟꯅ"),
                ("manipur", "ꯃꯅꯤꯄꯨꯔ"),
            ],
        ),
        (
            "skr",
            &[("hik", "ہک"), ("saraiki", "سرائیکی"), ("ditta", "ݙتا")],
        ),
        (
            "tcy",
            &[("bokka", "ಬೊಕ್ಕ"), ("onji", "ಒಂಜಿ"), ("tulu", "ತುಳು")],
        ),
        (
            "brx",
            &[("aro", "आरो"), ("mwnse", "मोनसे"), ("thakhai", "थाखाय")],
        ),
        ("doi", &[("kanne", "कन्नै"), ("gi", "गी"), ("eh", "एह्")]),
        ("hne", &[("havay", "हवय"), ("au", "अउ"), ("rahis", "रहिस")]),
        ("mag", &[("aau", "आउ"), ("halai", "हलइ"), ("ego", "एगो")]),
        ("brh", &[("na", "نا"), ("kanning", "کننگ"), ("e", "ءِ")]),
        (
            "sd-Deva",
            &[("aahe", "आहे"), ("sindhi", "सिंधी"), ("galhi", "ॻाल्हि")],
        ),
        (
            "mni-Beng",
            &[
                ("amasung", "অমসুং"),
                ("leibak", "লৈবাক"),
                ("numit", "নুমিৎ"),
                ("waphom", "ৱাফম"),
            ],
        ),
    ];
    let tables = files("translit", ".txt");
    assert_eq!(
        tables.len(),
        words.len(),
        "a table without words to read here"
    );
    for p in tables {
        let lang = p.file_stem().unwrap().to_string_lossy().to_string();
        let text = std::fs::read_to_string(&p).unwrap();
        let scheme = Scheme::parse(&text).unwrap_or_else(|e| panic!("{}: {e}", name(&p)));
        let layout: LayoutDef = serde_json::from_str(
            &std::fs::read_to_string(
                root().join(format!("layouts/{}-abc.json", lang.to_lowercase())),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(layout.ime, Some(ImeKind::Translit), "{lang}-abc");
        assert_eq!(layout.prediction_lang.as_deref(), Some(lang.as_str()));
        let lexicon = FstLexicon::open(&root().join(format!("predict/{lang}.fst")));
        let (_, expected) = words.iter().find(|(l, _)| *l == lang).expect("words");
        for (latin, word) in *expected {
            let got = scheme.candidates(latin, &lexicon, 3);
            assert_eq!(
                got.first().map(|c| c.text.as_str()),
                Some(*word),
                "{lang}: {latin} offers {:?}",
                got.iter().map(|c| &c.text).collect::<Vec<_>>()
            );
        }
    }
}
