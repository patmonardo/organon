//! SemDataset end-view: Corpus, LanguageModel, and SemForm.

use std::fmt;

use crate::collections::dataframe::GDSFrameError;
use crate::collections::dataset::corpus::Corpus;
use crate::collections::dataset::functions::model::preprocessing::padded_everygram_pipeline;
use crate::collections::dataset::lm::{LanguageModel, LmError};
use crate::collections::dataset::sem::SemForm;
use crate::collections::dataset::tokenizer::Tokenizer;
use crate::form::program::ProgramFeature;
use crate::ml::nlp::sem::logic::{LogicParseError, LogicParser};

#[derive(Debug)]
pub enum SemError {
    Frame(GDSFrameError),
    Lm(LmError),
    Logic(LogicParseError),
}

impl From<GDSFrameError> for SemError {
    fn from(e: GDSFrameError) -> Self {
        Self::Frame(e)
    }
}

impl From<LmError> for SemError {
    fn from(e: LmError) -> Self {
        Self::Lm(e)
    }
}

impl From<LogicParseError> for SemError {
    fn from(e: LogicParseError) -> Self {
        Self::Logic(e)
    }
}

impl fmt::Display for SemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Frame(e) => write!(f, "frame error: {e}"),
            Self::Lm(e) => write!(f, "lm error: {e}"),
            Self::Logic(e) => write!(f, "logic error: {e}"),
        }
    }
}

impl std::error::Error for SemError {}

#[derive(Debug, Clone)]
pub struct SemDataset<L> {
    corpus: Corpus,
    lm: L,
    forms: Vec<SemForm>,
}

impl<L> SemDataset<L> {
    pub fn new(corpus: Corpus, lm: L) -> Self {
        Self::with_forms(corpus, lm, Vec::new())
    }

    pub fn with_forms(corpus: Corpus, lm: L, forms: Vec<SemForm>) -> Self {
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

    pub fn forms(&self) -> &[SemForm] {
        &self.forms
    }

    pub fn forms_mut(&mut self) -> &mut Vec<SemForm> {
        &mut self.forms
    }

    pub fn ingest_forms<I>(&mut self, forms: I)
    where
        I: IntoIterator<Item = ProgramFeature>,
    {
        self.forms.extend(forms.into_iter().map(SemForm::from));
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

impl<L: LanguageModel> SemDataset<L> {
    pub fn fit<T: Tokenizer>(corpus: Corpus, mut lm: L, tokenizer: &T) -> Result<Self, SemError> {
        fit_lm(&corpus, &mut lm, tokenizer)?;
        Ok(Self::new(corpus, lm))
    }

    pub fn refit<T: Tokenizer>(&mut self, tokenizer: &T) -> Result<(), SemError> {
        fit_lm(&self.corpus, &mut self.lm, tokenizer)
    }
}

fn fit_lm<L: LanguageModel, T: Tokenizer>(
    corpus: &Corpus,
    lm: &mut L,
    tokenizer: &T,
) -> Result<(), SemError> {
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
    use crate::collections::dataset::lm::MLE;
    use crate::collections::dataset::tokenizer::WhitespaceTokenizer;
    use crate::form::program::ProgramFeatureKind;

    #[test]
    fn sem_dataset_pairs_corpus_with_fitted_lm() {
        let corpus = Corpus::from_texts(&["alpha beta alpha"]).expect("corpus");
        let semantic = SemDataset::fit(corpus, MLE::new(2), &WhitespaceTokenizer).expect("sem");

        assert_eq!(semantic.corpus().document_count(), 1);
        assert_eq!(semantic.lm().order(), 2);
        assert!(semantic.lm().vocab().len() > 0);
    }

    #[test]
    fn sem_forms_parse_logic_expressions() {
        let corpus = Corpus::from_texts(&["alpha beta"]).expect("corpus");
        let mut semantic = SemDataset::new(corpus, MLE::new(1));

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
