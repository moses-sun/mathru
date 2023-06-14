use mathru::algebra::abstr::Complex;
use mathru::algebra::linear::{
    matrix::{General, Solve},
    Vector,
};

#[test]
fn solve_matrix_f32() {
    let a: General<f32> = matrix![   1.0, -2.0, 3.0;
                                    2.0, -5.0, 12.0;
                                    0.0, 2.0, -10.0];

    let x_ref: General<f32> = matrix![   -13.0, 7.0, 4.5;
                                        -10.0, 5.0, 3.0;
                                        -2.0, 1.0, 0.5];

    let id: General<f32> = General::one(3);

    let x: General<f32> = a.solve(&id).unwrap();

    assert_relative_eq!(x, x_ref);
}

#[test]
fn solve_matrix_f64() {
    let a: General<f64> = matrix![   1.0, -2.0, 3.0;
                                    2.0, -5.0, 12.0;
                                    0.0, 2.0, -10.0];

    let x_ref: General<f64> = matrix![   -13.0, 7.0, 4.5;
                                        -10.0, 5.0, 3.0;
                                        -2.0, 1.0, 0.5];

    let id: General<f64> = General::one(3);

    let x: General<f64> = a.solve(&id).unwrap();

    assert_relative_eq!(x, x_ref);
}

#[test]
fn solve_vector_f64_2() {
    let a: General<f64> = matrix![  -2.0, 2.0, 1.0;
                                    2.0, -3.0, 1.0;
                                    2.0, -1.0, 1.0];

    let x_ref: Vector<f64> = vector![ 0.0;
                                      0.0;
                                      0.0];

    let rhs: Vector<f64> = vector![ 0.0;
                                    0.0;
                                    0.0];

    let x: Vector<f64> = a.solve(&rhs).unwrap();

    assert_relative_eq!(x, x_ref);
}

#[test]
fn solve_matrix_complex_f32() {
    let a: General<Complex<f32>> = matrix![  Complex::new(1.0, 0.0), Complex::new(-2.0, 0.0), Complex::new(3.0, 0.0);
                                            Complex::new(2.0, 0.0), Complex::new(-5.0, 0.0), Complex::new(12.0, 0.0);
                                            Complex::new(0.0, 0.0), Complex::new(2.0, 0.0), Complex::new(-10.0, 0.0)];

    let x_ref: General<Complex<f32>> = matrix![  Complex::new(-13.0, 0.0), Complex::new(7.0, 0.0), Complex::new(4.5, 0.0);
                                                Complex::new(-10.0, 0.0), Complex::new(5.0, 0.0), Complex::new(3.0, 0.0);
                                                Complex::new(-2.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.5, 0.0)];

    let id: General<Complex<f32>> = General::one(3);

    let x: General<Complex<f32>> = a.solve(&id).unwrap();

    assert_relative_eq!(x, x_ref);
}

#[test]
fn solve_matrix_complex_f64() {
    let a: General<Complex<f64>> = matrix![  Complex::new(1.0, 0.0), Complex::new(-2.0, 0.0), Complex::new(3.0, 0.0);
                                            Complex::new(2.0, 0.0), Complex::new(-5.0, 0.0), Complex::new(12.0, 0.0);
                                            Complex::new(0.0, 0.0), Complex::new(2.0, 0.0), Complex::new(-10.0, 0.0)];

    let x_ref: General<Complex<f64>> = matrix![  Complex::new(-13.0, 0.0), Complex::new(7.0, 0.0), Complex::new(4.5, 0.0);
                                                Complex::new(-10.0, 0.0), Complex::new(5.0, 0.0), Complex::new(3.0, 0.0);
                                                Complex::new(-2.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.5, 0.0)];
    let id: General<Complex<f64>> = General::one(3);

    let x: General<Complex<f64>> = a.solve(&id).unwrap();

    assert_relative_eq!(x, x_ref);
}

#[test]
fn solve_vector_f32() {
    let a: General<f32> = matrix![   6.0, 2.0, -1.0;
                                    -3.0, 5.0, 3.0;
                                    -2.0, 1.0, 3.0];

    let b: Vector<f32> = vector![   48.0;
                                    49.0;
                                    24.0];

    let x: Vector<f32> = a.solve(&b).unwrap();
    let x_ref: Vector<f32> = vector![   7.0;
                                        8.0;
                                        10.0];

    assert_relative_eq!(x, x_ref);
}

