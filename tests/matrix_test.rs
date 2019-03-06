extern crate mathru;

#[cfg(test)]
mod matrix_test
{
    use mathru::algebra::linear::{Vector, Matrix};
    use mathru::algebra::abstr::{Zero, One};
    use mathru::num::{Real};
    use mathru::elementary::{Trigonometry, Power};

    #[test]
    fn zeros()
    {
        let rows: usize = 5;
        let cols: usize = 7;
        let m_zero : Matrix<Real<f32>> = Matrix::zero(&rows, &cols);
        let dim: (usize, usize) = m_zero.dim();
        assert_eq!(dim, (rows, cols));

        for i in 0..rows
        {
            for k in 0..cols
            {
                assert_eq!(*(m_zero.get(&i, &k)), Real::zero());
            }
        }
    }

    #[test]
    fn one()
    {
        let rows: usize = 5;
        let m_ones: Matrix<Real<f32>> = Matrix::one(&rows);
        let dim: (usize, usize) = m_ones.dim();
        assert_eq!(dim, (rows, rows));

        for i in 0..rows
        {
            for k in 0..rows
            {
                if i == k
                {
                    assert_eq!(*m_ones.get(&i, &k), Real::one());
                }
                else
                {
                    assert_eq!(*m_ones.get(&i, &k), Real::zero());
                }
            }
        }
    }

    #[test]
    fn get_column()
    {
        let a: Matrix<Real<f32>> = Matrix::new(&4, &4, &vec![Real::new(4.0), Real::one(),
                                                                                         Real::new(-3.0), Real::new(2.0),
                                                                                         Real::one(), Real::new(2.0), Real::zero(), Real::one(), Real::new(-2.0), Real::zero(), Real::new(3.0), Real::new(-2.0), Real::new(2.0), Real::one(), Real::new(-2.0), Real::new(-1.0)]);

        let x: Vector<Real<f32>> = a.get_column(&0);

        let x_ref : Vector<Real<f32>> = Vector::new_column(&4, &vec![Real::new(4.0), Real::one(),
        Real::new(-2.0), Real::new(2.0)]);

        for i in 0..4
        {
            assert_eq!(*x.get(&i), *x_ref.get(&i));
        }
    }

    #[test]
    fn get_row()
    {
        let a: Matrix<Real<f64>> = Matrix::new(&4, &4, &vec![Real::new(4.0), Real::one(),
                                                                                         Real::new(-2.0), Real::new(2.0),
                                                                                         Real::one(), Real::new(2.0),
                                                                                         Real::new(3.0), Real::one(),
                                                                                         Real::new(-2.0), Real::zero
                                                                                             (), Real::new(3.0),
                                                                                         Real::new(-2.0), Real::new(2.0), Real::new(3.0), Real::new(-2.0), Real::new(-1.0)]);

        let x: Vector<Real<f64>> = a.get_row(&1);
        let x_ref : Vector<Real<f64>> = Vector::new_row(&4, &vec![Real::one(), Real::new(2.0),
                                                                                  Real::new(3.0), Real::one()]);

        for i in 0..4
        {
            assert_eq!(*(x.get(&i)), *(x_ref.get(&i)));
        }
    }

    #[test]
    fn add()
    {
        let dim: usize = 5;
        let m_zero : Matrix<Real<f32>> = Matrix::zero(&dim, &dim);
        let m_one : Matrix<Real<f32>> = Matrix::one(&dim);

        let m_res : Matrix<Real<f32>> = &m_zero + &m_one;

        for i in 0..dim
        {
            for k in 0..dim
            {
                if i == k
                {
                    assert_eq!(Real::one(), *(m_res.get(&i, &k)));
                }
                else
                {
                    assert_eq!(Real::zero(), *(m_res.get(&i, &k)));
                }
            }
        }
    }

    #[test]
    fn mul_1()
    {
        let size: usize = 23;

        let zero: Matrix<Real<f32>> = Matrix::zero(&size, &size);
        let one: Matrix<Real<f32>> = Matrix::one(&size);

        let res: Matrix<Real<f32>> = zero * one;

        assert_eq!(res, Matrix::zero(&size, &size));
    }

    #[test]
    fn mul_2()
    {
        let size: usize= 23;

        let i1: Matrix<Real<f32>> = Matrix::one(&size);
        let i2: Matrix<Real<f32>> = Matrix::one(&size);

        let res: Matrix<Real<f32>> = i1 * i2;

        assert_eq!(res, Matrix::one(&size));
    }

