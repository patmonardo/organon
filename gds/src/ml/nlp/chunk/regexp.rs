use regex::Regex;

use super::api::ChunkParser;
use super::types::{Chunk, ChunkItem, ChunkTree, TaggedToken};

#[derive(Debug, Clone)]
pub struct ChunkString {
    root_label: String,
    pieces: Vec<TaggedToken>,
    repr: String,
    debug_level: u8,
}

impl ChunkString {
    pub const IN_CHUNK_PATTERN: &'static str = r"(?=[^\{]*\})";
    pub const IN_STRIP_PATTERN: &'static str = r"(?=[^\}]*(\{|$))";

    pub fn from_tokens(
        root_label: impl Into<String>,
        tokens: &[TaggedToken],
        debug_level: u8,
    ) -> Self {
        let pieces = tokens.to_vec();
        Self {
            root_label: root_label.into(),
            repr: Self::repr_from_chunked(&pieces, &vec![false; pieces.len()]),
            pieces,
            debug_level,
        }
    }

    pub fn from_chunk_tree(tree: &ChunkTree, debug_level: u8) -> Self {
        let mut pieces = Vec::new();
        let mut repr = String::new();

        for item in &tree.items {
            match item {
                ChunkItem::Token(tok) => {
                    pieces.push(tok.clone());
                    repr.push('<');
                    repr.push_str(&tok.tag);
                    repr.push('>');
                }
                ChunkItem::Chunk(chunk) => {
                    repr.push('{');
                    for tok in &chunk.tokens {
                        pieces.push(tok.clone());
                        repr.push('<');
                        repr.push_str(&tok.tag);
                        repr.push('>');
                    }
                    repr.push('}');
                }
            }
        }

        Self {
            root_label: tree.root_label.clone(),
            pieces,
            repr,
            debug_level,
        }
    }

    pub fn xform(&mut self, regexp: &Regex, repl: &str) -> Result<(), String> {
        let before = self.representation();
        let mut after = regexp.replace_all(&before, repl).to_string();
        while after.contains("{}") {
            after = after.replace("{}", "");
        }

        self.verify_repr(&after)?;
        self.repr = after;

        if self.debug_level >= 2 {
            self.verify_repr(&self.representation())?;
        }

        Ok(())
    }

    pub fn to_chunk_tree(&self, chunk_label: &str) -> ChunkTree {
        let mut items = Vec::new();
        let token_re = Regex::new(r"\{|\}|<[^<>]+>").expect("valid token regex");
        let mut in_chunk = false;
        let mut chunk_tokens = Vec::new();
        let mut piece_idx = 0usize;

        for m in token_re.find_iter(&self.repr) {
            let tok = m.as_str();
            if tok == "{" {
                in_chunk = true;
            } else if tok == "}" {
                in_chunk = false;
                items.push(ChunkItem::Chunk(Chunk::new(
                    chunk_label,
                    std::mem::take(&mut chunk_tokens),
                )));
            } else if piece_idx < self.pieces.len() {
                let piece = self.pieces[piece_idx].clone();
                piece_idx += 1;
                if in_chunk {
                    chunk_tokens.push(piece);
                } else {
                    items.push(ChunkItem::Token(piece));
                }
            }
        }

        ChunkTree::new(self.root_label.clone(), items)
    }

    pub fn representation(&self) -> String {
        self.repr.clone()
    }

    fn verify_repr(&self, repr: &str) -> Result<(), String> {
        let token_re = Regex::new(r"\{|\}|<[^<>]+>").map_err(|e| e.to_string())?;

        let mut depth = 0i32;
        let mut tags = Vec::new();

        for m in token_re.find_iter(repr) {
            let tok = m.as_str();
            if tok == "{" {
                depth += 1;
                if depth > 1 {
                    return Err("Nested chunks are not allowed".to_string());
                }
            } else if tok == "}" {
                depth -= 1;
                if depth < 0 {
                    return Err("Mismatched chunk brackets".to_string());
                }
            } else {
                tags.push(
                    tok.trim_start_matches('<')
                        .trim_end_matches('>')
                        .to_string(),
                );
            }
        }

        if depth != 0 {
            return Err("Unbalanced chunk brackets".to_string());
        }

        let expected: Vec<String> = self.pieces.iter().map(|t| t.tag.clone()).collect();
        if tags != expected {
            return Err("Tag sequence was modified by transformation".to_string());
        }

        Ok(())
    }

