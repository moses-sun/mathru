

#[cfg(test)]
mod matrix
{
    use mathru::algebra::linear::{Vector, Matrix};
    use mathru::elementary::{Power};
    use mathru::algebra::abstr::Real;

    #[test]
    fn macro_0()
    {
        //Construct a 2x3 matrix of f32
        let mat: Matrix<f32> = matrix![ 1.0, 2.0, 3.0;
                                        4.0, 5.0, 6.0];

        let mat_ref: Matrix<f32> = Matrix::new(2, 3, vec![ 1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);

        assert_eq!(mat, mat_ref);
    }

    #[test]
    fn serde_0()
    {
        let mat:Matrix<f64> = matrix![1.0, 2.0; 3.0, 4.0];
        let serialized = serde_json::to_string(&mat).unwrap();

        let mat_s: Matrix<f64> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(mat_s, mat);
    }

    #[test]
    fn zeros()
    {
        let rows: usize = 5;
        let cols: usize = 7;
        let m_zero : Matrix<f32> = Matrix::zero(rows, cols);
        let dim: (usize, usize) = m_zero.dim();
        assert_eq!(dim, (rows, cols));

        for i in 0..rows
        {
            for k in 0..cols
            {
                assert_eq!(*(m_zero.get(&i, &k)), 0.0);
            }
        }
    }

    #[test]
    fn one()
    {
        let rows: usize = 5;
        let m_ones: Matrix<f32> = Matrix::one(rows);
        let dim: (usize, usize) = m_ones.dim();
        assert_eq!(dim, (rows, rows));

        for i in 0..rows
        {
            for k in 0..rows
            {
                if i == k
                {
                    assert_eq!(*m_ones.get(&i, &k), 1.0);
                }
                else
                {
                    assert_eq!(*m_ones.get(&i, &k), 0.0);
                }
            }
        }
    }

    #[test]
    fn get_column()
    {
        let a: Matrix<f32> = matrix![4.0, 1.0, -3.0, 2.0; 1.0, 2.0, 0.0, 1.0; -2.0, 0.0, 3.0, -2.0; 2.0, 1.0, -2.0,
        -1.0];

        let x: Vector<f32> = a.get_column(&0);

        let x_ref : Vector<f32> = Vector::new_column(4, vec![4.0, 1.0, -2.0, 2.0]);

        for i in 0..4
        {
            assert_eq!(*x.get(&i), *x_ref.get(&i));
        }
    }

    #[test]
    fn get_row()
    {
        let a: Matrix<f64> = matrix![   4.0, 1.0, -2.0, 2.0;
                                        1.0, 2.0, 3.0, 1.0;
                                        -2.0, 0.0, 3.0, -2.0;
                                        2.0, 3.0, -2.0, -1.0];

        let x: Vector<f64> = a.get_row(&1);
        let x_ref : Vector<f64> = Vector::new_row(4, vec![1.0, 2.0, 3.0, 1.0]);

        for i in 0..4
        {
            assert_eq!(*(x.get(&i)), *(x_ref.get(&i)));
        }
    }

    #[test]
    fn add()
    {
        let dim: usize = 5;
        let m_zero : Matrix<f32> = Matrix::zero(dim, dim);
        let m_one : Matrix<f32> = Matrix::one(dim);

        let m_res : Matrix<f32> = &m_zero + &m_one;

        assert!(compare_matrix_epsilon(&m_one, &m_res, 10e-10));

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

        assert!(compare_matrix_epsilon(&sum_ref, &(&a + &b), 10e-10));

    }

    #[test]
    fn mul_1()
    {
        let size: usize = 23;

        let zero: Matrix<f32> = Matrix::zero(size, size);
        let one: Matrix<f32> = Matrix::one(size);

        let res: Matrix<f32> = zero * one;

        assert_eq!(res, Matrix::zero(size, size));
    }

    #[test]
    fn mul_2()
    {
        let size: usize= 23;

        let i1: Matrix<f32> = Matrix::one(size);
        let i2: Matrix<f32> = Matrix::one(size);

        let res: Matrix<f32> = i1 * i2;

        assert_eq!(res, Matrix::one(size));
    }

    #[test]
    fn mul_3()
    {
        let a: Matrix<f64> = matrix![   1.0, 2.0, 5.0;
                                        3.0, 4.0, 6.0];

        let b: Matrix<f64> = matrix![   5.0, 8.0;
                                        6.0, 9.0;
                                        7.0, 10.0];


        let reference: Matrix<f64> = matrix![   52.0, 76.0;
                                                81.0, 120.0];

        let res: Matrix<f64> = &a * &b;

        assert_eq!(reference, res);
    }

    #[test]
    fn mul_4()
    {
        let a: Matrix<f64> = matrix![1.0, 2.0, 5.0];
        let b: Matrix<f64> = matrix![   5.0, 8.0;
                                        6.0, 9.0;
                                        7.0, 10.0];

        let reference: Matrix<f64> = matrix![52.0, 76.0];

        let res: Matrix<f64> = &a * &b;

        assert_eq!(reference, res);
    }

    #[test]
    fn mul_5()
    {
        let a: Matrix<f64> = matrix![1.0, 2.0, 5.0; 3.0, 4.0, 6.0];
        let b: Matrix<f64> = matrix![5.0; 6.0; 7.0];
        let reference: Matrix<f64> = matrix![52.0; 81.0];

        let res: Matrix<f64> = &a * &b;

        assert_eq!(reference, res);
    }

    #[test]
    fn mul_6()
    {
        let a: Matrix<f64> = matrix![1.0, 2.0, 5.0; 3.0, 4.0, 6.0; 7.0, 8.0, 9.0];
        let b: Matrix<f64> = matrix![5.0, 8.0; 6.0, 9.0; 7.0, 10.0];
        let reference: Matrix<f64> = matrix![52.0, 76.0; 81.0, 120.0; 146.0, 218.0];

        let res: Matrix<f64> = &a * &b;

        assert!(compare_matrix_epsilon(&reference, &res, 10e-10));
    }

    #[test]
    fn mul_scalar_0()
    {
        let a: Matrix<f64> = matrix![1.0, -2.0, 5.0; 3.0, 4.0, 6.0; 7.0, -8.0, 9.0];
        let reference = matrix![-2.0, 4.0, -10.0; -6.0, -8.0, -12.0; -14.0, 16.0, -18.0];

        let res: Matrix<f64> = &a * &-2.0;

        assert_eq!(reference, res);
    }

    #[test]
    fn decompose_lu_0()
    {
        let l_ref: Matrix<f64> = matrix![   1.0, 0.0, 0.0;
                                            0.0, 1.0, 0.0;
                                            0.5, 0.25, 1.0];

        let u_ref: Matrix<f64> = matrix![   2.0, -5.0, 12.0;
                                            0.0, 2.0, -10.0;
                                            0.0, 0.0, -0.5];

        let p_ref: Matrix<f64> = matrix![   0.0, 1.0, 0.0;
                                            0.0, 0.0, 1.0;
                                            1.0, 0.0, 0.0];

        let a: Matrix<f64> = matrix![   1.0, -2.0, 3.0;
                                        2.0, -5.0, 12.0;
                                        0.0, 2.0, -10.0];

        let (l, u, p) : (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu();

        assert!(compare_matrix_epsilon(&l_ref, &l, 1.0e-10));
        assert!(compare_matrix_epsilon(&u_ref, &u, 1.0e-10));
        assert!(compare_matrix_epsilon( &p_ref,&p, 1.0e-10));

        assert_eq!(&p * &a, &l * &u);
    }

    #[test]
    fn decompose_lu_1()
    {
        let a: Matrix<f64> = matrix![   4.0, 1.0, -2.0, 2.0;
                                        1.0, 2.0, 0.0, -2.0;
                                        0.0, 3.0, -2.0, 2.0;
                                        2.0, 1.0, -2.0, -1.0];

        let l_ref: Matrix<f64> = matrix![   1.0, 0.0, 0.0, 0.0;
                                            0.0, 1.0, 0.0, 0.0;
                                            0.25, 0.5833333333333334, 1.0, 0.0;
                                            0.5, 0.16666666666666666, -0.4, 1.0 ];

        let u_ref: Matrix<f64> = matrix![   4.0, 1.0, -2.0, 2.0;
                                            0.0, 3.0, -2.0, 2.0;
                                            0.0, 0.0, 1.6666666666666667, -3.666666666666667;
                                            0.0, 0.0, 0.0, -3.8000000000000003 ];

        let p_ref: Matrix<f64> = matrix![   1.0, 0.0, 0.0, 0.0;
                                            0.0, 0.0, 1.0, 0.0;
                                            0.0, 1.0, 0.0, 0.0;
                                            0.0, 0.0, 0.0, 1.0];

        let (l, u, p) : (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu();

        assert!(compare_matrix_epsilon(&l_ref, &l, 1.0e-10));
        assert!(compare_matrix_epsilon(&u_ref, &u, 1.0e-10));
        assert!(compare_matrix_epsilon( &p_ref,&p, 1.0e-10));
    }

    #[test]
    fn decompose_lu2()
    {
        let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 3.0, -2.0, -7.0]);
        let l_ref: Matrix<f64> = matrix![1.0, 0.0; 1.0 / 3.0, 1.0];
        let u_ref: Matrix<f64> = matrix![3.0, -7.0; 0.0, 1.0/3.0];

        let (l, u, p): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu();

        assert!(compare_matrix_epsilon(&l_ref, &l, 1.0e-10));
        assert!(compare_matrix_epsilon(&u_ref, &u, 1.0e-10));
    }
    #[test]
    fn givens()
    {
        let m: usize = 4;
        let i: usize = 1;
        let j: usize = 2;
        let theta : f32 = 1.0;
        let c : f32 = theta.cos();
        let s : f32 = theta.sin();

        let givens : Matrix<f32> = Matrix::givens(&m, &i, &j, &c, &s);

        assert_eq!(*(givens.get(&0, &0)), 1.0);
        assert_eq!(*(givens.get(&i, &i)), theta.cos());
        assert_eq!(*(givens.get(&j, &j)), theta.cos());
        assert_eq!(*(givens.get(&j, &i)), -theta.sin());
        assert_eq!(*(givens.get(&i, &j)), theta.sin());
    }

    #[test]
    fn mul3()
    {
        let m : usize = 4;
        let i : usize = 1;
        let j : usize = 2;
        let theta : f64 = 1.0;
        let c : f64 = theta.cos();
        let s : f64 = theta.sin();
        let givens : Matrix<f64> = Matrix::givens(&m, &i, &j, &c, &s);
        let givens_t : Matrix<f64> = givens.clone().transpose();
        let res_ref : Matrix<f64> = Matrix::one(m);
        let res : Matrix<f64> = givens_t * givens;
        assert_eq!(res_ref, res);
    }

    #[test]
    fn scalar_mul()
    {
        let m = matrix![1.0, 2.0; 3.0, 4.0];
        let prod_ref = matrix![-0.5, -1.0; -1.5, -2.0];

        let res = m * -0.5;

        assert_eq!(prod_ref, res);
    }

    #[test]
    fn vector_mul()
    {
        let m = matrix![1.0, 2.0; 3.0, 4.0];

        println!("{}", m);
        let v = vector![2.0; 4.0];
        println!("{}", v);
        let prod_ref = vector![10.0; 22.0];

        let res = m * v;

        assert_eq!(prod_ref, res);
    }

    #[test]
    fn transpose()
    {
        let uut: Matrix<f32> = matrix![ 1.0, 0.0;
                                        3.0, 0.0;
                                        1.0, -7.0;
                                        0.5, 0.25];

        let res: Matrix<f32> = uut.transpose();

        let trans_ref: Matrix<f32> = matrix![   1.0, 3.0, 1.0, 0.5;
                                                0.0, 0.0, -7.0, 0.25];

        assert_eq!(res, trans_ref);
    }

//    #[test]
//    fn transpose_inplace()
//    {
//        let mut uut: Matrix<f32> = matrix![ 1.0, 0.0;
//                                            3.0, 0.0;
//                                            1.0, -7.0;
//                                            0.5, 0.25];
//
//        uut = uut.transpose_inplace();
//
//        let trans_ref: Matrix<f32> = matrix![   1.0, 3.0, 1.0, 0.5;
//                                                0.0, 0.0, -7.0, 0.25];
//
//        assert_eq!(uut, trans_ref);
//    }

    #[cfg(feature = "native")]
    #[test]
    fn decompose_qr0()
    {
        let a : Matrix<f64> = matrix![  6.0, 5.0, 0.0;
                                        5.0, 1.0, 4.0;
                                        0.0, 4.0, 3.0];

        let (q,r) : (Matrix<f64>, Matrix<f64>) =  a.dec_qr();

        let q_ref: Matrix<f64> = matrix![   7.682212795973757e-01, 3.326541793600714e-01, 5.469709887444194e-01;
                                            6.401843996644797e-01, -3.991850152320858e-01, -6.563651864933034e-01;
                                            0.0, 8.543959975142890e-01, -5.196224393071984e-01];

        let r_ref : Matrix<f64> = matrix![  7.810249675906654, 4.48129079765136, 2.5607375986579197;
                                            0.0, 4.681669871625427, 0.9664479316145234;
                                            0.0, 0.0, -4.184328063894809];

        assert!(compare_matrix_epsilon(&q_ref, &q, 1.0e-10));
        assert!(compare_matrix_epsilon(&r_ref, &r, 1.0e-10));
    }

    #[cfg(feature = "blaslapack")]
    #[test]
    fn decompose_qr0()
    {
        let a : Matrix<f64> = matrix![  6.0, 5.0, 0.0;
                                        5.0, 1.0, 4.0;
                                        0.0, 4.0, 3.0];

        let (q,r) : (Matrix<f64>, Matrix<f64>) =  a.dec_qr();

        let q_ref: Matrix<f64> = matrix![   -7.682212795973757e-01, 3.326541793600714e-01, -5.469709887444194e-01;
                                            -6.401843996644797e-01, -3.991850152320858e-01, 6.563651864933034e-01;
                                            -0.000000000000000e+00, 8.543959975142890e-01, 5.196224393071984e-01];

        let r_ref : Matrix<f64> = matrix![  -7.810249675906654, -4.48129079765136, -2.5607375986579197;
                                            0.0, 4.681669871625427, 0.9664479316145234;
                                            0.0, 0.0, 4.184328063894809];

        assert!(compare_matrix_epsilon(&q_ref, &q, 1.0e-10));
        assert!(compare_matrix_epsilon(&r_ref, &r, 1.0e-10));
    }

    #[cfg(feature = "native")]
    #[test]
    fn decompose_qr1()
    {
        let a : Matrix<f64> = matrix![  3.0, 5.0;
                                        0.0, 2.0;
                                        0.0, 0.0;
                                        4.0, 5.0];


        let (q,r) : (Matrix<f64>, Matrix<f64>) =  a.dec_qr();

        let r_ref : Matrix<f64> = matrix![  5.0, 7.0;
                                            0.0, 5.0.pow(&0.5);
                                            0.0, 0.0;
                                            0.0, 0.0];

        let q_ref: Matrix<f64> =  matrix![  0.6, 0.35777087639996635, 0.0, -0.7155417527999327;
                                            0.0, 0.8944271909999159, 0.0, 0.4472135954999579;
                                            0.0, 0.0, 1.0, 0.0;
                                            0.8, -0.2683281572999747, 0.0, 0.5366563145999494 ];

        assert!(compare_matrix_epsilon(&r_ref, &r, 1.0e-10));
        assert!(compare_matrix_epsilon(&q_ref, &q, 1.0e-10));
        assert!(compare_matrix_epsilon(&a, &(q * r), 1.0e-10));
    }

    #[cfg(feature = "blaslapack")]
    #[test]
    fn decompose_qr1()
    {
        let a : Matrix<f64> = matrix![  3.0, 5.0;
                                        0.0, 2.0;
                                        0.0, 0.0;
                                        4.0, 5.0];

        let (_q,r) : (Matrix<f64>, Matrix<f64>) =  a.dec_qr();

        let r_ref : Matrix<f64> = matrix![  -5.0, -7.0;
                                            0.0, -5.0.pow(&0.5);
                                            0.0, 0.0;
                                            0.0, 0.0];

        let _q_ref: Matrix<f64> =  matrix![  0.6, 0.35777087639996635, 0.0, -0.7155417527999327;
                                            0.0, 0.8944271909999159, 0.0, 0.4472135954999579;
                                            0.0, 0.0, 1.0, 0.0;
                                            0.8, -0.2683281572999747, 0.0, 0.5366563145999494 ];

        assert!(compare_matrix_epsilon(&r_ref, &r, 1.0e-10));
        //assert!(compare_matrix_epsilon(&q_ref, &q, 1.0e-10));
        //assert!(compare_matrix_epsilon(&a, &(q * r), 1.0e-10));
    }

    #[cfg(feature = "native")]
    #[test]
    fn decompose_qr2()
    {
        let a : Matrix<f64> = matrix![  12.0, -51.0, 4.0;
                                        6.0, 167.0, -68.0;
                                        -4.0, 24.0, -41.0];

        let (q, r) : (Matrix<f64>, Matrix<f64>) =  a.dec_qr();

        let r_ref = matrix![    14.0, 21.0, -14.0;
                                0.0, 175.0, -70.0;
                                0.0, 0.0, -35.0];

        let q_ref: Matrix<f64> = matrix![   8.571428571428572e-01, -3.942857142857143e-01, 3.314285714285715e-01;
                                            4.285714285714286e-01, 9.028571428571428e-01, -3.428571428571425e-02;
                                            -2.857142857142858e-01, 1.714285714285714e-01,  9.428571428571428e-01];

        assert!(compare_matrix_epsilon(&q_ref, &q, 1.0e-10));
        assert!(compare_matrix_epsilon(&r_ref, &r, 1.0e-10));
        assert!(compare_matrix_epsilon(&(&q * &r), &a, 1.0e-10));
    }

    #[cfg(feature = "blaslapack")]
    #[test]
    fn decompose_qr2()
    {
        let a : Matrix<f64> = matrix![  12.0, -51.0, 4.0;
                                        6.0, 167.0, -68.0;
                                        -4.0, 24.0, -41.0];

        let (q, r) : (Matrix<f64>, Matrix<f64>) =  a.dec_qr();

        let q_ref: Matrix<f64> = matrix![   -8.571428571428572e-01, 3.942857142857143e-01, 3.314285714285715e-01;
                                            -4.285714285714286e-01, -9.028571428571428e-01, -3.428571428571425e-02;
                                            2.857142857142858e-01, -1.714285714285714e-01,  9.428571428571428e-01];

        let r_ref = matrix![    -14.0, -21.0, 14.0;
                                0.0, -175.0, 70.0;
                                0.0, 0.0, -35.0];

        assert!(compare_matrix_epsilon(&q_ref, &q, 1.0e-10));
        assert!(compare_matrix_epsilon(&r_ref, &r, 1.0e-10));
        assert!(compare_matrix_epsilon(&a, &(&q * &r), 1.0e-10));
    }

    #[test]
    fn determinant_0()
    {
        let a: Matrix<f64> = matrix![-2.0];
        let d: f64 = a.det();

        assert!((d - -2.0).abs() < 1.0e-10);
    }

    #[test]
    fn determinant_1()
    {
        let a: Matrix<f64> = matrix![   1.0, -2.0;
                                        3.0, -7.0];
        let d: f64 = a.det();

        assert!((d - -1.0).abs() < 1.0e-10);
    }

    #[test]
    fn determinant_2()
    {
        let a: Matrix<f32> = matrix![   1.0, -2.0, 3.0;
                                        2.0, -5.0, 12.0;
                                        1.0, 2.0, -10.0];
        let d: f32 = a.det();

        assert!((d - -11.0).abs() < 1.0e-4);
    }

    #[test]
    fn determinant_3()
    {
        let a: Matrix<f64> = matrix![   4.0, 1.0, -2.0, 2.0;
                                        1.0, 2.0, 0.0, -2.0;
                                        0.0, 3.0, -2.0, 2.0;
                                        2.0, 1.0, -2.0, -1.0];

        let d: f64 = a.det();

        assert!((76.0 - d).abs() < 1.0e-10);
    }

    #[test]
    fn determinant_4()
    {
        let a: Matrix<f64> = matrix![   -9.0, -8.0, -7.0;
                                        -6.0, 5.0, -6.0;
                                        -7.0, -8.0, -9.0];
        let d: f64 = a.det();

        assert!((d - 352.0).abs() < 1.0e-10);
    }

    #[test]
    fn trace_0()
    {
        let a: Matrix<f64> = matrix![0.0];
        let tr: f64 = a.trace();

        assert_eq!(0.0, tr);
    }

    #[test]
    fn trace_1()
    {
        let a: Matrix<f64> = matrix![-9.0];
        let tr: f64 = a.trace();

        assert_eq!(-9.0, tr);
    }

    #[test]
    fn trace_2()
    {
        let a: Matrix<f64> = matrix![   1.0, -2.0;
                                        3.0, -7.0];
        let tr: f64 = a.trace();

        assert_eq!(-6.0, tr);
    }

    #[test]
    fn householder_0()
    {
        let v: Vector<f64> = Vector::new_column(3, vec![1.0, 2.0, 3.0]);
        let h: Matrix<f64> = Matrix::householder(&v, 0);

        let h_ref: Matrix<f64> = matrix![   -0.2672612419124243, -0.5345224838248488, -0.8017837257372731;
                                            -0.5345224838248488, 0.7745419205884382, -0.33818711911734267;
                                            -0.8017837257372731, -0.33818711911734267, 0.4927193213239861];

        assert_eq!(h_ref, h);
    }

    #[test]
    fn householder_1()
    {
        let v: Vector<f64> = Vector::new_column(3, vec![1.0, 2.0, 3.0]);
        let h: Matrix<f64> = Matrix::householder(&v, 1);

        let h_ref: Matrix<f64> = matrix![   1.0, 0.0, 0.0;
                                            0.0, -0.5547001962252291, -0.8320502943378437;
                                            0.0, -0.8320502943378437, 0.5547001962252291];

        assert_eq!(h_ref, h);
    }

     #[test]
    fn householder_2()
    {
        let v: Vector<f64> = Vector::new_column(3, vec![1.0, 2.0, 3.0]);
        let h: Matrix<f64> = Matrix::householder(&v, 2);

        let h_ref: Matrix<f64> = matrix![   1.0, 0.0, 0.0;
                                            0.0, 1.0, 0.0;
                                            0.0, 0.0, -1.0];

        assert_eq!(h_ref, h);
    }

    #[test]
    fn slice_get_0()
    {
        let a: Matrix<f32> = matrix![   1.0, 2.0, 3.0;
                                        4.0, 5.0, 6.0;
                                        7.0, 8.0, 9.0];

        let a_ref: Matrix<f32> = matrix![5.0];

        let slice: Matrix<f32> = a.get_slice(1, 1, 1, 1);

        assert_eq!(a_ref, slice);
    }

    #[test]
    fn slice_get_1()
    {
        let a: Matrix<f32> = matrix![   1.0, 2.0, 3.0;
                                        4.0, 5.0, 6.0;
                                        7.0, 8.0, 9.0];

        let a_ref: Matrix<f32> = matrix![   5.0, 6.0;
                                            8.0, 9.0];

        let slice: Matrix<f32> = a.get_slice(1, 2, 1, 2);

        assert_eq!(a_ref, slice);
    }

    #[test]
    fn slice_set_1()
    {
        let a: Matrix<f32> = matrix![   1.0, 2.0, 3.0;
                                        4.0, 5.0, 6.0;
                                        7.0, 8.0, 9.0];

        let a_ref: Matrix<f32> = matrix![   1.0, 2.0, 3.0;
                                            4.0, -5.0, -6.0;
                                            7.0, -8.0, -9.0];

        let b: Matrix<f32> = matrix![   -5.0, -6.0;
                                        -8.0, -9.0];

        let slice: Matrix<f32> = a.set_slice(&b, 1, 1, );

        assert_eq!(a_ref, slice);
    }

    #[test]
    fn householder_bidiagonal_0()
    {
        let a: Matrix<f32> = matrix![   1.0, 2.0, 3.0;
                                        4.0, 5.0, 6.0;
                                        7.0, 8.0, 9.0;
                                        10.0, 11.0, 12.0];

        let b_ref: Matrix<f32> = matrix![   -12.884099, 21.876434, 0.0;
                                            0.0, 2.2462382, -0.61328155;
                                            0.0, 0.0, -0.000000029802322;
                                            0.0, 0.0, 0.0];

        let v_ref: Matrix<f32> = matrix![   1.0, 0.0, 0.0;
                                            0.0, -0.6670023, -0.7450557;
                                            0.0, -0.7450557, 0.6670023];

        let (_u, b, v): (Matrix<f32>, Matrix<f32>, Matrix<f32>) = a.householder_bidiag();

        assert_eq!(b_ref, b);
        assert_eq!(v_ref, v);
    }

    #[test]
    fn householder_bidiagonal_1()
    {
        let a: Matrix<f64> = matrix![   1.0, 5.0, 3.0;
                                        1.0, 0.0, -7.0;
                                        3.0, 8.0, 9.0];

        let b_ref: Matrix<f64> = matrix![   -3.3166247903554, 11.15999348321739, 0.0;
                                            0.0, -8.27496123318713, 5.336122204714563;
                                            0.0, 0.0, 2.5505610873193763];

        let v_ref: Matrix<f64> = matrix![   1.0, 0.0, 0.0;
                                            0.0, -0.783497679089534, -0.6213947110020441;
                                            0.0, -0.6213947110020441, 0.7834976790895338];

        let (_u, b, v): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.householder_bidiag();

        assert_eq!(b_ref, b);
        assert_eq!(v_ref, v);
    }

    #[test]
    fn householder_bidiagonal_2()
    {
        let a: Matrix<f32> = matrix![   4.0, 1.0, -2.0, 2.0;
                                        1.0, 2.0, 0.0, -2.0;
                                        0.0, 3.0, -2.0, 2.0;
                                        2.0, 1.0, -2.0, -1.0];

        let b_ref: Matrix<f32> = matrix![   -4.582576, 3.2659864, 0.0, 0.0;
                                            0.0, -3.7764935, -1.5535977, 0.0;
                                            0.0, 0.0, 1.4568509, -1.203649;
                                            0.0, 0.0, 0.0, -3.014395];

        let (_u, b, _v): (Matrix<f32>, Matrix<f32>, Matrix<f32>) = a.householder_bidiag();

        assert_eq!(b_ref, b);
    }

    #[test]
    fn rot_0()
    {
        let f: f64 = 0.0;
        let g: f64 = -3.0;
        let (c_ref, s_ref, r_ref): (f64, f64, f64) = (0.0, 1.0, -3.0);

        let (c, s, r): (f64, f64, f64) =  Matrix::rot(f, g);

        assert_eq!(c_ref, c);
        assert_eq!(s_ref, s);
        assert_eq!(r_ref, r);
    }

    #[test]
    fn rot_1()
    {
        let f: f64 = 2.0;
        let g: f64 = 3.0;
        let (c_ref, s_ref, r_ref): (f64, f64, f64) = (0.554700196225229, 0.8320502943378437, 3.6055512754639896);

        let (c, s, r): (f64, f64, f64) =  Matrix::rot(f, g);

        assert_eq!(c_ref, c);
        assert_eq!(s_ref, s);
        assert_eq!(r_ref, r);
    }

    #[test]
    fn rot_2()
    {
        let f: f64 = 3.0;
        let g: f64 = -5.0;
        let (c_ref, s_ref, r_ref): (f64, f64, f64) = (-0.5144957554275266, 0.8574929257125443, -5.8309518948453);

        let (c, s, r): (f64, f64, f64) =  Matrix::rot(f, g);

        assert_eq!(c_ref, c);
        assert_eq!(s_ref, s);
        assert_eq!(r_ref, r);
    }

    #[test]
    fn svd_0()
    {
        let a: Matrix<f64> = matrix![   4.0, 1.0, -2.0, 2.0;
                                        1.0, 2.0, 0.0, -2.0;
                                        0.0, 3.0, -2.0, 2.0;
                                        2.0, 1.0, -2.0, -1.0];

        let (u, s, v): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_sv();

        let u_ref: Matrix<f64> = matrix![   0.750618975420566, -0.10396419803551285, -0.5550030946996072, -0.3431241235169467;
                                            0.16660611095451502, 0.7151480228364393, 0.42071909480760994, -0.5327299057590199;
                                            0.49145768126990524, -0.4896500732330627, 0.7174571018374751, 0.06298777918484924;
                                            0.40900232890779536, 0.48784993260512777, -0.014910236788856494, 0.7710364602559809];

        let v_ref: Matrix<f64> = matrix![   0.641413939431974, 0.3770763401833474, -0.5915891468488664, -0.3105219369173845;
                                            0.4773429354587908, 0.10210161763764197, 0.7839592576263352, -0.3835711981175503;
                                            -0.5312275172316614, 0.06255922864595094, -0.09543998328421795, -0.8395087119487312;
                                            0.2802306286004916, -0.9184089700459931, -0.1622386307366823, -0.2273200062245417];

        let s_ref: Matrix<f64> = matrix![   6.216089837372844, 0.0, 0.0, 0.0;
                                            0.0, 3.3812545631600996, 0.0, 0.0;
                                            0.0, 0.0, 3.0918649662716553, 0.0;
                                            0.0, 0.0, 0.0, 1.1694937978293738];

        assert_eq!(u_ref, u);
        assert_eq!(s_ref, s);
        assert_eq!(v_ref, v);
    }

    #[test]
    fn svd_1()
    {
        let a: Matrix<f64> = matrix![   4.0, 1.0, -2.0, 2.0;
                                        1.0, 2.0, 0.0, -2.0;
                                        0.0, 3.0, -2.0, 2.0;
                                        2.0, 1.0, -2.0, -1.0];

        let (u, _s, v): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_sv();

        let u_ref: Matrix<f64> = matrix![   0.750618975420566, -0.10396419803551285, -0.5550030946996072, -0.3431241235169467;
                                            0.16660611095451502, 0.7151480228364393, 0.42071909480760994, -0.5327299057590199;
                                            0.49145768126990524, -0.4896500732330627, 0.7174571018374751, 0.06298777918484924;
                                            0.40900232890779536, 0.48784993260512777, -0.014910236788856494, 0.7710364602559809];

        let v_ref: Matrix<f64> = matrix![   0.641413939431974, 0.3770763401833474, -0.5915891468488664, -0.3105219369173845;
                                            0.4773429354587908, 0.10210161763764197, 0.7839592576263352, -0.3835711981175503;
                                            -0.5312275172316614, 0.06255922864595094, -0.09543998328421795, -0.8395087119487312;
                                             0.2802306286004916, -0.9184089700459931, -0.1622386307366823, -0.2273200062245417];

        assert_eq!(u_ref, u);
        assert_eq!(v_ref, v);
    }

    #[test]
    fn inv_0()
    {
        let a: Matrix<f64> = matrix![   1.0, -2.0, 3.0;
                                        2.0, -5.0, 12.0;
                                        0.0, 2.0, -10.0];

        //let (l, u, p) : (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu();
        let a_inv_ref : Matrix<f64> = matrix![  -13.0, 7.0, 4.5;
                                                -10.0, 5.0, 3.0;
                                                -2.0, 1.0, 0.5];

        let a_inv: Matrix<f64> = a.inv().unwrap();

        assert!(compare_matrix_epsilon(&a_inv_ref, &a_inv, 1.0e-10));
    }

    #[test]
    fn inv_1()
    {
        let a: Matrix<f64> = matrix![   1.0, 0.0, 2.0;
                                        -1.0, 5.0, 0.0;
                                        0.0, 3.0, -9.0];

        let a_inv_ref : Matrix<f64> = matrix![  0.8823529411764706, -0.11764705882352942, 0.19607843137254904;
                                                0.17647058823529413, 0.17647058823529413, 0.03921568627450981;
                                                0.05882352941176471, 0.05882352941176471, -0.09803921568627452];

        let a_inv: Matrix<f64> = a.inv().unwrap();

        assert!(compare_matrix_epsilon(&a_inv_ref, &a_inv, 1.0e-10));
    }

    #[test]
    fn inv_2()
    {
        let a: Matrix<f64> = matrix![   -1.0, 2.0, 3.0, 4.0, 5.0;
                                        -6.0, -7.0, 8.0, 9.0, 10.0;
                                        -11.0, 12.0, 13.0, 14.0, 15.0;
                                        -16.0, -17.0, -18.0, -19.0, 20.0;
                                        -21.0, 22.0, -23.0, 24.0, 25.0];

        let a_inv_ref : Matrix<f64> = matrix![  0.38478669499836576, -0.03759398496240601, -0.08489293886891143, -0.006578947368421052,
        -0.005720823798627002;
        0.03571428571428603, -0.07142857142857142, 0.03571428571428571, 0.0, -0.000000000000000001734723475976807;
        -0.021739130434782705, 0.0, 0.04347826086956519, 0.0, -0.021739130434782608;
        -0.024517816279830296, 0.06390977443609022, -0.033671134357633165, -0.02631578947368421, 0.020594965675057208;
        0.2953293559986926, -0.03007518796992481, -0.030414351095129147, 0.019736842105263157, -0.004576659038901602];

        let a_inv: Matrix<f64> = a.inv().unwrap();

        assert!(compare_matrix_epsilon(&a_inv_ref, &a_inv, 1.0e-10));
    }

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

        let (q, h): (Matrix<f64>, Matrix<f64>) = a.dec_hessenberg();

        assert!(compare_matrix_epsilon(&q_ref, &q, 1.0e-10));
        assert!(compare_matrix_epsilon(&h_ref, &h, 1.0e-10));
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

        let (q, h): (Matrix<f64>, Matrix<f64>) = a.dec_hessenberg();

        assert!(compare_matrix_epsilon(&q_ref, &q, 1.0e-10));
        assert!(compare_matrix_epsilon(&h_ref, &h, 1.0e-10));
    }