    #[test]
    fn decompose_lu_0()
    {
        let dim: usize = 3;
        let l_ref: Matrix<f64> = Matrix::new(&dim, &dim, &vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.5, 0.25, 1.0]);
        let u_ref: Matrix<f64> = Matrix::new(&dim, &dim, &vec![2.0, -5.0, 12.0, 0.0, 2.0, -10.0, 0.0, 0.0, -0.5]);
        let p_ref: Matrix<f64> = Matrix::new(&dim, &dim, &vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0]);
        let a: Matrix<f64> = Matrix::new(&dim, &dim, &vec![1.0, -2.0, 3.0, 2.0, -5.0, 12.0, 0.0, 2.0, -10.0]);

        let (l, u, p) : (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu();

        assert_eq!(l_ref, l);
        assert_eq!(u_ref, u);
        assert_eq!(p_ref, p);
        assert_eq!(p*a, l*u);
    }

    #[test]
    fn decompose_lu_1()
    {
        let a: Matrix<f64> = Matrix::new(&4, &4, &vec![4.0, 1.0, -2.0, 2.0, 1.0, 2.0, 0.0, -2.0, 0.0, 3.0, -2.0, 2.0,
         2.0, 1.0, -2.0, -1.0]);

        let l_ref: Matrix<f64> = Matrix::new(&4, &4, &vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.25, 0.5833333333333334, 1.0, 0.0, 0.5, 0.16666666666666666, -0.4, 1.0 ]);
        let u_ref: Matrix<f64> = Matrix::new(&4, &4, &vec![4.0, 1.0, -2.0, 2.0, 0.0, 3.0, -2.0, 2.0, 0.0, 0.0, 1.6666666666666667, -3.666666666666667, 0.0, 0.0, 0.0, -3.8000000000000003 ]);

        let (l, u, _p) : (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu();

        assert_eq!(l_ref, l);
        assert_eq!(u_ref, u);
    }

    #[test]
    fn givens()
    {
        let m: usize = 4;
        let i: usize = 1;
        let j: usize = 2;
        let theta : Real<f32> = Real::new(1.0);
        let c : Real<f32> = theta.cos();
        let s : Real<f32> = theta.sin();

        let givens : Matrix<Real<f32>> = Matrix::givens(&m, &i, &j, &c, &s);

        assert_eq!(*(givens.get(&0, &0)), Real::one());
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
        let theta : Real<f64> = Real::new(1.0);
        let c : Real<f64> = theta.cos();
        let s : Real<f64> = theta.sin();
        let givens : Matrix<Real<f64>> = Matrix::givens(&m, &i, &j, &c, &s);
        let givens_t : Matrix<Real<f64>> = givens.clone().transpose();
        let res_ref : Matrix<Real<f64>> = Matrix::one(&m);
        let res : Matrix<Real<f64>> = givens_t * givens;
        assert_eq!(res_ref, res);
    }

    #[test]
    fn transpose()
    {
        let uut: Matrix<Real<f32>> = Matrix::new(&4, &2, &vec![Real::one(), Real::zero(), Real::new(3.0), Real::zero(), Real::one(), Real::new(-7.0), Real::new(0.5), Real::new(0.25)]);
        let res: Matrix<Real<f32>> = uut.transpose();

        let trans_ref: Matrix<Real<f32>> = Matrix::new(&2, &4, &vec![Real::one(), Real::new(3.0), Real::one(), Real::new(0.5), Real::zero(), Real::zero(), Real::new(-7.0), Real::new(0.25)]);

        assert_eq!(res, trans_ref);
    }

    #[test]
    fn transpose_inplace()
    {
        let mut uut: Matrix<Real<f32>> = Matrix::new(&4, &2, &vec![Real::one(), Real::zero(), Real::new(3.0), Real::zero
        (), Real::one(), Real::new(-7.0), Real::new(0.5), Real::new(0.25)]);
        uut = uut.transpose_inplace();

        let trans_ref: Matrix<Real<f32>> = Matrix::new(&2, &4, &vec![Real::one(), Real::new(3.0), Real::one(), Real::new(0.5), Real::zero(), Real::zero(), Real::new(-7.0), Real::new(0.25)]);

        assert_eq!(uut, trans_ref);
    }

    #[test]
    fn decompose_qr0()
    {
        let a : Matrix<Real<f64>> = Matrix::new(&3, &3, &vec![Real::new(6.0), Real::new(5.0),
        Real::zero(), Real::new(5.0), Real::one(), Real::new(4.0), Real::zero(), Real::new(4.0), Real::new(3.0)]);

        let (_q,r) : (Matrix<Real<f64>>, Matrix<Real<f64>>) =  a.dec_qr();

        let r_ref : Matrix<Real<f64>> = Matrix::new(&3, &3, &vec![Real::new(7.810249675906654), Real::new(4.48129079765136), Real::new(2.5607375986579197),
        Real::new(0.0000000000000002307587184922959), Real::new(4.681669871625427), Real::new(0.9664479316145234),
        Real::new(0.00000000000000037942804343517665), Real::zero(), Real::new(-4.184328063894809)]);

        assert_eq!(r_ref, r);
    }

    #[test]
    fn decompose_qr1()
    {
        let a : Matrix<Real<f32>> = Matrix::new(&4, &2, &vec![Real::new(3.0), Real::new(5.0),
        Real::zero(), Real::new(2.0), Real::zero(), Real::zero(), Real::new(4.0), Real::new(5.0)]);
        let (_q,r) : (Matrix<Real<f32>>, Matrix<Real<f32>>) =  a.dec_qr();

        let r_ref : Matrix<Real<f32>> = Matrix::new(&4, &2, &vec![Real::new(5.0), Real::new(7.0), Real::zero(),
        Real::new(5.0).pow(&Real::new(0.5)), Real::zero(), Real::zero(), Real::zero(), Real::zero()]);

        assert_eq!(r_ref, r);
    }

    #[test]
    fn decompose_qr2()
    {
        let a : Matrix<Real<f32>> = Matrix::new(&3, &3, &vec![Real::new(12.0), Real::new
            (-51.0), Real::new(4.0), Real::new(6.0), Real::new(167.0), Real::new(-68.0), Real::new(-4.0), Real::new(24.0), Real::new(-41.0)]);

        let (q,r) : (Matrix<Real<f32>>, Matrix<Real<f32>>) =  a.dec_qr();

        let qr_ref : Matrix<Real<f32>> = Matrix::new(&3, &3, &vec![Real::new(11.999999), Real::new(-51.000008),
                                                                                              Real::new(4.000003),
                                                                                              Real::new(6.0),
                                                                                              Real::new(167.00002),
                                                                                                        Real::new(-68.000015)
                                                                                                  , Real::new(-3.9999998),
                                                                                              Real::new(24.000004),
                                                                                              Real::new(-41.0)]);

        assert_eq!(q*r, qr_ref);
    }

    #[test]
    fn determinant_0()
    {
        let a: Matrix<Real<f32>> = Matrix::new(&1, &1, &vec![Real::new(-2.0)]);
        let d: Real<f32> = a.det();
        assert_eq!(Real::new(-2.0), d);
    }

    #[test]
    fn determinant_1()
    {
        let a: Matrix<Real<f32>> = Matrix::new(&2, &2, &vec![Real::one(), Real::new(-2.0), Real::new(3.0), Real::new
        (-7.0)]);
        let d: Real<f32> = a.det();
        assert_eq!(Real::new(-1.0), d);
    }

    #[test]
    fn determinant_2()
    {
        let a: Matrix<f32> = Matrix::new(&3, &3, &vec![1.0, -2.0, 3.0, 2.0,
    -5.0, 12.0, 1.0, 2.0, -10.0]);
        let d: f32 = a.det();
        assert_eq!(-11.0, d);
    }

    #[test]
    fn determinant_3()
    {
        let a: Matrix<Real<f64>> = Matrix::new(&4, &4, &vec![Real::new(4.0), Real::new(1.0), Real::new(-2.0),
        Real::new(2.0), Real::new(1.0), Real::new(2.0), Real::new(0.0), Real::new(-2.0), Real::new(0.0), Real::new(3.0), Real::new(-2.0), Real::new(2.0), Real::new(2.0), Real::new(1.0), Real::new(-2.0), Real::new(-1.0)]);

        let d: Real<f64> = a.det();
        assert_eq!(Real::new(76.0), d);
    }

    #[test]
    fn determinant_4()
    {
        let a: Matrix<f64> = Matrix::new(&3, &3, &vec![-9.0, -8.0, -7.0, -6.0, 5.0, -6.0, -7.0, -8.0, -9.0]);
        let d: f64 = a.det();
        assert_eq!(351.99999999999994, d);
    }

