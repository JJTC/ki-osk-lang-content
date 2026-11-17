# Supported languages & layouts

What the language packs hold per language: the keyboard layouts, text
direction, predictive-text tier, swipe typing, and composer (CJK,
Ge'ez, transliteration). Each
layout is a file, `layouts/<id>.json`, in its language's pack (the Pack
column below, from `packs.toml`'s `[layouts]`), beside that language's
prediction and composer data.

**352 layouts**: 297 keyboard layouts (RTL: ar he fa + Urdu, Western
Punjabi, Saraiki, Kashmiri, Brahui, Khowar, Pashto, Sindhi, Uyghur,
Central Kurdish, South Azerbaijani, Mazandarani, Syriac, Dhivehi, Yiddish; Hinglish, Kokborok) · 7 IMEs (5 CJK +
Amharic/Tigrinya Ge'ez) · 46 transliteration layouts (Latin letters →
the language's script, `-abc`) · emoji + numeric. All render native
keycaps. 351 are here; the numeric keypad is built into ki-osk. The
table below is the complete list; **290 languages** carry a prediction
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
| `fa` | `fa` | Persian (Farsi) | ir | **RTL** | **Full** (fa)⁶ | — | — |
| `pt` | `pt` | Portuguese (PT) | pt | LTR | **Full** (pt) | ✓ | — |
| `ro` | `ro` | Romanian | ro · std | LTR | **Full** (ro) | ✓ | — |
| `hu` | `hu` | Hungarian | hu | LTR | **Full** (hu) | ✓ | — |
| `sk` | `sk` | Slovak | sk | LTR | **Full** (sk) | ✓ | — |
| `sl` | `sl` | Slovenian | si | LTR | **Full** (sl)⁷ | ✓ | — |
| `hr` | `hr` | Croatian | hr | LTR | **Full** (hr) | ✓ | — |
| `bs` | `bs` | Bosnian | ba | LTR | **Full** (bs)⁷ | ✓ | — |
| `sr-latn` | `sr-Latn` | Serbian (Latin) | rs · latin | LTR | **Full** (sr-Latn)⁷ | ✓ | — |
| `et` | `et` | Estonian | ee | LTR | **Full** (et) | ✓ | — |
| `ga` | `ga` | Irish | ie | LTR | **Full** (ga)⁷ | ✓ | — |
| `mt` | `mt` | Maltese | mt | LTR | **Full** (mt)⁷ | ✓ | — |
| `is` | `is` | Icelandic | is | LTR | **Full** (is) | ✓ | — |
| `fo` | `fo` | Faroese | fo | LTR | **Full** (fo)⁷ | ✓ | — |
| `sq` | `sq` | Albanian | al | LTR | **Full** (sq)⁷ | ✓ | — |
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
| `ckb` | `ckb` | Central Kurdish (Sorani) کوردیی ناوەندی | iq · ku_ara + long-press ێ ڵ ڕ | **RTL** | **Full** (ckb)⁷ | — | — |
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
| `kk` | `kk` | Kazakh | kz | LTR | **Full** (kk)⁷ | — | — |
| `ky` | `ky` | Kyrgyz | kg | LTR | **Full** (ky)⁷ | — | — |
| `mn` | `mn` | Mongolian | mn | LTR | **Full** (mn)⁶ | — | — |
| `tg` | `tg` | Tajik | tj | LTR | **Full** (tg)⁷ | — | — |
| `lt` | `lt` | Lithuanian | lt | LTR | **Full** (lt) | ✓ | — |
| `lv` | `lv` | Latvian | lv | LTR | **Full** (lv) | ✓ | — |
| `ka` | `ka` | Georgian | ge | LTR | **Full** (ka) | — | — |
| `hy` | `hy` | Armenian | am | LTR | **Full** (hy)⁶ | — | — |
| `hi` | `hi` | Hindi | in | LTR | **Full** (hi)³ | — | — |
| `brx` | `brx` | Bodo बड़ो | in | LTR | **Full** (brx)⁷ | — | — |
| `doi` | `doi` | Dogri डोगरी | in | LTR | **Full** (doi)⁷ | — | — |
| `hne` | `hne` | Chhattisgarhi छत्तीसगढ़ी | in | LTR | **Full** (hne)⁷ | — | — |
| `mag` | `mag` | Magahi मगही | in | LTR | **Full** (mag)⁶ ⁷ | — | — |
| `gbm` | `gbm` | Garhwali गढ़वाली | in (+ ळ long-press) | LTR | **Full** (gbm)⁸ | — | — |
| `raj` | `raj` | Rajasthani (Marwari) राजस्थानी | in (+ ळ long-press) | LTR | **Full** (raj)⁸ | — | — |
| `hi-latn` | `hi-Latn` | Hinglish (Hindi in Latin letters) | us | LTR | **Full** (hi-Latn)⁵ | ✓ | — |
| `trp` | `trp` | Kokborok | us | LTR | **Full** (trp)⁷ | ✓ | — |
| `lus` | `lus` | Mizo | us + long-press â ê î ô û ṭ | LTR | **Full** (lus)⁷ | ✓ | — |
| `kha` | `kha` | Khasi | us + long-press ï ñ | LTR | **Full** (kha)⁷ | ✓ | — |
| `pcm` | `pcm` | Nigerian Pidgin (Naijá) | us | LTR | **Full** (pcm)⁷ | ✓ | — |
| `mww` | `mww` | Hmong Daw (White Hmong) Hmoob Dawb | us | LTR | **Full** (mww)⁷ | ✓ | — |
| `cnh` | `cnh` | Hakha Chin Laiholh | us + long-press ṭ | LTR | **Full** (cnh)⁷ | ✓ | — |
| `kac` | `kac` | Jingpho (Kachin) Jinghpaw | us | LTR | **Full** (kac)⁷ | ✓ | — |
| `hil` | `hil` | Hiligaynon | us + long-press ñ | LTR | **Full** (hil)⁷ | ✓ | — |
| `ilo` | `ilo` | Iloko (Ilocano) Ilokano | us + long-press ñ | LTR | **Full** (ilo)⁷ | ✓ | — |
| `bcl` | `bcl` | Central Bikol Bikol Sentral | us + long-press ñ | LTR | **Full** (bcl)⁷ | ✓ | — |
| `pag` | `pag` | Pangasinan | us + long-press ñ | LTR | **Full** (pag)⁷ | ✓ | — |
| `pam` | `pam` | Kapampangan | us + long-press ñ | LTR | **Full** (pam)⁷ | ✓ | — |
| `krj` | `krj` | Kinaray-a | us + long-press ñ | LTR | **Full** (krj)⁷ | ✓ | — |
| `iba` | `iba` | Iban Jaku Iban | us | LTR | **Full** (iba)⁷ | ✓ | — |
| `bbc` | `bbc` | Batak Toba Hata Batak Toba | us | LTR | **Full** (bbc)⁷ | ✓ | — |
| `btx` | `btx` | Karo (Batak Karo) Cakap Karo | us | LTR | **Full** (btx)⁷ | ✓ | — |
| `bts` | `bts` | Batak Simalungun Sahap Simalungun | us | LTR | **Full** (bts)⁷ | ✓ | — |
| `ace` | `ace` | Acehnese Bahsa Acèh | us + long-press é ë è ô ö | LTR | **Full** (ace)⁷ | ✓ | — |
| `ban` | `ban` | Balinese Basa Bali | us + long-press é | LTR | **Full** (ban)⁷ | ✓ | — |
| `bjn` | `bjn` | Banjar Bahasa Banjar | us | LTR | **Full** (bjn)⁷ | ✓ | — |
| `dtp` | `dtp` | Kadazandusun | us | LTR | **Full** (dtp)⁷ | ✓ | — |
| `tet` | `tet` | Tetum Tetun | us + long-press ñ | LTR | **Full** (tet)⁷ | ✓ | — |
| `ium` | `ium` | Iu Mien Iu Mienh | us | LTR | **Full** (ium)⁷ | ✓ | — |
| `nn` | `nn` | Norwegian Nynorsk Norsk nynorsk | no | LTR | **Full** (nn)⁷ | ✓ | — |
| `ast` | `ast` | Asturian Asturianu | es | LTR | **Full** (ast)⁷ | ✓ | — |
| `an` | `an` | Aragonese Aragonés | es | LTR | **Full** (an)⁷ | ✓ | — |
| `lad` | `lad` | Ladino (Judaeo-Spanish) Ladino | es | LTR | **Full** (lad)⁷ | ✓ | — |
| `gsw` | `gsw` | Alemannic (Swiss German, Alsatian) Alemannisch | de + long-press è é à ì ò ù | LTR | **Full** (gsw)⁷ | ✓ | — |
| `bar` | `bar` | Bavarian Boarisch | de + long-press å | LTR | **Full** (bar)⁷ | ✓ | — |
| `nds` | `nds` | Low German Plattdüütsch | de | LTR | **Full** (nds)⁷ | ✓ | — |
| `pfl` | `pfl` | Palatine German Pälzisch | de | LTR | **Full** (pfl)⁷ | ✓ | — |
| `ksh` | `ksh` | Kölsch | de | LTR | **Full** (ksh)⁷ | ✓ | — |
| `li` | `li` | Limburgish Limburgs | nl + long-press è ë é ê ö ó ô ò | LTR | **Full** (li)⁷ | ✓ | — |
| `hsb` | `hsb` | Upper Sorbian Hornjoserbsce | de + long-press ć č ě ł ń ó ř š ž ź | LTR | **Full** (hsb)⁷ | ✓ | — |
| `dsb` | `dsb` | Lower Sorbian Dolnoserbski | de + long-press ć č ě ł ń ó ŕ ś š ž ź | LTR | **Full** (dsb)⁷ | ✓ | — |
| `nap` | `nap` | Neapolitan Napulitano | it | LTR | **Full** (nap)⁷ | ✓ | — |
| `scn` | `scn` | Sicilian Sicilianu | it + long-press à â è ê ì î ò ô ù û | LTR | **Full** (scn)⁷ | ✓ | — |
| `sc` | `sc` | Sardinian Sardu | it | LTR | **Full** (sc)⁷ | ✓ | — |
| `fur` | `fur` | Friulian Furlan | it + long-press â à ê è î ì ô ò û ù ç | LTR | **Full** (fur)⁷ | ✓ | — |
| `pms` | `pms` | Piedmontese Piemontèis | it + long-press ë é è ė ò ö ù ü | LTR | **Full** (pms)⁷ | ✓ | — |
| `lij` | `lij` | Ligurian Ligure | it + long-press â à æ é è ê ç ò ô ö ù û ü | LTR | **Full** (lij)⁷ | ✓ | — |
| `wa` | `wa` | Walloon Walon | fr-azerty + long-press å â à | LTR | **Full** (wa)⁷ | ✓ | — |
| `ltg` | `ltg` | Latgalian Latgalīšu | lv + long-press ō | LTR | **Full** (ltg)⁷ | ✓ | — |
| `vro` | `vro` | Võro | et + long-press š ž | LTR | **Full** (vro)⁷ | ✓ | — |
| `csb` | `csb` | Kashubian Kaszëbsczi | pl + long-press ë é ę ò ô ó ã ą ù | LTR | **Full** (csb)⁷ | ✓ | — |
| `szl` | `szl` | Silesian Ślōnski | pl + long-press ō ŏ ô ó ů ã | LTR | **Full** (szl)⁷ | ✓ | — |
| `crh` | `crh` | Crimean Tatar Qırımtatarca | tr + long-press ñ â | LTR | **Full** (crh)⁷ | ✓ | — |
| `sco` | `sco` | Scots | gb | LTR | **Full** (sco)⁷ | ✓ | — |
| `rue` | `rue` | Rusyn Русиньскый | uk + long-press ы ъ ё | LTR | **Full** (rue)⁷ | — | — |
| `cv` | `cv` | Chuvash Чӑвашла | ru + long-press ӑ ӗ ҫ ӳ | LTR | **Full** (cv)⁷ | — | — |
| `sah` | `sah` | Sakha (Yakut) Саха тыла | ru + long-press ҕ ҥ ө һ ү | LTR | **Full** (sah)⁷ | — | — |
| `kpv` | `kpv` | Komi-Zyrian Коми кыв | ru + long-press і ӧ | LTR | **Full** (kpv)⁷ | — | — |
| `koi` | `koi` | Komi-Permyak Перым коми кыв | ru + long-press і ӧ | LTR | **Full** (koi)⁷ | — | — |
| `mhr` | `mhr` | Meadow Mari Олык марий | ru + long-press ӓ ҥ ӧ ӱ | LTR | **Full** (mhr)⁷ | — | — |
| `mrj` | `mrj` | Hill Mari Кырык мары | ru + long-press ӓ ӧ ӱ ӹ | LTR | **Full** (mrj)⁷ | — | — |
| `tyv` | `tyv` | Tuvan Тыва дыл | ru + long-press ң ө ү | LTR | **Full** (tyv)⁷ | — | — |
| `bxr` | `bxr` | Buryat Буряад хэлэн | ru + long-press ө ү һ | LTR | **Full** (bxr)⁷ | — | — |
| `xal` | `xal` | Kalmyk Хальмг келн | ru + long-press ә җ ң ө ү һ | LTR | **Full** (xal)⁷ | — | — |
| `myv` | `myv` | Erzya Эрзянь кель | ru | LTR | **Full** (myv)⁷ | — | — |
| `mdf` | `mdf` | Moksha Мокшень кяль | ru | LTR | **Full** (mdf)⁷ | — | — |
| `krl` | `krl` | Karelian Karjala | fi + long-press č š ž | LTR | **Full** (krl)⁷ | ✓ | — |
| `olo` | `olo` | Livvi-Karelian Livvi | fi + long-press č š ž | LTR | **Full** (olo)⁷ | ✓ | — |
| `vep` | `vep` | Veps Vepsän kel’ | et + long-press č š ž | LTR | **Full** (vep)⁷ | ✓ | — |
| `sme` | `sme` | Northern Sami Davvisámegiella | no · smi | LTR | **Full** (sme)⁷ | ✓ | — |
| `azb` | `azb` | South Azerbaijani تۆرکجه | fa + long-press ۆ ۇ ې | **RTL** | **Full** (azb)⁷ | — | — |
| `mzn` | `mzn` | Mazandarani مازرونی | fa | **RTL** | **Full** (mzn)⁷ | — | — |
| `kiu` | `kiu` | Kirmanjki (Northern Zazaki) Kirmanckî | tr + long-press ê î û | LTR | **Full** (kiu)⁷ | ✓ | — |
| `diq` | `diq` | Zazaki (Southern Zaza) Zazakî | tr + long-press ê î û | LTR | **Full** (diq)⁷ | ✓ | — |
| `tly` | `tly` | Talysh Tolışə zıvon | az | LTR | **Full** (tly)⁷ | ✓ | — |
| `kaa` | `kaa` | Karakalpak Qaraqalpaqsha | us + long-press á ǵ ı ń ó ú | LTR | **Full** (kaa)⁷ | ✓ | — |
| `ce` | `ce` | Chechen Нохчийн | ru + key Ӏ | LTR | **Full** (ce)⁷ | — | — |
| `inh` | `inh` | Ingush ГӀалгӀай | ru + key Ӏ | LTR | **Full** (inh)⁷ | — | — |
| `av` | `av` | Avar Авар мацӀ | ru + key Ӏ | LTR | **Full** (av)⁷ | — | — |
| `kbd` | `kbd` | Kabardian Къэбэрдейбзэ | ru + key Ӏ | LTR | **Full** (kbd)⁷ | — | — |
| `ady` | `ady` | Adyghe Адыгабзэ | ru + key Ӏ | LTR | **Full** (ady)⁷ | — | — |
| `lez` | `lez` | Lezgian Лезги чӀал | ru + key Ӏ | LTR | **Full** (lez)⁷ | — | — |
| `lbe` | `lbe` | Lak Лакку маз | ru + key Ӏ | LTR | **Full** (lbe)⁷ | — | — |
| `tab` | `tab` | Tabasaran Табасаран чӀал | ru + key Ӏ | LTR | **Full** (tab)⁷ | — | — |
| `dar` | `dar` | Dargwa Дарган мез | ru + key Ӏ | LTR | **Full** (dar)⁷ | — | — |
| `kum` | `kum` | Kumyk Къумукъ тил | ru | LTR | **Full** (kum)⁷ | — | — |
| `krc` | `krc` | Karachay-Balkar Къарачай-малкъар тил | ru | LTR | **Full** (krc)⁷ | — | — |
| `ab` | `ab` | Abkhaz Аԥсшәа | ru + long-press ә қ ҟ ԥ ҭ ҳ ҵ ҿ ҽ ҷ ҩ ӡ џ ҕ ӷ | LTR | **Full** (ab)⁷ | — | — |
| `xmf` | `xmf` | Mingrelian მარგალური | ka + long-press ჷ ჲ ჸ | LTR | **Full** (xmf)⁷ | — | — |
| `crs` | `crs` | Seychellois Creole Kreol Seselwa | us | LTR | **Full** (crs)⁷ | ✓ | — |
| `nd` | `nd` | Northern Ndebele Sindebele | us | LTR | **Full** (nd)⁷ | ✓ | — |
| `ng` | `ng` | Ndonga Oshindonga | us | LTR | **Full** (ng)⁷ | ✓ | — |
| `efi` | `efi` | Efik Efịk | us + long-press ọ ẹ ụ ị | LTR | **Full** (efi)⁷ | ✓ | — |
| `sg` | `sg` | Sango Sängö | us + long-press ë ê é â ä ô ö û ï î | LTR | **Full** (sg)⁷ | ✓ | — |
| `kj` | `kj` | Kwanyama Oshikwanyama | us | LTR | **Full** (kj)⁷ | ✓ | — |
| `lua` | `lua` | Luba-Kasai Tshiluba | us | LTR | **Full** (lua)⁷ | ✓ | — |
| `lu` | `lu` | Luba-Katanga Kiluba | us + long-press ā ū ō ē ñ | LTR | **Full** (lu)⁷ | ✓ | — |
| `kea` | `kea` | Cape Verdean Creole Kabuverdianu | us + long-press á ã â à é ê ó ô ç | LTR | **Full** (kea)⁷ | ✓ | — |
| `mos` | `mos` | Mossi Mòoré | us + long-press ã ẽ ɛ ɩ ĩ ʋ ũ õ | LTR | **Full** (mos)⁷ | ✓ | — |
| `rcf` | `rcf` | Réunion Creole Kréol rényoné | us + long-press é è | LTR | **Full** (rcf)⁷ | ✓ | — |
| `kg` | `kg` | Kongo Kikongo | us | LTR | **Full** (kg)⁷ | ✓ | — |
| `wal` | `wal` | Wolaytta Wolayttatto | us | LTR | **Full** (wal)⁷ | ✓ | — |
| `luo` | `luo` | Luo Dholuo | us | LTR | **Full** (luo)⁷ | ✓ | — |
| `tiv` | `tiv` | Tiv | us + long-press ô é | LTR | **Full** (tiv)⁷ | ✓ | — |
| `iso` | `iso` | Isoko | us + long-press ọ ẹ | LTR | **Full** (iso)⁷ | ✓ | — |
| `bem` | `bem` | Bemba Ichibemba | us | LTR | **Full** (bem)⁷ | ✓ | — |
| `fon` | `fon` | Fon Fɔngbè | us + long-press ɔ ó ò ɛ é è ě ɖ í ǐ ú ǔ á ǎ | LTR | **Full** (fon)⁷ | ✓ | — |
| `ach` | `ach` | Acholi Acoli | us + long-press ŋ | LTR | **Full** (ach)⁷ | ✓ | — |
| `bci` | `bci` | Baoulé Wawle | us + long-press ɛ ɔ ɲ | LTR | **Full** (bci)⁷ | ✓ | — |
| `dyu` | `dyu` | Jula Julakan | us + long-press ɛ ɔ ɲ ŋ | LTR | **Full** (dyu)⁷ | ✓ | — |
| `ki` | `ki` | Kikuyu Gĩkũyũ | us + long-press ĩ ũ | LTR | **Full** (ki)⁷ | ✓ | — |
| `kmb` | `kmb` | Kimbundu | us | LTR | **Full** (kmb)⁷ | ✓ | — |
| `umb` | `umb` | Umbundu | us + long-press ã ẽ ĩ ũ õ ñ | LTR | **Full** (umb)⁷ | ✓ | — |
| `la` | `la` | Latin Latina | us | LTR | **Full** (la)⁷ | ✓ | — |
| `pap` | `pap` | Papiamento Papiamentu | us + long-press ñ | LTR | **Full** (pap)⁷ | ✓ | — |
| `yua` | `yua` | Yucatec Maya Maaya tʼaan | us + key ʼ | LTR | **Full** (yua)⁷ | ✓ | — |
| `quc` | `quc` | Kʼicheʼ | us + key ʼ + long-press ä | LTR | **Full** (quc)⁷ | ✓ | — |
| `kek` | `kek` | Qʼeqchiʼ | us + key ʼ | LTR | **Full** (kek)⁷ | ✓ | — |
| `quz` | `quz` | Cusco Quechua Qusqu runasimi | us + key ʼ + long-press ñ | LTR | **Full** (quz)⁷ | ✓ | — |
| `qug` | `qug` | Kichwa (Chimborazo) Kichwa | us + long-press ñ | LTR | **Full** (qug)⁷ | ✓ | — |
| `srn` | `srn` | Sranan Tongo Sranantongo | us | LTR | **Full** (srn)⁷ | ✓ | — |
| `gcf` | `gcf` | Guadeloupean Creole Kréyòl gwadloupéyen | us + long-press é è ò | LTR | **Full** (gcf)⁷ | ✓ | — |
| `bi` | `bi` | Bislama | us | LTR | **Full** (bi)⁷ | ✓ | — |
| `ia` | `ia` | Interlingua | us | LTR | **Full** (ia)⁷ | ✓ | — |
| `tpi` | `tpi` | Tok Pisin | us | LTR | **Full** (tpi)⁷ | ✓ | — |
| `gil` | `gil` | Kiribati Taetae ni Kiribati | us | LTR | **Full** (gil)⁷ | ✓ | — |
| `tvl` | `tvl` | Tuvaluan Te Ggana Tuuvalu | us + long-press ā ē ī ō ū | LTR | **Full** (tvl)⁷ | ✓ | — |
| `ho` | `ho` | Hiri Motu | us | LTR | **Full** (ho)⁷ | ✓ | — |
| `ch` | `ch` | Chamorro Finuʼ Chamoru | us + long-press å ñ | LTR | **Full** (ch)⁷ | ✓ | — |
| `jam` | `jam` | Jamaican Patois Patwa | us | LTR | **Full** (jam)⁷ | ✓ | — |
| `kri` | `kri` | Krio | us + long-press ɛ ɔ ŋ | LTR | **Full** (kri)⁸ | ✓ | — |
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
| `km` | `km` | Khmer | kh | LTR | **Full** (km)⁷ | — | — |
| `lo` | `lo` | Lao | la | LTR | **Full** (lo)⁷ | — | — |
| `my` | `my` | Burmese | mm | LTR | **Full** (my)⁷ | — | — |
| `syr` | `syr` | Syriac | sy · syc | **RTL** | **Full** (syr)⁷ | — | — |
| `ber` | `ber` | Tamazight | ma · tifinagh | LTR | **Full** (ber)⁷ | — | — |
| `kab` | `kab` | Kabyle | us | LTR | **Full** (kab) | ✓ | — |
| `qu` | `qu` | Quechua | es | LTR | **Full** (qu)⁷ | ✓ | — |
| `gn` | `gn` | Guaraní | es + long-press á ã é ẽ í ĩ ó õ ú ũ ý ỹ g̃, ' key | LTR | **Full** (gn)⁷ | ✓ | — |
| `ay` | `ay` | Aymara | es | LTR | **Full** (ay)⁷ | ✓ | — |
| `sw` | `sw` | Swahili | us | LTR | **Full** (sw)⁷ | ✓ | — |
| `ms` | `ms` | Malay | us | LTR | **Full** (ms) | ✓ | — |
| `so` | `so` | Somali | us | LTR | **Full** (so)⁷ | ✓ | — |
| `zu` | `zu` | Zulu | us | LTR | **Full** (zu)⁷ | ✓ | — |
| `xh` | `xh` | Xhosa | us | LTR | **Full** (xh)⁷ | ✓ | — |
| `af` | `af` | Afrikaans | us | LTR | **Full** (af)⁷ | ✓ | — |
| `jv` | `jv` | Javanese | us | LTR | **Full** (jv)⁷ | ✓ | — |
| `su` | `su` | Sundanese | us | LTR | **Full** (su)⁷ | ✓ | — |
| `om` | `om` | Oromo | us | LTR | **Full** (om)⁷ | ✓ | — |
| `ceb` | `ceb` | Cebuano | es | LTR | **Full** (ceb)⁷ | ✓ | — |
| `ha` | `ha` | Hausa | us + long-press ɓ ɗ ƙ ƴ | LTR | **Full** (ha)⁷ | ✓ | — |
| `yo` | `yo` | Yoruba | us + long-press dot/tone | LTR | **Full** (yo)⁷ | ✓ | — |
| `ig` | `ig` | Igbo | us + long-press ị ọ ụ ṅ | LTR | **Full** (ig)⁷ | ✓ | — |
| `uz` | `uz` | Uzbek | us + long-press oʻ gʻ | LTR | **Full** (uz)⁷ | ✓ | — |
| `tk` | `tk` | Turkmen | us + long-press ä ç ž ň ö ş ü ý | LTR | **Full** (tk)⁷ | ✓ | — |
| `mr` | `mr` | Marathi | in · marathi | LTR | **Full** (mr)³ | — | — |
| `kok` | `kok` | Konkani कोंकणी | in · marathi | LTR | **Full** (kok)⁶ | — | — |
| `ne` | `ne` | Nepali | np | LTR | **Full** (ne)⁶ | — | — |
| `dty` | `dty` | Dotyali डोटेली | np | LTR | **Full** (dty)⁶ | — | — |
| `new` | `new` | Newari नेपाल भाषा | np | LTR | **Full** (new)⁶ | — | — |
| `bho` | `bho` | Bhojpuri भोजपुरी | in | LTR | **Full** (bho)⁶ | — | — |
| `mai` | `mai` | Maithili मैथिली | in | LTR | **Full** (mai)⁶ | — | — |
| `anp` | `anp` | Angika अंगिका | in | LTR | **Full** (anp)⁶ | — | — |
| `sa` | `sa` | Sanskrit संस्कृतम् | in + long-press ऽ ॐ | LTR | **Full** (sa)⁶ | — | — |
| `awa` | `awa` | Awadhi अवधी | in | LTR | **Full** (awa)⁶ | — | — |
| `as` | `as` | Assamese | in · ben (+ ৰ ৱ long-press) | LTR | **Full** (as)⁷ | — | — |
| `bo` | `bo` | Tibetan བོད་ཡིག | cn · tib | LTR | — | — | — |
| `dv` | `dv` | Dhivehi ދިވެހި | mv | RTL | **Full** (dv)⁷ | — | — |
| `sat` | `sat` | Santali ᱥᱟᱱᱛᱟᱲᱤ | in · sat | LTR | **Full** (sat)⁶ | — | — |
| `cy` | `cy` | Welsh Cymraeg | us + long-press ŵ ŷ â ê î ô û | LTR | **Full** (cy)⁷ | ✓ | — |
| `gd` | `gd` | Scottish Gaelic Gàidhlig | us | LTR | **Full** (gd)⁷ | ✓ | — |
| `eu` | `eu` | Basque Euskara | es | LTR | **Full** (eu) | ✓ | — |
| `gl` | `gl` | Galician Galego | es | LTR | **Full** (gl) | ✓ | — |
| `oc` | `oc` | Occitan | us + long-press ç à á è é í ï ò ó ú ü | LTR | **Full** (oc) | ✓ | — |
| `br` | `br` | Breton Brezhoneg | us + long-press ñ ç â ê î ô û ù | LTR | **Full** (br) | ✓ | — |
| `co` | `co` | Corsican Corsu | us | LTR | **Full** (co)⁷ | ✓ | — |
| `lb` | `lb` | Luxembourgish Lëtzebuergesch | us + long-press ä ë é ï ö ü | LTR | **Full** (lb)⁷ | ✓ | — |
| `rm` | `rm` | Romansh Rumantsch | us | LTR | **Full** (rm)⁷ | ✓ | — |
| `fy` | `fy` | Frisian Frysk | us + long-press â ê ë î ï ô û ú | LTR | **Full** (fy)⁷ | ✓ | — |
| `tt` | `tt` | Tatar Татарча | ru · tt | LTR | **Full** (tt) | — | — |
| `ba` | `ba` | Bashkir Башҡортса | ru · bak | LTR | **Full** (ba)⁷ | — | — |
| `os` | `os` | Ossetian Ирон | ru · os_winkeys | LTR | **Full** (os) | — | — |
| `udm` | `udm` | Udmurt Удмурт | ru · udm | LTR | **Full** (udm)⁷ | — | — |
| `wo` | `wo` | Wolof | us + long-press ñ ŋ ë ó | LTR | **Full** (wo)⁷ | ✓ | — |
| `ln` | `ln` | Lingala Lingála | us + long-press ɛ ɔ | LTR | **Full** (ln)⁷ | ✓ | — |
| `rw` | `rw` | Kinyarwanda | us | LTR | **Full** (rw)⁷ | ✓ | — |
| `rn` | `rn` | Kirundi | us | LTR | **Full** (rn)⁷ | ✓ | — |
| `sn` | `sn` | Shona chiShona | us | LTR | **Full** (sn)⁷ | ✓ | — |
| `ny` | `ny` | Chichewa | us + long-press ŵ | LTR | **Full** (ny)⁷ | ✓ | — |
| `mg` | `mg` | Malagasy | us + long-press ñ ô | LTR | **Full** (mg)⁷ | ✓ | — |
| `st` | `st` | Sesotho | us | LTR | **Full** (st)⁷ | ✓ | — |
| `tn` | `tn` | Setswana | us + long-press š | LTR | **Full** (tn)⁷ | ✓ | — |
| `nso` | `nso` | Sepedi | us + long-press š ê ô | LTR | **Full** (nso)⁷ | ✓ | — |
| `lg` | `lg` | Luganda | us + long-press ŋ ɲ | LTR | **Full** (lg)⁷ | ✓ | — |
| `ak` | `ak` | Akan | us + long-press ɛ ɔ | LTR | **Full** (ak)⁷ | ✓ | — |
| `ee` | `ee` | Ewe Eʋegbe | us + long-press ɖ ɛ ƒ ɣ ɔ ʋ ŋ | LTR | **Full** (ee)⁷ | ✓ | — |
| `bm` | `bm` | Bambara Bamanankan | us + long-press ɛ ɔ ɲ ŋ | LTR | **Full** (bm)⁷ | ✓ | — |
| `ff` | `ff` | Fula Fulfulde | us + long-press ɓ ɗ ƴ ŋ ñ ɲ | LTR | **Full** (ff)⁷ | ✓ | — |
| `ts` | `ts` | Tsonga Xitsonga | us | LTR | **Full** (ts)⁷ | ✓ | — |
| `ve` | `ve` | Venda Tshivenḓa | us + long-press ḓ ḽ ṋ ṱ | LTR | **Full** (ve)⁷ | ✓ | — |
| `ss` | `ss` | Swati siSwati | us | LTR | **Full** (ss)⁷ | ✓ | — |
| `nr` | `nr` | Ndebele isiNdebele | us | LTR | **Full** (nr)⁷ | ✓ | — |
| `mi` | `mi` | Māori | us + long-press ā ē ī ō ū | LTR | **Full** (mi)⁷ | ✓ | — |
| `haw` | `haw` | Hawaiian ʻŌlelo Hawaiʻi | us + key ʻ + long-press ā ē ī ō ū | LTR | **Full** (haw)⁷ | ✓ | — |
| `sm` | `sm` | Samoan Gagana Sāmoa | us + key ʻ + long-press ā ē ī ō ū | LTR | **Full** (sm)⁷ | ✓ | — |
| `to` | `to` | Tongan Lea faka-Tonga | us + key ʻ + long-press ā ē ī ō ū | LTR | **Full** (to)⁷ | ✓ | — |
| `fj` | `fj` | Fijian Vosa Vakaviti | us + long-press ā ē ī ō ū | LTR | **Full** (fj)⁷ | ✓ | — |
| `ty` | `ty` | Tahitian Reo Tahiti | us + key ʻ + long-press ā ē ī ō ū | LTR | **Full** (ty)⁷ | ✓ | — |
| `ht` | `ht` | Haitian Creole Kreyòl | us | LTR | **Full** (ht)⁷ | ✓ | — |
| `nah` | `nah` | Nahuatl Nāhuatl | us + long-press ā ē ī ō | LTR | — | ~ | — |
| `kl` | `kl` | Greenlandic Kalaallisut | us | LTR | **Full** (kl)⁷ | ✓ | — |
| `chr` | `chr` | Cherokee ᏣᎳᎩ | us · chr | LTR | — | — | — |
| `iu` | `iu` | Inuktitut ᐃᓄᒃᑎᑐᑦ | ca · ike | LTR | **Full** (iu)⁷ | — | — |
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
| `dty-abc` | `dty` | Dotyali डोटेली (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the dty lexicon choosing |
| `new-abc` | `new` | Newari नेपाल भाषा (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the new lexicon choosing |
| `anp-abc` | `anp` | Angika अंगिका (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the anp lexicon choosing |
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
   thirty times, Marathi's ten) for Hindi, Bengali and Marathi, whose
   lexicons widen (Bengali's and Marathi's to 80 000 words) while their
   keystroke savings on Tatoeba's held-out text rise (Hindi 48.2%, from
   46.6% on Tatoeba's alone; Bengali 43.4%, from 40.7%; Marathi 42.8%,
   from 42.6%). Counted ten times, as they were until data release
   0.24.0, Tatoeba's text led less: Hindi saved 45.9%, Bengali 42.5%, for
   transliteration's first reading on Dakshina's test sentences right 0.3
   and 0.1 points more often and its keystrokes saved some 1 and 0.2
   points more (their tables fitted again to the new lexicons; Hindi's
   now reads 2.1 points more of Dakshina's test words); Marathi's at
   thirty gained 0.6 points on Tatoeba's text and lost 1.7 of its
   transliteration's keystrokes, and stays at ten. Keystroke
   savings of the nine on held-out Wikipedia text: Punjabi 41.8%, Urdu
   41.8%, Sindhi 40.4%, Sinhala 36.1%, Tamil 34.5%, Gujarati 33.6%, Telugu
   32.8%, Malayalam 31.1%, Kannada 30.3%. Malayalam's old spellings, which
   its lexicon held some 1 700 words of (ൻറ for ന്റ, the two-part ൌ for
   ൗ, a last ു് for ്, stray zero-width joiners), are written as its
   keyboard types them (`osk_data::words`, generation 5): from 31.0%, its
   unknown words 22.8% -> 22.6%. The Dravidian languages' words
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
   strip saves typing them: Hindi 84.3% (87.9%; 23.3%), Urdu 83.7% (86.5%;
   23.6%), Punjabi 80.8% (85.3%; 25.1%), Gujarati 79.3% (82.7%; 25.4%),
   Marathi 78.1% (80.2%; 18.8%), Telugu 78.0% (80.2%; 21.3%), Kannada
   77.4% (79.3%; 21.0%), Bengali 75.8% (79.1%; 21.7%), Sinhala 75.4%
   (78.9%; 22.1%), Tamil 74.5% (76.5%; 18.8%), Sindhi 72.8% (77.0%; 20.9%),
   Malayalam 67.8% (71.3%; 16.8%), the English words the tables read
   (below) among them; of the words the lexicon has, 82–92%
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
   their dev words. Of the words people typed that the lexicons have, 6%
   (Hindi) to 16% (Bengali) were no reading of the table at all, which no
   ranking mends, and the ranking put the word first for all but 2.5 to 7
   points of the rest, the misses mostly homophones (Urdu's wa: و or وہ):
   a ranker learned from Dakshina's training words and dev sentences
   (`make fit-ranker`, `osk_data::translit::Ranker`, a weighed sum of each
   reading's likelihood, cost and context and a few tanh units of them)
   gained 0.2 points at most on the dev words and lost keystrokes on
   Urdu's test sentences, so no table has one. The tables gained the ways
   their misses asked for instead: Urdu's a doubled consonant written once
   (taraqqi: ترقی), the ع a vowel is said with (istemaal: استعمال), a last
   o's nasal (dono: دونوں), a last ay's ے (from 81.2% first), Punjabi's the
   addak before an aspirate (mukkh: ਮੁੱਖ), hn ਨ੍ਹ, aya ਿਆ, eh ਹਿ, nasal
   ends (toh: ਤੋਂ; from 77.4%), Sindhi's the nasal its pronouns write
   before their h (jahin, jenh: جنهن) and the hamza's seat (huwa: هئا; from
   70.2%), Bengali's nch ঞ্চ, ddh ধ, tt ত্ব and a hyphen typed before a
   suffix (bangladesh-er: বাংলাদেশের; from 72.5%); and one letter typed
   alone is read as its name too, an initial (k: കെ; Malayalam from
   63.3%). Lexicons of 200 000 words (from 120 000) then added
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
   those their lexicons have, the word comes first for Sanskrit 87.4%
   (93.9% among the strip's two), Meitei 86.8% (91.7%), Maithili 85.9%
   (92.8%), Konkani 82.2% (88.0%), Nepali 81.2% (88.5%), Odia 76.6%
   (84.2%), Assamese 76.0% (80.6%), Bodo 73.7% (81.9%), Dogri 61.0%
   (67.5%) and Kashmiri 50.4% (65.2%), the English words their tables
   read (below) among them. Assamese's lexicon, learned from
   FineWeb-2's text now (note 7), has 48% of its test words, from 17%,
   and of all of them puts 40.6% first, from 24.0% (of the few it had,
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
   Sixteen tables of languages neither dataset has are measured on
   Dakshina's romanizations of the words their lexicons share with
   Hindi's, Urdu's, Kannada's or Telugu's: two thirds to nine tenths of
   the words of its test sentences (Tulu's 45%), and how the people of a
   neighbouring language type them, the closest measure open data gives.
   Of those words, first: Tulu 92.9% (94.7% among the strip's two),
   Adilabad Gondi 89.9% (93.1%), Bhojpuri 88.4% (93.9%), Awadhi 87.3%
   (94.5%), Sindhi in Devanagari 86.6% (92.1%), Rajasthani 86.3% (93.7%),
   Garhwali 86.2% (94.1%), Angika 83.9% (91.7%), Newari 82.8% (90.1%),
   Western Punjabi 82.3% (90.8%), Chhattisgarhi 81.0% (93.9%), Dotyali
   80.5% (89.5%), Khowar 80.2% (88.2%), Magahi 79.9% (91.2%), Saraiki
   79.5% (87.8%) and Brahui 70.4% (85.0%), the English words their tables
   read (below) among them. Gondi's is Telugu's table and
   costs, Angika's Hindi's, Khowar's Urdu's with its own letters (ݯ ݮ ݰ ݱ
   beside چ ج ش ژ, څ ځ for ts and dz), Dotyali's and Newari's Nepali's,
   Dotyali's with the avagraha it writes after a long last vowel (rahya:
   रह्याऽ), Newari's with its visarga for h at a word's end or before a
   consonant (khah: खः, yahgu: याःगु). Thirteen were fitted to those
   romanizations' training words as Dakshina's own tables were (Sindhi's
   stayed as written, Gondi's and Angika's at Telugu's and Hindi's
   costs), a cost no shared word takes kept as written (Saraiki's
   implosives, Garhwali's and Rajasthani's ळ, Khowar's own letters,
   Newari's visarga, which those words would only push down): on their
   dev words Khowar rose 6.8 points so, Brahui 4.4, Western Punjabi 3.9,
   Chhattisgarhi 3.5, Saraiki 2.1, Rajasthani 1.5, Newari 1.3, Awadhi
   1.2, Dotyali 1.0, Bhojpuri 0.6, Garhwali 0.3, Magahi 0.3 (with its
   larger lexicon) and Tulu 0.2. Urdu's new ways (above) raised the
   tables made from its: on those test words Brahui from 67.0%, Khowar
   from 77.2%, Western Punjabi from 80.0%, Saraiki from 78.2%, fitted
   again. Yiddish's table is measured on
   Wiktionary's words and their romanizations, which are YIVO's, the
   one people type (14 010 words and forms, as kaikki.org extracts them;
   `scripts/fetch-wiktionary-romanized.sh`): the word first for 82.3%
   of them and 87.8% of those its lexicon has, a word right without its
   points (Wiktionary writes פֿינף, Yiddish's wiki and many of its
   writers פינף, which of the two comes first being the lexicon's).
   Hebrew's plural endings ים and ות for -em and -es (khsidem: חסידים)
   raised the half of the words they were not chosen on from 85.6% to
   87.6% of the words the lexicon has. Armenian's, Persian's, Pashto's,
   Mongolian's and Santali's are measured on Wiktionary's words too,
   whose romanizations are scholarly (Armenian's ղ ġ, Persian's خ x): each
   is folded to the letters its table reads, as its header says they are
   typed (ġ gh, x kh, č ch; `scripts/wiktionary-romanized.py`), so this
   measures how often the table and its lexicon find a word spelled as
   the table expects, not how people spell it. Of the words its lexicon
   has, first: Armenian 95.8% (19 123 words, the lexicon having 50% of
   them), Mongolian 94.2% (4 906; 54%), Santali 87.3% (787; 77%), Pashto
   75.1% (1 409; 68%) and Persian 74.7% (13 531, Wiktionary's Iranian
   romanization, the one Finglish follows; 77%), where most of the rest
   are words Finglish spells alike (safar: سفر, صفر). Persian's table now
   reads a consonant typed twice, as Wiktionary writes a doubled one, as
   the script writes it, once (mosallam: مسلم; from 75.4% with its old
   lexicon to 79.0%), and Pashto's writes ی for i and y inside a word,
   where ي and ی are one letter (73.3% to 75.1%). Meitei in the Bengali
   script has no such measure (Wiktionary has three of its words); its
   tests read everyday words. Of the 51 checked in the seven tables new with
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
   their script (school: स्कूल). And forty tables read the English words
   their languages write in their script as English spells them, which is
   how people type them (college: कॉलेज, police: পুলিশ, team: டீம்):
   `translit/<lang>.english.txt`, made from CMUdict's pronunciations
   (Carnegie Mellon's; BSD-2-Clause-style, `NOTICES.md`), each spelled in
   Latin letters as the language's sounds are typed (meediya, nyoo,
   sheyar), read by the table, and the likeliest reading its lexicon has
   kept, 6 000 to 24 000 a language (`make english-assets`). A word typed
   whole is read as its English word too, ranked with the other readings
   at a cost (`osk_data::translit::ENGLISH_COST`, 4 nats: the word of the
   language's own spelled so still wins, men: में), and from three letters
   the strip offers the words of the English words they begin. On the
   test sentences above they raised the words first 0.3 (Sindhi) to 2.1
   points (Telugu; Bengali and Kannada 1.9, Malayalam 1.7, Punjabi 1.2,
   Hindi and Marathi 1.1), the romanization lexicons' test words 0.5 to
   3.3, and the keystrokes saved up to 0.3; the sixteen tables measured on
   a neighbour's words 0.3 to 3.7 points (Gondi; Tulu 3.2), and the
   Aksharantar ten 1.7 to 8.1 of the words their lexicons have (Meitei;
   Nepali 5.4). A word English's lexicon lacks is left out where the
   table reads its letters as a word already: CMUdict says names and
   other languages' words too (hain, yah), which had cost Sindhi in
   Devanagari and Dotyali up to 1.5 points. Greek's, Persian's,
   Armenian's, Mongolian's, Santali's and Yiddish's tables read none:
   their measures hold no English words, and lost up to 0.8 points to the
   readings they added. An English word typed with what the language adds
   to it (collegenalli: ಕಾಲೇಜಿನಲ್ಲಿ, germanyile: ജർമനിയിലെ) is read as
   the word that writes the English word, the rest read on from it as the
   rules read after a consonant or a vowel; the English words are spelled
   with a last u too (court: కోర్టు), and two of the ways a language
   writes one are kept where both are likely (ಕಾಲೇಜು, ಕಾಲೇಜ್), the second
   costing 2 nats more (`osk_data::translit::LATER_ENGLISH`). Two letters
   the script would join are read apart with a zero-width non-joiner too,
   where the lexicon has a word so written (ಗುಜರಾತ್‌ನ, മുൻ‌നിര). And the
   tables of the four lowest gained the ways the readings closest to their
   misses asked for, chosen on the dev sentences: Malayalam's d and t for
   ഠ (padanam: പഠനം), th for ത്ഥ, ndh for ന്ത, ls for ത്സ (chikilsa:
   ചികിത്സ), ng for ങ്ക (engil: എങ്കിൽ), e for the vowel a consonant
   carries (sheri: ശരി), a last y for ി, e for ഈ and il for ൽ, and its
   last k as ക് (organik: ഓർഗാനിക്), which its k$ had hidden; Telugu's dh
   for థ (pradhamika: ప్రాథమిక), dd for ద్ధ, ai for ాయి, a last y for ి
   and e for the vowel a consonant carries (Gondi's with them); Tamil's nn
   and ll for ண and ள, ae for ெ and a first r for இர (ramasamy:
   இராமசாமி); Kannada's last h for the visarga (moolatah: ಮೂಲತಃ).
   Telugu's and Tamil's costs were fitted again. On the test sentences
   above the words first rose: Malayalam 2.3 points (from 65.5%; 0.2 of
   them its old spellings made one, note 3), Telugu
   1.7, Bengali and Tamil 0.5, Marathi 0.3, Kannada 0.2; keystrokes saved
   up to 0.4 more (Malayalam), none fewer. Of the 73 measures above, 15
   put up to 0.4 points fewer words first (Tamil's test words, Dotyali),
   and the strip's two hold 0.4 points more on average.
5. Hinglish is Hindi typed in Latin letters and left in them: the US
   keys, suggestions and glide from a model of Hindi's corpora respelled
   in the spellings people use, as often as each is used (the Dakshina
   dataset's romanization lexicons; `scripts/build-hinglish.py`).
   Keystroke savings 31.9% on 500 of Dakshina's romanized test sentences,
   as their writers spelled them, 13.4% of whose words the lexicon (60 000
   words) lacks: no two writers spell alike. For the same reason
   autocorrect changes 2.2% of right words, more than elsewhere.
6. These twenty-one models learn from their own Wikipedia (CC BY-SA 4.0,
   share-alike; `scripts/fetch-wikipedia.sh`, the wikis' CirrusSearch
   dumps of 2026-11-08): Bhojpuri, Maithili, Mongolian, Nepali, Odia,
   Western Punjabi, Pashto, Sanskrit, Santali, Yiddish, Konkani (the
   Goan wiki's Devanagari articles), Kashmiri, Meitei, Awadhi, Saraiki,
   Tulu, and Dotyali, Angika, Newari, Armenian and Persian (below).
   Keystroke savings on
   held-out articles of the first sixteen: Santali 50.2%, Kashmiri
   48.7%, Meitei 43.2%, Bhojpuri 43.0%, Maithili 42.1%, Pashto 41.3%,
   Yiddish 41.1%, Awadhi 40.2%, Nepali 40.0%, Odia 39.6%, Western
   Punjabi 38.6%, Mongolian 38.3%, Saraiki 37.4%, Tulu 31.6%, Konkani
   31.1%, Sanskrit 26.9%. The wikis' sentences are cut where a word in
   another script stands, the word left out (`wikipedia-text.py
   --script-only`): their references and English passages had put Latin
   words into their lexicons, two thirds of Nepali's 30 000 and three
   fifths of Odia's in place of their own words, a sixth to a fifth of
   Pashto's, Western Punjabi's, Maithili's, Sanskrit's and Mongolian's.
   On the held-out articles cut so, the models before saved Odia 36.7%,
   Bhojpuri 42.0%, Maithili 41.3%, Pashto 40.5%, Nepali 39.3%, Western
   Punjabi 38.1%, Mongolian 37.9% and Saraiki 37.2%, the others as now;
   the words unknown fell most in Odia's (19.4% -> 15.8%) and Pashto's
   (4.9% -> 3.7%). The transliteration tables then find more of the
   words people typed (Aksharantar's words first, of all of them:
   Nepali 29.3% -> 31.2%, Odia 26.6% -> 27.8%, Maithili 38.5% ->
   40.4%, Sanskrit 34.1% -> 36.5%), the lexicons holding more of them,
   rarer and harder; Bhojpuri's, Western Punjabi's, Saraiki's, Odia's and
   Sanskrit's costs were fitted again to the new lexicons (Sanskrit's dev
   words 53.8% -> 57.3%). Each sentence is learned once: wikis that grew from
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
   stubs a bot wrote (98% of its sentences fit 2 000 templates), and so is
   GlotCC's Bishnupriya (copies of it). Three wikis came later: Dotyali's
   (34 827 sentences from 4 129 articles: keystroke savings 34.8% on its
   held-out articles, 15.9% of the words unknown), Angika's (9 717
   sentences, 497 articles heavy with Hindi's own words left out: 45.2%,
   9.3% unknown, a third of its sentences being village statistics, which
   flatters it) and Newari's (230 401 sentences from 75 059 articles, half
   of them village stubs a bot wrote). Newari learns from web text too,
   news from GlotCC (CC0 1.0 for its packaging, the text under Common
   Crawl's Terms of Use; `scripts/fetch-glotcc.sh`, its file at one
   revision pinned by sha256): 24 984 sentences from 1 241 documents
   (nepalbhasatimes.com the most), a document kept where its own words
   (दु खः नं थ्व जूगु) are at least 1% and Hindi's and Nepali's under 6%,
   every 20th held out. The news counts five times (WEB_WEIGHT) and the
   lexicon keeps 80 000 words: keystroke savings on the held-out news
   42.1% (6.6% unknown; from the news alone 42.6%, 10.8% unknown), on
   the wiki's held-out articles 56.0% (from the wiki alone 57.4%, but
   23.5% on the news). Dotyali and Newari are typed on Nepali's keys,
   Angika on Hindi's. Armenian's and Persian's models learn from their
   wikis too, beside their Tatoeba sentences: the first 400 000
   sentences of each (eight and thirteen times Tatoeba's), Tatoeba's
   counted a hundred times so that everyday text leads the strip, and
   80 000 words: keystroke savings on Tatoeba's held-out sentences 46.6%
   -> 48.2% (Armenian) and 41.0% -> 41.9% (Persian), the words unknown
   4.8% -> 2.4% and 3.6% -> 2.3%, and the share of Wiktionary's words
   their lexicons have 20% -> 50% and 46% -> 77% (counted ten times, the
   savings fell two points). Their sentences are cut
   where a word in another script stands: the references' Latin
   (Wayback Machine, IMDb) was a tenth of their commonest words. Pashto's wiki writes ي
   inside a word as often as ی, which are one letter there (22 000 of
   its words both ways), and Arabic ك, Persian گ and ى for ک ګ ی: its
   words are read with them made one, as its keyboard types them
   (`osk_data::words`, generation 4): keystroke savings 39.4% -> 40.2%,
   its words unknown 6.5% -> 5.9%.
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
   sentences and Marwari's empty: none is used. Rangpuri's (2 118
   sentences once 196 in standard Bengali are left out, with infobox
   fragments among them) is not used yet.
7. These two hundred and two models learn from web text, FineWeb-2's (ODC-By 1.0,
   subject to Common Crawl's Terms of Use; `scripts/fetch-fineweb.sh`,
   the dataset's files at one revision pinned by sha256): Bodo, Dogri,
   Chhattisgarhi, Magahi, Brahui, Kokborok, Mizo and Khasi, which have
   no Wikipedia of their own, Sindhi in Devanagari and Meitei in the
   Bengali script, whose wikis are in their other scripts, and Lao, Khmer
   and Burmese, whose wikis are small or written as few type; and
   Assamese's beside its 14 000 Tatoeba sentences
   (counted thirty times; at ten its savings on them were 42.3%, now
   44.3%, on the web's held-out text 34.1%, now 33.5%), whose model
   lacked two fifths of the web's running words.
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
   which the layout's generator had left out. Khmer and Burmese are read
   as Lao is, cut with ICU's Khmer and Burmese word lists (Burmese's is
   LeRoy Benjamin Sharon's, under its BSD notice), every 40th row group
   of each (Khmer's 602 000 sentences of its 3 000 million characters,
   Burmese's 476 000 of 7 000 million), a sentence ending at ។ ៕ and ။
   too, and a zero-width space, which Khmer text often puts between
   words, parting them as a space does (`osk_data::words`). A word starts
   in neither on a consonant written below another (after Khmer's coeng ្,
   Burmese's virama ္), nor in Burmese on one its asat makes the last of
   a syllable (the န of ကန်). Burmese's web was written in Zawgyi, a
   font's own encoding, until 2019: two fifths of FineWeb-2's Burmese
   text (9% of 2024's), and a third of the documents read are left out
   as Zawgyi (ေ before its consonant, ္ ending a syllable, the letters at
   U+1060 to U+1097) or as Zawgyi read as Unicode (ငျ for င်), the
   keyboard and the word list being Unicode's; the digit ၀ written for
   the letter ဝ is read as ဝ. Keystroke savings on FineWeb-2's held-out
   text: Khmer 46.7% (9 434 words), Burmese 45.6% (8 494). Their keyboards
   now have the letters and signs xkeyboard-config keeps on the number
   row's Shift, a key each on the top row (Burmese's ရ, among its
   commonest letters, ဍ ၒ ဋ; Khmer's ៗ ៍ ័ ៏ ៌), and Khmer's those on
   AltGr on a long press (ឯ ឱ ឫ ឦ ៈ…): of their held-out text's letters,
   2.2% and 1.0% had no key before, none and 0.2% now. Thirty-eight
   languages that had a keyboard and no model learn from FineWeb-2's text
   too, from the first file of its train split, every Nth row group (some
   100 million characters each), English sentences left out (Russian ones
   for Kazakh, Kyrgyz, Tajik and Bashkir); each sample's commonest words
   are its own language's. Their lexicons keep 80 000 words, 120 000 for
   the eleven whose words take most forms (Kazakh, Bosnian, Slovenian,
   Uzbek, Xhosa, Zulu, Shona, Luxembourgish, Dhivehi, Irish, Bashkir).
   Keystroke savings on all of FineWeb-2's held-out text of each (5 000 to
   17 000 words; Irish's first 3 000 are an Old Irish tale): Maltese
   53.2%, Sesotho 53.1%, Xhosa 51.4%, Hausa 51.1%, Malagasy 50.4%, Welsh
   50.2%, Yoruba 49.4%, Scottish Gaelic 49.1%, Igbo 49.0%, Chichewa 49.0%,
   Haitian Creole 48.6%, Cebuano 48.4%, Samoan 47.7%, Swahili 47.4%, Māori
   47.3%, Turkmen 46.5%, Javanese 45.5%, Hawaiian 45.1%, Dhivehi 45.0%,
   Tajik 45.0%, Somali 45.0%, Sundanese 44.8%, Frisian 44.5%, Kinyarwanda
   44.3%, Zulu 43.9%, Bashkir 43.6%, Afrikaans 43.1%, Corsican 42.6%, Irish
   42.6%, Albanian 42.4%, Kyrgyz 42.3%, Uzbek 41.5%, Shona 40.8%,
   Luxembourgish 40.6%, Faroese 39.5%, Slovenian 36.0%, Kazakh 35.9%,
   Bosnian 33.6%; 40.2% on average with taps astray, their words unknown
   1% to 10%, at most 0.75% of right words changed (Hawaiian's). Maltese's
   articles and elided prepositions glue the word after them on, as
   French's l' does (il-kelb, b'mod), and Irish's t- and n- before a vowel
   (t-athair); Uzbek's oʻ and gʻ are one spelling, whichever apostrophe a
   text writes for the ʻ (`osk_data::words`). Central Kurdish (Sorani) is
   typed on xkeyboard-config's Sorani keys (`iq(ku_ara)`), its ێ ڵ ڕ,
   which they put on Shift, on the long presses of ی ل ر too. Every 10th
   row group of its FineWeb-2 text is read. A third of it is in the old
   Arabic-keyboard spelling, which writes ە as ه (ئهو, دهکات), ه being h
   as well, and cannot be read back: a document in it is left out, and a
   sentence of it in a document that is not. The letters those keyboards
   typed (ك ي ى ھ) are read as the ones its keyboard types (ک ی ی ه), and
   a word's last ه as ە but after ا (شاه) and in الله (in text otherwise
   Unicode's, half the words ending in ه have their ە form commoner:
   لاپەڕه, ئیسلامەوه); 465 760 sentences from 36 798 of 56 000 documents.
   Its lexicon keeps 120 000 words, its words taking their prepositions
   and endings with them: keystroke savings 33.6% on FineWeb-2's held-out
   text (12 060 words, 10.4% unknown; with 80 000 words 33.2%, 12.2%),
   23.6% with taps astray (Arabic's, on Tatoeba's sentences, 33.6% and
   27.8%). Nigerian Pidgin,
   typed on the US keys, learns from FineWeb-2's text and GlotCC's (note
   6), what each files under it being poster shops, porn and English as
   well: a document is kept where its own words (di dey wey dem na don)
   are at least 5% of its words and more than English's (the is was
   are), 876 and 790 documents, 16 579 and 6 980 sentences (540 000
   words), GlotCC's those FineWeb-2's lacks; a third is the Bible in
   Pidgin, the rest news, sport and lyrics. Its Wikipedia, written in the
   Naija Langwej Akedemi's spelling (tek, afta, ples, where the web writes
   take, after, place), is not read. Keystroke savings 44.0% on the
   held-out text of both (397 sentences, 9 708 words, 5.7% unknown),
   39.1% with taps astray, its lexicon's 26 565 words all its text has.
   Nineteen languages of Southeast Asia learn from FineWeb-2's text whole
   (Hmong Daw's every sixth row group): Hmong Daw, Hakha Chin, Jingpho,
   Hiligaynon, Iloko, Central Bikol, Pangasinan, Kapampangan, Kinaray-a,
   Iban, Batak Toba, Karo, Simalungun, Acehnese, Balinese, Banjar,
   Kadazandusun, Tetum and Iu Mien, from 0.7 million words (Kadazandusun)
   to 23 million (Hakha Chin). A sentence of the big language written
   beside each is left out: English's, and Tagalog's in the Philippines'
   (ng ay hindi), Indonesian's or Malay's in Indonesia's and Borneo's
   (dengan untuk tidak), Portuguese's in Tetum, Vietnamese's in Iu Mien.
   Four fifths of FineWeb-2's Balinese is a dictionary site's entries,
   whose labels (In English, In Balinese) leave a sentence out; Tetum's
   saltillo (ꞌ) written for the apostrophe of ne'e is read as the
   apostrophe. FineWeb-2's Betawi is everyday Indonesian, its Musi porn
   spam, its Banjar in the Arabic script Malaysia's Jawi, its Chavacano
   Spanish lyrics: none is used. They are typed on the US keys, with ñ
   on n's long press for the Philippines' languages and Tetum, Acehnese's
   é ë è ô ö, Balinese's é and Hakha Chin's ṭ first in theirs. Their
   lexicons keep 80 000 words (Iloko's and Balinese's 120 000; at 30 000
   they saved 0.3 to 2.5 points fewer). Keystroke savings on FineWeb-2's
   held-out text of each (5 700 to 19 800 words): Iu Mien 63.4%,
   Kadazandusun 56.8%, Simalungun 55.7%, Tetum 55.3%, Hiligaynon 54.0%,
   Acehnese 53.8%, Karo 53.3%, Jingpho 53.2%, Iban 51.6%, Pangasinan
   50.7%, Hmong Daw 48.3%, Central Bikol 48.3%, Kinaray-a 47.2%, Batak
   Toba 46.6%, Hakha Chin 46.6%, Iloko 46.3%, Banjar 44.7%, Kapampangan
   43.3%, Balinese 40.6%; 46.8% on average with taps astray, their words
   unknown 0.5% to 7.9%. Most of Iu Mien's text, and of Acehnese's,
   Pangasinan's, Karo's and Simalungun's, is the Bible, as their held-out
   text is, which flatters them. With taps astray Hmong Daw's corrector
   changes 1.0% of the words typed right (0.3% when taps land true), its
   words told apart by their last letter, which writes the tone (kev,
   kej); 96.6% are right after the space. Shan, Mon and Sgaw Karen, in
   the Myanmar script, write their syllables run together between
   spaces, as Burmese does, and have no word list to cut them with: not
   yet used.
   Twenty-six of Europe's regional languages learn from FineWeb-2's text
   whole (Asturian's every second row group, Nynorsk's every 28th):
   Norwegian Nynorsk, Asturian, Aragonese, Ladino, Alemannic, Bavarian,
   Low German, Palatine German, Kölsch, Limburgish, Upper and Lower
   Sorbian, Neapolitan, Sicilian, Sardinian, Friulian, Piedmontese,
   Ligurian, Walloon, Latgalian, Võro, Kashubian, Silesian, Crimean
   Tatar, Scots and Rusyn, from 1.4 million words (Lower Sorbian) to 21
   million (Low German). Their text mixes with their national
   language's, which writes its own words for theirs (Bokmål's ikke jeg
   hun for Nynorsk's ikkje eg ho, German's ist nicht for Bavarian's is
   ned): a sentence more than 15% the national language's, or English's,
   is left out, and for the dialects mixed with it most (Neapolitan and
   Sicilian with Italian, the German dialects with German, Limburgish
   with Dutch, Scots with English) a document whose own words are under
   1% of its words or not more than the national language's (nearly half
   of Neapolitan's documents, three tenths of Limburgish's, a fifth of
   Bavarian's, Low German's and Sicilian's). Much
   of their text is their wikis' pages, whose section-editing links
   ([editar], [ännern | Bornkood ännern]) go; Neapolitan's has Roman's
   songs and comments in it (er pane, vojo), left out by Roman's article,
   Võro's a dictionary's entries, Crimean Tatar's escort spam. Each is
   typed on its national language's keys, its own letters first on the
   long presses (the table above). Their lexicons keep 80 000 words, or
   120 000 for the nineteen whose lexicons lacked 6.9% (Nynorsk) to 21%
   (Kashubian) of their held-out words at 80 000 (0.3 to 0.9 points more
   saved). Keystroke savings on
   FineWeb-2's held-out text of each (6 700 to 14 000 words): Crimean
   Tatar 47.5%, Asturian 43.7%, Aragonese 43.2%, Walloon 41.4%, Ligurian
   41.0%, Nynorsk 40.6%, Piedmontese 40.0%, Upper Sorbian 39.5%, Lower
   Sorbian 39.5%, Limburgish 38.6%, Ladino 37.9%, Scots 37.6%, Low German
   37.6%, Latgalian 36.5%, Friulian 36.5%, Sardinian 36.1%, Alemannic
   35.0%, Võro 33.6%, Sicilian 31.9%, Rusyn 31.3%, Kölsch 30.9%, Silesian
   29.8%, Bavarian 29.6%, Kashubian 28.7%, Neapolitan 27.3%, Palatine
   German 27.0%; 28.5% on average with taps astray, their words unknown
   4.9% to 18.5% (Kashubian's), at most 0.66% of right words changed
   (Sicilian's). The dialects with no spelling of their own save the
   least: each writer spells as they speak. A letter only a long press
   types is one eval-predict counts as having no key (Kashubian's 14%,
   the Sorbians' 8–9%, as Yoruba's 18%), so their savings undercount
   what a long press gives. FineWeb-2's Zeelandic (Middle Dutch from
   text archives), West Flemish (half Dutch Low Saxon's wiki), Picard (a
   dictionary's glosses in French), Lombard (half Ladin), Gagauz (Kazakh,
   and Cyrillic letters for Latin ones) and Venetian (spam) are not used
   yet.
   Sixteen of Russia's and the north's learn from FineWeb-2's text whole
   (Sakha's and Chuvash's every other row group): Sakha, Chuvash,
   Northern Sami, Meadow and Hill Mari, Udmurt, Tuvan, Buryat,
   Komi-Zyrian, Komi-Permyak, Erzya, Moksha, Kalmyk, Karelian, Livvi and
   Veps; and twenty-two more that had a keyboard and no model: Tongan,
   Fijian, Tahitian, Ewe, Venda, Tsonga, Lingala, Romansh, Akan (Twi's
   text), Sepedi, Kirundi, Setswana, Southern Ndebele, Oromo, Bambara,
   Luganda, Quechua (Ayacucho's), Wolof, Greenlandic, Swati, Aymara and
   Inuktitut. A sentence of the big language written beside each is left
   out (Russian's, Norwegian's, Danish's, German's, French's, Spanish's,
   English's). FineWeb-2 has chat sites' machine-made pages in the Maris'
   and Udmurt's name (half of Hill Mari's documents): a document read at
   such an address is left out. Letters typed for others are read as the
   keyboard types them: Chuvash's Latin ă ĕ ç as ӑ ӗ ҫ (вăл, вӑл), the
   Komis' and Maris' Latin ö ä ÿ as ӧ ӓ ӱ, Buryat's and Kalmyk's fita ѳ as
   ө, Bambara's Greek ε as ɛ, and a Polynesian glottal stop written as an
   apostrophe or a quote before a vowel (fa'a, ‘o) as ʻ. Nearly a third
   of FineWeb-2's Akan was typed in an old Ghanaian font that put ɛ on j and ɔ
   on c (yj, sj for yɛ, sɛ): left out, as Zawgyi's Burmese is, it took
   Akan's savings from 16.6% to 47.0% and its unknown words from 26% to
   1.4%. Russia's languages written in Cyrillic are typed on Russian's
   keys with their letters on the long presses of those xkeyboard-config's
   keymaps put them over (Chuvash's ӑ ӗ ҫ ӳ on а е с у…), Northern Sami on
   its Norwegian keymap, Karelian and Livvi on Finnish's keys and Veps on
   Estonian's with č š ž. Tongan, Tahitian, Hawaiian and Samoan have a key
   for the glottal stop, ʻ, which their keyboards lacked: Hawaiian's
   letters with no key fell from 9.5% to 4.3%, its savings rose 45.1% ->
   46.1%, Samoan's 47.7% -> 48.7%. Their lexicons keep 80 000 words, 120 000
   for fourteen and 200 000 for the five that build words from many parts
   (Greenlandic, Inuktitut, Swati, Aymara, Southern Ndebele). Keystroke
   savings on FineWeb-2's held-out text of each (2 000 to 15 000 words):
   Hill Mari 50.3%, Kalmyk 44.2%, Moksha 42.6%, Meadow Mari 42.6%, Chuvash
   42.4%, Tuvan 42.0%, Erzya 41.0%, Northern Sami 40.6%, Karelian 40.5%,
   Komi-Zyrian 40.1%, Veps 39.5%, Buryat 38.2%, Komi-Permyak 38.2%, Udmurt
   38.1%, Sakha 37.7%, Livvi 34.1%; Tongan 53.9%, Fijian 52.9%, Tahitian
   52.3%, Ewe 52.2%, Venda 51.9%, Tsonga 51.2%, Lingala 49.3%, Romansh
   48.8%, Akan 47.0%, Sepedi 46.7%, Kirundi 45.6%, Setswana 45.5%, Southern
   Ndebele 44.9%, Oromo 42.2%, Bambara 41.9%, Luganda 41.0%, Quechua 40.8%,
   Wolof 34.2%, Greenlandic 33.5%, Swati 28.8%, Aymara 28.0%, Inuktitut
   23.4%; at most 0.66% of right words changed (Wolof's). Greenlandic,
   Inuktitut, Swati and Aymara still lack a fifth to a third of their
   held-out words, a sentence's worth of word parts each, and with taps
   astray save little (Greenlandic 8.2%, Inuktitut 0.5%): a word typed
   wrong costs its deleting and retyping, which a keyboard with no model
   pays for every such word, and these strips and corrections take back
   only some of it. Most of Ewe's, Tsonga's, Sepedi's, Setswana's,
   Southern Ndebele's, Tahitian's and Fijian's text is the Bible and
   Watchtower, as their held-out text is. FineWeb-2's Nahuatl is three
   quarters Eastern Huasteca Nahuatl's Bible, not the language the
   keyboard of Nahuatl at large types: not used.
   Nineteen of Iran's and the Caucasus' learn from FineWeb-2's text whole:
   South Azerbaijani, Mazandarani, Kirmanjki and Zazaki, Talysh,
   Karakalpak (in Latin letters, its alphabet since 2016), Chechen,
   Ingush, Avar, Kabardian, Adyghe, Lezgian, Lak, Tabasaran, Dargwa,
   Kumyk, Karachay-Balkar, Abkhaz and Mingrelian. A sentence of the big
   language beside each is left out (Russian's, Persian's, Arabic's in
   Persian letters, Turkish's and Kurmanji's, Azerbaijani's, Georgian's),
   as are the Zazakis' spam, Lak's and Tabasaran's news sites' Russian
   page furniture, and a site of Arabic and Persian religious books that
   FineWeb-2 files under South Azerbaijani and Mazandarani. The
   palochka, Ӏ, which the Caucasus' alphabets write and keyboards without
   it type as 1, I, i, l, |, і or ι (д1а, гьечiо: a digit would cut the
   word in two), is read as Ӏ beside a Cyrillic letter, and the nine that
   write it (Chechen, Ingush, Avar, Kabardian, Adyghe, Lezgian, Lak,
   Tabasaran, Dargwa) have a key for it at their home row's end, on
   Russian's keys; Kumyk and Karachay-Balkar are typed on Russian's keys
   as they are, Abkhaz with its letters on long presses (ә on а, ԥ on п,
   ҭ on т…). Karakalpak's letters as its 1994 alphabet wrote them (ha'm)
   are read as 2016's (hám). Their lexicons keep 80 000 words, 120 000 for
   ten and 200 000 for Kabardian and Mingrelian. Keystroke savings on
   FineWeb-2's held-out text of each (2 200 to 8 500 words): Tabasaran
   55.2%, Abkhaz 51.8%, Adyghe 50.4%, Dargwa 45.3%, Lak 44.5%, Kumyk 44.0%,
   Lezgian 42.7%, Avar 41.4%, Chechen 41.1%, Karachay-Balkar 38.9%,
   Mazandarani 38.7%, Karakalpak 38.3%, Ingush 36.7%, Mingrelian 36.5%,
   Kirmanjki 36.0%, South Azerbaijani 35.5%, Talysh 35.2%, Zazaki 32.0%,
   Kabardian 30.8%; 34.0% on average with taps astray, their words unknown
   4.6% to 18.3%, at most 0.56% of right words changed (Mazandarani's).
   Tabasaran's and Mazandarani's held-out text is small (2 500 and 2 200
   words). FineWeb-2's Southern Kurdish is Sorani's news, its Gilaki
   Persian shops' spam and its Gorani mostly Sorani: none is used.
   Twenty-four of Africa's learn from FineWeb-2's text whole, those it has
   2 MB or more of: Seychellois Creole, Northern Ndebele, Ndonga, Efik,
   Sango, Kwanyama, Tshiluba, Kiluba, Cape Verdean Creole, Mossi, Réunion
   Creole, Kikongo, Wolaytta, Dholuo, Tiv, Isoko, Bemba, Fon, Acholi,
   Baoulé, Jula, Kikuyu, Kimbundu and Umbundu, each typed on the US keys
   with its letters first on the long presses (the table above). A
   sentence of English is left out, and of French, Portuguese or Spanish
   beside those whose state writes it. Most of their text is the Bible and
   Watchtower (Kiluba's, Isoko's and Efik's nearly all), and so is their
   held-out text, whose words their lexicons nearly all have (0.3% to 5%
   unknown): their savings say how well a strip offers a text it has read
   much like, more than everyday typing. Seychellois Creole's text is its
   National Assembly's records, Kikuyu's Kenya's news. Their lexicons keep
   80 000 words, Northern Ndebele's and Cape Verdean's 120 000. Keystroke
   savings on FineWeb-2's held-out text of each (6 500 to 12 000 words):
   Baoulé 61.4%, Kikongo 60.0%, Sango 59.6%, Efik 58.1%, Kwanyama 57.9%,
   Umbundu 57.8%, Isoko 56.8%, Kimbundu 55.9%, Fon 55.4%, Jula 55.1%,
   Kikuyu 54.0%, Seychellois Creole 53.8%, Tiv 53.3%, Tshiluba 52.8%,
   Kiluba 52.3%, Ndonga 51.2%, Mossi 50.8%, Acholi 49.4%, Bemba 49.0%,
   Réunion Creole 48.6%, Northern Ndebele 47.8%, Dholuo 47.6%, Wolaytta
   45.7%, Cape Verdean Creole 29.3% (its text spelled both as ALUPEC and
   as Portuguese, 10.4% of its words unknown); at most 0.59% of right
   words changed (Seychellois Creole's). FineWeb-2 has under 2 MB of the
   other Africa's languages Gboard has, mostly Bible pages, and none of
   Krio, Kanuri, Mandinka or Tumbuka.
   Eighteen of the Americas', the Pacific's and the classical and
   constructed languages' learn from FineWeb-2's text whole (Latin's every
   44th row group, Papiamento's every 5th): Latin, Papiamento, Syriac
   (FineWeb-2's Classical Syriac, for the Syriac keyboard, which had no
   model), Yucatec Maya, Kʼicheʼ, Qʼeqchiʼ, Cusco Quechua, Kichwa, Sranan
   Tongo, Guadeloupean Creole, Bislama, Interlingua, Tok Pisin, Kiribati,
   Tuvaluan, Hiri Motu, Chamorro and Jamaican Patois, each but Syriac typed
   on the US keys with its letters first on the long presses (the table
   above). A sentence of English is left out, of Spanish beside the Mayan
   languages and the Quechuas, of Spanish and Dutch beside Papiamento, of
   Dutch beside Sranan Tongo, of French beside Guadeloupean Creole. A third
   of FineWeb-2's Latin is Lorem ipsum, web pages' placeholder: a document
   holding it is left out. Yucatec Maya, Kʼicheʼ, Qʼeqchiʼ and Cusco
   Quechua write a glottal stop or an ejective as ʼ, a letter, which a key
   at the end of their home row types; an apostrophe, a quote or a saltillo
   after a letter in their text (k'o, saˈ) is read as it, and their words
   keep it (since 0.39.0: their first models read it as an apostrophe,
   U+0027, so the strip offered k'iin to keys that type kʼ, and the
   keystrokes it saves were counted as if ' were typed from the symbols'
   view, 2.4 to 2.6 points more than the key saves). Syriac's vowel
   points, which its Bibles write and few type, are left out. FineWeb-2's
   Kichwa is a Bible site's pages, a dozen languages' verses among them: a
   document is kept where Kichwa's own words (mana alli chai ima) are 3% of
   its words, and a sentence where e and o are no more than a quarter of
   its vowels (Kichwa writes a i u) and none of Twi's ɛ ɔ is written. Their
   lexicons keep 80 000 words, Cusco Quechua's 120 000 and Latin's, whose
   words take many forms, 200 000 (the held-out words it lacks 15.2% ->
   8.7%). Keystroke savings on FineWeb-2's held-out text of each (2 500 to
   14 900 words): Hiri Motu 66.1%, Tok Pisin 63.1%, Bislama 58.6%, Sranan
   Tongo 58.4%, Qʼeqchiʼ 58.2%, Kʼicheʼ 55.7%, Yucatec Maya 54.4%, Tuvaluan
   53.3%, Kiribati 52.4%, Chamorro 50.3%, Interlingua 49.0%, Cusco Quechua
   43.1%, Kichwa 42.9%, Guadeloupean Creole 42.1%, Papiamento 40.1%, Syriac
   35.5%, Jamaican Patois 35.5%, Latin 26.8%; 45.1% on average with taps
   astray, their words unknown 0.2% to 12.5%, at most 0.76% of right words
   changed (Jamaican Patois's, whose held-out text is small: 2 500
   words). FineWeb-2's Fiji Hindi is Hindi in Latin
   letters, Hinglish's: not used. Nor is its Lojban, whose short words
   differ by a letter (lo le la): autocorrect changed 1.5% of the words
   typed right.
   Four keyboards that had no model learn from FineWeb-2's text whole
   (Serbian's every 18th row group): Serbian in Latin letters, Guaraní,
   Fula and Tamazight in Tifinagh. Serbian's Latin keyboard has a model of
   its own, sr-Latn, its words in Latin letters (Serbian's, sr, reads Latin
   as Cyrillic, and would offer Cyrillic to the Latin keys): a document of
   300 letters written without č ć š ž đ is left out (956 of 33 000), and a
   sentence with a Cyrillic letter, or with è æ ð ţ ĉ, an old code page's č
   ć đ read as another's; 1.2 million sentences. Guaraní's text writes its
   nasal vowels and g̃ with a tilde, a circumflex or a diaeresis (peteĩ,
   peteî, peteï; hag̃ua, haĝua), read as the tilde, and its glottal stop,
   the puso, as ', ’ or ʼ; a sentence of Spanish (its words, but y, la and
   para, which Guaraní writes too) or of English is left out. Its keyboard
   gains the stressed vowels á é í ó ú ý first on the long presses (its
   text writes them more often than the nasal ones) and a key for the puso
   at the end of the home row. Fula's is the text of Pular, Maasina
   Fulfulde and the Fulfuldes of Nigeria and Western Niger together, typed
   on one keyboard, which gains ñ and ɲ on n's long press (Pulaar's and
   Pular's ny); a font that put ɓ ɗ on å ð, and ñ written ň, are read as
   theirs, and a sentence of English or French is left out. Tamazight's is
   Central Atlas and Standard Moroccan Tamazight's in Tifinagh, a third of
   it the Algerian press agency's: a run of another script's words is cut
   out, a sentence with ⵧ ⴵ ⵒ, which its keyboard does not type, left out,
   and a clitic is glued on at its hyphen as in Kabyle (ⴷ-ⵓⵙⵉⵖ). Their
   lexicons keep 120 000 words, Tamazight's its 73 000. Keystroke savings
   on FineWeb-2's held-out text of each (5 400 to 11 200 words): Guaraní
   45.6%, Fula 40.9%, Serbian in Latin letters 39.7%, Tamazight 39.4%; with
   taps astray 38.9%, 34.9%, 36.7% and 33.8%, their words unknown 2.0%
   (Serbian's) to 8.0%, at most 0.38% of right words changed (Tamazight's).
   Bodo's apostrophe, which
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
8. These three models learn from web text, MADLAD-400's (Google; CC BY 4.0
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
   tables are Hindi's, with ळ for l at a small cost. Rajasthani's text
   is Marwari's, the Rajasthani most written (its री रो सूं, म्हारो), and
   its pack is named so, "Rajasthani (Marwari)": no source here has
   Marwari apart from it (its test wiki in the Incubator is empty), so a
   pack of its own would be the same files.
   Krio's is MADLAD-400's too, FineWeb-2 having none of it. Half the
   documents MADLAD-400 files as Krio are Australian Kriol's Bible (langa
   blanga detlot), so a document is kept where Krio's own words (dɛn fɔ
   wetin gɛt) are 3% of its words and more than Kriol's: 970 of its 39 900
   documents, clean and noisy, 34 000 sentences, most of them the Jehovah's
   Witnesses' pages (Jiova and Baybul are among its commonest words); ε and
   כ written for ɛ ɔ are read as them, a parallel corpus's markup is left
   out, and every 10th document is held out. It is typed on the US keys
   with ɛ ɔ ŋ first on the long presses; its lexicon has 6 800 words.
   Keystroke savings on the held-out documents (9 300 words): 57.2%, 56.0%
   with taps astray, 0.6% of their words unknown, the text much like what
   it read.

## Prediction model tiers

- **Full n-gram model** — `predict/<lang>.fst` +
  `<lang>.ngram.bin`, **290 languages** (Hinglish among them). The original nine (en,
  de, es, fr, it, nl, pl, pt, sv) plus 40 from Tatoeba: no da fi cs tr vi
  el ru uk ar he fa · ro hu sk hr et is ca eo bg sr mk be · id fil az ku
  ug hi bn hy ka lt lv · kab (Kabyle) · **Tier E: oc gl br eu** (Occitan,
  Galician, Breton, Basque) · **corpus audit: tt mr as ms os** (Tatar 92k,
  Marathi 86k, Assamese 15k, Malay 8k, Ossetian 7k — layout-only languages
  that turned out to have large Tatoeba corpora) · **th** (Thai, 6.5k,
  cut into words with ICU's word list, as it has no spaces between
  them) · **Wikipedia text: gu kn ml pa sd si ta te ur** (the Dakshina
  dataset's, which hi bn mr also learn from; note 3) · **the wikis' own
  text: bho mai mn ne or pnb ps sa sat yi kok ks mni awa skr tcy dty anp
  new** (Newari with GlotCC's news too), **hy fa** beside Tatoeba's, and
  the Incubator's test wikis: **wsg khw** (note 6) ·
  **web text: brx doi hne mag brh sd-Deva mni-Beng lo trp lus kha km my**, and of
  thirty-eight languages that had a keyboard and no model, **sw af sl zu sq kk uz bs
  ky tg cy so ga mt ha lb mg rw gd fy jv tk ht ba xh fo ceb mi su co ny sm sn ig st
  haw yo dv**, **ckb pcm**, and of Southeast Asia **mww cnh kac hil ilo bcl pag pam krj iba bbc
  btx bts ace ban bjn dtp tet ium**, of Europe **nn ast an lad gsw bar nds pfl ksh li hsb dsb
  nap scn sc fur pms lij wa ltg vro csb szl crh sco rue**, of Russia and the north **sah cv sme
  mhr mrj udm tyv bxr kpv koi myv mdf xal krl olo vep**, and twenty-three more that had a
  keyboard and no model, **kl om rm rn lg ln fj ee ak ts nso tn to wo ty ve nr ay ss qu bm iu**,
  of Iran and the Caucasus **azb mzn kiu diq tly kaa ce inh av kbd ady lez lbe tab dar kum krc ab xmf**,
  of Africa **crs nd ng efi sg kj lua lu kea mos rcf kg wal luo tiv iso bem fon ach bci dyu ki kmb umb**,
  of the Americas, the Pacific and the classical and constructed **la pap syr yua quc kek quz qug srn
  gcf bi ia tpi gil tvl ho ch jam**, and four more that had a keyboard and no model, **sr-Latn gn ff ber**
  (FineWeb-2's, Nigerian Pidgin's with GlotCC's; note 7;
  Magahi with its Incubator text too; Lao, Khmer and Burmese cut into words with ICU's word lists too), **as** (FineWeb-2's beside Tatoeba's), **gbm raj kri** (MADLAD-400's; note 8). Built by
  `make fetch-corpora` + `make predict-assets` in the
  language-content repository; runtime-loaded from `/usr/share/ki-osk/predict/`.
- **Layout-only** — no bundled model yet: 4 layouts, Cherokee, whose
  text the web has little of (MADLAD-400's clean text, 190 000 words, is
  its wiki's and its New Testament), Nahuatl, whose is colonial
  documents, its wiki and its varieties' Bibles, each spelled its own
  way, and the two written without spaces
  whose word lists ICU lacks (Tibetan, Dzongkha). The recognizer runs but
  has no vocabulary for them. Sixty-six layout-only languages got their
  models from FineWeb-2's web text (note 7).
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
  own Noto Sans Syriac face; it predicts from FineWeb-2's Classical
  Syriac, note 7) and `ber` Tamazight in Tifinagh (Noto Sans Tifinagh;
  it predicts from FineWeb-2's text, note 7); `kab` Kabyle in Latin, which *does* carry a
  full prediction model (Tatoeba has ~790k Kabyle sentences).
- **South America**: `qu` Quechua, `gn` Guaraní, `ay`
  Aymara — Latin on the Spanish (`es`) base; all three predict from
  FineWeb-2's text (note 7).
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
| `gn` Guaraní | á é í ó ú ý, then ã ẽ ĩ õ ũ ỹ · g̃ (ñ is on the es base); the puso, ', a key of its own at the home row's end | `gn` |
| `ay` Aymara | ä ï ü | `ay` |
| `ha` Hausa | ɓ ɗ ƙ ƴ (hooked b/d/k/y) | `ha` |
| `yo` Yoruba | ẹ ọ ṣ + á à é è tone marks | `yo` |
| `ig` Igbo | ị ọ ụ ṅ | `ig` |
| `uz` Uzbek | oʻ gʻ (turned comma) | `uz` |
| `tk` Turkmen | ä ç ž ň ö ş ü ý | `tk` |
| `as` Assamese | ৰ ৱ (on Bengali র/ব) | `as` |
| `ckb` Central Kurdish | ێ ڵ ڕ (on ی ل ر; on Shift too) | `ckb` |
| `cnh` Hakha Chin | ṭ | `cnh` |
| `ace` Acehnese | é ë è ô ö | `ace` |
| `ban` Balinese | é | `ban` |
| `hil` `ilo` `bcl` `pag` `pam` `krj` `tet` | ñ (Hiligaynon, Iloko, Central Bikol, Pangasinan, Kapampangan, Kinaray-a, Tetum) | each its own |
| `gsw` `bar` `li` `hsb` `dsb` `scn` `fur` `pms` `lij` `wa` `ltg` `vro` `csb` `szl` `crh` `rue` | each language's own letters first on its national layout's long presses (the language table above lists them: Upper Sorbian's ć č ě ł ń ó ř š ž ź on German's keys, Rusyn's ы ъ ё on Ukrainian's…) | each its own |
| `haw` `sm` `to` `ty` | ʻ, the glottal stop, a key of its own at the home row's end | each its own |
| `ce` `inh` `av` `kbd` `ady` `lez` `lbe` `tab` `dar` | Ӏ, the palochka, a key of its own at the home row's end (on Russian's keys) | each its own |
| `yua` `quc` `kek` `quz` | ʼ, the glottal stop and ejective (Kʼicheʼ, kʼo), a key of its own at the home row's end | each its own |
| `cv` `sah` `kpv` `koi` `mhr` `mrj` `tyv` `bxr` `xal` `ab` | their letters on the long presses of the Russian letters they are made from (Chuvash's ӑ ӗ ҫ ӳ on а е с у, Abkhaz's ә on а…) | each its own |

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
