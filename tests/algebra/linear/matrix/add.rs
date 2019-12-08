
#[cfg(test)]
mod add
{
    use mathru::algebra::linear::{Matrix};

    #[test]
    fn add()
    {
        let dim: usize = 5;
        let m_zero : Matrix<f32> = Matrix::zero(dim, dim);
        let m_one : Matrix<f32> = Matrix::one(dim);

        let m_res : Matrix<f32> = &m_zero + &m_one;

        assert!(m_one.compare_neighbourhood(&m_res, 10e-10));

    }

    #[test]
    fn add_1()
    {
        let a : Matrix<f32> = matrix![  1.0, -2.0, -3.0;
                                            -4.0, -1.0, -2.5];

        let b : Matrix<f32> = matrix![ -2.0, -7.0, 3.0;
                                        -5.0, 3.5, 0.0];

        let sum_ref : Matrix<f32> = matrix![    -1.0, -9.0, 0.0;
                                                -9.0, 2.5, -2.5];

        assert!((a + b).compare_neighbourhood(&sum_ref, 10e-10));

    }
}