    #[test]
    fn eigenvalue_0()
    {
        let a: Matrix<f64> = matrix![   1.0, -3.0, 3.0;
                                        3.0, -5.0,  3.0;
                                        6.0, -6.0,  4.0];

        let eig_ref: Vector<f64> = Vector::new_column(3, vec![3.9999999999999996, -2.0, -1.9999999999999982]);
        let eig: Vector<f64> = a.eigenvalue();

        assert_eq!(true, compare_vector_epsilon(&eig_ref, &eig, 1.0e-10));
    }

    #[test]
    fn cholesky_decomposition()
    {
        let a: Matrix<f64> = matrix![   2.0, -1.0, 0.0;
                                        -1.0, 2.0, -1.0;
                                        0.0, -1.0,  2.0];

        let g_ref: Matrix<f64> = matrix![   1.414213562373095,   0.000000000000000,   0.000000000000000;
                                            -7.071067811865475e-1,   1.224744871391589,   0.000000000000000;
                                            0.000000000000000,  -8.164965809277261e-1,   1.154700538379251];

        let g = a.dec_cholesky();

        assert_eq!(true, compare_matrix_epsilon(&g_ref, &g, 1.0e-10));
    }

    #[test]
    fn apply_0()
    {
        let a: Matrix<f64> = matrix![   1.0, -3.0, 3.0;
                                        3.0, -5.0, 3.0;
                                        6.0, -6.0, 4.0];

        let a_ref: Matrix<f64> = matrix![   -1.0, 3.0, -3.0;
                                            -3.0, 5.0, -3.0;
                                            -6.0, 6.0, -4.0];

        let b: Matrix<f64> = a.apply(&|x| -x);

        assert_eq!(a_ref, b);
    }



