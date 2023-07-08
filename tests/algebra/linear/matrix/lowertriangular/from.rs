use mathru::algebra::linear::matrix::{General, LowerTriangular, UnitLowerTriangular};

#[test]
fn from() {
    let a: UnitLowerTriangular<f64> = matrix![1.0, 0.0;
                                              2.0, 1.0]
    .into();

    let b = LowerTriangular::from(a);
    let b_ref: LowerTriangular<f64> = matrix![1.0, 0.0;
                                              2.0, 1.0]
    .into();

    assert_abs_diff_eq!(b_ref, b);
}
