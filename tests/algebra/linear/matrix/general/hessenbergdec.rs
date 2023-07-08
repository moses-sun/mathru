use crate::mathru::algebra::linear::matrix::{
    General, HessenbergDecomposition, Transpose, UpperHessenberg,
};
use mathru::algebra::abstr::Complex;

#[test]
fn dec_f32() {
    let a: General<f32> = matrix![   1.0, 5.0, 3.0;
                                    1.0, 0.0, -7.0;
                                    3.0, 8.0, 9.0];

    let h_ref: UpperHessenberg<f32> = matrix![   1.0, -4.427188724235731, -3.7947331922020537;
                                        -3.162277660168379, 8.4, 5.2;
                                        0.0, -9.8, 0.6]
    .into();

    let q_ref: General<f32> = matrix![   1.0, 0.0, 0.0;
                                        0.0, -0.316227766016838, -0.9486832980505137;
                                        0.0, -0.9486832980505137, 0.3162277660168381];

    let (q, h): (General<f32>, UpperHessenberg<f32>) = a.dec_hessenberg().qh();

    assert_relative_eq!(q, q_ref, epsilon = 1.0e-5);
    assert_relative_eq!(h, h_ref, epsilon = 1.0e-5);

    assert_relative_eq!(
        &(&q * &General::<f32>::from(h)) * &q.transpose(),
        a,
        epsilon = 1.0e-5
    );
}

#[test]
fn dec_f64() {
    let a: General<f64> = matrix![   1.0, 5.0, 3.0;
                                    1.0, 0.0, -7.0;
                                    3.0, 8.0, 9.0];

    let h_ref: UpperHessenberg<f64> = matrix![   1.0, -4.427188724235731, -3.7947331922020537;
                                        -3.162277660168379, 8.4, 5.2;
                                        0.0, -9.8, 0.6]
    .into();

    let q_ref: General<f64> = matrix![   1.0, 0.0, 0.0;
                                        0.0, -0.316227766016838, -0.9486832980505137;
                                        0.0, -0.9486832980505137, 0.3162277660168381];

    let (q, h): (General<f64>, UpperHessenberg<f64>) = a.dec_hessenberg().qh();

    assert_relative_eq!(q, q_ref, epsilon = 1.0e-10);
    assert_relative_eq!(h, h_ref, epsilon = 1.0e-10);

    assert_relative_eq!(
        &(&q * &General::<f64>::from(h)) * &q.transpose(),
        a,
        epsilon = 1.0e-10
    );
}

#[test]
fn dec_f64_2() {
    let a: General<f64> = matrix![   2.0, 0.0, 0.0;
                                    0.0, -5.0, 3.0;
                                    0.0, -6.0, 4.0];

    let (q, h): (General<f64>, UpperHessenberg<f64>) = a.dec_hessenberg().qh();

    let q_ref: General<f64> = matrix![  1.0, 0.0, 0.0;
                                        0.0, 1.0, 0.0;
                                        0.0, 0.0, 1.0];

    let h_ref: UpperHessenberg<f64> = matrix![  2.0, 0.0, 0.0;
                                        0.0, -5.0, 3.0;
                                        0.0, -6.0, 4.0]
    .into();

    assert_relative_eq!(q, q_ref, epsilon = 1.0e-10);
    assert_relative_eq!(h, h_ref, epsilon = 1.0e-10);

    assert_relative_eq!(
        &(&q * &General::<f64>::from(h)) * &q.transpose(),
        a,
        epsilon = 1.0e-10
    );
}

#[test]
fn dec_complex_f32() {
    let a: General<Complex<f32>> = matrix![  Complex::new(1.0, 0.0), Complex::new(5.0, 0.0), Complex::new(3.0, 0.0);
                                            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(-7.0, 0.0);
                                            Complex::new(3.0, 0.0), Complex::new(8.0, 0.0), Complex::new(9.0, 0.0)];

    let q_ref: General<Complex<f32>> = matrix![  Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0);
                                                Complex::new(0.0, 0.0), Complex::new(-0.316227766016838, 0.0), Complex::new(-0.9486832980505137, 0.0);
                                                Complex::new(0.0, 0.0), Complex::new(-0.9486832980505137, 0.0), Complex::new(0.3162277660168381, 0.0)];

    let h_ref: UpperHessenberg<Complex<f32>> = matrix![  Complex::new(1.0, 0.0), Complex::new(-4.427188724235731, 0.0), Complex::new(-3.7947331922020537, 0.0);
                                                Complex::new(-3.162277660168379, 0.0), Complex::new(8.4, 0.0), Complex::new(5.2, 0.0);
                                                Complex::new(0.0, 0.0), Complex::new(-9.8, 0.0), Complex::new(0.6, 0.0)].into();

    let (q, h): (General<Complex<f32>>, UpperHessenberg<Complex<f32>>) = a.dec_hessenberg().qh();

    assert_relative_eq!(q, q_ref, epsilon = Complex::new(1.0e-5, 1.0e-5));
    assert_relative_eq!(h, h_ref, epsilon = Complex::new(1.0e-5, 1.0e-5));

    assert_relative_eq!(
        &(&q * &General::<Complex<f32>>::from(h)) * &q.transpose(),
        a,
        epsilon = Complex::new(1.0e-5, 1.0e-5)
    );
}