    fn repr_from_chunked(pieces: &[TaggedToken], chunked: &[bool]) -> String {
        let mut out = String::new();

        for (idx, tok) in pieces.iter().enumerate() {
            let prev = idx
                .checked_sub(1)
                .and_then(|i| chunked.get(i))
                .copied()
                .unwrap_or(false);
            let curr = chunked[idx];

            if curr && !prev {
                out.push('{');
            }

            out.push('<');
            out.push_str(&tok.tag);
            out.push('>');

            let next = chunked.get(idx + 1).copied().unwrap_or(false);
            if curr && !next {
                out.push('}');
            }
        }

        out
    }
}

#[derive(Debug, Clone)]
pub struct RegexpChunkRule {
    regexp: Regex,
    repl: String,
    descr: String,
}

impl RegexpChunkRule {
    pub fn new(regexp: Regex, repl: impl Into<String>, descr: impl Into<String>) -> Self {
        Self {
            regexp,
            repl: repl.into(),
            descr: descr.into(),
        }
    }

    pub fn apply(&self, chunkstr: &mut ChunkString) -> Result<(), String> {
        chunkstr.xform(&self.regexp, &self.repl)
    }

    pub fn descr(&self) -> &str {
        &self.descr
    }

    pub fn from_tag_rule(rule: &str, descr: impl Into<String>) -> Result<Self, String> {
        let trimmed = rule.trim();
        let description = descr.into();

        if trimmed.starts_with('{') && trimmed.ends_with('}') && trimmed.len() >= 2 {
            let inner = &trimmed[1..trimmed.len() - 1];
            return ChunkRule::new(inner, description).map(ChunkRule::into_rule);
        }

        if trimmed.starts_with('}') && trimmed.ends_with('{') && trimmed.len() >= 2 {
            let inner = &trimmed[1..trimmed.len() - 1];
            return StripRule::new(inner, description).map(StripRule::into_rule);
        }

        Err(format!("Unsupported tag rule syntax: {trimmed}"))
    }
}

#[derive(Debug, Clone)]
pub struct ChunkRule {
    inner: RegexpChunkRule,
}

impl ChunkRule {
    pub fn new(tag_pattern: &str, descr: impl Into<String>) -> Result<Self, String> {
        let re_pattern = tag_pattern2re_pattern(tag_pattern)?;
        let regex = Regex::new(&re_pattern).map_err(|e| e.to_string())?;
        Ok(Self {
            inner: RegexpChunkRule::new(regex, "{$0}", descr),
        })
    }

    pub fn into_rule(self) -> RegexpChunkRule {
        self.inner
    }
}

#[derive(Debug, Clone)]
pub struct StripRule {
    inner: RegexpChunkRule,
}

impl StripRule {
    pub fn new(tag_pattern: &str, descr: impl Into<String>) -> Result<Self, String> {
        let re_pattern = tag_pattern2re_pattern(tag_pattern)?;
        let pattern = format!(
            r"\{{(?P<pre>[^{{}}]*?)(?P<target>{})(?P<post>[^{{}}]*?)\}}",
            re_pattern
        );
        let regex = Regex::new(&pattern).map_err(|e| e.to_string())?;
        Ok(Self {
            inner: RegexpChunkRule::new(regex, "{$pre}$target{$post}", descr),
        })
    }

    pub fn into_rule(self) -> RegexpChunkRule {
        self.inner
    }
}

#[derive(Debug, Clone)]
pub struct UnChunkRule {
    inner: RegexpChunkRule,
}