    fn compare_matrix_epsilon<T: Real>(exp: &Matrix<T>, act: &Matrix<T>, epsilon: T) -> bool
    {
        let (exp_m, exp_n): (usize, usize) = exp.dim();
        let (act_m, act_n): (usize, usize) = act.dim();

        assert!(exp_m == act_m);
        assert!(exp_n == act_n);

        for i in 0..exp_m
        {
            for k in 0..exp_n
            {
                if (*exp.get(&i, &k) - *act.get(&i, &k)).abs() > epsilon
                {
                    println!("exp: {}, act: {} exp - act: {}", exp, act, (exp - act));
                    return false;
                }
            }
        }

        return true;
    }

    fn compare_vector_epsilon<T: Real>(a: &Vector<T>, b: &Vector<T>, epsilon: T) -> bool
    {
        let (a_m, a_n): (usize, usize) = a.dim();
        let (b_m, b_n): (usize, usize) = b.dim();

        if a_m != b_m || a_n != b_n
        {
            println!("dimension mismatch");
            return false;
        }

        for i in 0..a_m
        {
            if (*a.get(&i) - *b.get(&i)).abs() > epsilon
            {
                println!("a: {}, b: {} a-b: {}", a, b, a-b);
                return false;
            }
        }

        return true;
    }

}