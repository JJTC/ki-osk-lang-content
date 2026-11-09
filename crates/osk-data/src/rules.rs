//! Text rules: what typing a key does to the text before the caret beyond
//! typing it (the space a strip word left giving way to the punctuation
//! typed next, French spacing), and where autocorrect acts besides space.
//!
//! The rules are data, written in the style of Weir, the rule language of
//! the Harper grammar checker: each is a pattern over the end of the text
//! before the caret, the key just typed last, with what that becomes and
//! the rule's own tests, in one file. `rules/all.weir` holds the rules for
//! every language and `rules/<lang>.weir` a language's own, which come
//! first: for a key, the first rule whose pattern matches decides.
//!
//! # The format
//!
//! ```text
//! # A comment, on a line of its own.
//! expr closing [".", ",", "?"]
//!
//! rule SpaceGivesWay
//! let message "The space a strip word typed gives way to punctuation."
//! expr main (AUTOSPACE @closing)
//! let becomes "$2⎵"
//! test "word⎵," "word,⎵"
//! allows "word ,"
//! ```
//!
//! `expr name …` names a pattern for `@name`, in its file or, from a
//! language's file, in `all.weir`. A rule runs from its `rule` line to the
//! next; `expr main` is its pattern.
//!
//! A pattern matches tokens at the end of the text: a `WORD` (letters,
//! digits, marks and joiners), a `SPACE` (one space character, of any
//! width), the `AUTOSPACE` (the space a strip word or a glide typed after
//! itself, when it is the last character before the caret), and every
//! other character on its own. `"…"` matches its text (a word in any
//! case), as does a bare literal without spaces or `()[],!@"*`; `(a b)` is
//! a sequence, `[a, b]` either one, `!a` one token `a` does not match,
//! `*` any one token, `@name` a named pattern.
//!
//! `let becomes` is what the match turns into: its text, with `$1`, `$2`…
//! for what the main pattern's parts matched, and `⎵` at its end for an
//! automatic space after it; only what changes is retyped. A rule with
//! `let kind "Corrects"` instead marks where autocorrect acts on the word
//! before the key, as it does on space. In strings, `\u{202F}` is a
//! character by its code point, and `\"`, `\\` and `\$` are themselves.
//!
//! Tests: `test "typed" "result"` types the last character of `typed`
//! after the rest (a `⎵` before it is the automatic space) and expects
//! `result` (a `⎵` at its end: an automatic space after it); `allows
//! "typed"` expects what its rule is about not to happen: the key typed as
//! it is, or in a `Corrects` rule no correction; `corrects "typed"`
//! expects autocorrect to act on the last character. A file's
//! tests run against the rules it comes first in ([`Rules::failures`]): a
//! language's file with `all.weir` after it.

use std::collections::HashMap;
use std::fmt::{self, Write as _};
use std::ops::Range;

use unicode_normalization::char::is_combining_mark;

/// The mark for the automatic space in rules and their tests: the space a
/// strip word or a glide typed after itself, which the next key may move.
pub const AUTO_SPACE: char = '\u{23B5}';

/// What typing a key does under the rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    /// Characters to delete before the caret first; the key is not typed.
    pub delete: usize,
    /// What to type then, in place of the key.
    pub insert: String,
    /// Whether `insert` ends with an automatic space.
    pub auto_space: bool,
}

/// An error in a rule file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    /// The file's name, as given to [`RuleFile::parse`].
    pub file: String,
    /// The line, from 1; 0 for the file as a whole.
    pub line: usize,
    /// What is wrong.
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}", self.file, self.line, self.message)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Word,
    Space,
    Auto,
    Other,
}

/// A token of text: its kind, its text, and where it is, in characters.
#[derive(Debug, Clone)]
struct Tok {
    kind: Kind,
    text: String,
    at: Range<usize>,
}

