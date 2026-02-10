//! Tokenizer traits and default implementations.

use regex::Regex;

use crate::collections::dataset::token::{Token, TokenKind, TokenSpan};

/// Simple pluggable tokenizer trait.
pub trait Tokenizer {
    /// Tokenize the input text and return a sequence of `Token`s.
    fn tokenize(&self, text: &str) -> Vec<Token>;
}

/// Default whitespace tokenizer: splits on Unicode whitespace.
#[derive(Debug, Clone, Copy, Default)]
pub struct WhitespaceTokenizer;

impl Tokenizer for WhitespaceTokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        whitespace_spans(text)
            .into_iter()
            .map(|(start, end)| {
                Token::word(text[start..end].to_string(), TokenSpan::new(start, end))
            })
            .collect()
    }
}

/// Regex-based tokenizer (tokens or gaps).
#[derive(Debug, Clone)]
pub struct RegexpTokenizer {
    regex: Regex,
    gaps: bool,
    discard_empty: bool,
}

impl RegexpTokenizer {
    pub fn new(pattern: impl AsRef<str>) -> Result<Self, regex::Error> {
        Self::with_options(pattern, false, true)
    }

    pub fn with_options(
        pattern: impl AsRef<str>,
        gaps: bool,
        discard_empty: bool,
    ) -> Result<Self, regex::Error> {
        let regex = Regex::new(pattern.as_ref())?;
        Ok(Self {
            regex,
            gaps,
            discard_empty,
        })
    }

    fn spans(&self, text: &str) -> Vec<(usize, usize)> {
        if self.gaps {
            spans_from_gaps(&self.regex, text, self.discard_empty)
        } else {
            self.regex
                .find_iter(text)
                .map(|m| (m.start(), m.end()))
                .collect()
        }
    }
}

impl Tokenizer for RegexpTokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        self.spans(text)
            .into_iter()
            .map(|(start, end)| {
                Token::new(
                    text[start..end].to_string(),
                    TokenSpan::new(start, end),
                    TokenKind::Word,
                )
            })
            .collect()
    }
}

/// Tokenize on word characters and punctuation.
#[derive(Debug, Clone)]
pub struct WordPunctTokenizer {
    inner: RegexpTokenizer,
}

impl WordPunctTokenizer {
    pub fn new() -> Self {
        let inner = RegexpTokenizer::with_options(r"\w+|[^\w\s]+", false, true)
            .expect("valid word punct pattern");
        Self { inner }
    }
}

impl Default for WordPunctTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer for WordPunctTokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        self.inner.tokenize(text)
    }
}

/// Tokenize on blank lines (gaps).
#[derive(Debug, Clone)]
pub struct BlanklineTokenizer {
    inner: RegexpTokenizer,
}

impl BlanklineTokenizer {
    pub fn new() -> Self {
        let inner = RegexpTokenizer::with_options(r"\s*\n\s*\n\s*", true, true)
            .expect("valid blankline pattern");
        Self { inner }
    }
}

impl Default for BlanklineTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer for BlanklineTokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        self.inner.tokenize(text)
    }
}

/// Tokenize by splitting on a fixed delimiter.
#[derive(Debug, Clone)]
pub struct StringSplitTokenizer {
    delimiter: String,
    discard_empty: bool,
}

impl StringSplitTokenizer {
    pub fn new(delimiter: impl Into<String>, discard_empty: bool) -> Self {
        Self {
            delimiter: delimiter.into(),
            discard_empty,
        }
    }
}

impl Tokenizer for StringSplitTokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        spans_from_delimiter(text, &self.delimiter, self.discard_empty)
            .into_iter()
            .map(|(start, end)| {
                Token::new(
                    text[start..end].to_string(),
                    TokenSpan::new(start, end),
                    TokenKind::Word,
                )
            })
            .collect()
    }
}

/// Tokenize a string using the space character as a delimiter.
#[derive(Debug, Clone)]
pub struct SpaceTokenizer {
    inner: StringSplitTokenizer,
}

impl SpaceTokenizer {
    pub fn new() -> Self {
        Self {
            inner: StringSplitTokenizer::new(" ", false),
        }
    }
}

impl Default for SpaceTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer for SpaceTokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        self.inner.tokenize(text)
    }
}

/// Tokenize a string using the tab character as a delimiter.
#[derive(Debug, Clone)]
pub struct TabTokenizer {
    inner: StringSplitTokenizer,
}

impl TabTokenizer {
    pub fn new() -> Self {
        Self {
            inner: StringSplitTokenizer::new("\t", false),
        }
    }
}

impl Default for TabTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer for TabTokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        self.inner.tokenize(text)
    }
}

/// Tokenize a string into its individual characters.
#[derive(Debug, Clone, Copy, Default)]
pub struct CharTokenizer;

