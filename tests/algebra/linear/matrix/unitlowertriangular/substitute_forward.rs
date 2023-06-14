use mathru::algebra::linear::matrix::SubstituteForward;
use mathru::algebra::linear::matrix::UnitLowerTriangular;
use mathru::algebra::linear::General;
use mathru::algebra::linear::Vector;

#[test]
fn subst_forward() {
    let a: UnitLowerTriangular<f64> = matrix![  1.0, 0.0, 0.0;
                                            5.0, 1.0, 0.0;
                                            3.0, 2.0, 1.0]
    .into();

    let b: Vector<f64> = vector![9.0; 8.0; 7.0];

    let c_ref: Vector<f64> = vector![9.0; -37.0; 54.0];

    let c: Vector<f64> = a.substitute_forward(b).unwrap();

    assert_relative_eq!(c, c_ref, epsilon = 1.0e-10);
}
