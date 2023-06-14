use crate::mathru::algebra::linear::matrix::Determinant;
use mathru::algebra::abstr::Complex;
use mathru::algebra::linear::matrix::General;

#[test]
fn det_one_element() {
    let a: General<f64> = matrix![-2.0];
    let det: f64 = a.det();

    assert_abs_diff_eq!(-2.0, det, epsilon = 1.0e-10);
}

#[test]
fn determinant_quadratic_2() {
    let a: General<f64> = matrix![   1.0, -2.0;
                                    3.0, -7.0];
    let det: f64 = a.det();

    assert_abs_diff_eq!(-1.0, det, epsilon = 1.0e-10);
}

#[test]
fn determinant_quadratic_3() {
    let a: General<f32> = matrix![   1.0, -2.0, 3.0;
                                    2.0, -5.0, 12.0;
                                    1.0, 2.0, -10.0];
    let det: f32 = a.det();

    assert_abs_diff_eq!(-11.0, det, epsilon = 1.0e-4);
}

#[test]
fn determinant_permutation_4x4() {
    let a = matrix![
        0.0, 1.0, 0.0, 0.0;
        1.0, 0.0, 0.0, 0.0;
        0.0, 0.0, 0.0, 1.0;
        0.0, 0.0, 1.0, 0.0];
    let det = a.det();

    assert_abs_diff_eq!(1.0, det, epsilon = 1.0e-4);
}

#[test]
fn determinant_randomized_3x3() {
    use rand::distributions::OpenClosed01;
    use rand::{thread_rng, Rng};
    for _i in 0..100 {
        let r = || -> f32 { thread_rng().sample(OpenClosed01) };
        let a = matrix![r(), r(), r(); r(), r(), r(); r(), r(), r()];
        let det_computed = a.det();
        let det_expected = a[[0, 0]] * (a[[1, 1]] * a[[2, 2]] - a[[1, 2]] * a[[2, 1]])
            + a[[0, 1]] * (a[[1, 2]] * a[[2, 0]] - a[[1, 0]] * a[[2, 2]])
            + a[[0, 2]] * (a[[1, 0]] * a[[2, 1]] - a[[1, 1]] * a[[2, 0]]);
        assert_abs_diff_eq!(det_computed, det_expected, epsilon = 1.0e-4);
    }
}

#[test]
fn determinant_quadratic_4() {
    let a: General<f64> = matrix![   4.0, 1.0, -2.0, 2.0;
                                    1.0, 2.0, 0.0, -2.0;
                                    0.0, 3.0, -2.0, 2.0;
                                    2.0, 1.0, -2.0, -1.0];

    let det: f64 = a.det();

    assert_abs_diff_eq!(76.0, det, epsilon = 1.0e-10);
}

#[test]
fn determinant_quadratic_f32() {
    let a: General<f32> = matrix![   1.0, -2.0;
                                    3.0, -7.0];
    let det: f32 = a.det();

    assert_relative_eq!(-1.0, det, epsilon = 1.0e-5);
}

#[test]
fn determinant_quadratic_f64() {
    let a: General<f64> = matrix![   1.0, -2.0;
                                    3.0, -7.0];
    let det: f64 = a.det();

    assert_relative_eq!(-1.0, det, epsilon = 1.0e-10);
}

#[test]
fn determinant_complex_f32() {
    let a: General<Complex<f32>> = matrix![  Complex::new(1.0, -1.0), Complex::new(-2.0, -1.0);
                                            Complex::new(3.0, 2.0), Complex::new(-7.0, 2.0)];
    let det: Complex<f32> = a.det();

    assert_relative_eq!(
        Complex::new(-1.0, 16.0),
        det,
        epsilon = Complex::new(1.0e-5, 1.0e-5)
    );
}

#[test]
fn determinant_complex_f64() {
    let a: General<Complex<f64>> = matrix![  Complex::new(1.0, -1.0), Complex::new(-2.0, -1.0);
                                            Complex::new(3.0, 2.0), Complex::new(-7.0, 2.0)];
    let det: Complex<f64> = a.det();

    assert_relative_eq!(
        Complex::new(-1.0, 16.0),
        det,
        epsilon = Complex::new(1.0e-5, 1.0e-5)
    );
}

#[test]
fn determinant_is_one_for_identity() {
    for n in 1..20 {
        let mut a = General::zero(n, n);
        for k in 0..n {
            a[[k, k]] = 1.0f32;
        }
        let det_a = a.det();
        assert_abs_diff_eq!(det_a, 1.0f32, epsilon = 1.0e-4f32);
    }
}

#[test]
fn determinant_is_homogeneous_randomized() {
    use rand::distributions::{OpenClosed01, Uniform};
    use rand::{thread_rng, Rng};
    for _i in 0..100 {
        let r = || -> f32 { 2f32 * thread_rng().sample::<f32, OpenClosed01>(OpenClosed01) - 1f32 };
        let n = thread_rng().sample(Uniform::from(1..10));
        let k = thread_rng().sample(Uniform::from(0..n));
        let mut a = General::zero(n, n);
        for row in 0..n {
            for col in 0..n {
                a[[col, row]] = r();
            }
        }
        let mut b = a.clone();
        let c = r();
        for col in 0..n {
            b[[col, k]] = c * a[[col, k]];
        }
        let det_a = a.det();
        let det_b = b.det();
        assert_abs_diff_eq!(det_b, c * det_a, epsilon = 1.0e-4);
    }
}

#[test]
fn determinant_is_additive_randomized() {
    use rand::distributions::{OpenClosed01, Uniform};
    use rand::{thread_rng, Rng};
    for _i in 0..100 {
        let r = || -> f32 { 2f32 * thread_rng().sample::<f32, OpenClosed01>(OpenClosed01) - 1f32 };
        let n = thread_rng().sample(Uniform::from(2..10));
        let k = thread_rng().sample(Uniform::from(0..n));
        let l = thread_rng().sample(Uniform::from(0..n));
        if l == k {
            continue;
        }
        let mut a = General::zero(n, n);
        for row in 0..n {
            for col in 0..n {
                a[[col, row]] = r();
            }
        }
        let mut b = a.clone();
        for col in 0..n {
            b[[col, k]] += a[[col, l]];
        }
        let det_a = a.det();
        let det_b = b.det();
        assert_abs_diff_eq!(det_b, det_a, epsilon = 1.0e-4);
    }
}
