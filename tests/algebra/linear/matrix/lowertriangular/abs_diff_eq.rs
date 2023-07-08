use mathru::algebra::linear::matrix::{General, LowerTriangular};

#[test]
#[should_panic]
fn non_matching_content() {
    let a: LowerTriangular<f64> = matrix![1.0, 0.0;
                                          2.0, 2.0]
    .into();
    let b: LowerTriangular<f64> = matrix![2.0, 0.0;
                                          2.0, 2.0]
    .into();

    assert_abs_diff_eq!(a, b);
}

#[test]
#[should_panic]
fn non_matching_dimensions() {
    let a: LowerTriangular<f64> = matrix![1.0, 0.0;
                                          2.0, 2.0]
    .into();

    let b: LowerTriangular<f64> = matrix![-1.0].into();

    assert_abs_diff_eq!(a, b);
}

#[test]
fn equal() {
    let a: LowerTriangular<f64> = matrix![2.0, 0.0;
                                          2.0, 2.0]
    .into();
    let b: LowerTriangular<f64> = matrix![2.0, 0.0;
                                          2.0, 2.0]
    .into();

    assert_abs_diff_eq!(a, b);
}