//    #[test]
//    fn householder()
//    {
//        let a: Matrix<Real<f32>> = Matrix::new(&4, &4, &vec![Real::new(4.0), Real::one(),
//																						Real::new(-2.0), Real::new(2.0),
//        Real::one(), Real::new(2.0), Real::zero(), Real::one(), Real::new(-2.0), Real::zero(), Real::new(3.0), Real::new(-2.0), Real::new(2.0), Real::one(), Real::new(-2.0), Real::new(-1.0)]);
//
//        let h_ref: Matrix<Real<f32>> = Matrix::new(&4, &4, &vec![Real::new(0.8), Real::new(0.19999999), Real::new(-0.39999998), Real::new(0.39999998), Real::new(0.19999999), Real::new(0.8), Real::new(0.39999998), Real::new(-0.39999998),
//Real::new(-0.39999998), Real::new(0.39999998), Real::new(0.20000005), Real::new(0.79999995),
//Real::new(0.39999998), -Real::new(0.39999998), Real::new(0.79999995), Real::new(0.20000005)]);
//
//        let x: Vector<Real<f32>> = a.get_column(&0);
//        let (m, _n): (usize, usize) = x.dim();
//        let p: Real<f32> = Real::new(2.0);
//
//        let abs: Real<f32> =  x.p_norm(&p);
//        let mut y: Vector<Real<f32>> = Vector::zero(&m);
//        *y.get_mut(&0) = abs;
//
//        let v : Vector<Real<f32>> = &x - &y;
//
//        let u_1: Matrix<Real<f32>> = Matrix::householder(&v, 0);
//
//        assert_eq!(h_ref, u_1);
//    }

    #[test]
    fn slice_get_0()
    {
        let a: Matrix<Real<f32>> = Matrix::new(&3, &3, &vec![Real::new(1.0), Real::new(2.0), Real::new(3.0),
               Real::new(4.0), Real::new(5.0), Real::new(6.0), Real::new(7.0), Real::new(8.0), Real::new(9.0)]);

        let a_ref: Matrix<Real<f32>> = Matrix::new(&1, &1, &vec![Real::new(5.0)]);

        let slice: Matrix<Real<f32>> = a.get_slice(1, 1, 1, 1);

        assert_eq!(a_ref, slice);
    }

    #[test]
    fn slice_get_1()
    {
        let a: Matrix<Real<f32>> = Matrix::new(&3, &3, &vec![Real::new(1.0), Real::new(2.0), Real::new(3.0),
               Real::new(4.0), Real::new(5.0), Real::new(6.0), Real::new(7.0), Real::new(8.0), Real::new(9.0)]);

        let a_ref: Matrix<Real<f32>> = Matrix::new(&2, &2, &vec![Real::new(5.0), Real::new(6.0), Real::new(8.0),
        Real::new(9.0)]);

        let slice: Matrix<Real<f32>> = a.get_slice(1, 2, 1, 2);

        assert_eq!(a_ref, slice);
    }

    #[test]
    fn slice_set_1()
    {
        let a: Matrix<Real<f32>> = Matrix::new(&3, &3, &vec![Real::new(1.0), Real::new(2.0), Real::new(3.0),
               Real::new(4.0), Real::new(5.0), Real::new(6.0), Real::new(7.0), Real::new(8.0), Real::new(9.0)]);

        let a_ref: Matrix<Real<f32>> = Matrix::new(&3, &3, &vec![Real::new(1.0), Real::new(2.0), Real::new(3.0),
               Real::new(4.0), Real::new(-5.0), Real::new(-6.0), Real::new(7.0), Real::new(-8.0), Real::new(-9.0)]);

        let b: Matrix<Real<f32>> = Matrix::new(&2, &2, &vec![Real::new(-5.0), Real::new(-6.0), Real::new(-8.0),
        Real::new(-9.0)]);

        let slice: Matrix<Real<f32>> = a.set_slice(&b, 1, 1, );

        assert_eq!(a_ref, slice);
    }

    #[test]
    fn householder_bidiagonal_0()
    {
        let a: Matrix<Real<f32>> = Matrix::new(&4, &3, &vec![Real::new(1.0), Real::new(2.0), Real::new(3.0), Real::new
        (4.0),
        Real::new(5.0), Real::new(6.0), Real::new(7.0), Real::new(8.0), Real::new(9.0), Real::new(10.0), Real::new(11.0),
        Real::new(12.0)]);

        let b_ref: Matrix<Real<f32>> = Matrix::new(&4, &3, &vec![Real::new(-12.884102), Real::new(21.876442
        ), Real::zero(), Real::zero(), Real::new(2.2462401), Real::new(0.6132816),
        Real::zero(), Real::zero(), Real::new(-0.0000004172325), Real::zero(),
        Real::zero(), Real::zero()]);

        let v_ref: Matrix<Real<f32>> = Matrix::new(&3, &3, &vec![Real::one(), Real::zero(), Real::zero(), Real::zero
        (), Real::new(-0.6670023), Real::new(0.7450558), Real::zero(), Real::new(-0.7450558), Real::new(-0.6670022)]);
        let (_u, b, v): (Matrix<Real<f32>>, Matrix<Real<f32>>, Matrix<Real<f32>>) = a.householder_bidiag();

        assert_eq!(b_ref, b);
        assert_eq!(v_ref, v);
    }

    #[test]
    fn householder_bidiagonal_1()
    {
        let a: Matrix<Real<f64>> = Matrix::new(&3, &3, &vec![Real::new(1.0), Real::new(5.0), Real::new(3.0),
        Real::new(1.0), Real::new(0.0), Real::new(-7.0), Real::new(3.0), Real::new(8.0), Real::new(9.0)]);

        let b_ref: Matrix<Real<f64>> = Matrix::new(&3, &3, &vec![Real::new(-3.3166247903554), Real::new(11.159993483217384), Real::zero(), Real::zero(), Real::new(-8.274961233187126), Real::new(-5.336122204714561),
        Real::zero(), Real::zero(), Real::new(-2.550561087319375)]);

        let v_ref: Matrix<Real<f64>> = Matrix::new(&3, &3, &vec![Real::one(), Real::zero(), Real::zero(), Real::zero
       (), Real::new(-0.7834976790895336), Real::new(0.6213947110020438), Real::zero(), Real::new(-0.621394711002044), Real::new(-0.7834976790895335)]);
        let (_u, b, v): (Matrix<Real<f64>>, Matrix<Real<f64>>, Matrix<Real<f64>>) = a.householder_bidiag();

        assert_eq!(b_ref, b);
        assert_eq!(v_ref, v);
    }

    #[test]
    fn householder_bidiagonal_2()
    {
        let a: Matrix<Real<f32>> = Matrix::new(&4, &4, &vec![Real::new(4.0), Real::new(1.0), Real::new(-2.0),
        Real::new(2.0), Real::new(1.0), Real::new(2.0), Real::new(0.0), Real::new(-2.0), Real::new(0.0), Real::new(3.0), Real::new(-2.0), Real::new(2.0), Real::new(2.0), Real::new(1.0), Real::new(-2.0), Real::new(-1.0)]);

        let b_ref: Matrix<Real<f32>> = Matrix::new(&4, &4, &vec![Real::new(-4.582576), Real::new(3.2659864), Real::new(
        0.0), Real::new(0.0), Real::new(0.0), Real::new(-3.7764935), Real::new(-1.5535973), Real::new(0.0), Real::new
        (0.0), Real::new(0.0), Real::new(1.4568503), Real::new(1.2036494), Real::new(0.0), Real::new(0.0), Real::new
        (0.0), Real::new(3.014395)]);

        let (_u, b, _v): (Matrix<Real<f32>>, Matrix<Real<f32>>, Matrix<Real<f32>>) = a.householder_bidiag();

        assert_eq!(b_ref, b);
    }

