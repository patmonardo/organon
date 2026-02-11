//! Minimal corpus utilities inspired by NLTK's `corpus.reader.util`.

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use regex::Regex;

use crate::collections::dataset::error::DatasetIoError;
use crate::collections::dataset::tokenizer::{MarkupTokenizer, Tokenizer, WordPunctTokenizer};

pub type BlockReader<T> = fn(&mut BufReader<File>) -> Result<Vec<T>, DatasetIoError>;

#[derive(Debug)]
pub struct StreamBackedCorpusView<T> {
    fileid: PathBuf,
    reader: BlockReader<T>,
}

impl<T: Clone> StreamBackedCorpusView<T> {
    pub fn new(fileid: impl Into<PathBuf>, reader: BlockReader<T>) -> Result<Self, DatasetIoError> {
        let fileid = fileid.into();
        Ok(Self { fileid, reader })
    }

    pub fn fileid(&self) -> &Path {
        &self.fileid
    }

    pub fn len(&self) -> Result<usize, DatasetIoError> {
        let mut iter = self.iter_from(0)?;
        while let Some(_) = iter.next()? {}
        Ok(iter.len)
    }

    pub fn get(&self, index: usize) -> Result<Option<T>, DatasetIoError> {
        let mut iter = self.iter_from(index)?;
        Ok(iter.next()?)
    }

    pub fn iter(&self) -> Result<StreamBackedCorpusIter<T>, DatasetIoError> {
        self.iter_from(0)
    }

    pub fn iter_from(&self, start_tok: usize) -> Result<StreamBackedCorpusIter<T>, DatasetIoError> {
        StreamBackedCorpusIter::new(&self.fileid, self.reader, start_tok)
    }
}

pub struct StreamBackedCorpusIter<T> {
    reader: BlockReader<T>,
    eofpos: u64,
    stream: BufReader<File>,
    start_tok: usize,
    toknum: usize,
    filepos: u64,
    len: usize,
    cache: (usize, usize, Vec<T>),
}

impl<T: Clone> StreamBackedCorpusIter<T> {
    fn new(
        fileid: &Path,
        reader: BlockReader<T>,
        start_tok: usize,
    ) -> Result<Self, DatasetIoError> {
        let meta = std::fs::metadata(fileid).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        let eofpos = meta.len();
        let stream =
            BufReader::new(File::open(fileid).map_err(|e| DatasetIoError::Io(e.to_string()))?);
        Ok(Self {
            reader,
            eofpos,
            stream,
            start_tok,
            toknum: 0,
            filepos: 0,
            len: 0,
            cache: (0, 0, Vec::new()),
        })
    }

    pub fn next(&mut self) -> Result<Option<T>, DatasetIoError> {
        loop {
            if self.start_tok >= self.cache.0 && self.start_tok < self.cache.1 {
                let idx = self.start_tok - self.cache.0;
                if let Some(value) = self.cache.2.get(idx).cloned() {
                    self.start_tok += 1;
                    return Ok(Some(value));
                }
            }

            if self.filepos >= self.eofpos {
                self.len = self.toknum;
                return Ok(None);
            }

            let tokens = (self.reader)(&mut self.stream)?;
            let num_toks = tokens.len();
            let new_filepos = self
                .stream
                .stream_position()
                .map_err(|e| DatasetIoError::Io(e.to_string()))?;

            if num_toks == 0 {
                self.filepos = new_filepos;
                if self.filepos >= self.eofpos {
                    self.len = self.toknum;
                    return Ok(None);
                }
                continue;
            }

            self.cache = (self.toknum, self.toknum + num_toks, tokens);
            self.filepos = new_filepos;
            self.toknum += num_toks;

            if self.filepos >= self.eofpos {
                self.len = self.toknum;
            }
        }
    }
}

#[derive(Debug)]
pub struct ConcatenatedCorpusView<T> {
    pieces: Vec<StreamBackedCorpusView<T>>,
}

impl<T: Clone> ConcatenatedCorpusView<T> {
    pub fn new(pieces: Vec<StreamBackedCorpusView<T>>) -> Self {
        Self { pieces }
    }

    pub fn into_iter(self) -> ConcatenatedCorpusIter<T> {
        ConcatenatedCorpusIter {
            pieces: self.pieces,
            piece_index: 0,
            piece_iter: None,
        }
    }
}

