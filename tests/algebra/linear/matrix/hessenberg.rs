use mathru::algebra::linear::Matrix;

#[cfg(feature = "native")]
#[test]
fn hessenberg_decomposition_0()
{
    let a: Matrix<f64> = matrix![   1.0, 5.0, 3.0;
                                    1.0, 0.0, -7.0;
                                    3.0, 8.0, 9.0];

    let h_ref: Matrix<f64> = matrix![1.0, -4.427188724235731, 3.7947331922020537;
                                    -3.162277660168379, 8.4, -5.2;
                                    0.0, 9.8, 0.6];

    let q_ref: Matrix<f64> = matrix![   1.0, 0.0, 0.0;
                                        0.0, -0.316227766016838, 0.9486832980505137;
                                        0.0, -0.9486832980505137, -0.3162277660168381];

    let (q, h): (Matrix<f64>, Matrix<f64>) = a.dec_hessenberg().qh();

    assert!(q.compare_neighbourhood(&q_ref, 1.0e-10));
    assert!(h.compare_neighbourhood(&h_ref, 1.0e-10));
}

#[cfg(feature = "blaslapack")]
#[test]
fn hessenberg_decomposition_0()
{
    let a: Matrix<f64> = matrix![   1.0, 5.0, 3.0;
                                    1.0, 0.0, -7.0;
                                    3.0, 8.0, 9.0];

    let h_ref: Matrix<f64> = matrix![   1.0, -4.427188724235731, -3.7947331922020537;
                                        -3.162277660168379, 8.4, 5.2;
                                        0.0, -9.8, 0.6];

    let q_ref: Matrix<f64> = matrix![   1.0, 0.0, 0.0;
                                        0.0, -0.316227766016838, -0.9486832980505137;
                                        0.0, -0.9486832980505137, 0.3162277660168381];

    let (q, h): (Matrix<f64>, Matrix<f64>) = a.dec_hessenberg().qh();

    assert!(q.compare_neighbourhood(&q_ref, 1.0e-10));
    assert!(h.compare_neighbourhood(&h_ref, 1.0e-10));
}