//    #[test]
//    fn zero_diagonal_0()
//    {
//        let mut a: Matrix<Real<f32>> = Matrix::new(&4, &4, &vec![Real::new(1.0), Real::new(5.0), Real::new(0.0),
//        Real::new(0.0), Real::new(0.0), Real::new(-7.0), Real::new(3.0), Real::new(0.0), Real::new(0.0), Real::new(0.0), Real::new(0.0), Real::new(4.0), Real::new(0.0), Real::new(0.0), Real::zero(), Real::new(4.0)]);
//
//        let a_ref: Matrix<Real<f32>> = Matrix::new(&4, &4, &vec![Real::new(1.0), Real::new(5.0), Real::new(0.0),
//        Real::new(0.0), Real::new(0.0), Real::new(-7.0), Real::new(3.0), Real::new(0.0), Real::new(0.0), Real::new(0.0), Real::new(0.0), Real::new(0.0), Real::new(0.0), Real::new(0.0), Real::zero(), Real::new(4.0)]);
//
//        let b: bool = a.zero_diagonal();
//
//        assert_eq!(true, b);
//        assert_eq!(a_ref, a);
//    }
//
//    #[test]
//    fn find_nonzero_superdiagonal_0()
//    {
//        let a: Matrix<Real<f32>> = Matrix::new(&4, &4, &vec![Real::new(2.0), Real::new(0.0), Real::new(0.0),
//        Real::new(0.0), Real::new(0.0), Real::new(2.0), Real::new(1.0), Real::new(0.0), Real::new(0.0), Real::zero(),
//        Real::new(8.0), Real::new(0.0), Real::new(0.0), Real::new(0.0), Real::new(0.0), Real::new(2.0)]);
//
//        let b: usize = a.find_nonzero_superdiagonal();
//
//        assert_eq!(2, b);
//    }
//
//    #[test]
//    fn find_zero_superdiagonal_0()
//    {
//        let a: Matrix<Real<f32>> = Matrix::new(&4, &4, &vec![Real::new(2.0), Real::new(0.0), Real::new(0.0),
//        Real::new(0.0), Real::new(0.0), Real::new(2.0), Real::new(0.0), Real::new(0.0), Real::new(0.0), Real::zero(),
//        Real::new(8.0), Real::new(-1.0), Real::new(0.0), Real::new(0.0), Real::new(0.0), Real::new(2.0)]);
//
//        let b: usize = a.find_zero_superdiagonal();
//
//        assert_eq!(2, b);
//    }

