use std::collections::HashSet;

use regex::Regex;

use super::api::ChunkParser;
use super::types::{Chunk, ChunkItem, ChunkTree, TaggedToken};

pub fn accuracy<P: ChunkParser>(chunker: &P, gold: &[ChunkTree]) -> f64 {
    let mut gold_tags = Vec::new();
    let mut test_tags = Vec::new();

    for gold_tree in gold {
        let test_tree = chunker.parse(&gold_tree.flatten());
        gold_tags.extend(tree2conlltags(gold_tree));
        test_tags.extend(tree2conlltags(&test_tree));
    }

    if gold_tags.is_empty() {
        return 1.0;
    }

    let correct = gold_tags
        .iter()
        .zip(test_tags.iter())
        .filter(|(g, t)| g == t)
        .count();
    correct as f64 / gold_tags.len() as f64
}

#[derive(Debug, Clone)]
pub struct ChunkScore {
    correct: HashSet<((usize, usize), Chunk)>,
    guessed: HashSet<((usize, usize), Chunk)>,
    tp: HashSet<((usize, usize), Chunk)>,
    fp: HashSet<((usize, usize), Chunk)>,
    fn_: HashSet<((usize, usize), Chunk)>,
    chunk_label: String,
    count: usize,
    tags_correct: f64,
    tags_total: f64,
    measures_need_update: bool,
}

impl ChunkScore {
    pub fn new(chunk_label: impl Into<String>) -> Self {
        Self {
            correct: HashSet::new(),
            guessed: HashSet::new(),
            tp: HashSet::new(),
            fp: HashSet::new(),
            fn_: HashSet::new(),
            chunk_label: chunk_label.into(),
            count: 0,
            tags_correct: 0.0,
            tags_total: 0.0,
            measures_need_update: false,
        }
    }

    fn update_measures(&mut self) {
        if self.measures_need_update {
            self.tp = self.guessed.intersection(&self.correct).cloned().collect();
            self.fn_ = self.correct.difference(&self.guessed).cloned().collect();
            self.fp = self.guessed.difference(&self.correct).cloned().collect();
            self.measures_need_update = false;
        }
    }

    pub fn score(&mut self, correct: &ChunkTree, guessed: &ChunkTree) {
        self.correct
            .extend(chunksets(correct, self.count, &self.chunk_label));
        self.guessed
            .extend(chunksets(guessed, self.count, &self.chunk_label));
        self.count += 1;
        self.measures_need_update = true;

        let correct_tags = tree2conlltags(correct);
        let guessed_tags = tree2conlltags(guessed);
        self.tags_total += correct_tags.len() as f64;
        self.tags_correct += guessed_tags
            .iter()
            .zip(correct_tags.iter())
            .filter(|(g, c)| g == c)
            .count() as f64;
    }

    pub fn accuracy(&self) -> f64 {
        if self.tags_total == 0.0 {
            1.0
        } else {
            self.tags_correct / self.tags_total
        }
    }

    pub fn precision(&mut self) -> f64 {
        self.update_measures();
        let div = self.tp.len() + self.fp.len();
        if div == 0 {
            0.0
        } else {
            self.tp.len() as f64 / div as f64
        }
    }

    pub fn recall(&mut self) -> f64 {
        self.update_measures();
        let div = self.tp.len() + self.fn_.len();
        if div == 0 {
            0.0
        } else {
            self.tp.len() as f64 / div as f64
        }
    }

    pub fn f_measure(&mut self, alpha: f64) -> f64 {
        let p = self.precision();
        let r = self.recall();
        if p == 0.0 || r == 0.0 {
            0.0
        } else {
            1.0 / (alpha / p + (1.0 - alpha) / r)
        }
    }

    pub fn missed(&mut self) -> Vec<Chunk> {
        self.update_measures();
        self.fn_.iter().map(|(_, c)| c.clone()).collect()
    }

    pub fn incorrect(&mut self) -> Vec<Chunk> {
        self.update_measures();
        self.fp.iter().map(|(_, c)| c.clone()).collect()
    }

    pub fn correct_chunks(&self) -> Vec<Chunk> {
        self.correct.iter().map(|(_, c)| c.clone()).collect()
    }

    pub fn guessed_chunks(&self) -> Vec<Chunk> {
        self.guessed.iter().map(|(_, c)| c.clone()).collect()
    }
}

