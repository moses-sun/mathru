use mathru::algebra::linear::Matrix;

#[test]
fn add()
{
    let dim: usize = 5;
    let m_zero: Matrix<f32> = Matrix::zero(dim, dim);
    let m_one: Matrix<f32> = Matrix::one(dim);

    let m_res: Matrix<f32> = &m_zero + &m_one;

    assert_relative_eq!(m_one, m_res);
}

#[test]
fn add_1()
{
    let a: Matrix<f32> = matrix![   1.0, -2.0, -3.0;
                                    -4.0, -1.0, -2.5];

    let b: Matrix<f32> = matrix![   -2.0, -7.0, 3.0;
                                    -5.0, 3.5, 0.0];

    let sum_ref: Matrix<f32> = matrix![ -1.0, -9.0, 0.0;
                                        -9.0, 2.5, -2.5];

    assert_relative_eq!(sum_ref, a + b);
}