pub struct ConcatenatedCorpusIter<T> {
    pieces: Vec<StreamBackedCorpusView<T>>,
    piece_index: usize,
    piece_iter: Option<StreamBackedCorpusIter<T>>,
}

impl<T: Clone> ConcatenatedCorpusIter<T> {
    pub fn next(&mut self) -> Result<Option<T>, DatasetIoError> {
        loop {
            if self.piece_index >= self.pieces.len() {
                return Ok(None);
            }
            if self.piece_iter.is_none() {
                let iter = self.pieces[self.piece_index].iter_from(0)?;
                self.piece_iter = Some(iter);
            }
            if let Some(iter) = self.piece_iter.as_mut() {
                if let Some(value) = iter.next()? {
                    return Ok(Some(value));
                }
            }
            self.piece_iter = None;
            self.piece_index += 1;
        }
    }
}

pub fn concat<T: Clone>(mut views: Vec<StreamBackedCorpusView<T>>) -> ConcatenatedCorpusView<T> {
    ConcatenatedCorpusView::new(views.drain(..).collect())
}

pub fn read_whitespace_block(stream: &mut BufReader<File>) -> Result<Vec<String>, DatasetIoError> {
    let mut out = Vec::new();
    for _ in 0..20 {
        let mut line = String::new();
        if stream
            .read_line(&mut line)
            .map_err(|e| DatasetIoError::Io(e.to_string()))?
            == 0
        {
            break;
        }
        out.extend(line.split_whitespace().map(|s| s.to_string()));
    }
    Ok(out)
}

pub fn read_wordpunct_block(stream: &mut BufReader<File>) -> Result<Vec<String>, DatasetIoError> {
    let tokenizer = WordPunctTokenizer::new();
    let mut out = Vec::new();
    for _ in 0..20 {
        let mut line = String::new();
        if stream
            .read_line(&mut line)
            .map_err(|e| DatasetIoError::Io(e.to_string()))?
            == 0
        {
            break;
        }
        out.extend(
            tokenizer
                .tokenize(&line)
                .into_iter()
                .map(|t| t.text().to_string()),
        );
    }
    Ok(out)
}

pub fn read_markup_text_block(stream: &mut BufReader<File>) -> Result<Vec<String>, DatasetIoError> {
    let tokenizer = MarkupTokenizer::new();
    let mut out = Vec::new();
    for _ in 0..20 {
        let mut line = String::new();
        if stream
            .read_line(&mut line)
            .map_err(|e| DatasetIoError::Io(e.to_string()))?
            == 0
        {
            break;
        }
        for token in tokenizer.tokenize(&line) {
            let text = token.text();
            if text.starts_with('<') {
                continue;
            }
            out.push(text.to_string());
        }
    }
    Ok(out)
}

pub fn read_line_block(stream: &mut BufReader<File>) -> Result<Vec<String>, DatasetIoError> {
    let mut out = Vec::new();
    for _ in 0..20 {
        let mut line = String::new();
        if stream
            .read_line(&mut line)
            .map_err(|e| DatasetIoError::Io(e.to_string()))?
            == 0
        {
            break;
        }
        out.push(line.trim_end_matches(['\n', '\r']).to_string());
    }
    Ok(out)
}

pub fn read_blankline_block(stream: &mut BufReader<File>) -> Result<Vec<String>, DatasetIoError> {
    let mut buffer = String::new();
    loop {
        let mut line = String::new();
        let bytes = stream
            .read_line(&mut line)
            .map_err(|e| DatasetIoError::Io(e.to_string()))?;
        if bytes == 0 {
            if buffer.is_empty() {
                return Ok(Vec::new());
            }
            return Ok(vec![buffer.clone()]);
        }
        if line.trim().is_empty() {
            if !buffer.is_empty() {
                return Ok(vec![buffer.clone()]);
            }
            continue;
        }
        buffer.push_str(&line);
    }
}

