use mathru::algebra::linear::matrix::General;

#[test]
#[should_panic]
fn non_matching_content() {
    let a: General<f64> = matrix![1.0, 2.0];
    let b: General<f64> = matrix![2.0, 1.0];

    assert_relative_eq!(a, b);
}

#[test]
#[should_panic]
fn non_matching_dimensions() {
    let a: General<f64> = matrix![1.0, 2.0];
    let b: General<f64> = matrix![-1.0];

    assert_relative_eq!(a, b);
}
