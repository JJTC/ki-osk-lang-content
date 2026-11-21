#!/usr/bin/env python3
"""storyweaver-text.py — a language's stories from StoryWeaver as
sentences, one a line: what its model learns from, for languages that
have no other open text.

StoryWeaver (storyweaver.org.in, Pratham Books) publishes picture books
for children in hundreds of languages, written, translated and
illustrated by its community and by publishers, each under CC BY 4.0
(its terms of use: the Read Along videos alone are CC BY-NC-ND). Its
reader shows a story's pages to a signed-in account, and that is how
they are read here: the stories a language's model learns from are named
in scripts/storyweaver-stories.tsv (its tag, the story's id and slug, and
the sha256 of the text read from its pages), and each is fetched once
into RAW/<id>.json as the reader serves it.

    storyweaver-text.py fetch --stories TSV --raw RAW [LANG...]
    storyweaver-text.py text  --stories TSV --raw RAW --lang L OUT EVAL_OUT
    storyweaver-text.py pin   --stories TSV --raw RAW --lang L --name NAME...

`fetch` reads the stories RAW does not have yet, signed in as
$STORYWEAVER_EMAIL with $STORYWEAVER_PASSWORD (never written anywhere),
one every five seconds and slower whenever the site asks; with no
account given it reads none and exits 3, and a story the reader does not
serve is an error. `text` writes a language's corpus and held-out text
from them. `pin` (a maintainer's: it signs in too) lists the stories
StoryWeaver files under the language's names, reads them all again, and
writes the language's rows of TSV anew: each story whose attribution
says CC BY, with its text's sha256.

A story is read from its story pages (not its covers, nor the
attribution pages after them): their text, a line a paragraph or line
break. A story whose attribution names any licence but CC BY is an
error, and so is one whose text's sha256 is not the one pinned
(StoryWeaver's editors changed it: `pin` again, and look). A page is
left out where it writes English's or (--rival) another language's own
function words, two or more and 3% of its words or more, or where fewer
than nine in ten of its letters are in its language's script (--script);
and a story most of whose pages are left out so is left out whole (a
Spanish story filed under Nahuatl). Where a book is bilingual (--bold
NAME: Odia-Kui, Hindi-Sadri), its stories' bold text alone is read, the
convention of those books (the language in bold beneath the other). A
page's lines are joined where one ends no sentence (the reader breaks a
picture book's lines where its page needs), and its sentences end at
full stops, dandas (an Indic story's | read as one) and marks of
question and exclamation; a sentence set in capitals is read in lower
case, its first letter a capital; a sentence of one word is left out,
and each is kept once. Every 10th story is held out: its sentences the
corpus does not have are what eval-predict measures the model on.
"""

import argparse
import hashlib
import html
import html.parser
import http.cookiejar
import json
import os
import re
import sys
import time
import unicodedata
import urllib.error
import urllib.parse
import urllib.request

SITE = "https://storyweaver.org.in"
AGENT = "Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0"
# fetch's exit status when stories are to be read and no account given.
NO_ACCOUNT = 3
# Seconds between two requests, at first and at most: the site answers
# 429 to a reader much faster than one every five seconds.
PACE, SLOWEST = 5.0, 20.0


def words(text):
    """`text`'s words: runs of letters and marks (an Indic script's vowel
    signs and viramas among them) and joiners, with the apostrophes
    inside them."""
    found, cur = [], []
    for c in text:
        if unicodedata.category(c)[0] in "LM" or c in "\u200c\u200d" or (cur and c in "'’"):
            cur.append(c)
        elif cur:
            found.append("".join(cur).rstrip("'’"))
            cur = []
    if cur:
        found.append("".join(cur).rstrip("'’"))
    return [w for w in found if w]


# A sentence ends at a full stop, a danda or a mark of question or
# exclamation, with the quotes that close after it, and a space; the
# Indic stories often type their danda |.
END = re.compile(r"([.!?।॥|…]+[\"'”’»)\]]*)\s+")
# A line that does not end so goes on in the next: the reader breaks a
# picture book's lines where its page needs, mid-sentence or not.
GOES_ON = re.compile(r"[^.!?।॥|…\"'”’»)\]\s][\"'”’»)\]]*\s*$|^$")
# The scripts whose danda a | stands for.
DANDA = ("DEVANAGARI", "ORIYA", "BENGALI", "GUJARATI")
DANDA_GLUED = re.compile(r"([।॥|]+)(?=[^\s।॥|\"'”’»)\]])")
# An apostrophe inside a word written ’: the ' its keyboard types.
INSIDE = re.compile(r"(?<=[^\W\d_])’(?=[^\W\d_])")
# Zero-width characters the Devanagari stories scatter (a space, joiners
# between letters no font needs them for, a byte-order mark).
ZERO_WIDTH = dict.fromkeys(map(ord, "\u200b\u200d\ufeff"))