impl Tok {
    /// Whether this literal token matches `t`, a token of the text.
    fn matches(&self, t: &Tok) -> bool {
        self.kind == t.kind
            && if self.kind == Kind::Word {
                self.text.to_lowercase() == t.text.to_lowercase()
            } else {
                self.text == t.text
            }
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || is_combining_mark(c) || matches!(c, '\u{200C}' | '\u{200D}')
}

/// A space within a line: any width, not a line break nor a tab.
fn is_space(c: char) -> bool {
    c.is_whitespace()
        && !matches!(
            c,
            '\n' | '\r' | '\t' | '\u{B}' | '\u{C}' | '\u{85}' | '\u{2028}' | '\u{2029}'
        )
}

/// `text` in tokens, the character at `auto` (if any) the automatic space.
fn tokenize(text: &[char], auto: Option<usize>) -> Vec<Tok> {
    let mut out: Vec<Tok> = Vec::new();
    for (i, &c) in text.iter().enumerate() {
        let kind = if Some(i) == auto {
            Kind::Auto
        } else if is_word_char(c) {
            Kind::Word
        } else if is_space(c) {
            Kind::Space
        } else {
            Kind::Other
        };
        if kind == Kind::Word
            && let Some(last) = out.last_mut()
            && last.kind == Kind::Word
            && last.at.end == i
        {
            last.text.push(c);
            last.at.end = i + 1;
            continue;
        }
        out.push(Tok {
            kind,
            text: if kind == Kind::Auto {
                " ".to_string()
            } else {
                c.to_string()
            },
            at: i..i + 1,
        });
    }
    out
}

#[derive(Debug, Clone)]
enum Pat {
    /// Tokens with these texts, in order.
    Lit(Vec<Tok>),
    Word,
    Space,
    Auto,
    Any,
    Not(Box<Pat>),
    Seq(Vec<Pat>),
    Alt(Vec<Pat>),
    Ref(String),
}

impl Pat {
    /// The parts `$1`, `$2`… name: a sequence's, or the pattern whole.
    fn parts(&self) -> &[Pat] {
        match self {
            Pat::Seq(ps) => ps,
            p => std::slice::from_ref(p),
        }
    }

    fn refs<'a>(&'a self, out: &mut Vec<&'a str>) {
        match self {
            Pat::Ref(n) => out.push(n),
            Pat::Not(p) => p.refs(out),
            Pat::Seq(ps) | Pat::Alt(ps) => ps.iter().for_each(|p| p.refs(out)),
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
enum Piece {
    Text(String),
    /// A part of the main pattern, from 1.
    Part(usize),
}

#[derive(Debug, Clone)]
enum Test {
    Becomes {
        typed: String,
        result: String,
        line: usize,
    },
    Allows {
        typed: String,
        line: usize,
    },
    Corrects {
        typed: String,
        line: usize,
    },
}

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    main: Pat,
    becomes: Option<Vec<Piece>>,
    auto_after: bool,
    corrects: bool,
    tests: Vec<Test>,
}

/// One rule file, parsed.
#[derive(Debug, Clone)]
pub struct RuleFile {
    name: String,
    exprs: HashMap<String, Pat>,
    rules: Vec<Rule>,
}

/// A rule being read, before its end checks.
struct Draft {
    name: String,
    line: usize,
    main: Option<Pat>,
    becomes: Option<(Vec<Piece>, bool, usize)>,
    kind: Option<String>,
    tests: Vec<Test>,
}

impl RuleFile {
    /// Parse a rule file; `name` says which, in errors.
    ///
    /// # Errors
    /// On a line that is not one of the format's
    /// ([module docs](self)), with where.
    pub fn parse(name: &str, src: &str) -> Result<Self, Error> {
        let err = |line: usize, message: String| Error {
            file: name.to_string(),
            line,
            message,
        };
        let mut file = RuleFile {
            name: name.to_string(),
            exprs: HashMap::new(),
            rules: Vec::new(),
        };
        let mut draft: Option<Draft> = None;
        for (i, raw) in src.lines().enumerate() {
            let line_no = i + 1;
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (word, rest) = line.split_once(char::is_whitespace).unwrap_or((line, ""));
            let rest = rest.trim();
            match word {
                "rule" => {
                    if let Some(d) = draft.take() {
                        let line = d.line;
                        file.rules.push(d.finish().map_err(|m| err(line, m))?);
                    }
                    if rest.is_empty() || !rest.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        return Err(err(line_no, format!("a rule's name, not {rest:?}")));
                    }
                    if file.rules.iter().any(|r| r.name == rest) {
                        return Err(err(line_no, format!("a second rule {rest}")));
                    }
                    draft = Some(Draft {
                        name: rest.to_string(),
                        line: line_no,
                        main: None,
                        becomes: None,
                        kind: None,
                        tests: Vec::new(),
                    });
                }
                "expr" => {
                    let (ename, pat) = rest.split_once(char::is_whitespace).unwrap_or((rest, ""));
                    if ename.is_empty() || !ename.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        return Err(err(line_no, format!("an expression's name, not {ename:?}")));
                    }
                    let pat = parse_pattern(pat.trim()).map_err(|m| err(line_no, m))?;
                    if ename == "main" {
                        let Some(d) = draft.as_mut() else {
                            return Err(err(line_no, "`expr main` outside a rule".to_string()));
                        };
                        if d.main.replace(pat).is_some() {
                            return Err(err(
                                line_no,
                                format!("a second `expr main` in {}", d.name),
                            ));
                        }
                    } else if file.exprs.insert(ename.to_string(), pat).is_some() {
                        return Err(err(line_no, format!("a second `expr {ename}`")));
                    }
                }
                "let" | "test" | "allows" | "corrects" => {
                    let Some(d) = draft.as_mut() else {
                        return Err(err(line_no, format!("`{word}` outside a rule")));
                    };
                    d.line_item(word, rest, line_no)
                        .map_err(|m| err(line_no, m))?;
                }
                _ => return Err(err(line_no, format!("unknown statement `{word}`"))),
            }
        }
        if let Some(d) = draft.take() {
            let line = d.line;
            file.rules.push(d.finish().map_err(|m| err(line, m))?);
        }
        Ok(file)
    }

