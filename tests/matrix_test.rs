extern crate mathru;

#[cfg(test)]
mod matrix_test
{
    use mathru::algebra::linear::{Matrix};
    //use mathru::algebra::abstr::{Semiring, Ring, Field, Zero, One, Sqrt};
    use mathru::num::{Natural};
    //use mathru::geometry::Trigonometry;

//    #[test]
//    fn zeros()
//    {
//        let rows: Natural<usize>= Natural::new(5);
//        let cols: Natural<usize>= Natural::new(7);
//        let mut m_zero : Matrix<Real<f32>> = Matrix::zero(&rows, &cols);
//        let dim: (Natural<usize>, Natural<usize>) = m_zero.dim();
//        assert_eq!(dim, (rows, cols));

//        for i in 0..rows.get_primitive()
//        {
//            for k in 0..cols.get_primitive()
//            {
//                assert_eq!(*(m_zero.get(&Natural::new(i),&Natural::new(k))), Real::zero());
//            }
//        }
//    }
//
//    #[test]
//    fn one()
//    {
//        let rows: Natural<usize>= Natural::new(5);
//        let mut m_ones : Matrix<Real<f32>> = Matrix::one(&rows);
//        let dim : (Natural<usize>, Natural<usize>) = m_ones.dim();
//        assert_eq!(dim, (rows, rows));
//
//        for i in 0..rows.get_primitive()
//        {
//            for k in 0..rows.get_primitive()
//            {
//                if i == k
//                {
//                    assert_eq!(*(m_ones.get(&Natural::new(i),&Natural::new(k))), Real::one());
//                }
//                else
//                {
//                    assert_eq!(*(m_ones.get(&Natural::new(i),&Natural::new(k))), Real::zero());
//                }
//            }
//        }
//    }
//
//    #[test]
//    fn get_column()
//    {
//        let a: Matrix<Real<f32>> = Matrix::new(&Natural::new(4), &Natural::new(4), &vec![Real::new(4.0), Real::one(),
//                                                                                         Real::new(-3.0), Real::new(2.0),
//                                                                                         Real::one(), Real::new(2.0), Real::zero(), Real::one(), Real::new(-2.0), Real::zero(), Real::new(3.0), Real::new(-2.0), Real::new(2.0), Real::one(), Real::new(-2.0), Real::new(-1.0)]);
//
//        let mut x: Vector<Real<f32>> = a.get_column(&Natural::zero());
//        let mut x_ref : Vector<Real<f32>> = Vector::new_column(&Natural::new(4), &vec![Real::new(4.0), Real::one(),
//        Real::new(-2.0), Real::new(2.0)]);
//        for i in 0..4
//        {
//            assert_eq!(*(x.get(&Natural::new(i))), *(x_ref.get(&Natural::new(i))));
//        }
//    }
//
//    #[test]
//    fn get_row()
//    {
//        let a: Matrix<Real<f64>> = Matrix::new(&Natural::new(4), &Natural::new(4), &vec![Real::new(4.0), Real::one(),
//                                                                                         Real::new(-2.0), Real::new(2.0),
//                                                                                         Real::one(), Real::new(2.0),
//                                                                                         Real::new(3.0), Real::one(),
//                                                                                         Real::new(-2.0), Real::zero
//                                                                                             (), Real::new(3.0),
//                                                                                         Real::new(-2.0), Real::new(2.0), Real::new(3.0), Real::new(-2.0), Real::new(-1.0)]);
//
//        let mut x: Vector<Real<f64>> = a.get_row(&Natural::one());
//        let mut x_ref : Vector<Real<f64>> = Vector::new_row(&Natural::new(4), &vec![Real::one(), Real::new(2.0),
//                                                                                  Real::new(3.0), Real::one()]);
//
//        for i in 0..4
//        {
//            assert_eq!(*(x.get(&Natural::new(i))), *(x_ref.get(&Natural::new(i))));
//        }
//    }
//
//    #[test]
//    fn add()
//    {
//        let dim: usize = 5;
//        let m_zero : Matrix<f32> = Matrix::zero(&dim, &dim);
//        let m_one : Matrix<f32> = Matrix::one(&dim);
//
//        let mut m_res : Matrix<f32> = &m_zero + &m_one;
//
//        for i in 0..dim
//        {
//            for k in 0..dim
//            {
//                if i == k
//                {
//                    assert_eq!(1.0, *(m_res.get(&i,&k)));
//                }
//                else
//                {
//                    assert_eq!(0.0, *(m_res.get(&i,&k)));
//                }
//            }
//        }
//    }
//
//    #[test]
//    fn mul_1()
//    {
//        let size: Natural<usize>= Natural::new(23);
//
//        let zero: Matrix<Real<f32>> = Matrix::zero(&size, &size);
//        let one: Matrix<Real<f32>> = Matrix::one(&size);
//
//        let res: Matrix<Real<f32>> = zero * one;
//
//        assert_eq!(res, Matrix::zero(&size, &size));
//    }
//
//    #[test]
//    fn mul_2()
//    {
//        let size: Natural<usize>= Natural::new(23);
//
//        let i1: Matrix<Real<f32>> = Matrix::one(&size);
//        let i2: Matrix<Real<f32>> = Matrix::one(&size);
//
//        let res: Matrix<Real<f32>> = i1 * i2;
//
//        assert_eq!(res, Matrix::one(&size));
//    }
//
//    #[test]
//    fn decompose_lu()
//    {
//        let dim: Natural<usize>= Natural::new(3);
//        let l_ref: Matrix<Real<f32>> = Matrix::new(&dim, &dim, &vec![Real::one(), Real::zero(), Real::zero(), Real::zero(), Real::one(), Real::zero(), Real::new(0.5), Real::new(0.25), Real::one()]);
//        let u_ref: Matrix<Real<f32>> = Matrix::new(&dim, &dim, &vec![Real::new(2.0), Real::new(-5.0), Real::new(12.0), Real::zero(), Real::new(2.0), Real::new(-10.0), Real::zero(), Real::zero(), Real::new(-0.5)]);
//        let p_ref: Matrix<Real<f32>> = Matrix::new(&dim, &dim, &vec![Real::zero(), Real::one(), Real::zero(), Real::zero(), Real::zero(), Real::one(), Real::one(), Real::zero(), Real::zero()]);
//        let a: Matrix<Real<f32>> = Matrix::new(&dim, &dim, &vec![Real::one(), Real::new(-2.0), Real::new(3.0), Real::new(2.0), Real::new(-5.0), Real::new(12.0), Real::zero(), Real::new(2.0), Real::new(-10.0)]);
//
//        let (l, u, p) : (Matrix<Real<f32>>, Matrix<Real<f32>>, Matrix<Real<f32>>) = a.dec_lu();
//
//        assert_eq!(l_ref, l);
//        assert_eq!(u_ref, u);
//        assert_eq!(p_ref, p);
//        assert_eq!(p*a, l*u);
//    }
//
//    #[test]
//    fn givens()
//    {
//        let m : Natural<usize>= Natural::new(4);
//        let i : Natural<usize>= Natural::one();
//        let j : Natural<usize>= Natural::new(2);
//        let theta : Real<f32> = Real::new(1.0);
//        let c : Real<f32> = theta.cos();
//        let s : Real<f32> = theta.sin();
//        let mut givens : Matrix<Real<f32>> = Matrix::givens(&m, &i, &j, &c, &s);
//        assert_eq!(*(givens.get(&Natural::zero(),&Natural::zero())), Real::one());
//        assert_eq!(*(givens.get(&i,&i)), theta.cos());
//        assert_eq!(*(givens.get(&j,&j)), theta.cos());
//        assert_eq!(*(givens.get(&j,&i)), -theta.sin());
//        assert_eq!(*(givens.get(&i,&j)), theta.sin());
//    }
//
//    #[test]
//    fn mul3()
//    {
//        let m : Natural<usize>= Natural::new(4);
//        let i : Natural<usize>= Natural::one();
//        let j : Natural<usize>= Natural::new(2);
//        let theta : Real<f64> = Real::new(1.0);
//        let c : Real<f64> = theta.cos();
//        let s : Real<f64> = theta.sin();
//        let givens : Matrix<Real<f64>> = Matrix::givens(&m, &i, &j, &c, &s);
//        let givens_t : Matrix<Real<f64>> = givens.clone().trans();
//        let res_ref : Matrix<Real<f64>> = Matrix::one(&m);
//        let res : Matrix<Real<f64>> = givens_t * givens;
//        assert_eq!(res_ref, res);
//    }
//
//    #[test]
//    fn transpose()
//    {
//        let uut: Matrix<Real<f32>> = Matrix::new(&Natural::new(4), &Natural::new(2), &vec![Real::one(), Real::zero(), Real::new(3.0), Real::zero(), Real::one(), Real::new(-7.0), Real::new(0.5), Real::new(0.25)]);
//        let res: Matrix<Real<f32>> = uut.trans();
//        let trans_ref: Matrix<Real<f32>> = Matrix::new(&Natural::new(2), &Natural::new(4), &vec![Real::one(), Real::new(3.0), Real::one(), Real::new(0.5), Real::zero(), Real::zero(), Real::new(-7.0), Real::new(0.25)]);
//        assert_eq!(res, trans_ref);
//    }
//
////    #[test]
////    fn transpose_inplace()
////    {
////        let uut: Matrix<Real<f32>> = Matrix::new(&Natural::new(4), &Natural::new(2), &vec![Real::one(), Real::zero(), Real::new(3.0), Real::zero(), Real::one(), Real::new(-7.0), Real::new(0.5), Real::new(0.25)]);
////        uut.trans_inplace();
////        let trans_ref: Matrix<Real<f32>> = Matrix::new(&Natural::new(2), &Natural::new(4), &vec![Real::one(), Real::new(3.0), Real::one(), Real::new(0.5), Real::zero(), Real::zero(), Real::new(-7.0), Real::new(0.25)]);
////        assert_eq!(uut, trans_ref);
////    }
//
//    #[test]
//    fn decompose_qr1()
//    {
//        let a : Matrix<Real<f32>> = Matrix::new(&Natural::new(4), &Natural::new(2), &vec![Real::new(3.0), Real::new(5.0), Real::zero(), Real::new(2.0), Real::zero(), Real::zero(), Real::new(4.0), Real::new(5.0)]);
//        //println!("A: {}", a);
//        let (q,r) : (Matrix<Real<f32>>, Matrix<Real<f32>>) =  a.dec_qr();
//
//        let r_ref : Matrix<Real<f32>> = Matrix::new(&Natural::new(4), &Natural::new(2), &vec![Real::new(5.0), Real::new(7.0), Real::zero(), Real::new(5.0).sqrt(), Real::zero(), Real::zero(), Real::zero(), Real::zero()]);
//        //println!("R_ref: {}", r_ref);
//        assert_eq!(r_ref, r);
//    }
//
//    #[test]
//    fn decompose_qr2()
//    {
//        let a : Matrix<Real<f32>> = Matrix::new(&Natural::new(3), &Natural::new(3), &vec![Real::new(12.0), Real::new
//            (-51.0), Real::new(4.0), Real::new(6.0), Real::new(167.0), Real::new(-68.0), Real::new(-4.0), Real::new(24.0), Real::new(-41.0)]);
//        //println!("A: {}", a);
//        let (q,r) : (Matrix<Real<f32>>, Matrix<Real<f32>>) =  a.dec_qr();
//
//        let r_ref : Matrix<Real<f32>> = Matrix::new(&Natural::new(3), &Natural::new(3), &vec![Real::new(-14.0),
//                                                                                              Real::new(-21.0),
//                                                                                              Real::new(14.0),
//                                                                                              Real::zero(),
//                                                                                              Real::new(-175.0),
//                                                                                                        Real::new(70.0)
//                                                                                                  , Real::zero(),
//                                                                                              Real::zero(), Real::new
//                                                                                                  (-35.0)]);
//        //println!("ref: {}", r_ref);
//
//        //println!("r: {}", r);
//        //assert_eq!(r_ref, r);
//        //assert_eq!(q*r, a);
//    }
//
//
//    #[test]
//    fn determinant()
//    {
//        let a: Matrix<Real<f32>> = Matrix::new(&Natural::new(3), &Natural::new(3), &vec![Real::one(), Real::new(-2.0), Real::new(3.0), Real::new(2.0),
//    Real::new(-5.0), Real::new(12.0), Real::one(), Real::new(2.0), Real::new(-10.0)]);
//        let d: Real<f32> = a.det();
//        assert_eq!(Real::new(-11.0), d);
//    }
//
//
//    #[test]
//    fn householder()
//    {
//        let a: Matrix<Real<f32>> = Matrix::new(&Natural::new(4), &Natural::new(4), &vec![Real::new(4.0), Real::one(),
//																						Real::new(-2.0), Real::new(2.0),
//        Real::one(), Real::new(2.0), Real::zero(), Real::one(), Real::new(-2.0), Real::zero(), Real::new(3.0), Real::new(-2.0), Real::new(2.0), Real::one(), Real::new(-2.0), Real::new(-1.0)]);
//
//        //println!("a: {}", a);
//
//        let x: Vector<Real<f32>> = a.get_column(&Natural::zero());
//        let (m, _n) : (Natural<usize>, Natural<usize>) = x.dim();
//        let p: Real<f32> = Real::new(2.0);
//        let abs: Real<f32> =  x.p_norm(&p);
//        let mut y: Vector<Real<f32>> = Vector::zero(&m);
//        *y.get_mut(&Natural::zero()) = abs;
//
//        let v : Vector<Real<f32>> = x-y;
//
//        //println!("Reflector: {}", v);
//
//        let u_1: Matrix<Real<f32>> = Matrix::householder(&v);
//        //println!("Householder: {}", u_1);
//
//        //println!("h*a = {}", (&u_1*&a));
//    }
//
//    #[test]
//    fn svd()
//    {
//        let a: Matrix<Real<f32>> = Matrix::new(&Natural::new(4), &Natural::new(4), &vec![Real::new(4.0), Real::one(), Real::new(-2.0), Real::new(2.0),
//        Real::one(), Real::new(2.0), Real::zero(), Real::one(), Real::new(-2.0), Real::zero(), Real::new(3.0), Real::new(-2.0), Real::new(2.0), Real::one(), Real::new(-2.0), Real::new(-1.0)]);
//
//
//        let x: Vector<Real<f32>> = a.get_column(&Natural::zero());
//        a.dec_sv();
//
//
//    }
}
