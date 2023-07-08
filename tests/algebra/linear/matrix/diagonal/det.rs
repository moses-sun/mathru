use crate::mathru::algebra::linear::matrix::Determinant;
use mathru::algebra::linear::matrix::Diagonal;

#[test]
fn determinant() {
    let a: Diagonal<f64> = Diagonal::new(&[1.0, -2.0, 3.0]);

    assert_abs_diff_eq!(-6.0, a.det());
}
