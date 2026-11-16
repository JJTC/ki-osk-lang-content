#!/usr/bin/env bash
# fetch-fineweb.sh — web text of the languages that have no Wikipedia of
# their own, from FineWeb-2 (the Makefile's FINEWEB_LANGS): Bodo, Dogri,
# Chhattisgarhi, Magahi and Brahui, Sindhi in Devanagari and Meitei in
# the Bengali script, whose wikis are in their other scripts, Lao, Khmer
# and Burmese, whose wikis are small or few write as people do, and
# Kokborok, Mizo and Khasi, which have none; thirty-eight languages that
# had a keyboard and no model (Swahili, Slovenian, Kazakh, Zulu…), of
# whose train split the first file is read; Central Kurdish (Sorani),
# whose wiki is Wikipedia's prose, and Nigerian Pidgin, whose wiki
# spells as few type (scripts/fetch-glotcc.sh reads GlotCC's beside it,
# after this); nineteen languages of Southeast Asia (Hmong Daw, Hakha
# Chin, Hiligaynon, Iloko, Iban…); twenty-six of Europe's regional
# languages (Nynorsk, Asturian, Low German, Neapolitan, the Sorbians…),
# whose text FineWeb-2 has their wikis' pages among; fifteen of Russia's
# and the north's (Sakha, Chuvash, Northern Sami, the Maris…) and
# twenty-three more that had a keyboard and no model (Udmurt, Greenlandic,
# Oromo, Romansh, Kirundi…); and
# Assamese, whose Tatoeba corpus is small, beside it (the Makefile's
# FINEWEB_PLUS_LANGS).
#
# FineWeb-2 (Hugging Face) is Common Crawl's web text sorted by language;
# it is ODC-By 1.0, and its use subject to Common Crawl's Terms of Use.
# Each language's parquet files, its `train` and `test` splits at one
# revision of the dataset, are pinned by sha256 below.
# scripts/fineweb-text.py keeps the documents that read as their language
# (FineWeb-2's language classifier mistakes close languages: its Dogri
# holds Punjabi poetry) and writes
#
#   $OUT/<lang>.txt       the train split's sentences, each once: the
#                         corpus build-predict learns the model from
#   $EVAL_OUT/<lang>.txt  the test split's, none the corpus has: what
#                         eval-predict measures the model on
#
# Neither is committed: the text is the web's. The parquet files are kept
# in $OUT/raw, and a kept one whose sha256 is right is not downloaded
# again; with KEEP_RAW=0 each language's are deleted once its text is
# written (Khmer's and Burmese's are 2.3 and 3.1 GB: what a CI runner's
# disk is spared). Reading them needs pyarrow: PYTHON names a Python that
# has it.
#
#   scripts/fetch-fineweb.sh            # every language
#   scripts/fetch-fineweb.sh brx sd-Deva    # some
#
# Env: OUT=$TMPDIR/ki-osk-sources/fineweb  EVAL_OUT=$OUT/eval  PYTHON=python3
#      KEEP_RAW=1
set -euo pipefail

