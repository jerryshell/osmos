pub fn mean(data: &[f64]) -> Option<f64> {
    let sum = data.iter().sum::<f64>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

pub fn std_deviation(data: &[f64]) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - *value;

                    diff * diff
                })
                .sum::<f64>()
                / count as f64;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

pub fn zscore(data: f64, data_mean: f64, data_std_deviation: f64) -> f64 {
    let diff = data - data_mean;
    diff / data_std_deviation
}

pub fn array_zscore(data: &[f64]) -> Vec<f64> {
    let data_mean = crate::statistics::mean(data).expect("mean error");
    let data_std_deviation = crate::statistics::std_deviation(data).expect("std_deviation error");
    data.iter()
        .map(|&item| zscore(item, data_mean, data_std_deviation))
        .collect()
}
