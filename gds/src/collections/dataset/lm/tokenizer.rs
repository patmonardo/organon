//! Tokenizer traits and default implementations.

use regex::Regex;

use crate::collections::dataset::token::{Token, TokenKind, TokenSpan};

/// Simple pluggable tokenizer trait.
pub trait Tokenizer {
    /// Tokenize the input text and return a sequence of `Token`s.
    fn tokenize(&self, text: &str) -> Vec<Token>;

    /// Apply `tokenize()` to each input string.
    fn tokenize_sents(&self, texts: &[&str]) -> Vec<Vec<Token>> {
        texts.iter().map(|text| self.tokenize(text)).collect()
    }

    /// Return just the token spans for the given text.
    fn span_tokenize(&self, text: &str) -> Vec<TokenSpan> {
        self.tokenize(text)
            .into_iter()
            .map(|token| token.span())
            .collect()
    }
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
                    if next.is_ascii_digit()
                        || next == '.'
                        || next == 'e'
                        || next == 'E'
                        || next == '+'
                        || next == '-'
                    {
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

/// Multi-word-expression tokenizer.
#[derive(Debug, Clone)]
pub struct MWETokenizer {
    mwes: Vec<Vec<String>>,
    separator: String,
}

impl MWETokenizer {
    pub fn new(mwes: Vec<Vec<String>>, separator: impl Into<String>) -> Self {
        Self {
            mwes,
            separator: separator.into(),
        }
    }

    pub fn with_separator(separator: impl Into<String>) -> Self {
        Self::new(Vec::new(), separator)
    }

    pub fn add_mwe<I, S>(&mut self, mwe: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let candidate: Vec<String> = mwe.into_iter().map(Into::into).collect();
        if !candidate.is_empty() {
            self.mwes.push(candidate);
        }
    }

    pub fn tokenize_tokens<S: AsRef<str>>(&self, tokens: &[S]) -> Vec<String> {
        let token_texts: Vec<&str> = tokens.iter().map(|t| t.as_ref()).collect();
        self.merge_token_texts(&token_texts)
    }

    fn merge_token_texts(&self, token_texts: &[&str]) -> Vec<String> {
        let mut out = Vec::new();
        let mut idx = 0usize;

        while idx < token_texts.len() {
            let mut best_match: Option<&[String]> = None;
            for mwe in &self.mwes {
                if idx + mwe.len() > token_texts.len() {
                    continue;
                }
                let is_match = mwe
                    .iter()
                    .zip(&token_texts[idx..idx + mwe.len()])
                    .all(|(left, right)| left == right);
                if is_match && best_match.is_none_or(|best| mwe.len() > best.len()) {
                    best_match = Some(mwe);
                }
            }

            if let Some(matched) = best_match {
                out.push(matched.join(&self.separator));
                idx += matched.len();
            } else {
                out.push(token_texts[idx].to_string());
                idx += 1;
            }
        }

        out
    }
}

impl Default for MWETokenizer {
    fn default() -> Self {
        Self::new(Vec::new(), "_")
    }
}

impl Tokenizer for MWETokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        let base_tokens = WhitespaceTokenizer.tokenize(text);
        let mut out = Vec::new();
        let mut idx = 0usize;

        while idx < base_tokens.len() {
            let mut best_match: Option<&[String]> = None;
            for mwe in &self.mwes {
                if idx + mwe.len() > base_tokens.len() {
                    continue;
                }
                let is_match = mwe
                    .iter()
                    .zip(&base_tokens[idx..idx + mwe.len()])
                    .all(|(candidate, token)| candidate == token.text());
                if is_match && best_match.is_none_or(|best| mwe.len() > best.len()) {
                    best_match = Some(mwe);
                }
            }

            if let Some(matched) = best_match {
                let start = base_tokens[idx].span().start();
                let end = base_tokens[idx + matched.len() - 1].span().end();
                out.push(Token::new(
                    matched.join(&self.separator),
                    TokenSpan::new(start, end),
                    TokenKind::Word,
                ));
                idx += matched.len();
            } else {
                out.push(base_tokens[idx].clone());
                idx += 1;
            }
        }

        out
    }
}

pub fn regexp_tokenize(
    text: &str,
    pattern: impl AsRef<str>,
    gaps: bool,
    discard_empty: bool,
) -> Result<Vec<Token>, regex::Error> {
    let tokenizer = RegexpTokenizer::with_options(pattern, gaps, discard_empty)?;
    Ok(tokenizer.tokenize(text))
}

pub fn wordpunct_tokenize(text: &str) -> Vec<Token> {
    WordPunctTokenizer::new().tokenize(text)
}

pub fn blankline_tokenize(text: &str) -> Vec<Token> {
    BlanklineTokenizer::new().tokenize(text)
}

pub fn line_tokenize(text: &str, blank_mode: LineBlankMode) -> Vec<Token> {
    LineTokenizer::new(blank_mode).tokenize(text)
}

pub fn string_span_tokenize(text: &str, sep: &str) -> Result<Vec<TokenSpan>, String> {
    if sep.is_empty() {
        return Err("Token delimiter must not be empty".to_string());
    }

    let mut spans = Vec::new();
    let mut left = 0usize;
    while let Some(pos) = text[left..].find(sep) {
        let right = left + pos;
        if right != 0 {
            spans.push(TokenSpan::new(left, right));
        }
        left = right + sep.len();
    }

    if left != text.len() {
        spans.push(TokenSpan::new(left, text.len()));
    }

    Ok(spans)
}