RIVALS = {
    "en": set(
        "the and of to was were are with that this they his her said what there from have had not but"
        " for you my your".split()
    ),
    "es": set("el la los las del que con una por para pero como muy cuando dijo está".split()),
}


class Page(html.parser.HTMLParser):
    """A story page's paragraphs, from its content box: each a list of
    (text, bold) runs."""

    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.depth = 0  # inside the content box: its depth in divs
        self.bold = 0
        self.lines = [[]]

    def handle_starttag(self, tag, attrs):
        if tag == "div":
            if self.depth:
                self.depth += 1
            elif "content" in (dict(attrs).get("class") or "").split():
                self.depth = 1
        elif not self.depth:
            return
        elif tag in ("b", "strong"):
            self.bold += 1
        elif tag in ("br", "p"):
            self.lines.append([])

    def handle_endtag(self, tag):
        if not self.depth:
            return
        if tag == "div":
            self.depth -= 1
        elif tag in ("b", "strong"):
            self.bold = max(0, self.bold - 1)
        elif tag == "p":
            self.lines.append([])

    def handle_data(self, data):
        if self.depth:
            self.lines[-1].append((data, self.bold > 0))


def clean(text):
    text = unicodedata.normalize("NFC", text).translate(ZERO_WIDTH)
    return " ".join(INSIDE.sub("'", text).split())


def story_pages(reader):
    """A story's pages as the reader serves them: each a list of lines,
    each a list of (text, bold) runs."""
    pages = []
    for page in reader["data"]["pages"]:
        if page.get("pageType") != "StoryPage":
            continue
        p = Page()
        p.feed(page.get("html") or "")
        p.close()
        lines = [[(clean(t), b) for t, b in line if clean(t)] for line in p.lines]
        pages.append([line for line in lines if line])
    return pages


def text_of(pages):
    """What the pinned sha256 is of: the pages' lines, a page a block."""
    return "\n\n".join("\n".join(" ".join(t for t, _ in line) for line in page) for page in pages)


def licences(reader):
    """The licences the story's attribution pages name for its text."""
    found = set()
    for page in reader["data"]["pages"]:
        if page.get("pageType") != "BackInnerCoverPage":
            continue
        t = " ".join(html.unescape(re.sub(r"<[^>]+>", " ", page.get("html") or "")).split())
        if "Story Attribution" not in t:
            continue
        t = t.split("Images Attributions", 1)[0]
        found |= set(re.findall(r"Released under (.*?) licen[cs]e", t))
        if re.search(r"[Aa]ll rights reserved", t):
            found.add("All rights reserved")
    return found


def open_by_cc_by(found):
    return bool(found) and all(re.fullmatch(r"CC BY \d\.\d", x) for x in found)


def read_rows(path):
    rows = []
    with open(path, encoding="utf-8") as f:
        for line in f:
            if line.startswith("#") or not line.strip():
                continue
            lang, sid, slug, sha = line.rstrip("\n").split("\t")
            rows.append((lang, int(sid), slug, sha))
    return rows


class Site:
    """The signed-in reader, paced."""

    def __init__(self):
        email = os.environ.get("STORYWEAVER_EMAIL")
        password = os.environ.get("STORYWEAVER_PASSWORD")
        if not (email and password):
            sys.exit("storyweaver-text: StoryWeaver's reader needs an account: set STORYWEAVER_EMAIL and STORYWEAVER_PASSWORD")
        self.opener = urllib.request.build_opener(urllib.request.HTTPCookieProcessor(http.cookiejar.CookieJar()))
        self.pace = PACE
        self.last = 0.0
        body = urllib.parse.urlencode({"user[email]": email, "user[password]": password}).encode()
        reply = self.get("/node/api/v1/users/sign_in", body)
        if not reply.get("user"):
            sys.exit("storyweaver-text: StoryWeaver did not sign the account in")

    def get(self, path, body=None):
        for attempt in range(5):
            time.sleep(max(0.0, self.last + self.pace - time.monotonic()))
            self.last = time.monotonic()
            req = urllib.request.Request(SITE + path, data=body, headers={"User-Agent": AGENT, "locale": "en", "Accept": "application/json"})
            try:
                with self.opener.open(req, timeout=60) as r:
                    return json.loads(r.read())
            except urllib.error.HTTPError as e:
                if e.code not in (429, 500, 502, 503, 504):
                    raise SystemExit(f"storyweaver-text: {path}: HTTP {e.code} {e.read()[:200]!r}")
                if e.code == 429:
                    self.pace = min(self.pace * 1.5, SLOWEST)
            except (urllib.error.URLError, TimeoutError, ConnectionError) as e:
                print(f"storyweaver-text: {path}: {e}", file=sys.stderr)
            time.sleep(60 * 2**attempt)
        raise SystemExit(f"storyweaver-text: {path}: no answer")

    def reader(self, sid, slug, raw, fresh=False):
        """Story `sid` as the reader serves it, kept in `raw`: read again
        only when `fresh`."""
        path = os.path.join(raw, f"{sid}.json")
        if os.path.exists(path) and not fresh:
            with open(path, encoding="utf-8") as f:
                return json.load(f)
        reply = self.get(f"/api/v1/stories/{urllib.parse.quote(slug)}/read?ignore_count=true")
        if not (reply.get("ok") and "pages" in reply.get("data", {})):
            sys.exit(f"storyweaver-text: story {sid}: the reader serves no pages")
        with open(path + ".part", "w", encoding="utf-8") as f:
            json.dump(reply, f, ensure_ascii=False)
        os.replace(path + ".part", path)
        return reply


