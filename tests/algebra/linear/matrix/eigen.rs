#[cfg(test)]
mod eigen
{
    use mathru::algebra::linear::{Matrix, Vector};

    #[test]
    fn eigenvalue_0()
    {
        let a: Matrix<f64> = matrix![   1.0, -3.0, 3.0;
                                        3.0, -5.0,  3.0;
                                        6.0, -6.0,  4.0];

        let eig_ref: Vector<f64> = vector![4.0; -2.0; -2.0];
        let (value, _vector): (Vector<f64>, Matrix<f64>) = a.dec_eigen().pair();

        assert_eq!(true, value.compare_neighbourhood(&eig_ref, 1.0e-10));
    }
}
