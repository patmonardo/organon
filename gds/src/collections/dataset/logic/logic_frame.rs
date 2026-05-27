//! LogicFrame end-view: Corpus, LanguageModel, and LogicForm.

use std::fmt;

use crate::collections::dataframe::GDSFrameError;
use crate::collections::dataset::corpus::Corpus;
use crate::collections::dataset::functions::model::preprocessing::padded_everygram_pipeline;
use crate::collections::dataset::language::tokenizer::Tokenizer;
use crate::collections::dataset::language::{LanguageModel, LmError};
use crate::collections::dataset::logic::LogicForm;
use crate::form::program::ProgramFeature;
use crate::ml::nlp::sem::logic::{LogicParseError, LogicParser};

#[derive(Debug)]
pub enum LogicError {
    Frame(GDSFrameError),
    Lm(LmError),
    Logic(LogicParseError),
}

impl From<GDSFrameError> for LogicError {
    fn from(e: GDSFrameError) -> Self {
        Self::Frame(e)
    }
}

impl From<LmError> for LogicError {
    fn from(e: LmError) -> Self {
        Self::Lm(e)
    }
}

impl From<LogicParseError> for LogicError {
    fn from(e: LogicParseError) -> Self {
        Self::Logic(e)
    }
}

impl fmt::Display for LogicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Frame(e) => write!(f, "frame error: {e}"),
            Self::Lm(e) => write!(f, "lm error: {e}"),
            Self::Logic(e) => write!(f, "logic error: {e}"),
        }
    }
}

impl std::error::Error for LogicError {}

#[derive(Debug, Clone)]
pub struct LogicFrame<L> {
    corpus: Corpus,
    lm: L,
    forms: Vec<LogicForm>,
}

impl<L> LogicFrame<L> {
    pub fn new(corpus: Corpus, lm: L) -> Self {
        Self::with_forms(corpus, lm, Vec::new())
    }

    pub fn with_forms(corpus: Corpus, lm: L, forms: Vec<LogicForm>) -> Self {
        Self { corpus, lm, forms }
    }

    pub fn corpus(&self) -> &Corpus {
        &self.corpus
    }

    pub fn corpus_mut(&mut self) -> &mut Corpus {
        &mut self.corpus
    }

    pub fn lm(&self) -> &L {
        &self.lm
    }

    pub fn lm_mut(&mut self) -> &mut L {
        &mut self.lm
    }

    pub fn forms(&self) -> &[LogicForm] {
        &self.forms
    }

    pub fn forms_mut(&mut self) -> &mut Vec<LogicForm> {
        &mut self.forms
    }

    pub fn ingest_forms<I>(&mut self, forms: I)
    where
        I: IntoIterator<Item = ProgramFeature>,
    {
        self.forms.extend(forms.into_iter().map(LogicForm::from));
    }

    pub fn parse_forms(&mut self) -> usize {
        let parser = LogicParser::new();
        let mut parsed = 0usize;

        for form in &mut self.forms {
            match parser.parse(&form.text) {
                Ok(expr) => {
                    form.expr = Some(expr);
                    form.error = None;
                    parsed += 1;
                }
                Err(e) => {
                    form.expr = None;
                    form.error = Some(e.to_string());
                }
            }
        }

        parsed
    }

    pub fn into_parts(self) -> (Corpus, L) {
        (self.corpus, self.lm)
    }
}

impl<L: LanguageModel> LogicFrame<L> {
    pub fn fit<T: Tokenizer>(corpus: Corpus, mut lm: L, tokenizer: &T) -> Result<Self, LogicError> {
        fit_lm(&corpus, &mut lm, tokenizer)?;
        Ok(Self::new(corpus, lm))
    }

    pub fn refit<T: Tokenizer>(&mut self, tokenizer: &T) -> Result<(), LogicError> {
        fit_lm(&self.corpus, &mut self.lm, tokenizer)
    }
}

fn fit_lm<L: LanguageModel, T: Tokenizer>(
    corpus: &Corpus,
    lm: &mut L,
    tokenizer: &T,
) -> Result<(), LogicError> {
    let tokenized = corpus.tokenize(tokenizer)?;
    let text = tokenized
        .into_iter()
        .map(|doc| {
            doc.into_iter()
                .map(|token| token.text().to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
    let (train, vocab) = padded_everygram_pipeline(lm.order(), &text);
    lm.fit(train, Some(vocab))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::language::tokenizer::WhitespaceTokenizer;
    use crate::collections::dataset::language::MLE;
    use crate::form::program::ProgramFeatureKind;

    #[test]
    fn logic_frame_pairs_corpus_with_fitted_lm() {
        let corpus = Corpus::from_texts(&["alpha beta alpha"]).expect("corpus");
        let semantic = LogicFrame::fit(corpus, MLE::new(2), &WhitespaceTokenizer).expect("sem");

        assert_eq!(semantic.corpus().document_count(), 1);
        assert_eq!(semantic.lm().order(), 2);
        assert!(semantic.lm().vocab().len() > 0);
    }

    #[test]
    fn logic_forms_parse_logic_expressions() {
        let corpus = Corpus::from_texts(&["alpha beta"]).expect("corpus");
        let mut semantic = LogicFrame::new(corpus, MLE::new(1));

        semantic.ingest_forms(vec![
            ProgramFeature::new(
                ProgramFeatureKind::Judgment,
                "all x. (human(x) -> mortal(x))".to_string(),
                "test".to_string(),
            ),
            ProgramFeature::new(
                ProgramFeatureKind::Inference,
                "all x.".to_string(),
                "test".to_string(),
            ),
        ]);

        assert_eq!(semantic.parse_forms(), 1);
        assert!(semantic.forms()[0].parsed());
        assert!(semantic.forms()[1].error.is_some());
    }
}