    /// The file's name, as given to [`Self::parse`].
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// How many rules it has.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    /// Whether it has none.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

impl Draft {
    fn line_item(&mut self, word: &str, rest: &str, line: usize) -> Result<(), String> {
        let one = |strings: &[String]| -> Result<String, String> {
            match strings {
                [s] => Ok(s.clone()),
                _ => Err(format!("`{word}` takes one string")),
            }
        };
        if word == "let" {
            let (key, value) = rest.split_once(char::is_whitespace).unwrap_or((rest, ""));
            let value = one(&parse_strings(value.trim())?)?;
            return self.set(key, value, line);
        }
        // A test's strings are text: `\$` is a dollar.
        let strings: Vec<String> = parse_strings(rest)?
            .into_iter()
            .map(|s| s.replace("\\$", "$"))
            .collect();
        match word {
            "test" => match strings.as_slice() {
                [typed, result] => {
                    check_typed(typed)?;
                    if result.trim_end_matches(AUTO_SPACE).contains(AUTO_SPACE) {
                        return Err("a test's result has its ⎵ only at its end".to_string());
                    }
                    self.tests.push(Test::Becomes {
                        typed: typed.clone(),
                        result: result.clone(),
                        line,
                    });
                }
                _ => return Err("`test` takes the typed text and its result".to_string()),
            },
            "allows" => {
                let typed = one(&strings)?;
                check_typed(&typed)?;
                self.tests.push(Test::Allows { typed, line });
            }
            _ => {
                let typed = one(&strings)?;
                check_typed(&typed)?;
                self.tests.push(Test::Corrects { typed, line });
            }
        }
        Ok(())
    }

    fn set(&mut self, key: &str, value: String, line: usize) -> Result<(), String> {
        match key {
            "message" | "description" => {}
            "kind" => {
                if !matches!(value.as_str(), "Spacing" | "Corrects") {
                    return Err(format!("kind \"Spacing\" or \"Corrects\", not {value:?}"));
                }
                self.kind = Some(value);
            }
            "becomes" => {
                let (pieces, auto) = parse_template(&value)?;
                self.becomes = Some((pieces, auto, line));
            }
            _ => return Err(format!("unknown `let {key}`")),
        }
        Ok(())
    }

    fn finish(self) -> Result<Rule, String> {
        let name = self.name;
        let main = self
            .main
            .ok_or_else(|| format!("rule {name} has no `expr main`"))?;
        let corrects = self.kind.as_deref() == Some("Corrects");
        let (becomes, auto_after) = match (self.becomes, corrects) {
            (Some(_), true) => {
                return Err(format!("rule {name} corrects, and becomes nothing"));
            }
            (None, false) => return Err(format!("rule {name} has no `let becomes`")),
            (Some((pieces, auto, _)), false) => {
                let parts = main.parts().len();
                if let Some(n) = pieces.iter().find_map(|p| match p {
                    Piece::Part(n) if *n == 0 || *n > parts => Some(*n),
                    _ => None,
                }) {
                    return Err(format!("rule {name} becomes ${n}, of {parts} parts"));
                }
                (Some(pieces), auto)
            }
            (None, true) => (None, false),
        };
        if corrects && self.tests.iter().any(|t| matches!(t, Test::Becomes { .. })) {
            return Err(format!(
                "rule {name} corrects: its tests are `corrects` and `allows`"
            ));
        }
        Ok(Rule {
            name,
            main,
            becomes,
            auto_after,
            corrects,
            tests: self.tests,
        })
    }
}

/// A test's typed text: a character to type, its ⎵ only right before it.
fn check_typed(typed: &str) -> Result<(), String> {
    let mut cs: Vec<char> = typed.chars().collect();
    if cs.pop().is_none_or(|c| c == AUTO_SPACE) {
        return Err("a test types a character last".to_string());
    }
    if cs.last() == Some(&AUTO_SPACE) {
        cs.pop();
    }
    if cs.contains(&AUTO_SPACE) {
        return Err("a test's ⎵ is right before the character typed".to_string());
    }
    Ok(())
}

/// The strings on a line: `"…"` separated by spaces, unescaped (`$` kept).
fn parse_strings(s: &str) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    let mut rest = s.trim_start();
    while !rest.is_empty() {
        let (raw, after) = split_string(rest)?;
        out.push(unescape(raw)?);
        rest = after.trim_start();
    }
    Ok(out)
}