REV=af9c13333eb981300149d5ca60a8e9d659b276b9
BASE=https://huggingface.co/datasets/HuggingFaceFW/fineweb-2/resolve/$REV/data
# language  configuration  file  sha256
FILES="
brx  brx_Deva  train/000_00000.parquet  76b80110a811ad10b062dc5bc486fff813397d26f6289c6a04bdc5b862f761a8
brx  brx_Deva  test/000_00000.parquet   349eafb3641435fd7aebeb1dd1541c0e22cd27a0998d2428062bac00a339d42d
doi  doi_Deva  train/000_00000.parquet  18605bd59592a6ad4f9e97f48753996f41dec5c32842f49ecd8e70433900521e
doi  doi_Deva  test/000_00000.parquet   78011a0fc0cf93cbc2d057184fff07420657a3bc6ff3a1842404f5d2b356d51a
hne  hne_Deva  train/000_00000.parquet  b1a92142dc14e50b3faf46b4c1c228f01a88575d39210aea8ff892af10fab9b5
hne  hne_Deva  test/000_00000.parquet   b9ac9ba7cf3fabec8173fe96141447404499d5fda97c09733919f327a94b3497
mag  mag_Deva  train/000_00000.parquet  fcc6243b472151eb3b2078c7b44f8c415cfc5a882e18b8ecc10f7a3957d85177
mag  mag_Deva  test/000_00000.parquet   48cc4df0baa1595ea27aa1f194d51eec8758ead19362b02e8a1dde1099069f7c
brh  brh_Arab  train/000_00000.parquet  1b8ba9a400a80d9a8e510e2ea1ab308804d3fc68a6512e867a6ad67b4f05c2fb
brh  brh_Arab  test/000_00000.parquet   cd25fcf97bcd2311b89c1cd72a1f54e52594bfef38e30886c1538cdfbfdb4de2
sd-Deva   snd_Deva  train/000_00000.parquet  7f1ec469c99291070d1260138aaae2baea07bde8d67054762ffdf3681a66b9c9
sd-Deva   snd_Deva  test/000_00000.parquet   cf891207f43a93754a5f0c870fe9c4da73fb28e1e575e6c7d1e29523fa2a8638
mni-Beng  mni_Beng  train/000_00000.parquet  70c880e2be17a13639b86139961cda481a96c25429ba0f136e6061cf6288a73e
mni-Beng  mni_Beng  test/000_00000.parquet   92dd42dc4360a753b7e1a96a5c2a92edcf52a7f747c1c9e73346b24df8875773
lo   lao_Laoo  train/000_00000.parquet  19e5de2c1054ceed9946a91922db64690af5e61b248c8556de0832c8d3f66040
lo   lao_Laoo  test/000_00000.parquet   d0d84f18de6400f06a2ed725f0858a517593fbbe0ac0698dc3418b2f5138b4ce
km   khm_Khmr  train/000_00000.parquet  6f4506f65ad66b48e5f73350e593fcac045e15fdf65d4a55924199b84c7ee993
km   khm_Khmr  test/000_00000.parquet   26ae4e654770b07e24a4ef6b8ef3a01c84076a920f1922ac014364ab411b84f0
my   mya_Mymr  train/000_00000.parquet  d7d8182c4fed89e2a81e69a2c6ef5e6083c9ff3df48c276cd87934b41a6089a0
my   mya_Mymr  test/000_00000.parquet   61c4bb5b164bc690815f16f997e35a20068021e4279357042e399283306ed25d
sw   swh_Latn  train/000_00000.parquet  947eb094e73b5d934a21e996b21405092a0537dd0b2a44810e621c8664b1c60b
sw   swh_Latn  test/000_00000.parquet   691a5e068adafa26cafad5ae79675997a2742ad056dd1df19729ef0975673906
af   afr_Latn  train/000_00000.parquet  3a5b2c02fca9ed9f95232d6b56713dcb27f419c8bd837c6d3a8a7384a400a7de
af   afr_Latn  test/000_00000.parquet   95937767aadf4f677bb833a471a6fb928f025f0b781c55a1e2a2d6060754540b
sl   slv_Latn  train/000_00000.parquet  e9ef19f8901096acba2b871c9f1d1c66f837375195c773434c8f76089d51e8dd
sl   slv_Latn  test/000_00000.parquet   c511b10f3b92559a896bcd379a27890512d83b16a435e45cbaae2872f2d65509
zu   zul_Latn  train/000_00000.parquet  4821a416e2c8bf0c3d7ef075245be32b85fe52ca1f45fc767ac18bb1d80ea740
zu   zul_Latn  test/000_00000.parquet   07a39540741ff7e52a9aa6040e418e07f8f417e5a3ccec432d33e0b420f49ce8
sq   als_Latn  train/000_00000.parquet  63cdd078064346807de7db8051d2cf88059ed1f241aa2d31fe24b3de39341e7e
sq   als_Latn  test/000_00000.parquet   bf2b939e00de3942ef9a3a4abdb8081472c030edce4280d0b9b8d87f5c016f18
kk   kaz_Cyrl  train/000_00000.parquet  73c49c5c7d221481b8ae456252338428cd5f93d7f4c9233024bf404ba555caed
kk   kaz_Cyrl  test/000_00000.parquet   f3269a305f48bb9cccc3b69ec64cd9a05bcfa9e248bac1172523db38d2064608
uz   uzn_Latn  train/000_00000.parquet  1bf6de7b3180172ca2295cc60fa644eb2f83017d22ef194deb8b7f876b02b7e3
uz   uzn_Latn  test/000_00000.parquet   2c6e528e1726a146abdd76b6add8f7f614a244e937d14de15a91cd7157cb2bc7
bs   bos_Latn  train/000_00000.parquet  cf9200044d3ab62bbd54d5d70e7ab1230705aee2d5cf95404073227d7ff488fa
bs   bos_Latn  test/000_00000.parquet   359880fa4a663981b3092eb167451330bc4b4e1bb37eab426e04755f7871748b
ky   kir_Cyrl  train/000_00000.parquet  8ae30162ca29117c22d470a2a3837fb5748f7d3882db8ad54df4fde5e301bb41
ky   kir_Cyrl  test/000_00000.parquet   f28b3829be9f1197ecbc557b3c169e7116f1cf07ef516ba132ab166a067ffed1
tg   tgk_Cyrl  train/000_00000.parquet  2546bb779a97d49d469d8fb647999e7ddbd9242df2807ec52b9c22ec900a9f77
tg   tgk_Cyrl  test/000_00000.parquet   a3fd71c6250ebb3bc1efefccc726782c0300b1bef6caa6feb4a7f20ae47a75fa
cy   cym_Latn  train/000_00000.parquet  2e82ec4f4a2aa1bbe6051660b62886291b15e4db77d6d128c7d8c2c04496f44e
cy   cym_Latn  test/000_00000.parquet   8e21e5d68f968891f83b97ff20d2049af4b0fc6f3f915d81c27976fec338e1d2
so   som_Latn  train/000_00000.parquet  ef1914dab809c4835fa56c54f2e8e9d4b24b57d74fecb06dfc1c49eee22c91cd
so   som_Latn  test/000_00000.parquet   3723dc49123c674e3b103d1ade14a8b2a083214ab4beb49a90bfea09f73d9ee0
ga   gle_Latn  train/000_00000.parquet  03f2a5d94b506257cecd578a939eb297a26af84101444bd262a193721efa1ec6
ga   gle_Latn  test/000_00000.parquet   ce1b9eedd61c6505cabd1b4f506c9886573f9c278e69892c5cf89a950bc00917
mt   mlt_Latn  train/000_00000.parquet  4aff356520a50842f574f257c582543385c39bd387a68792cc7d42b341541c78
mt   mlt_Latn  test/000_00000.parquet   5b47e81b543e560263a89bf2d8fbb82c8b242d2454a90082c3e580ea82168140
ha   hau_Latn  train/000_00000.parquet  44a054948a9e92addb334b24e2dfb1775db97d785b47be3fbcd941badb323ef8
ha   hau_Latn  test/000_00000.parquet   f12a70a9c5d97fd051902a5007ab9659d808b28c769236502f36ed3fca1290b0
lb   ltz_Latn  train/000_00000.parquet  112b76629c3f23da980f2017aafc94b3d833f24c0f15a819d132babc88f7e5a9
lb   ltz_Latn  test/000_00000.parquet   ec7b91838d2e7cda1d185a550bb255fe0f9c404f6a07e447b13c1bdf015648a3
mg   plt_Latn  train/000_00000.parquet  be6ffe2ce57c1bc160839b56a97db601ef43fd4d565620dc8c421d490630cb9c
mg   plt_Latn  test/000_00000.parquet   fc5c381cbd8d99e658e5a86be7ca0da431383674b948277cf0737771620f98f9
rw   kin_Latn  train/000_00000.parquet  68c47e09e9fdf73baac4f21213e7ac11b3502729a19a4b0ae5901beade4f04ed
rw   kin_Latn  test/000_00000.parquet   1161a8f829264f8674e9f10b9bce9666d89bc310256591f5fd88106198370843
gd   gla_Latn  train/000_00000.parquet  5c3b2241dbd97de8e44534fd8794f1bd7419eb4ad935a2dada78e20555d5781d
gd   gla_Latn  test/000_00000.parquet   1d3050582f1e1d66d56db7219906e7c5d9f6a0a6b06b090cacb6516d8892656b
fy   fry_Latn  train/000_00000.parquet  3ed67ac48ccb722b87b3c0f1c241cc5d3328d2d4b815014c4a63f67b8783711b
fy   fry_Latn  test/000_00000.parquet   f72c2ca9962657d237c897411d53a9ecc0c0268014a1b8252cb52f5e0eb6bc80
jv   jav_Latn  train/000_00000.parquet  82a1785cfe88313ab9a1f47b6c8ec34233dbc10e8fe6d698cf6c80e4834f787e
jv   jav_Latn  test/000_00000.parquet   f450e11c7275f9a3d21ae44cb86faf2d760c5e7c4d01fe2768eed1f3bbca274d
tk   tuk_Latn  train/000_00000.parquet  f21e703f98eab1a58f4ad0a65893b7b16079492f7c37af1c242a89f479a529a8
tk   tuk_Latn  test/000_00000.parquet   cd3adc265ed4a6f4dade86961a70455de0683107af42426dc8d2a707dc125a26
ht   hat_Latn  train/000_00000.parquet  a20a800fdcc8876c15daf42e7d1980dbcefd12037f321048374c9c8affae62f4
ht   hat_Latn  test/000_00000.parquet   aaa350b3b2467830c15ac467d6ee0f101db2423e5c74ca9ea982e9da9c9893f3
ba   bak_Cyrl  train/000_00000.parquet  8acf0cda328accfe2db4093b6e618e5873f3f7a53499ff5e2d4b81b3fb8a0ef5
ba   bak_Cyrl  test/000_00000.parquet   6abfd5dbf3d756bb917ccb01b508242df657a24c589dc0393020f38810453a44
xh   xho_Latn  train/000_00000.parquet  65d92c6b8102931a838683881323054d9c1cf4471f3f9f5fbc6cb0c1d77c8790
xh   xho_Latn  test/000_00000.parquet   9a000bcc30bb70a1a28ae179d640c1f0fdac6da31628429a6698f650b7757573
fo   fao_Latn  train/000_00000.parquet  dc632925f2207c18f02e0197a7561da03ce80772524b8ac3fa2f9794997273c6
fo   fao_Latn  test/000_00000.parquet   16ab4dca9b97facf2fba92d00584fedb4b30fde8eaa19ee79c2ef63a0a5b73d0
ceb  ceb_Latn  train/000_00000.parquet  4982dae9f61107d1f9c60725bb8434e1d0d4b990f596c8fd1e890ebef5f98a3a
ceb  ceb_Latn  test/000_00000.parquet   6306e28bc11a70aafe22f6aebcd84a9d40d12e778106dd08b7702411e35c077b
mi   mri_Latn  train/000_00000.parquet  bf2719b555d52734700eb1cf030e91c4a48bc2d4ca7eed02c36a1c3511a387c5
mi   mri_Latn  test/000_00000.parquet   55677506712d6a64d6cef8fffa5e37ad29b33edf8fddf4f5dbe40b85e533481f
su   sun_Latn  train/000_00000.parquet  4686a31da809abfefefe19e1de3572f83ec70d54f543d4351663ddb158a65e55
su   sun_Latn  test/000_00000.parquet   5c2e06c1d0f1820793d2c9858f7261e5efc447fded05f22a6c2b339f8f851d0e
co   cos_Latn  train/000_00000.parquet  383c945a39bac0e55471ebd62686465afcfb87404d1318c6cfd3b36182caf138
co   cos_Latn  test/000_00000.parquet   779ea6415f0226a5f6ee4f25094c70b625295bfd59337dc9a23a4e79ef7c4cd9
ny   nya_Latn  train/000_00000.parquet  3e43f0fb7da66aa611ef01cd896a6a3d5e2b7f41327b0ca5b17530545350e084
ny   nya_Latn  test/000_00000.parquet   83f8661d63eb4254e1e8748153d22dfc021ed3371d1ed8ded1cae3809c9d0fec
sm   smo_Latn  train/000_00000.parquet  91aa508532c98294312deaea2aa2286297a9bf13d785f9e78cc7938e72a29c01
sm   smo_Latn  test/000_00000.parquet   0c71952fa37dbe2ccd12670ae61acd8d6d9401e4c305e175cefda8e62c5208d1
sn   sna_Latn  train/000_00000.parquet  584789dcda57a41648cee3003c46b49a18bd659b07f465e5da6c6fdd859159c2
sn   sna_Latn  test/000_00000.parquet   2cb65e119d95549d6c2ee1d06522b6ddb0d341c4339f33dde8e26aedfc574498
ig   ibo_Latn  train/000_00000.parquet  6fa74f8c0174f101aa1e8ed8deff9ac4de9e73d6a71a7ff7156d82d5531198c0
ig   ibo_Latn  test/000_00000.parquet   e84a4b13072ca3ed377514c6ccee90adb72b05b5c6d3315b8dcd6eaadabf829d
st   sot_Latn  train/000_00000.parquet  0870749a3e67319cd8b5567275ff57235dfd1f1e868ea7780837fa39d7180574
st   sot_Latn  test/000_00000.parquet   8fc2c018643c18b7ef0510ae9e51d3dfee97046d366667ca0da59e24b1fe64e8
haw  haw_Latn  train/000_00000.parquet  8f7c31e05173a24c01fa9452269784883db4a3c203d6c7d4ebda46fc120b1e9a
haw  haw_Latn  test/000_00000.parquet   dcc5b7aae61ad7aef036feb174a383681046ec69244e83eee27d76bb1c5211af
yo   yor_Latn  train/000_00000.parquet  c68be98df36ac8fbbf0f2e0dbf9e0c116df5aba239f5008f2362f9fdb9579520
yo   yor_Latn  test/000_00000.parquet   0d7031c38478bfe11f8a42cb1f63f8b6d644ce183b474b59d0a5e9e36317b113
dv   div_Thaa  train/000_00000.parquet  a0ec1b2b0f98968ed04fb1c37f33a3fbad43c611fa37d2c01681758664f5aa81
dv   div_Thaa  test/000_00000.parquet   302f0d4f9e8472c3eaa5fab9042df503bd6a149569268f821aaf6c79fd32e14d
as   asm_Beng  train/000_00000.parquet  66c64bd424c0084aee17ec4936a21d6e0b307b068f2fd29bf8acd60204a4891b
as   asm_Beng  test/000_00000.parquet   fe65d2e25063872f4593f01af4cb0e0e69fc50e5af9714bdefc3701a100dfcc3
trp  trp_Latn  train/000_00000.parquet  3becfceb5073d2991ba09f7f69d6d76a045bc154d10b5b058aca4bb0b44ace9c
trp  trp_Latn  test/000_00000.parquet   18b9059b96e18148350efbb260dc6120c72ca251dc317fe51b543100bef495ea
lus  lus_Latn  train/000_00000.parquet  ae86b93ee0ce7565f66dc07a63ea4111530d3088ba9fac91cdf195885e225da3
lus  lus_Latn  test/000_00000.parquet   82931cb2bc0affbe7402e3d92b616485a30cab1f29a226d4a17490e6a4090444
kha  kha_Latn  train/000_00000.parquet  d2293b5ec62d94d3b1019aab7a3ce03f01801d7c7a8ac1a1a3ce3315d4c44f66
kha  kha_Latn  test/000_00000.parquet   c2e85f6aa8725b703a40b395668198eec7d3036c2e4013b8c116152381eb894b
ckb  ckb_Arab  train/000_00000.parquet  3ab3414974031e9e561b61af2c356185263dea30929a8044c7367f9eb9c6c9b2
ckb  ckb_Arab  test/000_00000.parquet   8a40340a3b4684f6a2a6d88ca358d329cc994fd72537e5db03f4e7cf2aad8605
pcm  pcm_Latn  train/000_00000.parquet  5711b000d4ebd02cf25016bae966411be0a9d908484fcb825fa04493d40ddbb6
pcm  pcm_Latn  test/000_00000.parquet   143c3b204d839b630d0c78c8ae199d8704601e7a1b31e7da80fd10e12e1bb039
mww  mww_Latn  train/000_00000.parquet  9fb86f543904d6aac25ce8579f55fdfbace553db0f29832e102f37a43910c5d5
mww  mww_Latn  test/000_00000.parquet   0a0d4ffce4cfe9d8160c2badafbb5c5eb8dead634ae5028c8921a6011578d798
cnh  cnh_Latn  train/000_00000.parquet  60e1303e9af94acfd5e4cd51fab994226d3649a46092b1540d83d9534cd6c011
cnh  cnh_Latn  test/000_00000.parquet   152207cc49a19742b83a735bcf9993ebbe242e613b71eba8b5f9a437e592fcfe
kac  kac_Latn  train/000_00000.parquet  874231df9b50402d9c3d535479418039028119264debe93297ebe2ece111c87d
kac  kac_Latn  test/000_00000.parquet   11cb0e7638cb0978d28379810c582e9dd854b5d0cb8503b515d6432544a82093
hil  hil_Latn  train/000_00000.parquet  bd9f6fb7f06ae8b1cb0fc62a4a01221ec9b78178ddccbd2ce77acd5a7452a87c
hil  hil_Latn  test/000_00000.parquet   190a82ed8823aa989d24ebe26bfa6744f09179a6efe947439d1003464f6dbc29
ilo  ilo_Latn  train/000_00000.parquet  f970a137ca919e02f830646a6157d9ba4b6b39c33791b847413fbc8053ac22e1
ilo  ilo_Latn  test/000_00000.parquet   64f021f41df577cf0ec9841e2c9303e1565a6bcf4a161d61e0c25469885403e4
bcl  bcl_Latn  train/000_00000.parquet  109069b0235c2248fbb89ff35e46a9f647f0c4f0ef4a09a29c53eadefb02b616
bcl  bcl_Latn  test/000_00000.parquet   dffdbdfb8ac4b21d85fa8d882bd5cec0ce47e6f56df3fab2e4296901a69828d1
pag  pag_Latn  train/000_00000.parquet  8a22a7845a02055643982a602162e5663e7e2ead8bf5395cd55ca72e284f0008
pag  pag_Latn  test/000_00000.parquet   cab847bb05556a976158403339c44a3e6218e63f9de04d747eeb5ee76629c3d7
pam  pam_Latn  train/000_00000.parquet  6f867746f2a32391bd20741abf3d18f7dbd50bb0746f16ca1a989b9911cf7677
pam  pam_Latn  test/000_00000.parquet   56a514e50d29889dbf6785cae2564d2e05a90b4e77a2582c384665622788f1d0
krj  krj_Latn  train/000_00000.parquet  f79a17ed50b51e8607142c609541ee8d5292a04a7c3ae4e185a3f090f1e49379
krj  krj_Latn  test/000_00000.parquet   e55109608d7d35f34400515f51cad9fff991fb50a11b1d605ad8da4b344ed268
iba  iba_Latn  train/000_00000.parquet  a3519066a4a80b9d9b7192f1fd2885487590365783635054b89ab1ae0ab861d6
iba  iba_Latn  test/000_00000.parquet   eef6b2d90148565d21b325ae4956c397c190609c2e93bf9af7b9724e26d0b4fd
bbc  bbc_Latn  train/000_00000.parquet  db895b72716b129e5fa32a419210388f7c784868b7928132704b13f471f11476
bbc  bbc_Latn  test/000_00000.parquet   69d6a3c08f4949394b1b5323a683ecbb3c734d06bea566fa290a574520779434
ace  ace_Latn  train/000_00000.parquet  02532c90394b33abdfab9544a5348bcc1a7b6fbf3ea9c4041a1f4f873b953b88
ace  ace_Latn  test/000_00000.parquet   c63392eb5a048003ee3e6faa6cbd76b8715f9ddec245b70019823eb77a9f9f27
btx  btx_Latn  train/000_00000.parquet  52fd145df6f623aaef6cf1527dd1a66b74cc5c283dd73537d89e1db0d478ead1
btx  btx_Latn  test/000_00000.parquet   9c10f7f842354d7f20c663e1b39500d9723ca43b4d877900cb950aed58525475
bts  bts_Latn  train/000_00000.parquet  a27e10a2012c751037d7d09251311de6315c2c54b02566056febdc57c3ab7659
bts  bts_Latn  test/000_00000.parquet   95db1795e5289c79c1bc903ccd07e51e92a41894c4bd2cf3dc8bdfaa2c158e29
bjn  bjn_Latn  train/000_00000.parquet  4c4c4719eb8170c84b159927c0569a079ec9c8b01fc7e0afa86e1d19fc5e262d
bjn  bjn_Latn  test/000_00000.parquet   60f844a68a1d1648f7b857483e6957200028926ac2ea48c3c47f15aa053fdb6c
dtp  dtp_Latn  train/000_00000.parquet  ce627a07f39876a8eb996266f25fabd3b0bbb58f72b73f077bb6dacfac4541c2
dtp  dtp_Latn  test/000_00000.parquet   68ede3dae4c1e69470bebcee560b1ecfc0712e3d4312663f462e7eedfa41f65a
ban  ban_Latn  train/000_00000.parquet  2e5419955cdccc06d7d49d2b5bd1defcfb26d2fb7f391292b31b2e3adbf57451
ban  ban_Latn  test/000_00000.parquet   b0f1ee4a8aee5ee4be6fdd435ea6fa7e3171cc97b05877d8c60a4112332ad548
tet  tet_Latn  train/000_00000.parquet  505ad3d4c161b06cf725d7b6f3e81270e85f60128b3657de731a73318d384016
tet  tet_Latn  test/000_00000.parquet   0bdbc2a74f84071381fa936010c2aa4beb38d5faacf8f423f435403ad5ae436c
ium  ium_Latn  train/000_00000.parquet  58fbd1f9f0bd7511da7bd7ec50dc013fbe0eddb0f95ea77ecbb0d885e64b5838
ium  ium_Latn  test/000_00000.parquet   8ae6913a15c5a3b3a7e2e601af3225ab9cbbb3c4506fde406ac7acd077c028d7
nn   nno_Latn  train/000_00000.parquet  4c8db1523a7f7ef1c3ae9f032001a99e68fe2242406d3f77610fc9f1c6893d45
nn   nno_Latn  test/000_00000.parquet   c2624e31ae0b6dbee239e69da796a23748d2685fb1281030d9402ab927363d0e
ast  ast_Latn  train/000_00000.parquet  8ad0472c575ef23d47ede2994cfec5437f7782013a0e4a19bda3ac78456223fe
ast  ast_Latn  test/000_00000.parquet   f0c9cc5ac8313addd0ab2ea889243a38bf81c8ac249629a9b72a6b5a11c991d0
an   arg_Latn  train/000_00000.parquet  70b37a437f4165c574efdd1a9d2a90c7c7ad2b4d019e3b474199fe8ab464a242
an   arg_Latn  test/000_00000.parquet   3a2f7a5f7e0e7b0bb9f75b67bc42237fb71f47bb9c981d91b24021f9f4e70913
lad  lad_Latn  train/000_00000.parquet  a9b12be8c3a6fe79a689a2cc0b0a03b026cc632ec35d3c179195428c9e9c70ef
lad  lad_Latn  test/000_00000.parquet   0ab31e805657a68b8e357c574d1b14758e836f7601bd399c2a95ae484b3059e0
gsw  gsw_Latn  train/000_00000.parquet  5c3a9671fc9ab923a40855f84b328f0a015703b7e3200873a0d758ebfd24aa52
gsw  gsw_Latn  test/000_00000.parquet   7b720ec7e806bca9ebe5cbf4beb6034dff41b528a799ffe362eb620cafc492ca
bar  bar_Latn  train/000_00000.parquet  d2597a1fb531530df1ef11cfb563b583358b389ea8853e6ef8641a3902105a4b
bar  bar_Latn  test/000_00000.parquet   7b6e491a8a715bc0ea88fd12b1894eda3f0dc94b538077e5a11d740e3fbc40d2
nds  nds_Latn  train/000_00000.parquet  eac2c22dad84354b34b9bdc6ca1c3440d009f3416738d743f7f01274d22cb37e
nds  nds_Latn  test/000_00000.parquet   a85a673c3d67b45e481267e73f63f9e2ac671e475cc21c9ee611357d73cde3fb
pfl  pfl_Latn  train/000_00000.parquet  9587172ad7edc6477d286b5803a214e5f27e568eb7a7b64ddbd1ffd864da6173
pfl  pfl_Latn  test/000_00000.parquet   f15dd6ea96646bba2f5aeca47ce027ea2b87d559a21c86a8cfdb83c945da27b3
ksh  ksh_Latn  train/000_00000.parquet  ab67ff43fc508674beb4675884b4401e1587782eea7c0e967f5b149ef497a700
ksh  ksh_Latn  test/000_00000.parquet   900414db30de3d701e742b72380637a7925f924c5f340d821544decdeaf686c9
li   lim_Latn  train/000_00000.parquet  b80ca89323e0069518fca15c3619f31aaad8a9ccc8283b8140ec04f0bf658bfb
li   lim_Latn  test/000_00000.parquet   da20254bd6096ea90b205cd291400dc4ae58a115fe6345d5874333dcdd4fba1e
hsb  hsb_Latn  train/000_00000.parquet  37e46a5c5c0b944efc6e22953f4f4be403c737a79501fe88155b4228418e200e
hsb  hsb_Latn  test/000_00000.parquet   a53196705dbfe4628a188f396c8da51380cf7c8f001d41dde0d12d79f0fc9b07
dsb  dsb_Latn  train/000_00000.parquet  9caf20a27f42b054c72956e75f69df0ed9d4a138a34d68ffb969876fbebe312b
dsb  dsb_Latn  test/000_00000.parquet   495bfb484b9571d89f2aea3e44bce73cfa242fac1a008b6b2d633d7d8200df80
nap  nap_Latn  train/000_00000.parquet  0c645c77001b713b05a9d5d4b19a5e7da15217d9503fe16bcb59e4d55c2e4989
nap  nap_Latn  test/000_00000.parquet   86f139a6fa3c3f9c51e8fbecff0520ea01fedc6937b0ddb47aaf60c37f133455
scn  scn_Latn  train/000_00000.parquet  2f7e2bb279629930f8c7df1b18b540578a62732e5dbb99ea8ed0d45469186956
scn  scn_Latn  test/000_00000.parquet   861dcd9299d6e201436dbc606a4a97311a53f15e4c45a4d71fa3ecdea9b82fd4
sc   srd_Latn  train/000_00000.parquet  cf5d807e21dbf0c2f1d10b376c89c3608b5d326329151010ffcfb7aeb58124df
sc   srd_Latn  test/000_00000.parquet   88f7f6a68e7e83b000a0137103ed07b76be52ff6881e1c71bc54937f8f4dcb95
fur  fur_Latn  train/000_00000.parquet  4df549bc1beb34f42fce12b1894e13b33fa328a93b7c4bc862ddb4f3a7964d4c
fur  fur_Latn  test/000_00000.parquet   6812d3a90336ecce9f38c3c8c2e22fa4667ade1633ea9a92183eb5322acc314b
pms  pms_Latn  train/000_00000.parquet  dfbcbbdde1c1fba06f41a16309373e69bff273c9e2af97149c2a66a4b946d5db
pms  pms_Latn  test/000_00000.parquet   492a5650b1091272bb46d413b1e0c579dad9ab2381ecb0766824f56f27a8d28e
lij  lij_Latn  train/000_00000.parquet  73a36961d674c73818f11f5e82c6865fdaade9cc0079f2e477c5cf364fc47ad0
lij  lij_Latn  test/000_00000.parquet   791fe7b0c33743f7305defbf229fdfeccac74063b6b54c33ff6a2f30aa54895b
wa   wln_Latn  train/000_00000.parquet  ea5ee3668b0fb937887cb0980d7694d620d145d451c380c292dcaea7b8c1b77f
wa   wln_Latn  test/000_00000.parquet   cd9d0ea38e12d8ff44dd31e54cb4e59a445e2da75aa347a6cb8b4efd03add764
ltg  ltg_Latn  train/000_00000.parquet  efbea58a7d562549d20f05a4df6c352941bb46b83727a91069860da972fbff0e
ltg  ltg_Latn  test/000_00000.parquet   89f117a413b922f1f6fef3dc1931a90d42883809f617b169859eebb6d1ebeb07
vro  vro_Latn  train/000_00000.parquet  b66600a3436e7d186da6c546a2936187d4e8dd98f65f0023bf449326394bbfd9
vro  vro_Latn  test/000_00000.parquet   4b224fceb7841bb0e9bdcd5082718bb01f84641c3ebd1b1a5b0dd63a3a43ecf1
csb  csb_Latn  train/000_00000.parquet  f211813a6c295d5a54e0e4a7a18a67dff5abc8280f12eb9879340476ab128d05
csb  csb_Latn  test/000_00000.parquet   b36e4e31d61860caf60517225a3d120af85c0cee6099d98a24aafd00f28e1419
szl  szl_Latn  train/000_00000.parquet  fc628ffb59dea4d679afb3aee254c0dbe158f8bb47c8f74bddbeec7227cd314d
szl  szl_Latn  test/000_00000.parquet   0615fde9757d9a1a27873e8beb04ae9922cd6161d55c9c7dd4088799bb0241fe
crh  crh_Latn  train/000_00000.parquet  69ecf985016655497119d4a4343813e01a0f2ca9d1326562be8f5f3208d427d6
crh  crh_Latn  test/000_00000.parquet   cd0d797b13a9f2eefe4f87c6e906f51d53a2424623fd3d477e89470cb6522bf7
sco  sco_Latn  train/000_00000.parquet  c026625d2c6327c0763c5bed40b7a988df194a3543592e3bb292948b63f19522
sco  sco_Latn  test/000_00000.parquet   a927e9a5fe7402b24b033246fd5e78db08feb42b97f7e04fb18e246736b20fb1
rue  rue_Cyrl  train/000_00000.parquet  65579203ebf8606a5f1b2d0112fcb5c2425efe76eb952903a7ade442b67a9093
rue  rue_Cyrl  test/000_00000.parquet   7dd795f4a260b89691ade2497de57f1785ea155f9b108a003a9edcda13525a50
sah  sah_Cyrl  train/000_00000.parquet  9ff9992c37de82d05a58a029d42fe89c7d724ee356eda3f9815c9cc389d18a9c
sah  sah_Cyrl  test/000_00000.parquet   0328ba01f46f56a1e85ce6015bad4a90730a510cb0e9ddc6b8a9da8dc11ffbbc
cv   chv_Cyrl  train/000_00000.parquet  c044bfd8ddff94a34160b34ce84b8196624eca5582a7607f91b9032273b85b32
cv   chv_Cyrl  test/000_00000.parquet   4d0544097dd372cfb99fc7bb33909c5e2f0a7beb9d058975d12ed389ac744a0d
sme  sme_Latn  train/000_00000.parquet  d635b159ba6900cac36f9def77e05855117056736bc25a291005a8df8c1e7373
sme  sme_Latn  test/000_00000.parquet   44729b46a4bb8d3e47c5ff4d3cbec0f1188f31e1a832efbedf79832f7663fd42
mhr  mhr_Cyrl  train/000_00000.parquet  7669442c6fbd90094cfa6c3d766e60f346b7621a18e16db63eb3a9ac9a0728e7
mhr  mhr_Cyrl  test/000_00000.parquet   7f43fcf6496c041032076a813f03d1866023dadc01038d70b59e6fb33fa1de64
mrj  mrj_Cyrl  train/000_00000.parquet  fd56bacbf1cab7dff2bcbf6dfb9dab02a5c97263fc41757cde1da7cfe1853a27
mrj  mrj_Cyrl  test/000_00000.parquet   b981cb26bd5884101d730ba96ec8214b64f29f7d075cbb5a4161234dc3862005
udm  udm_Cyrl  train/000_00000.parquet  991b6377876248637fef028eea08654004c4b45cf31c8d4aa8042105a5e380d4
udm  udm_Cyrl  test/000_00000.parquet   742b2d6c263c33d9c01662c2e336650851620578d417ac66b4d8214b87ee932e
tyv  tyv_Cyrl  train/000_00000.parquet  b2ec4f33f2dc3b938a2bd64a2c2450b3959b15aabc6b6debb1701ec7be3c93d0
tyv  tyv_Cyrl  test/000_00000.parquet   8f1f6dfe1a28c593ccd411565100a4d8192f29b09ad280091c24dd80389d51d8
bxr  bxr_Cyrl  train/000_00000.parquet  c9743da8b78fdf10295612468dec2836e86edd5e758f42680bf2dd6a83c75770
bxr  bxr_Cyrl  test/000_00000.parquet   bebbfd3b193d5b8913a22548d31858168d5f33b000210a0053dbfdf07fe9dc2e
kpv  kpv_Cyrl  train/000_00000.parquet  4dc8f0a7e4f8f5b37fb3ab750cf9f58783c8d15b26a277fdd44fb867757626e8
kpv  kpv_Cyrl  test/000_00000.parquet   3f5805ad54aeeecb5cc9c37dfb6f1005f2a2954cda6b39946016baeda63274b3
koi  koi_Cyrl  train/000_00000.parquet  92b7e1bbb8fcc0cc42774a1659e4ad32c1e8e7e957739f3925d270cc0ba63fe2
koi  koi_Cyrl  test/000_00000.parquet   9fb7a9e2c5c984db6940c9b3a6146c582a3ead29cd9d0b60863bba64dc9fda5f
myv  myv_Cyrl  train/000_00000.parquet  32c72bc514f2c4a50c6da803da5eaa11912e48c484723e63c8f3a5c9a44fe707
myv  myv_Cyrl  test/000_00000.parquet   3e8f85020055032dfd20cd4885c46ece48018fc34f7e71bd68c36cb9f74eecc2
mdf  mdf_Cyrl  train/000_00000.parquet  27f1cc0c79063250adb6e83152cd8c3db7299d05b017031d1b15167ffa2bd30e
mdf  mdf_Cyrl  test/000_00000.parquet   2b016d593cb759463d74dc95e26945dc49d245614e9e28d5ff1027da8f4e6185
xal  xal_Cyrl  train/000_00000.parquet  f86f00f3195eec44445e21c4927235c5002311047654aa9f3a4aab9076fb761f
xal  xal_Cyrl  test/000_00000.parquet   ca5cde7ebc6381239a444f08c74938bfcdb954863ac04253cae3f85da4dac083
krl  krl_Latn  train/000_00000.parquet  f984f2ee4a113ae8a4660a4972236ce5d72d730b040959023d648054b15bd7b4
krl  krl_Latn  test/000_00000.parquet   9d368f7c1bc1281e650c4dded2ac471ddceac15bdce3861606ec2f5c0a8598fa
olo  olo_Latn  train/000_00000.parquet  d72040a97d8565fcfe80b9e6ea8f79b6f91a892b31cddc15ee1c50cfb568afcc
olo  olo_Latn  test/000_00000.parquet   8ed94fa64ae9131e3ae9a448a546513a9135a0ed3e97ccc37d9812fc293bf199
vep  vep_Latn  train/000_00000.parquet  7c085e92de69153ae7ca8787afe577edc87e27ba7099a7f4b1bd0c98b589c6eb
vep  vep_Latn  test/000_00000.parquet   1a7f21713cf363428ec1453534ef641dbc9e7c501d370a28135314a69a7cbcf1
kl   kal_Latn  train/000_00000.parquet  4727c82026b59732fac66936cbfd78a11c98315cac9d4c9d3423d3e2cd453195
kl   kal_Latn  test/000_00000.parquet   e5ae8bd3092336b9b7ff6afeae772fdf8ac5c9b1e281b39a3492f12e6b30117b
om   gaz_Latn  train/000_00000.parquet  368ccd9fd0b3f3d4de433d970ab8b43fd49ce934e9edac77208cba268d9e12d3
om   gaz_Latn  test/000_00000.parquet   3f50999a364577981a9db77b37a647022deaf44de28d21ab6422722bae9c6c49
rm   roh_Latn  train/000_00000.parquet  43b1b5c12c3fbf67b21b8d85c4f6e4637ca4ba7c7730ccc6fa4ed8ce9de97b48
rm   roh_Latn  test/000_00000.parquet   f1230ee5583a1991c6436d758d79ac09e30c7a0fc0b40ef293c516f5e7748980
rn   run_Latn  train/000_00000.parquet  2efc4f7f5cf64bb3544778e2387cc7826a631d565745181d0568d75cdfcd49c4
rn   run_Latn  test/000_00000.parquet   b1de6010932f5222df67f1e92c50728010bda0b63d155e618aa8c6d2253b73b6
lg   lug_Latn  train/000_00000.parquet  41c5d7fc07628aca12f59739fa8e5e1d49e83d1e943ba6c21717fdf6b3953a8c
lg   lug_Latn  test/000_00000.parquet   920b8fad5f134e4f9f0ac5990017b0df4f2f17d24578d3d52ea2a2cee4238e4b
ln   lin_Latn  train/000_00000.parquet  eb44c52f33d2016cc2a1f1ff2f9a5c54e6e83a0c59b79ac5a5fb1f5c7b0bfccb
ln   lin_Latn  test/000_00000.parquet   d620d05dbb1bded5a4e35cab4f0ec8c810d1c3dbeb68570281bd3d0354443131
fj   fij_Latn  train/000_00000.parquet  d1e76119d02e32cb4a7298dad8599bb306390c3630f7a9a7b63f404e74542eb5
fj   fij_Latn  test/000_00000.parquet   5c235ac8bcf1302c8065ff36ed764ac437db0296a7d603394b830e1936638f22
ee   ewe_Latn  train/000_00000.parquet  a4df0bcdc977d5428b7c5422954f38aba69a3ef42463e773deaf2353164f1dd9
ee   ewe_Latn  test/000_00000.parquet   8329df76ca71ffc2861ef37e329c3467769e926976b17d95713a7e2cb3524e56
ak   twi_Latn  train/000_00000.parquet  f8d5e04b8395d8a961e2f21e4c7f2accb4923023eefe784d9be934b594feed73
ak   twi_Latn  test/000_00000.parquet   1c9f2696a3130ab3eb74c257ec0a2e56bfb889b55395a1cb3ae05eb20662955c
ts   tso_Latn  train/000_00000.parquet  98e8dde6d5e0badd2bce3298f427e54d0aa5de28a0f8a6054b5613f45b13976c
ts   tso_Latn  test/000_00000.parquet   f9b2d1f2aeef3ade203493b173a8b1171b8edd448879f2c7f366ab5d8e2c9347
nso  nso_Latn  train/000_00000.parquet  980830e45bad910dc1a51e00906d44a505d24bebadc69e4be631c432ca0b1f02
nso  nso_Latn  test/000_00000.parquet   59dc5d95f99ce26fe636e89783415aa3f18eb304b224eb5ae59e0106aacbc3bd
tn   tsn_Latn  train/000_00000.parquet  98946d5873c51c682592d482837f77356e75912089b0abf7539b0be36698783a
tn   tsn_Latn  test/000_00000.parquet   9400a8bcf84af97cbc34d9bbcb3cf745451a9c245a9b79980cce8c2602a69940
to   ton_Latn  train/000_00000.parquet  94c0b5bc0059ea283ff47ee8a32cba9a60a89fc99a90abf41b7f394f2306cdba
to   ton_Latn  test/000_00000.parquet   18c94e03173fd253d9310b30f558c1f33998b801c235ec94a6662c3c5d766eaf
wo   wol_Latn  train/000_00000.parquet  b8333d9a0f44d80d7b6dd1e8a8957a100e1ad219928014b7c23ea9e0cd3ba6d0
wo   wol_Latn  test/000_00000.parquet   dcd435775b7d0c8cdbfa7bb8fcb1ff4c2455ec0d1064cb4f6828694cda0152bf
ty   tah_Latn  train/000_00000.parquet  ae956ce6e6b773b38291bd09da357d2cac20eacca929bc0a7446dbc4aa4351c2
ty   tah_Latn  test/000_00000.parquet   591487cf68aeb4dc9a9c06f789e6da87be56aeb2bea3592f688bdb260c22ca98
ve   ven_Latn  train/000_00000.parquet  eac517f6edd46fc4289fa09e28618d4d0beeda5b8534d696843fca4260fee471
ve   ven_Latn  test/000_00000.parquet   07b19dcfea30d1bc40bd654d683d6ed7f7cfd1fda80a589fd499f4494c047c20
nr   nbl_Latn  train/000_00000.parquet  09a58adad2324e07f7db3fa7b5c32d13600551ccedf92ac5ddd7204a6b54e004
nr   nbl_Latn  test/000_00000.parquet   a90165b3bb9456e29df9b30d1537561627135a789e00d4f094b8571ca1ba2460
ay   ayr_Latn  train/000_00000.parquet  a3f483b8182435d6021ba8736bb4c2c70e4ae838cdfedadc0aa6403fc6e1b3b4
ay   ayr_Latn  test/000_00000.parquet   b47f92a02be589d0509221f5afc5cf39d4d15f78978e0cf0618f509b0611334f
ss   ssw_Latn  train/000_00000.parquet  5423474aa7b736e4b82032cad2dcc5dbc600c49010da471184a5f14812225dba
ss   ssw_Latn  test/000_00000.parquet   e67f8e0375aebfea1e6f4cc18ce187cebe464f5c978e2aead67f21a8875e4f44
qu   quy_Latn  train/000_00000.parquet  f97657e794f6aac0e8676fe2c7349b2c43fce4b119c669559bbe86721aa993e8
qu   quy_Latn  test/000_00000.parquet   b6069bc9a67dfd61d67398651aa60e922f5eb7a783e73e53020c8602b37fc05b
bm   bam_Latn  train/000_00000.parquet  4e3add228d973a67eb8f15930ad84e32548cf85a55546cf2547b0876d4b53d6c
bm   bam_Latn  test/000_00000.parquet   c1b7333c5d7419dafbbcbe30328fa906705b9de0dae24b866d890b97c7044c41
iu   ike_Cans  train/000_00000.parquet  f242c9da7a006a77e3c20b67f2eabdc2131e60050f93e19a2dd5d984baf4a4af
iu   ike_Cans  test/000_00000.parquet   fcc17d15da1e130c9adfa97653ed76f4b4aeed1b167c0f379929ec81094cdf98
"

