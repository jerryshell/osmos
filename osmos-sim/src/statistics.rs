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

#[cfg(test)]
mod tests {
    mod mean {
        #[test]
        fn test() {
            let data = [3.0, 1.0, 6.0, 1.0, 5.0, 8.0, 1.0, 8.0, 10.0, 11.0];
            let data_mean = crate::statistics::mean(&data);
            assert_eq!(data_mean, Some(5.4));
        }
    }

    mod std_deviation {
        #[test]
        fn test() {
            let data = [3.0, 1.0, 6.0, 1.0, 5.0, 8.0, 1.0, 8.0, 10.0, 11.0];
            let data_std_deviation = crate::statistics::std_deviation(&data);
            assert_eq!(data_std_deviation, Some(3.6110940170535573));
        }
    }

    mod zscore {
        #[test]
        fn test() {
            let data = [3.0, 1.0, 6.0, 1.0, 5.0, 8.0, 1.0, 8.0, 10.0, 11.0];
            let data_mean = crate::statistics::mean(&data).expect("mean error");
            let data_std_deviation =
                crate::statistics::std_deviation(&data).expect("std_deviation error");
            let data_4_zscore = crate::statistics::zscore(data[4], data_mean, data_std_deviation);
            assert_eq!(data_4_zscore, -0.11076975512434237);
        }
    }

    mod array_zscore {
        #[test]
        fn test() {
            let data = [3.0, 1.0, 6.0, 1.0, 5.0, 8.0, 1.0, 8.0, 10.0, 11.0];
            let az = crate::statistics::array_zscore(&data);
            assert_eq!(
                az,
                [
                    -0.6646185307460537,
                    -1.2184673063677651,
                    0.16615463268651331,
                    -1.2184673063677651,
                    -0.11076975512434237,
                    0.7200034083082247,
                    -1.2184673063677651,
                    0.7200034083082247,
                    1.2738521839299362,
                    1.5507765717407918
                ]
            );
        }
    }
}
