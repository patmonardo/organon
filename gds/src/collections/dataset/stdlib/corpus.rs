//! Dataset corpus readers (resource-backed, minimal NLTK-style surface).

use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;

use crate::collections::dataset::error::DatasetIoError;
use crate::collections::dataset::functions::tree::parse::parse_bracketed;
use crate::collections::dataset::parse::{Parse, ParseForest, ParseKind};
use crate::collections::dataset::stdlib::corpus_util::{
    concat as corpus_concat, read_blankline_block, read_line_block, read_markup_text_block,
    read_sexpr_block_default, read_wordpunct_block, BlockReader, ConcatenatedCorpusView,
    StreamBackedCorpusView,
};
use crate::collections::dataset::stdlib::resources::{
    fetch_resource, list_resources, DatasetResource,
};
use crate::collections::dataset::token::Token;
use crate::collections::dataset::tokenizer::{
    LineBlankMode, LineTokenizer, Tokenizer, WordPunctTokenizer,
};
use crate::collections::dataset::tree::TreeValue;

#[derive(Debug, Clone)]
pub struct CorpusFiles {
    root: PathBuf,
    fileids: Vec<String>,
}

impl CorpusFiles {
    pub fn new(root: impl Into<PathBuf>, fileids: Vec<String>) -> Self {
        Self {
            root: root.into(),
            fileids,
        }
    }

    pub fn from_root(
        root: impl Into<PathBuf>,
        pattern: Option<&str>,
    ) -> Result<Self, DatasetIoError> {
        let root = root.into();
        let regex = match pattern {
            Some(value) => Some(Regex::new(value).map_err(|e| {
                DatasetIoError::Unsupported(format!("invalid fileid pattern: {e}"))
            })?),
            None => None,
        };
        let fileids = discover_fileids(&root, regex.as_ref())?;
        Ok(Self { root, fileids })
    }

    pub fn from_resource(
        resource: &CorpusResource,
        pattern: Option<&str>,
    ) -> Result<Self, DatasetIoError> {
        Self::from_root(resource.data_dir.clone(), pattern)
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn fileids(&self) -> &[String] {
        &self.fileids
    }

    pub fn abspath(&self, fileid: &str) -> Result<PathBuf, DatasetIoError> {
        ensure_safe_fileid(fileid)?;
        Ok(self.root.join(fileid))
    }

    pub fn read_text(&self, fileid: &str) -> Result<String, DatasetIoError> {
        let path = self.abspath(fileid)?;
        fs::read_to_string(&path).map_err(|e| DatasetIoError::Io(e.to_string()))
    }
}

pub trait CorpusReader {
    fn files(&self) -> &CorpusFiles;

    fn root(&self) -> &Path {
        self.files().root()
    }

    fn fileids(&self) -> &[String] {
        self.files().fileids()
    }

    fn open_text(&self, fileid: &str) -> Result<String, DatasetIoError> {
        self.files().read_text(fileid)
    }

    fn raw(&self, fileids: Option<&[String]>) -> Result<String, DatasetIoError> {
        let fileids = fileids.unwrap_or_else(|| self.fileids());
        let mut out = String::new();
        for fileid in fileids {
            let text = self.open_text(fileid)?;
            out.push_str(&text);
        }
        Ok(out)
    }
}

#[derive(Debug, Clone)]
pub struct CorpusResource {
    resource: DatasetResource,
    data_dir: PathBuf,
}

impl CorpusResource {
    pub fn from_name(
        name: &str,
        data_home_override: Option<impl AsRef<Path>>,
    ) -> Result<Self, DatasetIoError> {
        let report = fetch_resource(name, data_home_override)?;
        let resource = list_resources()
            .iter()
            .find(|r| r.name == name)
            .copied()
            .ok_or_else(|| DatasetIoError::Unsupported(format!("unknown resource: {name}")))?;
        Ok(Self {
            resource,
            data_dir: report.data_dir,
        })
    }

    pub fn resource(&self) -> &DatasetResource {
        &self.resource
    }

    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }
}

pub struct PlaintextCorpusReader {
    files: CorpusFiles,
    word_tokenizer: Box<dyn Tokenizer>,
    line_tokenizer: LineTokenizer,
}

impl PlaintextCorpusReader {
    pub fn new(files: CorpusFiles) -> Self {
        Self {
            files,
            word_tokenizer: Box::new(WordPunctTokenizer::new()),
            line_tokenizer: LineTokenizer::new(LineBlankMode::DiscardEof),
        }
    }

    pub fn with_word_tokenizer(mut self, tokenizer: impl Tokenizer + 'static) -> Self {
        self.word_tokenizer = Box::new(tokenizer);
        self
    }