impl UnChunkRule {
    pub fn new(tag_pattern: &str, descr: impl Into<String>) -> Result<Self, String> {
        let re_pattern = tag_pattern2re_pattern(tag_pattern)?;
        let pattern = format!(r"\{{({})\}}", re_pattern);
        let regex = Regex::new(&pattern).map_err(|e| e.to_string())?;
        Ok(Self {
            inner: RegexpChunkRule::new(regex, "$1", descr),
        })
    }

    pub fn into_rule(self) -> RegexpChunkRule {
        self.inner
    }
}

#[derive(Debug, Clone)]
pub struct MergeRule {
    inner: RegexpChunkRule,
}

impl MergeRule {
    pub fn new(
        left_tag_pattern: &str,
        right_tag_pattern: &str,
        descr: impl Into<String>,
    ) -> Result<Self, String> {
        let left_re = tag_pattern2re_pattern(left_tag_pattern)?;
        let right_re = tag_pattern2re_pattern(right_tag_pattern)?;
        let pattern = format!(
            r"\{{(?P<left>[^{{}}]*?{})\}}\{{(?P<right>{}[^{{}}]*?)\}}",
            left_re, right_re
        );
        let regex = Regex::new(&pattern).map_err(|e| e.to_string())?;
        Ok(Self {
            inner: RegexpChunkRule::new(regex, "{${left}${right}}", descr),
        })
    }

    pub fn into_rule(self) -> RegexpChunkRule {
        self.inner
    }
}

#[derive(Debug, Clone)]
pub struct SplitRule {
    inner: RegexpChunkRule,
}

impl SplitRule {
    pub fn new(
        left_tag_pattern: &str,
        right_tag_pattern: &str,
        descr: impl Into<String>,
    ) -> Result<Self, String> {
        let left_re = tag_pattern2re_pattern(left_tag_pattern)?;
        let right_re = tag_pattern2re_pattern(right_tag_pattern)?;
        let pattern = format!(
            r"\{{(?P<pre>[^{{}}]*?)(?P<left>{})(?P<right>{})(?P<post>[^{{}}]*?)\}}",
            left_re, right_re
        );
        let regex = Regex::new(&pattern).map_err(|e| e.to_string())?;
        Ok(Self {
            inner: RegexpChunkRule::new(regex, "{${pre}${left}}{${right}${post}}", descr),
        })
    }

    pub fn into_rule(self) -> RegexpChunkRule {
        self.inner
    }
}

#[derive(Debug, Clone)]
pub struct ExpandLeftRule {
    inner: RegexpChunkRule,
}

impl ExpandLeftRule {
    pub fn new(
        left_tag_pattern: &str,
        right_tag_pattern: &str,
        descr: impl Into<String>,
    ) -> Result<Self, String> {
        let left_re = tag_pattern2re_pattern(left_tag_pattern)?;
        let right_re = tag_pattern2re_pattern(right_tag_pattern)?;
        let pattern = format!(r"(?P<left>{})\{{(?P<right>{})", left_re, right_re);
        let regex = Regex::new(&pattern).map_err(|e| e.to_string())?;
        Ok(Self {
            inner: RegexpChunkRule::new(regex, "{${left}${right}", descr),
        })
    }

    pub fn into_rule(self) -> RegexpChunkRule {
        self.inner
    }
}

#[derive(Debug, Clone)]
pub struct ExpandRightRule {
    inner: RegexpChunkRule,
}

impl ExpandRightRule {
    pub fn new(
        left_tag_pattern: &str,
        right_tag_pattern: &str,
        descr: impl Into<String>,
    ) -> Result<Self, String> {
        let left_re = tag_pattern2re_pattern(left_tag_pattern)?;
        let right_re = tag_pattern2re_pattern(right_tag_pattern)?;
        let pattern = format!(r"(?P<left>{})\}}(?P<right>{})", left_re, right_re);
        let regex = Regex::new(&pattern).map_err(|e| e.to_string())?;
        Ok(Self {
            inner: RegexpChunkRule::new(regex, "${left}${right}}", descr),
        })
    }

    pub fn into_rule(self) -> RegexpChunkRule {
        self.inner
    }
}

#[derive(Debug, Clone)]
pub struct RegexpChunkParser {
    rules: Vec<RegexpChunkRule>,
    chunk_label: String,
    root_label: String,
    trace: u8,
}

