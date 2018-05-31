use summary;

#[test]
fn calculate_mean() {
    let data = vec![1, 10, 2, 5, 7, 2];
    let avg = summary::average(&data);
    assert_eq!(avg, Some(27.0 / 6.0));
}

#[test]
fn calculate_median_even() {
    let mut data = vec![1, 10, 2, 5, 7, 2];
    let median = summary::median(&mut data);
    assert_eq!(median, Some((2.0 + 5.0) / 2.0));
}

#[test]
fn calculate_median_odd() {
    let mut data = vec![1, 10, 2, 5, 7, 2, 5];
    let median = summary::median(&mut data);
    assert_eq!(median, Some(5.0));
}

#[test]
fn calculate_mode() {
    let data = vec![1, 10, 2, 5, 7, 2];
    let avg = summary::mode(&data);
    assert_eq!(avg, Some(2));
}