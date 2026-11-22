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

    use osk_data::wordbreak::Script;
    for p in &lexicons {
        let bytes = std::fs::read(p).unwrap();
        let map = fst::Map::new(bytes).unwrap_or_else(|e| panic!("{}: {e}", name(p)));
        // A language whose words are its script's syllables, each letter
        // one, has fewer: Nuosu's Yi has 1 165.
        let lang = p.file_stem().unwrap().to_string_lossy().to_string();
        let least = if Script::of_lang(&lang).is_some_and(Script::needs_no_list) {
            500
        } else {
            1000
        };
        assert!(map.len() > least, "{}: only {} words", name(p), map.len());
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

/// Lao written as it is written, for its word list: its held-out text is
/// FineWeb-2's, which is not committed.
const LAO: &[&str] = &[
    "ພາສາລາວເປັນພາສາທາງການຂອງປະເທດລາວ",
    "ຂ້ອຍຮັກເຈົ້າຫຼາຍ",
    "ມື້ນີ້ອາກາດດີ",
    "ລາວໄປໂຮງຮຽນທຸກມື້",
    "ຂອບໃຈຫຼາຍໆ",
    "ພວກເຮົາກິນເຂົ້າຢູ່ເຮືອນ",
    "ນະຄອນຫຼວງວຽງຈັນ",
];

/// Everyday Khmer and Burmese, as [`LAO`] is: their held-out text is the
/// web's too.
const KHMER: &[&str] = &[
    "ខ្ញុំស្រលាញ់ភាសាខ្មែរ",
    "សួស្តីអ្នកសុខសប្បាយជាទេ",
    "ថ្ងៃនេះអាកាសធាតុល្អ",
    "គាត់ទៅសាលារៀនរាល់ថ្ងៃ",
    "សូមអរគុណច្រើន",
    "យើងញ៉ាំបាយនៅផ្ទះ",
    "រាជធានីភ្នំពេញ",
];
const BURMESE: &[&str] = &[
    "မြန်မာဘာသာစကားကို ကျွန်တော် ချစ်တယ်",
    "မင်္ဂလာပါ နေကောင်းလား",
    "ဒီနေ့ ရာသီဥတု ကောင်းတယ်",
    "သူ နေ့တိုင်း ကျောင်းသွားတယ်",
    "ကျေးဇူးတင်ပါတယ်",
    "ကျွန်တော်တို့ အိမ်မှာ ထမင်းစားတယ်",
    "ရန်ကုန်မြို့",
];

/// Each word list parses, belongs to a language written without spaces
/// between words, and cuts its held-out text (Lao's, Khmer's and
/// Burmese's: [`LAO`], [`KHMER`], [`BURMESE`]) into words of the list. A
/// language cut into syllables (Shan, Mon, Sgaw Karen, Pa'O, Arakanese,
/// Eastern Pwo Karen) has a list of them, a thousand or more (Eastern Pwo
/// Karen's small wiki writes 1 522 twice), not tens of thousands, and cuts
/// its commonest run together back into them.
#[test]
fn every_word_list_cuts_its_language() {
    use std::sync::Arc;

    use osk_data::wordbreak::{Script, WordList};
    use osk_data::words::Splitter;

    let lists = files("wordbreak", ".txt");
    assert!(!lists.is_empty(), "no word list in wordbreak/");
    for p in lists {
        let lang = p.file_stem().unwrap().to_string_lossy().to_string();
        let text = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{}: {e}", name(&p)));
        let list = Arc::new(WordList::parse(&text).unwrap_or_else(|e| panic!("{}: {e}", name(&p))));
        let syllables = Script::of_lang(&lang).is_some_and(Script::cut_into_syllables);
        assert!(
            list.len() >= if syllables { 1_000 } else { 10_000 },
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
        if syllables {
            // Its 300 commonest syllables (the list's first lines), run
            // together, are cut back into them, each a word of the list.
            let common: Vec<&str> = text
                .lines()
                .filter(|l| !l.starts_with('#'))
                .take(300)
                .collect();
            let words: Vec<String> = splitter
                .words(&common.concat())
                .into_iter()
                .map(|w| w.text)
                .collect();
            assert_eq!(words, common, "{}", name(&p));
            continue;
        }
        let eval = match lang.as_str() {
            "lo" => LAO.join("\n"),
            "km" => KHMER.join("\n"),
            "my" => BURMESE.join("\n"),
            _ => std::fs::read_to_string(root().join(format!("eval/{lang}.txt")))
                .unwrap_or_else(|e| panic!("eval/{lang}.txt: {e}")),
        };
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

#[test]
fn lao_is_cut_as_written() {
    use std::sync::Arc;

    use osk_data::wordbreak::WordList;
    use osk_data::words::Splitter;

    let text = std::fs::read_to_string(root().join("wordbreak/lo.txt")).unwrap();
    let list = WordList::parse(&text).unwrap();
    let splitter = Splitter::new("lo").with_word_list(Arc::new(list));
    let cut = |s: &str| -> Vec<String> { splitter.words(s).into_iter().map(|w| w.text).collect() };
    assert_eq!(cut("ຂ້ອຍຮັກເຈົ້າຫຼາຍ"), ["ຂ້ອຍ", "ຮັກ", "ເຈົ້າ", "ຫຼາຍ"]);
    // The same words written the other way (ຫລ for ຫຼ; ໍາ for ຳ and ຫນ for
    // ໜ alike) are the same words.
    assert_eq!(cut("ຂ້ອຍຮັກເຈົ້າຫລາຍ"), cut("ຂ້ອຍຮັກເຈົ້າຫຼາຍ"));
    assert_eq!(
        splitter.normalize("ຫລາຍ"),
        splitter.normalize("ຫຼາຍ"),
        "one word, written two ways"
    );
    assert_eq!(splitter.normalize("ສໍາລັບ"), "ສຳລັບ");
}

#[test]
fn khmer_and_burmese_are_cut_as_written() {
    use std::sync::Arc;

    use osk_data::wordbreak::WordList;
    use osk_data::words::Splitter;

    let splitter = |lang: &str| {
        let text = std::fs::read_to_string(root().join(format!("wordbreak/{lang}.txt"))).unwrap();
        Splitter::new(lang).with_word_list(Arc::new(WordList::parse(&text).unwrap()))
    };
    let (km, my) = (splitter("km"), splitter("my"));
    let cut =
        |s: &Splitter, t: &str| -> Vec<String> { s.words(t).into_iter().map(|w| w.text).collect() };
    assert_eq!(cut(&km, "ខ្ញុំស្រលាញ់ភាសាខ្មែរ"), ["ខ្ញុំ", "ស្រលាញ់", "ភាសាខ្មែរ"]);
    // A zero-width space between words, as Khmer text often has, is no
    // part of either.
    assert_eq!(cut(&km, "ខ្ញុំ\u{200B}ស្រលាញ់"), ["ខ្ញុំ", "ស្រលាញ់"]);
    assert_eq!(cut(&my, "ကျေးဇူးတင်ပါတယ်"), ["ကျေးဇူးတင်", "ပါ", "တယ်"]);
    assert_eq!(cut(&my, "မင်္ဂလာပါ"), ["မင်္ဂလာ", "ပါ"]);
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
/// and reads everyday words of its language first with its lexicon, or
/// alone where no model predicts the language (Vai, Nuosu).
/// `scheme`, reading the English words `lang` writes, where it has them
/// (`translit/<lang>.english.txt`), as the keyboard reads them.
fn with_english(scheme: osk_data::translit::Scheme, lang: &str) -> osk_data::translit::Scheme {
    let p = root().join(format!("translit/{lang}.english.txt"));
    match std::fs::read_to_string(&p) {
        Ok(text) => scheme
            .with_english(&text)
            .unwrap_or_else(|e| panic!("{}: {e}", name(&p))),
        Err(_) => scheme,
    }
}

#[test]
fn every_english_word_list_reads_with_its_table() {
    use osk_data::translit::Scheme;

    // English words typed as English spells them, which the tables alone
    // do not read as the word.
    let words: &[(&str, &[(&str, &str)])] = &[
        (
            "hi",
            &[("college", "कॉलेज"), ("system", "सिस्टम"), ("railway", "रेलवे")],
        ),
        (
            "bn",
            &[
                ("college", "কলেজ"),
                ("police", "পুলিশ"),
                ("station", "স্টেশন"),
            ],
        ),
        (
            "ta",
            &[("college", "காலேஜ்"), ("police", "போலீஸ்"), ("team", "டீம்")],
        ),
        ("te", &[("college", "కాలేజ్"), ("station", "స్టేషన్")]),
        (
            "ur",
            &[("college", "کالج"), ("police", "پولیس"), ("system", "سسٹم")],
        ),
    ];
    let lists: Vec<_> = files("translit", ".english.txt");
    assert!(!lists.is_empty(), "no English words");
    for p in &lists {
        let file = p.file_name().unwrap().to_string_lossy().to_string();
        let lang = file.trim_end_matches(".english.txt").to_string();
        let table = root().join(format!("translit/{lang}.txt"));
        let scheme = Scheme::parse(&std::fs::read_to_string(&table).unwrap()).unwrap();
        let scheme = with_english(scheme, &lang);
        assert!(
            scheme.english_words() > 1000,
            "{}: few English words",
            name(p)
        );
    }
    for (lang, expected) in words {
        let table = root().join(format!("translit/{lang}.txt"));
        let scheme = Scheme::parse(&std::fs::read_to_string(&table).unwrap()).unwrap();
        let plain = scheme.clone();
        let scheme = with_english(scheme, lang);
        let lexicon = FstLexicon::open(&root().join(format!("predict/{lang}.fst")));
        for (latin, word) in *expected {
            // Among the strip's two: a language may write an English word
            // two ways (స్టేషను, స్టేషన్), its lexicon ranking them.
            let got = scheme.candidates(latin, &lexicon, 3);
            assert!(
                got.iter().take(2).any(|c| c.text == *word),
                "{lang}: {latin} offers {:?}",
                got.iter().map(|c| &c.text).collect::<Vec<_>>()
            );
            let alone = plain.candidates(latin, &lexicon, 1);
            assert_ne!(
                alone.first().map(|c| c.text.as_str()),
                Some(*word),
                "{lang}: {latin}"
            );
        }
    }
}

#[test]
fn every_translit_table_reads_its_language() {
    use osk_data::layout::{ImeKind, KeyOutput, LayoutDef, ViewId};
    use osk_data::translit::Scheme;

    let words: &[(&str, &[(&str, &str)])] = &[
        ("as", &[("moi", "মই"), ("tumi", "তুমি"), ("axom", "অসম")]),
        (
            "bn",
            &[
                ("ami", "আমি"),
                ("tumi", "তুমি"),
                ("bhalo", "ভালো"),
                ("onchole", "অঞ্চলে"),
                ("bangladesh-er", "বাংলাদেশের"),
            ],
        ),
        (
            "el",
            &[
                ("kalimera", "καλημέρα"),
                ("nai", "ναι"),
                ("Ellada", "Ελλάδα"),
                ("8elw", "θέλω"),
                ("k", "και"),
            ],
        ),
        (
            "fa",
            &[
                ("salam", "سلام"),
                ("man", "من"),
                ("iran", "ایران"),
                ("tashakkor", "تشکر"),
            ],
        ),
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
        (
            "pa",
            &[
                ("punjab", "ਪੰਜਾਬ"),
                ("vich", "ਵਿੱਚ"),
                ("gaya", "ਗਿਆ"),
                ("toh", "ਤੋਂ"),
                ("mukkh", "ਮੁੱਖ"),
            ],
        ),
        (
            "sd",
            &[
                ("sindh", "سنڌ"),
                ("pakistan", "پاڪستان"),
                ("karachi", "ڪراچي"),
                ("jahin", "جنهن"),
                ("pahinje", "پنهنجي"),
            ],
        ),
        (
            "si",
            &[("mama", "මම"), ("sinhala", "සිංහල"), ("lankawa", "ලංකාව")],
        ),
        ("ta", &[("tamil", "தமிழ்"), ("enna", "என்ன")]),
        ("te", &[("telugu", "తెలుగు"), ("nenu", "నేను")]),
        (
            "ur",
            &[
                ("kya", "کیا"),
                ("pakistan", "پاکستان"),
                ("taraqqi", "ترقی"),
                ("istemaal", "استعمال"),
                ("dono", "دونوں"),
            ],
        ),
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
                ("roghtya", "روغتیا"),
                ("gul", "ګل"),
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
                ("simkhes", "שמחות"),
                ("khsidem", "חסידים"),
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
        ("gbm", &[("ar", "अर"), ("ma", "मा"), ("garhwali", "गढ़वाली")]),
        (
            "raj",
            &[("ri", "री"), ("mharo", "म्हारो"), ("sagla", "सगळा")],
        ),
        (
            "wsg",
            &[("koytur", "కోయ్తూర్"), ("gond", "గోండ్"), ("manta", "మంత")],
        ),
        (
            "khw",
            &[
                ("tsiq", "څیق"),
                ("sher", "شیر"),
                ("oche", "اوچے"),
                ("khowar", "کھوار"),
            ],
        ),
        (
            "dty",
            &[("cha", "छ"), ("doteli", "डोटेली"), ("awasthami", "अवस्थामी")],
        ),
        ("anp", &[("chhai", "छै"), ("aaru", "आरू"), ("kerau", "केरौ")]),
        ("new", &[("khah", "खः"), ("thwa", "थ्व"), ("yahgu", "याःगु")]),
        // Read by the table alone: no model.
        (
            "vai",
            &[
                ("vai", "ꕙꔤ"),
                ("kɔ", "ꗛ"),
                ("kpɔ", "ꗅ"),
                ("hɛ", "ꗤ"),
                ("nga", "ꕋ"),
                ("ka~", "ꕫ"),
                ("x", "ꘋ"),
            ],
        ),
        (
            "ii",
            &[
                ("nuosuhxop", "ꆈꌠꉙ"),
                ("bit", "ꀖ"),
                ("bita", "ꀘꄣ"),
                ("w", "ꀕ"),
            ],
        ),
    ];
    let tables: Vec<_> = files("translit", ".txt")
        .into_iter()
        .filter(|p| !name(p).ends_with(".english.txt"))
        .collect();
    assert_eq!(
        tables.len(),
        words.len(),
        "a table without words to read here"
    );
    let layout = |id: &str| -> LayoutDef {
        serde_json::from_str(
            &std::fs::read_to_string(root().join(format!("layouts/{id}.json"))).unwrap(),
        )
        .unwrap()
    };
    for p in tables {
        let lang = p.file_stem().unwrap().to_string_lossy().to_string();
        let text = std::fs::read_to_string(&p).unwrap();
        let scheme = with_english(
            Scheme::parse(&text).unwrap_or_else(|e| panic!("{}: {e}", name(&p))),
            &lang,
        );
        let (_, expected) = words.iter().find(|(l, _)| *l == lang).expect("words");
        // A language whose own layout types through its table (Vai's,
        // Nuosu's): the table alone reads, and where a model predicts the
        // language (Vai's), its lexicon finds the same words first.
        let own = layout(&lang.to_lowercase());
        if own.ime == Some(ImeKind::Translit) {
            assert!(!own.prediction_enabled, "{lang}");
            // No Shift view: a capital is a letter of its own to Vai's
            // table (T is THA, t TA), and a field's first letter would be
            // one. Every Latin letter its keys type, the table reads (the
            // script's own, Vai's lengthener, are typed as they are).
            assert!(own.view(ViewId::Shift).is_none(), "{lang} has a Shift view");
            let base = own.view(ViewId::Base).expect("a base view");
            // What typing each character costs: a tap on its key, two for
            // a long press.
            let mut taps: std::collections::HashMap<char, u32> = std::collections::HashMap::new();
            for k in base.rows.iter().flat_map(|r| r.keys.iter()) {
                let outs =
                    std::iter::once((&k.output, 1)).chain(k.long_press.iter().map(|o| (o, 2)));
                for (out, cost) in outs {
                    if let KeyOutput::Text { text } = out {
                        for c in text.chars().filter(|c| c.is_lowercase()) {
                            assert!(scheme.reads(c), "{lang}: the table does not read {c}");
                        }
                        let mut chars = text.chars();
                        if let (Some(c), None) = (chars.next(), chars.next()) {
                            let t = taps.entry(c).or_insert(cost);
                            *t = (*t).min(cost);
                        }
                    }
                }
            }
            for (latin, word) in *expected {
                assert_eq!(scheme.literal(latin), *word, "{lang}: {latin}");
                // The layout's keys spell it (what eval-predict types it
                // as), in as few taps as the letters given.
                let spelled = scheme
                    .spelling(word, |c| taps.get(&c).copied())
                    .unwrap_or_else(|| panic!("{lang}: no keys spell {word}"));
                let cost = |s: &str| {
                    s.chars()
                        .map(|c| taps.get(&c).copied().unwrap_or(9))
                        .sum::<u32>()
                };
                assert!(
                    cost(&spelled) <= cost(latin),
                    "{lang}: {word} spelled {spelled}"
                );
            }
            let fst = root().join(format!("predict/{lang}.fst"));
            if fst.exists() {
                let lexicon = FstLexicon::open(&fst);
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
            continue;
        }
        let layout = layout(&format!("{}-abc", lang.to_lowercase()));
        assert_eq!(layout.ime, Some(ImeKind::Translit), "{lang}-abc");
        assert_eq!(layout.prediction_lang.as_deref(), Some(lang.as_str()));
        let lexicon = FstLexicon::open(&root().join(format!("predict/{lang}.fst")));
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
