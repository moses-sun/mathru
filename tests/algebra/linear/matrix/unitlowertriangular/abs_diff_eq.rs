use mathru::algebra::linear::matrix::{General, UnitLowerTriangular};

#[test]
#[should_panic]
fn non_matching_content() {
    let a: UnitLowerTriangular<f64> = matrix![1.0, 0.0;
                                          2.0, 1.0]
    .into();
    let b: UnitLowerTriangular<f64> = matrix![1.0, 0.0;
                                          2.0, 1.0]
    .into();

    assert_abs_diff_eq!(a, b);
}

#[test]
#[should_panic]
fn non_matching_dimensions() {
    let a: UnitLowerTriangular<f64> = matrix![1.0, 0.0;
                                          2.0, 1.0]
    .into();

    let b: UnitLowerTriangular<f64> = matrix![-1.0].into();

    assert_abs_diff_eq!(a, b);
}

#[test]
fn equal() {
    let a: UnitLowerTriangular<f64> = matrix![1.0, 0.0;
                                          2.0, 1.0]
    .into();
    let b: UnitLowerTriangular<f64> = matrix![1.0, 0.0;
                                          2.0, 1.0]
    .into();

    assert_abs_diff_eq!(a, b);
}
