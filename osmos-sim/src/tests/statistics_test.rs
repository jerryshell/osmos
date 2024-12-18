#[test]
fn mean_test() {
    let data = [3.0, 1.0, 6.0, 1.0, 5.0, 8.0, 1.0, 8.0, 10.0, 11.0];
    let data_mean = crate::statistics::mean(&data);
    assert_eq!(data_mean, Some(5.4));
}

#[test]
fn std_deviation_test() {
    let data = [3.0, 1.0, 6.0, 1.0, 5.0, 8.0, 1.0, 8.0, 10.0, 11.0];
    let data_std_deviation = crate::statistics::std_deviation(&data);
    assert_eq!(data_std_deviation, Some(3.6110940170535573));
}

#[test]
fn zscore_test() {
    let data = [3.0, 1.0, 6.0, 1.0, 5.0, 8.0, 1.0, 8.0, 10.0, 11.0];
    let data_mean = crate::statistics::mean(&data).expect("mean error");
    let data_std_deviation = crate::statistics::std_deviation(&data).expect("std_deviation error");
    let data_4_zscore = crate::statistics::zscore(data[4], data_mean, data_std_deviation);
    assert_eq!(data_4_zscore, -0.11076975512434237);
}

#[test]
fn array_zscore_test() {
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