/// The raw text of the string `s` starts with, and what follows it.
fn split_string(s: &str) -> Result<(&str, &str), String> {
    let body = s
        .strip_prefix('"')
        .ok_or_else(|| format!("a string, not {s:?}"))?;
    let mut escaped = false;
    for (i, c) in body.char_indices() {
        match c {
            _ if escaped => escaped = false,
            '\\' => escaped = true,
            '"' => return Ok((&body[..i], &body[i + 1..])),
            _ => {}
        }
    }
    Err(format!("an unclosed string: {s:?}"))
}

/// A string's text, its escapes read; `\$` stays escaped for templates.
fn unescape(raw: &str) -> Result<String, String> {
    let mut out = String::new();
    let mut cs = raw.chars();
    while let Some(c) = cs.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match cs.next() {
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some('$') => out.push_str("\\$"),
            Some('u') => {
                let hex: String = match cs.next() {
                    Some('{') => cs.by_ref().take_while(|&c| c != '}').collect(),
                    _ => return Err("`\\u` is followed by {hex}".to_string()),
                };
                let ch = u32::from_str_radix(&hex, 16)
                    .ok()
                    .and_then(char::from_u32)
                    .ok_or_else(|| format!("no character \\u{{{hex}}}"))?;
                out.push(ch);
            }
            other => return Err(format!("an unknown escape \\{}", other.unwrap_or(' '))),
        }
    }
    Ok(out)
}

/// A `becomes` template: its pieces, and whether it ends with ⎵.
fn parse_template(s: &str) -> Result<(Vec<Piece>, bool), String> {
    let (body, auto) = match s.strip_suffix(AUTO_SPACE) {
        Some(b) => (b, true),
        None => (s, false),
    };
    if body.contains(AUTO_SPACE) {
        return Err("`becomes` has its ⎵ only at its end".to_string());
    }
    let mut pieces = Vec::new();
    let mut text = String::new();
    let mut cs = body.chars().peekable();
    while let Some(c) = cs.next() {
        match c {
            '\\' if cs.peek() == Some(&'$') => {
                cs.next();
                text.push('$');
            }
            '$' => {
                let mut digits = String::new();
                while let Some(d) = cs.peek().copied().filter(char::is_ascii_digit) {
                    digits.push(d);
                    cs.next();
                }
                let n: usize = digits
                    .parse()
                    .map_err(|_| "`$` is followed by a part's number (`\\$` for itself)")?;
                if !text.is_empty() {
                    pieces.push(Piece::Text(std::mem::take(&mut text)));
                }
                pieces.push(Piece::Part(n));
            }
            _ => text.push(c),
        }
    }
    if !text.is_empty() {
        pieces.push(Piece::Text(text));
    }
    Ok((pieces, auto))
}

/// The lexemes of a pattern.
#[derive(Debug, Clone, PartialEq)]
enum Lex {
    Open,
    Close,
    OpenAlt,
    CloseAlt,
    Comma,
    Not,
    Any,
    Ref(String),
    Str(String),
    Bare(String),
}

fn lex_pattern(s: &str) -> Result<Vec<Lex>, String> {
    let mut out = Vec::new();
    let mut rest = s;
    loop {
        rest = rest.trim_start();
        let Some(c) = rest.chars().next() else {
            return Ok(out);
        };
        let simple = match c {
            '(' => Some(Lex::Open),
            ')' => Some(Lex::Close),
            '[' => Some(Lex::OpenAlt),
            ']' => Some(Lex::CloseAlt),
            ',' => Some(Lex::Comma),
            '!' => Some(Lex::Not),
            '*' => Some(Lex::Any),
            _ => None,
        };
        if let Some(l) = simple {
            out.push(l);
            rest = &rest[c.len_utf8()..];
            continue;
        }
        if c == '"' {
            let (raw, after) = split_string(rest)?;
            let text = unescape(raw)?.replace("\\$", "$");
            if text.is_empty() || text.contains(AUTO_SPACE) {
                return Err("a pattern's string is text, not empty nor ⎵ (AUTOSPACE)".to_string());
            }
            out.push(Lex::Str(text));
            rest = after;
            continue;
        }
        let end = rest
            .find(|c: char| c.is_whitespace() || "()[],!@\"*#".contains(c))
            .unwrap_or(rest.len());
        if c == '@' {
            let name_end = rest[1..]
                .find(|c: char| !(c.is_alphanumeric() || c == '_'))
                .map_or(rest.len(), |i| i + 1);
            if name_end == 1 {
                return Err("`@` is followed by an expression's name".to_string());
            }
            out.push(Lex::Ref(rest[1..name_end].to_string()));
            rest = &rest[name_end..];
            continue;
        }
        if end == 0 {
            return Err(format!("unexpected {c:?} in a pattern"));
        }
        out.push(Lex::Bare(rest[..end].to_string()));
        rest = &rest[end..];
    }
}

