//! Statistical modeling expressions.
//!
//! These types describe pipelines, steps, and evaluation intent without
//! committing to concrete data containers.

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum StatExpr {
    Pipeline(StatPipeline),
    Step(StatStep),
    Split(StatSplit),
    Metric(StatMetric),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatPipeline {
    steps: Vec<StatStep>,
}

impl StatPipeline {
    pub fn new(steps: Vec<StatStep>) -> Self {
        Self { steps }
    }

    pub fn steps(&self) -> &[StatStep] {
        &self.steps
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatStep {
    Preprocess(StatStepSpec),
    Feature(StatStepSpec),
    Model(StatStepSpec),
}

impl StatStep {
    pub fn preprocess(name: impl Into<String>, params: BTreeMap<String, String>) -> Self {
        StatStep::Preprocess(StatStepSpec::new(name, params))
    }

    pub fn feature(name: impl Into<String>, params: BTreeMap<String, String>) -> Self {
        StatStep::Feature(StatStepSpec::new(name, params))
    }

    pub fn model(name: impl Into<String>, params: BTreeMap<String, String>) -> Self {
        StatStep::Model(StatStepSpec::new(name, params))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatStepSpec {
    name: String,
    params: BTreeMap<String, String>,
}

impl StatStepSpec {
    pub fn new(name: impl Into<String>, params: BTreeMap<String, String>) -> Self {
        Self {
            name: name.into(),
            params,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn params(&self) -> &BTreeMap<String, String> {
        &self.params
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatSplit {
    train: f64,
    valid: f64,
    test: f64,
    shuffle: bool,
    seed: Option<u64>,
}

impl StatSplit {
    pub fn new(train: f64, valid: f64, test: f64) -> Self {
        Self {
            train,
            valid,
            test,
            shuffle: true,
            seed: None,
        }
    }

    pub fn with_shuffle(mut self, shuffle: bool) -> Self {
        self.shuffle = shuffle;
        self
    }

    pub fn with_seed(mut self, seed: Option<u64>) -> Self {
        self.seed = seed;
        self
    }

    pub fn train(&self) -> f64 {
        self.train
    }

    pub fn valid(&self) -> f64 {
        self.valid
    }

    pub fn test(&self) -> f64 {
        self.test
    }

    pub fn shuffle(&self) -> bool {
        self.shuffle
    }

    pub fn seed(&self) -> Option<u64> {
        self.seed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatMetric {
    name: String,
    params: BTreeMap<String, String>,
}

impl StatMetric {
    pub fn new(name: impl Into<String>, params: BTreeMap<String, String>) -> Self {
        Self {
            name: name.into(),
            params,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn params(&self) -> &BTreeMap<String, String> {
        &self.params
    }
}
