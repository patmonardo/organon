#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaggedToken {
    pub word: String,
    pub tag: String,
}

impl TaggedToken {
    pub fn new(word: impl Into<String>, tag: impl Into<String>) -> Self {
        Self {
            word: word.into(),
            tag: tag.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Chunk {
    pub label: String,
    pub tokens: Vec<TaggedToken>,
}

impl Chunk {
    pub fn new(label: impl Into<String>, tokens: Vec<TaggedToken>) -> Self {
        Self {
            label: label.into(),
            tokens,
        }
    }

    pub fn leaves(&self) -> &[TaggedToken] {
        &self.tokens
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChunkItem {
    Token(TaggedToken),
    Chunk(Chunk),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChunkTree {
    pub root_label: String,
    pub items: Vec<ChunkItem>,
}

impl ChunkTree {
    pub fn new(root_label: impl Into<String>, items: Vec<ChunkItem>) -> Self {
        Self {
            root_label: root_label.into(),
            items,
        }
    }

    pub fn leaves(&self) -> Vec<TaggedToken> {
        let mut out = Vec::new();
        for item in &self.items {
            match item {
                ChunkItem::Token(tok) => out.push(tok.clone()),
                ChunkItem::Chunk(chunk) => out.extend(chunk.tokens.clone()),
            }
        }
        out
    }

    pub fn flatten(&self) -> Vec<TaggedToken> {
        self.leaves()
    }
}