fn parse_pattern(s: &str) -> Result<Pat, String> {
    let lex = lex_pattern(s)?;
    let mut at = 0;
    let p = parse_seq(&lex, &mut at)?;
    if at != lex.len() {
        return Err(format!("unexpected {:?} in a pattern", lex[at]));
    }
    Ok(p)
}

/// One or more units, until `)`, `]`, `,` or the end.
fn parse_seq(lex: &[Lex], at: &mut usize) -> Result<Pat, String> {
    let mut parts = Vec::new();
    while let Some(l) = lex.get(*at) {
        if matches!(l, Lex::Close | Lex::CloseAlt | Lex::Comma) {
            break;
        }
        parts.push(parse_unit(lex, at)?);
    }
    match parts.len() {
        0 => Err("an empty pattern".to_string()),
        1 => Ok(parts.pop().expect("one part")),
        _ => Ok(Pat::Seq(parts)),
    }
}

fn parse_unit(lex: &[Lex], at: &mut usize) -> Result<Pat, String> {
    let l = lex.get(*at).ok_or("a pattern ends too soon")?.clone();
    *at += 1;
    Ok(match l {
        Lex::Not => Pat::Not(Box::new(parse_unit(lex, at)?)),
        Lex::Any => Pat::Any,
        Lex::Ref(n) => Pat::Ref(n),
        Lex::Open => {
            let inner = parse_seq(lex, at)?;
            if lex.get(*at) != Some(&Lex::Close) {
                return Err("`(` is closed by `)`".to_string());
            }
            *at += 1;
            // A sequence in parentheses is one, whatever it holds.
            match inner {
                Pat::Seq(ps) => Pat::Seq(ps),
                p => Pat::Seq(vec![p]),
            }
        }
        Lex::OpenAlt => {
            let mut alts = vec![parse_seq(lex, at)?];
            loop {
                match lex.get(*at) {
                    Some(Lex::Comma) => {
                        *at += 1;
                        alts.push(parse_seq(lex, at)?);
                    }
                    Some(Lex::CloseAlt) => {
                        *at += 1;
                        break;
                    }
                    _ => return Err("`[` is closed by `]`".to_string()),
                }
            }
            Pat::Alt(alts)
        }
        Lex::Str(text) => literal(&text),
        Lex::Bare(text) => match text.as_str() {
            "WORD" => Pat::Word,
            "SPACE" => Pat::Space,
            "AUTOSPACE" => Pat::Auto,
            _ if text.chars().all(|c| c.is_uppercase() || c == '_') && text.chars().count() > 1 => {
                return Err(format!("no token class {text} (WORD, SPACE, AUTOSPACE)"));
            }
            _ => literal(&text),
        },
        Lex::Close | Lex::CloseAlt | Lex::Comma => {
            return Err(format!("unexpected {l:?} in a pattern"));
        }
    })
}

fn literal(text: &str) -> Pat {
    let cs: Vec<char> = text.chars().collect();
    Pat::Lit(tokenize(&cs, None))
}

/// Rules in force: one language's file first, then `all.weir`.
#[derive(Debug, Clone, Default)]
pub struct Rules {
    files: Vec<RuleFile>,
}

/// How deep patterns may refer to patterns.
const MAX_DEPTH: usize = 32;

impl Rules {
    /// The rules of `files`, the first first.
    ///
    /// # Errors
    /// When a pattern refers to a name no file defines, or names refer to
    /// each other in a loop.
    pub fn new(files: Vec<RuleFile>) -> Result<Self, Error> {
        let rules = Rules { files };
        for f in &rules.files {
            let mut pats: Vec<(&str, &Pat)> =
                f.rules.iter().map(|r| (r.name.as_str(), &r.main)).collect();
            pats.extend(f.exprs.iter().map(|(n, p)| (n.as_str(), p)));
            for (name, pat) in pats {
                let mut seen = vec![name];
                rules.check_refs(pat, &mut seen).map_err(|message| Error {
                    file: f.name.clone(),
                    line: 0,
                    message,
                })?;
            }
        }
        Ok(rules)
    }

