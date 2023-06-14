use mathru::algebra::abstr::Complex;
use mathru::algebra::linear::matrix::{General, Transpose};

#[test]
fn transpose_f32() {
    let uut: General<f32> = matrix![ 1.0, 0.0;
                                    3.0, 0.0;
                                    1.0, -7.0;
                                   0.5, 0.25];

    let res: General<f32> = uut.clone().transpose();

    let trans_ref: General<f32> = matrix![   1.0, 3.0, 1.0, 0.5;
                                            0.0, 0.0, -7.0, 0.25];

    assert_relative_eq!(res.clone().transpose(), uut);
    assert_relative_eq!(trans_ref, res);
}

#[test]
fn transpose_f64() {
    let uut: General<f64> = matrix![ 1.0, 0.0;
                                    3.0, 0.0;
                                    1.0, -7.0;
                                   0.5, 0.25];

    let res: General<f64> = uut.clone().transpose();

    let trans_ref: General<f64> = matrix![   1.0, 3.0, 1.0, 0.5;
                                            0.0, 0.0, -7.0, 0.25];

    assert_relative_eq!(res.clone().transpose(), uut);
    assert_relative_eq!(trans_ref, res);
}

#[test]
fn transpose_square() {
    let uut: General<f32> = matrix![ 1.0, 0.0, 4.0;
                                    3.0, 2.0, 5.0;
                                    7.0, -1.0, 8.0];

    let uut_t_ref: General<f32> =
        General::new(3, 3, vec![1.0, 0.0, 4.0, 3.0, 2.0, 5.0, 7.0, -1.0, 8.0]);

    assert_relative_eq!(uut_t_ref, uut.transpose());
}

#[test]
fn transpose_row() {
    let uut: General<f32> = matrix![ 1.0;
                                    3.0;
                                    1.0;
                                    0.5];

    let uut_t_ref: General<f32> = matrix![1.0, 3.0, 1.0, 0.5];

    assert_relative_eq!(uut_t_ref, uut.transpose());
}

#[test]
fn transpose_column() {
    let uut: General<f32> = matrix![1.0, 3.0, 1.0, 0.5];

    let uut_t_ref: General<f32> = matrix![   1.0;
                                            3.0;
                                            1.0;
                                            0.5];

    assert_relative_eq!(uut_t_ref, uut.transpose());
}

#[test]
fn transpose_complex_f32() {
    let uut: General<Complex<f32>> = matrix![    Complex::new(1.0, 1.0);
                                                Complex::new(13.0, -2.0);
                                                Complex::new(11.0, 6.0);
                                                Complex::new(10.5, -8.0)];

    let uut_t_ref: General<Complex<f32>> = matrix![
        Complex::new(1.0, 1.0),
        Complex::new(13.0, -2.0),
        Complex::new(11.0, 6.0),
        Complex::new(10.5, -8.0)
    ];

    assert_relative_eq!(uut_t_ref, uut.transpose());
}

#[test]
fn transpose_complex_f64() {
    let uut: General<Complex<f64>> = matrix![    Complex::new(1.0, 1.0);
                                                Complex::new(13.0, -2.0);
                                                Complex::new(11.0, 6.0);
                                                Complex::new(10.5, -8.0)];

    let uut_t_ref: General<Complex<f64>> = matrix![
        Complex::new(1.0, 1.0),
        Complex::new(13.0, -2.0),
        Complex::new(11.0, 6.0),
        Complex::new(10.5, -8.0)
    ];

    assert_relative_eq!(uut_t_ref, uut.transpose());
}