impl RegexpChunkParser {
    pub fn new(
        rules: Vec<RegexpChunkRule>,
        chunk_label: impl Into<String>,
        root_label: impl Into<String>,
        trace: u8,
    ) -> Self {
        Self {
            rules,
            chunk_label: chunk_label.into(),
            root_label: root_label.into(),
            trace,
        }
    }

    pub fn rules(&self) -> &[RegexpChunkRule] {
        &self.rules
    }

    pub fn from_grammar(
        grammar: &str,
        root_label: impl Into<String>,
        trace: u8,
    ) -> Result<Self, String> {
        let mut chunk_label: Option<String> = None;
        let mut rules = Vec::new();

        for (idx, raw_line) in grammar.lines().enumerate() {
            let line_no = idx + 1;
            let without_comment = raw_line.split('#').next().unwrap_or_default().trim();
            if without_comment.is_empty() {
                continue;
            }

            if let Some(prefix) = without_comment.strip_suffix(':') {
                let label = prefix.trim();
                if label.is_empty() {
                    return Err(format!("Empty chunk label at grammar line {line_no}"));
                }
                if chunk_label.is_some() {
                    return Err(format!(
                        "Multiple chunk labels are not supported (line {line_no})"
                    ));
                }
                chunk_label = Some(label.to_string());
                continue;
            }

            if chunk_label.is_none() {
                return Err(format!(
                    "Rule appears before chunk label at grammar line {line_no}"
                ));
            }

            let (rule, descr) = parse_grammar_rule(without_comment)?;
            let built = RegexpChunkRule::from_tag_rule(&rule, descr)
                .map_err(|e| format!("Grammar line {line_no}: {e}"))?;
            rules.push(built);
        }

        let label = chunk_label.ok_or_else(|| "Missing chunk label in grammar".to_string())?;
        Ok(Self::new(rules, label, root_label, trace))
    }
}

fn parse_grammar_rule(line: &str) -> Result<(String, String), String> {
    if let Some((rule, comment)) = line.split_once("//") {
        let cleaned_rule = rule.trim();
        if cleaned_rule.is_empty() {
            return Err("Empty rule in grammar line".to_string());
        }
        return Ok((cleaned_rule.to_string(), comment.trim().to_string()));
    }

    if line.is_empty() {
        return Err("Empty rule in grammar line".to_string());
    }

    Ok((line.to_string(), String::new()))
}

impl ChunkParser for RegexpChunkParser {
    fn parse(&self, tokens: &[TaggedToken]) -> ChunkTree {
        let mut chunkstr = ChunkString::from_tokens(self.root_label.clone(), tokens, 1);
        for rule in &self.rules {
            let _ = rule.apply(&mut chunkstr);
            if self.trace > 0 {
                let _ = chunkstr.representation();
            }
        }
        chunkstr.to_chunk_tree(&self.chunk_label)
    }
}