//    #[test]
//    fn find_zero_superdiagonal_1()
//    {
//        let a: Matrix<Real<f32>> = Matrix::new(&4, &4, &vec![Real::new(2.0), Real::new(0.0), Real::new(0.0),
//        Real::new(0.0), Real::new(0.0), Real::new(2.0), Real::new(1.0), Real::new(0.0), Real::new(0.0), Real::zero(),
//        Real::new(8.0), Real::new(0.0), Real::new(0.0), Real::new(0.0), Real::new(0.0), Real::new(2.0)]);
//
//        let b: usize = a.find_zero_superdiagonal();
//
//        assert_eq!(1, b);
//    }

//    #[test]
//    fn find_zero_superdiagonal_1()
//    {
//        let a: Matrix<Real<f32>> = Matrix::new(&4, &4, &vec![Real::new(1.0), Real::new(5.0), Real::new(0.0),
//        Real::new(0.0), Real::new(0.0), Real::new(-7.0), Real::new(3.0), Real::new(0.0), Real::new(0.0), Real::new(0.0), Real::new(0.0), Real::new(4.0), Real::new(0.0), Real::new(0.0), Real::zero(), Real::new(4.0)]);
//
//        println!("{}");
//        let b: usize = a.find_zero_superdiagonal();
//
//        assert_eq!(2, b);
//        assert_eq!(a_ref, a);
//    }