    pub fn words(&self, fileids: Option<&[String]>) -> Result<Vec<String>, DatasetIoError> {
        let view = self.wordpunct_view(fileids)?;
        let mut iter = view.into_iter();
        let mut out = Vec::new();
        while let Some(token) = iter.next()? {
            out.push(token);
        }
        Ok(out)
    }

    pub fn sents(&self, fileids: Option<&[String]>) -> Result<Vec<Vec<String>>, DatasetIoError> {
        let view = self.line_view(fileids)?;
        let mut iter = view.into_iter();
        let mut out = Vec::new();
        while let Some(line) = iter.next()? {
            out.push(tokens_to_texts(self.word_tokenizer.tokenize(&line)));
        }
        Ok(out)
    }

    pub fn paras(
        &self,
        fileids: Option<&[String]>,
    ) -> Result<Vec<Vec<Vec<String>>>, DatasetIoError> {
        let view = self.blankline_view(fileids)?;
        let mut iter = view.into_iter();
        let mut out = Vec::new();
        while let Some(para) = iter.next()? {
            let lines = tokens_to_texts(self.line_tokenizer.tokenize(&para));
            let mut sent_out = Vec::with_capacity(lines.len());
            for line in lines {
                sent_out.push(tokens_to_texts(self.word_tokenizer.tokenize(&line)));
            }
            out.push(sent_out);
        }
        Ok(out)
    }

    pub fn wordpunct_view(
        &self,
        fileids: Option<&[String]>,
    ) -> Result<ConcatenatedCorpusView<String>, DatasetIoError> {
        Ok(corpus_concat(build_views(
            &self.files,
            fileids,
            read_wordpunct_block,
        )?))
    }

    pub fn line_view(
        &self,
        fileids: Option<&[String]>,
    ) -> Result<ConcatenatedCorpusView<String>, DatasetIoError> {
        Ok(corpus_concat(build_views(
            &self.files,
            fileids,
            read_line_block,
        )?))
    }

    pub fn blankline_view(
        &self,
        fileids: Option<&[String]>,
    ) -> Result<ConcatenatedCorpusView<String>, DatasetIoError> {
        Ok(corpus_concat(build_views(
            &self.files,
            fileids,
            read_blankline_block,
        )?))
    }
}

impl CorpusReader for PlaintextCorpusReader {
    fn files(&self) -> &CorpusFiles {
        &self.files
    }
}

#[derive(Debug, Clone)]
pub struct WordListCorpusReader {
    files: CorpusFiles,
}

impl WordListCorpusReader {
    pub fn new(files: CorpusFiles) -> Self {
        Self { files }
    }

    pub fn words(
        &self,
        fileids: Option<&[String]>,
        ignore_lines_startswith: &str,
    ) -> Result<Vec<String>, DatasetIoError> {
        let view = self.line_view(fileids)?;
        let mut iter = view.into_iter();
        let mut out = Vec::new();
        while let Some(line) = iter.next()? {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with(ignore_lines_startswith) {
                continue;
            }
            out.push(trimmed.to_string());
        }
        Ok(out)
    }

    pub fn line_view(
        &self,
        fileids: Option<&[String]>,
    ) -> Result<ConcatenatedCorpusView<String>, DatasetIoError> {
        Ok(corpus_concat(build_views(
            &self.files,
            fileids,
            read_line_block,
        )?))
    }
}

impl CorpusReader for WordListCorpusReader {
    fn files(&self) -> &CorpusFiles {
        &self.files
    }
}

#[derive(Debug)]
pub struct BracketedCorpusReader {
    files: CorpusFiles,
}

impl BracketedCorpusReader {
    pub fn new(files: CorpusFiles) -> Result<Self, DatasetIoError> {
        Ok(Self { files })
    }

    pub fn parses(&self, fileids: Option<&[String]>) -> Result<ParseForest, DatasetIoError> {
        let view = self.sexpr_view(fileids)?;
        let mut iter = view.into_iter();
        let mut out = Vec::new();
        while let Some(expr) = iter.next()? {
            if !expr.trim_start().starts_with('(') {
                continue;
            }
            if let Ok(tree) = parse_bracketed(&expr) {
                out.push(Parse::new(tree, ParseKind::Constituency));
            }
        }
        Ok(out)
    }

    pub fn trees(&self, fileids: Option<&[String]>) -> Result<Vec<TreeValue>, DatasetIoError> {
        let parses = self.parses(fileids)?;
        Ok(parses.into_iter().map(|p| p.tree().clone()).collect())
    }

