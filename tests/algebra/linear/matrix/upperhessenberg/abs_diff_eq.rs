use mathru::algebra::linear::matrix::{General, UpperHessenberg};

#[test]
#[should_panic]
fn non_matching_content() {
    let a: UpperHessenberg<f64> = matrix![2.0, 3.0, 5.0;
                                         -4.0, 2.0, 6.0;
                                        0.0, -2.0, -0.5]
    .into();
    let b: UpperHessenberg<f64> = matrix![1.0, 3.0, 5.0;
                                        -4.0, 2.0, 6.0;
                                        0.0, -2.0, -0.5]
    .into();

    assert_abs_diff_eq!(a, b);
}

#[test]
#[should_panic]
fn non_matching_dimensions() {
    let a: UpperHessenberg<f64> = matrix![2.0, 3.0, 5.0;
                                          -4.0, 2.0, 6.0;
                                         0.0, -2.0, -0.5]
    .into();

    let b: UpperHessenberg<f64> = matrix![-1.0].into();

    assert_abs_diff_eq!(a, b);
}

#[test]
fn equal() {
    let a: UpperHessenberg<f64> = matrix![2.0, 3.0, 5.0;
                                          -4.0, 2.0, 6.0;
                                         0.0, -2.0, -0.5]
    .into();
    let b: UpperHessenberg<f64> = matrix![2.0, 3.0, 5.0;
                                        -4.0, 2.0, 6.0;
                                        0.0, -2.0, -0.5]
    .into();

    assert_abs_diff_eq!(a, b);
}
