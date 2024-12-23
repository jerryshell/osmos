#[test]
fn mean_test() {
    let data = [3.0, 1.0, 6.0, 1.0, 5.0, 8.0, 1.0, 8.0, 10.0, 11.0];
    let data_mean = osmos_sim::statistics::mean(&data);
    assert_eq!(data_mean, Some(5.4));
}

#[test]
fn std_deviation_test() {
    let data = [3.0, 1.0, 6.0, 1.0, 5.0, 8.0, 1.0, 8.0, 10.0, 11.0];
    let data_std_deviation = osmos_sim::statistics::std_deviation(&data);
    assert_eq!(data_std_deviation, Some(3.6110942));
}

#[test]
fn zscore_test() {
    let data = [3.0, 1.0, 6.0, 1.0, 5.0, 8.0, 1.0, 8.0, 10.0, 11.0];
    let data_mean = osmos_sim::statistics::mean(&data).expect("mean error");
    let data_std_deviation =
        osmos_sim::statistics::std_deviation(&data).expect("std_deviation error");
    let data_4_zscore = osmos_sim::statistics::zscore(data[4], data_mean, data_std_deviation);
    assert_eq!(data_4_zscore, -0.11076978);
}

#[test]
fn array_zscore_test() {
    let data = [3.0, 1.0, 6.0, 1.0, 5.0, 8.0, 1.0, 8.0, 10.0, 11.0];
    let az = osmos_sim::statistics::array_zscore(&data);
    assert_eq!(
        az,
        [
            -0.6646185,
            -1.2184672,
            0.1661546,
            -1.2184672,
            -0.11076978,
            0.72000337,
            -1.2184672,
            0.72000337,
            1.2738521,
            1.5507765
        ]
    );
}
