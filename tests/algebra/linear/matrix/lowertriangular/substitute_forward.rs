use mathru::algebra::linear::matrix::General;
use mathru::algebra::linear::matrix::LowerTriangular;
use mathru::algebra::linear::matrix::SubstituteForward;
use mathru::algebra::linear::vector::Vector;

#[test]
fn subst_forward() {
    let a: LowerTriangular<f64> = matrix![  6.0, 0.0, 0.0;
                                            5.0, 4.0, 0.0;
                                            3.0, 2.0, 1.0]
    .into();

    let b: Vector<f64> = vector![9.0; 
                                8.0; 
                                7.0];

    let x_ref: Vector<f64> = vector![1.5; 
                                    0.125; 
                                    2.25];

    let x: Vector<f64> = a.substitute_forward(b).unwrap();

    assert_relative_eq!(x, x_ref, epsilon = 1.0e-10);
}
#[cfg(feature = "native")]
#[test]
fn subst_forward_zero_diagonal() {
    let a: LowerTriangular<f64> = matrix![  6.0, 0.0, 0.0;
                                            5.0, 0.0, 0.0;
                                            3.0, 2.0, 1.0]
    .into();

    let b: Vector<f64> = vector![9.0; 
                                7.5; 
                                7.0];

    let x_ref: Vector<f64> = vector![1.5; 
                                    1.0; 
                                    0.5];

    let x: Vector<f64> = a.substitute_forward(b).unwrap();

    assert_relative_eq!(x, x_ref, epsilon = 1.0e-10);
}

#[cfg(feature = "native")]
#[test]
fn subst_forward_error() {
    let a: LowerTriangular<f64> = matrix![  6.0, 0.0, 0.0;
                                            5.0, 0.0, 0.0;
                                            3.0, 2.0, 1.0]
    .into();

    let b: Vector<f64> = vector![9.0; 
                                7.0; 
                                7.0];

    let c: Result<Vector<f64>, ()> = a.substitute_forward(b);

    assert_eq!(c, Result::Err(()));
}

#[cfg(feature = "native")]
#[test]
fn subst_forward_equals_zero() {
    let a: LowerTriangular<f64> = matrix![  6.0, 0.0, 0.0;
                                            5.0, 0.0, 0.0;
                                            3.0, 2.0, 1.0]
    .into();

    let b: Vector<f64> = vector![0.0; 
                                0.0; 
                                0.0];

    let x_ref: Vector<f64> = vector![0.0; 
                                    1.0; 
                                    -2.0];

    let x: Vector<f64> = a.substitute_forward(b.clone()).unwrap();

    assert_relative_eq!(x, x_ref, epsilon = 1.0e-10);
    assert_abs_diff_eq!(b, General::from(a) * x);
}
