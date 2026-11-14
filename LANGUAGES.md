# Supported languages & layouts

What the language packs hold per language: the keyboard layouts, text
direction, predictive-text tier, swipe typing, and composer (CJK,
Ge'ez, transliteration). Each
layout is a file, `layouts/<id>.json`, in its language's pack (the Pack
column below, from `packs.toml`'s `[layouts]`), beside that language's
prediction and composer data.

**223 layouts**: 171 keyboard layouts (RTL: ar he fa + Urdu, Western
Punjabi, Saraiki, Kashmiri, Brahui, Khowar, Pashto, Sindhi, Uyghur,
Syriac, Dhivehi, Yiddish; Hinglish, Kokborok) · 7 IMEs (5 CJK +
Amharic/Tigrinya Ge'ez) · 43 transliteration layouts (Latin letters →
the language's script, `-abc`) · emoji + numeric. All render native
keycaps. 222 are here; the numeric keypad is built into ki-osk. The
table below is the complete list; **96 languages** carry a prediction
model (suggestion bar, plus dictionary-backed swipe on the Latin ones),
the rest are layout-only, gated only by how much permissively-licensed
corpus exists (see [Prediction model tiers](#prediction-model-tiers)).

**A device has the layouts of the packs it installs** — `de` for German's,
`sw` for Swahili's — see [Choosing languages](#choosing-languages). Their
runtime fonts load lazily on first use.

Legend — **Prediction**: `Full` = bundled FST + n-gram model · `—` = none
(no suggestions) · `IME` = candidate bar.
**Swipe**: `✓` Latin + full dictionary · `~` Latin layout, gesture on
but no model to match the glide against · `—` off
(non-Latin scripts, IMEs, emoji/numeric).

| Layout (id) | Pack | Language | Keymap (xkb) | Dir | Prediction | Swipe¹ | IME / composer |
|---|---|---|---|---|---|---|---|
| `us` | `en` | English (US) | us | LTR | **Full** (en) | ✓ | — |
| `us-intl` | `en` | English (US, intl) | us · intl | LTR | **Full** (en) | ✓ | — |
| `gb` | `en` | English (UK) | gb | LTR | **Full** (en) | ✓ | — |
| `de` | `de` | German | de | LTR | **Full** (de) | ✓ | — |
| `es` | `es` | Spanish | es | LTR | **Full** (es) | ✓ | — |
| `fr-azerty` | `fr` | French | fr | LTR | **Full** (fr) | ✓ | — |
| `it` | `it` | Italian | it | LTR | **Full** (it) | ✓ | — |
| `nl` | `nl` | Dutch | nl | LTR | **Full** (nl) | ✓ | — |
| `pl` | `pl` | Polish | pl | LTR | **Full** (pl) | ✓ | — |
| `pt-br` | `pt` | Portuguese (BR) | br | LTR | **Full** (pt) | ✓ | — |
| `se` | `sv` | Swedish | se | LTR | **Full** (sv) | ✓ | — |
| `no` | `no` | Norwegian | no | LTR | **Full** (no) | ✓ | — |
| `dk` | `da` | Danish | dk | LTR | **Full** (da) | ✓ | — |
| `fi` | `fi` | Finnish | fi | LTR | **Full** (fi) | ✓ | — |
| `cz` | `cs` | Czech | cz | LTR | **Full** (cs) | ✓ | — |
| `tr` | `tr` | Turkish | tr | LTR | **Full** (tr) | ✓ | — |
| `vi` | `vi` | Vietnamese | vn | LTR | **Full** (vi) | ✓ | — |
| `el` | `el` | Greek | gr | LTR | **Full** (el) | — | — |
| `ru` | `ru` | Russian | ru | LTR | **Full** (ru) | — | — |
| `uk` | `uk` | Ukrainian | ua | LTR | **Full** (uk) | — | — |
| `th` | `th` | Thai | th | LTR | **Full** (th)² | — | — |
| `ar` | `ar` | Arabic | ara | **RTL** | **Full** (ar) | — | — |
| `he` | `he` | Hebrew | il | **RTL** | **Full** (he) | — | — |
| `yi` | `yi` | Yiddish ייִדיש | il + long-press אַ אָ בֿ וּ װ ױ יִ ײ ײַ כּ פּ פֿ שׂ תּ | **RTL** | **Full** (yi)⁶ | — | — |
| `fa` | `fa` | Persian (Farsi) | ir | **RTL** | **Full** (fa) | — | — |
| `pt` | `pt` | Portuguese (PT) | pt | LTR | **Full** (pt) | ✓ | — |
| `ro` | `ro` | Romanian | ro · std | LTR | **Full** (ro) | ✓ | — |
| `hu` | `hu` | Hungarian | hu | LTR | **Full** (hu) | ✓ | — |
| `sk` | `sk` | Slovak | sk | LTR | **Full** (sk) | ✓ | — |
| `sl` | `sl` | Slovenian | si | LTR | — | ~ | — |
| `hr` | `hr` | Croatian | hr | LTR | **Full** (hr) | ✓ | — |
| `bs` | `bs` | Bosnian | ba | LTR | — | ~ | — |
| `sr-latn` | `sr` | Serbian (Latin) | rs · latin | LTR | — | ~ | — |
| `et` | `et` | Estonian | ee | LTR | **Full** (et) | ✓ | — |
| `ga` | `ga` | Irish | ie | LTR | — | ~ | — |
| `mt` | `mt` | Maltese | mt | LTR | — | ~ | — |
| `is` | `is` | Icelandic | is | LTR | **Full** (is) | ✓ | — |
| `fo` | `fo` | Faroese | fo | LTR | — | ~ | — |
| `sq` | `sq` | Albanian | al | LTR | — | ~ | — |
| `ca` | `ca` | Catalan | es · cat | LTR | **Full** (ca) | ✓ | — |
| `eo` | `eo` | Esperanto | epo | LTR | **Full** (eo) | ✓ | — |
| `bg` | `bg` | Bulgarian | bg | LTR | **Full** (bg) | — | — |
| `sr` | `sr` | Serbian | rs | LTR | **Full** (sr) | — | — |
| `mk` | `mk` | Macedonian | mk | LTR | **Full** (mk) | — | — |
| `be` | `be` | Belarusian | by | LTR | **Full** (be) | — | — |
| `id` | `id` | Indonesian | id | LTR | **Full** (id) | ✓ | — |
| `fil` | `fil` | Filipino | ph | LTR | **Full** (fil) | ✓ | — |
| `az` | `az` | Azerbaijani | az | LTR | **Full** (az) | ✓ | — |
| `ku` | `ku` | Kurdish (Kurmanji) | iq · ku | LTR | **Full** (ku) | ✓ | — |
| `ur` | `ur` | Urdu | pk | **RTL** | **Full** (ur)³ | — | — |
| `brh` | `brh` | Brahui براہوئی | pk + long-press ۺ | **RTL** | **Full** (brh)⁷ | — | — |
| `khw` | `khw` | Khowar کھوار | pk + long-press ځ ݮ څ ݯ ݰ ݱ | **RTL** | **Full** (khw)⁶ | — | — |
| `pnb` | `pnb` | Western Punjabi پنجابی | pk + long-press ݨ | **RTL** | **Full** (pnb)⁶ | — | — |
| `skr` | `skr` | Saraiki سرائیکی | pk + long-press ٻ ڄ ݙ ڳ ݨ | **RTL** | **Full** (skr)⁶ | — | — |
| `ks` | `ks` | Kashmiri کٲشُر | pk + long-press ٲ ۄ ۆ ؠ ۍ and the vowel marks ٕ ٟ ٖ ٗ ٚ ٛ | **RTL** | **Full** (ks)⁶ | — | — |
| `ps` | `ps` | Pashto | af · ps | **RTL** | **Full** (ps)⁶ | — | — |
| `sd` | `sd` | Sindhi | pk · snd | **RTL** | **Full** (sd)³ | — | — |
| `sd-deva` | `sd-Deva` | Sindhi सिंधी (Devanagari) | in + long-press ॻ ॼ ॾ ॿ | LTR | **Full** (sd-Deva)⁷ | — | — |
| `ug` | `ug` | Uyghur | cn · ug | **RTL** | **Full** (ug) | — | — |
| `kk` | `kk` | Kazakh | kz | LTR | — | — | — |
| `ky` | `ky` | Kyrgyz | kg | LTR | — | — | — |
| `mn` | `mn` | Mongolian | mn | LTR | **Full** (mn)⁶ | — | — |
| `tg` | `tg` | Tajik | tj | LTR | — | — | — |
| `lt` | `lt` | Lithuanian | lt | LTR | **Full** (lt) | ✓ | — |
| `lv` | `lv` | Latvian | lv | LTR | **Full** (lv) | ✓ | — |
| `ka` | `ka` | Georgian | ge | LTR | **Full** (ka) | — | — |
| `hy` | `hy` | Armenian | am | LTR | **Full** (hy) | — | — |
| `hi` | `hi` | Hindi | in | LTR | **Full** (hi)³ | — | — |
| `brx` | `brx` | Bodo बड़ो | in | LTR | **Full** (brx)⁷ | — | — |
| `doi` | `doi` | Dogri डोगरी | in | LTR | **Full** (doi)⁷ | — | — |
| `hne` | `hne` | Chhattisgarhi छत्तीसगढ़ी | in | LTR | **Full** (hne)⁷ | — | — |
| `mag` | `mag` | Magahi मगही | in | LTR | **Full** (mag)⁶ ⁷ | — | — |
| `gbm` | `gbm` | Garhwali गढ़वाली | in (+ ळ long-press) | LTR | **Full** (gbm)⁸ | — | — |
| `raj` | `raj` | Rajasthani राजस्थानी | in (+ ळ long-press) | LTR | **Full** (raj)⁸ | — | — |
| `hi-latn` | `hi-Latn` | Hinglish (Hindi in Latin letters) | us | LTR | **Full** (hi-Latn)⁵ | ✓ | — |
| `trp` | `trp` | Kokborok | us | LTR | **Full** (trp)⁷ | ✓ | — |
| `lus` | `lus` | Mizo | us + long-press â ê î ô û ṭ | LTR | **Full** (lus)⁷ | ✓ | — |
| `kha` | `kha` | Khasi | us + long-press ï ñ | LTR | **Full** (kha)⁷ | ✓ | — |
| `bn` | `bn` | Bengali | in · ben | LTR | **Full** (bn)³ | — | — |
| `ta` | `ta` | Tamil | in · tam | LTR | **Full** (ta)³ | — | — |
| `te` | `te` | Telugu | in · tel | LTR | **Full** (te)³ | — | — |
| `kn` | `kn` | Kannada | in · kan | LTR | **Full** (kn)³ | — | — |
| `tcy` | `tcy` | Tulu ತುಳು | in · kan | LTR | **Full** (tcy)⁶ | — | — |
| `wsg` | `wsg` | Gondi గోండి (Adilabad) | in · tel | LTR | **Full** (wsg)⁶ | — | — |
| `ml` | `ml` | Malayalam | in · mal | LTR | **Full** (ml)³ | — | — |
| `gu` | `gu` | Gujarati | in · guj | LTR | **Full** (gu)³ | — | — |
| `pa` | `pa` | Punjabi | in · guru | LTR | **Full** (pa)³ | — | — |
| `or` | `or` | Odia | in · ori | LTR | **Full** (or)⁶ | — | — |
| `si` | `si` | Sinhala | lk · sin_phonetic | LTR | **Full** (si)³ | — | — |
| `km` | `km` | Khmer | kh | LTR | — | — | — |
| `lo` | `lo` | Lao | la | LTR | **Full** (lo)⁷ | — | — |
| `my` | `my` | Burmese | mm | LTR | — | — | — |
| `syr` | `syr` | Syriac | sy · syc | **RTL** | — | — | — |
| `ber` | `ber` | Tamazight | ma · tifinagh | LTR | — | — | — |
| `kab` | `kab` | Kabyle | us | LTR | **Full** (kab) | ✓ | — |
| `qu` | `qu` | Quechua | es | LTR | — | ~ | — |
| `gn` | `gn` | Guaraní | es | LTR | — | ~ | — |
| `ay` | `ay` | Aymara | es | LTR | — | ~ | — |
| `sw` | `sw` | Swahili | us | LTR | — | ~ | — |
| `ms` | `ms` | Malay | us | LTR | **Full** (ms) | ✓ | — |
| `so` | `so` | Somali | us | LTR | — | ~ | — |
| `zu` | `zu` | Zulu | us | LTR | — | ~ | — |
| `xh` | `xh` | Xhosa | us | LTR | — | ~ | — |
| `af` | `af` | Afrikaans | us | LTR | — | ~ | — |
| `jv` | `jv` | Javanese | us | LTR | — | ~ | — |
| `su` | `su` | Sundanese | us | LTR | — | ~ | — |
| `om` | `om` | Oromo | us | LTR | — | ~ | — |
| `ceb` | `ceb` | Cebuano | es | LTR | — | ~ | — |
| `ha` | `ha` | Hausa | us + long-press ɓ ɗ ƙ ƴ | LTR | — | ~ | — |
| `yo` | `yo` | Yoruba | us + long-press dot/tone | LTR | — | ~ | — |
| `ig` | `ig` | Igbo | us + long-press ị ọ ụ ṅ | LTR | — | ~ | — |
| `uz` | `uz` | Uzbek | us + long-press oʻ gʻ | LTR | — | ~ | — |
| `tk` | `tk` | Turkmen | us + long-press ä ç ž ň ö ş ü ý | LTR | — | ~ | — |
| `mr` | `mr` | Marathi | in · marathi | LTR | **Full** (mr)³ | — | — |
| `kok` | `kok` | Konkani कोंकणी | in · marathi | LTR | **Full** (kok)⁶ | — | — |
| `ne` | `ne` | Nepali | np | LTR | **Full** (ne)⁶ | — | — |
| `bho` | `bho` | Bhojpuri भोजपुरी | in | LTR | **Full** (bho)⁶ | — | — |
| `mai` | `mai` | Maithili मैथिली | in | LTR | **Full** (mai)⁶ | — | — |
| `sa` | `sa` | Sanskrit संस्कृतम् | in + long-press ऽ ॐ | LTR | **Full** (sa)⁶ | — | — |
| `awa` | `awa` | Awadhi अवधी | in | LTR | **Full** (awa)⁶ | — | — |
| `as` | `as` | Assamese | in · ben (+ ৰ ৱ long-press) | LTR | **Full** (as)⁷ | — | — |
| `bo` | `bo` | Tibetan བོད་ཡིག | cn · tib | LTR | — | — | — |
| `dv` | `dv` | Dhivehi ދިވެހި | mv | RTL | — | — | — |
| `sat` | `sat` | Santali ᱥᱟᱱᱛᱟᱲᱤ | in · sat | LTR | **Full** (sat)⁶ | — | — |
| `cy` | `cy` | Welsh Cymraeg | us + long-press ŵ ŷ â ê î ô û | LTR | — | ~ | — |
| `gd` | `gd` | Scottish Gaelic Gàidhlig | us | LTR | — | ~ | — |
| `eu` | `eu` | Basque Euskara | es | LTR | **Full** (eu) | ✓ | — |
| `gl` | `gl` | Galician Galego | es | LTR | **Full** (gl) | ✓ | — |
| `oc` | `oc` | Occitan | us + long-press ç à á è é í ï ò ó ú ü | LTR | **Full** (oc) | ✓ | — |
| `br` | `br` | Breton Brezhoneg | us + long-press ñ ç â ê î ô û ù | LTR | **Full** (br) | ✓ | — |
| `co` | `co` | Corsican Corsu | us | LTR | — | ~ | — |
| `lb` | `lb` | Luxembourgish Lëtzebuergesch | us + long-press ä ë é ï ö ü | LTR | — | ~ | — |
| `rm` | `rm` | Romansh Rumantsch | us | LTR | — | ~ | — |
| `fy` | `fy` | Frisian Frysk | us + long-press â ê ë î ï ô û ú | LTR | — | ~ | — |
| `tt` | `tt` | Tatar Татарча | ru · tt | LTR | **Full** (tt) | — | — |
| `ba` | `ba` | Bashkir Башҡортса | ru · bak | LTR | — | — | — |
| `os` | `os` | Ossetian Ирон | ru · os_winkeys | LTR | **Full** (os) | — | — |
| `udm` | `udm` | Udmurt Удмурт | ru · udm | LTR | — | — | — |
| `wo` | `wo` | Wolof | us + long-press ñ ŋ ë ó | LTR | — | ~ | — |
| `ln` | `ln` | Lingala Lingála | us + long-press ɛ ɔ | LTR | — | ~ | — |
| `rw` | `rw` | Kinyarwanda | us | LTR | — | ~ | — |
| `rn` | `rn` | Kirundi | us | LTR | — | ~ | — |
| `sn` | `sn` | Shona chiShona | us | LTR | — | ~ | — |
| `ny` | `ny` | Chichewa | us + long-press ŵ | LTR | — | ~ | — |
| `mg` | `mg` | Malagasy | us + long-press ñ ô | LTR | — | ~ | — |
| `st` | `st` | Sesotho | us | LTR | — | ~ | — |
| `tn` | `tn` | Setswana | us + long-press š | LTR | — | ~ | — |
| `nso` | `nso` | Sepedi | us + long-press š ê ô | LTR | — | ~ | — |
| `lg` | `lg` | Luganda | us + long-press ŋ ɲ | LTR | — | ~ | — |
| `ak` | `ak` | Akan | us + long-press ɛ ɔ | LTR | — | ~ | — |
| `ee` | `ee` | Ewe Eʋegbe | us + long-press ɖ ɛ ƒ ɣ ɔ ʋ ŋ | LTR | — | ~ | — |
| `bm` | `bm` | Bambara Bamanankan | us + long-press ɛ ɔ ɲ ŋ | LTR | — | ~ | — |
| `ff` | `ff` | Fula Fulfulde | us + long-press ɓ ɗ ƴ ŋ | LTR | — | ~ | — |
| `ts` | `ts` | Tsonga Xitsonga | us | LTR | — | ~ | — |
| `ve` | `ve` | Venda Tshivenḓa | us + long-press ḓ ḽ ṋ ṱ | LTR | — | ~ | — |
| `ss` | `ss` | Swati siSwati | us | LTR | — | ~ | — |
| `nr` | `nr` | Ndebele isiNdebele | us | LTR | — | ~ | — |
| `mi` | `mi` | Māori | us + long-press ā ē ī ō ū | LTR | — | ~ | — |
| `haw` | `haw` | Hawaiian Ōlelo Hawaiʻi | us + long-press ā ē ī ō ū | LTR | — | ~ | — |
| `sm` | `sm` | Samoan Gagana Sāmoa | us + long-press ā ē ī ō ū | LTR | — | ~ | — |
| `to` | `to` | Tongan Lea faka-Tonga | us + long-press ā ē ī ō ū | LTR | — | ~ | — |
| `fj` | `fj` | Fijian Vosa Vakaviti | us + long-press ā ē ī ō ū | LTR | — | ~ | — |
| `ty` | `ty` | Tahitian Reo Tahiti | us + long-press ā ē ī ō ū | LTR | — | ~ | — |
| `ht` | `ht` | Haitian Creole Kreyòl | us | LTR | — | ~ | — |
| `nah` | `nah` | Nahuatl Nāhuatl | us + long-press ā ē ī ō | LTR | — | ~ | — |
| `kl` | `kl` | Greenlandic Kalaallisut | us | LTR | — | ~ | — |
| `chr` | `chr` | Cherokee ᏣᎳᎩ | us · chr | LTR | — | — | — |
| `iu` | `iu` | Inuktitut ᐃᓄᒃᑎᑐᑦ | ca · ike | LTR | — | — | — |
| `mni` | `mni` | Meitei ꯃꯤꯇꯩ ꯂꯣꯟ | in · mni | LTR | **Full** (mni)⁶ | — | — |
| `mni-beng` | `mni-Beng` | Meitei মণিপুরী (Bengali script) | in · ben + long-press ৱ | LTR | **Full** (mni-Beng)⁷ | — | — |
| `dz` | `dz` | Dzongkha རྫོང་ཁ | bt | LTR | — | — | — |
| `cn-pinyin` | `zh-Hans` | Chinese 中文 (拼音) | — | LTR | IME | — | **Pinyin** (CC-CEDICT) |
| `tw-zhuyin` | `zh-Hant` | Chinese 中文 (注音) | — | LTR | IME | — | **Zhuyin** Bopomofo → Traditional (CC-CEDICT) |
| `yue-jyutping` | `yue` | Cantonese 粵語 (粵拼) | — | LTR | IME | — | **Jyutping** Latin → Traditional (CC-Canto) |
| `jp-romaji` | `ja` | Japanese 日本語 | — | LTR | IME | — | **Kana→Kanji** (mozc, connection-cost lattice) |
| `kr-2set` | `ko` | Korean 한국어 (두벌식) | — | LTR | IME | — | **Hangul** (algorithmic) |
| `am` | `am` | Amharic አማርኛ | — | LTR | IME | — | **Ge'ez** (phonetic Latin→fidel + homophone candidate bar) |
| `ti` | `ti` | Tigrinya ትግርኛ | — | LTR | IME | — | **Ge'ez** (shares the Amharic composer) |
| `hi-abc` | `hi` | Hindi हिन्दी (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the hi lexicon choosing |
| `bn-abc` | `bn` | Bengali বাংলা (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Bengali, the bn lexicon choosing |
| `mr-abc` | `mr` | Marathi मराठी (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the mr lexicon choosing |
| `gu-abc` | `gu` | Gujarati ગુજરાતી (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Gujarati, the gu lexicon choosing |
| `pa-abc` | `pa` | Punjabi ਪੰਜਾਬੀ (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Gurmukhi, the pa lexicon choosing |
| `ta-abc` | `ta` | Tamil தமிழ் (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Tamil, the ta lexicon choosing |
| `te-abc` | `te` | Telugu తెలుగు (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Telugu, the te lexicon choosing |
| `kn-abc` | `kn` | Kannada ಕನ್ನಡ (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Kannada, the kn lexicon choosing |
| `ml-abc` | `ml` | Malayalam മലയാളം (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Malayalam, the ml lexicon choosing |
| `ur-abc` | `ur` | Urdu اردو (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Perso-Arabic, the ur lexicon choosing |
| `fa-abc` | `fa` | Persian فارسی (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Perso-Arabic, the fa lexicon choosing |
| `el-abc` | `el` | Greek Ελληνικά (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Greek, the el lexicon choosing |
| `hy-abc` | `hy` | Armenian Հայերեն (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Armenian, the hy lexicon choosing |
| `as-abc` | `as` | Assamese অসমীয়া (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Bengali script, the as lexicon choosing |
| `sd-abc` | `sd` | Sindhi سنڌي (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Perso-Arabic, the sd lexicon choosing |
| `sd-deva-abc` | `sd-Deva` | Sindhi सिंधी (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the sd-Deva lexicon choosing |
| `si-abc` | `si` | Sinhala සිංහල (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Sinhala, the si lexicon choosing |
| `ne-abc` | `ne` | Nepali नेपाली (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the ne lexicon choosing |
| `or-abc` | `or` | Odia ଓଡ଼ିଆ (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Odia, the or lexicon choosing |
| `ps-abc` | `ps` | Pashto پښتو (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Perso-Arabic, the ps lexicon choosing |
| `mn-abc` | `mn` | Mongolian Монгол (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Cyrillic, the mn lexicon choosing |
| `bho-abc` | `bho` | Bhojpuri भोजपुरी (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the bho lexicon choosing |
| `mai-abc` | `mai` | Maithili मैथिली (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the mai lexicon choosing |
| `pnb-abc` | `pnb` | Western Punjabi پنجابی (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Perso-Arabic (Shahmukhi), the pnb lexicon choosing |
| `sa-abc` | `sa` | Sanskrit संस्कृतम् (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the sa lexicon choosing |
| `sat-abc` | `sat` | Santali ᱥᱟᱱᱛᱟᱲᱤ (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Ol Chiki, the sat lexicon choosing |
| `yi-abc` | `yi` | Yiddish ייִדיש (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Hebrew script, the yi lexicon choosing |
| `kok-abc` | `kok` | Konkani कोंकणी (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the kok lexicon choosing |
| `awa-abc` | `awa` | Awadhi अवधी (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the awa lexicon choosing |
| `tcy-abc` | `tcy` | Tulu ತುಳು (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Kannada script, the tcy lexicon choosing |
| `skr-abc` | `skr` | Saraiki سرائیکی (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Perso-Arabic (Shahmukhi), the skr lexicon choosing |
| `ks-abc` | `ks` | Kashmiri کٲشُر (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Perso-Arabic, the ks lexicon choosing |
| `mni-abc` | `mni` | Meitei ꯃꯤꯇꯩ ꯂꯣꯟ (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Meetei Mayek, the mni lexicon choosing |
| `mni-beng-abc` | `mni-Beng` | Meitei মণিপুরী (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Bengali script, the mni-Beng lexicon choosing |
| `brx-abc` | `brx` | Bodo बड़ो (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the brx lexicon choosing |
| `doi-abc` | `doi` | Dogri डोगरी (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the doi lexicon choosing |
| `hne-abc` | `hne` | Chhattisgarhi छत्तीसगढ़ी (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the hne lexicon choosing |
| `mag-abc` | `mag` | Magahi मगही (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the mag lexicon choosing |
| `brh-abc` | `brh` | Brahui براہوئی (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Perso-Arabic, the brh lexicon choosing |
| `gbm-abc` | `gbm` | Garhwali गढ़वाली (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the gbm lexicon choosing |
| `raj-abc` | `raj` | Rajasthani राजस्थानी (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the raj lexicon choosing |
| `khw-abc` | `khw` | Khowar کھوار (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Perso-Arabic, the khw lexicon choosing |
| `wsg-abc` | `wsg` | Gondi గోండి (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Telugu script, the wsg lexicon choosing |
| `emoji` | `emoji` | Emoji picker | — | — | — | — | Unicode 16 — categories, search, skin-tone, recents |
| `numeric` | — (ki-osk) | Numeric keypad | — | — | — | — | — |

## Notes

1. **Swipe and autocorrect draw candidates from the prediction
   dictionary**, so their quality tracks the model. Latin `Full`-model
   layouts get real dictionary-backed swipe; non-Latin models feed the
   suggestion bar only (no swipe on non-Latin scripts).
2. A layout marked `Full` loads its bundled FST + n-gram model. Thai is
   written without spaces between words: its text is cut into words with
   ICU's Thai word list (`wordbreak/th.txt`, in its pack), the same in
   the trainer, the keyboard and `eval-predict`, and a word from the
   strip is typed with no space after it. Its layout has every key of the
   Kedmanee keymap, the letters and vowel signs of the number row on a
   row above the others. The very-small-corpus languages
   (see below) remain layout-only — they type fine, but their suggestion
   bar is empty until a model is bundled.
3. These twelve models learn from Wikipedia text, the Dakshina dataset's
   (CC BY-SA 4.0, share-alike; `scripts/fetch-dakshina.sh`): all of it
   for the nine Tatoeba has too few sentences of, beside Tatoeba's (counted
   ten times) for Hindi, Bengali and Marathi, whose lexicons widen
   (Bengali's and Marathi's to 80 000 words) while their keystroke savings
   on Tatoeba's held-out text hold or rise (Hindi 46.1%, from 46.6%;
   Bengali 42.5%, from 40.7%; Marathi 42.8%, from 42.6%). Keystroke
   savings of the nine on held-out Wikipedia text: Punjabi 41.8%, Urdu
   41.8%, Sindhi 40.4%, Sinhala 36.1%, Tamil 34.5%, Gujarati 33.6%, Telugu
   32.8%, Malayalam 30.7%, Kannada 30.3%. The Dravidian languages' words
   take their endings with them: their lexicons keep 200 000 words (2.4
   to 3.0 MB) and still lack 15% (Tamil, Telugu) to 23% (Malayalam) of
   the running words; at 120 000 Tamil's and Telugu's lacked 17%,
   Kannada's 20%, Malayalam's a quarter, and they saved 0.5 to 1 point
   fewer keystrokes.
4. A transliteration layout types the US keys' Latin letters into its
   language's composer: its table (`translit/<lang>.txt`) says every way
   each letter or group may be written, at a cost, and its lexicon picks
   the readings that are words, the likeliest first, and offers the words
   the likeliest readings begin from the first letter (`namas`: नमस्ते);
   the strip shows two and the letters as typed, and Space takes the
   reading of the letters typed; the readings are ranked after the word
   written before them too, with the language's n-grams
   (`osk_data::translit::CONTEXT_WEIGHT`: `hai` is है after वह, हैं after
   वे). How often the word meant comes first
   (and among the strip's two), on the test half of Dakshina's romanized
   Wikipedia sentences (some 5 000 a language), and the keystrokes the
   strip saves typing them: Hindi 83.4% (86.6%; 23.3%), Urdu 81.2% (84.0%;
   23.2%), Gujarati 78.4% (81.7%; 25.3%), Punjabi 77.4% (81.6%; 24.6%),
   Marathi 76.7% (78.7%; 18.4%), Kannada 75.2% (76.6%; 20.8%), Sinhala
   74.9% (78.2%; 22.0%), Telugu 74.1% (75.7%; 20.8%), Tamil 73.2% (75.1%;
   18.6%), Bengali 72.5% (75.4%; 21.6%), Sindhi 70.2% (74.7%; 20.4%),
   Malayalam 63.3% (66.2%; 16.1%); of the words the lexicon has, 79–91%
   (`make eval-translit`, a word being the same word as its lexicon keys
   it: Sindhi's two h letters one). The word before raised Urdu 1.5
   points, Sindhi 1.1, Hindi 0.9 and the others up to 0.3, whose
   homophones it tells apart, and the words offered as letters are typed
   most: 0.7 to 4.2 points more keystrokes saved. The sentences are left
   out of the text the lexicons learn from, which held a few hundred to
   4 500 lines of them (`scripts/fetch-dakshina.sh`); the figures moved a
   tenth at most. These twelve tables were written by
   hand, then their costs fitted to how people romanized Dakshina's
   training words (`make fit-translit`: each word meant made as likely
   among its readings as the costs allow, each cost held to the one
   written), kept where that put more of its dev words and dev sentences'
   words first; nothing measured above was fitted on or chosen by. On the
   test sentences Sindhi rose 2.1 points so (from 67.0%), Urdu 1.0,
   Marathi 0.8, Punjabi 0.6, Hindi and Sinhala 0.5, Bengali and Telugu
   0.4, Malayalam 0.3 and Tamil 0.1, their keystroke savings with them;
   Gujarati's and Kannada's stayed as written, no fit doing better on
   their dev words. Lexicons of 200 000 words (from 120 000) then added
   Kannada 1.2 points, Telugu 0.9 (and its table, fitted again to it,
   0.7) and Tamil 0.8. Twenty-seven of the tables name the Latin letters
   as their language's Wikipedia or web text writes them (`[letters]`;
   Hindi's names for the Devanagari languages whose text has too few
   abbreviations to tell), for abbreviations typed letter by letter (tv:
   टीवी, bbc: பிபிசி, phd: পিএইচডি): rare in running text, they add at
   most 0.2 points there, Hindi's test words 0.6. Malayalam's and Tamil's tables
   read a doubled consonant typed single (palakad: പാലക്കാട്, thaakuthal:
   தாக்குதல்), which their typists often do, and Malayalam's ri as ൃ, mb
   as മ്പ and nch as ഞ്ച, with its lexicon's 200 000 words. Bengali's
   reads ৎ before a consonant too (utsob: উৎসব). Sindhi's texts write its
   h as ه and as ھ, and its lexicon kept a word both ways, a tenth of its
   misses; the splitter reads the two as one letter for Sindhi alone
   (`osk_data::words`, its generation 3), and its table writes ه. Assamese,
   Maithili, Nepali, Odia, Sanskrit, Konkani, Meitei, Kashmiri, Bodo and
   Dogri are measured on the words people romanized for AI4Bharat's
   Aksharantar dataset (its test split, 2 000 to 6 900 words each, mostly
   rare words and names; fetched to measure and fit, never shipped): of
   those their lexicons have, the word comes first for Sanskrit 85.7%
   (91.6% among the strip's two), Maithili 83.7% (89.3%), Konkani 79.5%
   (84.9%), Meitei 77.8% (81.9%), Nepali 77.1% (82.9%), Odia 76.7%
   (82.5%), Assamese 73.2% (77.1%), Bodo 67.7% (74.5%), Dogri 57.4%
   (63.1%) and Kashmiri 47.1% (61.2%). Assamese's lexicon, learned from
   FineWeb-2's text now (note 7), has 48% of its test words, from 17%,
   and of all of them puts 39.7% first, from 24.0% (of the few it had,
   86.0%). Seven of these tables' costs were fitted
   as Dakshina's were, to four fifths of the words Aksharantar's train
   and valid splits romanized by hand and of its Wikidata names, the
   other fifth to choose by: Kashmiri and Meitei rose 2.6 points so,
   Dogri 1.2, Maithili 0.8, Konkani 0.4, Sanskrit 0.1 and Bodo none, and
   of all their test words, names too, Konkani 4.1, Dogri 2.6, Meitei 2.2,
   Bodo 2.0, Sanskrit 1.8, Kashmiri 0.8 and Maithili 0.6; Assamese's,
   Nepali's and Odia's stayed as written. The tables read what those
   romanizations do: the visarga said again after its vowel (ramaha:
   रामः), Assamese's khy for ক্ষ, Nepali's and Maithili's ँ, Odia's ଙ୍କ
   and ର୍ତ୍ତ, Konkani's dh for ड and ie for ई, Meitei's e for a first ꯏ,
   Bodo's w for its vowel ो and its p, t and k for फ थ ख, Dogri's h after
   a vowel for the tone of a lost aspirate (kahr: घर). Many of Dogri's
   pairs are misaligned (deya: पतयाने), which caps its figure.
   Kashmiri's spelling marks its vowels as each writer chooses, and its
   lexicon has many words both with marks and without (مقبول, مقبوٗل): a
   third of its misses are the word with other marks. Greek is measured
   on AUEB's real-world Greeklish (Toumazatos et al., LREC-COLING 2024;
   Apache-2.0; fetched to measure, never shipped): forum comments and chat
   messages, each written out in Greek by four or six people, 2 275 words
   weighed by how many wrote each so: 77.2% first (80.3% among the
   strip's two), 90.5% of those its lexicon has (from 75.6% and 88.5%),
   its table reading Greeklish's digits for letters (8a: θα, pai3ei:
   παίζει), v for ν and ω (vomizo: νομίζω, mporv: μπορώ) and k for και;
   many of the rest are a writer's spelling without its accent (ΠΟΛΥ).
   Thirteen tables of languages neither dataset has are measured on
   Dakshina's romanizations of the words their lexicons share with
   Hindi's, Urdu's, Kannada's or Telugu's: two thirds to nine tenths of
   the words of its test sentences (Tulu's 45%), and how the people of a
   neighbouring language type them, the closest measure open data gives.
   Of those words, first: Tulu 89.4% (91.0% among the strip's two),
   Bhojpuri 87.2% (92.2%), Awadhi 86.2% (93.2%), Sindhi in Devanagari
   85.9% (91.3%), Rajasthani 85.7% (93.0%), Adilabad Gondi 85.5% (87.8%),
   Garhwali 85.4% (93.2%), Chhattisgarhi 80.1% (92.9%), Magahi 79.1%
   (90.6%), Western Punjabi 78.6% (88.5%), Saraiki 77.9% (85.7%), Khowar
   77.2% (86.2%) and Brahui 67.0% (82.0%). Gondi's is Telugu's table and
   costs, Khowar's Urdu's with its own letters (ݯ ݮ ݰ ݱ beside چ ج ش ژ,
   څ ځ for ts and dz). Eleven were fitted to those romanizations'
   training words as Dakshina's own tables were (Sindhi's stayed as
   written, Gondi's at Telugu's costs), a cost no shared word takes kept
   as written (Saraiki's implosives, Garhwali's and Rajasthani's ळ,
   Khowar's own letters, which those words would only push down): on
   their dev words Khowar rose 6.8 points so, Brahui 4.4, Western Punjabi
   3.9, Chhattisgarhi 3.5, Saraiki 2.1, Rajasthani 1.5, Awadhi 1.2,
   Bhojpuri 0.6, Garhwali 0.3, Magahi 0.3 (with its larger lexicon) and
   Tulu 0.2. Yiddish's table is measured on
   Wiktionary's words and their romanizations, which are YIVO's, the
   one people type (13 976 words and forms, as kaikki.org extracts them;
   `scripts/fetch-wiktionary-romanized.sh`): the word first for 82.3%
   of them and 87.8% of those its lexicon has, a word right without its
   points (Wiktionary writes פֿינף, Yiddish's wiki and many of its
   writers פינף, which of the two comes first being the lexicon's).
   Hebrew's plural endings ים and ות for -em and -es (khsidem: חסידים)
   raised the half of the words they were not chosen on from 85.6% to
   87.6% of the words the lexicon has. Armenian, Persian, Pashto,
   Mongolian, Santali and Meitei in the Bengali script have no such
   measure (Persian's and Pashto's speakers type the words they share
   with Urdu otherwise; Meitei shares few with Bengali); their tests
   read everyday words. Of the 51 checked in the seven tables new with
   Wikipedia's text, 48 come first and 49 are on the strip;
   Kathmandu's and Lahore's English spellings are not, and Pashto's da
   comes after ده. Sanskrit's, Santali's and Yiddish's tables put first
   all of 127 of their wikis' commonest words, typed as their Latin
   spellings have them; the tables were written with those words in view,
   so this says they read them, not how they do on what people type.
   Sanskrit's reads Harvard-Kyoto's and ITRANS's capitals too (A: आ, T:
   ट, Sh: ष), Santali's a word's checked consonants (mit: ᱢᱤᱫ), and
   Yiddish's YIVO's romanization, and the words from Hebrew that keep
   Hebrew's spelling (sholem: שלום). Of 54 everyday words of Awadhi,
   Saraiki and Tulu, 53 come first (Tulu's undu is ಉಂದು, "this", before
   ಉಂಡು, "is"), and of 74 of Dogri, Chhattisgarhi, Magahi and Brahui, 70,
   the others being two words typed alike, the other first (Chhattisgarhi's
   la: ला before ल). Of 69 everyday words of Sindhi in Devanagari, 65 come
   first and all are on the strip (its implosives typed single, the word
   without one first where the web writes both: boli: बोली, ॿोली), and all
   54 of Meitei's in the Bengali script. Hindi's, Marathi's and
   Malayalam's tables also read
   English spellings, for the English words those languages write in
   their script (school: स्कूल).
5. Hinglish is Hindi typed in Latin letters and left in them: the US
   keys, suggestions and glide from a model of Hindi's corpora respelled
   in the spellings people use, as often as each is used (the Dakshina
   dataset's romanization lexicons; `scripts/build-hinglish.py`).
   Keystroke savings 31.9% on 500 of Dakshina's romanized test sentences,
   as their writers spelled them, 13.4% of whose words the lexicon (60 000
   words) lacks: no two writers spell alike. For the same reason
   autocorrect changes 2.2% of right words, more than elsewhere.
6. These sixteen models learn from their own Wikipedia (CC BY-SA 4.0,
   share-alike; `scripts/fetch-wikipedia.sh`, the wikis' CirrusSearch
   dumps of 2026-11-08): Bhojpuri, Maithili, Mongolian, Nepali, Odia,
   Western Punjabi, Pashto, Sanskrit, Santali, Yiddish, Konkani (the
   Goan wiki's Devanagari articles), Kashmiri, Meitei, Awadhi, Saraiki and
   Tulu. Keystroke savings on held-out articles: Santali 50.2%, Kashmiri
   48.6%, Maithili 43.4%, Meitei 43.2%, Bhojpuri 41.6%, Yiddish 41.0%,
   Nepali 40.3%, Awadhi 40.2%, Pashto 39.4%, Mongolian 38.7%, Western
   Punjabi 38.0%, Saraiki 37.0%, Odia 34.6%, Tulu 31.6%, Konkani 31.1%,
   Sanskrit 27.5%. Each sentence is learned once: wikis that grew from
   templates repeat theirs (a quarter of Meitei's sentences were such
   copies, a fifth of Konkani's, an eighth of Maithili's), and the
   held-out text has none the model learned, as it had when Maithili
   measured 50.4%. Kashmiri's village articles still differ only in their
   names, which flatters its figure. A quarter of Sanskrit's running
   words are not among the 30 000 its lexicon keeps: compounds and sandhi
   make most of its words rare, and 120 000 would save 3 or 4 points
   more, at nearly twice the size. The text leaves out the wiki markup
   of templates that failed (9 000 of Sanskrit's sentences) and, where
   a script is named, sentences mostly in another (Santali's wiki has
   whole articles in English, Konkani's in Roman Konkani). Bhojpuri,
   Maithili, Sanskrit and Awadhi are typed on Hindi's keys, Konkani on
   Marathi's, Tulu on Kannada's, Western Punjabi, Saraiki and Kashmiri
   on Urdu's with their own letters, Yiddish on Hebrew's with its
   pointed letters and ligatures (`scripts/build-derived-layouts.py`).
   Bishnupriya Manipuri's wiki is not used: nearly all of it is town
   stubs a bot wrote (98% of its sentences fit 2 000 templates).
   Languages with no Wikipedia yet have test wikis in the Wikimedia
   Incubator, whose dump (the same date's, CC BY-SA 4.0) holds them all,
   each article's title starting with its wiki's prefix (`Wp/mag/`).
   Three are read: Adilabad Gondi's, in the Telugu script (2 301
   sentences from 355 articles, a lexicon of 12 803 words: keystroke
   savings 29.8% on 1 592 held-out words, 27.8% of them unknown),
   Khowar's (6 268 sentences from 2 482 articles, 21 670 words: 40.4% on
   3 085, 13.2% unknown) and Magahi's (33 341 sentences, beside its
   FineWeb-2 text: note 7). A test wiki's writers paste their
   neighbour's articles in: an article more than 6% of whose words are
   Urdu's or Hindi's own function words is left out (257 of Khowar's,
   77 of Magahi's), and a sentence more than 15% so. Khowar is typed on
   Urdu's keys with its own letters on the long presses of those that
   sound nearest (ځ ݮ on ج, څ ݯ on چ, ݰ on ش, ݱ on ز), Gondi on
   Telugu's. Haryanvi's test wiki is in the Urdu script and mostly lists
   of names, Pahari-Pothwari's mostly a script page, Chakma's 267
   sentences and Marwari's empty: none is used.
7. These twelve models learn from web text, FineWeb-2's (ODC-By 1.0,
   subject to Common Crawl's Terms of Use; `scripts/fetch-fineweb.sh`,
   the dataset's files at one revision pinned by sha256): Bodo, Dogri,
   Chhattisgarhi, Magahi, Brahui, Kokborok, Mizo and Khasi, which have
   no Wikipedia of their own, Sindhi in Devanagari and Meitei in the
   Bengali script, whose wikis are in their other scripts, and Lao, whose
   wiki is small; and Assamese's beside its 14 000 Tatoeba sentences
   (counted ten times), whose model lacked two fifths of the web's
   running words.
   Its language classifier mistakes close languages, so a document is
   kept only when it reads as its language (`scripts/fineweb-text.py`):
   none heavy with Hindi's or Urdu's own function words, none that is a
   scanned book's garbled text, and for Dogri none whose words are more
   Punjabi's than Dogri's (half of FineWeb-2's Dogri is Punjabi poetry
   and Himachali). Its Angika (Hindi news), Malvi and Devanagari Kashmiri
   are not used, nor its Garo, all one Bible. Keystroke savings on
   FineWeb-2's held-out test text, which is not committed, the text
   being the web's: Bodo 48.1%, Brahui 46.4%, Magahi 31.4%,
   Chhattisgarhi 30.1%, Dogri 30.1% (its test text only 33 sentences),
   Sindhi in Devanagari 40.6% (125), Meitei in the Bengali script 36.5%.
   Three quarters of the Bengali-script Meitei is the Prime Minister's
   speeches in translation (narendramodi.in), which its model's
   suggestions lean to. Sindhi's implosives, which the web mostly writes
   as a plain letter with the underline ॒ (ग॒), are read as the letters
   Unicode has for them (ॻ ॼ ॾ ॿ), which its keyboard types. Bodo, Dogri,
   Chhattisgarhi, Magahi and Sindhi are typed on Hindi's keys (Sindhi's
   implosives on the long presses of ग ज ड ब), Meitei on Bengali's with ৱ,
   Brahui on Urdu's with ۺ. Lao, written without spaces between words,
   is cut into words with ICU's Lao word list, as Thai is with its own;
   FineWeb-2 has 780 million characters of it, of which every 12th row
   group is read (590 000 sentences), cut where an English word stands
   (half the words of a model learned with them were English names,
   each rare), and its letters written two ways (ໍາ and ຳ, ຫນ and ໜ, ຫມ
   and ໝ, ຫລ and ຫຼ) are one word in the text, the word list and what
   is typed (`osk_data::wordbreak::lao_fold`). Keystroke savings 44.0%
   on 9 593 held-out words. Its keyboard now has the vowel signs
   xkeyboard-config keeps on the number row (ຸ ູ ໍ, and ໌ ຼ on Shift),
   which the layout's generator had left out. Bodo's apostrophe, which
   marks a vowel inside a word and at its end (बर'), stays in the word
   (बर' is among its lexicon's commonest). Assamese's text is every 6th
   row group of its FineWeb-2 (856 000 sentences), each document's ৰ
   outnumbering Bengali's র (only 14 of 267 000 failed): keystroke
   savings on FineWeb-2's held-out text 17.9% -> 34.1%, the words its
   lexicon lacks 41.4% -> 10.2% (80 000 words; on Tatoeba's held-out
   sentences 42.3%, from 43.3%); its lexicon has 48% of Aksharantar's
   test words, from 17%, the word first for 39.7% of them, from 24.0%.
   Magahi learns from its Incubator text too (note 6), twice its
   FineWeb-2 text, which is counted five times (FINEWEB_WEIGHT) so that
   the web's text, nearer what is typed, leads the strip, and its lexicon
   keeps 72 000 words: keystroke savings on FineWeb-2's held-out text
   31.5% (31.4% before), its words unknown 4.6% (5.7%), and on the
   Incubator's held-out articles 38.5% (17.8%); counted once, 29.8% and
   42.4%. Mizo and Khasi are written in Latin letters and typed on the US
   keys with their own letters first on the long presses (Mizo's â ê î ô
   û ṭ, Khasi's ï ñ). Mizo's FineWeb-2 text is large (100 MB): every
   third row group is read, 499 931 sentences from 30 099 documents (a
   sixth of its documents are the Mizoram government's press releases,
   the rest news sites, blogs and churches'). Khasi's is two fifths Pnar
   (Jaintia), its neighbour, which FineWeb-2 files under Khasi: a
   document is kept where Khasi's own words (ba jong bad kiba kaba)
   outnumber Pnar's (wa ya kiwa kawa ko wow) and English's, 18 451 of
   29 993 (Pnar's wyrta.com left out), 210 145 sentences. Keystroke
   savings on FineWeb-2's held-out text: Khasi 49.6% (1.1% of the words
   unknown), Mizo 39.0% (2.4%), with 30 000 words each; 80 000 would
   save a point more, at four times the lexicon's size.
   Kokborok, written in Latin letters in Tripura and typed on the US
   keys, has 226 documents in FineWeb-2, two fifths of them from
   adivasilivesmatter.com and most of the rest from Kokborok sites; the
   217 of its 222 training documents whose own words (tei ni no bai khe)
   outnumber English's give 4 664 sentences, a sentence mostly English
   left out, and a lexicon of 15 356 words. Keystroke savings 36.9% on
   the 76 sentences of its four test documents (1 806 words, 15.8% of
   them unknown). MADLAD-400's Kokborok is not used: it is mostly Thai
   dating and download spam, and Garo news.
8. These two models learn from web text, MADLAD-400's (Google; CC BY 4.0
   by its card, whose metadata says ODC-By; subject to Common Crawl's
   Terms of Use; `scripts/fetch-madlad.sh`, its files at one revision
   pinned by sha256): Garhwali and Rajasthani, which neither Wikipedia nor
   FineWeb-2 has. Its clean files are small (137 and 40 documents), so its
   noisy ones are read too, a document kept where Hindi's function words
   are under 6% of its words and its own (Garhwali's मा च छौ ह्वे कु,
   Rajasthani's री सूं मांय रो) are more, and Garhwali's where Nepali's are
   fewer (its noisy file is half Nepali casino spam): 977 and 493
   documents, 46 000 and 42 000 sentences, every 20th document held out.
   Keystroke savings on those: Garhwali 31.9%, Rajasthani 30.1% (a tenth
   of the words unknown). Both are typed on Hindi's keys with ळ on the
   long press of ल, which both write (Rajasthani's सगळा, all); their
   tables are Hindi's, with ळ for l at a small cost.

## Prediction model tiers

- **Full n-gram model** — `predict/<lang>.fst` +
  `<lang>.ngram.bin`, **96 languages** (Hinglish among them). The original nine (en,
  de, es, fr, it, nl, pl, pt, sv) plus 40 from Tatoeba: no da fi cs tr vi
  el ru uk ar he fa · ro hu sk hr et is ca eo bg sr mk be · id fil az ku
  ug hi bn hy ka lt lv · kab (Kabyle) · **Tier E: oc gl br eu** (Occitan,
  Galician, Breton, Basque) · **corpus audit: tt mr as ms os** (Tatar 92k,
  Marathi 86k, Assamese 15k, Malay 8k, Ossetian 7k — layout-only languages
  that turned out to have large Tatoeba corpora) · **th** (Thai, 6.5k,
  cut into words with ICU's word list, as it has no spaces between
  them) · **Wikipedia text: gu kn ml pa sd si ta te ur** (the Dakshina
  dataset's, which hi bn mr also learn from; note 3) · **the wikis' own
  text: bho mai mn ne or pnb ps sa sat yi kok ks mni awa skr tcy**, and the
  Incubator's test wikis: **wsg khw** (note 6) ·
  **web text: brx doi hne mag brh sd-Deva mni-Beng lo trp lus kha** (FineWeb-2's; note 7;
  Magahi with its Incubator text too; Lao cut into words with ICU's word list too), **as** (FineWeb-2's beside Tatoeba's), **gbm raj** (MADLAD-400's; note 8). Built by
  `make fetch-corpora` + `make predict-assets` in the
  language-content repository; runtime-loaded from `/usr/share/ki-osk/predict/`.
- **Layout-only** — no bundled model yet, because Tatoeba has too few
  sentences (< ~6k): the smaller Indic / SE-Asian / minor
  languages (km my mt fo bs sl ga sq tg kk ky, sr-latn) plus syr (Syriac), ber (Tamazight),
  qu, gn, ay, and most of Tiers E–H (European minority, African, Pacific /
  Americas, and the new-script layouts) — **except `oc gl br eu` and the
  corpus-audit batch `tt mr as ms os`, which cleared the bar** (see above).
  The recognizer runs but has no vocabulary for the rest; the next
  candidates, just under the ~6k threshold, are Afrikaans (4.9k), Udmurt
  (4.9k), Swahili (4.6k), and Kazakh (4.5k).
- **IME dictionaries** — Pinyin (CC-CEDICT `pinyin/cedict.bin`),
  Japanese (`kana/mozc-kanji.bin`); Hangul is algorithmic and needs no
  dictionary.

## Fonts

Keycaps are drawn with DejaVu Sans (Latin, Greek, Cyrillic, core Arabic).
Every other script falls back to a runtime Noto Sans face, **loaded
lazily the first time a layout that needs it is shown** — the daemon
never opens the Tamil font unless a Tamil layout appears, so a build with
many scripts enabled still starts fast and stays lean in memory. The
layout's `font_script` field (or its IME kind, for the CJK candidate bar)
drives which face loads:

| `font_script` | Face | Layouts |
|---|---|---|
| `cjk` | a glyf Han/Kana/Hangul face (wqy-zenhei, Droid, …) | the 5 CJK IMEs |
| `thai` | Noto Sans Thai | `th` |
| `arabic` | Noto Sans Arabic | `ur ps sd ug` (ar/he/fa use DejaVu) |
| `devanagari`…`myanmar` | `NotoSans<Script>-Regular.ttf` | the 15 Tier-3 layouts |
| `syriac` | Noto Sans Syriac | `syr` (RTL) |
| `tifinagh` | Noto Sans Tifinagh | `ber` |
| `ethiopic` | Noto Sans Ethiopic | `am ti` (Ge'ez fidel preedit) |
| `tibetan` | Noto Sans Tibetan | `bo dz` (stacking via shaping) |
| `thaana` | Noto Sans Thaana | `dv` (RTL) |
| `olchiki` | Noto Sans Ol Chiki | `sat` |
| `cherokee` | Noto Sans Cherokee | `chr` (syllabary) |
| `canadian_aboriginal` | Noto Sans Canadian Aboriginal | `iu` (UCAS syllabics) |
| `meetei_mayek` | Noto Sans Meetei Mayek | `mni` (syllabary) |

Install the matching face or those caps render as tofu. See the full
fonts table in ki-osk-wayland's `DEPLOY.md`.

## Expansion layouts

Generated from xkb by `scripts/build-layout.py` (native keycaps). Most now
carry a Tier-4 prediction model — the [Prediction model tiers](#prediction-model-tiers)
list says which. The rest are corpus-limited: they type fine but show no
suggestions until enough permissively-licensed text exists.

Per-layout capabilities are in the table above; the tiers below just
record how each group was added and what it needs at runtime:

- **Tier 1 — European**: 16 Latin + 4 Cyrillic,
  all DejaVu-drawable, no new font.
- **Tier 2 — Asian in already-renderable scripts**: 4
  Latin (`id fil az ku`) + 4 Arabic-script RTL (`ur ps sd ug`, which need
  Noto Sans Arabic).
- **Tier 3 — complex scripts**, each needing its Noto Sans face (loaded
  lazily): Caucasus (`ka hy`), Brahmic (10 Indian scripts), Southeast
  Asian (`km lo my`).
  *Composed* clusters (conjuncts, stacks, contextual joining) now shape
  correctly through the fontations engine; they ship layout-only pending
  prediction corpora — see
  [Complex-script rendering](#complex-script-rendering).
- **Middle East / North Africa**: `syr` Syriac (RTL, its
  own Noto Sans Syriac face) and `ber` Tamazight in Tifinagh (Noto Sans
  Tifinagh) — both layout-only; `kab` Kabyle in Latin, which *does* carry a
  full prediction model (Tatoeba has ~790k Kabyle sentences).
- **South America**: `qu` Quechua, `gn` Guaraní, `ay`
  Aymara — Latin on the Spanish (`es`) base, layout-only (sparse corpora).
- **African + more Asian**: a coverage batch the shaping engine unblocked
  (rendering is no longer the gate). Clean a–z — Swahili, Somali, Zulu,
  Xhosa, Afrikaans, Oromo; West-African letters on long-press — Hausa
  (ɓ ɗ ƙ ƴ), Yoruba (dot-below + tone), Igbo (ị ọ ụ ṅ); SE-Asian Malay,
  Javanese, Sundanese, Cebuano; Central-Asian Uzbek (oʻ gʻ) and Turkmen
  (ä ç ž ň ö ş ü ý). Split-outs of already-rendered scripts: Marathi +
  Nepali (Devanagari), Assamese (Bengali, its ৰ ৱ on long-press). All
  layout-only — prediction turns on per language as a permissively-
  licensed corpus appears.
- **Amharic / Tigrinya Ge'ez**: the first *new script*
  added since the shaping engine landed. `am` is a Latin QWERTY driving a
  phonetic composer (ki-osk's `ImeKind::Geez`, `crates/osk-core/src/ime/geez.rs`)
  that maps SERA-style transliteration to Ethiopic fidel — a consonant
  sets the fidel, a vowel picks one of the seven orders (`base + 0..=6` in
  the contiguous Ethiopic block), and a bare consonant takes the 6th
  (vowelless) order, so `selam` → ሰላም. Rendered with Noto Sans Ethiopic.
  Amharic's homophone families (the "four h's" ሀ ሐ ኀ, "two s's" ሰ ሠ, "two
  ts's" ጸ ፀ, the glottal/pharyngeal አ ዐ) surface as a **candidate bar** —
  the composer offers the same syllable in each family member and a tap
  swaps it, so any word can be spelled with its correct fidel. Labiovelars
  (`Cʷ`) compose via `Cw` + vowel (`kwa` → ኳ). Tigrinya (`ti`, same
  composer) reaches its distinctive fidels via digraphs (`hh` ሐ, `qh` ቐ,
  `x` ኸ).
- **Additional scripts**: three minority scripts the
  shaping engine now renders, generated from their xkb layouts — Tibetan
  `bo` (`cn`/`tib`; subjoined stacks compose via shaping), Dhivehi `dv`
  (`mv`, the Thaana script, **RTL**), and Santali `sat` (`in`/`sat`, the
  Ol Chiki alphabet). Each loads its own Noto Sans face on demand;
  Santali predicts since its wiki's text gave it a model (note 6), the
  others are layout-only pending corpora. Verified headless: Dhivehi renders
  RTL-mirrored, Santali in Ol Chiki, and Tibetan now renders too — proven
  on the test machine via `dz` Dzongkha (Tier H), which shares the `bo` face.
- **Tier E — European minority / regional**: 14 more,
  all DejaVu-drawable. Ten Latin on the us/es base — Celtic (`cy` Welsh
  with ŵ/ŷ, `gd` Scottish Gaelic), Iberian (`eu` Basque, `gl` Galician),
  and regional Romance/Germanic (`oc` Occitan, `br` Breton, `co` Corsican,
  `lb` Luxembourgish, `rm` Romansh, `fy` Frisian) — plus four Cyrillic
  minorities of Russia whose alphabet overflows to a top row: `tt` Tatar
  (ә ө ү җ ң һ), `ba` Bashkir (ө ҡ ғ ҫ ҙ), `os` Ossetian (ӕ), `udm`
  Udmurt (ӝ ӧ ӵ ӥ ӟ). Read from their national xkb (native keycaps).
- **Tier F — African expansion**: 19 more Latin on the
  us base, all DejaVu-drawable. Clean a–z — Kinyarwanda, Kirundi, Shona,
  Sesotho, Tsonga, Swati, Ndebele (inherited US accent menus). Extra
  letters on long-press — open vowels ɛ/ɔ (Lingala, Akan, Bambara, Ewe),
  Ewe's full set (ɖ ƒ ɣ ʋ ŋ), velar ŋ / palatal ɲ (Luganda, Fula),
  hooked ɓ ɗ ƴ (Fula), š (Setswana, Sepedi), ñ/ŵ (Wolof, Malagasy,
  Chichewa), and Venda's ḓ ḽ ṋ ṱ. Cased to the key (a→ā, A→Ā;
  ɛ→Ɛ on shift).
- **Tier G — Austronesian / Pacific + Americas**: nine more Latin on the us base, all DejaVu-drawable.
  Six Polynesian with macron long vowels on long-press (`mi` Māori,
  `haw` Hawaiian, `sm` Samoan, `to` Tongan, `fj` Fijian, `ty` Tahitian —
  the ʻokina is typed from the symbols-view apostrophe), and three
  Americas (`ht` Haitian Creole with inherited è/ò/à, `nah` Nahuatl with
  macrons, `kl` Greenlandic clean a–z).
- **Tier H — new scripts + Tigrinya**:
  three syllabaries generated from their xkb — `chr` Cherokee (Noto Sans
  Cherokee), `iu` Inuktitut in Canadian Aboriginal Syllabics (Noto Sans
  Canadian Aboriginal), `mni` Meitei in Meetei Mayek (Noto Sans Meetei
  Mayek; its wiki has given it a model since, note 6) — plus `dz` Dzongkha, which is the Tibetan script and reuses the
  `bo` face. `ti` Tigrinya shares the Amharic
  Ge'ez phonetic composer (`ImeKind::Geez`) and Noto Sans Ethiopic. Each
  new face loads lazily like the others. **Render-verified headless on the
  test machine** — all four new-script keyboards paint real glyphs (Meetei/Tibetan
  combining signs show on a dotted circle, the mobile convention). (N'Ko
  and Yi were probed but aren't in this host's xkeyboard-config, so they're
  deferred.)

Two groups place their letters unusually, so they get a dedicated
treatment (the specific letters, which the table above doesn't show):

**Number-row-overflow layouts** — alphabets that spill past the 32-key
letter grid, so their surplus letters ride a **centred top row** lifted
from the national layout's number-row keycodes (digits stay in the symbol
view). All DejaVu-drawable, no extra font:

| Layout | Extra letters | Pack |
|---|---|---|
| `kk` Kazakh | ә і ң ғ ү ұ қ ө һ | `kk` |
| `ky` Kyrgyz | ө ң | `ky` |
| `mn` Mongolian | е щ (ө ү are in the grid) | `mn` |
| `tg` Tajik | ғ ӯ (ӣ қ ҳ ҷ in the grid) | `tg` |
| `lt` Lithuanian | ą č ę ė į š ų ū ž | `lt` |

**Long-press national letters** — layouts whose extra letters ride a
**long-press menu** on a related base key (the mobile convention) rather
than their own keycode. Latvian's letters compose through an xkb dead key
(`'` + `a` → `ā`) the keycode reader can't see; the ME/NA/SA Latin layouts
are built on a `us`/`es` base and add theirs the same way:

| Layout | Long-press letters | Feature |
|---|---|---|
| `lv` Latvian | ā č ē ģ ī ķ ļ ņ š ū ž | `lv` |
| `kab` Kabyle | č ǧ ɣ ɛ ḍ ḥ ṛ ṣ ṭ ẓ (g → ǧ/ɣ) | `kab` |
| `gn` Guaraní | ã ẽ ĩ õ ũ ỹ · g̃ (ñ is on the es base) | `gn` |
| `ay` Aymara | ä ï ü | `ay` |
| `ha` Hausa | ɓ ɗ ƙ ƴ (hooked b/d/k/y) | `ha` |
| `yo` Yoruba | ẹ ọ ṣ + á à é è tone marks | `yo` |
| `ig` Igbo | ị ọ ụ ṅ | `ig` |
| `uz` Uzbek | oʻ gʻ (turned comma) | `uz` |
| `tk` Turkmen | ä ç ž ň ö ş ü ý | `tk` |
| `as` Assamese | ৰ ৱ (on Bengali র/ব) | `as` |

Devanagari serves Hindi and, unchanged, Marathi/Nepali; one layout per
*script*. InScript keeps the alphabet on the letter block and digits on
the number row, so the generator's three cloned letter rows capture the
script; ki-osk's digits live in the symbol view, untouched.

Coverage: the generator now reaches **every base-level letter** of the
supported national keymaps — the letter block, the number-row overflow,
the punctuation-position keys (`41 43 53 86`), and long-press diacritics
together leave nothing stranded. Icelandic `þ`, Hungarian `í`/`ő`/`ű`,
Maltese `ċ`/`ż`, Belarusian/Kyrgyz/Tajik `ё`, and the rest all have keys.
What can still be absent is a character only reachable through AltGr or a
compose sequence in its national layout — rare, and usually a variant of a
letter that already has a key. **No language is fully deferred.**

## Choosing languages

A device has the layouts — and the prediction and composer data — of the
packs it installs, so the languages a keyboard carries are the packs it
is given, not a build of it:

- a kiosk: ki-osk-wayland's `make dist-data PACKS="en de emoji"`, or the
  all-languages bundle (`ki-osk-lang-X.Y.Z.tar.xz`);
- Android: ki-osk-android's `PACKS="en de emoji" ./build.sh`.

The Pack column above says which pack holds a layout. `emoji` is the
picker and its search, and `offensive` only `--filter-offensive` needs. A
layout can also be added without a pack, by dropping its JSON into
`/usr/share/ki-osk/layouts/` (see ki-osk-wayland's `DEPLOY.md`).

## Complex-script rendering

The Wayland keyboard shapes every label — keycaps, the suggestion and
candidate bar, the emoji strip — through its fontations text engine
(ki-osk-wayland's `crates/osk/src/render/text.rs`): Indic conjuncts,
Khmer/Myanmar stacks and Arabic/Syriac contextual joining compose, and
each script's Noto Sans face loads on first use. So turning prediction on
for these scripts is a **data** question — a permissively-licensed corpus
for the n-gram model — not a rendering one; until then they ship
layout-only. See the [prediction tiers](#prediction-model-tiers).

## Selecting layouts at runtime

- `--default-layout <id>` and `--enabled-layouts <csv>` on the daemon
  (see ki-osk-wayland's `DEPLOY.md` / `ki-osk --help`).
- The globe key cycles enabled layouts; long-press it for a picker. The
  current layout persists to `$XDG_STATE_HOME/ki-osk/state.toml`.