    fn check_refs<'a>(&'a self, pat: &'a Pat, seen: &mut Vec<&'a str>) -> Result<(), String> {
        let mut refs = Vec::new();
        pat.refs(&mut refs);
        for r in refs {
            if seen.contains(&r) {
                return Err(format!("@{r} refers back to itself"));
            }
            let p = self
                .lookup(r)
                .ok_or_else(|| format!("no expression @{r}"))?;
            seen.push(r);
            self.check_refs(p, seen)?;
            seen.pop();
        }
        Ok(())
    }

    fn lookup(&self, name: &str) -> Option<&Pat> {
        self.files.iter().find_map(|f| f.exprs.get(name))
    }

    fn rules(&self) -> impl Iterator<Item = &Rule> {
        self.files.iter().flat_map(|f| f.rules.iter())
    }

    /// How many rules are in force.
    #[must_use]
    pub fn len(&self) -> usize {
        self.files.iter().map(RuleFile::len).sum()
    }

    /// Whether none is.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The starts of the matches of `p` that end at token `end`, longest
    /// first.
    fn starts(&self, p: &Pat, toks: &[Tok], end: usize, depth: usize) -> Vec<usize> {
        if depth > MAX_DEPTH {
            return Vec::new();
        }
        let one = |ok: bool| if ok { vec![end - 1] } else { Vec::new() };
        let last = end.checked_sub(1).map(|i| &toks[i]);
        let mut out = match p {
            Pat::Lit(ls) => {
                let n = ls.len();
                if n <= end
                    && ls
                        .iter()
                        .zip(&toks[end - n..end])
                        .all(|(l, t)| l.matches(t))
                {
                    vec![end - n]
                } else {
                    Vec::new()
                }
            }
            Pat::Word => one(last.is_some_and(|t| t.kind == Kind::Word)),
            Pat::Space => one(last.is_some_and(|t| t.kind == Kind::Space)),
            Pat::Auto => one(last.is_some_and(|t| t.kind == Kind::Auto)),
            Pat::Any => one(last.is_some()),
            Pat::Not(q) => {
                one(last.is_some() && !self.starts(q, toks, end, depth + 1).contains(&(end - 1)))
            }
            Pat::Seq(ps) => {
                let mut ends = vec![end];
                for q in ps.iter().rev() {
                    let mut next = Vec::new();
                    for &e in &ends {
                        next.extend(self.starts(q, toks, e, depth + 1));
                    }
                    ends = next;
                }
                ends
            }
            Pat::Alt(ps) => ps
                .iter()
                .flat_map(|q| self.starts(q, toks, end, depth + 1))
                .collect(),
            Pat::Ref(name) => self
                .lookup(name)
                .map(|q| self.starts(q, toks, end, depth + 1))
                .unwrap_or_default(),
        };
        out.sort_unstable();
        out.dedup();
        out
    }

    /// Where each of `parts` matched, ending at token `end`, if they did.
    fn spans(&self, parts: &[Pat], toks: &[Tok], end: usize) -> Option<Vec<Range<usize>>> {
        let Some((last, rest)) = parts.split_last() else {
            return Some(Vec::new());
        };
        for s in self.starts(last, toks, end, 0) {
            if let Some(mut v) = self.spans(rest, toks, s) {
                v.push(s..end);
                return Some(v);
            }
        }
        None
    }

    /// What typing `key` after `before`, the text before the caret, does
    /// under the rules: `None` when it types the key and nothing else.
    /// `auto_space`: the last character of `before` is the space a strip
    /// word or a glide typed after itself.
    #[must_use]
    pub fn typed(&self, before: &str, auto_space: bool, key: &str) -> Option<Edit> {
        let b: Vec<char> = before.chars().collect();
        let auto = (auto_space && b.last().copied().is_some_and(is_space)).then(|| b.len() - 1);
        let text: Vec<char> = b.iter().copied().chain(key.chars()).collect();
        let toks = tokenize(&text, auto);
        for rule in self.rules().filter(|r| !r.corrects) {
            let Some(spans) = self.spans(rule.main.parts(), &toks, toks.len()) else {
                continue;
            };
            let Some(template) = &rule.becomes else {
                continue;
            };
            let mut new = String::new();
            for piece in template {
                match piece {
                    Piece::Text(s) => new.push_str(s),
                    Piece::Part(n) => {
                        for t in &toks[spans[n - 1].clone()] {
                            new.push_str(&t.text);
                        }
                    }
                }
            }
            if rule.auto_after {
                new.push(' ');
            }
            let from = spans.first().map_or(text.len(), |s| {
                toks.get(s.start).map_or(text.len(), |t| t.at.start)
            });
            let result: Vec<char> = text[..from].iter().copied().chain(new.chars()).collect();
            let kept = b.iter().zip(&result).take_while(|(x, y)| x == y).count();
            let edit = Edit {
                delete: b.len() - kept,
                insert: result[kept..].iter().collect(),
                auto_space: rule.auto_after,
            };
            if edit.delete == 0 && edit.insert == key && !edit.auto_space {
                return None;
            }
            return Some(edit);
        }
        None
    }

    /// Whether autocorrect acts on the word before `key`, typed after
    /// `before`, as it does on space.
    #[must_use]
    pub fn corrects(&self, before: &str, key: &str) -> bool {
        let text: Vec<char> = before.chars().chain(key.chars()).collect();
        let toks = tokenize(&text, None);
        self.rules()
            .filter(|r| r.corrects)
            .any(|r| !self.starts(&r.main, &toks, toks.len(), 0).is_empty())
    }

    /// The tests of the first file that fail against these rules, each
    /// said in a line; none when all pass.
    #[must_use]
    pub fn failures(&self) -> Vec<String> {
        let Some(file) = self.files.first() else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for rule in &file.rules {
            for test in &rule.tests {
                let (typed, line) = match test {
                    Test::Becomes { typed, line, .. }
                    | Test::Allows { typed, line }
                    | Test::Corrects { typed, line } => (typed, *line),
                };
                let (before, auto, key) = split_typed(typed);
                let edit = self.typed(&before, auto, &key);
                let corrects = self.corrects(&before, &key);
                let fail = match test {
                    Test::Becomes { result, .. } => {
                        let got = apply(&before, &key, edit.as_ref());
                        (got != *result).then(|| {
                            format!("{} gives {}, not {}", show(typed), show(&got), show(result))
                        })
                    }
                    Test::Allows { .. } if rule.corrects => {
                        corrects.then(|| format!("{} corrects", show(typed)))
                    }
                    Test::Allows { .. } => edit.as_ref().map(|_| {
                        format!(
                            "{} gives {}",
                            show(typed),
                            show(&apply(&before, &key, edit.as_ref()))
                        )
                    }),
                    Test::Corrects { .. } => {
                        (!corrects).then(|| format!("{} does not correct", show(typed)))
                    }
                };
                if let Some(f) = fail {
                    out.push(format!("{}:{line}: {}: {f}", file.name, rule.name));
                }
            }
        }
        out
    }
}

