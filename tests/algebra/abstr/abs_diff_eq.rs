#[test]
fn test_zero() {
    assert_abs_diff_eq!(0.0, 0.0);
}

#[test]
fn test_negative_zero() {
    assert_abs_diff_eq!(0.0, -0.0);
}

#[test]
fn test_bigger() {
    let a = 0.0;
    let tol = f64::EPSILON;
    assert_abs_diff_eq!(a, a + tol);
}

#[test]
fn test_smaller() {
    let a = 0.0;
    let tol = f64::EPSILON;
    assert_abs_diff_eq!(a, a - tol);
}

#[test]
fn abs_diff_eq_12() {
    let a = 0.0f64;

    assert_abs_diff_eq!(a, -7.347880794884119e-16, epsilon = 4.0 * f64::EPSILON);
}