#[test]
fn dec_complex_f64() {
    let a: General<Complex<f64>> = matrix![  Complex::new(1.0, 0.0), Complex::new(5.0, 0.0), Complex::new(3.0, 0.0);
                                            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(-7.0, 0.0);
                                            Complex::new(3.0, 0.0), Complex::new(8.0, 0.0), Complex::new(9.0, 0.0)];

    let q_ref: General<Complex<f64>> = matrix![  Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0);
                                                Complex::new(0.0, 0.0), Complex::new(-0.316227766016838, 0.0), Complex::new(-0.9486832980505137, 0.0);
                                                Complex::new(0.0, 0.0), Complex::new(-0.9486832980505137, 0.0), Complex::new(0.3162277660168381, 0.0)];

    let h_ref: UpperHessenberg<Complex<f64>> = matrix![  Complex::new(1.0, 0.0), Complex::new(-4.427188724235731, 0.0), Complex::new(-3.7947331922020537, 0.0);
                                                Complex::new(-3.162277660168379, 0.0), Complex::new(8.4, 0.0), Complex::new(5.2, 0.0);
                                                Complex::new(0.0, 0.0), Complex::new(-9.8, 0.0), Complex::new(0.6, 0.0)].into();

    let (q, h): (General<Complex<f64>>, UpperHessenberg<Complex<f64>>) = a.dec_hessenberg().qh();

    assert_relative_eq!(q, q_ref, epsilon = Complex::new(1.0e-10, 1.0e-10));
    assert_relative_eq!(h, h_ref, epsilon = Complex::new(1.0e-10, 1.0e-10));

    assert_relative_eq!(
        &(&q * &General::<Complex<f64>>::from(h)) * &q.transpose(),
        a,
        epsilon = Complex::new(1.0e-10, 1.0e-10)
    );
}

#[test]
fn dec_f64_6_by_6() {
    let a: General<f64> = matrix![  7.0, 3.0, 4.0, -11.0, -9.0, -2.0;
                                    -6.0, 4.0, -5.0, 7.0, 1.0, 12.0;
                                    -1.0, -9.0, 2.0, 2.0, 9.0, 1.0;
                                    -8.0, 0.0, -1.0, 5.0, 0.0, 8.0;
                                    -4.0, 3.0, -5.0, 7.0, 2.0, 10.0;
                                    6.0, 1.0, 4.0, -11.0, -7.0, -1.0];

    let h_ref: UpperHessenberg<f64> = matrix![7.0000, 7.2761, 5.8120, -0.1397, 9.0152, 7.9363;
                                            12.3693, 4.1307, 18.9685, -1.2071, 10.6833, 2.4160;
                                            0.0, -7.1603, 2.4478, -0.5656, -4.1814, -3.2510;
                                            0.0, 0.0, -8.5988, 2.9151, -3.4169, 5.7230;
                                            0.0, 0.0, 0.0, 1.0464, -2.8351, -10.9792;
                                            0.0, 0.0, 0.0, 0.0, 1.4143, 5.3415]
    .into();

    let (_, h): (General<f64>, UpperHessenberg<f64>) = a.dec_hessenberg().qh();

    assert_relative_eq!(h, h_ref, epsilon = 0.001);
}

#[test]
#[should_panic]
fn dec_non_square() {
    let a: General<f64> = matrix![1.0, 2.0];
    let _ = a.dec_hessenberg();
}

#[test]
#[should_panic]
fn dec_empty() {
    let a: General<f64> = General::new(0, 0, vec![]);
    let _ = a.dec_hessenberg();
}
