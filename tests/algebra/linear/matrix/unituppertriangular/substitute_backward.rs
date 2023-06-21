use mathru::algebra::linear::matrix::General;
use mathru::algebra::linear::matrix::SubstituteBackward;
use mathru::algebra::linear::matrix::UnitUpperTriangular;
use mathru::algebra::linear::vector::Vector;

#[test]
fn subst_backward() {
    let a: UnitUpperTriangular<f64> = matrix![  1.0, 2.0, 3.0;
                                                0.0, 1.0, 5.0;
                                                0.0, 0.0, 1.0]
    .into();

    let b: Vector<f64> = vector![7.0; 8.0; 9.0];

    let c_ref: Vector<f64> = vector![54.0; -37.0; 9.0];

    let c: Vector<f64> = a.substitute_backward(b).unwrap();

    assert_relative_eq!(c, c_ref, epsilon = 1.0e-10);
}