#[test]
fn solve_vector_f64() {
    let a: General<f64> = matrix![   6.0, 2.0, -1.0;
                                    -3.0, 5.0, 3.0;
                                    -2.0, 1.0, 3.0];

    let b: Vector<f64> = vector![   48.0;
                                    49.0;
                                    24.0];

    let x: Vector<f64> = a.solve(&b).unwrap();
    let x_ref: Vector<f64> = vector![   7.0;
                                        8.0;
                                        10.0];

    assert_relative_eq!(x, x_ref);
}

#[test]
fn solve_vector_complex_f32() {
    let a: General<Complex<f32>> = matrix![  Complex::new(1.0, 0.0), Complex::new(0.0, 1.0), Complex::new(-3.0, 1.0);
                                            Complex::new(2.0, 0.0), Complex::new(1.0, 3.0), Complex::new(-4.0, 2.0);
                                            Complex::new(0.0, 2.0), Complex::new(-2.0, 0.0), Complex::new(-2.0, -3.0)];

    let b: Vector<Complex<f32>> = vector![  Complex::new(-1.0, -1.0);
                                            Complex::new(0.0, 2.0);
                                            Complex::new(-1.0, 1.0)];

    let x: Vector<Complex<f32>> = a.solve(&b).unwrap();
    let x_ref: Vector<Complex<f32>> = vector![  Complex::new(4.0, 0.0);
                                                Complex::new(1.0, 1.0);
                                                Complex::new(1.0, 1.0)];

    assert_relative_eq!(x, x_ref);
}

#[test]
fn solve_vector_complex_f64() {
    let a: General<Complex<f64>> = matrix![  Complex::new(1.0, 0.0), Complex::new(0.0, 1.0), Complex::new(-3.0, 1.0);
                                            Complex::new(2.0, 0.0), Complex::new(1.0, 3.0), Complex::new(-4.0, 2.0);
                                            Complex::new(0.0, 2.0), Complex::new(-2.0, 0.0), Complex::new(-2.0, -3.0)];

    let b: Vector<Complex<f64>> = vector![  Complex::new(-1.0, -1.0);
                                            Complex::new(0.0, 2.0);
                                            Complex::new(-1.0, 1.0)];

    let x: Vector<Complex<f64>> = a.solve(&b).unwrap();
    let x_ref: Vector<Complex<f64>> = vector![  Complex::new(4.0, 0.0);
                                                Complex::new(1.0, 1.0);
                                                Complex::new(1.0, 1.0)];

    assert_relative_eq!(x, x_ref);
}

/**
* Many thanks to Lukas Prediger @lumip
* https://gitlab.com/matthiaseiholzer/mathru/-/issues/7
*/
#[test]
fn negative_pivot_in_lu_decomposition() {
    let m: General<f64> =
        matrix![1., 0., 0., 0. ; 1., 0., -1., 0. ; 0., 1., 0., -1. ; 0., 1., 0., 0. ];
    let b: Vector<f64> = vector![3.; 0.; 0.; 1.];

    let x: Vector<f64> = m.solve(&b).unwrap();

    assert_relative_eq!(&m * &x, b);
}

#[cfg(feature = "native")]
#[test]
fn solve_vector_infinite_many_solutions() {
    let a: General<f32> = matrix![   6.0, 2.0, -1.0;
                                    -3.0, 5.0, 3.0;
                                    -3.0, 5.0, 3.0];

    let b: Vector<f32> = vector![   48.0;
                                    49.0;
                                    49.0];

    let x: Vector<f32> = a.solve(&b).unwrap();
    let x_ref: Vector<f32> = vector![   4.25;
                                        11.75;
                                        1.0];

    assert_relative_eq!(x, x_ref);
    assert_relative_eq!(b, &a * &x);
}

#[cfg(feature = "native")]
#[test]
fn solve_vector_infinite_many_solutions1() {
    let a: General<f32> = matrix![   -3.0, -3.0, 3.0;
                                    3.0, -9.0, 3.0;
                                    6.0, -6.0, 0.0];

    let b: Vector<f32> = vector![   0.0;
                                    0.0;
                                    0.0];

    let x: Vector<f32> = a.solve(&b).unwrap();
    let x_ref: Vector<f32> = vector![   0.5;
                                        0.5;
                                        1.0];

    assert_relative_eq!(x, x_ref);
    assert_relative_eq!(b, &a * &x);
}

#[test]
fn solve_vector_no_solution() {
    let a: General<f32> = matrix![   1.0, 1.0, 1.0;
                                    1.0, 1.0, 2.0;
                                    1.0, 1.0, 3.0];

    let b: Vector<f32> = vector![   1.0;
                                    3.0;
                                    -1.0];

    assert_eq!(a.solve(&b), Err(()));
}
