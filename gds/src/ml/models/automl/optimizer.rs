use super::config::TunableTrainerConfig;
use super::hyperparameter::DoubleRangeParameter;
use super::hyperparameter::IntegerRangeParameter;
use super::hyperparameter::NumericalRangeParameter;
use crate::ml::models::TrainerConfig;
use crate::ml::models::TrainingMethod;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::collections::HashMap;

/// HyperParameterOptimizer defines a strategy for searching through a hyperparameter space
///
pub trait HyperParameterOptimizer: Iterator<Item = Box<dyn TrainerConfig>> {}

/// A basic implementation of random search through a hyperparameter space
///
pub struct RandomSearch {
    concrete_configs: Vec<TunableTrainerConfig>,
    tunable_configs: Vec<TunableTrainerConfig>,
    total_number_of_trials: usize,
    number_of_concrete_trials: usize,
    random: StdRng,
    number_of_finished_trials: usize,
}

impl RandomSearch {
    pub fn new(
        parameter_space: HashMap<TrainingMethod, Vec<TunableTrainerConfig>>,
        max_trials: usize,
        random_seed: u64,
    ) -> Self {
        Self::new_with_seed(parameter_space, max_trials, Some(random_seed))
    }

    pub fn new_with_seed(
        parameter_space: HashMap<TrainingMethod, Vec<TunableTrainerConfig>>,
        max_trials: usize,
        random_seed: Option<u64>,
    ) -> Self {
        let mut ordered_parameter_space: Vec<_> = parameter_space.into_iter().collect();
        ordered_parameter_space.sort_by_key(|(method, _)| training_method_rank(*method));
        let ordered_configs: Vec<TunableTrainerConfig> = ordered_parameter_space
            .into_iter()
            .flat_map(|(_, configs)| configs)
            .collect();
        let concrete_configs: Vec<TunableTrainerConfig> = ordered_configs
            .iter()
            .filter(|config| config.is_concrete())
            .cloned()
            .collect();

        let tunable_configs: Vec<TunableTrainerConfig> = ordered_configs
            .iter()
            .filter(|config| !config.is_concrete())
            .cloned()
            .collect();

        let number_of_concrete_trials = concrete_configs.len();
        let total_number_of_trials = max_trials + number_of_concrete_trials;
        let random = random_seed
            .map(StdRng::seed_from_u64)
            .unwrap_or_else(StdRng::from_entropy);

        Self {
            concrete_configs,
            tunable_configs,
            total_number_of_trials,
            number_of_concrete_trials,
            random,
            number_of_finished_trials: 0,
        }
    }

    pub fn has_next(&self) -> bool {
        (self.number_of_finished_trials < self.number_of_concrete_trials)
            || (self.number_of_finished_trials < self.total_number_of_trials
                && !self.tunable_configs.is_empty())
    }

    fn sample_integer(&mut self, range: &IntegerRangeParameter) -> i32 {
        self.random.gen_range(range.min()..range.max())
    }

    fn sample_double(&mut self, range: &DoubleRangeParameter) -> f64 {
        if range.log_scale() {
            let min = if range.min() < 1e-20 {
                (1e-20_f64).ln()
            } else {
                range.min().ln()
            };
            let max = range.max().ln();
            self.random.gen_range(min..max).exp()
        } else {
            self.random.gen_range(range.min()..range.max())
        }
    }

    fn sample(&mut self, tunable_config: &TunableTrainerConfig) -> Box<dyn TrainerConfig> {
        let mut hyper_parameter_values = HashMap::new();
        let mut double_ranges: Vec<_> = tunable_config.double_ranges().iter().collect();
        double_ranges.sort_by_key(|(name, _)| *name);
        for (name, range) in double_ranges {
            hyper_parameter_values.insert(
                name.clone(),
                serde_json::Value::Number(
                    serde_json::Number::from_f64(self.sample_double(range)).unwrap(),
                ),
            );
        }

        let mut integer_ranges: Vec<_> = tunable_config.integer_ranges().iter().collect();
        integer_ranges.sort_by_key(|(name, _)| *name);
        for (name, range) in integer_ranges {
            hyper_parameter_values.insert(
                name.clone(),
                serde_json::Value::Number(self.sample_integer(range).into()),
            );
        }
        tunable_config.materialize(hyper_parameter_values)
    }
}

impl Iterator for RandomSearch {
    type Item = Box<dyn TrainerConfig>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next() {
            return None;
        }

        let config = if self.number_of_finished_trials < self.concrete_configs.len() {
            let config =
                self.concrete_configs[self.number_of_finished_trials].materialize(HashMap::new());
            self.number_of_finished_trials += 1;
            config
        } else {
            self.number_of_finished_trials += 1;
            let idx = self.random.gen_range(0..self.tunable_configs.len());
            let tunable = self.tunable_configs[idx].clone();
            self.sample(&tunable)
        };

        Some(config)
    }
}

impl HyperParameterOptimizer for RandomSearch {}

fn training_method_rank(method: TrainingMethod) -> usize {
    match method {
        TrainingMethod::LogisticRegression => 0,
        TrainingMethod::LinearRegression => 1,
        TrainingMethod::RandomForestClassification => 2,
        TrainingMethod::RandomForestRegression => 3,
        TrainingMethod::SVMClassification => 4,
        TrainingMethod::MLPClassification => 5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn tunable_linear_config() -> TunableTrainerConfig {
        TunableTrainerConfig::of(
            &HashMap::from([
                ("penalty".to_string(), json!({"range": [0.01, 1.0]})),
                ("learningRate".to_string(), json!({"range": [0.001, 0.1]})),
                ("maxEpochs".to_string(), json!({"range": [10, 50]})),
            ]),
            TrainingMethod::LinearRegression,
        )
        .expect("valid linear parameter ranges")
    }

    fn tunable_random_forest_config() -> TunableTrainerConfig {
        TunableTrainerConfig::of(
            &HashMap::from([
                (
                    "numberOfDecisionTrees".to_string(),
                    json!({"range": [2, 8]}),
                ),
                ("maxDepth".to_string(), json!({"range": [2, 6]})),
            ]),
            TrainingMethod::RandomForestRegression,
        )
        .expect("valid random forest parameter ranges")
    }

    #[test]
    fn seeded_search_is_independent_of_parameter_space_insertion_order() {
        let forward = HashMap::from([
            (
                TrainingMethod::LinearRegression,
                vec![tunable_linear_config()],
            ),
            (
                TrainingMethod::RandomForestRegression,
                vec![tunable_random_forest_config()],
            ),
        ]);
        let reverse = HashMap::from([
            (
                TrainingMethod::RandomForestRegression,
                vec![tunable_random_forest_config()],
            ),
            (
                TrainingMethod::LinearRegression,
                vec![tunable_linear_config()],
            ),
        ]);

        let first: Vec<_> = RandomSearch::new(forward, 6, 42)
            .map(|config| config.to_map())
            .collect();
        let second: Vec<_> = RandomSearch::new(reverse, 6, 42)
            .map(|config| config.to_map())
            .collect();

        assert_eq!(first, second);
        assert_eq!(first.len(), 6);
    }

    #[test]
    fn exhausted_search_returns_none() {
        let config = TunableTrainerConfig::of(
            &HashMap::from([("penalty".to_string(), json!(0.0))]),
            TrainingMethod::LinearRegression,
        )
        .expect("valid concrete config");
        let mut search = RandomSearch::new(
            HashMap::from([(TrainingMethod::LinearRegression, vec![config])]),
            10,
            42,
        );

        assert!(search.next().is_some());
        assert!(search.next().is_none());
        assert!(search.next().is_none());
    }
}