HERE=$(cd "$(dirname "$0")" && pwd)
OUT=${OUT:-${TMPDIR:-/tmp}/ki-osk-sources/fineweb}
EVAL_OUT=${EVAL_OUT:-$OUT/eval}
PYTHON=${PYTHON:-python3}
langs=${*:-$(echo "$FILES" | awk 'NF {print $1}' | uniq)}

"$PYTHON" -c 'import pyarrow' 2>/dev/null ||
	{ echo "fetch-fineweb: $PYTHON has no pyarrow (pip install pyarrow, or set PYTHON)" >&2; exit 2; }
mkdir -p "$OUT/raw" "$EVAL_OUT"
for lang in $langs; do
	rows=$(echo "$FILES" | awk -v l="$lang" '$1 == l')
	[ -n "$rows" ] || { echo "fetch-fineweb: no files for $lang" >&2; exit 2; }
	train=()
	test=()
	while read -r _ config file sha; do
		local_file="$OUT/raw/$config/$file"
		mkdir -p "$(dirname "$local_file")"
		if [ ! -f "$local_file" ] || [ "$(sha256sum "$local_file" | cut -d' ' -f1)" != "$sha" ]; then
			curl -fsSL --retry 5 --retry-all-errors -o "$local_file.part" "$BASE/$config/$file"
			got=$(sha256sum "$local_file.part" | cut -d' ' -f1)
			if [ "$got" != "$sha" ]; then
				rm -f "$local_file.part"
				echo "fetch-fineweb: $config/$file's sha256 is $got, not $sha" >&2
				exit 1
			fi
			mv -f "$local_file.part" "$local_file"
		fi
		case "$file" in
			train/*) train+=("$local_file") ;;
			test/*) test+=("$local_file") ;;
		esac
	done <<<"$rows"
	"$PYTHON" -I "$HERE/fineweb-text.py" --lang "$lang" "$OUT/$lang.txt" "$EVAL_OUT/$lang.txt" \
		--train "${train[@]}" --test "${test[@]}"
	if [ "${KEEP_RAW:-1}" = 0 ]; then
		rm -f -- "${train[@]}" "${test[@]}"
	fi
done