pub fn tag_pattern2re_pattern(tag_pattern: &str) -> Result<String, String> {
    let mut pattern = tag_pattern.replace(char::is_whitespace, "");
    pattern = pattern.replace('<', "(<(");
    pattern = pattern.replace('>', ")>)");

    let valid = Regex::new(r"^((([^\{\}<>]|\{\d+,?\}|\{\d*,\d+\})+|<[^\{\}<>]+>)*)$")
        .map_err(|e| e.to_string())?;
    if !valid.is_match(&pattern) {
        return Err(format!("Bad tag pattern: {tag_pattern}"));
    }

    let mut out = String::new();
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        let ch = chars[i];
        if ch == '.' {
            out.push_str("[^{}<>]");
        } else {
            out.push(ch);
        }
        i += 1;
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_rule_builds_chunks() {
        let tokens = vec![
            TaggedToken::new("the", "DT"),
            TaggedToken::new("little", "JJ"),
            TaggedToken::new("cat", "NN"),
            TaggedToken::new("sat", "VBD"),
        ];

        let rule = ChunkRule::new("<DT>?<JJ>*<NN>", "NP rule")
            .expect("valid chunk rule")
            .into_rule();
        let parser = RegexpChunkParser::new(vec![rule], "NP", "S", 0);

        let tree = parser.parse(&tokens);
        assert_eq!(tree.items.len(), 2);
        match &tree.items[0] {
            ChunkItem::Chunk(chunk) => {
                assert_eq!(chunk.label, "NP");
                assert_eq!(chunk.tokens.len(), 3);
            }
            _ => panic!("expected leading NP chunk"),
        }
    }

    #[test]
    fn tag_pattern_conversion_works() {
        let pattern = tag_pattern2re_pattern("<NN.*>").expect("valid pattern");
        let regex = Regex::new(&pattern).expect("regex compile");
        assert!(regex.is_match("<NNP>"));
        assert!(regex.is_match("<NNS>"));
    }

    #[test]
    fn strip_rule_splits_chunk_content() {
        let tokens = vec![
            TaggedToken::new("the", "DT"),
            TaggedToken::new("little", "JJ"),
            TaggedToken::new("cat", "NN"),
        ];

        let chunk_rule = ChunkRule::new("<DT><JJ><NN>", "chunk triad")
            .expect("chunk rule")
            .into_rule();
        let strip_rule = StripRule::new("<JJ>", "strip adjectives")
            .expect("strip rule")
            .into_rule();

        let mut chunkstr = ChunkString::from_tokens("S", &tokens, 1);
        chunk_rule.apply(&mut chunkstr).expect("apply chunk rule");
        strip_rule.apply(&mut chunkstr).expect("apply strip rule");

        assert_eq!(chunkstr.representation(), "{<DT>}<JJ>{<NN>}");
    }

    #[test]
    fn unchunk_rule_removes_matching_chunk() {
        let tokens = vec![
            TaggedToken::new("the", "DT"),
            TaggedToken::new("cat", "NN"),
            TaggedToken::new("sat", "VBD"),
        ];

        let chunk_rule = ChunkRule::new("<DT><NN>", "chunk noun phrase")
            .expect("chunk rule")
            .into_rule();
        let unchunk_rule = UnChunkRule::new("<DT><NN>", "unchunk that noun phrase")
            .expect("unchunk rule")
            .into_rule();

        let parser = RegexpChunkParser::new(vec![chunk_rule, unchunk_rule], "NP", "S", 0);
        let tree = parser.parse(&tokens);

        assert!(tree
            .items
            .iter()
            .all(|item| matches!(item, ChunkItem::Token(_))));
    }

    #[test]
    fn merge_rule_joins_adjacent_chunks() {
        let tokens = vec![
            TaggedToken::new("the", "DT"),
            TaggedToken::new("cat", "NN"),
            TaggedToken::new("small", "JJ"),
            TaggedToken::new("dog", "NN"),
        ];

        let left_chunk = ChunkRule::new("<DT><NN>", "left chunk")
            .expect("left chunk rule")
            .into_rule();
        let right_chunk = ChunkRule::new("<JJ><NN>", "right chunk")
            .expect("right chunk rule")
            .into_rule();
        let merge_rule = MergeRule::new("<NN>", "<JJ>", "merge chunk boundary")
            .expect("merge rule")
            .into_rule();

        let mut chunkstr = ChunkString::from_tokens("S", &tokens, 1);
        left_chunk.apply(&mut chunkstr).expect("apply left chunk");
        right_chunk.apply(&mut chunkstr).expect("apply right chunk");
        merge_rule.apply(&mut chunkstr).expect("apply merge rule");

        assert_eq!(chunkstr.representation(), "{<DT><NN><JJ><NN>}");
    }

    #[test]
    fn split_rule_splits_single_chunk() {
        let tokens = vec![
            TaggedToken::new("the", "DT"),
            TaggedToken::new("cat", "NN"),
            TaggedToken::new("small", "JJ"),
            TaggedToken::new("dog", "NN"),
        ];

        let chunk_rule = ChunkRule::new("<DT><NN><JJ><NN>", "chunk all")
            .expect("chunk rule")
            .into_rule();
        let split_builder =
            SplitRule::new("<NN>", "<JJ>", "split at NN/JJ boundary").expect("split rule");

        let mut chunkstr = ChunkString::from_tokens("S", &tokens, 1);
        chunk_rule.apply(&mut chunkstr).expect("apply chunk rule");
        assert!(
            split_builder
                .inner
                .regexp
                .is_match(&chunkstr.representation()),
            "split regex should match current chunk representation"
        );

        let split_rule = split_builder.into_rule();
        split_rule.apply(&mut chunkstr).expect("apply split rule");

        assert_eq!(chunkstr.representation(), "{<DT><NN>}{<JJ><NN>}");
    }

    #[test]
    fn expand_left_rule_absorbs_left_strip_tokens() {
        let tokens = vec![
            TaggedToken::new("the", "DT"),
            TaggedToken::new("cat", "NN"),
            TaggedToken::new("sat", "VBD"),
        ];

        let chunk_rule = ChunkRule::new("<NN>", "chunk noun")
            .expect("chunk rule")
            .into_rule();
        let expand_left = ExpandLeftRule::new("<DT>", "<NN>", "expand chunk left")
            .expect("expand-left rule")
            .into_rule();

        let mut chunkstr = ChunkString::from_tokens("S", &tokens, 1);
        chunk_rule.apply(&mut chunkstr).expect("apply chunk rule");
        expand_left
            .apply(&mut chunkstr)
            .expect("apply expand-left rule");

        assert_eq!(chunkstr.representation(), "{<DT><NN>}<VBD>");
    }

    #[test]
    fn expand_right_rule_absorbs_right_strip_tokens() {
        let tokens = vec![
            TaggedToken::new("the", "DT"),
            TaggedToken::new("cat", "NN"),
            TaggedToken::new("sat", "VBD"),
        ];

        let chunk_rule = ChunkRule::new("<NN>", "chunk noun")
            .expect("chunk rule")
            .into_rule();
        let expand_right = ExpandRightRule::new("<NN>", "<VBD>", "expand chunk right")
            .expect("expand-right rule")
            .into_rule();

        let mut chunkstr = ChunkString::from_tokens("S", &tokens, 1);
        chunk_rule.apply(&mut chunkstr).expect("apply chunk rule");
        expand_right
            .apply(&mut chunkstr)
            .expect("apply expand-right rule");

        assert_eq!(chunkstr.representation(), "<DT>{<NN><VBD>}");
    }

    #[test]
    fn from_tag_rule_builds_chunk_and_strip() {
        let chunk_rule = RegexpChunkRule::from_tag_rule("{<DT><NN>}", "chunk")
            .expect("chunk rule from tag rule");
        let strip_rule =
            RegexpChunkRule::from_tag_rule("}<DT>{", "strip").expect("strip rule from tag rule");

        let tokens = vec![TaggedToken::new("the", "DT"), TaggedToken::new("cat", "NN")];
        let mut chunkstr = ChunkString::from_tokens("S", &tokens, 1);
        chunk_rule.apply(&mut chunkstr).expect("apply chunk rule");
        strip_rule.apply(&mut chunkstr).expect("apply strip rule");

        assert_eq!(chunkstr.representation(), "<DT>{<NN>}");
    }

    #[test]
    fn parser_from_grammar_applies_rules() {
        let grammar = r#"
            NP:
              {<DT><JJ><NN>}   // chunk simple NP
              }<JJ>{           // strip adjectives
        "#;

        let parser =
            RegexpChunkParser::from_grammar(grammar, "S", 0).expect("build parser from grammar");

        let tokens = vec![
            TaggedToken::new("the", "DT"),
            TaggedToken::new("little", "JJ"),
            TaggedToken::new("cat", "NN"),
        ];

        let tree = parser.parse(&tokens);
        assert_eq!(tree.items.len(), 3);
        assert!(matches!(tree.items[0], ChunkItem::Chunk(_)));
        assert!(matches!(tree.items[1], ChunkItem::Token(_)));
        assert!(matches!(tree.items[2], ChunkItem::Chunk(_)));
    }
}
