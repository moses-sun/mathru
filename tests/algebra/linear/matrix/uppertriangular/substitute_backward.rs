use mathru::algebra::linear::matrix::General;
use mathru::algebra::linear::matrix::SubstituteBackward;
use mathru::algebra::linear::matrix::UpperTriangular;
use mathru::algebra::linear::vector::Vector;

#[test]
fn subst_backward() {
    let a: UpperTriangular<f64> = matrix![  1.0, 2.0, 3.0;
                                            0.0, 4.0, 5.0;
                                            0.0, 0.0, 6.0]
    .into();

    let b: Vector<f64> = vector![7.0; 8.0; 9.0];

    let c_ref: Vector<f64> = vector![2.25; 0.125; 1.5];

    let c: Vector<f64> = a.substitute_backward(b).unwrap();

    assert_relative_eq!(c, c_ref, epsilon = 1.0e-10);
}

#[cfg(feature = "native")]
#[test]
fn subst_backward_zero_diagonal() {
    let a: UpperTriangular<f64> = matrix![  1.0, 2.0, 3.0;
                                            0.0, 0.0, 5.0;
                                            0.0, 0.0, 6.0]
    .into();

    let b: Vector<f64> = vector![8.0; 
                                7.5; 
                                9.0];

    let x_ref: Vector<f64> = vector![1.5; 
                                    1.0; 
                                    1.5];

    let x: Vector<f64> = a.substitute_backward(b).unwrap();

    assert_relative_eq!(x, x_ref, epsilon = 1.0e-10);
}

#[cfg(feature = "native")]
#[test]
fn subst_backward_error() {
    let a: UpperTriangular<f64> = matrix![  1.0, 2.0, 3.0;
                                            0.0, 0.0, 5.0;
                                            0.0, 0.0, 6.0]
    .into();

    let b: Vector<f64> = vector![8.0; 
                                7.0; 
                                9.0];

    let x: Result<Vector<f64>, ()> = a.substitute_backward(b);

    assert_eq!(x, Result::Err(()));
}

#[cfg(feature = "native")]
#[test]
fn subst_backward_equals_zero() {
    let a: UpperTriangular<f64> = matrix![  1.0, 2.0, 3.0;
                                            0.0, 0.0, 5.0;
                                            0.0, 0.0, 6.0]
    .into();

    let b: Vector<f64> = vector![0.0; 
                                0.0; 
                                0.0];

    let x_ref: Vector<f64> = vector![-2.0; 
                                    1.0; 
                                    0.0];

    let x: Vector<f64> = a.substitute_backward(b).unwrap();

    assert_relative_eq!(x, x_ref, epsilon = 1.0e-10);
}

#[cfg(feature = "native")]
#[test]
fn subst_backward_equals_zero_1() {
    let a: UpperTriangular<f64> = matrix![  1.0, 2.0, 3.0;
                                            0.0, 0.0, 0.0;
                                            0.0, 0.0, 0.0]
    .into();

    let b: Vector<f64> = vector![0.0; 
                                0.0; 
                                0.0];

    let x_ref: Vector<f64> = vector![-5.0; 
                                    1.0; 
                                    1.0];

    let x: Vector<f64> = a.substitute_backward(b).unwrap();

    assert_relative_eq!(x, x_ref, epsilon = 1.0e-10);
}