    pub fn sexpr_view(
        &self,
        fileids: Option<&[String]>,
    ) -> Result<ConcatenatedCorpusView<String>, DatasetIoError> {
        Ok(corpus_concat(build_views(
            &self.files,
            fileids,
            read_sexpr_block_default,
        )?))
    }
}

impl CorpusReader for BracketedCorpusReader {
    fn files(&self) -> &CorpusFiles {
        &self.files
    }
}

pub struct XmlCorpusReader {
    files: CorpusFiles,
    word_tokenizer: Box<dyn Tokenizer>,
}

impl XmlCorpusReader {
    pub fn new(files: CorpusFiles) -> Self {
        Self {
            files,
            word_tokenizer: Box::new(WordPunctTokenizer::new()),
        }
    }

    pub fn with_word_tokenizer(mut self, tokenizer: impl Tokenizer + 'static) -> Self {
        self.word_tokenizer = Box::new(tokenizer);
        self
    }

    pub fn text(&self, fileid: &str) -> Result<String, DatasetIoError> {
        let view = self.text_view(Some(&[fileid.to_string()]))?;
        let mut iter = view.into_iter();
        let mut out = Vec::new();
        while let Some(text) = iter.next()? {
            out.push(text);
        }
        Ok(out.join(" "))
    }

    pub fn words(&self, fileid: &str) -> Result<Vec<String>, DatasetIoError> {
        let view = self.text_view(Some(&[fileid.to_string()]))?;
        let mut iter = view.into_iter();
        let mut out = Vec::new();
        while let Some(text) = iter.next()? {
            out.extend(tokens_to_texts(self.word_tokenizer.tokenize(&text)));
        }
        Ok(out)
    }

    pub fn words_all(&self, fileids: Option<&[String]>) -> Result<Vec<String>, DatasetIoError> {
        let view = self.text_view(fileids)?;
        let mut iter = view.into_iter();
        let mut out = Vec::new();
        while let Some(text) = iter.next()? {
            out.extend(tokens_to_texts(self.word_tokenizer.tokenize(&text)));
        }
        Ok(out)
    }

    pub fn text_view(
        &self,
        fileids: Option<&[String]>,
    ) -> Result<ConcatenatedCorpusView<String>, DatasetIoError> {
        Ok(corpus_concat(build_views(
            &self.files,
            fileids,
            read_markup_text_block,
        )?))
    }
}

impl CorpusReader for XmlCorpusReader {
    fn files(&self) -> &CorpusFiles {
        &self.files
    }
}

fn tokens_to_texts(tokens: Vec<Token>) -> Vec<String> {
    tokens.into_iter().map(|t| t.text().to_string()).collect()
}

fn build_views<T: Clone>(
    files: &CorpusFiles,
    fileids: Option<&[String]>,
    reader: BlockReader<T>,
) -> Result<Vec<StreamBackedCorpusView<T>>, DatasetIoError> {
    let fileids = fileids.unwrap_or_else(|| files.fileids());
    let mut out = Vec::with_capacity(fileids.len());
    for fileid in fileids {
        let path = files.abspath(fileid)?;
        out.push(StreamBackedCorpusView::new(path, reader)?);
    }
    Ok(out)
}

fn discover_fileids(root: &Path, pattern: Option<&Regex>) -> Result<Vec<String>, DatasetIoError> {
    let mut out = Vec::new();
    walk_dir(root, root, pattern, &mut out)?;
    out.sort();
    Ok(out)
}

fn walk_dir(
    root: &Path,
    dir: &Path,
    pattern: Option<&Regex>,
    out: &mut Vec<String>,
) -> Result<(), DatasetIoError> {
    for entry in fs::read_dir(dir).map_err(|e| DatasetIoError::Io(e.to_string()))? {
        let entry = entry.map_err(|e| DatasetIoError::Io(e.to_string()))?;
        let path = entry.path();
        if path.is_dir() {
            walk_dir(root, &path, pattern, out)?;
            continue;
        }
        if !path.is_file() {
            continue;
        }
        let rel = path
            .strip_prefix(root)
            .map_err(|e| DatasetIoError::Io(e.to_string()))?;
        let rel = rel.to_string_lossy().replace('\\', "/");
        if let Some(re) = pattern {
            if !re.is_match(&rel) {
                continue;
            }
        }
        out.push(rel);
    }
    Ok(())
}

fn ensure_safe_fileid(fileid: &str) -> Result<(), DatasetIoError> {
    if Path::new(fileid).is_absolute() {
        return Err(DatasetIoError::Unsupported(
            "absolute fileids are not allowed".to_string(),
        ));
    }
    if fileid.split('/').any(|part| part == "..") {
        return Err(DatasetIoError::Unsupported(
            "path traversal is not allowed".to_string(),
        ));
    }
    Ok(())
}