def fetch(opts):
    rows = [r for r in read_rows(opts.stories) if not opts.langs or r[0] in opts.langs]
    todo = [r for r in rows if not os.path.exists(os.path.join(opts.raw, f"{r[1]}.json"))]
    if not todo:
        return
    if not (os.environ.get("STORYWEAVER_EMAIL") and os.environ.get("STORYWEAVER_PASSWORD")):
        print(f"storyweaver-text: {len(todo)} stories not read yet, and no account to read them with (STORYWEAVER_EMAIL, STORYWEAVER_PASSWORD)", file=sys.stderr)
        sys.exit(NO_ACCOUNT)
    site = Site()
    for n, (lang, sid, slug, _) in enumerate(todo, 1):
        site.reader(sid, slug, opts.raw)
        if n % 50 == 0:
            print(f"storyweaver-text: {n} of {len(todo)} stories read", file=sys.stderr)


def pin(opts):
    site = Site()
    books = {}
    for name in opts.name:
        page = 1
        while True:
            q = urllib.parse.urlencode({"page": page, "per_page": 100, "languages[]": name})
            reply = site.get(f"/node/api/v1/books-search?{q}")
            for b in reply["data"]:
                if b.get("language") == name:
                    books[b["id"]] = b["slug"]
            if page >= reply["metadata"]["totalPages"]:
                break
            page += 1
    kept = []
    for sid in sorted(books):
        reader = site.reader(sid, books[sid], opts.raw, fresh=True)
        found = licences(reader)
        if not open_by_cc_by(found):
            print(f"storyweaver-text: {opts.lang} story {sid}: left out, its licence {sorted(found) or 'unnamed'}", file=sys.stderr)
            continue
        sha = hashlib.sha256(text_of(story_pages(reader)).encode()).hexdigest()
        kept.append(f"{opts.lang}\t{sid}\t{books[sid]}\t{sha}\n")
    with open(opts.stories, encoding="utf-8") as f:
        lines = [x for x in f if x.startswith("#") or x.split("\t", 1)[0] != opts.lang]
    head = [x for x in lines if x.startswith("#")]
    body = sorted([x for x in lines if not x.startswith("#")] + kept, key=lambda x: (x.split("\t")[0], int(x.split("\t")[1])))
    with open(opts.stories + ".part", "w", encoding="utf-8") as f:
        f.writelines(head + body)
    os.replace(opts.stories + ".part", opts.stories)
    print(f"{opts.stories}: {len(kept)} stories of {opts.lang} ({', '.join(opts.name)})")


def in_script(line, script):
    letters = [c for c in line if c.isalpha()]
    if not letters:
        return False
    hits = sum(unicodedata.name(c, "").startswith(script) for c in letters)
    return hits >= 0.9 * len(letters)


def foreign(page, rivals, script):
    found = [w.lower() for line in page for w in words(line)]
    hits = sum(w in rivals for w in found)
    if hits >= 2 and hits >= 0.03 * len(found):
        return True
    return bool(script) and not in_script(" ".join(page), script)


def unshouted(s):
    """`s` in lower case, its first letter a capital, where it is written
    in capitals (a story set in them): as a sentence is typed."""
    cased = [c for c in s if c.isupper() or c.islower()]
    if len(cased) < 4 or sum(c.isupper() for c in cased) < 0.9 * len(cased):
        return s
    s = s.lower()
    first = next((i for i, c in enumerate(s) if c.isalpha()), None)
    return s if first is None else s[:first] + s[first].upper() + s[first + 1:]


