//! Fundamental dataset metrics.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BinaryMetrics {
    tp: usize,
    fp: usize,
    tn: usize,
    fn_: usize,
}

impl BinaryMetrics {
    pub fn new(tp: usize, fp: usize, tn: usize, fn_: usize) -> Self {
        Self { tp, fp, tn, fn_ }
    }

    pub fn tp(&self) -> usize {
        self.tp
    }

    pub fn fp(&self) -> usize {
        self.fp
    }

    pub fn tn(&self) -> usize {
        self.tn
    }

    pub fn fn_(&self) -> usize {
        self.fn_
    }

    pub fn precision(&self) -> f64 {
        ratio(self.tp, self.tp + self.fp)
    }

    pub fn recall(&self) -> f64 {
        ratio(self.tp, self.tp + self.fn_)
    }

    pub fn f1(&self) -> f64 {
        let p = self.precision();
        let r = self.recall();
        if p == 0.0 && r == 0.0 {
            0.0
        } else {
            2.0 * p * r / (p + r)
        }
    }

    pub fn accuracy(&self) -> f64 {
        ratio(self.tp + self.tn, self.tp + self.tn + self.fp + self.fn_)
    }

    pub fn from_labels<T: AsRef<str>, U: AsRef<str>>(
        y_true: &[T],
        y_pred: &[U],
        positive_label: &str,
    ) -> Result<Self, MetricError> {
        if y_true.len() != y_pred.len() {
            return Err(MetricError::LengthMismatch {
                expected: y_true.len(),
                actual: y_pred.len(),
            });
        }

        let mut metrics = Self::default();
        for (true_label, pred_label) in y_true.iter().zip(y_pred.iter()) {
            let is_pos = true_label.as_ref() == positive_label;
            let pred_pos = pred_label.as_ref() == positive_label;
            match (is_pos, pred_pos) {
                (true, true) => metrics.tp += 1,
                (false, true) => metrics.fp += 1,
                (false, false) => metrics.tn += 1,
                (true, false) => metrics.fn_ += 1,
            }
        }

        Ok(metrics)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricError {
    LengthMismatch { expected: usize, actual: usize },
}

fn ratio(numer: usize, denom: usize) -> f64 {
    if denom == 0 {
        0.0
    } else {
        numer as f64 / denom as f64
    }
}