//	#[test]
//	fn golub_kahan_step_0()
//	{
//	    let a: Matrix<Real<f32>> = Matrix::new(&4, &4, &vec![Real::one(), Real::new(2.0), Real::zero(), Real::zero(),
//        Real::zero(), Real::new(3.0), Real::new(4.0), Real::zero(), Real::zero(), Real::zero(), Real::new(5.0),
//        Real::new(7.0), Real::zero(), Real::zero(), Real::zero(), Real::new(7.0)]);
//
//        let (u, b, v): (Matrix<Real<f32>>, Matrix<Real<f32>>, Matrix<Real<f32>>) = a.golub_kahan_step();
//
//       //println!("b: {}", b);
//	}

    #[test]
    fn rot_0()
    {
        let f: Real<f64> = Real::new(0.0);
        let g: Real<f64> = Real::new(-3.0);
        let (c_ref, s_ref, r_ref): (Real<f64>, Real<f64>, Real<f64>) = (Real::zero(), Real::new(1.0),
        Real::new(-3.0));

        let (c, s, r): (Real<f64>, Real<f64>, Real<f64>) =  Matrix::rot(f, g);

        assert_eq!(c_ref, c);
        assert_eq!(s_ref, s);
        assert_eq!(r_ref, r);
    }

    #[test]
    fn rot_1()
    {
        let f: Real<f64> = Real::new(2.0);
        let g: Real<f64> = Real::new(3.0);
        let (c_ref, s_ref, r_ref): (Real<f64>, Real<f64>, Real<f64>) = (Real::new(0.554700196225229), Real::new(0.8320502943378437), Real::new(3.6055512754639896));

        let (c, s, r): (Real<f64>, Real<f64>, Real<f64>) =  Matrix::rot(f, g);

        assert_eq!(c_ref, c);
        assert_eq!(s_ref, s);
        assert_eq!(r_ref, r);
    }

    #[test]
    fn rot_2()
    {
        let f: Real<f64> = Real::new(3.0);
        let g: Real<f64> = Real::new(-5.0);
        let (c_ref, s_ref, r_ref): (Real<f64>, Real<f64>, Real<f64>) = (Real::new(-0.5144957554275266), Real::new(0.8574929257125443), Real::new(-5.8309518948453));

        let (c, s, r): (Real<f64>, Real<f64>, Real<f64>) =  Matrix::rot(f, g);

        assert_eq!(c_ref, c);
        assert_eq!(s_ref, s);
        assert_eq!(r_ref, r);
    }