def sentences(lines, script=""):
    """A page's sentences: its lines joined where one goes on in the
    next, cut at their ends."""
    joined, buf = [], []
    for line in lines:
        buf.append(line)
        if not GOES_ON.search(line):
            joined.append(" ".join(buf))
            buf = []
    if buf:
        joined.append(" ".join(buf))
    for text in joined:
        if script in DANDA:
            # A danda typed with no space after it still ends its sentence.
            text = DANDA_GLUED.sub(r"\1 ", text)
        parts = END.split(text + " ")
        for i in range(0, len(parts) - 1, 2):
            s = unshouted(" ".join((parts[i] + parts[i + 1]).split()))
            if script in DANDA:
                s = s.replace("|", "।")
            if len(words(s)) >= 2:
                yield s
        tail = unshouted(" ".join(parts[-1].split()))
        if len(words(tail)) >= 2:
            yield tail.replace("|", "।") if script in DANDA else tail


def text(opts):
    rows = [r for r in read_rows(opts.stories) if r[0] == opts.lang]
    if not rows:
        sys.exit(f"storyweaver-text: no story of {opts.lang} in {opts.stories}")
    rivals = set().union(*(RIVALS[r] for r in ["en", *opts.rival]))
    seen, held = set(), {}
    corpus = left = gone = 0
    changed = []
    with open(opts.out, "w", encoding="utf-8") as out:
        for n, (_, sid, _, sha) in enumerate(sorted(rows, key=lambda r: r[1]), 1):
            with open(os.path.join(opts.raw, f"{sid}.json"), encoding="utf-8") as f:
                reader = json.load(f)
            pages = story_pages(reader)
            if hashlib.sha256(text_of(pages).encode()).hexdigest() != sha:
                changed.append(sid)
                continue
            if not open_by_cc_by(licences(reader)):
                sys.exit(f"storyweaver-text: story {sid}: its licence is no longer CC BY")
            texts = []
            bold = reader["data"].get("language") in opts.bold
            for page in pages:
                if bold:
                    lines = [" ".join(t for t, b in line if b) for line in page]
                else:
                    lines = [" ".join(t for t, _ in line) for line in page]
                lines = [x for x in (" ".join(x.split()) for x in lines) if x]
                if lines:
                    texts.append((lines, foreign(lines, rivals, opts.script)))
            # A story most of whose pages are another language's is that
            # language's (a Spanish story filed under Nahuatl): none of it.
            if sum(f for _, f in texts) * 2 > len(texts):
                gone += 1
                continue
            for lines, other in texts:
                if other:
                    left += 1
                    continue
                for s in sentences(lines, opts.script):
                    if n % opts.holdout == 0:
                        held.setdefault(s, None)
                    elif s not in seen:
                        seen.add(s)
                        out.write(s + "\n")
                        corpus += 1
    if changed:
        sys.exit(f"storyweaver-text: {opts.lang}: {len(changed)} stories' text is not as pinned (StoryWeaver's editors changed it; pin again): {' '.join(map(str, changed[:20]))}")
    kept = [s for s in held if s not in seen]
    with open(opts.eval_out, "w", encoding="utf-8") as ev:
        for s in kept:
            ev.write(s + "\n")
    if corpus == 0 or not kept:
        sys.exit(f"storyweaver-text: {opts.lang}: no text")
    print(f"{opts.out}: {corpus} sentences from {len(rows) - gone} stories ({gone} left out as another language's), {left} pages left out as another language's, {len(kept)} held out")


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    for name in ("fetch", "text", "pin"):
        p = sub.add_parser(name)
        p.add_argument("--stories", required=True, help="scripts/storyweaver-stories.tsv")
        p.add_argument("--raw", required=True, help="the readers' replies, a file a story")
    sub.choices["fetch"].add_argument("langs", nargs="*")
    t = sub.choices["text"]
    t.add_argument("--lang", required=True)
    t.add_argument("--rival", action="append", default=[], choices=sorted(RIVALS), help="another language whose pages to leave out (English's always)")
    t.add_argument("--script", default="", help="the Unicode script of its letters (DEVANAGARI, ORIYA, LATIN)")
    t.add_argument("--bold", action="append", default=[], metavar="NAME", help="a bilingual book's name at StoryWeaver (Odia-Kui): its stories' bold text alone, the language's")
    t.add_argument("--holdout", type=int, default=10, help="every K-th story held out")
    t.add_argument("out")
    t.add_argument("eval_out")
    p = sub.choices["pin"]
    p.add_argument("--lang", required=True)
    p.add_argument("--name", action="append", required=True, help="the language's name at StoryWeaver (repeat)")
    opts = ap.parse_args()
    os.makedirs(opts.raw, exist_ok=True)
    {"fetch": fetch, "text": text, "pin": pin}[opts.cmd](opts)


if __name__ == "__main__":
    main()
