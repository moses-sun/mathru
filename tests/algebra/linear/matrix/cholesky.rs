use crate::mathru::algebra::abstr::Zero;
use mathru::algebra::abstr::Complex;
use mathru::algebra::linear::Matrix;

#[test]
fn cholesky_f32() {
    let a: Matrix<f32> = matrix![   2.0, -1.0, 0.0;
                                    -1.0, 2.0, -1.0;
                                    0.0, -1.0,  2.0];

    let g_ref: Matrix<f32> = matrix![   1.414213562373095,   0.000000000000000,   0.000000000000000;
                                        -7.071067811865475e-1,   1.224744871391589,   0.000000000000000;
                                        0.000000000000000,  -8.164965809277261e-1,   1.154700538379251];

    let g: Matrix<f32> = a.dec_cholesky().unwrap().l();

    assert_relative_eq!(g, g_ref, epsilon = 1.0e-5);
}

#[test]
fn cholesky_f64() {
    let a: Matrix<f64> = matrix![   2.0, -1.0, 0.0;
                                    -1.0, 2.0, -1.0;
                                    0.0, -1.0,  2.0];

    let g_ref: Matrix<f64> = matrix![   1.414213562373095,   0.000000000000000,   0.000000000000000;
                                        -7.071067811865475e-1,   1.224744871391589,   0.000000000000000;
                                        0.000000000000000,  -8.164965809277261e-1,   1.154700538379251];

    let g: Matrix<f64> = a.dec_cholesky().unwrap().l();

    assert_relative_eq!(g, g_ref, epsilon = 1.0e-10);
}

#[test]
fn cholesky_f32_randomized() {
    use crate::mathru::algebra::linear::matrix::Transpose;
    use rand::distributions::OpenClosed01;
    use rand::{thread_rng, Rng};

    for _i in 0..100 {
        let r = || thread_rng().sample(OpenClosed01);
        let l = Matrix::new(
            3,
            3,
            vec![1.0 + r(), r(), r(), 0.0, 1. + r(), r(), 0.0, 0.0, 1.0 + r()],
        );
        let a = l.clone() * l.clone().transpose();
        let k = a.dec_cholesky().unwrap().l();
        assert_relative_eq!(l, k, epsilon = 1.0e-10);
    }
}

#[test]
fn cholesky_complex_f32() {
    let a: Matrix<Complex<f32>> = matrix![  Complex::new(2.0, 0.0), Complex::new(-1.0, 0.0), Complex::zero();
                                            Complex::new(-1.0, 0.0), Complex::new(2.0, 0.0), Complex::new(-1.0, 0.0);
                                            Complex::zero(), Complex::new(-1.0, 0.0),  Complex::new(2.0, 0.0)];

    let g_ref: Matrix<Complex<f32>> = matrix![  Complex::new(1.414213562373095, 0.0), Complex::zero(), Complex::zero();
                                                Complex::new(-7.071067811865475e-1, 0.0), Complex::new(1.224744871391589, 0.0), Complex::zero();
                                                Complex::zero(), Complex::new(-8.164965809277261e-1, 0.0), Complex::new(1.154700538379251, 0.0)];

    let g: Matrix<Complex<f32>> = a.dec_cholesky().unwrap().l();

    assert_relative_eq!(g, g_ref, epsilon = Complex::new(1.0e-5, 1.0e-5));
}

#[test]
fn cholesky_complex_f64() {
    let a: Matrix<Complex<f64>> = matrix![  Complex::new(2.0, 0.0), Complex::new(-1.0, 0.0), Complex::zero();
                                            Complex::new(-1.0, 0.0), Complex::new(2.0, 0.0), Complex::new(-1.0, 0.0);
                                            Complex::zero(), Complex::new(-1.0, 0.0),  Complex::new(2.0, 0.0)];

    let g_ref: Matrix<Complex<f64>> = matrix![  Complex::new(1.414213562373095, 0.0), Complex::zero(), Complex::zero();
                                                Complex::new(-7.071067811865475e-1, 0.0), Complex::new(1.224744871391589, 0.0), Complex::zero();
                                                Complex::zero(), Complex::new(-8.164965809277261e-1, 0.0), Complex::new(1.154700538379251, 0.0)];

    let g: Matrix<Complex<f64>> = a.dec_cholesky().unwrap().l();

    assert_relative_eq!(g, g_ref, epsilon = Complex::new(1.0e-10, 1.0e-10));
}
