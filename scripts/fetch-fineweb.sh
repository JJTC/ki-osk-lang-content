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
# after this); and
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