/// A test's typed text: the text before, whether it ends with the
/// automatic space, and the key.
fn split_typed(typed: &str) -> (String, bool, String) {
    let mut cs: Vec<char> = typed.chars().collect();
    let key = cs.pop().map(String::from).unwrap_or_default();
    let auto = cs.last() == Some(&AUTO_SPACE);
    if auto {
        cs.pop();
        cs.push(' ');
    }
    (cs.into_iter().collect(), auto, key)
}

/// The text typing `key` after `before` leaves, with ⎵ at its end for an
/// automatic space, as tests write it.
fn apply(before: &str, key: &str, edit: Option<&Edit>) -> String {
    match edit {
        None => format!("{before}{key}"),
        Some(e) => {
            let keep = before.chars().count() - e.delete;
            let mut out: String = before.chars().take(keep).collect();
            if e.auto_space {
                out.push_str(e.insert.strip_suffix(' ').unwrap_or(&e.insert));
                out.push(AUTO_SPACE);
            } else {
                out.push_str(&e.insert);
            }
            out
        }
    }
}

/// `s` quoted, its spaces other than U+0020 written as escapes.
fn show(s: &str) -> String {
    let mut out = String::from('"');
    for c in s.chars() {
        if c != ' ' && is_space(c) {
            let _ = write!(out, "\\u{{{:X}}}", u32::from(c));
        } else {
            out.push(c);
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rules(srcs: &[&str]) -> Rules {
        Rules::new(
            srcs.iter()
                .enumerate()
                .map(|(i, s)| RuleFile::parse(&format!("f{i}"), s).expect("parses"))
                .collect(),
        )
        .expect("resolves")
    }

    const ALL: &str = r#"
expr closing [".", ",", "?", "!"]

rule SpaceGivesWay
expr main (AUTOSPACE @closing)
let becomes "$2⎵"
test "word⎵," "word,⎵"
allows "word ,"

rule CorrectsOnPunctuation
let kind "Corrects"
expr main (WORD @closing)
corrects "teh,"
allows "teh'"
"#;

    #[test]
    fn the_automatic_space_gives_way_and_only_it() {
        let r = rules(&[ALL]);
        assert_eq!(
            r.typed("a word ", true, ","),
            Some(Edit {
                delete: 1,
                insert: ", ".into(),
                auto_space: true
            })
        );
        // A space typed is the user's.
        assert_eq!(r.typed("a word ", false, ","), None);
        // Not punctuation it gives way to.
        assert_eq!(r.typed("a word ", true, "("), None);
        assert!(r.failures().is_empty(), "{:?}", r.failures());
    }

    #[test]
    fn a_languages_rule_comes_first_and_only_what_changes_is_retyped() {
        let fr = r#"
rule NarrowSpace
expr main (WORD AUTOSPACE ["?", "!"])
let becomes "$1\u{202F}$3⎵"
test "Bonjour⎵?" "Bonjour\u{202F}?⎵"
test "Quoi\u{202F}?⎵!" "Quoi\u{202F}?!⎵"
"#;
        let r = rules(&[fr, ALL]);
        assert_eq!(
            r.typed("Bonjour ", true, "?"),
            Some(Edit {
                delete: 1,
                insert: "\u{202F}? ".into(),
                auto_space: true
            })
        );
        assert!(r.failures().is_empty(), "{:?}", r.failures());
        // Punctuation it has no rule for: all.weir's.
        assert_eq!(
            r.typed("Bonjour ", true, ","),
            rules(&[ALL]).typed("Bonjour ", true, ",")
        );
    }

    #[test]
    fn autocorrect_acts_where_a_corrects_rule_matches() {
        let r = rules(&[ALL]);
        assert!(r.corrects("teh", ","));
        assert!(!r.corrects("teh", "'"));
        assert!(!r.corrects("teh, ", "?"));
    }

    #[test]
    fn failing_tests_are_reported_with_where() {
        let bad = r#"
rule Wrong
expr main (AUTOSPACE ",")
let becomes ";⎵"
test "word⎵," "word,⎵"
allows "word⎵,"
"#;
        let f = rules(&[bad]).failures();
        assert_eq!(f.len(), 2, "{f:?}");
        assert!(
            f[0].starts_with("f0:5: Wrong: \"word⎵,\" gives \"word;⎵\""),
            "{f:?}"
        );
    }

    #[test]
    fn words_match_whole_and_in_any_case() {
        let r = rules(&[r#"
rule Teh
expr main (teh SPACE)
let becomes "the "
test "TEH " "the "
allows "tehx "
"#]);
        assert!(r.failures().is_empty(), "{:?}", r.failures());
    }

    #[test]
    fn exceptions_alternatives_and_wildcards() {
        let r = rules(&[r#"
rule NoSpaceBeforeQuestion
expr main (!no SPACE "?")
let becomes "$1?"
test "si ?" "si?"
allows "no ?"

rule Interrobang
expr main (* "\u{203D}")
let becomes "$1?!"
test "a\u{203D}" "a?!"

rule SwallowedSpace
expr main ("\u{AB}" "\u{A0}" SPACE)
let becomes "\u{AB}\u{A0}"
test "dit \u{AB}\u{A0} " "dit \u{AB}\u{A0}"

rule Guillemet
expr main (WORD [AUTOSPACE, SPACE] "\u{BB}")
let becomes "$1\u{A0}\u{BB}"
test "oui⎵\u{BB}" "oui\u{A0}\u{BB}"
test "oui \u{BB}" "oui\u{A0}\u{BB}"
"#]);
        assert!(r.failures().is_empty(), "{:?}", r.failures());
        // A space swallowed is an edit that types nothing.
        assert_eq!(
            r.typed("dit \u{AB}\u{A0}", false, " "),
            Some(Edit {
                delete: 0,
                insert: String::new(),
                auto_space: false
            })
        );
    }

    #[test]
    fn errors_say_where() {
        let e = RuleFile::parse("x.weir", "rule A\nexpr main (WORD\nlet becomes \"\"\n")
            .expect_err("unclosed");
        assert_eq!((e.file.as_str(), e.line), ("x.weir", 2));
        let e = RuleFile::parse("x.weir", "rule A\nexpr main WORD\n").expect_err("no becomes");
        assert!(e.message.contains("no `let becomes`"), "{e}");
        let e = RuleFile::parse("x.weir", "rule A\nexpr main WORD\nlet becomes \"$2\"\n")
            .expect_err("no part 2");
        assert!(e.message.contains("$2"), "{e}");
        let e = RuleFile::parse("x.weir", "rule A\nexpr main WROD\nlet becomes \"\"\n")
            .expect_err("no class");
        assert!(e.message.contains("WROD"), "{e}");
        let f = RuleFile::parse("x.weir", "rule A\nexpr main @nope\nlet becomes \"\"\n")
            .expect("parses");
        assert!(Rules::new(vec![f]).is_err());
        let f = RuleFile::parse("x.weir", "expr a @b\nexpr b @a\n").expect("parses");
        assert!(Rules::new(vec![f]).is_err());
    }
}
