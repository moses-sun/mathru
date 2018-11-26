//#[macro_use]
//extern crate mathru;
//
//#[cfg(test)]
//mod real_test
//{
//    //use mathru::algebra::abstr::{Abs, Zero, One};
////    use mathru::numeric::{Bound, Number, Natural, Real, Integer};
////    use std::cmp::Ordering;
////    use std::ops::Neg;
////    use mathru::geometry::Trigonometry;
////    use mathru::analysis::Exponential;
//    use mathru::num::{Real, Real32, Real64};
//    use mathru::algebra::abstr::identity::{Zero, One};
//    //use std::{f32, f64};
//
////    #[test]
////    fn lower_bound()
////    {
////        let lower: Real<f32> = Real::lower_bound();
////        assert_eq!(f32::MIN, lower.get_primitive())
////    }
////
////    #[test]
////    fn max()
////    {
////        let upper: Real<f32> = Real::upper_bound();
////        assert_eq!(f32::MAX, upper.get_primitive())
////    }
////
////    #[test]
////    fn zero0() {
////        let refer: Real<f32> = Real::zero();
////        assert_eq!(Real::new(0.0), refer);
////    }
////
////    #[test]
////    fn one0() {
////        let refer: Real32 = Real::one();
////        assert_eq!(Real!(1.0), refer);
////    }
////
////    #[test]
////    fn one1() {
////        let refer: Real32 = Real::one();
////        assert_eq!(Real!(1.0), refer);
////    }
////
////    #[test]
////    fn cpmeq() {
////        let a: Real<f32> = Real::zero();
////        let b: Real<f32> = Real::zero();
////        assert_eq!(Ordering::Equal, a.partial_cmp(&b).unwrap());
////    }
////
////    #[test]
////    fn cmpgr() {
////        let a: Real<f32> = Real::one();
////        let b: Real<f32> = Real::zero();
////        assert_eq!(Ordering::Greater, a.partial_cmp(&b).unwrap());
////    }
////
////    #[test]
////    fn cmplo()
////    {
////        let a: Real<f32> = Real::zero();
////        let b: Real<f32> = Real::one();
////        assert_eq!(Ordering::Less, a.partial_cmp(&b).unwrap());
////    }
////
////    #[test]
////    fn eq0() {
////        let a: Real<f32> = Real::zero();
////        let b: Real<f32> = Real::zero();
////        assert_eq!(true, a == b);
////    }
////
////    #[test]
////    fn eq1() {
////        let a: Real<f32> = Real::zero();
////        let b: Real<f32> = Real::one();
////        assert_eq!(false, a == b);
////    }
////
////    #[test]
////    fn cast0()
////    {
////        let a: Real<f64> = Real::new(2.2);
////        let b: Natural<u8> = a.cast();
////        assert_eq!(Natural::new(2), b)
////    }
////
////    #[test]
////    fn cast1()
////    {
////        let a: Real<f64> = Real::new(2.2);
////        let b: Natural<u16> = a.cast();
////        assert_eq!(Natural::new(2), b)
////    }
////
////    #[test]
////    fn cast2()
////    {
////        let a: Real<f64> = Real::new(-2.2);
////        let b: Integer<i16> = a.cast();
////        assert_eq!(Integer::new(-2), b)
////    }
////
//////    #[test]
//////    fn from0()
//////    {
//////        let a: Real<f32> =  Real::new(2.2);
//////        let b: Natural<u16> = Natural::from(a);
//////        assert_eq!(Natural::new(2), b)
//////    }
////
////    #[test]
////    fn abs()
////    {
////        let a : Real<f64>= Real::new(-3.5);
////        let b : Real<f64>= Real::new(3.5);
////        assert_eq!(b , a.abs());
////    }
//
//    #[test]
//    fn add0()
//    {
//        let a: Real64 = Real!(12.0);
//        let b : Real64 = Real!(-6.0);
//        let c : Real64 = Real!(6.0);
//        assert_eq!(c, a + b);
//    }
//
////    #[test]
////    fn add1()
////    {
////        let a : Real64 = Real64!(12.0);
////        let b : Real64 = Real64!(-6.0);
////        let c : Real64 = Real64!(6.0);
////        let d : Real64 = Real64!(12.0);
////        assert_eq!(d, a + b + c);
////    }
//
////    #[test]
////    fn add1_ref()
////    {
////        let a : Real<f64>= Real::new(12.0);
////        let b : Real<f64>= Real::new(-6.0);
////        let c : Real<f64>= Real::new(6.0);
////        let d : Real<f64>= Real::new(12.0);
////        assert_eq!(d, &(&a + &b) + &c);
////    }
////
////    #[test]
////    fn add_assign()
////    {
////        let a: Real<f64>= Real::one();
////        let mut b: Real<f64>= Real::zero();
////        b += a;
////        assert_eq!(a, b);
////    }
////
////    #[test]
////    fn sub0()
////    {
////        let a : Real<f64>= Real::new(12.0);
////        let b : Real<f64>= Real::new(4.0);
////        let c : Real<f64>= Real::new(8.0);
////        assert_eq!(c, a-b);
////    }
////
////    #[test]
////    fn sub1()
////    {
////        let a : Real<f64>= Real::new(12.0);
////        let b : Real<f64>= Real::new(4.0);
////        let c : Real<f64>= Real::new(8.0);
////        let d : Real<f64>= Real::new(0.0);
////        assert_eq!(d, a-b-c);
////    }
////
////    #[test]
////    fn sub1_ref()
////    {
////        let a : Real<f64>= Real::new(12.0);
////        let b : Real<f64>= Real::new(4.0);
////        let c : Real<f64>= Real::new(8.0);
////        let d : Real<f64>= Real::new(0.0);
////        assert_eq!(d, &(&a-&b) - &c);
////    }
////
////    #[test]
////    fn sub_assign()
////    {
////        let a: Real<f64>= Real::one();
////        let mut b: Real<f64>= Real::new(2.0);
////        b -= a;
////        assert_eq!(a, b);
////    }
////
////    #[test]
////    fn div()
////    {
////        let a : Real<f64>= Real::new(4.0);
////        let b : Real<f64>= Real::new(2.0);
////        let c : Real<f64>= Real::new(2.0);
////        assert_eq!(c, a / b);
////    }
////
////    #[test]
////    fn div_ref()
////    {
////        let a : Real<f64>= Real::new(4.0);
////        let b : Real<f64>= Real::new(2.0);
////        let c : Real<f64>= Real::new(2.0);
////        assert_eq!(c, &a / &b);
////    }
////
////    #[test]
////    fn div_assign()
////    {
////        let a: Real<f64>= Real::one();
////        let mut b: Real<f64>= Real::new(1.0);
////        b /= a;
////        assert_eq!(a, b);
////    }
////
////    #[test]
////    fn neg()
////    {
////        let a : Real<f64>= Real::new(2.0);
////        let b : Real<f64>= Real::new(-2.0);
////        assert_eq!(b, a.neg());
////    }
////
//////  #[test]
//////  fn inv() {
//////    let a : Real<f64>= Real::new(2.0);
//////    let b : Real<f64>= Real::new(0.5);
//////    assert_eq!(b, a.inv());
//////  }
////
////    #[test]
////    fn mul0()
////    {
////        let a : Real<f64>= Real::new(12.0);
////        let b : Real<f64>= Real::new(-6.0);
////        let c : Real<f64>= Real::new(-72.0);
////        assert_eq!(c, a * b);
////    }
////
////    #[test]
////    fn mul0_ref()
////    {
////        let a : Real<f64>= Real::new(12.0);
////        let b : Real<f64>= Real::new(-6.0);
////        let c : Real<f64>= Real::new(-72.0);
////        assert_eq!(c, &a * &b);
////    }
////
////    #[test]
////    fn mul_assign()
////    {
////        let a: Real<f64>= Real::new(2.0);
////        let mut b: Real<f64>= Real::one();
////        b *= a;
////        assert_eq!(a, b);
////    }
////
////    #[test]
////    fn sin()
////    {
////        let limit: Real<f64> = Real::new(0.0000000000005);
////        assert!(Real::pi().sin() < limit);
////    }
////
////    #[test]
////    fn cos()
////    {
////        let upper_limit : Real<f64> = Real::one();
////        let lower_limit : Real<f64> = Real::new(0.9999999999999999999);
////        assert!((lower_limit <= Real::zero().cos()) && (Real::zero().cos() <= upper_limit));
////    }
////
////    #[test]
////    fn exp()
////    {
////        let a : Real<f64> = Real::zero();
////        let c : Real<f64> = Real::one();
////        assert_eq!(c, a.exp());
////    }
////
////    #[test]
////    fn ln()
////    {
////        let a: Real<f64> = Real::one();
////        let c: Real<f64> = Real::zero();
////        assert_eq!(c, a.ln());
////    }
////
////    #[test]
////    fn pow0()
////    {
////        let base: Real<f64>= Real::one();
////        let exp: Real<f64>= Real::zero();
////        assert_eq!(base, base.pow(exp));
////    }
////
////    #[test]
////    fn pow1()
////    {
////        let base: Real<f64> = Real::new(2.0);
////        let exp: Real<f64> = Real::new(3.0);
////        let res: Real<f64> = Real::new(8.0);
////        assert_eq!(res, base.pow(exp));
////    }
//
//}
