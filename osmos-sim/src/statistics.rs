pub fn mean(data: &[f32]) -> Option<f32> {
    let sum = data.iter().sum::<f32>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

pub fn std_deviation(data: &[f32]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - *value;

                    diff * diff
                })
                .sum::<f32>()
                / count as f32;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

pub fn zscore(data: f32, data_mean: f32, data_std_deviation: f32) -> f32 {
    let diff = data - data_mean;
    diff / data_std_deviation
}

pub fn array_zscore(data: &[f32]) -> Vec<f32> {
    let data_mean = crate::statistics::mean(data).expect("mean error");
    let data_std_deviation = crate::statistics::std_deviation(data).expect("std_deviation error");
    data.iter()
        .map(|&item| zscore(item, data_mean, data_std_deviation))
        .collect()
}