impl Tokenizer for CharTokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        let mut out = Vec::new();
        for (start, ch) in text.char_indices() {
            let end = start + ch.len_utf8();
            out.push(Token::new(
                ch.to_string(),
                TokenSpan::new(start, end),
                TokenKind::Symbol,
            ));
        }
        out
    }
}

/// Behavior for blank lines in `LineTokenizer`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineBlankMode {
    Discard,
    Keep,
    DiscardEof,
}

/// Tokenize a string into its lines.
#[derive(Debug, Clone, Copy)]
pub struct LineTokenizer {
    blank_mode: LineBlankMode,
}

impl LineTokenizer {
    pub fn new(blank_mode: LineBlankMode) -> Self {
        Self { blank_mode }
    }
}

impl Tokenizer for LineTokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        line_spans(text, self.blank_mode)
            .into_iter()
            .map(|(start, end)| {
                Token::new(
                    text[start..end].to_string(),
                    TokenSpan::new(start, end),
                    TokenKind::Word,
                )
            })
            .collect()
    }
}

/// Tokenize strings into s-expressions and non-parenthesized tokens.
#[derive(Debug, Clone)]
pub struct SExprTokenizer {
    open: char,
    close: char,
    strict: bool,
}

impl SExprTokenizer {
    pub fn new(parens: &str, strict: bool) -> Result<Self, &'static str> {
        let mut chars = parens.chars();
        let Some(open) = chars.next() else {
            return Err("parens must contain two chars");
        };
        let Some(close) = chars.next() else {
            return Err("parens must contain two chars");
        };
        Ok(Self {
            open,
            close,
            strict,
        })
    }
}

impl Tokenizer for SExprTokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        let mut out = Vec::new();
        let mut depth = 0i64;
        let mut pos = 0usize;
        let mut i = 0usize;
        let bytes = text.as_bytes();

        while i < bytes.len() {
            let ch = text[i..].chars().next().unwrap();
            let ch_len = ch.len_utf8();
            if ch == self.open {
                if depth == 0 {
                    for (start, end) in whitespace_spans(&text[pos..i]) {
                        let s = start + pos;
                        let e = end + pos;
                        out.push(Token::new(
                            text[s..e].to_string(),
                            TokenSpan::new(s, e),
                            TokenKind::Word,
                        ));
                    }
                    pos = i;
                }
                depth += 1;
            } else if ch == self.close {
                depth -= 1;
                if self.strict && depth < 0 {
                    return Vec::new();
                }
                if depth == 0 {
                    let end = i + ch_len;
                    out.push(Token::new(
                        text[pos..end].to_string(),
                        TokenSpan::new(pos, end),
                        TokenKind::Word,
                    ));
                    pos = end;
                }
            }
            i += ch_len;
        }

        if self.strict && depth > 0 {
            return Vec::new();
        }

        if pos < text.len() {
            for (start, end) in whitespace_spans(&text[pos..]) {
                let s = start + pos;
                let e = end + pos;
                out.push(Token::new(
                    text[s..e].to_string(),
                    TokenSpan::new(s, e),
                    TokenKind::Word,
                ));
            }
        }

        out
    }
}

/// Tokenize XML/HTML-like markup tags and text.
#[derive(Debug, Clone)]
pub struct MarkupTokenizer {
    inner: RegexpTokenizer,
}

impl MarkupTokenizer {
    pub fn new() -> Self {
        let inner = RegexpTokenizer::with_options(r"<[^>]+>|[^<>\s]+", false, true)
            .expect("valid markup pattern");
        Self { inner }
    }
}

impl Default for MarkupTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer for MarkupTokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        self.inner.tokenize(text)
    }
}

/// Tokenize JSON text into strings, numbers, literals, and punctuation tokens.
#[derive(Debug, Clone, Copy, Default)]
pub struct JsonTokenizer;