fn chunksets(tree: &ChunkTree, count: usize, chunk_label: &str) -> HashSet<((usize, usize), Chunk)> {
    let mut pos = 0usize;
    let mut chunks = HashSet::new();
    let label_re = Regex::new(chunk_label).unwrap_or_else(|_| Regex::new(".*").expect("valid regex"));

    for item in &tree.items {
        match item {
            ChunkItem::Token(_) => {
                pos += 1;
            }
            ChunkItem::Chunk(chunk) => {
                if label_re.is_match(&chunk.label) {
                    chunks.insert(((count, pos), chunk.clone()));
                }
                pos += chunk.tokens.len();
            }
        }
    }
    chunks
}

pub fn tagstr2tree(s: &str, chunk_label: &str, root_label: &str, sep: Option<&str>) -> Result<ChunkTree, String> {
    let token_re = Regex::new(r"\[|\]|[^\[\]\s]+").map_err(|e| e.to_string())?;

    let mut stack: Vec<Vec<ChunkItem>> = vec![Vec::new()];
    for m in token_re.find_iter(s) {
        let text = m.as_str();
        if text == "[" {
            if stack.len() != 1 {
                return Err(format!("Unexpected [ at char {}", m.start()));
            }
            stack.push(Vec::new());
        } else if text == "]" {
            if stack.len() != 2 {
                return Err(format!("Unexpected ] at char {}", m.start()));
            }
            let chunk_items = stack.pop().ok_or_else(|| "parser stack underflow".to_string())?;
            let tokens = chunk_items
                .into_iter()
                .map(|item| match item {
                    ChunkItem::Token(tok) => Ok(tok),
                    ChunkItem::Chunk(_) => Err("Nested chunk not allowed in tagstr2tree".to_string()),
                })
                .collect::<Result<Vec<_>, _>>()?;
            stack[0].push(ChunkItem::Chunk(Chunk::new(chunk_label, tokens)));
        } else {
            let tok = if let Some(sep_value) = sep {
                let (word, tag) = str2tuple(text, sep_value);
                TaggedToken::new(word, tag.unwrap_or_default())
            } else {
                TaggedToken::new(text, "")
            };

            let top = stack.last_mut().ok_or_else(|| "parser stack missing".to_string())?;
            top.push(ChunkItem::Token(tok));
        }
    }

    if stack.len() != 1 {
        return Err(format!("Expected ] at char {}", s.len()));
    }

    Ok(ChunkTree::new(root_label, stack.pop().unwrap_or_default()))
}

pub fn conllstr2tree(s: &str, chunk_types: Option<&[&str]>, root_label: &str) -> Result<ChunkTree, String> {
    let line_re = Regex::new(r"(\S+)\s+(\S+)\s+([IOB])-?(\S+)?").map_err(|e| e.to_string())?;

    let mut items = Vec::<ChunkItem>::new();
    let mut current_label: Option<String> = None;
    let mut current_tokens = Vec::<TaggedToken>::new();

    for (lineno, line) in s.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let caps = line_re
            .captures(line)
            .ok_or_else(|| format!("Error on line {}", lineno))?;

        let word = caps.get(1).map(|x| x.as_str()).unwrap_or_default();
        let tag = caps.get(2).map(|x| x.as_str()).unwrap_or_default();
        let mut state = caps.get(3).map(|x| x.as_str()).unwrap_or("O").to_string();
        let chunk_type = caps.get(4).map(|x| x.as_str().to_string()).unwrap_or_default();

        if let Some(allowed) = chunk_types {
            if !allowed.contains(&chunk_type.as_str()) {
                state = "O".to_string();
            }
        }

        let mismatch_i = state == "I" && current_label.as_deref() != Some(chunk_type.as_str());

        if state == "B" || state == "O" || mismatch_i {
            if let Some(label) = current_label.take() {
                items.push(ChunkItem::Chunk(Chunk::new(label, std::mem::take(&mut current_tokens))));
            }
        }

        if state == "B" || mismatch_i {
            current_label = Some(chunk_type.clone());
        }

        let tok = TaggedToken::new(word, tag);
        if current_label.is_some() {
            current_tokens.push(tok);
        } else {
            items.push(ChunkItem::Token(tok));
        }
    }

    if let Some(label) = current_label.take() {
        items.push(ChunkItem::Chunk(Chunk::new(label, current_tokens)));
    }

    Ok(ChunkTree::new(root_label, items))
}

