use crate::ml::models::svm::SVMKernelType;

pub fn compute(kernel: &SVMKernelType, left: &[f64], right: &[f64]) -> f64 {
    match kernel {
        SVMKernelType::Linear => dot(left, right),
        SVMKernelType::Rbf { gamma } => {
            let squared_distance = left
                .iter()
                .zip(right.iter())
                .map(|(l, r)| {
                    let delta = l - r;
                    delta * delta
                })
                .sum::<f64>();
            (-gamma * squared_distance).exp()
        }
    }
}

fn dot(left: &[f64], right: &[f64]) -> f64 {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l * r)
        .sum::<f64>()
}