impl Tokenizer for JsonTokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut i = 0usize;
        let bytes = text.as_bytes();

        while i < bytes.len() {
            let ch = text[i..].chars().next().unwrap();
            let ch_len = ch.len_utf8();

            if ch.is_whitespace() {
                i += ch_len;
                continue;
            }

            if ch == '"' {
                let start = i;
                i += ch_len;
                while i < bytes.len() {
                    let next = text[i..].chars().next().unwrap();
                    let next_len = next.len_utf8();
                    if next == '\\' {
                        i += next_len;
                        if i < bytes.len() {
                            let esc_len = text[i..].chars().next().unwrap().len_utf8();
                            i += esc_len;
                        }
                        continue;
                    }
                    if next == '"' {
                        i += next_len;
                        break;
                    }
                    i += next_len;
                }
                let end = i.min(text.len());
                tokens.push(Token::new(
                    text[start..end].to_string(),
                    TokenSpan::new(start, end),
                    TokenKind::Word,
                ));
                continue;
            }

            if ch == '-' || ch.is_ascii_digit() {
                let start = i;
                i += ch_len;
                while i < bytes.len() {
                    let next = text[i..].chars().next().unwrap();
                    if next.is_ascii_digit() || next == '.' || next == 'e' || next == 'E' || next == '+' || next == '-' {
                        i += next.len_utf8();
                    } else {
                        break;
                    }
                }
                let end = i;
                tokens.push(Token::new(
                    text[start..end].to_string(),
                    TokenSpan::new(start, end),
                    TokenKind::Number,
                ));
                continue;
            }

            if ch.is_ascii_alphabetic() {
                let start = i;
                i += ch_len;
                while i < bytes.len() {
                    let next = text[i..].chars().next().unwrap();
                    if next.is_ascii_alphabetic() {
                        i += next.len_utf8();
                    } else {
                        break;
                    }
                }
                let end = i;
                tokens.push(Token::new(
                    text[start..end].to_string(),
                    TokenSpan::new(start, end),
                    TokenKind::Word,
                ));
                continue;
            }

            let start = i;
            let end = i + ch_len;
            tokens.push(Token::new(
                text[start..end].to_string(),
                TokenSpan::new(start, end),
                TokenKind::Punct,
            ));
            i = end;
        }

        tokens
    }
}

fn whitespace_spans(text: &str) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut start: Option<usize> = None;
    for (idx, ch) in text.char_indices() {
        if ch.is_whitespace() {
            if let Some(s) = start.take() {
                spans.push((s, idx));
            }
        } else if start.is_none() {
            start = Some(idx);
        }
    }
    if let Some(s) = start {
        spans.push((s, text.len()));
    }
    spans
}

fn spans_from_gaps(regex: &Regex, text: &str, discard_empty: bool) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut last = 0usize;
    for m in regex.find_iter(text) {
        let start = m.start();
        if !(discard_empty && start == last) {
            spans.push((last, start));
        }
        last = m.end();
    }
    if !(discard_empty && last == text.len()) {
        spans.push((last, text.len()));
    }
    spans
}

fn spans_from_delimiter(text: &str, delimiter: &str, discard_empty: bool) -> Vec<(usize, usize)> {
    if delimiter.is_empty() {
        return Vec::new();
    }
    let mut spans = Vec::new();
    let mut start = 0usize;
    while let Some(pos) = text[start..].find(delimiter) {
        let end = start + pos;
        if !(discard_empty && end == start) {
            spans.push((start, end));
        }
        start = end + delimiter.len();
    }
    if !(discard_empty && start == text.len()) {
        spans.push((start, text.len()));
    }
    spans
}

fn line_spans(text: &str, mode: LineBlankMode) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut start = 0usize;
    let bytes = text.as_bytes();
    for (idx, b) in bytes.iter().enumerate() {
        if *b == b'\n' {
            spans.push((start, idx));
            start = idx + 1;
        }
    }
    if start <= text.len() {
        spans.push((start, text.len()));
    }

    match mode {
        LineBlankMode::Keep => spans,
        LineBlankMode::Discard => spans.into_iter().filter(|(s, e)| *s != *e).collect(),
        LineBlankMode::DiscardEof => {
            if spans.last().is_some_and(|(s, e)| s == e) {
                spans.pop();
            }
            spans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace_tokenizer_splits_on_space() {
        let t = WhitespaceTokenizer::default();
        let toks = t.tokenize("hello  world\nnext");
        let texts: Vec<&str> = toks.iter().map(|tk| tk.text()).collect();
        assert_eq!(texts, vec!["hello", "world", "next"]);
    }

    #[test]
    fn regex_tokenizer_matches_words() {
        let tok = RegexpTokenizer::new(r"\w+").unwrap();
        let toks = tok.tokenize("a b");
        let texts: Vec<&str> = toks.iter().map(|tk| tk.text()).collect();
        assert_eq!(texts, vec!["a", "b"]);
    }

    #[test]
    fn wordpunct_tokenizer_splits_punct() {
        let tok = WordPunctTokenizer::new();
        let toks = tok.tokenize("hi!!!");
        let texts: Vec<&str> = toks.iter().map(|tk| tk.text()).collect();
        assert_eq!(texts, vec!["hi", "!!!"]);
    }

    #[test]
    fn blankline_tokenizer_splits_paragraphs() {
        let tok = BlanklineTokenizer::new();
        let toks = tok.tokenize("a\n\nb");
        let texts: Vec<&str> = toks.iter().map(|tk| tk.text()).collect();
        assert_eq!(texts, vec!["a", "b"]);
    }

    #[test]
    fn markup_tokenizer_keeps_tags() {
        let tok = MarkupTokenizer::new();
        let toks = tok.tokenize("<p>Hello</p>");
        let texts: Vec<&str> = toks.iter().map(|tk| tk.text()).collect();
        assert_eq!(texts, vec!["<p>", "Hello", "</p>"]);
    }
}