//    #[test]
//    fn vsweep_0()
//    {
//        let u: Matrix<f64> = Matrix::one(&4);
//        let v: Matrix<f64> = Matrix::one(&4);
//
//        let d: Vector<f64> = Vector::new_row(&4, &vec![1.0 , 2.0, 3.0, 4.0]);
//        let e: Vector<f64> = Vector::new_row(&3, &vec![5.0, 6.0, 7.0]);
//
//        let d_ref: Vector<f64> = Vector::new_row(&4, &vec![5.463163355250679, 6.36054616275083, 7.359559770025111, 0.0031239153071850067]);
//        let e_ref: Vector<f64> = Vector::new_row(&3, &vec![2.1584718652373973, 3.295858340787664, 0.11174033462404505]);
//
//        let (u_p, d_n, e_n, v_p): (Matrix<f64>, Vector<f64>, Vector<f64>, Matrix<f64>) = Matrix::vsweep(u, d, e, v);
//
//        assert_eq!(d_ref, d_n);
//        assert_eq!(e_ref, e_n);
//    }
//
//     #[test]
//    fn vsweep_1()
//    {
//        let u: Matrix<f64> = Matrix::one(&4);
//        let v: Matrix<f64> = Matrix::one(&4);
//
//        let d: Vector<f64> = Vector::new_row(&4, &vec![-4.5826 , -3.7765, 1.4569, 3.0144]);
//        let e: Vector<f64> = Vector::new_row(&3, &vec![3.266, -1.5536, 1.2036]);
//
//        let d_ref: Vector<f64> = Vector::new_row(&4, &vec![-6.039122259764489, -3.277103355963835, 2.6844174078076057, 1.6235439947772108]);
//        let e_ref: Vector<f64> = Vector::new_row(&3, &vec![1.2504987877306921, -0.35519399014449327, 1.5027028697913907]);
//
//        let (u_p, d_n, e_n, v_p): (Matrix<f64>, Vector<f64>, Vector<f64>, Matrix<f64>) = Matrix::vsweep(u, d, e, v);
//
//        assert_eq!(d_ref, d_n);
//        assert_eq!(e_ref, e_n);
//    }

    #[test]
    fn svd_0()
    {
        let a: Matrix<f64> = Matrix::new(&4, &4, &vec![4.0, 1.0, -2.0,
        2.0, 1.0, 2.0, 0.0, -2.0, 0.0, 3.0, -2.0, 2.0, 2.0, 1.0, -2.0, -1.0]);

        let (u, s, v): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_sv();

        let u_ref: Matrix<f64> = Matrix::new(&4, &4, &vec![0.7506189754205654, -0.10396419803550966, -0.5550030946996072, 0.3431241235169469, 0.16660611095451494, 0.7151480228364367, 0.42071909480761316, 0.5327299057590197, 0.4914576812699054, -0.48965007323306603, 0.7174571018374724, -0.06298777918484932, 0.40900232890779525, 0.48784993260512727, -0.014910236788854345, -0.7710364602559812]);
        let v_ref: Matrix<f64> = Matrix::new(&4, &4, &vec![0.6414139394319739, 0.37707634018335073, -0.5915891468488654, 0.3105219369173846, 0.4773429354587908, 0.10210161763763852, 0.7839592576263362, 0.3835711981175504, -0.5312275172316613, 0.06255922864595147, -0.0954399832842174, 0.8395087119487317, 0.28023062860049164, -0.9184089700459931, -0.1622386307366864, 0.22732000622454226]);
        let s_ref: Matrix<f64> = Matrix::new(&4, &4, &vec![6.216089837372844, 0.0, 0.0, 0.0, 0.0, 3.3812545631601023, 0.0, 0.0, 0.0, 0.0, 3.0918649662716566, 0.0, 0.0, 0.0, 0.0, 1.1694937978293751]);

        assert_eq!(u_ref, u);
        assert_eq!(s_ref, s);
        assert_eq!(v_ref, v);
    }

    #[test]
    fn svd_1()
    {
        let a: Matrix<Real<f64>> = Matrix::new(&4, &4, &vec![Real::new(4.0), Real::new(1.0), Real::new(-2.0),
        Real::new(2.0), Real::new(1.0), Real::new(2.0), Real::new(0.0), Real::new(-2.0), Real::new(0.0), Real::new(3.0), Real::new(-2.0), Real::new(2.0), Real::new(2.0), Real::new(1.0), Real::new(-2.0), Real::new(-1.0)]);

        let (u, _s, v): (Matrix<Real<f64>>, Matrix<Real<f64>>, Matrix<Real<f64>>) = a.dec_sv();

        let u_ref: Matrix<Real<f64>> = Matrix::new(&4, &4, &vec![Real::new(0.7506189754205658), Real::new(-0.1039641980355133), Real::new(-0.555003094699607),
         Real::new(0.343124123516947),
Real::new(0.16660611095451502), Real::new(0.7151480228364399), Real::new(0.4207190948076098), Real::new(0.5327299057590196),
Real::new(0.49145768126990524), Real::new(-0.4896500732330621), Real::new(0.7174571018374756), Real::new(-0.06298777918484935),
Real::new(0.40900232890779536), Real::new(0.48784993260512743), Real::new(-0.014910236788857048), Real::new(-0.7710364602559812)]);

        let v_ref: Matrix<Real<f64>> = Matrix::new(&4, &4, &vec![Real::new(0.6414139394319739), Real::new(0.377076340183347), Real::new(-0.5915891468488668),
         Real::new(0.31052193691738456),
Real::new(0.4773429354587908), Real::new(0.10210161763764261), Real::new(0.7839592576263353), Real::new(0.38357119811755047),
Real::new(-0.5312275172316613), Real::new(0.06255922864595102), Real::new(-0.09543998328421785), Real::new(0.8395087119487314),
Real::new(0.2802306286004916), Real::new(-0.9184089700459929), Real::new(-0.16223863073668135), Real::new(0.22732000622454218)]);

        assert_eq!(u_ref, u);
        assert_eq!(v_ref, v);
    }

    #[test]
    fn inv_0()
    {
        let a: Matrix<f64> = Matrix::new(&3, &3, &vec![1.0, -2.0, 3.0, 2.0, -5.0, 12.0, 0.0, 2.0, -10.0]);
        //let (l, u, p) : (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu();
        let a_inv_ref : Matrix<f64> = Matrix::new(&3, &3, &vec![-13.0, 7.0, 4.5, -10.0, 5.0, 3.0, -2.0, 1.0, 0.5]);

        let a_inv: Matrix<f64> = a.inv();

        assert_eq!(a_inv_ref, a_inv);
    }

    #[test]
    fn inv_1()
    {
        let a: Matrix<f64> = Matrix::new(&3, &3, &vec![1.0, 0.0, 2.0, -1.0, 5.0, 0.0, 0.0, 3.0, -9.0]);
        //let (l, u, p) : (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu();
        let a_inv_ref : Matrix<f64> = Matrix::new(&3, &3, &vec![0.8823529411764706, -0.11764705882352942, 0.19607843137254904, 0.17647058823529413, 0.17647058823529413, 0.03921568627450981, 0.05882352941176471, 0.05882352941176471, -0.09803921568627452]);

        let a_inv: Matrix<f64> = a.inv();

        assert_eq!(a_inv_ref, a_inv);
    }

    #[test]
    fn inv_2()
    {
        let a: Matrix<f64> = Matrix::new(&5, &5, &vec![-1.0, 2.0, 3.0, 4.0, 5.0, -6.0, -7.0, 8.0, 9.0, 10.0, -11.0,
        12.0, 13.0, 14.0, 15.0, -16.0, -17.0, -18.0, -19.0, 20.0, -21.0, 22.0, -23.0, 24.0, 25.0]);

        let a_inv_ref : Matrix<f64> = Matrix::new(&5, &5, &vec![0.38478669499836576, -0.03759398496240601, -0.08489293886891143, -0.006578947368421052, -0.005720823798627002, 0.03571428571428603, -0.07142857142857142, 0.03571428571428571, 0.0, -0.000000000000000001734723475976807, -0.021739130434782705, 0.0, 0.04347826086956519, 0.0, -0.021739130434782608, -0.024517816279830296, 0.06390977443609022, -0.033671134357633165, -0.02631578947368421, 0.020594965675057208, 0.2953293559986926, -0.03007518796992481, -0.030414351095129147, 0.019736842105263157, -0.004576659038901602]);

        let a_inv: Matrix<f64> = a.inv();

        assert_eq!(a_inv_ref, a_inv);
    }

    #[test]
    fn hessenberg_decomposition_0()
    {
        let a: Matrix<f64> = Matrix::new(&3, &3, &vec![1.0, 5.0, 3.0, 1.0, 0.0, -7.0, 3.0, 8.0, 9.0]);

        let h_ref: Matrix<f64> = Matrix::new(&3, &3, &vec![1.0, -4.427188724235729, -3.794733192202053, -3.162277660168378, 8.399999999999995, -5.199999999999996, 0.0000000000000007771561172376096, 9.799999999999999, 0.600000000000001]);

        let q_ref: Matrix<f64> = Matrix::new(&3, &3, &vec![1.0, 0.0, 0.0, 0.0, -0.3162277660168378, 0.9486832980505135, 0.0, -0.9486832980505135, -0.3162277660168381]);

        let (q, h): (Matrix<f64>, Matrix<f64>) = a.dec_hessenberg();

        assert_eq!(q_ref, q);
        assert_eq!(h_ref, h);

    }
}