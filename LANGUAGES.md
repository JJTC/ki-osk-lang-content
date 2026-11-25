# Supported languages & layouts

What the language packs hold per language: the keyboard layouts, text
direction, predictive-text tier, swipe typing, and composer (CJK,
Ge'ez, transliteration). Each
layout is a file, `layouts/<id>.json`, in its language's pack (the Pack
column below, from `packs.toml`'s `[layouts]`), beside that language's
prediction and composer data.

**727 layouts**: 649 keyboard layouts (RTL: ar he fa + Urdu, Western
Punjabi, Saraiki, Kashmiri, Brahui, Khowar, Pashto, Sindhi, Uyghur,
Central and Southern Kurdish, South Azerbaijani, Mazandarani, Western Balochi, Gilaki, Gorani, Syriac, Dhivehi, Yiddish, and with no model Balti, Gujari, the two Kohistanis, Hindko, Pothwari, Shina, Kohistani Shina, Torwali, Balochi, Hazaragi, Dhatki, Parkari Koli, Bakhtiari, Northern and Southern Luri, Lari, Kuhmareyi, Takestani, Maba, Mehri, Southeastern and Southwestern Pashayi, Wanetsi, Southern Hindko, Laki, Qashqai, Khorasani Turkic, Southern and Eastern Balochi, Dari, Dakhini, in the Syriac script Chaldean Neo-Aramaic and, in the Hanifi script, Rohingya; Hinglish, Kokborok) · 15 IMEs (5 CJK +
Amharic/Tigrinya/Ge'ez/Gedeo/Hadiyya/Kambaata/Silt'e/Sebat Bet Gurage Ge'ez + Vai's and Nuosu's syllables) · 61 transliteration layouts (Latin letters →
the language's script, `-abc`) · emoji + numeric. All render native
keycaps. 726 are here; the numeric keypad is built into ki-osk. The
table below is the complete list; **460 languages** carry a prediction
model (suggestion bar, plus dictionary-backed swipe on the Latin ones),
the rest are layout-only, gated only by how much permissively-licensed
corpus exists (see [Prediction model tiers](#prediction-model-tiers)).

**A device has the layouts of the packs it installs** — `de` for German's,
`sw` for Swahili's — see [Choosing languages](#choosing-languages). Their
runtime fonts load lazily on first use.

Legend — **Prediction**: `Full` = bundled FST + n-gram model · `—` = none
(no suggestions) · `IME` = candidate bar.
**Swipe**: `✓` Latin + full dictionary · `—` off
(no model, non-Latin scripts, IMEs, emoji/numeric).

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
| `ps` | `ps` | Pashto | af · ps + long-press ۀ | **RTL** | **Full** (ps)⁶ | — | — |
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
| `tdx` | `tdx` | Tandroy-Mahafaly Malagasy | mg + long-press à è é ò ì ỳ, ' key | LTR | **Full** (tdx)⁸ | ✓ | — |
| `hvn` | `hvn` | Hawu | us + long-press è, ꞌ key | LTR | **Full** (hvn)⁸ | ✓ | — |
| `ktu` | `ktu` | Kituba | us | LTR | **Full** (ktu)⁸ | ✓ | — |
| `quy` | `quy` | Ayacucho Quechua Runa Simi | Quechua's keys (ñ) + long-press á é | LTR | **Full** (quy)⁸ | ✓ | — |
| `myx` | `myx` | Masaaba Lumasaaba | us + long-press ŋ, ' key | LTR | **Full** (myx)¹³ | ✓ | — |
| `tuv` | `tuv` | Turkana Ng’aturkana | us, ' key | LTR | **Full** (tuv)¹³ | ✓ | — |
| `kbp` | `kbp` | Kabiye Kabɩyɛ | us + long-press ɛ ɩ ɔ ʊ ɖ ɣ ŋ ñ, ɛ ɩ keys, ʊ ɔ on q x | LTR | **Full** (kbp)⁷ | ✓ | — |
| `dag` | `dag` | Dagbani Dagbanli | us + long-press ɛ ɔ ŋ ɣ ʒ | LTR | **Full** (dag)⁷ | ✓ | — |
| `seh` | `seh` | Sena Cisena | us | LTR | **Full** (seh)⁷ | ✓ | — |
| `ada` | `ada` | Dangme | us + long-press ɛ ɔ | LTR | **Full** (ada)⁷ | ✓ | — |
| `nyn` | `nyn` | Nkore Runyankore | us | LTR | **Full** (nyn)⁷ | ✓ | — |
| `kwy` | `kwy` | Koongo Kisikongo | us | LTR | **Full** (kwy)⁷ | ✓ | — |
| `sid` | `sid` | Sidamo Sidaamu Afoo | us | LTR | **Full** (sid)⁷ | ✓ | — |
| `koo` | `koo` | Konzo Olukonzo | us | LTR | **Full** (koo)⁷ | ✓ | — |
| `kam` | `kam` | Kamba Kĩkamba | us + long-press ĩ ũ | LTR | **Full** (kam)⁷ | ✓ | — |
| `sop` | `sop` | Songe Kisonge | us | LTR | **Full** (sop)⁷ | ✓ | — |
| `vmw` | `vmw` | Makhuwa Emakhuwa | us | LTR | **Full** (vmw)⁷ | ✓ | — |
| `ndc` | `ndc` | Ndau ChiNdau | us | LTR | **Full** (ndc)⁷ | ✓ | — |
| `gur` | `gur` | Farefare Gurenɛ | us + long-press ɛ ɔ ŋ | LTR | —¹⁹ | — | — |
| `bum` | `bum` | Bulu | us + long-press é ô ñ | LTR | **Full** (bum)⁷ | ✓ | — |
| `bin` | `bin` | Edo Ẹdo | us + long-press ẹ ọ | LTR | **Full** (bin)⁷ | ✓ | — |
| `teo` | `teo` | Teso Ateso | us + long-press ŋ | LTR | **Full** (teo)⁷ | ✓ | — |
| `sgc` | `sgc` | Kipsigis | us | LTR | **Full** (sgc)⁷ | ✓ | — |
| `mfq` | `mfq` | Moba | us + long-press ɛ ɔ ŋ | LTR | —¹⁹ | — | — |
| `yao` | `yao` | Yao Chiyao | us + long-press ŵ | LTR | **Full** (yao)⁷ | ✓ | — |
| `dik` | `dik` | Dinka Thuɔŋjäŋ | us + long-press ä ë ï ö ɛ ɔ ɛ̈ ɔ̈ ŋ ɣ | LTR | **Full** (dik)⁷ | ✓ | — |
| `min` | `min` | Minangkabau Baso Minang | us | LTR | **Full** (min)⁷ | ✓ | — |
| `war` | `war` | Waray Winaray | us | LTR | **Full** (war)⁷ | ✓ | — |
| `nia` | `nia` | Nias Li Niha | us + long-press ö ŵ | LTR | **Full** (nia)⁷ | ✓ | — |
| `mad` | `mad` | Madurese Basa Madhura | us + long-press â è é ḍ ṭ | LTR | **Full** (mad)⁷ | ✓ | — |
| `cor` | `cor` | Cornish Kernewek | gb + long-press à â ê ë è î ò ô ù û ü ÿ | LTR | **Full** (cor)⁷ | ✓ | — |
| `glv` | `glv` | Manx Gaelg | gb + long-press ç á é í ó ú | LTR | **Full** (glv)⁷ | ✓ | — |
| `ext` | `ext` | Extremaduran Estremeñu | es + long-press ç | LTR | **Full** (ext)⁷ | ✓ | — |
| `pcd` | `pcd` | Picard | fr-azerty + long-press ò | LTR | **Full** (pcd)⁷ | ✓ | — |
| `sgs` | `sgs` | Samogitian Žemaitėška | lt + long-press ā ē ī ō | LTR | **Full** (sgs)⁷ | ✓ | — |
| `pnt` | `pnt` | Pontic Greek Ποντιακά | el | LTR | **Full** (pnt)⁷ | — | — |
| `vec` | `vec` | Venetian Vèneto | it + long-press ł | LTR | **Full** (vec)⁷ | ✓ | — |
| `vls` | `vls` | West Flemish West-Vlams | nl + long-press ê è ë é ô | LTR | **Full** (vls)⁷ | ✓ | — |
| `lmo` | `lmo` | Lombard Lumbard | it + long-press ò ö ó ü ù | LTR | **Full** (lmo)⁷ | ✓ | — |
| `lld` | `lld` | Ladin | it + long-press ë é è ê ö ò ó ô ü ù ć à á ś | LTR | **Full** (lld)⁷ | ✓ | — |
| `gag` | `gag` | Gagauz Gagauzça | tr + long-press ä ê ţ | LTR | **Full** (gag)⁷ | ✓ | — |
| `bn` | `bn` | Bengali | in · ben + long-press ৎ ঃ | LTR | **Full** (bn)³ | — | — |
| `ta` | `ta` | Tamil | in · tam | LTR | **Full** (ta)³ | — | — |
| `te` | `te` | Telugu | in · tel | LTR | **Full** (te)³ | — | — |
| `kn` | `kn` | Kannada | in · kan | LTR | **Full** (kn)³ | — | — |
| `tcy` | `tcy` | Tulu ತುಳು | in · kan | LTR | **Full** (tcy)⁶ | — | — |
| `wsg` | `wsg` | Gondi గోండి (Adilabad) | in · tel | LTR | **Full** (wsg)⁶ | — | — |
| `ml` | `ml` | Malayalam | in · mal | LTR | **Full** (ml)³ | — | — |
| `gu` | `gu` | Gujarati | in · guj | LTR | **Full** (gu)³ | — | — |
| `pa` | `pa` | Punjabi | in · guru | LTR | **Full** (pa)³ | — | — |
| `or` | `or` | Odia | in · ori + long-press ୟ ୱ | LTR | **Full** (or)⁶ | — | — |
| `si` | `si` | Sinhala | lk · sin_phonetic | LTR | **Full** (si)³ | — | — |
| `km` | `km` | Khmer | kh | LTR | **Full** (km)⁷ | — | — |
| `lo` | `lo` | Lao | la | LTR | **Full** (lo)⁷ | — | — |
| `my` | `my` | Burmese | mm | LTR | **Full** (my)⁷ | — | — |
| `shn` | `shn` | Shan တႆး | mm · shn | LTR | **Full** (shn)⁷ | — | — |
| `mnw` | `mnw` | Mon မန် | mm · mnw | LTR | **Full** (mnw)⁷ | — | — |
| `ksw` | `ksw` | Sgaw Karen ကညီ | Keyman's sil_sgaw_karen | LTR | **Full** (ksw)⁷ | — | — |
| `blk` | `blk` | Pa'O ပအိုဝ်ႏ | mm, ꩻ ႏ for ဩ ၏ | LTR | **Full** (blk)⁷ | — | — |
| `nan` | `nan` | Southern Min Bân-lâm-gú | us + long-press á â ā à a̍ ê ē… o͘ ⁿ | LTR | **Full** (nan)⁷ | ✓ | — |
| `hak` | `hak` | Hakka Hak-kâ-ngî | us + long-press â à á a̍ ê è… ṳ | LTR | **Full** (hak)⁷ | ✓ | — |
| `ami` | `ami` | Amis Pangcah | us, ' key | LTR | **Full** (ami)⁷ | ✓ | — |
| `za` | `za` | Zhuang Vahcuengh | us | LTR | **Full** (za)⁷ | ✓ | — |
| `zne` | `zne` | Zande Pazande | us + long-press ã á ẽ ĩ í õ ũ ú | LTR | **Full** (zne)⁷ | ✓ | — |
| `alz` | `alz` | Alur | us, ' key | LTR | **Full** (alz)⁷ | ✓ | — |
| `cce` | `cce` | Chopi | us + long-press ñ | LTR | **Full** (cce)⁷ | ✓ | — |
| `skg` | `skg` | Sakalava Malagasy Sakalava | us + long-press à è é ì ò | LTR | **Full** (skg)⁷ | ✓ | — |
| `bss` | `bss` | Akoose | us + long-press á â ǎ ā é ɛ ɛ́ ē ê ə…, é á keys, ɛ ʼ on q x | LTR | **Full** (bss)⁷ | ✓ | — |
| `men` | `men` | Mende Mɛnde | us + long-press ɛ ɔ ŋ | LTR | **Full** (men)⁷ | ✓ | — |
| `lgg` | `lgg` | Lugbara Lugbarati | us + long-press ᴜ ᴜ́ ú ɨ ɨ́ ɨ̀ í ì á à…, ' key | LTR | **Full** (lgg)⁷ | ✓ | — |
| `xon` | `xon` | Konkomba | us + long-press ɔ ŋ | LTR | **Full** (xon)⁷ | ✓ | — |
| `dnj` | `dnj` | Dan | us + long-press ë ɛ ö ɔ ü ʋ̈ ʋ ɩ | LTR | —¹⁹ | — | — |
| `gbo` | `gbo` | Northern Grebo Grebo | us + long-press à ǎ ä ɛ ě è ɛ̌ ɛ̀ ɛ̈ ǐ…, ɛ ɔ keys, ǒ ě on q x | LTR | —¹⁹ | — | — |
| `heh` | `heh` | Hehe Kihehe | us | LTR | **Full** (heh)⁷ | ✓ | — |
| `gux` | `gux` | Gourmanché Gulmancema | us + long-press ŋ ñ | LTR | **Full** (gux)⁷ | ✓ | — |
| `gmv` | `gmv` | Gamo | us | LTR | **Full** (gmv)⁷ ¹⁸ | ✓ | — |
| `suk` | `suk` | Sukuma Kisukuma | us + long-press ī ū, ' key | LTR | **Full** (suk)⁷ | ✓ | — |
| `spp` | `spp` | Supyire | us + long-press à á â ɛ è ɛ̀ é ê ɛ̂ ɛ́…, ' key | LTR | **Full** (spp)⁷ | ✓ | — |
| `bas` | `bas` | Basaa | us + long-press é è ñ ô à | LTR | **Full** (bas)⁷ | ✓ | — |
| `hay` | `hay` | Haya Ekihaya | us, ' key | LTR | —¹⁹ | — | — |
| `kbr` | `kbr` | Kafa | us, ' key | LTR | —¹⁹ | — | — |
| `ngl` | `ngl` | Lomwe Elomwe | us + long-press á à ã é ó ò | LTR | **Full** (ngl)⁷ | ✓ | — |
| `gog` | `gog` | Gogo Cigogo | us | LTR | —¹⁹ | — | — |
| `chw` | `chw` | Chuwabu Echuwabo | us + long-press á ã ç é í ñ ó | LTR | **Full** (chw)⁷ | ✓ | — |
| `fat` | `fat` | Fante Mfantse | us + long-press ɛ ɔ õ ã ũ | LTR | **Full** (fat)⁷ | ✓ | — |
| `tsc` | `tsc` | Tswa Xitshwa | us | LTR | **Full** (tsc)⁷ | ✓ | — |
| `bud` | `bud` | Ntcham Ncam | us + long-press á à ā b́ b̀ é è ē í ì… | LTR | —¹⁹ | — | — |
| `kao` | `kao` | Xaasongaxango | us + long-press ɲ ŋ ń í é, ' key | LTR | —¹⁹ | — | — |
| `gvl` | `gvl` | Gulay | us + long-press è é ê à á ɓ ɗ m̂ ɔ ò…, ' key | LTR | —¹⁹ | — | — |
| `nnb` | `nnb` | Nande Kinande | us + long-press ĩ í į ũ ú ų á é ó, ĩ ' keys, ũ on q | LTR | —¹⁹ | — | — |
| `nyo` | `nyo` | Nyoro Runyoro | us, ' key | LTR | —¹⁹ | — | — |
| `niu` | `niu` | Niuean Vagahau Niuē | us + long-press ā ē ō ū | LTR | **Full** (niu)⁷ | ✓ | — |
| `hwc` | `hwc` | Hawaiian Pidgin Pidgin | us | LTR | **Full** (hwc)⁷ | ✓ | — |
| `acf` | `acf` | Saint Lucian Creole Kwéyòl | us + long-press é è ò | LTR | **Full** (acf)⁷ | ✓ | — |
| `mrw` | `mrw` | Maranao Meranaw | us + long-press ā ū ī | LTR | **Full** (mrw)⁷ | ✓ | — |
| `nij` | `nij` | Ngaju | us | LTR | **Full** (nij)⁷ | ✓ | — |
| `guc` | `guc` | Wayuu Wayuunaiki | us + long-press ü ú á é í ó ñ, ü ' keys | LTR | **Full** (guc)⁷ | ✓ | — |
| `ljp` | `ljp` | Lampung Api Lampung | us | LTR | —¹⁹ | — | — |
| `sas` | `sas` | Sasak Basa Sasak | us + long-press ẽ | LTR | —¹⁹ | — | — |
| `gor` | `gor` | Gorontalo Hulontalo | us, ' key | LTR | **Full** (gor)⁷ | ✓ | — |
| `itv` | `itv` | Itawit | us | LTR | —¹⁹ | — | — |
| `bku` | `bku` | Buhid | us | LTR | —¹⁹ | — | — |
| `bpr` | `bpr` | Koronadal Blaan Blaan | us + long-press à é è ê ì ò ù | LTR | —¹⁹ | — | — |
| `quh` | `quh` | South Bolivian Quechua Qhichwa | us + long-press ñ á é í ó ú, ʼ key | LTR | **Full** (quh)⁷ | ✓ | — |
| `gcr` | `gcr` | Guianan Creole Kriyòl | us + long-press é è ò à ï | LTR | **Full** (gcr)⁷ | ✓ | — |
| `tum` | `tum` | Tumbuka Chitumbuka | us + long-press ŵ | LTR | **Full** (tum)⁷ | ✓ | — |
| `tll` | `tll` | Tetela Otetela | us + long-press ɛ ɔ, ' key | LTR | **Full** (tll)⁷ | ✓ | — |
| `guz` | `guz` | Gusii Ekegusii | us, ' key | LTR | **Full** (guz)⁷ | ✓ | — |
| `kde` | `kde` | Makonde Chimakonde | us + long-press á à â é ê è í î ì ô… | LTR | **Full** (kde)⁷ | ✓ | — |
| `nyy` | `nyy` | Nyakyusa Ikinyakyusa | us + long-press ị í ị̄ ī ụ ū ụ̄ ú ā ē… | LTR | **Full** (nyy)⁷ | ✓ | — |
| `mas` | `mas` | Maasai Maa | us + long-press á â é ê í ó ô ú | LTR | **Full** (mas)⁷ | ✓ | — |
| `mnk` | `mnk` | Mandinka | us + long-press ŋ ñ ǹ ì à é m̀ | LTR | **Full** (mnk)⁷ | ✓ | — |
| `mer` | `mer` | Meru Kĩmĩĩrũ | us + long-press ĩ ũ | LTR | **Full** (mer)⁷ | ✓ | — |
| `dga` | `dga` | Dagaare | us + long-press ɛ ē é ɛ̃ ŋ ɔ õ ɔ̃ ã ŭ… | LTR | **Full** (dga)⁷ | ✓ | — |
| `lhu` | `lhu` | Lahu Laˇhuˍ | us, its tones on keys: ˬ ˇ at the home row's end, ꞈ ' ˍ on q x z, long-press ˆ ˉ | LTR | —¹⁹ | — | — |
| `hrx` | `hrx` | Hunsrik | us + long-press ë ã á ó ç | LTR | **Full** (hrx)⁷ | ✓ | — |
| `crk` | `crk` | Plains Cree nēhiyawēwin | us + long-press ā ē ī ō, ā ē keys | LTR | **Full** (crk)⁷ | ✓ | — |
| `csw` | `csw` | Swampy Cree Ininîmowin | us + long-press â ê î ô š, â ê keys | LTR | **Full** (csw)⁷ | ✓ | — |
| `otw` | `otw` | Ottawa Nishnaabemwin | us, ' key | LTR | **Full** (otw)⁷ | ✓ | — |
| `tkl` | `tkl` | Tokelauan Gagana Tokelau | us + long-press ā ē ī ō ū | LTR | **Full** (tkl)⁷ | ✓ | — |
| `kne` | `kne` | Kankanaey | us | LTR | **Full** (kne)⁷ | ✓ | — |
| `mqj` | `mqj` | Mamasa | us, ' key | LTR | **Full** (mqj)⁷ | ✓ | — |
| `liv` | `liv` | Livonian Līvõ kēļ | lv + long-press õ ō ȯ ȭ ȱ ä ǟ ḑ ț ŗ, õ key | LTR | **Full** (liv)⁷ | ✓ | — |
| `abs` | `abs` | Ambonese Malay Bahasa Ambon | us | LTR | **Full** (abs)⁷ | ✓ | — |
| `ajg` | `ajg` | Aja Ajagbe | us + long-press ɛ é è ɔ ó ɖ ŋ í ì ʒ…, ɔ ɛ keys | LTR | **Full** (ajg)⁷ | ✓ | — |
| `yrk` | `yrk` | Nenets Ненэцяˮ вада | ru, ӈ in place of ъ, ' ˮ keys | LTR | **Full** (yrk)⁷ | — | — |
| `dng` | `dng` | Dungan Хуэйзў йүян | ru + long-press ү ў, ә in place of ъ, җ key | LTR | **Full** (dng)⁷ | — | — |
| `tsg` | `tsg` | Tausug Bahasa Sūg | us + long-press ī ā ū, ' key | LTR | **Full** (tsg)⁷ | ✓ | — |
| `ewo` | `ewo` | Ewondo | us + long-press é ë è ə ê á à ò ó ô ɔ í ï ú ñ ŋ ṅ | LTR | **Full** (ewo)⁷ | ✓ | — |
| `ttj` | `ttj` | Tooro Rutooro | us + long-press ŋ, ' key | LTR | **Full** (ttj)⁷ | ✓ | — |
| `mgh` | `mgh` | Makhuwa-Meetto Emeetto | us | LTR | **Full** (mgh)⁷ | ✓ | — |
| `knc` | `knc` | Kanuri | us + long-press ə, ə key | LTR | **Full** (knc)⁷ | ✓ | — |
| `nyf` | `nyf` | Giryama Kigiryama | us + long-press v̱, ' key | LTR | **Full** (nyf)⁷ | ✓ | — |
| `laj` | `laj` | Lango Lëblaŋo | us + long-press ŋ | LTR | **Full** (laj)⁷ | ✓ | — |
| `hnn` | `hnn` | Hanunoo Hanunuo | us + long-press ê | LTR | **Full** (hnn)⁷ | ✓ | — |
| `sda` | `sda` | Toraja-Sa'dan Basa Toraya | us, ' key | LTR | **Full** (sda)⁷ | ✓ | — |
| `lew` | `lew` | Ledo Kaili Kaili Ledo | us | LTR | **Full** (lew)⁷ | ✓ | — |
| `sdc` | `sdc` | Sassarese Sassaresu | it, ' key | LTR | **Full** (sdc)⁷ | ✓ | — |
| `rmy` | `rmy` | Vlax Romani Romani čhib (Vlax) | us + long-press š č ć ž đ | LTR | **Full** (rmy)⁷ | ✓ | — |
| `rmn` | `rmn` | Balkan Romani Romani čhib (Balkan) | rs · latin | LTR | **Full** (rmn)⁷ | ✓ | — |
| `ojb` | `ojb` | Northwestern Ojibwe ᐊᓂᔑᓇᐯᒧᐎᓐ | Keyman's fv_ojibwa, long-press its long vowels and w-dots | LTR | **Full** (ojb)⁷ | — | — |
| `cnr` | `cnr` | Montenegrin Crnogorski | rs · latin + long-press ś ź | LTR | **Full** (cnr)⁶ | ✓ | — |
| `cnr-cyrl` | `cnr` | Montenegrin Црногорски (Cyrillic) | rs + long-press с́ з́ | LTR | — | — | — |
| `bto` | `bto` | Rinconada Bikol | us + long-press ñ | LTR | **Full** (bto)⁶ | ✓ | — |
| `jax` | `jax` | Jambi Malay Bahaso Jambi | us | LTR | **Full** (jax)⁶ | ✓ | — |
| `mfp` | `mfp` | Makassar Malay Bahasa Melayu Makassar | us | LTR | **Full** (mfp)⁶ | ✓ | — |
| `xmm` | `xmm` | Manado Malay Bahasa Manado | us | LTR | **Full** (xmm)⁶ | ✓ | — |
| `mui` | `mui` | Musi Baso Pelémbang | us + long-press é è ó | LTR | **Full** (mui)⁶ | ✓ | — |
| `mfa` | `mfa` | Kelantan-Pattani Malay Kècèk Nnayu Klatae-Ttaning | us + long-press è é ó ũ | LTR | **Full** (mfa)⁶ | ✓ | — |
| `mak` | `mak` | Makassarese Basa Mangkasarak | us, ' key | LTR | **Full** (mak)⁶ | ✓ | — |
| `pmf` | `pmf` | Pamona Basa Pamona | us, ' key | LTR | **Full** (pmf)⁶ | ✓ | — |
| `ibb` | `ibb` | Ibibio | us + long-press ọ ñ n̄ ị ẹ ụ ʌ | LTR | **Full** (ibb)⁶ | ✓ | — |
| `vmf` | `vmf` | East Franconian Fränggisch | de + long-press â ô | LTR | **Full** (vmf)⁶ | ✓ | — |
| `sdh` | `sdh` | Southern Kurdish کوردی خوارین | iq · ku_ara + long-press ێ ڵ ڕ ۊ | **RTL** | **Full** (sdh)⁶ | — | — |
| `bgn` | `bgn` | Western Balochi بلوچی رخشانی | ir + long-press ے ں ٹ ڈ ڑ ہ ھ ګ | **RTL** | **Full** (bgn)⁶ | — | — |
| `rkt` | `rkt` | Rangpuri (Bengali script) | in · ben, Assamese's keys (ৰ ৱ, ৎ ঃ long-press) | LTR | **Full** (rkt)⁶ | — | — |
| `abr` | `abr` | Abron Bono | us + long-press ɛ ɔ, Akan's keys | LTR | **Full** (abr)⁶ | ✓ | — |
| `cdo` | `cdo` | Eastern Min Mìng-dĕ̤ng-ngṳ̄ | us + long-press á â ă ā à, ā̤ â̤…, ṳ̄ ṳ̆… | LTR | **Full** (cdo)⁶ | ✓ | — |
| `hif` | `hif` | Fiji Hindi | us | LTR | **Full** (hif)⁶ | ✓ | — |
| `egl` | `egl` | Emilian | it + long-press ś ṡ ṅ ō ē ū ć ṿ ł ǵ ż | LTR | **Full** (egl)⁶ | ✓ | — |
| `cu` | `cu` | Church Slavonic Словѣньскъ | ru, є ѣ ꙑ ꙁ ꙗ for е э ы з я, keys for ѥ and its accent marks + long-press ѧ ѫ ѡ ї і ѵ… | LTR | **Full** (cu)⁶ | — | — |
| `zea` | `zea` | Zeelandic Zeêuws | nl + long-press ê ô first, ' key | LTR | **Full** (zea)⁶ | ✓ | — |
| `btm` | `btm` | Batak Mandailing Saro Mandailing | us | LTR | **Full** (btm)⁶ | ✓ | — |
| `gpe` | `gpe` | Ghanaian Pidgin | us | LTR | **Full** (gpe)⁶ | ✓ | — |
| `igl` | `igl` | Igala | us + long-press ẹ ọ first, é è ó ò í ì á à ú ù, ñ, ' key | LTR | **Full** (igl)⁶ | ✓ | — |
| `rki` | `rki` | Arakanese ရခိုင် | mm, Burmese's keys | LTR | **Full** (rki)⁶ | — | — |
| `vel` | `vel` | Veluws | nl + long-press ö ä å ü first, ' key | LTR | **Full** (vel)⁶ | ✓ | — |
| `tbw` | `tbw` | Aborlan Tagbanwa Tagbanwa | us, ꞌ key | LTR | —⁹ | — | — |
| `aa` | `aa` | Afar Qafar | us | LTR | —⁹ | — | — |
| `bom` | `bom` | Berom | us + long-press ɛ é è ê ē ě ɛ́ ɛ̀ ɛ̂ ɛ̄ ɔ ó ò ô ō ɔ́ ɔ̀ ɔ̂ á à â ā í î ī û | LTR | —⁹ | — | — |
| `bew` | `bew` | Betawi | us + long-press é è ĕ, ' key | LTR | **Full** (bew)⁶ | ✓ | — |
| `sef` | `sef` | Cebaara Senoufo Cebaara | us + long-press ɛ ŋ ɔ, ˀ key | LTR | —⁹ | — | — |
| `cbk` | `cbk` | Chavacano | us + long-press ñ á é í ó ú | LTR | —⁹ | — | — |
| `cho` | `cho` | Choctaw Chahta | us + long-press a̱ á i̱ í o̱ ó ʋ u̱ | LTR | —⁹ | — | — |
| `dzg` | `dzg` | Dazaga | us + long-press ẹ ə ɛ ŋ ọ ɔ š | LTR | —⁹ | — | — |
| `mvp` | `mvp` | Duri | us, ' key | LTR | —⁹ | — | — |
| `igb` | `igb` | Ebira | us + long-press á à â ā é è ê ē ẹ ệ í ì î ī ị ó ò ô ō ọ ộ ú ù û ū ụ | LTR | —⁹ | — | — |
| `fan` | `fan` | Fang | us + long-press ñ ŋ ɔ ô ò é è ê â à î ï | LTR | —⁹ | — | — |
| `gqr` | `gqr` | Gor | us + long-press ɓ ɗ ə ɨ ɔ, ɨ ' keys, ə ɔ on q x | LTR | —⁹ | — | — |
| `pov` | `pov` | Guinea-Bissau Creole Kriol | us + long-press á â ã é ê í ñ ŋ ó ô õ ŝ ú ẑ ç | LTR | —⁹ | — | — |
| `cgg` | `cgg` | Kiga Rukiga | us, ' key | LTR | —⁹ | — | — |
| `mkw` | `mkw` | Kituba | us⁹ | LTR | **Full** (mkw)¹⁸ | ✓ | — |
| `lkt` | `lkt` | Lakota Lakȟólʼiyapi | us + long-press á č ʼ é ǧ ȟ í ŋ ó š ú ž, ŋ á keys, č é on q x | LTR | —⁹ | — | — |
| `las` | `las` | Lama | us + long-press ɖ ǝ ɛ ɨ ɩ ñ ŋ ɔ ʋ | LTR | —⁹ | — | — |
| `led` | `led` | Lendu | us + long-press ɨ ø ʉ, ʼ key | LTR | —⁹ | — | — |
| `jbo` | `jbo` | Lojban | us, ' key | LTR | —⁹ | — | — |
| `mnb` | `mnb` | Muna | us, ꞌ key | LTR | —⁹ | — | — |
| `mua` | `mua` | Mundang Mundaŋ | us + long-press ã ɓ ɗ ë ǝ ẽ ĩ ŋ õ ṽ, ǝ ' keys | LTR | —⁹ | — | — |
| `nv` | `nv` | Navajo Diné Bizaad | us + long-press á ą ą́ é ę ę́ í į į́ ł ó ǫ ǫ́ ń, ʼ í keys, á é on q x | LTR | —⁹ | — | — |
| `sba` | `sba` | Ngambay | us + long-press ɛ ə é è ḛ ɔ ó o̰ ə́ ə̰ ɓ ɗ ŋ á à a̰ í ḭ, ə ' keys, é ɔ on q x | LTR | —⁹ | — | — |
| `nup` | `nup` | Nupe | us | LTR | **Full** (nup)⁶ | ✓ | — |
| `pmy` | `pmy` | Papuan Malay Melayu Papua | us | LTR | —⁹ | — | — |
| `srr` | `srr` | Serer Seereer | us + long-press ɓ ƈ ɗ ñ ŋ ƥ ƭ ƴ é, ' key | LTR | —⁹ | — | — |
| `xog` | `xog` | Soga Olusoga | us, ' key | LTR | —⁹ | — | — |
| `fub-latn` | `fub` | Adamawa Fulfulde (Latin) | us + long-press ɓ ɗ ŋ ƴ, ’ key | LTR | —⁹ | — | — |
| `bcq-latn` | `bcq` | Bench (Latin) | us, ʼ key | LTR | —⁹ | — | — |
| `snk` | `snk` | Soninke Sooninkanxanne | us + long-press ŋ | LTR | —⁹ | — | — |
| `tem` | `tem` | Temne | us + long-press ə ɛ ŋ ɔ ʌ ȧ å, ɔ ŋ keys, ə ɛ on q x | LTR | —⁹ | — | — |
| `dje` | `dje` | Zarma Zarmaciine | us + long-press ã ẽ ɲ ŋ õ š ž, ' key | LTR | **Full** (dje)¹⁴ | ✓ | — |
| `bft` | `bft` | Balti بلتی | pk + long-press ڃ ڇ ڗ ݜ ݨ ݩ | **RTL** | —⁹ | — | — |
| `gju` | `gju` | Gujari گوجری | pk + long-press څ ڵ ݩ | **RTL** | —⁹ | — | — |
| `mvy` | `mvy` | Indus Kohistani کوہستانی | pk + long-press ڇ څ ڙ ݜ ݨ | **RTL** | —⁹ | — | — |
| `gwc` | `gwc` | Kalam Kohistani گاؤری | pk + long-press ٞ ڄ څ ݫ ݭ ݪ | **RTL** | —⁹ | — | — |
| `hno` | `hno` | Northern Hindko ہندکو | pk | **RTL** | —⁹ | — | — |
| `phr` | `phr` | Pahari-Pothwari پوٹھواری | pk + long-press ؐ ؑ ؓ ؔ ة ݨ ه ي | **RTL** | —⁹ | — | — |
| `scl` | `scl` | Shina شینا | pk + long-press ࣿ ٗ ڇ څ ڙ ݜ ݨ | **RTL** | —⁹ | — | — |
| `trw` | `trw` | Torwali توروالی | pk + long-press ٶ ڇ څ ݲ ڙ ݜ | **RTL** | —⁹ | — | — |
| `bal` | `bal` | Balochi بلۆچی | pk + long-press ۆ ه ێ | **RTL** | —⁹ | — | — |
| `haz` | `haz` | Hazaragi هزارگی | pk + long-press ه ي | **RTL** | —⁹ | — | — |
| `mki` | `mki` | Dhatki ڍاٽڪي | pk · snd + long-press ۯ | **RTL** | —⁹ | — | — |
| `kvx` | `kvx` | Parkari Koli پارڪري | pk · snd + long-press ٰ ؤ ۮ ۯ ۿ | **RTL** | —⁹ | — | — |
| `bqi` | `bqi` | Bakhtiari لوری بختیاری | ir | **RTL** | —⁹ | — | — |
| `glk` | `glk` | Gilaki گیلکی | ir, ۊ ؤ keys in place of ذ ظ + long-press ٚ ى ذ ظ | **RTL** | **Full** (glk)⁶ | — | — |
| `lrc` | `lrc` | Northern Luri لۊری شومالی | iq · ku_ara + long-press ٙ ٛ ھ ۉ ۊ ؽ | **RTL** | —⁹ | — | — |
| `mde` | `mde` | Maba Bura Mabang | ara + long-press ٚ ٛ پ ڃ چ ڊ ڔ ڗ ژ ڠ ݦ ݧ | **RTL** | —⁹ | — | — |
| `bfy` | `bfy` | Bagheli बघेली | in | LTR | **Full** (bfy)¹⁴ | — | — |
| `bgq` | `bgq` | Bagri बागड़ी | in | LTR | **Full** (bgq)¹⁴ | — | — |
| `taj` | `taj` | Eastern Tamang तामाङ | in | LTR | —⁹ | — | — |
| `bgc` | `bgc` | Haryanvi हरियाणवी | in + long-press ॅ | LTR | —⁹ | — | — |
| `xnr` | `xnr` | Kangri कांगड़ी | in + long-press ॐ ऍ ऽ ॅ | LTR | **Full** (xnr)¹⁴ | — | — |
| `lif` | `lif` | Limbu लिम्बू | in + long-press ॽ | LTR | —⁹ | — | — |
| `xsr` | `xsr` | Sherpa शेर्पा | in | LTR | —⁹ | — | — |
| `sgj` | `sgj` | Surgujia सरगुजिया | in | LTR | **Full** (sgj)¹⁴ | — | — |
| `syl` | `syl` | Sylheti সিলেটি (Bengali script) | in · ben + long-press ঃ ৎ ৗ ʼ | LTR | —⁹ | — | — |
| `syl-sylo` | `syl-Sylo` | Sylheti ꠍꠤꠟꠐꠤ (Syloti Nagri) | Keyman's sylheti_nagri¹⁰ | LTR | **Full** (syl-Sylo)⁶ ⁷ | — | — |
| `ccp` | `ccp` | Chakma 𑄌𑄋𑄴𑄟𑄳𑄦 | Keyman's easy_chakma | LTR | —¹⁰ | — | — |
| `rhg` | `rhg` | Rohingya 𐴌𐴗𐴥𐴝𐴙𐴚𐴒𐴙𐴝 (Hanifi script) | Keyman's hanifi_rohingya, long-press 𐴗 on 𐴖 | **RTL** | —¹⁰ | — | — |
| `blt` | `blt` | Tai Dam ꪼꪕꪒꪾ | Keyman's sil_tai_dam, long-press its second taps (ꪂ on ꪀ) | LTR | —¹⁰ | — | — |
| `kyu` | `kyu` | Western Kayah (Kayah Li script) | Keyman's sil_kayah_kali | LTR | —¹⁰ | — | — |
| `lis` | `lis` | Lisu (Fraser script) | Keyman's sil_lisu_standard | LTR | —¹⁰ | — | — |
| `lep` | `lep` | Lepcha | Keyman's sil_lepcha, long-press its aspirates and l clusters (ᰂ ᰁ on ᰀ) | LTR | —¹⁰ | — | — |
| `kjg` | `kjg` | Khmu (Lao script) | Keyman's sil_kmhmu | LTR | —¹⁰ | — | — |
| `saz` | `saz` | Saurashtra | Keyman's saurashtra_inscript | LTR | —¹⁰ | — | — |
| `bax` | `bax` | Bamum | Keyman's bamum, its two marks beside the space bar | LTR | —¹⁰ | — | — |
| `cjm` | `cjm` | Eastern Cham (Cham script) | Keyman's sil_cham_phonetic, long-press its AltGr letters | LTR | —¹⁰ | — | — |
| `tdd` | `tdd` | Tai Nuea (Tai Le script) | Keyman's tainua¹⁰ | LTR | **Full** (tdd)⁶ | — | — |
| `khb` | `khb` | Tai Lue (New Tai Lue script) | Keyman's ntl_onekey | LTR | —¹⁰ | — | — |
| `osa` | `osa` | Osage 𐓏𐓘𐓻𐓘𐓻𐓟 | Keyman's osage_nation, its acute and double acute beside the space bar | LTR | —¹⁰ | — | — |
| `hac` | `hac` | Gorani | Keyman's horami¹⁰ | **RTL** | **Full** (hac)⁷ | — | — |
| `psi` | `psi` | Southeastern Pashayi | Keyman's pashai, long-press its AltGr letters | **RTL** | —¹⁰ | — | — |
| `hnd` | `hnd` | Southern Hindko | Keyman's rac_hindko, long-press ݨ | **RTL** | —¹⁰ | — | — |
| `cld` | `cld` | Chaldean Neo-Aramaic (Syriac script) | Keyman's east_syriac_qwerty, its vowel points and marks on Shift | **RTL** | —¹⁰ | — | — |
| `nod` | `nod` | Northern Thai (Tai Tham script) | Keyman's tai_lanna_thai (experimental), long-press its AltGr letters and ᩀ | LTR | —¹⁰ | — | — |
| `und-fonipa` | `und-fonipa` | International Phonetic Alphabet (IPA) | us + Keyman's sil_ipa: IPA letters and modifier letters on long presses, ː ˈ ˌ and diacritics on the symbol views | LTR | —¹⁰ | — | — |
| `nod-thai` | `nod` | Northern Thai (Thai script) | th, Thai's keys | LTR | —¹² | — | — |
| `adx` | `adx` | Amdo Tibetan (Tibetan script) | cn · tib, Tibetan's keys | LTR | —¹² | — | — |
| `khg` | `khg` | Khams Tibetan (Tibetan script) | cn · tib, Tibetan's keys | LTR | —¹² | — | — |
| `sck` | `sck` | Sadri (Devanagari) | in, Hindi's keys | LTR | —¹² | — | — |
| `knn` | `knn` | Maharashtrian Konkani (Devanagari) | in · marathi, Marathi's keys | LTR | —¹² | — | — |
| `kfr` | `kfr` | Kutchi (Gujarati script) | in · guj, Gujarati's keys | LTR | —¹² | — | — |
| `kjp` | `kjp` | Eastern Pwo Karen (Myanmar script) | mm, ႋ ၜ ၠ for ဩ ၏ ၒ + long-press ၮ ၯ ၰ ၞ ၟ ၚ ဴ¹² | LTR | **Full** (kjp)⁶ | — | — |
| `tyz` | `tyz` | Tày | vn + long-press its two tones Vietnamese lacks (a̱ ȁ, e̱ ȅ…) | LTR | —¹² | — | — |
| `mnp` | `mnp` | Northern Min Mâing-bă̤-ngṳ̌ | us + long-press â̤ á a̿ ă ǎ…, ṳ in its tones | LTR | —¹² | — | — |
| `sty` | `sty` | Siberian Tatar Себер татар | ru · bak, Bashkir's keys + long-press ё | LTR | —¹² | — | — |
| `tts` | `tts` | Isan อีสาน (Thai script) | th, Thai's keys | LTR | —¹² | — | — |
| `mdh` | `mdh` | Maguindanao Magindanawn | us | LTR | —¹² | — | — |
| `cps` | `cps` | Capiznon | us | LTR | —¹² | — | — |
| `meo` | `meo` | Kedah Malay | us, Malay's keys + long-press å | LTR | —¹² | — | — |
| `qxq` | `qxq` | Qashqai قشقایی | ir, South Azerbaijani's keys + long-press ێ | **RTL** | —¹² | — | — |
| `rej` | `rej` | Rejang Baso Jang | us + long-press ê | LTR | —¹² | — | — |
| `bbj` | `bbj` | Ghomala' | us + long-press ə ɛ ɔ ʉ ŋ ƴ and their tones | LTR | —¹² | — | — |
| `bug` | `bug` | Buginese Basa Ugi | us + long-press é | LTR | —¹² | — | — |
| `lki` | `lki` | Laki لەکی | iq · ku_ara, Sorani's keys | **RTL** | —¹² | — | — |
| `bcc` | `bcc` | Southern Balochi بلۏچی | pk, Balochi's keys + long-press ۏ ݔ ګ | **RTL** | —¹² | — | — |
| `ahr` | `ahr` | Ahirani अहिराणी | in · marathi, Marathi's keys | LTR | **Full** (ahr)¹⁴ | — | — |
| `hoj` | `hoj` | Harauti हाड़ौती | in, Hindi's keys | LTR | —¹² | — | — |
| `bjj` | `bjj` | Kannauji कन्नौजी | in, Hindi's keys | LTR | —¹² | — | — |
| `ctg` | `ctg` | Chittagonian চাটগাঁইয়া | in · ben, Bengali's keys + long-press ৱ ৰ | LTR | —¹² | — | — |
| `hni` | `hni` | Hani Haqniqdoq | us | LTR | —¹² | — | — |
| `niq` | `niq` | Nandi | us + long-press ñ ā ē ĩ ō, ' key | LTR | —¹² | — | — |
| `max` | `max` | North Moluccan Malay | us + long-press à á é è í ò ó ù ú ŭ | LTR | —¹² | — | — |
| `mfb` | `mfb` | Bangka Malay | us | LTR | —¹² | — | — |
| `bgz` | `bgz` | Banggai | us | LTR | —¹² | — | — |
| `cyo` | `cyo` | Cuyonon | us + long-press ñ | LTR | —¹² | — | — |
| `zyj` | `zyj` | Youjiang Zhuang | us | LTR | —¹² | — | — |
| `mog` | `mog` | Mongondow | us + long-press ḷ ḻ, ' key | LTR | —¹² | — | — |
| `pko` | `pko` | Pokot | us | LTR | —¹² | — | — |
| `kmz` | `kmz` | Khorasani Turkic | ir, South Azerbaijani's keys | **RTL** | —¹² | — | — |
| `kxm` | `kxm` | Northern Khmer (Thai script) | th, Thai's keys | LTR | —¹² | — | — |
| `mqy` | `mqy` | Manggarai | us + long-press é, ’ key | LTR | —¹² | — | — |
| `mhy` | `mhy` | Ma'anyan | us | LTR | —¹² | — | — |
| `prs` | `prs` | Dari دری | ir, Persian's keys + long-press ټ ځ څ ډ ړ ږ ښ ګ ڼ ى | **RTL** | —¹² | — | — |
| `ca-valencia` | `ca-valencia` | Valencian Valencià | es · cat, Catalan's keys | LTR | —¹² | — | — |
| `bhb` | `bhb` | Bhili (Devanagari) | in, Hindi's keys | LTR | —¹² | — | — |
| `kfy` | `kfy` | Kumaoni (Devanagari) | in, Hindi's keys | LTR | **Full** (kfy)¹⁴ | — | — |
| `kru` | `kru` | Kurukh (Devanagari) | in, Hindi's keys + long-press ' | LTR | **Full** (kru)¹⁴ | — | — |
| `mtr` | `mtr` | Mewari (Devanagari) | in, Hindi's keys | LTR | —¹² | — | — |
| `noe` | `noe` | Nimadi (Devanagari) | in, Hindi's keys + long-press ऽ | LTR | **Full** (noe)¹⁴ | — | — |
| `swv` | `swv` | Shekhawati (Devanagari) | in, Hindi's keys | LTR | —¹² | — | — |
| `wbr` | `wbr` | Wagdi (Devanagari) | in, Hindi's keys | LTR | —¹² | — | — |
| `unr` | `unr` | Mundari (Devanagari) | in, Hindi's keys | LTR | **Full** (unr)¹⁴ | — | — |
| `bns` | `bns` | Bundeli (Devanagari) | in, Hindi's keys | LTR | **Full** (bns)¹⁴ | — | — |
| `dhd` | `dhd` | Dhundhari (Devanagari) | in, Hindi's keys | LTR | —¹² | — | — |
| `kfq` | `kfq` | Korku (Devanagari) | in, Hindi's keys | LTR | **Full** (kfq)¹⁷ | — | — |
| `mup` | `mup` | Malvi (Devanagari) | in, Hindi's keys | LTR | —¹² | — | — |
| `wry` | `wry` | Merwari (Devanagari) | in, Hindi's keys | LTR | —¹² | — | — |
| `bfz` | `bfz` | Mahasu Pahari (Devanagari) | in, Hindi's keys | LTR | —¹² | — | — |
| `hlb` | `hlb` | Halbi (Devanagari) | in, Hindi's keys | LTR | **Full** (hlb)¹⁷ | — | — |
| `lmn` | `lmn` | Lambadi (Telugu script) | in · tel, Telugu's keys | LTR | —¹² | — | — |
| `sou` | `sou` | Southern Thai | th, Thai's keys | LTR | —¹² | — | — |
| `dcc` | `dcc` | Dakhini (Urdu script) | pk, Urdu's keys | **RTL** | —¹² | — | — |
| `bpy` | `bpy` | Bishnupriya Manipuri বিষ্ণুপ্রিয়া মণিপুরী | in · ben, Bengali's keys + long-press ৱ | LTR | **Full** (bpy)⁶ | — | — |
| `gax` | `gax` | Southern Oromo | us, Oromo's keys, ' key¹² | LTR | **Full** (gax)¹⁸ | ✓ | — |
| `yaf` | `yaf` | Yaka | us + long-press á é í ó ú¹² | LTR | **Full** (yaf)¹⁸ | ✓ | — |
| `ymm` | `ymm` | Maay | us, Somali's keys, ' key | LTR | —¹² | — | — |
| `nym` | `nym` | Nyamwezi Kinyamwezi | us, ' key | LTR | —¹² | — | — |
| `cjk` | `cjk` | Chokwe | us | LTR | —¹² | — | — |
| `hea` | `hea` | Northern Qiandong Miao Hmu | us | LTR | —¹² | — | — |
| `grt` | `grt` | Garo A·chik | us, · key | LTR | **Full** (grt)¹⁴ | ✓ | — |
| `grt-beng` | `grt-Beng` | Garo গারো (Bengali script) | in · ben, Bengali's keys, · on ঋ's | LTR | **Full** (grt-Beng)¹⁴ | — | — |
| `mdr` | `mdr` | Mandar | us | LTR | —¹² | — | — |
| `pcc` | `pcc` | Bouyei Haausqyaix | us | LTR | —¹² | — | — |
| `mtq` | `mtq` | Muong Thiểng Mường | vn, Vietnamese's keys | LTR | —¹² | — | — |
| `kxu` | `kxu` | Kui (Odia script) | in · ori, Odia's keys (+ long-press ୟ ୱ), ୕ key | LTR | —¹² | — | — |
| `pse` | `pse` | Central Malay Baso Melayu Tenga | us | LTR | —¹² | — | — |
| `kge` | `kge` | Komering Basa Kumoring | us | LTR | —¹² | — | — |
| `vkt` | `vkt` | Kutai Malay | us | LTR | —¹² | — | — |
| `gdx` | `gdx` | Godwari गोड़वाड़ी | in, Hindi's keys | LTR | —¹² | — | — |
| `mjl` | `mjl` | Mandeali मंडयाली | in, Hindi's keys | LTR | **Full** (mjl)¹⁴ | — | — |
| `sjp` | `sjp` | Surjapuri सुरजापुरी | in, Hindi's keys | LTR | **Full** (sjp)¹⁴ | — | — |
| `vah` | `vah` | Varhadi वऱ्हाडी | in · marathi, Marathi's keys | LTR | —¹² | — | — |
| `vas` | `vas` | Vasavi वासवी (Devanagari) | in, Hindi's keys | LTR | —¹² | — | — |
| `vas-gujr` | `vas` | Vasavi વસાવી (Gujarati script) | in · guj, Gujarati's keys | LTR | —¹² | — | — |
| `gjk` | `gjk` | Kachi Koli કચ્છી કોળી (Gujarati script) | in · guj, Gujarati's keys | LTR | —¹² | — | — |
| `uum` | `uum` | Urum Урум | ru + long-press ӧ ӱ ґ, ʼ key | LTR | —¹² | — | — |
| `bxk` | `bxk` | Bukusu Lubukusu | us, ' key | LTR | —¹³ | — | — |
| `lon` | `lon` | Malawi Lomwe Ellomwe | us, ' key | LTR | —¹³ | — | — |
| `fuc` | `fuc` | Pulaar | us + long-press ñ ŋ ɲ ɓ ɗ ƴ, Fula's keys with ñ first | LTR | —¹⁴ | — | — |
| `gya` | `gya` | Northwest Gbaya | us + long-press á à ɛ é í ì ɔ ó ò ú ù ŋ ɓ ɗ, ɛ ' keys (long-press ɛ́ ɛ̀), ɔ and the nasal cedilla on x q | LTR | —¹² | — | — |
| `mwm` | `mwm` | Sar | us + long-press ɓ, ə ɔ on q x, macron and acute keys (long-press the tilde below) | LTR | —¹² | — | — |
| `mcn` | `mcn` | Masana | us + long-press ɗ ŋ ɓ ɦ, ' key | LTR | —¹² | — | — |
| `mpg` | `mpg` | Marba | us + long-press â á ä à î ï ì ê ô û, ’ key | LTR | —¹² | — | — |
| `rim` | `rim` | Nyaturu Kinyaturu | us + long-press ë ü, ë ' keys | LTR | —¹² | — | — |
| `hae` | `hae` | Eastern Oromo | us, Oromo's keys, ’ key | LTR | —¹² | — | — |
| `luz` | `luz` | Southern Luri | ir, Persian's keys, ة ە ݖ ۿ keys in place of ث ذ ظ ض + long-press ۊ ۋ ۏ ڤ ڍ ڌ ڲ ݩ ھ ٛ | **RTL** | —¹² | — | — |
| `nau` | `nau` | Nauruan Dorerin Naoero | us + long-press â ñ | LTR | —¹² | — | — |
| `mel` | `mel` | Central Melanau | us, Malay's keys | LTR | —¹² | — | — |
| `msi` | `msi` | Sabah Malay | us, Malay's keys | LTR | —¹² | — | — |
| `gyn` | `gyn` | Guyanese Creole Creolese | us | LTR | —¹² | — | — |
| `prk` | `prk` | Parauk Lox Vax | us + long-press á ā é ē í ī ó ō ú ū first, : key | LTR | —¹² | — | — |
| `gay` | `gay` | Gayo | id, Indonesian's keys + long-press é è ö first | LTR | —¹² | — | — |
| `ijc` | `ijc` | Izon Ịzọn | us, Igbo's keys + long-press ẹ and the tones á à é è í ì ó ò ú ù | LTR | —¹² | — | — |
| `bgx` | `bgx` | Balkan Gagauz Turkish Rumelice | tr, Turkish's keys | LTR | —¹² | — | — |
| `lrl` | `lrl` | Lari اچمی | ir, Persian's keys | **RTL** | —¹² | — | — |
| `btz` | `btz` | Batak Alas-Kluet | id, Indonesian's keys | LTR | —¹² | — | — |
| `nut` | `nut` | Nung Nùng | vn, Vietnamese's keys | LTR | —¹² | — | — |
| `plk` | `plk` | Kohistani Shina ݜݨیاٗ کستِین٘و | pk, Shina's keys + long-press ڦ ڱ ٗ first | **RTL** | —¹² | — | — |
| `fay` | `fay` | Kuhmareyi کوهمَره‌ای (Southwestern Fars) | ir, Persian's keys + long-press ڌ ۊ ۆ ۉ ٱ ێ ؽ ݩ ٛ first | **RTL** | —¹² | — | — |
| `wne` | `wne` | Wanetsi وڼېڅي | af · ps, Pashto's keys + long-press ۀ ں first | **RTL** | —¹² | — | — |
| `gdq` | `gdq` | Mehri مهريّت | ara, Arabic's keys | **RTL** | —¹² | — | — |
| `tks` | `tks` | Takestani تاتی | ir, Persian's keys | **RTL** | —¹² | — | — |
| `bgp` | `bgp` | Eastern Balochi بلوچی | pk, Balochi's keys | **RTL** | —¹² | — | — |
| `psh` | `psh` | Southwestern Pashayi | Southeastern Pashayi's keys (Keyman's pashai) | **RTL** | —¹² | — | — |
| `cja` | `cja` | Western Cham ꨌꩌ ꨚꨭꩉ (Cham script) | Eastern Cham's keys (Keyman's sil_cham_phonetic) | LTR | —¹² | — | — |
| `cja-latn` | `cja` | Western Cham Cam pur (Latin letters) | us + long-press â ă first | LTR | —¹² | — | — |
| `mmr` | `mmr` | Xong Dut Xongb | us | LTR | —¹² | — | — |
| `wes` | `wes` | Cameroon Pidgin Kamtok | us | LTR | —¹² | — | — |
| `bah` | `bah` | Bahamian Creole Bahamian | us | LTR | —¹² | — | — |
| `trf` | `trf` | Trinidadian Creole | us | LTR | —¹² | — | — |
| `sxu` | `sxu` | Upper Saxon Obersächsisch | de, German's keys | LTR | —¹² | — | — |
| `bxg` | `bxg` | Bangala Ngala | us, Lingala's keys + long-press á ɓ first | LTR | —¹² | — | — |
| `anw` | `anw` | Anaang Anaañ | us + long-press ñ ö ü first | LTR | —¹² | — | — |
| `haq` | `haq` | Ha Igiha | us, ' key | LTR | —¹² | — | — |
| `bmm` | `bmm` | Northern Betsimisaraka Malagasy Betsimisaraka Avaratra | us, Malagasy's keys + long-press n̈ ỳ à ê ì ï first, ' key | LTR | —¹² | — | — |
| `bzc` | `bzc` | Southern Betsimisaraka Malagasy Betsimisaraka Atsimo | us, Malagasy's keys + long-press à é ỳ ì first, ' key | LTR | —¹² | — | — |
| `syr` | `syr` | Syriac | sy · syc | **RTL** | **Full** (syr)⁷ | — | — |
| `ber` | `ber` | Tamazight | ma · tifinagh | LTR | **Full** (ber)⁷ | — | — |
| `cop` | `cop` | Coptic Ⲙⲉⲧⲣⲉⲙⲛ̀ⲭⲏⲙⲓ (Bohairic) | eg · cop, a jinkim key (long-press the overline) | LTR | **Full** (cop)⁷ | — | — |
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
| `om` | `om` | Oromo | us, ' key | LTR | **Full** (om)⁷ | ✓ | — |
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
| `as` | `as` | Assamese | in · ben (+ ৰ ৱ, ৎ ঃ long-press) | LTR | **Full** (as)⁷ | — | — |
| `bo` | `bo` | Tibetan བོད་ཡིག | cn · tib | LTR | **Full** (bo)⁷ | — | — |
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
| `rw` | `rw` | Kinyarwanda | us, ' key | LTR | **Full** (rw)⁷ | ✓ | — |
| `rn` | `rn` | Kirundi | us, ' key | LTR | **Full** (rn)⁷ | ✓ | — |
| `sn` | `sn` | Shona chiShona | us | LTR | **Full** (sn)⁷ | ✓ | — |
| `ny` | `ny` | Chichewa | us + long-press ŵ | LTR | **Full** (ny)⁷ | ✓ | — |
| `mg` | `mg` | Malagasy | us + long-press ñ ô à ì, ' key | LTR | **Full** (mg)⁷ | ✓ | — |
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
| `nah` | `nah` | Nahuatl Nāhuatl | us + long-press ā ē ī ō | LTR | **Full** (nah)¹⁷ | ✓ | — |
| `kl` | `kl` | Greenlandic Kalaallisut | us | LTR | **Full** (kl)⁷ | ✓ | — |
| `chr` | `chr` | Cherokee ᏣᎳᎩ | us · chr | LTR | — | — | — |
| `iu` | `iu` | Inuktitut ᐃᓄᒃᑎᑐᑦ | ca · ike | LTR | **Full** (iu)⁷ | — | — |
| `mni` | `mni` | Meitei ꯃꯤꯇꯩ ꯂꯣꯟ | in · mni | LTR | **Full** (mni)⁶ | — | — |
| `mni-beng` | `mni-Beng` | Meitei মণিপুরী (Bengali script) | in · ben + long-press ৱ ৎ ঃ | LTR | **Full** (mni-Beng)⁷ | — | — |
| `dz` | `dz` | Dzongkha རྫོང་ཁ | bt (+ ་ key) | LTR | **Full** (dz)⁷ | — | — |
| `cn-pinyin` | `zh-Hans` | Chinese 中文 (拼音) | — | LTR | IME | — | **Pinyin** (CC-CEDICT) |
| `tw-zhuyin` | `zh-Hant` | Chinese 中文 (注音) | — | LTR | IME | — | **Zhuyin** Bopomofo → Traditional (CC-CEDICT) |
| `yue-jyutping` | `yue` | Cantonese 粵語 (粵拼) | — | LTR | IME | — | **Jyutping** Latin → Traditional (CC-Canto) |
| `jp-romaji` | `ja` | Japanese 日本語 | — | LTR | IME | — | **Kana→Kanji** (mozc, connection-cost lattice) |
| `kr-2set` | `ko` | Korean 한국어 (두벌식) | — | LTR | IME | — | **Hangul** (algorithmic) |
| `am` | `am` | Amharic አማርኛ | — | LTR | **Full** (am)⁷ | — | **Ge'ez** (phonetic Latin→fidel, its words on the strip¹⁶ and the last syllable's homophones in its last slot; capitals on the Shift view for the ejectives ጠ ጨ ጰ ጸ and ዸ, the 5th order and the 6th (I: እ alone); ። ፣ keys) |
| `ti` | `ti` | Tigrinya ትግርኛ | — | LTR | **Full** (ti)⁷ | — | **Ge'ez** (shares the Amharic composer; its words on the strip¹⁶) |
| `gez` | `gez` | Ge'ez ግዕዝ | — | LTR | IME | — | **Ge'ez** (shares the Amharic composer) |
| `drs` | `drs` | Gedeo (Ge'ez script) | — | LTR | IME | — | **Ge'ez** (shares the Amharic composer; its ዸ as D) |
| `hdy` | `hdy` | Hadiyya (Ge'ez script) | — | LTR | IME | — | **Ge'ez** (shares the Amharic composer) |
| `ktb` | `ktb` | Kambaata (Ge'ez script) | — | LTR | IME | — | **Ge'ez** (shares the Amharic composer; its ኧ as A) |
| `stv` | `stv` | Silt'e (Ge'ez script) | — | LTR | IME | — | **Ge'ez** (shares the Amharic composer; its -wa forms ሏ ሟ ቧ…) |
| `sgw` | `sgw` | Sebat Bet Gurage ሰባት ቤት ጉራጌ | — | LTR | —¹⁹ | — | **Ge'ez** (shares the Amharic composer; ʷ and ʸ keys for its labialised bilabials ᎀ ᎄ ᎈ ᎌ and palatalised velars ⷀ ⷈ ⷐ ⷘ, in place of v and the hyphen¹⁵) |
| `vai` | `vai` | Vai ꕙꔤ | — | LTR | **Full** (vai)⁶ | — | **Translit** Latin → Vai syllables, as Keyman's athinkra_vai types them (ka ꕪ, ka~ ꕫ), its words on the strip¹¹; ɛ ɔ ~ keys, long-press ŋ þ ð ʒ ꘌ |
| `ii` | `ii` | Nuosu ꆈꌠꉙ (Yi) | — | LTR | **Full** (ii)⁶ ⁷ | — | **Translit** Yi pinyin → syllables, as Keyman's sil_yi types them (bit ꀖ), the syllables its letters begin on the strip¹¹; Space adds no space; ， 。 keys |
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
| `ahr-abc` | `ahr` | Ahirani अहिराणी (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the ahr lexicon choosing |
| `bfy-abc` | `bfy` | Bagheli बघेली (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the bfy lexicon choosing |
| `bgq-abc` | `bgq` | Bagri बागड़ी (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the bgq lexicon choosing |
| `bns-abc` | `bns` | Bundeli (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the bns lexicon choosing |
| `hlb-abc` | `hlb` | Halbi (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the hlb lexicon choosing |
| `kfq-abc` | `kfq` | Korku (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the kfq lexicon choosing |
| `kfy-abc` | `kfy` | Kumaoni (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the kfy lexicon choosing |
| `kru-abc` | `kru` | Kurukh (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the kru lexicon choosing |
| `mjl-abc` | `mjl` | Mandeali मंडयाली (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the mjl lexicon choosing |
| `noe-abc` | `noe` | Nimadi (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the noe lexicon choosing |
| `sjp-abc` | `sjp` | Surjapuri सुरजापुरी (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Devanagari, the sjp lexicon choosing |
| `rkt-abc` | `rkt` | Rangpuri (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Bengali script, the rkt lexicon choosing |
| `bpy-abc` | `bpy` | Bishnupriya Manipuri বিষ্ণুপ্রিয়া মণিপুরী (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Bengali script, the bpy lexicon choosing |
| `grt-beng-abc` | `grt-Beng` | Garo গারো (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Bengali script, the grt-Beng lexicon choosing |
| `lo-abc` | `lo` | Lao ລາວ (abc) | — | LTR | IME | — | **Translit**⁴ Latin → Lao, the lo lexicon choosing; vowels before their consonant, tones untyped |
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
   Nepali's and Odia's stayed as written. Meitei in the Bengali script,
   which nothing romanizes, is measured on Aksharantar's Meitei words
   respelled in that script as its FineWeb-2 text spells them (a quarter
   of them have a spelling the text writes at least three times: 1 212
   test words; `make aksharantar-respelled`): of those its lexicon has,
   84.1% first (88.1% among the strip's two), 73.8% of all, its costs
   fitted to the respelled training words as the others' were (from 83.0%
   and 72.7%). The tables read what those
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
   read (below) among them. Thirteen more, of languages whose models came
   later, are measured the same way, Ahirani's on Marathi's words,
   Rangpuri's and Bishnupriya's on Bengali's: first, Bagri 91.0% (95.8%),
   Bundeli 91.0% (96.1%), Ahirani 90.5% (93.5%), Korku 90.4% (94.7%),
   Kumaoni 90.4% (96.0%), Bagheli 90.1% (96.1%), Surjapuri 89.0% (96.4%),
   Halbi 86.6% (92.9%), Bishnupriya 86.5% (89.9%), Mandeali 85.2% (93.8%),
   Kurukh 81.7% (89.6%), Nimadi 74.9% (80.6%) and Rangpuri 74.7% (90.5%):
   Nimadi's lowest where its own form comes first (क where Hindi writes
   के, हई for है), Rangpuri's where its ৰ meets Bengali's র. Each is
   Hindi's table, Rajasthani's (Bagri's), Garhwali's (Kumaoni's),
   Marathi's (Ahirani's) or Bengali's (Rangpuri's, with ৰ for r, and
   Bishnupriya's, with ৱ for w) with what its text writes: Nimadi's
   avagraha for a word's last spoken a (karna: करनऽ), Kurukh's visarga
   after a long vowel (आःद), its ङ (tanga: तङआ) and the apostrophe in one
   word in twenty-six (ra'i: र'ई), Korku's and Nimadi's ळ, Ahirani's and
   Korku's ऱ (rhas: ऱ्हास), Halbi's and Kumaoni's ङ, a word's last h as ः,
   English words' a as ॅ (madam: मॅडम) and ॉ as some write it, ाॅ
   (डाॅक्टर).
   Gondi's is Telugu's table and
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
   where ي and ی are one letter (73.3% to 75.1%). Lao's (`lo-abc`) is
   measured so too, on Wiktionary's 1 239 Lao words, their macrons and
   tones dropped and their syllables run together as Lao writes them
   (sā bāi dī: sabaidi): of the 88% its lexicon has, 56.2% first (69.6%
   among the strip's two). Its table writes a vowel's first part before
   its consonant (ເ◌ົາ: kao is ເກົາ), the high consonant Lao writes
   unspoken (ໝາ) and the tone marks nothing typed says where its lexicon
   has a word so written (chao: ເຈົ້າ); most of what it misses is what Lao
   spelled in Latin cannot tell apart, which the lexicon decides by how
   often each is written: a vowel's length (pa: ປະ, ປາ), the class of a
   consonant (s: ສ, ຊ), the unspoken ຫ (ma: ມາ, ໝາ) and the tone (pa:
   ປາ, ປ່າ). Meitei in the Bengali script has no such measure
   (Wiktionary has three of its words); its tests read everyday words. Of the 51 checked in the seven tables new with
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
6. These thirty-seven models learn from their own Wikipedia (CC BY-SA 4.0,
   share-alike; `scripts/fetch-wikipedia.sh`, the wikis' CirrusSearch
   dumps of 2026-11-08): Bhojpuri, Maithili, Mongolian, Nepali, Odia,
   Western Punjabi, Pashto, Sanskrit, Santali, Yiddish, Konkani (the
   Goan wiki's Devanagari articles), Kashmiri, Meitei, Awadhi, Saraiki,
   Tulu, and Dotyali, Angika, Newari, Armenian, Persian, Eastern Min,
   Fiji Hindi, Emilian, Church Slavonic, Zeelandic, Batak Mandailing,
   Ghanaian Pidgin, Igala, Arakanese, Veluws, Betawi, Nupe, Gilaki,
   Bishnupriya Manipuri, Tai Nuea and Sylheti in Syloti Nagri (below).
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
   Bishnupriya Manipuri's wiki was not used at first: nearly all of it is
   stubs a bot wrote (98% of its sentences fit 2 000 templates), and so is
   GlotCC's Bishnupriya (copies of it); with three of a shape kept, its
   own sentences came to teach a model later (below). Three wikis came later: Dotyali's
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
   its words unknown 6.5% -> 5.9%. Three wikis no model had learned from
   came last: Eastern Min's, in Bàng-uâ-cê (14 904 sentences from 16 787
   articles, 12 413 sentences of bots' stubs left out, the Han its
   sentences write beside cut out as another script: keystroke savings
   58.0% on its 4 392 held-out syllables, 57.2% with taps astray, 1.1%
   unknown), whose words are read as Southern Min's are, a syllable to a
   word, the next glued on at its hyphen (`osk_data::words`:
   Mìng-dĕ̤ng-ngṳ̄); Fiji Hindi's (33 694 sentences from 12 677 articles,
   16 778 sentences of its villages' stubs left out, and the 13 700 of
   its 46 000 that write none of its own function words, hae ke aur
   rahaa…: its references, in English; `wikipedia-text.py --own`: 50.5%,
   46.1%, 4.6% unknown); and the Emilian–Romagnol wiki's Emilian (76 053
   sentences from 14 245 articles, 57 894 stub sentences left out, the
   dialect each names first left out of its text and its 111 Romagnol
   articles, another language's, left out, `--emilian`: 41.0%, 35.8%,
   8.2% unknown; its lexicon keeps 80 000 of its 144 000 kinds of word,
   its dialects, most of all Mirandola's and Carpi's, each spelled its
   own way, and at 120 000 saves 0.3 points more). Eastern Min is typed
   on the US keys with its toned vowels first on the long presses, the
   commonest first (á â ă ā à, then those with the diaeresis below, ā̤
   â̤…; ṳ in its tones on u), Fiji Hindi on them as they are, and Emilian
   on Italian's with its dialects' letters first on the long presses (ś ṡ
   ş ṣ on s, ṅ, ō ē ū beside the graves, ć ṿ ł ǵ ż). Church Slavonic's
   wiki, in the old letters (ѣ є ꙑ ѥ ѧ ѫ ꙁ ꙗ…), came after: 2 476
   sentences from 1 544 articles, 721 sentences of its bots' stubs left
   out and the 333 that write none of its own letters (`--letters`: its
   references, in Russian); keystroke savings 36.6% on 2 524 held-out
   words, 23.5% with taps astray, 25.3% unknown, a lexicon of 15 573. It
   is typed on Russian's keys with its own letters in place of those it
   hardly writes (є ѣ ꙑ ꙁ ꙗ for е э ы з я, each written in under 6% of
   its words, which are on their long presses), ѥ and a key for its
   accent marks (́, the titlo ҃ and ҄ ҆ ̀ ̑ on its long press) at the
   home row's end, and the rest on the long presses of the letters they
   are made from or sound like (ѧ ѩ on ꙗ, ѫ ꙋ ѹ on у, ѭ on ю, ѡ ѻ ѿ on о,
   ї і ѵ ѷ on и, ѳ ѯ ѱ on ф к п, ѕ ꙃ on ꙁ); its letters from Cyrillic
   Extended-B, which DejaVu Sans lacks (ꙁ ꙃ ꙋ), draw with Noto Sans.
   Zeelandic's wiki came last: 30 842 sentences from 7 488 articles,
   17 374 sentences of its bots' stubs left out (France's communes, 'n
   hemeênte in 't departement …, and the calendar's days); keystroke
   savings 52.2% on 3 008 held-out words, 45.6% with taps astray, 7.8%
   unknown, right words changed 0.30%, a lexicon of all its 60 021 kinds
   of word, its dialects each spelled their own way (Zeeuws-Vlaams drops
   its h's and writes its g's h: hemeênte, 'Olland). Its words keep the
   apostrophe they start with, as Breton's do (`osk_data::words`): Dutch's
   clipped 't 'n 's, and the h and ge- it drops ('oôdstad, 'ebouwd), 4 375
   of its 15 755 such words; one word in sixteen writes an apostrophe. It
   is typed on Dutch's keys with ê ô first on the long presses of e o (in
   2.8% and 2.1% of its words; è ë ò after them) and an apostrophe key at
   the home row's end.
   Five more wikis no round had read came after it: Batak Mandailing's
   (6 080 sentences from 1 417 articles, 146 000 words), Ghanaian
   Pidgin's (115 304 sentences, 2.6 million words) and Igala's (13 626
   sentences, 331 000 words), each the sentences that write one of its
   own words (ima, dohot, sian, adong; dey, wey, insyd, ein; ma, ki, efu,
   kpai, ọdọ: `wikipedia-text.py --own`), their references English (two
   thirds of Ghanaian Pidgin's sentences, `Retrieved 2 August 2025.`)
   and Batak Mandailing's articles heavy with Indonesian's words left
   out, and, where a template failed, the sentence that shows its name
   after the wiki's word for template left out (`Éwn malábó:Infobox …`,
   805 of Igala's: `--markup`); Arakanese's, in the Myanmar script
   (79 817 sentences, cut after its full stop ။, their Latin words and
   stray zero-width joiners left out), cut into syllables as Pa'O's are,
   its word list the 3 979 syllables its words write twice or more
   (`wordbreak/rki.txt`); and Veluws', the 1 018 articles the Dutch Low
   Saxon wiki files as written in West or East Veluws (13 215 sentences,
   229 000 words: `--category`), whose words keep the apostrophe Dutch's
   clipped 't 'n 's do (3 090 of them). Their bots' stubs are left out
   but three of a shape, and every 20th article is held out (Batak
   Mandailing's every 10th). Keystroke savings on about 3 000 held-out
   words (Arakanese's 12 824 syllables): Ghanaian Pidgin 49.8% (45.9%
   with taps astray, 3.4% unknown), Batak Mandailing 41.5% (33.0%,
   12.8%), Igala 41.2% (36.2%, 10.7%), Arakanese 38.2% (37.9%, 0.1%) and
   Veluws 32.7% (19.4%, 19.8% unknown: each writer spells their own
   dialect). Batak Mandailing, Igala and Veluws keep all their 21 000 to
   37 000 kinds of word, Ghanaian Pidgin 80 000 of its 113 000. Batak
   Mandailing and Ghanaian Pidgin are typed on the US keys as they are;
   Igala on them with ọ and ẹ first on the long presses of o and e (each
   in one word in twenty), the tones its writers mark after them and on
   the other vowels (í in 4% of its words, é ó á in 2%), ñ on n's, and an
   apostrophe key at the home row's end (ny’ẹ́yọ́, l'amẹnwu: two words in a
   hundred); Arakanese on Burmese's keys; and Veluws on Dutch's, ö first
   on o's long press (in 1.5% of its words), ä å on a's and ü on u's,
   and an apostrophe key.
   Four more wikis, of languages whose keyboards had no model, came after
   those: Betawi's (17 444 sentences, 270 000 words), Nupe's (5 157,
   115 000) and Gilaki's (18 511, 271 000), the sentences that write their
   own words (`--own`: nyang, entu, atawa, deri; nan, nyan, wun, eya; ٚ,
   ایسه, مئن, جه), their references English, some of Betawi's
   Indonesian and Gilaki's articles on its villages Persian; Gilaki's
   read as Persian's keys type it (`--persian`: ي ك ى as ی ک ی, the wiki
   writing ايسه beside ایسه); and Bishnupriya Manipuri's (11 785
   sentences, 103 000 words), in the Bengali script, 182 682 of its
   sentences its bots' stubs of years left out (Gilaki's 130 097, its
   American towns). Every 20th article is held out (Nupe's every 10th).
   Keystroke savings on about 3 000 held-out words: Betawi 49.0% (42.7%
   with taps astray, 9.6% unknown), Bishnupriya Manipuri 45.0% (35.0%,
   16.1%), Nupe 42.0% (34.3%, 11.8%) and Gilaki 37.3% (29.6%, 11.9%: read
   as typed, 36.0% and 27.9% before); autocorrect changes at most 0.40%
   of the words typed right (Gilaki's). They keep all their 10 000 to
   23 000 kinds of word, and their keyboards are as they were (notes 9
   and 12), with suggestions now, and glides on Betawi's and Nupe's.
   Tai Nuea's wiki, in Tai Le, came next (4 389 sentences, 87 000
   words): its Chinese and Latin words left out, its sentences cut after
   ။ and Chinese's 。 ？ ！ too, as a third of them end, every 10th
   article held out. Keystroke savings 45.2% on 3 070 held-out words
   (39.9% with taps astray, 5.0% unknown); autocorrect changes 0.66% of
   the words typed right, a word for one that differs in its tone letter
   (Mon's and Sgaw Karen's change up to 0.96%). It keeps all its 4 883
   kinds of word, and its keyboard (note 10) has suggestions now.
   Sylheti's Wikipedia is written in the Syloti Nagri script, and its
   keyboard here in the Bengali script has no model; since its keyboard in
   Syloti Nagri came (note 10), the wiki teaches that one's (`syl-Sylo`):
   8 027 sentences, 85 000 words, cut after ⁕ and the poetry marks ꠨ ꠩ ꠪
   ꠫ too (two of three of its sentences end with ⁕), every 10th article
   held out. Since data 0.74.0 a news site's pages in it join the wiki's
   text, counted once (note 7): keystroke savings on 3 006 of the wiki's
   held-out words 29.8% (17.4% with taps astray; 30.1% and 16.8% from the
   wiki alone), 18.4% of them unknown (21.5%), and on the news's 49.8%
   (44.4%; 24.3% and 11.6% from the wiki alone); autocorrect changes 0.38%
   of the words typed right.
   Languages with no Wikipedia yet have test wikis in the Wikimedia
   Incubator, whose dump (the same date's, CC BY-SA 4.0) holds them all,
   each article's title starting with its wiki's prefix (`Wp/mag/`).
   Twenty-one are read: Adilabad Gondi's, in the Telugu script (2 301
   sentences from 355 articles, a lexicon of 12 803 words: keystroke
   savings 29.8% on 1 592 held-out words, 27.8% of them unknown),
   Khowar's (6 268 sentences from 2 482 articles, 21 670 words: 40.4% on
   3 085, 13.2% unknown), Montenegrin's, in Latin letters, its few
   Cyrillic sentences left out (54 399 sentences from 1 948 articles,
   900 000 words, a lexicon of 106 217: 36.9% on 9 384, 30.7% with taps
   astray, 8.6% unknown; WIDE, as its lexicon lacks 10.4% of them at
   80 000, where it saves 0.4 points fewer) and Magahi's (33 341
   sentences, beside its FineWeb-2 text: note 7); and fourteen small ones,
   each its language's whole corpus (755 to 3 749 sentences, 17 000 to
   89 000 words, lexicons of 3 032 to 18 628), every 10th of their
   articles held out: Pamona's (keystroke savings 55.5% on 1 848
   held-out words, 48.8% with taps astray, 11.7% unknown), Makassar
   Malay's (44.8%, 35.8%, 20.5%), Ibibio's (43.4%, 36.2%, 14.7%),
   Makassarese's (41.7%, 28.1%, 23.6%), Manado Malay's (41.4%, 31.9%,
   15.8%), Kelantan-Pattani Malay's (40.4%, 30.2%, 19.0%), Abron's
   (39.8%, 30.5%, 17.1%: its sentences that write its ɛ or ɔ,
   `--letters`, as a third of its wiki's are English, references among
   them; from all of them it saved 23.2%), Jambi Malay's
   (36.8%, 26.5%, 24.6%), Rangpuri's (36.0%, 23.6%, 28.3%), Rinconada
   Bikol's (34.6%, 22.8%, 29.8%),
   Musi's (34.2%, 20.5%, 27.5%), Southern Kurdish's (32.4%, 20.8%,
   24.4%), Western Balochi's (29.2%, 20.6%, 20.1%) and East Franconian's
   (25.9%, 8.3%, 28.2%: its wiki writes several of its dialects,
   Nuremberg's and Lower Franconia's among them); right words changed
   0.36% at most, with taps astray. And Vai's, in its own script (7 376
   sentences, 200 000 words, its Latin words left out, every 10th
   article held out), which its keyboard types through its
   transliteration table (note 11): measured through that composer,
   each held-out word typed as the fewest taps on Vai's keys that the
   table reads back as it, until the strip offers it, it saves 37.1% of
   2 990 words' taps (38.3% of them offered before the first key, 2.5%
   unknown, every one offered or written right once typed; 0.3% write a
   letter no key types, an accent, and are left out). FineWeb-2's pages
   in the Vai script are tables of its letters (1 038 of its 1 391
   keyboard.cool's) and copies of the Universal Declaration of Human
   Rights: not used. And Nuosu's, in Yi (since data 0.75.0; 2 691
   sentences, 84 000 syllables, every 10th article held out), beside
   FineWeb-2's text of it (note 7), which its keyboard types through its
   transliteration table too (note 11). Yi writes no space between its
   words and has no list of them to be had, so each syllable is a word, as
   each of Tibetan's is (`osk_data::wordbreak`; the words' generation 8),
   and a sentence is three syllables or more, cut after Chinese's full
   stop and marks (。！？), as its wiki writes them; its lists of links
   (three quarters of its pieces: a bar between links, an Incubator title)
   and its bots' stubs (1 939 sentences: places' areas and peoples,
   Japan's eras) are left out. Typed through the composer, each
   held-out syllable as the fewest taps that the table reads back as it
   and Space, as Keyman's keyboard has them typed, it saves 38.5% of the
   taps of 16 587 syllables (33.8% offered before the first key, 56.7%
   after one letter, 67.5% after two: the strip offers the syllables the
   letters begin; 0.1% unknown). And Eastern Pwo Karen's, in the Myanmar
   script (since data 0.76.0; 1 411 sentences, 194 000 characters, their
   Latin words left out, every 10th article held out), whose keyboard is
   Burmese's keys with its own letters (note 12), cut into syllables as
   Arakanese's is, its word list the 1 522 syllables its wiki writes twice
   or more (`wordbreak/kjp.txt`). It writes the asat after an initial too,
   before the syllable's vowel sign or medial (လ်ု, ဆ်ု, လ်ှ, သှ်ေ: 7 600
   times), where Shan's and Burmese's asat ends a syllable (ဖိုင်, ထာင်,
   25 000 times), so a consonant ends the syllable before only where its
   asat, past its tones (ဝ့်), has no vowel sign or medial after it:
   its three commonest syllables, လ်ု အ်ု ဆ်ု, would otherwise be glued
   to the one before. Keystroke savings 41.7% on 8 005 held-out syllables
   (40.8% with taps astray, 4.2% unknown, 0.53% of right syllables
   changed); 3.2% of its letters are on long presses (ၮ ၯ ႉ, the digits).
   A bot's stubs are left out but three
   of each shape (`wikipedia-text.py --shapes`: a sentence whose four
   words after the first, or last four, digits aside, three kept
   sentences had), which a model would learn and its eval measure as the
   language: 6 579 of Rinconada Bikol's 8 150 sentences (its towns' "A …
   usad na ikatlung klaseng banwaan sa probinsya kan …"), 4 336 of
   Western Balochi's (Germany's towns, the calendar's days), 401 of
   Pamona's (its villages). Southern Kurdish is read as Sorani's keys
   type it (`--sorani`, as Sorani's web text is read: ە where 320 of its
   sentences write ه and a zero-width non-joiner, as the old keyboards
   could; ھ ي ك as ه ی ک). A test wiki's writers paste their
   neighbour's articles in: an article more than 6% of whose words are
   Urdu's, Hindi's, Persian's, Indonesian's or English's own function
   words is left out (257 of Khowar's, 77 of Magahi's, 114 of Jambi
   Malay's, one-line Indonesian stubs of its villages, 51 of Western
   Balochi's), and a sentence more than 15% so. Khowar is typed on
   Urdu's keys with its own letters on the long presses of those that
   sound nearest (ځ ݮ on ج, څ ݯ on چ, ݰ on ش, ݱ on ز), Gondi on
   Telugu's, Montenegrin on Serbian's Latin keys with ś ź on the long
   presses of s z and on its Cyrillic ones (with no model) with с́ з́ on
   с з; the twelve on the US keys (the Malays of Jambi, Makassar and
   Manado as they are; Rinconada Bikol's ñ, Musi's é è ó,
   Kelantan-Pattani Malay's è é ó ũ and Ibibio's ọ ñ n̄ ị ẹ ụ ʌ first on
   the long presses; Makassarese's and Pamona's glottal stop an
   apostrophe key), East Franconian on German's (â ô), Southern Kurdish
   on Sorani's (ۊ on و), Western Balochi on Persian's (ے ں ٹ ڈ ڑ ہ ھ
   ګ on ی ن ت د ر ه گ) and Rangpuri, in the Bengali script as Assamese
   writes it (ৰ for nine of its ten ra, ৱ), on Assamese's, ৎ ঃ on the
   long presses of ত ং as Bengali's are and Bengali's digits on the
   symbol view's; Abron on Akan's (ɛ ɔ on the long presses of e o).
   Haryanvi's test wiki is in the Urdu script and mostly lists
   of names, Pahari-Pothwari's mostly a script page, Chakma's 267
   sentences and Marwari's empty: none is used. Central Malay's (59
   articles, 20 held-out sentences), Laki's (mostly Persian articles),
   Siberian Tatar's (11 000 words), Southern Balochi's (31 000, with
   Urdu's and Persian's text and presentation forms) and Northern Min's
   (7 000 words once its bots' stubs are out) teach no model; all five
   give their keyboards their letters (note 12), as Maguindanao's,
   Capiznon's, Kedah Malay's, Qashqai's, Rejang's, Ghomala's and Isan's
   test wikis do, and the Buginese Wikipedia (nearly all a bot's stubs
   of French communes); and, from 500 to 6 700 words, Ahirani's,
   Harauti's, Kannauji's, Chittagonian's, Hani's, Nandi's, North
   Moluccan Malay's, Bangka Malay's, Banggai's, Cuyonon's, Youjiang
   Zhuang's, Mongondow's, Pokot's and Khorasani Turkic's (note 12).
   Southern Luri's writes two spellings, one with letters of its own (ݖ
   ۿ ڍ ڲ), and Eastern Balochi's is half a garbled transliteration: not
   used. Rakhine's and Sylheti's hold only Wiktionary, Wikivoyage or
   Wikinews pages, as Zeelandic's does (Zeelandic's and Rakhine's
   Wikipedias are wikis of their own: above), and Komering's and Kutai Malay's give their keyboards
   their letters (note 12); Northern Thai has keyboards (notes 10 and 12)
   and no model yet, and Eastern Pwo Karen a model now (above).
7. These three hundred and thirteen models learn from web text, FineWeb-2's (ODC-By 1.0,
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
   are not used, nor its Garo in the Bengali script, all bible.is's Bible
   (note 19). Keystroke savings on
   FineWeb-2's held-out test text, which is not committed, the text
   being the web's: Bodo 48.1%, Brahui 46.4%, Magahi 31.8%, Chhattisgarhi
   30.9%, Dogri 30.1% (its test text only 33 sentences),
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
   text: Khmer 46.6% (9 434 words), Burmese 46.3% (10 803). Their keyboards
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
   53.2%, Sesotho 53.1%, Xhosa 51.4%, Hausa 51.1%, Welsh 50.2%, Malagasy
   49.7%, Yoruba 49.4%, Scottish Gaelic 49.1%, Igbo 49.0%, Chichewa 49.0%,
   Samoan 48.7%, Haitian Creole 48.5%, Cebuano 48.0%, Swahili 47.4%, Māori
   47.3%, Turkmen 46.5%, Somali 46.3%, Javanese 45.5%, Hawaiian 45.1%,
   Dhivehi 45.0%, Tajik 45.0%, Sundanese 44.8%, Frisian 44.5%, Kinyarwanda
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
   held-out text of each (3 100 to 20 000 words; since data 0.84.0 without
   bible.is's pages, note 19): Iu Mien 63.4%, Kadazandusun 56.8%, Tetum
   55.3%, Hiligaynon 54.0%, Karo 53.4%, Jingpho 53.2%, Simalungun 52.7%,
   Pangasinan 51.6%, Iban 51.6%, Central Bikol 48.6%, Hmong Daw 47.5%,
   Batak Toba 46.6%, Hakha Chin 46.6%, Iloko 46.4%, Kinaray-a 46.1%, Banjar
   44.7%, Acehnese 42.9%, Kapampangan 39.2%, Balinese 37.3%; 45.2% on
   average with taps astray, their words unknown 0.5% to 9.8%. Most of Iu
   Mien's text, and of Acehnese's, Pangasinan's, Karo's and Simalungun's,
   is the Bible, as their held-out text is, which flatters them (less since
   bible.is's pages went: Acehnese's text lost a quarter of its words, and
   10.9 points of its savings). With taps astray Hmong Daw's corrector
   changes 1.3% of the words typed right (0.3% when taps land true), its
   words told apart by their last letter, which writes the tone (kev, kej);
   96.3% are right after the space. Shan, Mon and Sgaw Karen, in
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
   and Cyrillic letters for Latin ones) and Venetian (spam) were not used
   then; all but Zeelandic are since 0.42.0, their sites sorted (below).
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
   Hill Mari 50.3%, Moksha 42.6%, Meadow Mari 42.6%, Chuvash 42.4%, Tuvan
   41.9%, Kalmyk 41.7%, Erzya 41.0%, Komi-Zyrian 40.9%, Northern Sami
   40.6%, Karelian 40.5%, Veps 39.5%, Komi-Permyak 38.2%, Udmurt 38.1%,
   Sakha 37.7%, Buryat 37.5%, Livvi 34.1%; Tongan 53.9%, Fijian 52.7%,
   Tahitian 52.3%, Ewe 52.2%, Venda 51.9%, Tsonga 51.2%, Lingala 49.3%,
   Romansh 48.8%, Akan 47.0%, Sepedi 46.7%, Kirundi 46.0%, Setswana 45.5%,
   Southern Ndebele 44.9%, Oromo 42.2%, Bambara 41.9%, Luganda 41.0%,
   Quechua 41.0%, Wolof 34.2%, Greenlandic 33.5%, Swati 28.8%, Aymara
   28.0%, Inuktitut 23.4%; at most 0.66% of right words changed (Wolof's).
   Greenlandic,
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
   54.4%, Abkhaz 51.8%, Adyghe 50.4%, Dargwa 45.3%, Lak 44.5%, Kumyk 44.0%,
   Lezgian 42.7%, Avar 41.4%, Chechen 41.0%, Karachay-Balkar 38.9%,
   Mazandarani 38.7%, Karakalpak 38.3%, Ingush 36.7%, Mingrelian 36.5%,
   Kirmanjki 36.0%, South Azerbaijani 35.5%, Talysh 35.2%, Zazaki 32.0%,
   Kabardian 30.8%; 34.0% on average with taps astray, their words unknown
   4.6% to 18.3%, at most 0.56% of right words changed (Mazandarani's).
   Tabasaran's and Mazandarani's held-out text is small (2 500 and 2 200
   words). FineWeb-2's Southern Kurdish is Sorani's news, its Gilaki
   Persian shops' spam and its Gorani mostly Sorani: neither of the first
   two is used, and of Gorani's only its Hawrami (below).
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
   Baoulé 61.4%, Kikongo 60.0%, Sango 59.7%, Efik 58.1%, Kwanyama 57.9%,
   Umbundu 57.8%, Isoko 56.8%, Kimbundu 55.9%, Fon 55.4%, Jula 55.0%,
   Kikuyu 54.0%, Seychellois Creole 53.8%, Tiv 53.3%, Tshiluba 52.8%,
   Kiluba 52.3%, Ndonga 51.2%, Mossi 50.8%, Acholi 49.8%, Bemba 49.0%,
   Réunion Creole 48.6%, Wolaytta 48.6%, Northern Ndebele 47.8%, Dholuo
   47.6%, Cape Verdean Creole 29.3% (its text spelled both as ALUPEC and
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
   8.7%). Keystroke savings on FineWeb-2's held-out text of each (2 400 to
   14 700 words; since data 0.84.0 without bible.is's pages, note 19): Hiri
   Motu 66.1%, Qʼeqchiʼ 58.2%, Bislama 58.2%, Sranan Tongo 57.1%, Kʼicheʼ
   55.7%, Tok Pisin 55.2%, Yucatec Maya 54.5%, Tuvaluan 53.3%, Kiribati
   50.6%, Chamorro 50.3%, Interlingua 49.0%, Kichwa 42.9%, Guadeloupean
   Creole 42.1%, Papiamento 40.1%, Syriac 35.5%, Cusco Quechua 35.5%,
   Jamaican Patois 35.2%, Latin 26.8%; 43.9% on average with taps astray,
   their words unknown 0.2% to 16.0%, at most 0.60% of right words changed
   (Papiamento's). FineWeb-2's Fiji Hindi is Hindi in Latin
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
   Tibetan and Dzongkha, which had keyboards and no model, learn from
   FineWeb-2's text whole (Tibetan's every 4th row group), a line or a
   sentence ended by a shad (། ༎) each, a run of another script's words
   (Latin, Chinese) cut out. They write no space between words: a syllable
   ends at its tsheg (་), and the syllable is their models' word, kept with
   its tsheg, so a syllable taken from the strip is typed with it and the
   next starts right after, offered at once, as French's next word is after
   l' (words.rs); the last syllable before a shad has none, and a tsheg
   typed before a shad gives way to it, but after ང (rules/bo.weir,
   rules/dz.weir: རོལ། but ཁང་།). Two fifths of FineWeb-2's Dzongkha is
   Tibetan, its Tibetan news sites': a document is kept where Dzongkha's
   own words, counted in syllables (its plural ཚུ, its dative ལུ, its
   copulas ཨིན ཨིནམ, ཡོདཔ, འབད), outnumber Tibetan's (ཚོ, རེད, འདུག, བྱེད):
   10 013 of 19 239, mostly Kuensel's. Dzongkha's keymap types the tsheg on
   the space bar, which ki-osk's keyboards share: its keyboard gains a key
   for it at the end of the home row, Tibetan's has one. Their lexicons
   keep every syllable they read, with its tsheg and without (49 300 and
   13 700), and 90 000 n-grams (180 000 save Tibetan 0.9 points more and
   Dzongkha 0.6, 300 000 1.6 and 1.0, for 1.5 and 3.4 MB more each).
   Keystroke savings on FineWeb-2's held-out text of each (12 400 and
   21 500 syllables): Dzongkha 54.1%, Tibetan 46.5%, 0.2% and 0.02% of
   their syllables unknown, at most 0.65% of right ones changed
   (Tibetan's); with taps astray much the same, a quarter of Dzongkha's
   characters and an eighth of Tibetan's (the tsheg, vowel signs and
   subjoined letters) typed exactly, as no letter key's.
   Twenty of Africa's languages and four of Southeast Asia's, of which
   FineWeb-2 has 1 to 2 MB, learned from its text whole, and all but
   Farefare and Moba still do (they have no model since data 0.84.0, too
   little left without bible.is's pages: note 19), Kabiye's, Minangkabau's
   and Waray's with GlotCC's beside it (note 7's Nigerian Pidgin's way):
   Kabiye, Dagbani, Sena, Dangme, Nkore, Koongo, Sidamo, Konzo, Kamba,
   Songe, Makhuwa, Ndau, Farefare, Bulu, Edo, Teso, Kipsigis, Moba, Yao and
   Dinka, Minangkabau, Waray, Nias and Madurese, each typed on the US keys
   with its letters first on the long presses (the table above). A sentence
   of English is left out, and of French, Portuguese, Swahili, Indonesian
   or Tagalog beside those whose state writes it, and a wiki's table line.
   Most of their text is the Bible and Watchtower, as Africa's
   twenty-four's is. Waray's and Minangkabau's wikis are a bot's, a species
   or a place in a sentence each: their pages are left out, as are
   Dagbani's spam pages and Moba's Romanian and English documents. Letters
   written for others are read as theirs: Kabiye's ʋ (jw.org's) as ʊ (its
   wiki's), Moba's ɑ as a (its Bibles write the same words with either),
   Nias' õ as ö, Sidamo's ˈ as its glottal stop ', Kamba's ī ū as ĩ ũ, a
   Yao font's Ć and Ǝ as Ŵ. FineWeb-2's test split of Dagbani and of Yao
   holds a hundred words: every 10th of their documents is held out
   instead; Maranao, which has none, is not used. Their lexicons keep every
   word they read (Dangme's 6 000 to Nkore's 66 000). Keystroke savings on
   their held-out text (1 700 to 10 800 words; since data 0.84.0 without
   bible.is's pages, note 19): Edo 60.7%, Dangme 56.0%, Koongo 54.6%, Sena
   54.5%, Bulu 54.1%, Nias 52.6%, Waray 51.2%, Ndau 51.1%, Yao 50.8%,
   Kipsigis 49.5%, Songe 49.2%, Kamba 49.0%, Konzo 48.5%, Kabiye 48.4%,
   Makhuwa 48.3%, Teso 44.1%, Dagbani 42.8%, Sidamo 42.0%, Minangkabau
   41.5%, Nkore 39.3%, Dinka 35.1%, Madurese 28.1%; 41.9% on average with
   taps astray, their words unknown 0.5% to 25.1% (Madurese's), at most
   0.90% of right words changed (Nkore's, 0.86% with every tap true, its
   words of many parts 16.3% unknown). A third of Kabiye's letters are
   those its long presses type, as are a sixth of Dangme's and Dinka's.
   Eleven of Europe's regional languages learn from FineWeb-2's text
   whole, each typed on its national language's keys with its own
   letters first on the long presses (the table above): Cornish, Manx,
   Extremaduran, Picard, Samogitian, Pontic Greek, Venetian, West
   Flemish, Lombard, Ladin and Gagauz. A sentence of English is left out,
   and of the state's language by the words the regional one writes
   otherwise (Spanish's como, pero, desde beside Extremaduran's comu,
   peru, dendi; French's beside Picard's, Lithuanian's beside
   Samogitian's, standard Greek's beside Pontic's, Italian's, Dutch's,
   German's), and a wiki's table line. A third of what FineWeb-2 files as
   Lombard is Ladin (La Usc di Ladins on issuu, the province's pages),
   five times what it files as Ladin: a document is Ladin's where Ladin's
   words (y dla dl ie nia tl…) are 2% of its words and more than
   Lombard's (el del anca minga hinn…), Lombard's the other way round.
   Two thirds of its Gagauz is Turkish typed with Cyrillic letters among
   the Latin, Azerbaijani and Kyrgyz: a document is kept where Gagauz's
   words (hem ani taa nicä deyni…) are 1% of its words and more than
   theirs (ve vä menen dep üçün…), and a sentence with a Cyrillic letter
   is left out. Sites in another tongue are left out: Cantabrian's,
   Leonese's, Murcian's and A Fala's beside Extremaduran; a Voynich
   manuscript's transcription and an English book's scan beside Manx;
   Venetian's spam, song sites and old Italian; West Flemish's Low Saxon
   and Brabantian sites and old Dutch; Picard's Norman; Lombard's other
   Gallo-Italic tongues (Trentino's, Emilian's, Piedmont's Occitan).
   Zeelandic's is Middle Dutch archives in the main: not used. Letters
   written for others are read as theirs (Gagauz's ȇ ț as ê ţ,
   Venetian's ƚ as ł, Pontic's ќ as κ, a scanned magazine's ﬁ ﬂ as fi
   fl). Picard's elisions are cut as French's are (`osk_data::words`:
   ch'est is ch' and est, d'nos d' and nos), a ninth of its words being
   written so: keystroke savings 35.7% -> 40.3% at 80 000 words, 28.7%
   -> 34.6% with taps astray. Cut as Italian's, Lombard's, Venetian's and
   Ladin's saved a point at most and lost some with taps astray, their
   clitics written two ways (gh'è, g'è) and corrected one to the other:
   they stay whole. Gagauz reads Turkish's dotted and dotless i, which its
   text writes (99% of the capitals of its i-words are İ), and since
   0.42.0 so do Crimean Tatar, Zazaki and Talysh (94%, 80% and 81%;
   generation 7): their sentences' first words were capitalised Ilk for
   İlk, a word written with İ kept no case, and their savings are the
   same (Zazaki's 0.1 points more). Kirmanjki writes I for 73% of them, as
   Kurdish does, and keeps its rules. Pontic Greek ends a question with ;
   as Greek does. Their lexicons keep 80 000 words (Manx's all its
   59 800, Pontic's its 44 300), 120 000 for Picard, Extremaduran,
   Venetian, Lombard and West Flemish, 200 000 for Samogitian (all its
   126 500). Keystroke savings on their held-out text (4 900 to 14 100
   words): Manx 51.6%, Ladin 44.5%, Gagauz 43.2%, Cornish 42.0%, Picard
   40.6%, West Flemish 39.8%, Extremaduran 37.9%, Venetian 35.7%,
   Lombard 31.1%, Samogitian 29.1%, Pontic Greek 27.6%; 31.6% on average
   with taps astray, their words unknown 2.2% (Ladin's) to 20.6%
   (Pontic's), at most 0.72% of right words changed (Lombard's). Each of
   them writes as its writers speak, in several spellings (Cornish's
   Kemmyn, Unified and Standard Written Form; Lombard's Milanese and
   Eastern Lombard), and Pontic's and Samogitian's text is small: a fifth
   and a sixth of their held-out words are unknown.
   Shan, Mon, Sgaw Karen and Pa'O, written in the Myanmar script without
   spaces between words, learn from FineWeb-2's text whole (Shan's 28.7
   million characters, Mon's 8.4, Karen's 5.8, Pa'O's 0.8), cut into
   syllables, as Tibetan is: no list of their words is to be had, so each
   one's word list is the syllables its text writes twice or more (`make
   syllable-lists`, 2 200 to 8 800 of them; `osk_data::wordbreak`), its
   words' as the keyboard reads them, so not the symbols ၌ ၍ ၎ ၏, which
   end a word as punctuation does (since data 0.63.1), and
   the keyboard types no space after a syllable from the strip. A
   syllable starts on a letter, but not on a sign, nor on a consonant
   stacked under another, nor on one the asat after it ends a syllable
   with, as Burmese's words do; a Mon consonant stacked over the next
   starts one (ပ္ဍဲ, a minor syllable), and Karen's asat is a tone that
   ends no consonant (ဒ်, like, is a syllable). Their lists with their
   3 000 commonest pairs of syllables as words besides saved less (Shan
   53.2%, Mon 34.0%). Shan is typed on xkeyboard-config's Shan keys, Mon
   on its Mon keys, Pa'O on Burmese's with its tones ꩻ and ႏ (an eighth
   of its letters) in place of ဩ and ၏, and Sgaw Karen on Keyman's
   sil_sgaw_karen (SIL Global, MIT), a phone's layout whose keys type its
   commonest signs with the asat in one tap (ၣ် ၢ် ာ်): on the base view
   each key types one letter, the asat a key of its own, as the corrector
   takes a tap for a letter (with keys of two letters it changed one
   Karen word typed right in fifteen). Their letters are where Zawgyi put
   its shapes, so a document is Zawgyi's by ေ before its consonant or ္
   ending a syllable alone (a third of Pa'O's, Burmese blogs; 0.3% and
   less of the others'); a sentence whose syllables are more than 4%
   Burmese's own (သည် များ တွင် ၏…) is left out, and a Pa'O document
   writes its tones, 2% of its letters and more. Karen's older fonts'
   letters are read as Unicode's (ၦ as ပှ, ၥ as ာ, and Shan's tones ႉ ႋ
   and ႇ, written for its full stop and comma, as . and ,), and Mon's ၚ,
   written as Burmese's င nearly a quarter of the time (more than half of
   the time in its wiki), as ၚ, which its keyboard types. Their lexicons
   hold all their syllables (3 300 to 12 900 with the runs no list
   holds). Keystroke savings on their held-out text (16 300 to 48 000
   syllables; Pa'O's every 10th document, its test split's 31 sentences
   too few): Shan 53.5%, Sgaw Karen 47.3%, Pa'O 40.1%, Mon 35.3%; 52.9%,
   47.1%, 39.5% and 34.9% with taps astray, at most 0.96% of right
   syllables changed (Mon's).
   Four of East Asia's languages written in Latin letters learn from
   FineWeb-2's text whole: Southern Min in Pe̍h-ōe-jī, the church
   romanization (its Wikisource's and Wikipedia's text, the Bible, church
   and literary sites; 644 000 words), Hakka in Pha̍k-fa-sṳ (its
   Wikipedia, the Bible; 199 000), Amis (its Wikipedia; 344 000) and
   Zhuang, the standard written language, which FineWeb-2 files as
   Yongbei Zhuang's (the People's Daily's and the Guangxi Minzu Bao's
   Zhuang pages, its Wikipedia; 375 000). Their text's Han runs (Hàn-lô
   text, a wiki's menus) are left out, a document is kept where its own
   words are 1% of its words and more than English's, and Hakka's spam
   sites and the Indonesian Wiktionary's pages, and a parallel corpus's
   markup beside Amis, are left out. Southern Min's ⁿ, which Unicode
   names a superscript, counts as a Latin letter where the Latin runs are
   kept (every word with it had been cut away), its i written as the
   dotless ı under a tone mark (ı̄ ı̍ ı̀ ı̂, 30 000 of them) is read as i,
   and its o͘ written o· as o͘; Amis' glottal stop, written ’ mostly and
   ˈ ' ‘ ` ´ ʼ and ^ in the same words, is ’. Southern Min and Hakka
   join a word's syllables with hyphens (half of Southern Min's words
   are written so): cut at each hyphen, the next syllable glued on as
   Kabyle's clitics are (`osk_data::words`), so that a syllable after a
   hyphen is offered at once, they saved 3.2 and 4.7 points more than
   whole words, 5.6 and 9.1 with taps astray, and their unknown words
   fell from 6.3% and 10.8% to 2.5% and 2.6%. They are typed on the US
   keys, Southern Min's and Hakka's toned vowels first on the long
   presses, the commonest first (Southern Min's á â ā à a̍, o͘ in its five
   tones, ⁿ first on n; Hakka's â à á a̍, ṳ in its five on u: a quarter
   and a sixth of their letters), Amis' glottal stop a key of its own at
   the home row's end (one word in fifteen has it), Zhuang's tones being
   letters. Their lexicons keep all their words (8 400 to 34 000).
   Keystroke savings on their held-out text (Southern Min's test split,
   the others' every 10th document, theirs too small; 10 200 to 15 200
   words): Zhuang 61.9%, Hakka 54.9%, Southern Min 49.0%, Amis 46.1%;
   60.6%, 53.9%, 47.7% and 40.2% with taps astray, at most 0.37% of right
   words changed (Amis'). Hakka capitalises each syllable of a
   place's name (Sî-Fî), and a capital after a hyphen cannot be
   foreseen: 89% of its words come out in the right case.
   Twenty-eight of Africa's languages that FineWeb-2 has 0.4 to 0.9 MB of
   learned from it whole, the Bible's and Jehovah's Witnesses' text in the
   main: Zande, Alur, Chopi, Sakalava Malagasy, Akoose, Mende, Lugbara,
   Konkomba, Dan, Northern Grebo, Hehe, Gourmanché, Gamo, Sukuma, Supyire,
   Basaa, Haya, Kafa, Lomwe, Gogo, Chuwabu, Fante, Tswa, Ntcham,
   Xaasongaxango, Gulay, Nande and Nyoro. Without bible.is's pages (since
   data 0.84.0, note 19) eighteen still do, from 27 000 (Hehe) to 390 000
   words (Zande), Gamo beside two of its Bibles since data 0.85.0 (note
   18); Northern Grebo, Haya, Kafa, Gogo, Ntcham, Xaasongaxango, Gulay,
   Nande, Nyoro and Dan have too little left, and no model. A sentence of
   English is left out, and of the state's language: French's (une pour
   dans avec…: not le, which several of them write), Portuguese's beside
   Mozambique's four, and beside the Bantu languages of East Africa
   Swahili's own words (katika kwamba lakini…: not kwa ni wa ya, which they
   write too); every 10th document is held out, their test splits holding a
   few hundred words. Letters written for others are read as theirs:
   Fante's ͻ (a Greek look-alike) ɑ ε as ɔ a ɛ, Lugbara's ʉ as the ᴜ its
   text writes forty times as often, a scan's ﬁ as fi. They are typed on
   the US keys, each language's letters first on the long presses, the
   commonest first, with their tones where its text writes them (Akoose's ɛ
   ə ɔ ŋ and seven tones between them, Northern Grebo's, Supyire's,
   Ntcham's on b l m n too); the eleven that write an apostrophe in 3% to
   11% of their words (Sukuma's ng'wa, Gulay's le’n, Nande's n'abandu) have
   a key for it at the home row's end, Akoose's typing its glottal stop, ʼ,
   which its text writes and its words keep (`osk_data::words`, as
   Kʼicheʼ's do). Their lexicons keep all their words (2 200 to 24 000;
   Gamo's 64 000 with its Bibles). Keystroke savings on the eighteen's
   held-out text (800 to 13 400 words): Konkomba 63.8%, Zande 54.1%, Chopi
   53.1%, Tswa 51.8%, Lomwe 51.5%, Sakalava 51.2%, Alur 51.1%, Basaa 50.1%,
   Supyire 50.0%, Lugbara 49.9%, Gourmanché 49.6%, Mende 46.8%, Akoose
   46.3%, Gamo 44.3%, Chuwabu 43.2%, Fante 43.0%, Sukuma 34.9%, Hehe 29.0%;
   48.0% on average, 42.7% with taps astray, at most 0.41% of right words
   changed (Lugbara's). The long presses' letters weigh in the savings: a
   third of Akoose's letters are typed so, which the strip spares. Hehe's
   and Sukuma's held-out words are 30% and 22% unknown, their texts now
   small (27 000 and 32 000 words), Chuwabu's and Akoose's 13% and 12%,
   their words of many parts (Gamo's 5.2%, 14.0% without its Bibles).
   Twenty-three more of the long tail learned from FineWeb-2's text whole:
   Southeast Asia's, the Americas' and the Pacific's that it has 0.4 to 1.2
   MB of (Niuean, Hawaiian Pidgin, Saint Lucian Creole, Maranao, Ngaju,
   Wayuu, Lampung Api, Sasak, Gorontalo, Itawit, Buhid, Koronadal Blaan,
   South Bolivian Quechua, Guianan Creole) and Africa's with 0.2 to 0.4 MB
   (Tumbuka, Tetela, Gusii, Makonde, Nyakyusa, Maasai, Mandinka, Meru,
   Dagaare), the Bible's in the main, Niuean's New Zealand's health pages
   too, Guianan Creole's and Dagaare's their wikis, Maasai's, Meru's and
   Gusii's Kenyan news and radio. Without bible.is's pages (since data
   0.84.0, note 19) eighteen still do, from 25 000 (Gorontalo) to 640 000
   words (Niuean); Lampung Api, Sasak, Itawit, Buhid and Koronadal Blaan
   have too little left, and no model. A sentence of English is left out,
   and of the state's language (Indonesian's, Tagalog's, Spanish's,
   French's, Portuguese's, Swahili's own words); Maranao's Arabic (sermons,
   the Quran) is left out by script, the LOLcat Bible beside Hawaiian
   Pidgin and a book pirate's pages beside Itawit by site; Meru's ĩ written
   ῖ is read as ĩ; every 10th document is held out. They are typed on the
   US keys, their letters first on the long presses; Gorontalo, Tetela,
   Gusii and Wayuu have an apostrophe key at the home row's end, Wayuu a ü
   key beside it, and South Bolivian Quechua an ʼ key for its ejectives,
   which its words keep as Cusco Quechua's do. Their lexicons keep all
   their words. Keystroke savings on the eighteen's held-out text (1 800 to
   13 000 words): Saint Lucian Creole 60.7%, Hawaiian Pidgin 59.2%, Niuean
   52.9%, Ngaju 52.8%, Mandinka 48.6%, Maranao 47.2%, Tetela 47.2%, Makonde
   45.0%, Tumbuka 43.5%, Nyakyusa 42.9%, Guianan Creole 42.5%, Meru 41.9%,
   Wayuu 40.8%, Maasai 40.5%, Dagaare 40.0%, Gusii 38.4%, Gorontalo 38.0%,
   South Bolivian Quechua 28.9%; 45.1% on average, 36.7% with taps astray,
   at most 0.42% of right words changed (Gusii's). The smallest texts know
   least of what is typed: Gorontalo's and South Bolivian Quechua's
   held-out words are 26% and 25% unknown, Wayuu's, Gusii's and Maasai's
   16% to 19%; with taps astray South Bolivian Quechua saves 7%, the others
   24% to 31%. Makassarese is not used: its Bible writes the glottal stop '
   and its chronicles q, and it saved 27% (8% with taps astray), a third of
   its held-out words unknown.
   Coptic learns from FineWeb-2's Bohairic, the dialect of the Coptic
   Church's liturgy (207 000 words: the New Testament on several sites, the
   liturgy as tasbeha.org's readers write it out, the Fathers on
   Wikisource, its test wiki): its runs of Coptic alone, the Arabic,
   English and French beside them left out. Its jinkim, the grave over a
   letter that is a syllable of its own, written ⳿ before the letter
   (⳿ⲛⲧⲉ), a backtick or a lone grave before it, or a dot over it, is read
   as Unicode writes it, the grave after the letter (ⲛ̀ⲧⲉ); the nomina
   sacra's ⳪ as ϭ̅ⲥ̅; a number in letters (ⲁ̅, ⲓ̅ⲃ̅: the verses') is no
   word; and a sentence whose letters are those of one read before (a
   verse's copy on another site) is left out. Sahidic is not used, a
   document being Bohairic's by its words (ⲟⲩⲟϩ ⲛⲉⲙ ϧⲉⲛ against ⲁⲩⲱ ⲙⲛ̅
   ϩⲛ̅): Sahidic writes Bohairic's words with its stroke where Bohairic
   writes the jinkim (ⲛ̅ⲧⲉ, ⲛ̀ⲧⲉ), and beside it autocorrect changed one
   Bohairic word typed right in 27 (0.15% without it); three editions of
   its New Testament are half of FineWeb-2's Coptic. Coptic is typed on
   its keymap, eg(cop), with a key for the jinkim at the home row's end and
   the overline of the nomina sacra (Ⲓⲏ̅ⲥ̅) on its long press. Lahu learned
   from its Bible and Jehovah's Witnesses' pages (344 000 words), its tone
   ꞈ written as a caret, ‸, read as ꞈ; since data 0.84.0, bible.is's Bible
   left out (note 19), 36 000 words are left, of fewer than a thousand
   kinds, and it has no model. It is typed on the US keys with its tones on
   keys: ˬ and ˇ (a fifth of its words end in each) at the home row's end,
   ꞈ, its apostrophe and ˍ on q, x and z, which it does not write, ˆ and ˉ
   on the long presses of ꞈ and ˍ. Ten more of the long tail that FineWeb-2
   has 0.15 to 0.3 MB of learn from its text whole: Hunsrik (its standard
   spelling's, ESCRITHU's, documents, Jehovah's Witnesses' and Brasil
   Alemanha's: a blog in German's spelling is left out), Plains Cree (its
   long vowels, written with the macron fifteen times as often as with the
   circumflex, with it), Swampy Cree (its syllabics left out), Ottawa,
   Tokelauan (New Zealand's government's pages), Kankanaey and Mamasa (the
   Bible), Livonian (its wiki and its institute's pages: a Kwak'wala prayer
   book and a Kurdish book filed as Livonian are left out), Ambonese Malay
   (its news, blogs and songs: sites that write chords between its words
   are left out) and Aja (Jehovah's Witnesses', its ʒ written ӡ read as ʒ;
   Ewe's localisation of a browser left out): 31 000 (Swampy Cree) to 93
   000 words (Tokelauan). A sentence of English is left out (beside the
   Crees and Ottawa by its commonest words, in, is, of, on, too), and of
   the state's language (Portuguese's, Tagalog's, Indonesian's, Latvian's
   and Estonian's, French's own words). They are typed on the US keys,
   Livonian on Latvian's, their letters first on the long presses; the
   letters a fifth to a third of their words write have keys of their own
   at the home row's end (Plains Cree's ā ē, Swampy Cree's â ê, Livonian's
   õ, Aja's ɔ ɛ), and so does Ottawa's and Mamasa's apostrophe. Their
   lexicons keep all their words (4 400 kinds, Tokelauan's, to 35 000,
   Coptic's). Keystroke savings on their held-out text (2 300 to 10 400
   words; since data 0.84.0 without bible.is's pages, note 19): Mamasa
   55.0%, Kankanaey 51.9%, Tokelauan 49.5%, Hunsrik 49.5%, Aja 47.9%,
   Ambonese Malay 41.6%, Coptic 39.0%, Ottawa 33.3%, Plains Cree 31.4%,
   Livonian 29.0%, Swampy Cree 25.2%; 41.2% on average, 30.5% with taps
   astray, at most 0.26% of right words changed (Ambonese Malay's).
   The Algonquian languages write words
   of many parts, few of which so small a text holds: Plains Cree's,
   Ottawa's and Swampy Cree's held-out words are 31% to 37% unknown, and
   with taps astray they save 10% or less (Swampy Cree nothing); Livonian's
   are 26% unknown, and it saves 18% with taps astray. Thirteen more of the
   long tail that FineWeb-2 has 0.1 to 1.4 MB of learn from its text whole:
   Nenets (its newspaper's and Yamal's pages; the Russian translation the
   newspaper puts after each article is left out, by Russian's words and by
   its writing neither ӈ nor a glottal stop, one of which nearly every
   Nenets sentence writes), Dungan (Kazakhstan's and Kyrgyzstan's Dungan
   papers and its Incubator wiki: English songs written in Cyrillic letters
   are left out), Tausug (the Bible, Ahmadiyya and Sufi pages: its Arabic
   pages are left out by script), Ewondo (news, Jehovah's Witnesses'),
   Tooro (the Bible, a church: magazine scans read by OCR are left out),
   Makhuwa-Meetto (the Bible, Cabo Delgado's radio news), Kanuri (Dandal
   Kura's radio news, its Incubator wiki), Giryama (the Bible: Kikuyu radio
   pages are left out), Lango (the Bible, news), Hanunoo (the Bible:
   another Mangyan language's recordings are left out), Toraja-Sa'dan (the
   Bible, a culture blog: Indonesian Wiktionary's word list is left out),
   Ledo Kaili (Palu's news) and Sassarese (news, stories): 26 000
   (Toraja-Sa'dan) to 166 000 words (Nenets). Letters written for others
   are read as theirs: Nenets' two glottal stops, written ’ and ’’ (and ʼ,
   ʼʼ, ”), as ’ and ˮ, the letter its keyboard types for the second (two
   apostrophes are one to the corrector, which puts back an apostrophe a
   word was typed without: beside ’’ it changed one Nenets word typed right
   in 80, ненэцие’ to ненэцие’’), and its ң as ӈ; Kanuri's ǝ and ә, and on
   Dandal Kura's site the digit 3 beside a letter (d3, k3re), as ə;
   Dungan's Latin ə and ǝ as ә; Tausug's saltillo ꞌ, the Bible's glottal
   stop, as the ’ its other sites write; Ewondo's ǝ as ə. A sentence of
   English is left out, and of the state's language (Russian's, Tagalog's,
   French's, Portuguese's, Swahili's own words, Indonesian's, Italian's).
   Nenets is typed on Russian's keys, ӈ in place of the ъ it hardly writes
   and its glottal stops, the apostrophe and ˮ, at the home row's end;
   Dungan on them too, ә in place of ъ, җ at the home row's end and ү ў on
   у's long press; the rest on the US keys, Sassarese on Italian's, their
   letters first on the long presses, with an apostrophe key for those
   whose words write one (Tausug, Tooro, Giryama, Toraja-Sa'dan, Sassarese)
   and a key for Kanuri's ə, which a quarter of its words write. Their
   lexicons keep all their words (3 500 kinds, Hanunoo's, to 28 000,
   Nenets'). Keystroke savings on their held-out text (1 600 to 9 700
   words; since data 0.84.0 without bible.is's pages, note 19): Hanunoo
   54.4%, Tausug 52.0%, Makhuwa-Meetto 49.2%, Giryama 43.4%, Nenets 42.6%,
   Ledo Kaili 42.3%, Lango 42.0%, Ewondo 40.2%, Toraja-Sa'dan 35.2%, Dungan
   34.5%, Kanuri 34.1%, Tooro 27.4%, Sassarese 20.8%; 39.9% on average,
   29.9% with taps astray, at most 0.58% of right words changed
   (Sassarese's). Tooro's and Sassarese's texts are small for
   how their words are made and spelled (Tooro's of many parts, Sassarese's
   spelled several ways, th and dh, k and ch): a third of their held-out
   words are unknown, and with taps astray they save 6% and 2%. Learning
   from Nyoro's text too, which shares Tooro's written standard, Tooro
   knew more of its words (29% unknown) but autocorrect changed 1.5% of
   its right ones: it is not used.
   The two Romanis learn from both of FineWeb-2's Romani files, for each
   holds the other's documents beside Spanish spam (two thirds of its Vlax
   Romani) and Macedonian in Latin letters (three fifths of its Balkan
   Romani): a document is the dialect whose own words it writes more of
   (Vlax's sas naj ande kodo…, Balkan's sine taro hem isi…), and is kept in
   the international standard's spelling, more of č š ž than of sh ch zh,
   which Serbia's, North Macedonia's and Sweden's Romani text writes:
   Balkan Romani's news (Roma Times, 24 Vakti, Romapress), Jehovah's
   Witnesses' and Sweden's authorities' pages; Vlax Romani's rights groups'
   (ERRC), a Romani blog, Sweden's authorities' Kalderash, Croatia's and
   Serbia's Romani sites and the Bible. Sweden's Romani radio, 6 MB of
   Kalderash spelled with digraphs (sh ch zh), and the Romani written in
   Romanian's, Hungarian's, German's or Albanian's letters are not used:
   each is one country's own, and the model spells as one. 500 000 words
   (Vlax) and a million (Balkan). Vlax Romani is typed on the US keys, its
   č ć š ž đ first on the long presses of c s z d, and Balkan Romani on
   Serbian's Latin keys, which have them all. Northwestern Ojibwe learns
   from its syllabics, mostly the Bible (gospelgo's; bible.is's left out
   since data 0.84.0, note 19): 86 000 words (115 000 before), the w-dot
   written apart before its syllable (ᐧᐃ) and West Cree's dot on the right
   (ᐏ) read as the one syllable with the dot on its left (ᐎ), as its
   keyboard types it. It is typed on Keyman's fv_ojibwa (FirstVoices), a
   syllable a key, the long vowels and the w-dotted syllables (in 55% and
   37% of its words), which Keyman types with keys of their own by its
   rules, first on the syllables' long presses. Keystroke savings on their
   held-out text (700 to 10 600 words, Northwestern Ojibwe's 88 sentences):
   Northwestern Ojibwe 55.2%, Balkan Romani 42.5%, Vlax Romani 33.4%; with
   taps astray 52.4%, 37.3% and 24.7%; at most 0.06% of right words
   changed. Ge'ez,
   which FineWeb-2 has no text of, is typed as Amharic is (layout-only).
   Kabiye, Akoose, Grebo and Nande got keys for the letters their words
   write most of those only a long press typed (Kabiye's ɛ ɩ ʊ ɔ, Akoose's
   é á ɛ, Grebo's ɛ ɔ ǒ ě, Nande's ĩ ũ): the same held-out text takes 4% to
   11% fewer taps with the strip (Kabiye 10.5%, Akoose 8.2%, Grebo 5.0%,
   Nande 4.3%), and their keystroke savings, counted against typing it out,
   which takes 5% to 16% fewer too, are 0.5 to 3.2 points lower (Kabiye
   48.4%, Akoose 65.2%, Grebo 56.7%, Nande 48.1%).
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
   unknown), Mizo 42.1% (3.1%), with 30 000 words each; 80 000 would
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
   Amharic and Tigrinya, which FineWeb-2's classifier mistakes for each
   other, learn from the documents whose own function words (Amharic's
   ነው እና ላይ ውስጥ, Tigrinya's እዩ ኣብ ናይ ምስ) outnumber the other's: every
   12th of Amharic's 429 row groups (35 538 of 36 000 documents, 833 304
   sentences, 14.6 million words) and every 2nd of Tigrinya's 73 (35 784
   of 36 033 documents, 678 831 sentences, 13.0 million words), their
   words cut at the Ethiopic wordspace and punctuation and their
   sentences ended at its full stop ። and question mark ፧ as well. Their
   keyboards type through the Ge'ez composer, whose strip offers their
   words (note 16). Sebat Bet Gurage's was read so too (data 0.68.0 to
   0.83.0), whole, against Amharic's function words (its own ሰብ ኸማ ዝኽ
   ቃር, which Amharic writes ሰው እንደ ይህ ቃል); its text was its New
   Testament from bible.is, a story by Sahle Selassie and a few articles,
   and with no page of bible.is read (note 19) too little is left for a
   model.
   Gorani's keyboard (Keyman's, note 10) had no model: FineWeb-2's Gorani
   (hac_Arab, 16 201 documents, 38 MB) is mostly Sorani, Badini and
   Persian, and its Hawrami is read alone (since data 0.74.0): a document
   whose own function words (جە, پەی, ئی, چەنی…, found on the Hawrami
   sites' pages and not on Sorani's: جە on 93% of hewraman.net's,
   horam.info's, hawramannews.com's and ANF's Hawrami pages, on 26% of
   Sorani news') outnumber Sorani's, Badini's and Persian's (لە, on 77% of
   Sorani's pages and 6% of Hawrami's), 2 878 of them; read as Keyman's
   horami types it (the soft d written د٘ as ڎ, as most of its pages
   write it, ۊ as the keyboard's ۉ, Arabic's kaf, yeh, alef maksura and
   heh doachashmee as Kurdish keyboards type them), its sentences in
   another spelling, whose vowels its keys do not type (ٙ ٚ ۍ ې), left
   out, and its Latin words cut out: 34 270 sentences, 771 000 words,
   146 000 kinds of word, all kept (WIDER: at 80 000, 12.6% of its
   held-out words unknown, 10.3% at 200 000). Its test split's Hawrami is
   26 documents, two of them half its sentences, so every 20th document
   of its train split is held out instead: keystroke savings 41.3% on
   3 018 words (30.9% with taps astray, 10.3% unknown, 0.23% of right
   words changed); on the test split's it saves 22.3%, one of those two
   documents in the old spelling that writes ە as ه (نهوروز for نەورۆز).
   And Sylheti's in Syloti Nagri (since data 0.74.0), FineWeb-2's und_Sylo:
   a news site's pages (surmafarorkhobor.com, 3 404 of its 4 881
   documents) beside the Sylheti wiki's text (note 6), the Incubator's
   copies of that wiki and the character tables left out, its sentences
   ended at ⁕ and the poetry marks too: 8 380 sentences, 116 000 words,
   every 20th document held out (it has no test split), counted once
   beside the wiki's (WEB_EVEN_LANGS: at 5, as Magahi's web text counts,
   the wiki's held-out articles were saved 27.7%, its right words changed
   0.73%).
   And Nuosu's in Yi (since data 0.75.0), beside its Incubator wiki's text
   (note 6): FineWeb-2's und_Yiii (Yi, its language undetermined, 3 878
   documents) is mostly the web's tables of the script's characters
   (keyboard.cool's 2 381 pages, unicode-table.com's, the fancy-text
   sites') and pages of syllables strung at random. A page is read when a
   tenth of its syllables are Nuosu's twelve commonest (ꃅ ꌠ ꇬ ꆀ ꄉ ꏦ ꇩ ꂷ
   ꀋ ꆏ ꅉ ꍏ: 21 to 26% of People's Daily's Yi pages', 12 to 30% of the
   Incubator's, 1 to 3% of the tables'), the Incubator's copies of its
   wiki left out: 323 documents (People's Daily's Yi edition, Liangshan's
   government, yizuren.com, a Bible dictionary), their lines and sentences
   of three syllables or more, 2 711 of them, 38 850 syllables. Counted
   once beside the wiki's (WEB_EVEN_LANGS), the wiki's held-out articles
   are saved 38.5% of their taps (37.2% at 5); its own held-out documents'
   sentences are eight, most of their pages' lines being the corpus's
   already (39.8% on them).
8. These seven models learn from web text, MADLAD-400's (Google; CC BY 4.0
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
   Tandroy-Mahafaly Malagasy's, Hawu's, Kituba's and Ayacucho Quechua's
   are MADLAD-400's too, mostly their Bibles and the Jehovah's Witnesses'
   pages, and in their noisy files as much spam and English: a document
   is kept where its own words are 5% of its words (Ayacucho Quechua's
   3%) and more than its rival's, Tandroy's ty amy zay laha lafa against
   standard Malagasy's ny amin'ny izay raha rehefa, Hawu's do nga ri
   ddau dhara against Indonesian's, Kituba's yandi mono yina mpi sambu
   against Lingala's (ezali mpe oyo) and French's, and Ayacucho
   Quechua's mana chay hinaspa nispa (its text writes no ejective, as
   Cusco's would); Hawu's apostrophe between letters is read as the
   saltillo ꞌ its Bible writes. Kept: Tandroy 834 of 1 950 documents
   (23 417 sentences, 327 000 words), Hawu 405 of 937 (10 675, 245 000,
   every 10th held out), Kituba 2 325 of 3 467 (102 802, 2.4 million),
   Ayacucho Quechua 586 of 666 (31 324, 332 000). Keystroke savings on
   the held-out documents, with taps astray and the words unknown:
   Kituba 60.8%, 60.0%, 0.3%; Hawu 57.3%, 55.5%, 1.5%; Tandroy 52.8%,
   49.6%, 2.6%; Ayacucho Quechua 42.3%, 35.1%, 9.0%; flattered, as
   Krio's are, by a Bible's held-out chapters being much like the ones
   it read. Tandroy is typed on Malagasy's keys with à è é ò ì ỳ first on
   the long presses, Hawu on the US keys with è first on e and a key for
   its saltillo at the home row's end, Kituba on them as they are, and
   Ayacucho Quechua on Quechua's, with á é first on the long presses.
   MADLAD-400's Hmong is White Hmong in the RPA's letters (lawv, Hmoob),
   much of it machine-translated pages: Gboard's Hmong beside its Hmong
   Daw is the language the Hmong Daw pack (note 7) types and predicts.
   Its Eastern Balochi is Roman Urdu, its Sabah Malay Peninsular Malay's
   blogs, its Nung Vietnamese spam and its Takestani Persian: none is
   used.

9. Thirty keyboards of languages no text here predicts take their
   letters from CLDR's exemplar characters (Unicode License v3; release
   49, `scripts/build-exemplar-layouts.py`): ten from CLDR's own locales
   (Afar, Betawi, Kiga, Choctaw, Zarma, Lojban, Lakota, Mundang, Navajo,
   Soga) and twenty from its exemplars directory, unconfirmed (Berom,
   Chavacano, Dazaga, Fang, Gor, Ebira, Lama, Lendu, Kituba, Muna, Duri,
   Nupe, Papuan Malay, Guinea-Bissau Creole, Ngambay, Cebaara Senoufo,
   Soninke, Serer, Aborlan Tagbanwa, Temne; Kituba's has a model now, its
   Bible's, note 18), and two in Latin letters
   whose CLDR file is of another script, the Latin set in a file of its
   own there: Adamawa Fulfulde (`fub_Latn`; its main file's set is Ajami)
   and Bench (`bcq_Latn`; its main file's is Ethiopic). Each is typed on the US
   keys, every letter they lack first on the long press of the one it is
   made from: by its decomposition (ą́ on a), by its Unicode name (ɓ on b,
   ɨ on i, ʋ on v), or by the vowel or consonant it writes (ɛ ə on e, ɔ on
   o, ŋ on n, ʌ on a); its capital on the Shift view's. A glottal stop
   written as a letter (Aborlan's and Muna's ꞌ, Lendu's ʼ, Cebaara's ˀ)
   is a key of its own at the home row's end. Where a language's text
   could be read, FineWeb-2's or its Incubator test wiki's (900 to 41 000
   words, too few for a model), it decided what the list could not: the
   letters it writes that the list lacks (Ngambay's ə ɓ ɗ ŋ, its tones
   and its creaky vowels ḛ ḭ a̰ o̰; Fang's é è ô, as its Bible's and the
   Jehovah's Witnesses' pages spell; Berom's tones; Mundang's ẽ;
   Choctaw's u̱; Adamawa Fulfulde's glottal stop the ’ its web text
   writes, eight times as often as ', for the list's ʼ), an apostrophe key for the ten whose words write one
   (Duri's in a fifth of its words, Gor's, Ngambay's, Soga's…), Navajo's
   ʼ (its text's, not the list's ’), and keys for the letters its words
   write most of those only a long press would type: at the home row's
   end those a fifth of its words write, and where long presses would
   still be in a quarter of its words, the next most written there and
   in place of q and x, which it hardly writes (q and x on those keys'
   long presses). The words a long press is needed in: Navajo's 78.5% ->
   37.1% (ʼ í at the home row's end, á é on q x), Lakota's 71.4% -> 37.6%
   (ŋ á, č é; its ʼ on the full stop's long press), Temne's 61.4% ->
   18.3% (ɔ ŋ, ə ɛ), Gor's 60.4% -> 2.0% (ɨ and its apostrophe, ə ɔ),
   Ngambay's 59.2% -> 29.3% (ə and its apostrophe, é ɔ) and Mundang's
   29.8% -> 20.0% (ǝ and its apostrophe). None had a model: neither
   suggestions nor glides; Zarma's now learns from SMOL's translations
   (note 14), and Betawi's and Nupe's from their wikis (note 6).
   Twenty-five more, in other scripts, are typed on their script's keys
   the same way, the letters those lack on a long press (six from CLDR's
   own locales: Torwali, Balochi, Bakhtiari, Northern Luri, Haryanvi and
   Kangri, which name themselves; eighteen from its exemplars directory).
   In the Arabic script, on whichever keys lack fewest of a language's
   letters: Urdu's for Balti, Gujari, Indus and Kalam Kohistani, Northern
   Hindko, Pahari-Pothwari, Shina, Torwali, Balochi and Hazaragi; Sindhi's
   for Dhatki and Parkari Koli; Persian's for Bakhtiari and Gilaki;
   Sorani's for Northern Luri; Arabic's for Maba. A letter is on the key of
   the one its Unicode name says it is made from (ڗ, REH WITH TWO DOTS
   ABOVE, on ر; ݨ ݩ on ن; څ on ح), or the one it is drawn from (ڇ on چ,
   ڃ ڄ on ج); Arabic's ي ك and Urdu's ه as the keys' ی ک ہ; a vowel or
   honorific mark the keys lack on the full stop's long press, where
   Persian's keys have theirs (Pothwari's ؐ ؑ ؓ ؔ, Northern Luri's ٙ ٛ).
   In Devanagari, on Hindi's InScript keys, which have them all (the
   candra ॅ ऍ on their long presses, first for Haryanvi and Kangri) but
   Kangri's ॐ (on ओ) and avagraha ऽ and Limbu's glottal stop ॽ (on the
   visarga ः, as Sanskrit's ऽ is): Bagheli, Bagri, Eastern Tamang,
   Haryanvi, Kangri, Limbu (its Devanagari spelling; Limbu's own script
   needs a keyboard), Sherpa and Surgujia. A letter typed as the letters
   and marks it is made of (ड़ as ड and the nukta, کٔ as ک and the hamza
   above) needs no key of its own. Eastern Tamang's, Bagheli's, Bagri's,
   Sherpa's, Surgujia's, Northern Hindko's and Bakhtiari's letters are all
   on their keys already: their keyboards are those keys, in their name.
   Bagheli's, Bagri's, Kangri's and Surgujia's models now learn from
   SMOL's translations (note 14), and Gilaki's from its wiki (note 6).
   Sylheti, in the Bengali script (its own Syloti Nagri needs a keyboard),
   on Bengali's keys: their ঃ and ৎ, the au length mark ৗ on ৌ, and its
   glottal stop ʼ on the full stop's long press (Bengali's keys, a row of
   letters above their home row, leave no room for a key at its end).
10. Thirteen keyboards of languages no text here predicts, each in a
   script of its own, are Keyman's (MIT; `scripts/build-keyman-layouts.py`,
   their files pinned by sha256): Chakma's easy_chakma, Rohingya's
   hanifi_rohingya (in the Hanifi script, right to left), Tai Dam's
   sil_tai_dam, Western Kayah's sil_kayah_kali (in Kayah Li), Lisu's
   sil_lisu_standard, Lepcha's sil_lepcha, Khmu's sil_kmhmu (in the Lao
   script), Saurashtra's saurashtra_inscript, Bamum's bamum, Eastern
   Cham's sil_cham_phonetic, Tai Nuea's tainua (in Tai Le), Tai Lue's
   ntl_onekey (in New Tai Lue) and Osage's osage_nation. Their keys keep
   Keyman's rows and places, a key that types nothing of the script a gap.
   What Keyman types by a rule with a context, which a key of one letter
   has none of, is on a long press: Tai Dam's second taps (ꪂ, ꪀ tapped
   twice, on ꪀ), Lepcha's aspirates and l clusters (ᰂ, ᰀ then h, and ᰁ,
   ᰀ then l, on ᰀ), and Cham's AltGr letters (ꨒ on ꨗ). Rohingya's kinna
   wa 𐴗, in its own name, which Keyman's phone layout has no key for, is
   on wa's (𐴖) long press. They type every letter of CLDR's exemplar
   characters where it has them (Chakma, Rohingya, Tai Dam, Western Kayah,
   Osage), and everywhere every letter Keyman's keyboard types: all of
   their Unicode blocks' letters, marks and digits but for Khmu, which
   writes those of Lao's letters it needs, Saurashtra's ꢟ and Rohingya's
   va 𐴜, which their Keyman keyboards do not type. Digits no letter key
   has (Rohingya's, Saurashtra's, Bamum's, Tai Lue's, Khmu's Lao ones)
   are on the long presses of the symbol view's. A language CLDR names in its own script
   is named so (Chakma, Rohingya, Tai Dam, Osage), the others in English.
   Their scripts' Noto faces draw every key. None had a model; Tai
   Nuea's has its Wikipedia's now (note 6), suggestions and no glides.
   Vai's and Nuosu's Keyman keyboards type their
   syllables from Latin letters, as a composer does: note 11.
   Six more are Keyman's at a later commit (2026-11-17), each pinned as
   it is: Gorani's horami (with a model now, its Hawrami web text's, note
   7) and Southeastern Pashayi's pashai (SIL's),
   Southern Hindko's rac_hindko and Chaldean Neo-Aramaic's
   east_syriac_qwerty, right to left, and Northern Thai's tai_lanna_thai,
   in Tai Tham, its letters in Thai's keys' places, the one from
   Keyman's experimental keyboards. What a key types under AltGr is on
   its long press (Pashayi's ك ڰ), digits there on the symbol view's
   (Tai Tham's Hora digits; its Tham digits are on its Shift view's top
   row, where Thai's keys have Thai's). Chaldean's Shift view is the
   layer Keyman's Shift key turns to, East Syriac's vowel points and
   marks; its West Syriac vowels, which Chaldean does not write, are
   left out. Hindko's ݨ, which its keyboard types on a layer of symbols,
   is on ن's long press, and Tai Tham's high ya ᩀ, which CLDR's exemplar
   characters list and its keyboard lacks, on low ya's (ᨿ). Each types
   every letter its Keyman keyboard types, and Northern Thai's every
   letter of CLDR's exemplar characters; Noto Sans Arabic, Syriac and Tai
   Tham draw them. Named in English, as CLDR names none of them. And
   Sylheti's in its own script, Syloti Nagri, is Sylheti Translation And
   Research's sylheti_nagri, for a tablet (`syl-sylo`; its other keyboard
   is in the Bengali script, note 9): its five rows of letters and signs,
   the hasanta's clusters on the long presses (ꠘ꠆; ꠇ's ꠬, as its rules
   type, its label ꠇ꠆ aside), Bengali's digits, which Sylheti writes, on
   the symbol view's, and its full stop key the danda, as Keyman's bottom
   row and Hindi's layout have it, with on its long press the ⁕ its
   writers end two of three sentences with, its poetry marks ꠨ ꠩ ꠪ ꠫, ॥
   and the Latin full stop. It types every letter and sign of the
   script's block, which Noto Sans Syloti Nagri draws (its alternate
   hasanta ꠬ only Noto's current one: older copies, Android's among them,
   lack it); it is named as its
   wiki writes Sylheti, ꠍꠤꠟꠐꠤ, and has that wiki's model (note 6).
   The International Phonetic Alphabet's is SIL's sil_ipa too, laid out
   on the US keys (`und-fonipa`, as BCP 47 tags a phonetic
   transcription): each letter's long press is Keyman's for that key, its
   IPA letters (e's ɘ ə ɛ ɜ œ…), then the modifier letters its super- and
   subscript layer has in that key's place (h's ʰ ʱ ᶣ…); Shift's small
   capitals (ᴇ ɶ, ʀ ʁ, ɢ ʛ) on the capitals'; its numeric layer the first
   symbol view (the digits with their super- and subscripts, ː ˑ ˈ ˌ ʼ ‿
   ꜝ ‖) and its diacritic layer the second (its combining marks, each on
   a dotted circle, ◌̃ ◌̥ ◌̪…, its tone letters ˧ ˩˥ ꜔ and arrows ↗);
   every character its five layers type is on it. Its letters of
   Unicode's Phonetic Extensions Supplement and Latin Extended-F (ᶒ ᵲ,
   𐞥), which DejaVu Sans lacks, draw with Noto Sans.
11. Vai's and Nuosu's Keyman keyboards (Athinkra's athinkra_vai and
   SIL's sil_yi, MIT) type each syllable as Latin letters, so their
   layouts type through the transliteration composer: their tables
   (`translit/vai.txt`, `translit/ii.txt`, `scripts/build-keyman-composers.py`,
   the keyboards' files pinned by sha256) are what each syllable is typed
   as, found by running the keyboards' rules, and a word is read as the
   fewest syllables its letters spell. Vai's (ka ꕪ, ka~ the nasal ꕫ, s h
   a ꕣ) reads Keyman's capitals as ɛ ɔ ŋ þ ð ʒ too, which its keys type:
   ɛ in q's place, ɔ at the home row's end and the nasal ~ in Shift's
   (Vai's ɛ is in 12% of its syllables, its ɔ in 10%, its nasals in 5%),
   the others on the long presses of n t d j and the lengthener ꘌ on x's;
   it has no Shift view, for a capital is a letter of its own (T is THA,
   t TA). It types every syllable of CLDR's exemplar characters, main and
   auxiliary: Keyman's hE and hE~, which type HEN and WE, a place past,
   read as HE and HEN, and the Ndole forms its keyboard lacks typed as
   its others are (-ma ꘪ). Every word of Vai's Incubator wiki, spelled
   so, reads back as written. Nuosu's is Yi pinyin (bit ꀖ, nuosuhxop
   ꆈꌠꉙ), all 1 165 syllables, and writes no space between words: Space
   takes the syllables typed without one. Where a tone letter could end
   a syllable or begin the next (bita), it begins the next, as Nuosu
   writes more often: of its Incubator wiki's words typed whole, 96.7%
   read back as written (75% the other way); typed a syllable and Space
   at a time, as Keyman's keyboard has them typed, all of them. Its ， and
   。 are keys, its other punctuation fullwidth, as Keyman's rules type it.
   Vai's has its Incubator wiki's model (note 6): the strip offers the
   readings that are its words, the likeliest after the word before
   first, and the words they begin, then the letters as typed; and with
   nothing composed, the words likeliest next. Nuosu's has its Incubator
   wiki's and FineWeb-2's model (notes 6 and 7), each syllable a word:
   with nothing composed the strip offers the syllables likeliest next,
   and from a syllable's first letter the syllables its letters begin, the
   likeliest after the one before first (n: ꆈ nuo, ꆀ nip…; nuo: ꆈ, then
   ꆇ nuox and ꆉ nuop, its other tones), then the letters as typed. Without them it saved
   20.3% of the taps (the next syllable alone), with them 38.5%.
12. A hundred and seventeen keyboards of languages no text here predicts, and Eastern
   Pwo Karen's, which its wiki's text does since data 0.76.0 (note 6), are
   another language's keys in their name, as they write with those letters:
   Amdo and Khams Tibetan on Tibetan's, in its spelling (Keyman's
   Tibetan keyboards name them among their languages); Sadri on Hindi's
   and Maharashtrian Konkani on Marathi's (Keyman's Devanagari keyboard
   names them); Northern Thai in the Thai script on Thai's, which type
   every letter CLDR's exemplar characters list for it; Kutchi on
   Gujarati's, which type every letter its Incubator wiki writes but
   ૅ and ૢ (21 times in 12 000 letters); Tày on Vietnamese's, with the
   two tones Vietnamese lacks first on each vowel's long press, the
   macron below and the double grave (a̱ ȁ, e̱ ȅ: SIL's tay_latn types
   both, its Incubator wiki writes the first, as in le̱); and Eastern Pwo
   Karen on Burmese's, its tone ႋ and its ၜ and ၠ, the commonest of the
   letters those keys lack (2.1%, 1.1% and 0.9% of its Incubator wiki's
   letters), in place of ဩ ၏ ၒ, which it does not write, and its ၮ ၯ ၰ ၞ
   ၟ ၚ ဴ on the long presses of ဏ ယ ဃ န မ င ု, as Keyman's
   eastern_pwo_karen types them (but ၮ, which its wiki writes and the
   keyboard lacks), Myanmar digits on the symbol view's: of its wiki's
   letters 97.5% a tap types and 2.5% a long press, where Burmese's keys
   type no 6.7% (Keyman's keyboard's own Pwo Karen digits, U+116DA, are
   left out: no Noto face draws them yet, and its wiki writes Burmese's).
   Twelve more take their letters from their wikis, too little text, or
   too much of it another language's, to learn from (note 6): Northern
   Min on the US keys with its toned vowels first on the long presses,
   as Southern Min's (its wiki's Kienning romanization: â̤ á a̿ ă ǎ…,
   its seventh tone the double overline; ṳ in its tones on u); Siberian
   Tatar on Bashkir's, which have the ҡ ғ ҙ ҫ its wiki writes, ё on е's
   long press; Isan on Thai's; Maguindanao, Capiznon and Buginese on the
   US keys as they are (Buginese's é first on e's long press, as there);
   Kedah Malay on Malay's; Qashqai in the Arabic script on South
   Azerbaijani's, ێ on ی's long press (its wiki's Latin text is Turkish,
   about it); Rejang on the US keys, ê first on e's long press; Ghomala'
   on them with ə ɛ ɔ ʉ ŋ ƴ and its tones on the long presses of e o u n
   y, its vowels' tones first (é ə ə́ è…); Laki on Sorani's, whose ڵ ێ ە
   ۆ its wiki's Laki writes; and Southern Balochi on Balochi's, ۏ ݔ ګ on
   the long presses of و ی گ (بلۏچی, انگرݔزی, ګون). Seventeen more take
   them from smaller test wikis still (500 to 6 700 words) or FineWeb-2's
   few pages: Ahirani on Marathi's keys, Harauti and Kannauji on Hindi's
   (three of South Asia's Devanagari languages, their test wikis in it);
   Chittagonian on Bengali's, ৱ and ৰ on the long presses of ব and র;
   Hani, Bangka Malay, Banggai, Youjiang Zhuang (its wiki in the 1982
   alphabet's 26 letters), Pokot and Ma'anyan (FineWeb-2's) on the US
   keys as they are; Nandi on them with ñ ā ē ĩ ō first on the long
   presses and an apostrophe key (para'ab: 3% of its words); North
   Moluccan Malay with the accents its wiki marks its vowels with (tù,
   kalò, adalá: a tenth of its words) first on the long presses;
   Cuyonon with ñ; Mongondow with ḷ ḻ (its Bible's) and an apostrophe
   key for its glottal stop (lopi': 7% of its web text's words);
   Manggarai with é first on e's long press (in 15% of its words) and
   ’ for its glottal stop (Pu’ung); Khorasani Turkic on South
   Azerbaijani's; and Northern Khmer in the Thai script (Khmer Surin) on
   Thai's, which type its ฺ and ํ (its New Testament, FineWeb-2's).
   Buginese's é and Kedah Malay's å, which only their long presses type,
   now turn the gesture on, which data 0.57.0 left off. The first eight
   are named in English, the twelve as their wikis name them, Kedah Malay
   but in English, and the seventeen as theirs write their names where
   they do (अहिराणी, हाड़ौती, कन्नौजी, চাটগাঁইয়া, Haqniqdoq), in English
   otherwise. Twenty-one more have no text here to take letters from, but
   a source that names their script, whose national keys type all of it:
   Dari, a pack of its own, on Persian's keys, with the Pashto letters
   CLDR's Dari (fa_AF) lists among its auxiliary ones (ټ ځ څ ډ ړ ږ ښ ګ ڼ,
   and ى) on the long presses of the letters they are made from, named
   دری, as CLDR's Persian names it; Valencian on Catalan's keys, its
   letters Catalan's (CLDR's ca_ES_VALENCIA), named valencià; the
   languages CLDR's language data (supplementalData's languageData) says
   are written in Devanagari, Bhili, Kumaoni, Kurukh, Mewari, Nimadi,
   Shekhawati and Wagdi, and Mundari (Bengali or Devanagari), on Hindi's
   keys; Lambadi, in the Telugu script by it, on Telugu's; Southern Thai
   on Thai's; Dakhini, in the Arabic script, on Urdu's; Bishnupriya
   Manipuri, in the Bengali script, on Bengali's, with the ৱ its wiki
   writes (0.4% of its words) on ব's long press; and those Wikidata's
   "writing system" (P282) gives Devanagari, Bundeli, Dhundhari, Korku,
   Malvi, Merwari, Mahasu Pahari and Halbi, on Hindi's keys (Mandeali,
   Takri alone there, came later, below; Wanetsi, "Arabic alphabet", has
   not). These
   are named in English but Dari, Valencian and Bishnupriya Manipuri
   (as its wiki writes it). Two take their letters from the Bibles
   eBible.org has of them, the only ones of the languages left it has:
   Southern Oromo (its Guji) on Oromo's keys, with a key for the
   apostrophe that is its glottal stop (in 8% of its words), and Yaka
   on the US keys with its high tones á é í ó ú first on the long
   presses (á in 4.4% of its words); both have models from those Bibles
   now (note 18). Four take theirs from the Universal
   Declaration of Human Rights in their languages (the UDHR in Unicode
   project's texts, 1 255 to 2 724 words each): Maay on Somali's keys and
   Nyamwezi on the US keys, each with an apostrophe key (Bani’aadynky,
   ng’wana: in 2.2% and 2.5% of their words), and Chokwe and Northern
   Qiandong Miao, whose tones its letters spell, on the US keys as they
   are. The Declaration's Plateau Malagasy and Yongbei Zhuang are
   Malagasy and Standard Zhuang, whose packs type and predict them
   (Malagasy's model learns from FineWeb-2's plt_Latn; the Declaration
   tags its Yongbei Zhuang za): they have no pack of their own. Five take
   them from Wiktionary's entries (en.wiktionary.org's lemmas, their
   spelling only): Garo (1 077 entries) on the US keys with a key for
   the raised dot of A·chik (in a quarter of them), and in the Bengali
   script, which CLDR's language data names for it, on Bengali's keys,
   both in Garo's pack; Mandar (324 entries) and Bouyei (166; its tones
   are letters) on the US keys as they are, as their test wikis' few
   pages write; Muong (247) on Vietnamese's keys, whose letters it
   writes (the turned comma above and circumflex below three entries
   write, one source's spelling, are left out); and Kui, in the Odia
   script, on Odia's keys with a key at the home row's end for the
   overline Unicode encodes for it (U+0B55 ORIYA SIGN OVERLINE, "Kuvi,
   Kui": in 4 of its 14 entries), which Noto Sans Oriya draws. And three
   take them from their Incubator test wikis, too little text, or too
   much of it Indonesian, to learn from (note 6): Central Malay (30 000
   words in two dialects' spellings, Baso and Basau), Komering (its test
   Wiktionary's and Wikiquote's 4 000) and Kutai Malay (1 700), on the
   US keys as they are. These are named as their texts write their names
   (A·chik and গারো, Kinyamwezi, Haausqyaix, Thiểng Mường, Baso Melayu
   Tenga, Basa Kumoring), in English otherwise (Maay, Chokwe, Hmu,
   Mandar, Kui, Kutai Malay). And eight have a script LinguaMeta (Google
   Research's language metadata, CC BY 4.0) names for them, whose
   national keys type all of it: Godwari, Mandeali and Surjapuri, in
   Devanagari, on Hindi's keys; Varhadi on Marathi's (its name, वऱ्हाडी,
   writes Marathi's eyelash ra ऱ); Vasavi, in Devanagari and in the
   Gujarati script, on Hindi's keys and on Gujarati's, both in Vasavi's
   pack; Kachi Koli, in the Gujarati script, on Gujarati's (LinguaMeta
   names the Arabic script for it too, whose letters no source here
   gives); and Urum, in Cyrillic, which settles the script of
   Wiktionary's 147 entries (ӧ ӱ ґ, and ʼ where Russian writes ь), on
   Russian's keys with ӧ ӱ ґ on the long presses of о у г and a key at
   the home row's end for its ʼ (in one entry in thirteen). These are
   named as LinguaMeta writes their names (गोड़वाड़ी, मंडयाली, सुरजापुरी,
   वऱ्हाडी, वासवी and વસાવી, કચ્છી કોળી, Урум). And six take their
   letters from FineWeb-2's few pages of them, five from their Bibles'
   (bible.com's and Bible.is', 2 700 to 18 700 words each) and Eastern
   Oromo from its web text (50 000 words); their text is not used.
   Northwest Gbaya (Gboard's Ngbaka Gbaya), whose letters long presses
   would type in 58% of its words, is typed on the US keys with ɛ (in a
   quarter of its words) and an apostrophe at the home row's end, ɔ in
   place of x and the cedilla of its nasal vowels (a̧ ɔ̧: an eighth of its
   words) in place of q, its tones (á à, ɛ́ ɛ̀ on ɛ's) and ŋ ɓ ɗ first on
   the long presses: a long press in 26% of its words now. Sar marks its
   tones with the macron and the acute (in 40% and 35% of its words), on
   its consonants too (n̄, ḿ): both are keys of their own at the home
   row's end, typed after the letter they mark, the tilde below on the
   acute's long press, with ə and ɔ in place of q and x and ɓ on b's long
   press (77% of its words needed one, now 9%). Masana (Gboard's Massa)
   has ɗ ŋ ɓ ɦ first on the long presses of d n b h and an apostrophe key
   (di'e, kulo'o: 3% of its words); Marba â á ä à î ï ì ê ô û first on
   the vowels' and a key for the ’ of its implosive d’ (in 11% of its
   words); Nyaturu ë (in 28% of its words) and an apostrophe (ng', 1.2%)
   at the home row's end, ü first on u's long press; and Eastern Oromo,
   Oromo in its eastern spelling (keeysa, tahe), Oromo's keys with a ’
   key (je’e: 1.8% of its words). They are named in English, Nyaturu as
   its Gospel of John names it, Kinyaturu. None had a model:
   neither suggestions nor glides. Southern Luri's test wiki (1 070
   words, 860 of them in the Arabic script) writes two spellings:
   Persian's (184 words), and one that writes every vowel (673): ة for a
   (in 43% of all its words), ە for e and ݖ (in a fifth each), with ۿ for
   h (17%), ي for y and ی for i, ۊ for u and ڍ ڲ for its soft d and g. It
   is typed on Persian's keys, its zero-width non-joiner among them (in a
   fifth of its words), with ة ە ݖ ۿ in place of ث ذ ظ ض, which it hardly
   writes (in 0.1% of its words at most) and which their long presses
   keep, ۊ ۋ ۏ first on و's long press, ڤ on ف's, ڍ ڌ on د's, ڲ on
   گ's, ݩ on ن's, ھ on ه's and the mark ٛ on the full stop's: a long
   press in a quarter of its words (in half on Persian's keys as they
   are). Noto Sans Arabic draws every key (DejaVu Sans lacks ݖ ۿ ۊ ٛ ݩ
   ۏ). It has no model: its wiki is too small, as is Eastern Balochi's,
   half of it Latin transliteration, whose words in the Arabic script give
   Eastern Balochi's keyboard its letters (below). And ten of Gboard's
   languages that had no keyboard here (since data 0.77.0) take their
   letters from Wiktionary's entries (en.wiktionary.org's lemmas) and their
   Incubator test wikis' few pages, too little text to learn from: Nauruan
   (127 entries) on the US keys with â and ñ on the long presses of a and
   n (one entry each); Central Melanau (76) and Sabah Malay (19) on
   Malay's keys as they are, as their entries write; Guyanese Creole (36,
   and its test wiki's 330 words, which name it Creolese) on the US keys
   as they are; Parauk, whose test wiki writes a colon inside 18 of its
   115 words (cia:, cao:, Ye:sux), a key at the home row's end, and whose
   entries mark tones on their vowels (20 of 34: í ó á é ī ā ē), á ā é ē
   í ī ó ō ú ū first on its vowels' long presses; Gayo, whose test wiki
   (492 words) writes é è ö, as one of its entries (unggör), on
   Indonesian's keys with é è first on e's long press and ö on o's; Izon,
   whose name its wiki writes Ịzọn and whose entries mark its tones (bení,
   kúkúndùkú), on Igbo's keys (ị ọ ụ first on their long presses) with ẹ
   first on e's and the acute and grave on every vowel's; Balkan Gagauz
   Turkish on Turkish's keys, as its test wiki's main page writes
   (Rumelice); Lari in the Arabic script on Persian's keys, as its test
   wiki's sentence writes it (اچمی, its vowels' marks on the full stop's
   long press); and Batak Alas-Kluet on Indonesian's keys as they are,
   whose letters its test wiki's articles write. They are named as their
   texts write their names (Dorerin Naoero, Creolese, Lox Vax, Gayo, Ịzọn,
   Rumelice, اچمی), in English otherwise. And fifteen more of Gboard's
   (since data 0.78.0), on sixteen keyboards, take their letters from
   what their texts write, too little to learn from: Nung, in SIL's
   spelling of Nùng Phạn Slinh (Wiktionary's 55 entries: chéht, slíhp), on
   Vietnamese's keys as they are; Kohistani Shina, whose 316 entries in
   the Arabic script and Wikipedia's article on its alphabet write ڦ and
   ڱ, its own letters, and the inverted damma ٗ of its rising tone (in 37
   entries), on Shina's keys with ڦ ڱ first on the long presses of ف and
   گ and ٗ on the Shift view's damma's; Southwestern Fars (Kuhmareyi, as
   Gboard names its keyboard), whose 115 entries write Persian's letters
   and whose Kuhmareyi forms in Persian Wikipedia's article write ڌ ۊ ٱ ٛ
   (in 112 of its 406 words, ڌ and ۊ 27 times each) and ۆ ێ ؽ ݩ ۉ, on
   Persian's keys with those first on the long presses of د و ا ی ن and
   ٛ on the full stop's, as Southern Luri's are; Wanetsi, whose poem and
   sentences in Wikipedia's article (547 words) are in the Pashto
   alphabet with ۀ (نۀ, ماندۀ) and ں for its nasal vowels (بويں), on
   Pashto's keys with ۀ first on ه's long press and ں on ن's; Mehri on
   Arabic's keys as they are, the unmodified Arabic alphabet being the
   commonest way it is written (Wikipedia's article); Takestani on
   Persian's keys as they are, as Persian Wikipedia's article writes its
   words (بُن, تیج, خوآکه); Eastern Balochi on Balochi's keys as they
   are, which type all 243 words of its test wiki in the Arabic script
   (ث ذ its dental fricatives: بیث, سیاذی); Southwestern Pashayi on
   Southeastern Pashayi's keys, the one alphabet drawn up for the Pashai
   languages since 2003 (Persian Wikipedia's article on them), no text of
   its own being found; Western Cham, as Gboard has it, in the Cham
   script, which Wikipedia writes its name in (ꨌꩌ ꨚꨭꩉ, Cam pur), on
   Eastern Cham's keys, and in Latin letters on the US keys with â ă
   first on a's long press, as its 30 Latin entries write (anâk, hlăk),
   both in Western Cham's pack; Xong on the US keys as they are, its 1956
   alphabet's letters, its tones written with letters (b x d l t s), as
   Wikipedia's article and its entries write; Cameroon Pidgin (22 entries,
   and the spellings Wikipedia's article describes: ey ow, ch j ny ng),
   Bahamian Creole (das ya book?) and Trinidadian Creole (allyuh) on the
   US keys as they are; Upper Saxon on German's keys, as German
   Wikipedia's article writes its words (bäse, Biehne); and Bangala,
   Lingala's sister language, on Lingala's keys with á (makilá, sánza)
   and ɓ (Wikipedia's ɓisu) first on the long presses of a and b. They are
   named as Wikipedia's articles (ݜݨیاٗ کستِین٘و, کوهمَره‌ای, وڼېڅي,
   مهريّت, ꨌꩌ ꨚꨭꩉ, Cam pur) and LinguaMeta (Nùng, تاتی, بلوچی, Dut
   Xongb, Kamtok, Bahamian, Trinidadian Creole, Obersächsisch, Ngala)
   write their names, Southwestern Pashayi in English, as Southeastern
   Pashayi's keys are. The last four of Gboard's languages that had none
   have keyboards since data 0.79.0: Anaang on the US keys with ñ ö ü
   first on the long presses of n, o and u, the letters its alphabet has
   besides the 26 (with the digraphs ch gh gw kp kw ny ñw, its long vowels
   written double), as Owulette's Annang–English dictionary (2019, which
   French Wikipedia's article cites), Omniglot's chart and the thirty
   letters of Michael and Ijioma's pronouncing dictionary (2019) list
   them; Ha on the US keys with an apostrophe key, for the vowel it drops
   before the next word (nʼumunsi, chʼamanza), as PanLex's 1,684 entries
   (CC0) write it, with bh ch sh, and l beside r; and Northern and
   Southern Betsimisaraka Malagasy on Malagasy's keys, its apostrophe key
   among them (amin’ny: one word in seventeen of each), as JW.org's
   brochure Good News From God! in each writes them (6,300 words each):
   Northern's velar nasal n̈ (zain̈y, ten̈a: nearly one word in five) first
   on n's long press, before ñ, and ỳ à ê ì ï (Vakỳ, lalàn̈a, marìny);
   Southern's à é ỳ ì (koà, maé, tatỳ), its velar nasal gn (fôgn), as
   Wikipedia's articles' tables write both (agnaragnanazy, etoagna). They
   are named as they name themselves, Anaañ, and Igiha (as Kirundi is
   Ikirundi), and as JW.org and Wikipedia name the Betsimisarakas,
   Betsimisaraka Avaratra and Atsimo. Nine of this note's keyboards now
   have models from SMOL's translations (note 14): Ahirani's, Bundeli's,
   Kumaoni's, Kurukh's, Nimadi's, Mundari's, Garo's in Latin letters,
   Mandeali's and Surjapuri's; Bishnupriya Manipuri's has one from its
   wiki (note 6); and Halbi's and Korku's, since data 0.69.0, from
   StoryWeaver's stories (note 17). Kui's, on Odia's keys, types the ୟ
   and ୱ their long presses now give (1.2% and 0.7% of the letters of
   its StoryWeaver books' Kui).

13. These two models learn from the African Storybook's stories
   (africanstorybook.org, an initiative of Saide: picture books for
   children written and translated by teachers and writers across
   Africa), as the Global African Storybook Project's asp-raw-db keeps
   them (`scripts/fetch-storybooks.sh`, its archive at one commit pinned
   by sha256): Masaaba's and Turkana's, which no wiki or web text here
   has. Only the stories under CC BY 3.0 or 4.0 are read, 90 of
   Masaaba's 102 and 151 of Turkana's 175, each one's author and
   illustrator named in its file there (NOTICES.md). A page is left out
   where English's or Swahili's own words are two of its words and 3%
   or more (18 and 16 pages: one Masaaba story holds a Swahili story's
   pages and an English one's), an apostrophe inside a word written ’
   is read as the ' its keyboard types, and every 10th story is held
   out (`scripts/storybook-text.py`): 2 079 and 3 054 sentences, 20 000
   and 23 000 words, all of whose 6 700 and 5 700 kinds of word the
   lexicons keep. Keystroke savings on the held-out stories (1 400 and
   2 400 words), with taps astray and the words unknown: Turkana 33.0%,
   19.5%, 23.7%; Masaaba 27.2%, 10.2%, 36.0%, so little text leaving a
   third of its words unknown. Of Turkana's words typed right,
   autocorrect changed 1.2%, most of them by putting back the apostrophe
   a held-out story left out (ngesi, ngina for ng'esi, ng'ina). Turkana
   writes an apostrophe in a sixth of its words (ng'esi, ng'olo) and
   Masaaba in one word in a hundred and twenty (n'aaba, s'unyala): both
   are typed on the US keys with a key for it at the home row's end,
   Masaaba with ŋ on n's long press, which a few of its stories write
   (liŋaasa). And two keyboards of languages too little of whose text is
   CC BY to learn from take their letters from their stories there:
   Bukusu's (11 stories, 1 700 words) and Malawi Lomwe's (19, 3 200),
   on the US keys as they are, with an apostrophe key at the home row's
   end for the ng' they write (Bukusu's chiisang'i, in one word in
   forty; Malawi Lomwe's eng'ombe, fewer). These four are named as the
   stories write their names: Lumasaaba, Ng’aturkana, Lubukusu,
   Ellomwe.

14. These fourteen models learn from SMOL's translations (Google
   Research, at Hugging Face as google/smol, CC BY 4.0: English text
   translated by professional translators into languages the web has
   little of; `scripts/fetch-smol.sh`, its files at one revision pinned
   by sha256): SmolDoc's documents, translated a sentence at a time, in
   twelve of India's languages in Devanagari (Ahirani, Bagheli, Bagri,
   Bundeli, Kumaoni, Kurukh, Mandeali, Nimadi, Surgujia, Surjapuri,
   Mundari and Kangri: 299 to 457 documents each) and in Garo in Latin
   letters (457), and SmolDoc's and SmolSent's in Zarma (130 documents
   and 863 sentences). Their keyboards had no model (notes 9 and 12) and
   keep their keys, but Kurukh's, which gains ' first on its full stop's
   long press for the apostrophe inside one of its words in twenty-six
   (र'ईर), Hindi's home row having no spacers to give it a key of its
   own, and Nimadi's, its avagraha ऽ on the visarga's long press, as
   Sanskrit's is, for the one word in twenty that ends in it (करनऽ). The
   text is read as the keyboards type it (`scripts/smol-text.py`): an
   apostrophe written ’ or ‘ between letters as ', Garo's raised dot
   written ∙ or • as ·, which its words keep as Catalan's keep theirs
   (an·chingni: one word in sixteen writes it), and in Devanagari no
   zero-width joiner, non-joiner or space, which the translations
   scatter between letters and at words' ends and Hindi's keys do not
   type; a sentence of one word is left out and every 10th document
   held out: 2 218 (Zarma) to 5 429 sentences, 35 000 words (Zarma) and
   55 000 to 93 000, all of whose 4 400 to 16 200 kinds of word the
   lexicons keep. Keystroke savings on the held-out documents (the first
   3 000 words of each one's 3 900 to 10 100), with taps astray and the
   words unknown: Surgujia 45.8%, 41.7%, 5.9%; Kangri 45.3%, 39.9%,
   6.7%; Mandeali 42.5%, 38.2%, 6.7%; Zarma 40.6%, 34.3%, 9.8%; Bundeli
   39.0%, 33.2%, 8.2%; Surjapuri 37.6%, 30.0%, 11.5%; Kurukh 36.5%,
   28.4%, 13.2%; Bagheli 36.3%, 29.1%, 10.5%; Mundari 36.1%, 29.1%,
   10.5%; Garo 34.6%, 25.1%, 15.2%; Ahirani 33.7%, 25.5%, 14.6%; Bagri
   32.3%, 25.2%, 10.0%; Nimadi 32.2%, 25.1%, 9.5%; Kumaoni 30.9%,
   23.5%, 12.9%. Of the words typed right autocorrect changed 0.11%
   (Ahirani's) to 0.94% (Kumaoni's, most of them रम, rum, which only a
   held-out document writes, as हम or कम); Kurukh's most by putting
   back the apostrophe a held-out document left out (रई for र'ई).
   Bundeli's and Bagheli's text is the nearest Hindi's, as the languages
   are: Hindi's twenty-two commonest words are a fifth and a sixth of
   their words, more than a quarter of SMOL's Hindi's; none is its
   Hindi copied. Not used: SMOL's Dhundhari, Mewari and Wagdi, which are
   one text (their sentences 88% to 93% alike, where two languages'
   translations of the same documents are 51% to 65%), Mewari's and
   Wagdi's with letters swapped through it (हर जगह, everywhere, written
   सर जगस); Lepcha's, which its keyboard types 6% of the letters of on
   no key, and whose autocorrect changed 4% of its words typed right;
   Afar's and Chiga's, too little (28 000 and 25 000 words: 23% of the
   keystrokes saved, 7% and none with taps astray); Limbu's and
   Sherpa's, in the Limbu and Tibetan scripts, whose keyboards here are
   in Devanagari; and Chakma's in Latin letters, whose keyboard is in
   its own script. Pulaar, which Gboard names apart from Fula, has a
   keyboard of its own on Fula's keys, with no model and ñ first on n's
   long press: SMOL's 863 Pulaar sentences (11 800 words, too few to
   learn from; Fula's model saves 15.6% of their keystrokes, and its
   autocorrect changed 2.4% of their words typed right) write a b ɓ c d
   ɗ e f g h i j k l m n ñ ŋ o p r s t u w y ƴ, the letters CLDR's Fula,
   whose name for itself is Pulaar, lists, and ñ the commonest of n's
   (100 to ŋ's 18). Garo in the Bengali script (`grt-Beng`) learns from
   SMOL's Garo too, spelled in that script (`make respelled-text`;
   `crates/osk-data-tools/src/bin/respell.rs`): each word as its
   transliteration table (`translit/grt-Beng.txt`) reads it, the cheapest
   reading. The table follows the spelling of the one Bengali-script Garo
   text read when it was written, bible.is's Bible, which no model here
   learns from (note 19): k t p at a word's start খ থ ফ for Garo's
   aspirates, a consonant between vowels often doubled (ine: ইন্নে), n m l
   before the raka written after it, doubled (man·a: মা·ন্না), a first o হ
   (ong·a: হং·আ). The table offers each of these, but the respelled text
   has each word's cheapest reading, so its lexicon spells man·a মান·আ and
   ong·a ওং·আ, which are offered first. 14 185 kinds of word; keystrokes
   saved on SMOL's held-out documents so spelled 38.3% (28.0% with taps
   astray, 14.8% unknown), which no text written in the Bengali script
   measures. Its keyboard is Bengali's keys with the raka · on ঋ's key, and
   its transliteration keyboard (`grt-beng-abc`) reads Garo's Latin
   alphabet, the raka typed ' or ·.

15. Sebat Bet Gurage's keyboard is Amharic's keys, the Ge'ez composer
   typing its syllables from Latin letters as Amharic's does, with two
   keys of its own on both its letters' views: ʷ in place of the v it
   does not write and ʸ in place of the hyphen (on ?123 still). After m
   b f p, ʷ is their labialised syllable, Ethiopic Supplement's
   Sebatbeit row (mʷe ᎀ, mʷi ᎁ, mʷE ᎂ, mʷ ᎃ; mʷa ሟ, as mwa is); after q k
   x g, ʸ is their palatalised one, Ethiopic Extended's seven-order row
   (kʸa ⷋ); anywhere else they are w and y. Amharic's keyboard has
   neither key: its mwe stays ምወ and its kya ክያ. Its letters are those
   SIL's Locale Data Repository lists for it (SLDR, MIT: `sldr/s/sgw.xml`),
   all of which it types, ፀ's row from the candidate bar (ts or S, then
   ፀ, as on Amharic's), and it is named as SLDR names it, ሰባት ቤት ጉራጌ. From
   data 0.68.0 to 0.83.0 its composer offered its model's words; it has
   no model since (note 19). Every
   Ge'ez keyboard now types the
   6th order as I, as SERA writes it: alone, the carrier's እ (InE እኔ),
   which none of them could type before, and after a consonant its bare
   form before a vowel of its own (sIIl ስእል).

16. Amharic's and Tigrinya's keyboards type through the Ge'ez composer
   (Latin letters composing fidel, as SERA writes them), which since
   data 0.67.0 offers their models' words on the
   strip, as the transliteration keyboards offer their readings: the
   words the syllables typed begin, likeliest first, in whichever of
   their homophones' rows the lexicon spells them (`sa` offers words in
   ሳ and in ሣ alike), with the pending consonant at any of its orders
   (`sel` offers ሰላም), ranked after the words before by the n-grams. A
   pick types the word and a space. Where the last syllable has
   homophones, the strip's last slot cycles it through them (ሃ → ሓ → ኃ),
   two words beside it; Space types the word as composed. Since data
   0.68.0 (ki-osk's composers), once a word is done with a space, the
   strip offers the words likeliest next before a key, as a keyboard
   without a composer does, and after a sentence's end and its space the
   words that start sentences; a pick types one and a space, and offers
   again; Backspace with nothing composed takes the offers away, and
   while composing takes the last fidel shown. The transliteration
   keyboards with a model offer the next word so too. Keystroke savings
   in taps, each held-out word's keys typed as SERA writes them (a
   capital two taps, Shift and its key, and a tap on the last slot for
   each syllable in a homophone's row) until the strip offers it, which
   a tap takes, before its first key too: Amharic 36.5%, Tigrinya 35.3%,
   their next word on the strip before its first key 11.0% and 11.3% of
   the time (34.2% and 32.7% without; `eval-predict`, 3 000 words of
   FineWeb-2's held-out text each; 7.5% and 9.0% of the words unknown to
   their 200 000-word lexicons, and 0.3% and 0.9%, Latin or with digits,
   left out). Ge'ez's, Gedeo's, Hadiyya's, Kambaata's, Silt'e's and,
   since data 0.84.0, Sebat Bet Gurage's keyboards (note 19), Amharic's
   keys, name no model and offer none.

17. These three models learn from StoryWeaver's stories
   (storyweaver.org.in, Pratham Books: picture books for children,
   written, translated and illustrated by its community and by
   publishers, each under CC BY 4.0), which no wiki or web text here has
   of them: Halbi's (177 stories, most of them translated by
   Shiksharth), Korku's (105, most of them Unnati ISEC's) and Nahuatl's
   (419, filed there under Nahuatl and Nawatl, written by its speakers
   in today's spellings, Huasteca's and others', uan, wan and iwan
   alike: not the classical spelling with the macrons its keyboard has
   on long presses, nor its wiki's or colonial documents', note 7).
   StoryWeaver's reader shows a story's pages to a signed-in account,
   and they are read through it (`scripts/fetch-storyweaver.sh`, the
   account's sign-in from the environment; without one, and the stories
   not read before, the models are kept as they are), the stories named
   in `scripts/storyweaver-stories.tsv`, each pinned by the sha256 of
   its text; every one's attribution page says CC BY 4.0, and one of
   Nahuatl's 420 the reader would not serve. A page is left out where
   another language's function words are two of its words and 3% or
   more (English's, and for Nahuatl Spanish's), or where fewer than nine
   in ten of its letters are its script's (Korku's sound effects typed
   with Latin s's, वँsssssss: 19 pages), and so is a story most of whose
   pages are (two of Nahuatl's, in Spanish); a line that ends no
   sentence goes on in the next, where the reader broke a picture
   book's line; a danda typed | is read as ।; a sentence set in
   capitals (a fifth of Nahuatl's stories) is read in lower case, its
   first letter a capital; and every 10th story is held out
   (`scripts/storyweaver-text.py`): 4 037, 4 181 and 6 733 sentences,
   33 000, 30 000 and 40 000 words, all of whose 6 700, 6 300 and
   12 300 kinds of word the lexicons keep. Keystroke savings on the
   held-out stories (3 000, 1 500 and 3 000 words), with taps astray and
   the words unknown: Korku 33.1%, 22.9%, 18.5%; Halbi 29.0%, 20.6%,
   17.6%; Nahuatl 28.7%, 8.9%, 28.2%, its varieties' words each spelled
   their own way (autocorrect changed 0.20% of its words typed right).
   Not used, too little text or too little saved: Lusoga's 242 stories
   (33 000 words, a third of the held-out ones unknown: 21.7%, 2.5% with
   taps astray), Wagdi's 52 (filed as Vagadi; 21 000 words: 18.4%),
   Haryanvi's 78 (11 000: 18.1%), Kui's in 87 Odia-Kui books, Kui in
   bold beneath the Odia (11 000: 26.7%, 13.0%, but 1.0% of its right
   words changed), Sadri's 54 (10 000: 20.2% on 300 held-out words) and
   Rukiga's 29 (1 100 words).
18. These three models learn from eBible.org's Bibles, each under its
   own copyright and CC BY-SA 4.0, as its about page says (share-alike;
   each pack names its Bible and holders, and NOTICES.md what was done
   to it), which no wiki or web text here has of them: Southern Oromo's,
   Kitaaba Woyyuu, the Bible in Borana-Arsi-Guji Oromo (The Word for the
   World International, 2025), Kituba of the Congo Republic's, Mambu ya
   Nzambi (Wycliffe Bible Translators, 2025), and Yaka's New Testament
   (The Seed Company and the Forum Communautaire des Traducteurs de la
   Bible et Alphabétisation, 2022); of the keyboards here with no model
   only these three had a Bible there under a licence that allows a
   model (before data 0.84.0, note 19) (Chavacano's, Limbu's and Eastern Tamang's forbid derived
   works; Mahasu Pahari's, Haryanvi's and Rohingya's are in Latin letters,
   which their keyboards are not). A translation's verses
   (`<id>_vpl.txt`) are pinned by sha256 (`scripts/fetch-ebible.sh`; one
   changed since, which the archive's address names no version of, is
   not read, and its model kept), each chapter read as running text, cut
   into sentences, NFC (Yaka's Bible writes its tones' accents as
   combining marks, its keys type them precomposed), and every 10th
   chapter held out (`scripts/ebible-text.py`): 33 518, 49 524 and 8 105
   sentences, 542 000, 803 000 and 127 000 words, all of whose 36 000,
   6 600 and 12 000 kinds of word the lexicons keep. Keystroke savings on
   3 000 held-out words, with taps astray and the words unknown: Kituba
   60.9%, 59.0%, 0.7%; Yaka 57.2%, 54.4%, 3.7%; Southern Oromo 50.0%,
   45.4%, 4.4%; autocorrect changes at most 0.22% of the words typed
   right (Kituba's). With whole books held out instead (every 10th) they
   save 60.9%, 50.9% (on 3 000 words of its two short books) and 51.7%.
   Measured on their own Bibles, these overstate what a typist saves on
   everyday text, whose words a Bible lacks: Southern Oromo's model saves
   12.6% of the keystrokes of Oromo's held-out web text (note 7), 58% of
   its words unknown, and Kituba's 31.6% of the Kituba of the
   Democratic Republic of the Congo's (note 8), 36% unknown, varieties
   of their own that spell otherwise. Their keyboards are as they were
   (notes 9 and 12), with suggestions and glides now.

   Gamo's model learns from two of its Bibles there beside its FineWeb-2
   text (note 7) since data 0.85.0, bible.is's pages having been more
   than half of that text (note 19): Geeshsha Mexaafa, its Bible in Latin
   letters (The Word for the World International, 2024; CC BY-SA 4.0),
   and Gamo Geesha Maxxafa, its New Testament (the same, 2017; CC BY
   4.0), read in turn as the three's are, 42 656 sentences and 584 000
   words, beside the web's 48 000, which count five times (`WEB_WEIGHT`,
   as beside a wiki's). Its lexicon keeps all its 64 000 kinds of word. On
   its held-out web text it saves 44.3% of the keystrokes (40.2% without
   them, 42.5% with the Bible and not the New Testament), 38.9% with taps
   astray (31.6%), 5.2% of the words unknown (14.0%); on its Bibles'
   held-out chapters 44.2%. Of the languages bible.is's pages were a tenth
   or more of, eBible.org has a Bible it marks redistributable of ten, and
   only Gamo's and Gourmanché's allow a model: Koronadal Blaan's,
   Supyire's, Akoose's, Konkomba's, Wolaytta's, South Bolivian Quechua's,
   Tok Pisin's and Southwestern Dinka's forbid derived works and
   commercial use (CC BY-NC-ND), and Dan's is not redistributable.
   Gourmanché's two New Testaments (CC BY 4.0, and Togo's Gourmanché's,
   CC BY-SA 4.0) are left out: FineWeb-2's text of it is half the first
   already (four fifths of its held-out text), and the second saved 0.1%
   more of the held-out words that are neither.

19. No page of bible.is (Faith Comes By Hearing's Bible site, whose terms
   allow personal, non-commercial use only) is read since data 0.84.0, in
   any language: `scripts/fineweb-text.py` leaves out every document whose
   address is on bible.is or a host under it (live.bible.is), FineWeb-2's
   and GlotCC's alike (MADLAD-400 names no address, and none of the
   documents read from it is bible.is's). It was in the text 136 of the
   models here learned from, most of it for dozens of small languages, and
   in their held-out text, so those models and the figures in the notes
   above were made again without it. Nineteen were left with too little to
   learn from (under 20 000 words, under a thousand kinds of word, or no
   held-out text) and have their keyboards and no model: Buhid, Dan,
   Farefare, Gogo, Gulay, Haya, Itawit, Kafa, Koronadal Blaan, Lahu,
   Lampung Api, Moba, Nande, Northern Grebo, Ntcham, Nyoro, Sasak, Sebat
   Bet Gurage and Xaasongaxango (Dan's and Lahu's text nine tenths
   bible.is's). Others keep a model learned from much less: bible.is's
   pages were seven eighths of Supyire's, Gorontalo's, Sukuma's and Hehe's,
   three quarters of Ngaju's and Akoose's, more than half of Gamo's,
   Gourmanché's, Konkomba's, Mende's, Lugbara's and Chopi's. Their savings
   fell most: Hehe 60.8% -> 29.0%, Gorontalo 66.2% -> 38.0%, Sukuma 61.2%
   -> 34.9%, Akoose 66.8% -> 46.3%, Gamo 60.3% -> 40.2% (44.3% with
   eBible.org's Bibles since data 0.85.0, note 18), Ngaju 64.9% ->
   52.8%, Supyire 62.0% -> 50.0%, Acehnese 53.8% -> 42.9%, South Bolivian
   Quechua 38.6% -> 28.9%, Tok Pisin 63.1% -> 55.2%, Cusco Quechua 43.1% ->
   35.5%, Northwestern Ojibwe 61.3% -> 55.2%; some rose, their held-out
   text changed with them (Lango 33.8% -> 42.0%, Mandinka 43.3% -> 48.6%,
   Maranao 42.1% -> 47.2%, Alur 47.9% -> 51.1%, Mizo 39.0% -> 42.1%). Much
   of what is left of the small languages' text is other Bible sites'
   (Jehovah's Witnesses', eBible.org's, YouVersion's), which this leaves as
   it was. Garo's model in the Bengali script learns from SMOL's Garo
   spelled by its table alone (note 14), and Shan's and Mon's syllable
   lists (`wordbreak/`) were made again from their text without bible.is's
   pages.

## Prediction model tiers

- **Full n-gram model** — `predict/<lang>.fst` +
  `<lang>.ngram.bin`, **460 languages** (Hinglish among them). The original nine (en,
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
  new cdo hif egl cu zea btm gpe igl rki vel bew nup glk bpy tdd syl-Sylo** (Newari with GlotCC's news too; Arakanese cut into syllables), **hy fa** beside Tatoeba's, and
  the Incubator's test wikis: **wsg khw cnr bto jax mfp xmm mui mak pmf mfa ibb vmf sdh bgn rkt abr vai ii kjp** (note 6; Vai's words and Nuosu's syllables offered through their transliteration tables, note 11; Nuosu's beside FineWeb-2's text, note 7; Eastern Pwo Karen's cut into syllables) ·
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
  gcf bi ia tpi gil tvl ho ch jam**, six more that had a keyboard and no model, **sr-Latn gn ff ber bo dz**, of Africa's and Southeast Asia's long tail **kbp dag seh ada nyn kwy sid koo kam sop vmw ndc bum bin teo sgc yao dik min war nia mad**,
  of Europe's regional languages **cor glv ext pcd sgs pnt vec vls lmo lld gag**, of Myanmar's,
  cut into syllables, **shn mnw ksw blk**, of East Asia's in Latin letters **nan hak ami za**, of the
  long tail **zne alz cce skg bss men lgg xon heh gux gmv suk spp bas ngl chw fat
  tsc niu hwc acf mrw nij guc gor quh gcr tum tll guz kde nyy
  mas mnk mer dga hrx crk csw otw tkl kne mqj liv abs ajg yrk dng tsg ewo ttj mgh knc nyf laj hnn sda lew
  sdc**, Coptic **cop**, the two Romanis **rmy rmn**, Northwestern Ojibwe **ojb**, Amharic and Tigrinya **am ti**, whose words the Ge'ez composer offers (note 16), and Gorani's Hawrami **hac**
  (FineWeb-2's, Nigerian Pidgin's, Kabiye's, Minangkabau's and Waray's with GlotCC's; note 7;
  Magahi with its Incubator text too; Lao, Khmer and Burmese cut into words with ICU's word lists too), **as** (FineWeb-2's beside Tatoeba's), **gbm raj kri tdx hvn ktu quy** (MADLAD-400's; note 8), **myx tuv** (the African Storybook's stories; note 13), **hlb kfq nah** (StoryWeaver's stories; note 17), **ahr bfy bgq bns dje grt kfy kru mjl noe sgj sjp unr xnr** (SMOL's translations; note 14), **gax mkw yaf** (eBible.org's Bibles; note 18; and Gamo's, **gmv**, beside its FineWeb-2 text). Built by
  `make fetch-corpora` + `make predict-assets` in the
  language-content repository; runtime-loaded from `/usr/share/ki-osk/predict/`.
- **Layout-only** — no bundled model yet: 173 layouts, Montenegrin's in
  Cyrillic, the forty-eight whose letters are CLDR's exemplar characters
  (note 9: twenty-eight in Latin letters, twenty in the Arabic script,
  Devanagari and Bengali), the seventeen from Keyman's (note 10: twelve
  of a script of their own, Chakma to Osage, Southeastern
  Pashayi, Southern Hindko, Chaldean Neo-Aramaic and Northern Thai in Tai
  Tham, and the International Phonetic Alphabet's), the hundred and three on another language's keys (note 12: Northern
  Thai in the Thai script, Amdo and Khams Tibetan, Sadri, Maharashtrian
  Konkani, Kutchi and Tày, twelve whose letters their
  wikis give, Northern Min to Southern Balochi, and sixteen from smaller
  wikis and FineWeb-2's few pages, Harauti to Ma'anyan, thirteen
  whose script CLDR's or Wikidata's data names, Dari to Mahasu Pahari, four the
  Universal Declaration of Human Rights, Maay to Northern Qiandong Miao,
  five Wiktionary's entries, Garo's in the Bengali script to Kui, three their test
  wikis, Central Malay, Komering and Kutai Malay, six whose script
  LinguaMeta names, Godwari to Urum, six whose letters FineWeb-2's few
  pages give, Northwest Gbaya to Eastern Oromo, Southern Luri, whose
  test wiki's two spellings give its letters, ten of Gboard's that had
  none, Nauruan to Batak Alas-Kluet, whose Wiktionary entries and test
  wikis give theirs, sixteen for fifteen more of them, Nung to
  Bangala, whose Wiktionary entries, Wikipedia's articles and Eastern
  Balochi's test wiki give theirs, and the last four, Anaang to Southern
  Betsimisaraka Malagasy, whose alphabet, PanLex's entries and JW.org's
  brochures give theirs), the two whose letters the African
  Storybook's stories give, Bukusu and Malawi Lomwe (note 13), Pulaar's,
  on Fula's keys with the letters SMOL's Pulaar translations write
  (note 14), and
  Cherokee, whose
  text the web has little of (MADLAD-400's clean text, 190 000 words, is
  its wiki's and its New Testament). They have their glides off. Sixty-eight
  layout-only languages got their models from FineWeb-2's web text
  (note 7).
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
| `thai` | Noto Sans Thai | `th nod-thai tts kxm sou` |
| `arabic` | Noto Sans Arabic | `ur ps sd ug` and 43 more, Southern Luri's (`luz`), Kuhmareyi's (`fay`), Western Balochi's (`bgn`: ہ ے), Gilaki's (`glk`: ۊ) and Maba's (`mde`: ٛ ݦ ݧ) among them, whose letters DejaVu Sans lacks (ar/he/fa use DejaVu) |
| `devanagari`…`myanmar` | `NotoSans<Script>-Regular.ttf` | the 15 Tier-3 layouts |
| `syriac` | Noto Sans Syriac | `syr cld` (RTL) |
| `tifinagh` | Noto Sans Tifinagh | `ber` |
| `ethiopic` | Noto Sans Ethiopic | `am ti gez drs hdy ktb stv sgw` (Ge'ez fidel preedit) |
| `tibetan` | Noto Sans Tibetan | `bo dz adx khg` (stacking via shaping) |
| `thaana` | Noto Sans Thaana | `dv` (RTL) |
| `olchiki` | Noto Sans Ol Chiki | `sat` |
| `cherokee` | Noto Sans Cherokee | `chr` (syllabary) |
| `canadian_aboriginal` | Noto Sans Canadian Aboriginal | `iu ojb` (UCAS syllabics) |
| `meetei_mayek` | Noto Sans Meetei Mayek | `mni` (syllabary) |
| `coptic` | Noto Sans Coptic | `cop` |
| `chakma` `hanifi_rohingya` `tai_viet` `kayah_li` `lisu` `lepcha` `saurashtra` `bamum` `cham` `tai_le` `new_tai_lue` `osage` | Noto Sans Chakma, Hanifi Rohingya, Tai Viet, Kayah Li, Lisu, Lepcha, Saurashtra, Bamum, Cham, Tai Le, New Tai Lue, Osage | `ccp rhg blt kyu lis lep saz bax cjm tdd khb osa` (Keyman's; `kjg` draws with Lao's), and `cja`, Western Cham's on Eastern Cham's keys |
| `vai` `yi` | Noto Sans Vai, Noto Sans Yi | `vai ii` (the composer's syllables on the strip and the preedit) |
| `tai_tham` | Noto Sans Tai Tham | `nod` (Keyman's) |
| `syloti_nagri` | Noto Sans Syloti Nagri | `syl-sylo` (Keyman's) |
| `cyrillic` | Noto Sans | `cu` (its Cyrillic Extended-B letters, ꙁ ꙃ ꙋ, which DejaVu Sans lacks) |
| `latin` | Noto Sans | `und-fonipa` (the IPA's phonetic letters DejaVu Sans lacks: ᶒ ᵲ, Latin Extended-F's 𐞥), `lhu` (Lahu's tone letter ꞈ) |

Install the matching face or those caps render as tofu. See the full
fonts table in ki-osk-wayland's `DEPLOY.md`. A layout that names no
`font_script` holds only what DejaVu Sans 2.37 draws
(`crates/osk-data/tests/dejavu-sans.txt`, its character map), as
`tests/layouts.rs` checks.

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
  (`Cʷ`) compose via `Cw` + vowel (`kwa` → ኳ), and the -wa forms of the
  other consonants whose rows have them likewise (`lwa` → ሏ, `bwa` → ቧ,
  `Twat` → ጧት). Tigrinya (`ti`, same
  composer) reaches its distinctive fidels via digraphs (`hh` ሐ, `qh` ቐ,
  `x` ኸ; `qhw` ቘ and `xw` ዀ their labiovelars). Since data 0.53.0 the
  keys have a Shift view of capitals, which the composer reads as SERA
  does: `T` ጠ, `C` ጨ, `P` ጰ, `S` ጸ (the ejectives, which until then no
  key typed; with the -wa forms, 45 of the 282 syllables CLDR lists for
  Amharic), `D` ዸ, `E`
  the 5th order (until then out of reach on screen: only a physical
  keyboard's Shift gave it), and `A` alone ኧ; the punctuation keys are
  Ethiopic's ። and ፣, and the symbol views every other layout has hold the
  digits, ፤ ፥ ፦ ፧ and Latin's punctuation. Ge'ez (`gez`, layout-only:
  FineWeb-2 has no text of it) is Amharic's keys and composer, named for
  it, and so are Gedeo, Hadiyya, Kambaata and Silt'e, the four whose
  syllables, as CLDR's exemplar characters list them, the composer types
  all of (Kambaata's ኧ as `A`). Since data 0.67.0 Amharic's and
  Tigrinya's composers offer their models' words on the strip, and since
  0.68.0 the next word (note 16); Sebat Bet Gurage's did from 0.68.0 to
  0.83.0 (note 19).
  And since data 0.66.0 Sebat Bet Gurage,
  whose labialised and palatalised syllables (Ethiopic Supplement's ᎀ…,
  Extended's ⷀ…) its keyboard's ʷ and ʸ keys type (note 15), and `I`
  types the 6th order (alone, the carrier's እ, which no key typed
  before). Bench's Ethiopic Extended syllables (ⶡ) are not keyed yet: its
  keyboard is in Latin letters.
- **Additional scripts**: three minority scripts the
  shaping engine now renders, generated from their xkb layouts — Tibetan
  `bo` (`cn`/`tib`; subjoined stacks compose via shaping), Dhivehi `dv`
  (`mv`, the Thaana script, **RTL**), and Santali `sat` (`in`/`sat`, the
  Ol Chiki alphabet). Each loads its own Noto Sans face on demand;
  Santali predicts since its wiki's text gave it a model (note 6), Dhivehi
  and Tibetan since FineWeb-2's did (note 7). Verified headless: Dhivehi renders
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
are built on a `us`/`es` base and add theirs the same way. A layout with a
letter only a long press types says so (`long_press_letters`), which turns
the menu on for it where long-press menus are otherwise off, as on a kiosk
by default; until data 0.52.0 thirty-one of these (Yoruba, Hausa, Igbo,
Welsh, Turkmen, Wolof, Māori, Akan…) did not, and there their letters
could not be typed, nor could Bengali's ৎ ঃ, on no key until then; nor,
until data 0.54.0, Ukrainian's ґ, Khmer's independent vowels (ឱ ឯ ឫ…,
on AltGr in its keymap), Hindi's ऍ (and so the thirteen layouts on
Hindi's keys) and Northwestern Ojibwe's long vowels and w-dotted
syllables; nor, until data 0.69.0, Odia's ୟ and ୱ (nor Kui's, on its
keys):

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
| `bn` Bengali | ৎ ঃ (on ত ং), which its keymap has on no level the keys show | `bn` |
| `or` Odia | ୟ ୱ (on ଯ ଵ), which its keymap has on no level (InScript's Shift+/ ୟ is @ there): ୟ in one word in eleven of its Wikipedia's text (ସମୟ, ବିଷୟ), ୱ in one in fifty | `or` |
| `as` Assamese | ৰ ৱ (on Bengali র/ব); ৎ ঃ (on ত ং), as Bengali's | `as` |
| `rkt` Rangpuri | Assamese's (ৰ a key, র on its long press; ৱ ৎ ঃ on ব ত ং), Bengali's digits on the symbol view's | `rkt` |
| `ckb` Central Kurdish | ێ ڵ ڕ (on ی ل ر; on Shift too) | `ckb` |
| `cnh` Hakha Chin | ṭ | `cnh` |
| `ace` Acehnese | é ë è ô ö | `ace` |
| `ban` Balinese | é | `ban` |
| `hil` `ilo` `bcl` `pag` `pam` `krj` `tet` | ñ (Hiligaynon, Iloko, Central Bikol, Pangasinan, Kapampangan, Kinaray-a, Tetum) | each its own |
| `gsw` `bar` `li` `hsb` `dsb` `scn` `fur` `pms` `lij` `wa` `ltg` `vro` `csb` `szl` `crh` `rue` `cor` `glv` `ext` `pcd` `sgs` `vec` `vls` `lmo` `lld` `gag` | each language's own letters first on its national layout's long presses (the language table above lists them: Upper Sorbian's ć č ě ł ń ó ř š ž ź on German's keys, Rusyn's ы ъ ё on Ukrainian's…) | each its own |
| `haw` `sm` `to` `ty` | ʻ, the glottal stop, a key of its own at the home row's end | each its own |
| `ce` `inh` `av` `kbd` `ady` `lez` `lbe` `tab` `dar` | Ӏ, the palochka, a key of its own at the home row's end (on Russian's keys) | each its own |
| `yua` `quc` `kek` `quz` | ʼ, the glottal stop and ejective (Kʼicheʼ, kʼo), a key of its own at the home row's end | each its own |
| `dz` Dzongkha | ་, the tsheg, a key of its own at the home row's end (its keymap types it on the space bar) | `dz` |
| `blk` Pa'O | its tones ꩻ and ႏ, keys of their own in place of Burmese's ဩ and ၏ (on Burmese's keys) | `blk` |
| `nan` `hak` | their toned vowels first on the long presses, the commonest first: Southern Min's á â ā à a̍, o͘ in its five tones on o, ⁿ first on n; Hakka's â à á a̍, ṳ in its five tones on u | each its own |
| `ami` Amis | ', the glottal stop, a key of its own at the home row's end (its text writes it ’, ˈ and ^ too) | `ami` |
| `mg` Malagasy | ñ ô à ì on n o a i (à for the stress that tells words apart, tanàna, alàlana: in 0.6% of its web text's words); ', the apostrophe a word writes before the next (amin'ny: one word in thirteen of the UDHR's), a key of its own at the home row's end, which Tandroy-Mahafaly's and the Betsimisarakas' keyboards keep | `mg` |
| `rn` `rw` Kirundi, Kinyarwanda | ', for the vowel a word drops before the next (n'umwe, y'igihugu: one word in eleven of each one's UDHR, 8.1% and 6.6% of their web text's), a key of its own at the home row's end | each its own |
| `ps` Pashto | ۀ on ه (نۀ, پۀ: Pakistan's spelling of the final ə), which xkb's keymap lacks | `ps` |
| `glk` Gilaki | ۊ and ؤ, in nearly a tenth of its words each (2.8% and 2.6% of its Wikipedia's letters), keys of their own in place of ذ and ظ (0.07% and 0.09%), which are on their long presses | `glk` |
| `scl` Shina | the inverted damma ٗ, its rising tone (as the superscript alef ٰ its falling one), on the Shift view's damma, which CLDR's exemplars lack | `scl` |
| `om` Oromo | ', its glottal stop, the hudhaa (ta'e, danda'a: one word in twenty of the UDHR's), a key of its own at the home row's end, which Southern and Eastern Oromo's keyboards keep (Eastern's typing ’) | `om` |
| `zne` `skg` `bss` `men` `lgg` `xon` `dnj` `gbo` `gux` `suk` `spp` `bas` `ngl` `chw` `fat` `bud` `kao` `gvl` `nnb` `cce` | each language's own letters first on the long presses, with their tones where its text writes them (Akoose's ɛ ə ɔ with seven tones between them, Northern Grebo's ɛ ɔ with three, Ntcham's on b l m n too) | each its own |
| `niu` `acf` `mrw` `guc` `sas` `bpr` `quh` `gcr` `tum` `tll` `kde` `nyy` `mas` `mnk` `mer` `dga` | each language's own letters first on the long presses (Niuean's macrons, Sasak's ẽ, Tumbuka's ŵ, Nyakyusa's ị ụ, Dagaare's ɛ ɔ ŋ and nasals…) | each its own |
| `alz` `lgg` `suk` `spp` `hay` `kbr` `kao` `gvl` `nnb` `nyo` `gor` `tll` `guz` `quh` `guc` | an apostrophe, a key of its own at the home row's end: they write one in 3% to 11% of their words (Sukuma's ng'wa, Gulay's le’n, Nande's n'abandu, Gorontalo's nga'amila); South Bolivian Quechua's is its ejective, ʼ, as its text writes it; Wayuu's ü, in a third of its words, has one beside it (Akoose's glottal stop ʼ was one too, until it moved to x: below) | each its own |
| `ksw` Sgaw Karen | Keyman's phone layout, one letter a key on the base view, the asat ် a key of its own at the home row's end; whole words on long presses (လီၤ on လ, ဒီး on ဒ) | `ksw` |
| `hrx` `crk` `csw` `tkl` `liv` `ajg` | each language's own letters first on the long presses (Hunsrik's ë, the Crees' long vowels, Tokelauan's macrons, Livonian's õ ō ȯ ȭ ȱ ä ǟ ḑ ț ŗ on Latvian's keys, Aja's ɛ ɔ ɖ ŋ and tones) | each its own |
| `crk` `csw` `liv` `ajg` | the letters a fifth to a third of their words write, keys of their own at the home row's end: Plains Cree's ā ē, Swampy Cree's â ê, Livonian's õ, Aja's ɔ ɛ | each its own |
| `otw` `mqj` | an apostrophe, a key of its own at the home row's end (Ottawa's Mzin’igan, Mamasa's tae', in 4% and 9% of their words) | each its own |
| `lhu` Lahu | its tones, written after the syllable, on keys: ˬ and ˇ (a fifth of its words end in each) at the home row's end, where its desktop keyboard has them; ꞈ, its apostrophe and ˍ on q, x and z, which it does not write; ˆ and ˉ on the long presses of ꞈ and ˍ | `lhu` |
| `cop` Coptic | its jinkim (U+0300, ⲛ̀ⲧⲉ, in a third of Bohairic's words) a key of its own at the home row's end, in place of its keymap's dot above and macron; the overline of the nomina sacra (Ⲓⲏ̅ⲥ̅) on its long press | `cop` |
| `tsg` `ewo` `ttj` `knc` `nyf` `laj` `hnn` | each language's own letters first on the long presses (Tausug's macrons, Ewondo's é ë è ə and tones, Tooro's and Lango's ŋ, Kanuri's ə, Giryama's v̱, Hanunoo's ê) | each its own |
| `tsg` `ttj` `nyf` `sda` `sdc` | an apostrophe, a key of its own at the home row's end: they write one in 3% to 16% of their words (Tausug's and Toraja-Sa'dan's glottal stop, di’ and tae’; Tooro's n’omu, Giryama's at'u, Sassarese's cu’) | each its own |
| `knc` Kanuri | ə, which a quarter of its words write, a key of its own at the home row's end | `knc` |
| `yrk` Nenets | on Russian's keys: ӈ in place of the ъ it hardly writes (ъ stays on ь's long press); its two glottal stops, the apostrophe and ˮ (поʼ, ненэцяˮ), keys of their own at the home row's end: nearly half its words write one | `yrk` |
| `dng` Dungan | on Russian's keys: ә in place of ъ, which it does not write; җ, which a seventh of its words write, a key of its own at the home row's end; ү ў first on у's long press | `dng` |
| `rmy` Vlax Romani | its č ć š ž đ first on the long presses of c s z d (on the US keys; Balkan Romani's are keys of their own on Serbian's Latin keys) | `rmy` |
| `kbp` `bss` `gbo` `nnb` | keys for the letters their words write most of those only a long press typed (more than a quarter of Kabiye's, Akoose's and Grebo's letters; in a quarter of Nande's words): two at the home row's end, Kabiye's ɛ ɩ, Akoose's é á, Grebo's ɛ ɔ and Nande's ĩ beside its apostrophe; and in place of q and x, which they hardly write (q and x on those keys' long presses), Kabiye's ʊ ɔ, Akoose's ɛ and glottal stop ʼ, Grebo's ǒ ě and Nande's ũ. The words a long press is needed in: Kabiye's 76% → 24%, Akoose's 85% → 52%, Grebo's 70% → 40%, Nande's 26% → 1% | each its own |
| `ojb` Northwestern Ojibwe | Keyman's fv_ojibwa, a syllable a key: the i and a syllables and the finals on the base view, the e and o syllables on Shift; each syllable's long vowel and w-dotted forms first on its long press (ᑭ's ᑮ ᑶ ᑸ), which Keyman types with keys of their own (˙ after a syllable, ᐧ before it) by rules a key of one letter has not; the final ᕽ on ᒃ's, the full stop ᙮ on the full stop's | `ojb` |
| `ccp` `rhg` `blt` `kyu` `lis` `lep` `kjg` `saz` `bax` `cjm` `tdd` `khb` `osa` | Keyman's keyboards for their scripts, their rows and places kept (note 10): what Keyman types after another key on a long press (Tai Dam's ꪂ on ꪀ, Lepcha's ᰂ ᰁ on ᰀ), and what it types under AltGr (Cham's ꨒ on ꨗ); Rohingya's 𐴗 on 𐴖; Bamum's two marks and Osage's acute and double acute beside the space bar; the digits no letter key has on the long presses of the symbol view's, their stops (Saurashtra's dandas ꣎ ꣏, Bamum's ꛳ ꛵…) on the full stop's | each its own |
| `hac` `psi` `hnd` `cld` `nod` `syl-sylo` | Keyman's keyboards read at a later commit (note 10), their rows and places kept: what a key types under AltGr on its long press (Pashayi's ك ڰ, Tai Tham's), Hindko's ݨ on ن, Tai Tham's ᩀ on ᨿ; Chaldean's East Syriac vowel points and marks its Shift view; Syloti Nagri's hasanta clusters (ꠘ꠆) on their letters, and ⁕ ꠨ ꠩ ꠪ ꠫ ॥ . on its danda key's | each its own |
| `und-fonipa` IPA | Keyman's sil_ipa on the US keys (note 10): each letter's IPA letters and then its modifier letters on its long press (e's ɘ ə ɛ ɜ œ… ᵉ ᵊ, h's ħ ɥ ɦ ɧ… ʰ ʱ), Shift's small capitals on the capitals'; ː ˈ ˌ ʼ ‿ and the super- and subscript digits on the first symbol view, the combining diacritics and tone letters on the second | `und-fonipa` |
| `kjp` Eastern Pwo Karen | its tone ႋ and ၜ ၠ keys of their own in place of Burmese's ဩ ၏ ၒ; ၮ ၯ ၰ ၞ ၟ ၚ ဴ on the long presses of ဏ ယ ဃ န မ င ု (on Burmese's keys; note 12) | `kjp` |
| `tyz` Tày | the macron below and the double grave, the two tones Vietnamese lacks, first on each vowel's long press (a̱ ȁ; on Vietnamese's keys; note 12) | `tyz` |
| `cdo` `mnp` | their toned vowels first on the long presses, the commonest first, as Southern Min's and Hakka's: Eastern Min's á â ă ā à, those with the diaeresis below after (ā̤ â̤…; ṳ̄ ṳ̆ on u); Northern Min's â̤ á a̿ ă ǎ…, its seventh tone the double overline (note 12) | each its own |
| `egl` Emilian | its dialects' letters first on Italian's long presses: ś ṡ ş ṣ on s, ṅ on n, ō ē ū beside the graves, ć ṿ ł ǵ ż | `egl` |
| `bbj` Ghomala' | ə ɛ and their tones on e, ɔ and its tones on o, ʉ on u, ŋ on n, ƴ on y, its vowels' tones first (é ə ə́ è…; on the US keys; note 12) | `bbj` |
| `sty` `qxq` `rej` `bcc` | Siberian Tatar's ё on е (on Bashkir's keys), Qashqai's ێ on ی (on South Azerbaijani's), Rejang's ê first on e (on the US keys), Southern Balochi's ۏ ݔ ګ on و ی گ (on Balochi's; note 12) | each its own |
| `zea` Zeelandic | ê ô first on Dutch's long presses of e and o (è ë ò after them), and an apostrophe key at the home row's end (note 6) | `zea` |
| `igl` Igala | ọ ẹ first on the long presses of o and e, the tones its writers mark after them (ó ò ọ́ ọ̀, é è ẹ́ ẹ̀) and on i a u (í ì î, á à, ú ù), ñ ń on n, and an apostrophe key at the home row's end (on the US keys; note 6) | `igl` |
| `vel` Veluws | ö first on Dutch's long press of o, ä å on a's, ü on u's, and an apostrophe key at the home row's end (note 6) | `vel` |
| `uum` Urum | on Russian's keys: ӧ ӱ ґ on the long presses of о у г, and a key at the home row's end for the ʼ it writes where Russian writes ь (note 12) | `uum` |
| `myx` `tuv` `bxk` `lon` | an apostrophe, a key of its own at the home row's end, for the ng' and elisions they write (Turkana's ng'esi, in a sixth of its words; Bukusu's chiisang'i; Malawi Lomwe's eng'ombe; Masaaba's n'aaba), and Masaaba's ŋ on n's long press (on the US keys; note 13) | each its own |
| `kru` `noe` `fuc` | Kurukh's apostrophe (र'ईर, one word in twenty-six) first on the long press of its full stop । and Nimadi's avagraha ऽ (करनऽ, one word in twenty) on the visarga ः's, as Sanskrit's (on Hindi's keys); Pulaar's ñ first on n's long press, before ŋ ɲ (on Fula's keys; note 14) | each its own |
| `cu` Church Slavonic | on Russian's keys: є ѣ ꙑ ꙁ ꙗ in place of е э ы з я, which are on their long presses; ѥ and its accent marks (́, then ҃ ҄ ҆ ̀ ̑ on its long press) keys of their own at the home row's end; ѧ ѩ on ꙗ, ѫ ꙋ ѹ on у, ѭ on ю, ѡ ѻ ѿ on о, ї і ѵ ѷ on и, ѳ ѯ ѱ on ф к п, ѕ ꙃ on ꙁ | `cu` |
| `ctg` `niq` `max` `cyo` `mog` `mqy` `bug` `meo` | Chittagonian's ৱ ৰ on ব র (on Bengali's keys); Nandi's ñ ā ē ĩ ō, North Moluccan Malay's accented vowels, Cuyonon's ñ, Mongondow's ḷ ḻ, Manggarai's and Buginese's é and Kedah Malay's å first on the long presses (on the US and Malay's keys; note 12) | each its own |
| `niq` `mog` `mqy` | an apostrophe, a key of its own at the home row's end: Nandi's ' (para'ab), Mongondow's glottal stop ' (lopi'), Manggarai's ’ (Pu’ung) | each its own |
| `gya` `mwm` `mcn` `mpg` `rim` `hae` | keys for the letters their words write most, from FineWeb-2's pages of them (note 12): Northwest Gbaya's ɛ and apostrophe at the home row's end, ɔ and the cedilla of its nasal vowels in place of x and q, its tones and ŋ ɓ ɗ first on the long presses; Sar's tone marks, the macron and the acute (the tilde below on its long press), at the home row's end, ə ɔ in place of q x, ɓ on b; Masana's ɗ ŋ ɓ ɦ on d n b h and an apostrophe key; Marba's â á ä à î ï ì ê ô û and a ’ key (its implosive d’); Nyaturu's ë and apostrophe keys, ü on u; Eastern Oromo's ’ key (on Oromo's keys) | each its own |
| `luz` Southern Luri | the letters its test wiki's spelling of every vowel writes most, ة ە ݖ ۿ, keys of their own in place of ث ذ ظ ض (on their long presses); ۊ ۋ ۏ on و, ڤ on ف, ڍ ڌ on د, ڲ on گ, ݩ on ن, ھ on ه and the mark ٛ on the full stop (on Persian's keys; note 12) | `luz` |
| `nau` `prk` `gay` `ijc` | Nauruan's â and ñ on the long presses of a and n (on the US keys); Parauk's tones, á ā é ē í ī ó ō ú ū first on its vowels', and a key at the home row's end for the colon it writes inside its words (cia:, Ye:sux); Gayo's é è first on e's and ö on o's (on Indonesian's keys); Izon's ẹ first on e's and the acute and grave tones on every vowel's (on Igbo's keys, ị ọ ụ first on theirs) (note 12) | `nau` `prk` `gay` `ijc` |
| `plk` `fay` `wne` `cja-latn` `bxg` | Kohistani Shina's ڦ ڱ first on the long presses of ف and گ and its rising tone's ٗ on the Shift view's damma (on Shina's keys); Kuhmareyi's ڌ on د, ۊ ۆ ۉ on و, ٱ on ا, ێ ؽ on ی, ݩ on ن and the mark ٛ on the full stop (on Persian's keys); Wanetsi's ۀ on ه and ں on ن (on Pashto's keys); Western Cham's â ă on a (on the US keys); Bangala's á on a and ɓ on b (on Lingala's keys) (note 12) | `plk` `fay` `wne` `cja` `bxg` |
| `anw` `haq` `bmm` `bzc` | Anaang's ñ ö ü on the long presses of n, o and u (on the US keys); Ha's apostrophe, for the vowel it drops before the next word (nʼumunsi), a key of its own at the home row's end; Northern Betsimisaraka Malagasy's n̈ first on n's, before Malagasy's ñ, and ỳ à ê ì ï on y a e i, Southern's à é ỳ ì on a e y i (on Malagasy's keys, its apostrophe key among them) (note 12) | each its own |
| `prs` Dari | the Pashto letters CLDR's Dari lists, ټ on ت, ځ څ on ح, ډ on د, ړ ږ on ر, ښ on س, ګ on گ, ڼ on ن, and ى after ئ ي on ی (on Persian's keys; note 12) | `prs` |
| `bpy` Bishnupriya Manipuri | ৱ on ব (on Bengali's keys; note 12) | `bpy` |
| `tdx` `hvn` `quy` | Tandroy-Mahafaly Malagasy's à è é ò ì ỳ (on Malagasy's keys), Hawu's è (on the US keys) and Ayacucho Quechua's á é (on Quechua's) first on the long presses; Hawu's saltillo ꞌ, its glottal stop, a key of its own at the home row's end (note 8) | each its own |
| `vai` Vai | its letters Keyman's capitals type, which its composer reads (note 11): ɛ in q's place, ɔ at the home row's end, the nasal ~ in Shift's; ŋ þ ð ʒ on the long presses of n t d j, the lengthener ꘌ on x's; Vai's comma, full stop and question mark on those of , . ? | `vai` |
| `cnr` `cnr-cyrl` Montenegrin | ś ź on the long presses of s z (on Serbian's Latin keys), с́ з́ on с з (on its Cyrillic ones) | `cnr` |
| `aa` `bew` `bom` `cbk` `cgg` `cho` `dje` `dzg` `fan` `gqr` `igb` `jbo` `las` `led` `lkt` `mkw` `mnb` `mua` `mvp` `nup` `nv` `pmy` `pov` `sba` `sef` `snk` `srr` `tbw` `tem` `xog` `fub-latn` `bcq-latn` | their letters CLDR's exemplar characters, first on the long presses of the letters they are made from (note 9); a glottal stop written as a letter, and an apostrophe their words write, keys of their own at the home row's end; keys for the letters their words write most of those: Navajo's ʼ í at the home row's end and á é on q x, Lakota's ŋ á and č é, Temne's ɔ ŋ and ə ɛ, Gor's ɨ and apostrophe and ə ɔ, Ngambay's ə and apostrophe and é ɔ, Mundang's ǝ and apostrophe | each its own |
| `bto` `mui` `mfa` `ibb` `vmf` `sdh` `bgn` | the letters of the Incubator wikis' languages first on the long presses of those they are made from: Rinconada Bikol's ñ, Musi's é è ó, Kelantan-Pattani Malay's è é ó ũ, Ibibio's ọ ñ n̄ ị ẹ ụ ʌ (on the US keys); East Franconian's â ô (on German's); Southern Kurdish's ۊ on و (on Sorani's); Western Balochi's ے ں ٹ ڈ ڑ ہ ھ ګ on ی ن ت د ر ه گ (on Persian's) | each its own |
| `mak` `pmf` | an apostrophe, a key of its own at the home row's end, for their glottal stop: Makassarese's se're, Pamona's ince'emo (a seventh of Pamona's words) | each its own |
| `bft` `gju` `mvy` `gwc` `phr` `scl` `trw` `bal` `haz` `mki` `kvx` `glk` `lrc` `mde` `bgc` `xnr` `lif` `syl` | their letters CLDR's exemplar characters, on their script's keys (Urdu's, Sindhi's, Persian's, Sorani's, Arabic's, Hindi's, Bengali's): those the keys lack first on the long press of the one they are made or drawn from (Balti's ڗ on ر, ݨ ݩ on ن; Shina's ݜ on س; Dhatki's ۯ on ر; Kangri's ॐ on ओ, ऽ on ः), a mark the keys lack on the full stop's (note 9) | each its own |
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