pub fn tree2conlltags(tree: &ChunkTree) -> Vec<(String, String, String)> {
    let mut tags = Vec::new();

    for item in &tree.items {
        match item {
            ChunkItem::Token(tok) => tags.push((tok.word.clone(), tok.tag.clone(), "O".to_string())),
            ChunkItem::Chunk(chunk) => {
                let mut prefix = "B-";
                for tok in &chunk.tokens {
                    tags.push((
                        tok.word.clone(),
                        tok.tag.clone(),
                        format!("{}{}", prefix, chunk.label),
                    ));
                    prefix = "I-";
                }
            }
        }
    }

    tags
}

pub fn conlltags2tree(
    sentence: &[(String, String, Option<String>)],
    chunk_types: Option<&[&str]>,
    root_label: &str,
    strict: bool,
) -> Result<ChunkTree, String> {
    let mut items = Vec::<ChunkItem>::new();
    let mut current_label: Option<String> = None;
    let mut current_tokens = Vec::<TaggedToken>::new();

    for (word, postag, chunktag_opt) in sentence {
        let chunktag = chunktag_opt.clone().unwrap_or_else(|| "O".to_string());

        if chunktag == "O" {
            if let Some(label) = current_label.take() {
                items.push(ChunkItem::Chunk(Chunk::new(label, std::mem::take(&mut current_tokens))));
            }
            items.push(ChunkItem::Token(TaggedToken::new(word.clone(), postag.clone())));
            continue;
        }

        if let Some(rest) = chunktag.strip_prefix("B-") {
            if let Some(label) = current_label.take() {
                items.push(ChunkItem::Chunk(Chunk::new(label, std::mem::take(&mut current_tokens))));
            }

            if chunk_types.map(|types| types.contains(&rest)).unwrap_or(true) {
                current_label = Some(rest.to_string());
                current_tokens.push(TaggedToken::new(word.clone(), postag.clone()));
            } else {
                items.push(ChunkItem::Token(TaggedToken::new(word.clone(), postag.clone())));
            }
            continue;
        }

        if let Some(rest) = chunktag.strip_prefix("I-") {
            if current_label.as_deref() != Some(rest) {
                if strict {
                    return Err("Bad conll tag sequence".to_string());
                }

                if let Some(label) = current_label.take() {
                    items.push(ChunkItem::Chunk(Chunk::new(label, std::mem::take(&mut current_tokens))));
                }
                current_label = Some(rest.to_string());
            }
            current_tokens.push(TaggedToken::new(word.clone(), postag.clone()));
            continue;
        }

        return Err(format!("Bad conll tag {chunktag:?}"));
    }

    if let Some(label) = current_label.take() {
        items.push(ChunkItem::Chunk(Chunk::new(label, current_tokens)));
    }

    Ok(ChunkTree::new(root_label, items))
}

pub fn tree2conllstr(tree: &ChunkTree) -> String {
    tree2conlltags(tree)
        .into_iter()
        .map(|(w, t, c)| format!("{w} {t} {c}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn str2tuple(text: &str, sep: &str) -> (String, Option<String>) {
    if let Some((word, tag)) = text.rsplit_once(sep) {
        (word.to_string(), Some(tag.to_string()))
    } else {
        (text.to_string(), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct PassthroughChunker;

    impl ChunkParser for PassthroughChunker {
        fn parse(&self, tokens: &[TaggedToken]) -> ChunkTree {
            ChunkTree::new(
                "S",
                tokens.iter().cloned().map(ChunkItem::Token).collect(),
            )
        }
    }

    #[test]
    fn tagstr_and_conll_roundtrip() {
        let text = "[ the/DT little/JJ cat/NN ] sat/VBD";
        let tree = tagstr2tree(text, "NP", "S", Some("/")).expect("tagstr parse");
        let conll = tree2conllstr(&tree);
        let reparsed = conllstr2tree(&conll, None, "S").expect("conll parse");
        assert_eq!(tree2conlltags(&tree), tree2conlltags(&reparsed));
    }

    #[test]
    fn chunk_score_metrics_work() {
        let gold = conllstr2tree("the DT B-NP\ncat NN I-NP\nsat VBD O", None, "S")
            .expect("gold parse");
        let guess = conllstr2tree("the DT B-NP\ncat NN I-NP\nsat VBD O", None, "S")
            .expect("guess parse");

        let mut score = ChunkScore::new(".*");
        score.score(&gold, &guess);

        assert!((score.precision() - 1.0).abs() < 1e-12);
        assert!((score.recall() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn parser_accuracy_runs() {
        let gold_tree = conllstr2tree("the DT B-NP\ncat NN I-NP\nsat VBD O", None, "S")
            .expect("gold parse");
        let acc = accuracy(&PassthroughChunker, &[gold_tree]);
        assert!(acc <= 1.0);
    }
}
