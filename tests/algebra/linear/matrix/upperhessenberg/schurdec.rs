use mathru::algebra::linear::matrix::General;
use mathru::algebra::linear::matrix::UpperHessenberg;

#[test]
fn schurdec() {
    let h: UpperHessenberg<f64> = matrix![7.0000, 7.2761, 5.8120, -0.1397,9.0152, 7.9363;
                                            12.3693, 4.1307, 18.9685, -1.2071, 10.6833, 2.4160;
                                            0.0, -7.1603, 2.4478, -0.5656, -4.1814, -3.2510;
                                            0.0, 0.0, -8.5988, 2.9151, -3.4169, 5.7230;
                                            0.0, 0.0, 0.0, 1.0464, -2.8351, -10.9792;
                                            0.0, 0.0, 0.0, 0.0, 1.4143, 5.3415]
    .into();

    let u_ref = matrix![5.0000, 5.9998, 10.2824, 4.2036, -14.9310, -19.2838;
                                    -6.0000, 5.0, 0.1940, 1.2371, 7.2177, -8.6102;
                                    0.0, 0.0, 2.2893, 5.4630, 10.0210, -4.5043;
                                    0.0, 0.0, -1.0365, -0.2893, -6.0471, -9.6567;
                                    0.0, 0.0, 0.0, 0.0, 3.9995, 4.9227;
                                    0.0, 0.0, 0.0, 0.0, 0.0, 3.0004];

    let (_q, u) = h.dec_schur().unwrap().qu();

    assert_abs_diff_eq!(u, u_ref.into(), epsilon = 1.0e-4);
    //assert_abs_diff_eq!(Into::<General<f64>>::into(h), &(&q * &Into::<General<f64>>::into(u)) * &q.transpose() , epsilon = 1.0e-4);
}