pub fn regexp_span_tokenize(
    text: &str,
    regexp: impl AsRef<str>,
) -> Result<Vec<TokenSpan>, regex::Error> {
    let regex = Regex::new(regexp.as_ref())?;
    let mut spans = Vec::new();
    let mut left = 0usize;

    for m in regex.find_iter(text) {
        let right = m.start();
        if right != left {
            spans.push(TokenSpan::new(left, right));
        }
        left = m.end();
    }

    spans.push(TokenSpan::new(left, text.len()));
    Ok(spans)
}

pub fn spans_to_relative(spans: &[TokenSpan]) -> Vec<(usize, usize)> {
    let mut prev = 0usize;
    let mut out = Vec::with_capacity(spans.len());
    for span in spans {
        out.push((
            span.start().saturating_sub(prev),
            span.end().saturating_sub(span.start()),
        ));
        prev = span.end();
    }
    out
}

pub fn align_tokens(tokens: &[Token], sentence: &str) -> Result<Vec<TokenSpan>, String> {
    align_token_texts(
        &tokens.iter().map(|t| t.text()).collect::<Vec<_>>(),
        sentence,
    )
}

pub fn align_token_texts(token_texts: &[&str], sentence: &str) -> Result<Vec<TokenSpan>, String> {
    let mut point = 0usize;
    let mut offsets = Vec::with_capacity(token_texts.len());

    for token in token_texts {
        match sentence[point..].find(token) {
            Some(rel_start) => {
                let start = point + rel_start;
                let end = start + token.len();
                offsets.push(TokenSpan::new(start, end));
                point = end;
            }
            None => {
                return Err(format!("substring {token:?} not found in {sentence:?}"));
            }
        }
    }

    Ok(offsets)
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

    #[test]
    fn tokenizer_span_tokenize_uses_token_spans() {
        let t = WhitespaceTokenizer;
        let spans = t.span_tokenize("a bb");
        assert_eq!(spans, vec![TokenSpan::new(0, 1), TokenSpan::new(2, 4)]);
    }

    #[test]
    fn tokenizer_tokenize_sents_applies_per_input() {
        let t = WordPunctTokenizer::new();
        let batch = t.tokenize_sents(&["a!", "b?"]);
        let first: Vec<&str> = batch[0].iter().map(|tk| tk.text()).collect();
        let second: Vec<&str> = batch[1].iter().map(|tk| tk.text()).collect();
        assert_eq!(first, vec!["a", "!"]);
        assert_eq!(second, vec!["b", "?"]);
    }

    #[test]
    fn mwe_tokenizer_merges_longest_matches() {
        let mut t = MWETokenizer::new(
            vec![
                vec!["a".into(), "little".into()],
                vec!["a".into(), "little".into(), "bit".into()],
                vec!["a".into(), "lot".into()],
            ],
            "_",
        );
        t.add_mwe(vec!["in", "spite", "of"]);

        let toks = t.tokenize("In a little or a little bit or a lot in spite of");
        let texts: Vec<&str> = toks.iter().map(|tk| tk.text()).collect();
        assert_eq!(
            texts,
            vec![
                "In",
                "a_little",
                "or",
                "a_little_bit",
                "or",
                "a_lot",
                "in_spite_of"
            ]
        );
    }

    #[test]
    fn helper_function_regexp_tokenize_matches_words() {
        let toks = regexp_tokenize("Good muffins", r"\w+", false, true).unwrap();
        let texts: Vec<&str> = toks.iter().map(|tk| tk.text()).collect();
        assert_eq!(texts, vec!["Good", "muffins"]);
    }

    #[test]
    fn string_span_tokenize_supports_fixed_separator() {
        let spans = string_span_tokenize("Good muffins", " ").expect("valid separator");
        assert_eq!(spans, vec![TokenSpan::new(0, 4), TokenSpan::new(5, 12)]);
    }

    #[test]
    fn regexp_span_tokenize_supports_regex_separator() {
        let spans = regexp_span_tokenize("a\tb c", r"\s+").expect("valid regex");
        assert_eq!(
            spans,
            vec![
                TokenSpan::new(0, 1),
                TokenSpan::new(2, 3),
                TokenSpan::new(4, 5)
            ]
        );
    }

    #[test]
    fn spans_to_relative_converts_offsets() {
        let spans = vec![
            TokenSpan::new(0, 4),
            TokenSpan::new(5, 12),
            TokenSpan::new(13, 17),
        ];
        let rel = spans_to_relative(&spans);
        assert_eq!(rel, vec![(0, 4), (1, 7), (1, 4)]);
    }

    #[test]
    fn align_token_texts_finds_offsets_in_sentence() {
        let spans =
            align_token_texts(&["The", "plane", ","], "The plane,").expect("alignable tokens");
        assert_eq!(
            spans,
            vec![
                TokenSpan::new(0, 3),
                TokenSpan::new(4, 9),
                TokenSpan::new(9, 10)
            ]
        );
    }
}
