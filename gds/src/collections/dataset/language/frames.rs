//! Language persistence frame scaffolds.
//!
//! These frame types are intentionally lightweight. They define database-facing
//! row shapes so the Dataset layer can iterate on storage design without
//! committing to parser/runtime semantics yet.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenRow {
    pub token_id: String,
    pub document_id: String,
    pub text: String,
    pub start: usize,
    pub end: usize,
    pub sentence_index: Option<usize>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TokenFrame {
    rows: Vec<TokenRow>,
}

impl TokenFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<TokenRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: TokenRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[TokenRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StemRow {
    pub stem_id: String,
    pub token_id: String,
    pub stem: String,
    pub kind: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StemFrame {
    rows: Vec<StemRow>,
}

impl StemFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<StemRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: StemRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[StemRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagRow {
    pub tag_id: String,
    pub token_id: String,
    pub tag: String,
    pub tagset: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TagFrame {
    rows: Vec<TagRow>,
}

impl TagFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<TagRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: TagRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[TagRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseRow {
    pub parse_id: String,
    pub document_id: String,
    pub kind: String,
    pub tree: String,
    pub score: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParseFrame {
    rows: Vec<ParseRow>,
}

impl ParseFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<ParseRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: ParseRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[ParseRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeRow {
    pub tree_id: String,
    pub parse_id: Option<String>,
    pub label: String,
    pub parent_tree_id: Option<String>,
    pub ordinal: Option<usize>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TreeFrame {
    rows: Vec<TreeRow>,
}

impl TreeFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<TreeRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: TreeRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[TreeRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VocabularyRow {
    pub term: String,
    pub document_frequency: u64,
    pub token_frequency: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VocabularyFrame {
    rows: Vec<VocabularyRow>,
}

impl VocabularyFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<VocabularyRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: VocabularyRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[VocabularyRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NgramRow {
    pub ngram: String,
    pub n: usize,
    pub count: u64,
    pub left_context: Option<String>,
    pub right_context: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NgramFrame {
    rows: Vec<NgramRow>,
}

impl NgramFrame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<NgramRow>) -> Self {
        Self { rows }
    }

    pub fn push(&mut self, row: NgramRow) {
        self.rows.push(row);
    }

    pub fn rows(&self) -> &[NgramRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_frame_is_row_oriented() {
        let mut frame = TokenFrame::new();
        frame.push(TokenRow {
            token_id: "tok:1".to_string(),
            document_id: "doc:1".to_string(),
            text: "hello".to_string(),
            start: 0,
            end: 5,
            sentence_index: Some(0),
        });
        assert_eq!(frame.len(), 1);
        assert!(!frame.is_empty());
    }

    #[test]
    fn ngram_frame_is_row_oriented() {
        let frame = NgramFrame::from_rows(vec![NgramRow {
            ngram: "hello world".to_string(),
            n: 2,
            count: 3,
            left_context: Some("say".to_string()),
            right_context: None,
        }]);
        assert_eq!(frame.rows()[0].n, 2);
    }
}
