#[test]
fn test_zero() {
    assert_abs_diff_eq!(0.0, 0.0);
}

#[test]
fn test_negative_zero() {
    assert_abs_diff_eq!(0.0, -0.0);
}
