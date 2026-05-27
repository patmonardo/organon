use crate::ml::models::TrainerConfig;
use crate::ml::models::TrainingMethod;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SVMKernelType {
    Linear,
    Rbf { gamma: f64 },
}

impl Default for SVMKernelType {
    fn default() -> Self {
        Self::Linear
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SVMClassifierTrainConfig {
    #[serde(default)]
    pub kernel: SVMKernelType,

    #[serde(default = "default_c")]
    pub c: f64,

    #[serde(default = "default_tolerance")]
    pub tolerance: f64,

    #[serde(default = "default_max_passes")]
    pub max_passes: usize,

    #[serde(default = "default_max_iterations")]
    pub max_iterations: usize,
}

fn default_c() -> f64 {
    1.0
}

fn default_tolerance() -> f64 {
    1e-3
}

fn default_max_passes() -> usize {
    5
}

fn default_max_iterations() -> usize {
    1000
}

impl Default for SVMClassifierTrainConfig {
    fn default() -> Self {
        Self {
            kernel: SVMKernelType::default(),
            c: default_c(),
            tolerance: default_tolerance(),
            max_passes: default_max_passes(),
            max_iterations: default_max_iterations(),
        }
    }
}

impl TrainerConfig for SVMClassifierTrainConfig {
    fn method(&self) -> TrainingMethod {
        TrainingMethod::SVMClassification
    }

    fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert(
            "method".to_string(),
            serde_json::Value::String("SVMClassification".to_string()),
        );

        let kernel_name = match self.kernel {
            SVMKernelType::Linear => "linear".to_string(),
            SVMKernelType::Rbf { .. } => "rbf".to_string(),
        };
        map.insert(
            "kernel".to_string(),
            serde_json::Value::String(kernel_name),
        );

        if let SVMKernelType::Rbf { gamma } = self.kernel {
            map.insert(
                "gamma".to_string(),
                serde_json::Value::Number(
                    serde_json::Number::from_f64(gamma)
                        .expect("gamma should serialize as finite f64"),
                ),
            );
        }

        map.insert(
            "c".to_string(),
            serde_json::Value::Number(
                serde_json::Number::from_f64(self.c).expect("c should serialize as finite f64"),
            ),
        );
        map.insert(
            "tolerance".to_string(),
            serde_json::Value::Number(
                serde_json::Number::from_f64(self.tolerance)
                    .expect("tolerance should serialize as finite f64"),
            ),
        );
        map.insert(
            "max_passes".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.max_passes)),
        );
        map.insert(
            "max_iterations".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.max_iterations)),
        );

        map
    }
}