pub fn read_regexp_block(
    stream: &mut BufReader<File>,
    start_re: &Regex,
    end_re: Option<&Regex>,
) -> Result<Vec<String>, DatasetIoError> {
    let mut line = String::new();
    loop {
        line.clear();
        if stream
            .read_line(&mut line)
            .map_err(|e| DatasetIoError::Io(e.to_string()))?
            == 0
        {
            return Ok(Vec::new());
        }
        if start_re.is_match(&line) {
            break;
        }
    }

    let mut lines = vec![line.clone()];
    loop {
        let oldpos = stream
            .stream_position()
            .map_err(|e| DatasetIoError::Io(e.to_string()))?;
        line.clear();
        if stream
            .read_line(&mut line)
            .map_err(|e| DatasetIoError::Io(e.to_string()))?
            == 0
        {
            return Ok(vec![lines.concat()]);
        }
        if end_re.map_or(false, |re| re.is_match(&line)) {
            return Ok(vec![lines.concat()]);
        }
        if end_re.is_none() && start_re.is_match(&line) {
            stream
                .seek(SeekFrom::Start(oldpos))
                .map_err(|e| DatasetIoError::Io(e.to_string()))?;
            return Ok(vec![lines.concat()]);
        }
        lines.push(line.clone());
    }
}

pub fn read_sexpr_block(
    stream: &mut BufReader<File>,
    block_size: usize,
) -> Result<Vec<String>, DatasetIoError> {
    let start = stream
        .stream_position()
        .map_err(|e| DatasetIoError::Io(e.to_string()))?;
    let mut block = vec![0u8; block_size];
    let read = stream
        .read(&mut block)
        .map_err(|e| DatasetIoError::Io(e.to_string()))?;
    block.truncate(read);
    let mut buffer = String::from_utf8_lossy(&block).to_string();

    loop {
        match parse_sexpr_block(&buffer) {
            Ok((tokens, offset)) => {
                stream
                    .seek(SeekFrom::Start(start + offset as u64))
                    .map_err(|e| DatasetIoError::Io(e.to_string()))?;
                return Ok(tokens);
            }
            Err(SexprParseError::BlockTooSmall) => {
                let mut next_block = vec![0u8; block_size];
                let read = stream
                    .read(&mut next_block)
                    .map_err(|e| DatasetIoError::Io(e.to_string()))?;
                if read == 0 {
                    let trimmed = buffer.trim();
                    return Ok(if trimmed.is_empty() {
                        Vec::new()
                    } else {
                        vec![trimmed.to_string()]
                    });
                }
                next_block.truncate(read);
                buffer.push_str(&String::from_utf8_lossy(&next_block));
            }
            Err(SexprParseError::Invalid(msg)) => {
                return Err(DatasetIoError::Unsupported(msg.to_string()));
            }
        }
    }
}

pub fn read_sexpr_block_default(
    stream: &mut BufReader<File>,
) -> Result<Vec<String>, DatasetIoError> {
    read_sexpr_block(stream, 16_384)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SexprParseError {
    BlockTooSmall,
    Invalid(&'static str),
}

fn parse_sexpr_block(block: &str) -> Result<(Vec<String>, usize), SexprParseError> {
    let mut tokens = Vec::new();
    let mut end = 0usize;

    while end < block.len() {
        let Some(next) = block[end..].find(|c: char| !c.is_whitespace()) else {
            return Ok((tokens, end));
        };
        let start = end + next;

        let ch = block[start..]
            .chars()
            .next()
            .ok_or(SexprParseError::Invalid("empty"))?;
        if ch != '(' {
            let mut local_end = start;
            for (idx, c) in block[start..].char_indices() {
                if c.is_whitespace() || c == '(' {
                    local_end = start + idx;
                    break;
                }
                local_end = start + idx + c.len_utf8();
            }
            if local_end == start {
                return Err(SexprParseError::BlockTooSmall);
            }
            tokens.push(block[start..local_end].to_string());
            end = local_end;
            continue;
        }

        let mut nesting = 0i64;
        let mut found = None;
        for (idx, c) in block[start..].char_indices() {
            if c == '(' {
                nesting += 1;
            } else if c == ')' {
                nesting -= 1;
                if nesting == 0 {
                    found = Some(start + idx + c.len_utf8());
                    break;
                }
            }
        }
        let Some(local_end) = found else {
            return if tokens.is_empty() {
                Err(SexprParseError::BlockTooSmall)
            } else {
                Ok((tokens, end))
            };
        };
        tokens.push(block[start..local_end].to_string());
        end = local_end;
    }

    Ok((tokens, end))
}
