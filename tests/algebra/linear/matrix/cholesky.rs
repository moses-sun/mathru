#[cfg(test)]
mod cholesky
{
    use mathru::algebra::linear::Matrix;

    #[test]
    fn cholesky_decomposition()
    {
        let a: Matrix<f64> = matrix![   2.0, -1.0, 0.0;
                                        -1.0, 2.0, -1.0;
                                        0.0, -1.0,  2.0];

        let g_ref: Matrix<f64> = matrix![   1.414213562373095,   0.000000000000000,   0.000000000000000;
                                            -7.071067811865475e-1,   1.224744871391589,   0.000000000000000;
                                            0.000000000000000,  -8.164965809277261e-1,   1.154700538379251];

        let g = a.dec_cholesky().unwrap().l();

        assert_eq!(true, g.compare_neighbourhood(&g_ref, 1.0e-10));
    }
}
