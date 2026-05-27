#[derive(Clone, Debug)]
pub struct PlattScaler {
    a: f64,
    b: f64,
}

impl Default for PlattScaler {
    fn default() -> Self {
        Self { a: 1.0, b: 0.0 }
    }
}

impl PlattScaler {
    pub fn new(a: f64, b: f64) -> Self {
        Self { a, b }
    }

    pub fn fit(scores: &[f64], labels: &[f64]) -> Self {
        let positive_count = labels.iter().filter(|&&label| label > 0.0).count();
        let negative_count = labels.len().saturating_sub(positive_count);

        if scores.is_empty() || positive_count == 0 || negative_count == 0 {
            return Self::default();
        }

        let positive_target = (positive_count as f64 + 1.0) / (positive_count as f64 + 2.0);
        let negative_target = 1.0 / (negative_count as f64 + 2.0);
        let targets = labels
            .iter()
            .map(|&label| {
                if label > 0.0 {
                    positive_target
                } else {
                    negative_target
                }
            })
            .collect::<Vec<f64>>();

        let mut a = 0.0;
        let mut b = ((negative_count as f64 + 1.0) / (positive_count as f64 + 1.0)).ln();
        let lambda = 1e-6;

        for _ in 0..100 {
            let mut g_a = 0.0;
            let mut g_b = 0.0;
            let mut h_aa = lambda;
            let mut h_ab = 0.0;
            let mut h_bb = lambda;

            for (&score, &target) in scores.iter().zip(targets.iter()) {
                let probability = sigmoid(a * score + b);
                let error = probability - target;
                let weight = probability * (1.0 - probability);

                g_a += error * score;
                g_b += error;
                h_aa += weight * score * score;
                h_ab += weight * score;
                h_bb += weight;
            }

            if g_a.abs() < 1e-5 && g_b.abs() < 1e-5 {
                break;
            }

            let determinant = h_aa * h_bb - h_ab * h_ab;
            if determinant.abs() < 1e-12 {
                break;
            }

            let delta_a = (h_bb * g_a - h_ab * g_b) / determinant;
            let delta_b = (-h_ab * g_a + h_aa * g_b) / determinant;

            a -= delta_a;
            b -= delta_b;

            if delta_a.abs() < 1e-5 && delta_b.abs() < 1e-5 {
                break;
            }
        }

        Self { a, b }
    }

    pub fn calibrate(&self, score: f64) -> f64 {
        sigmoid(self.a * score + self.b)
    }
}

fn sigmoid(value: f64) -> f64 {
    if value >= 0.0 {
        let exp = (-value).exp();
        1.0 / (1.0 + exp)
    } else {
        let exp = value.exp();
        exp / (1.0 + exp)
    }
}
