#[macro_use]
extern crate mathru;

#[cfg(test)]
mod real_test
{
    use mathru::algebra::abstr::{Zero, One};
    use mathru::elementary::{Exponential, Trigonometry, Power, Hyperbolic};
    use mathru::num::{Real, Real32, Real64};
    use std::ops::Neg;

//    #[test]
//    fn lower_bound()
//    {
//        let lower: Real<f32> = Real::lower_bound();
//        assert_eq!(f32::MIN, lower.get_primitive())
//    }
//
//    #[test]
//    fn max()
//    {
//        let upper: Real<f32> = Real::upper_bound();
//        assert_eq!(f32::MAX, upper.get_primitive())
//    }

    #[test]
    fn zero0()
    {
        let refer: Real<f32> = Real::zero();
        assert_eq!(Real::new(0.0), refer);
    }

    #[test]
    fn one0()
    {
        let refer: Real32 = Real::one();
        assert_eq!(Real!(1.0), refer);
    }

    #[test]
    fn one1()
    {
        let refer: Real32 = Real::one();
        assert_eq!(Real!(1.0), refer);
    }
//
//    #[test]
//    fn cpmeq() {
//        let a: Real<f32> = Real::zero();
//        let b: Real<f32> = Real::zero();
//        assert_eq!(Ordering::Equal, a.partial_cmp(&b).unwrap());
//    }
//
//    #[test]
//    fn cmpgr() {
//        let a: Real<f32> = Real::one();
//        let b: Real<f32> = Real::zero();
//        assert_eq!(Ordering::Greater, a.partial_cmp(&b).unwrap());
//    }
//
//    #[test]
//    fn cmplo()
//    {
//        let a: Real<f32> = Real::zero();
//        let b: Real<f32> = Real::one();
//        assert_eq!(Ordering::Less, a.partial_cmp(&b).unwrap());
//    }

    #[test]
    fn eq0()
    {
        let a: Real<f32> = Real::zero();
        let b: Real<f32> = Real::zero();
        assert_eq!(true, a == b);
    }

    #[test]
    fn eq1()
    {
        let a: Real<f32> = Real::zero();
        let b: Real<f32> = Real::one();
        assert_eq!(false, a == b);
    }
//
//    #[test]
//    fn cast0()
//    {
//        let a: Real<f64> = Real::new(2.2);
//        let b: Natural<u8> = a.cast();
//        assert_eq!(Natural::new(2), b)
//    }
//
//    #[test]
//    fn cast1()
//    {
//        let a: Real<f64> = Real::new(2.2);
//        let b: Natural<u16> = a.cast();
//        assert_eq!(Natural::new(2), b)
//    }
//
//    #[test]
//    fn cast2()
//    {
//        let a: Real<f64> = Real::new(-2.2);
//        let b: Integer<i16> = a.cast();
//        assert_eq!(Integer::new(-2), b)
//    }
//
////    #[test]
////    fn from0()
////    {
////        let a: Real<f32> =  Real::new(2.2);
////        let b: Natural<u16> = Natural::from(a);
////        assert_eq!(Natural::new(2), b)
////    }
//
    #[test]
    fn abs()
    {
        let a : Real<f64>= Real::new(-3.5);
        let b : Real<f64>= Real::new(3.5);
        assert_eq!(b , a.abs());
    }

    #[test]
    fn add0()
    {
        let a: Real64 = Real!(12.0);
        let b : Real64 = Real!(-6.0);
        let c : Real64 = Real!(6.0);
        assert_eq!(c, a + b);
    }

    #[test]
    fn add1()
    {
        let a : Real64 = Real64!(12.0);
        let b : Real64 = Real64!(-6.0);
        let c : Real64 = Real64!(6.0);
        let d : Real64 = Real64!(12.0);
        assert_eq!(d, a + b + c);
    }

    #[test]
    fn add1_ref()
    {
        let a : Real<f64>= Real::new(12.0);
        let b : Real<f64>= Real::new(-6.0);
        let c : Real<f64>= Real::new(6.0);
        let d : Real<f64>= Real::new(12.0);
        assert_eq!(d, &(&a + &b) + &c);
    }

    #[test]
    fn add_assign()
    {
        let a: Real<f64>= Real::one();
        let refer: Real<f64> = Real!(1.0);
        let mut b: Real<f64>= Real::zero();
        b += a;
        assert_eq!(refer, b);
    }

    #[test]
    fn sub0()
    {
        let a : Real<f64>= Real::new(12.0);
        let b : Real<f64>= Real::new(4.0);
        let c : Real<f64>= Real::new(8.0);
        assert_eq!(c, a-b);
    }

    #[test]
    fn sub1()
    {
        let a : Real<f64>= Real::new(12.0);
        let b : Real<f64>= Real::new(4.0);
        let c : Real<f64>= Real::new(8.0);
        let d : Real<f64>= Real::new(0.0);
        assert_eq!(d, a-b-c);
    }

    #[test]
    fn sub1_ref()
    {
        let a : Real<f64>= Real::new(12.0);
        let b : Real<f64>= Real::new(4.0);
        let c : Real<f64>= Real::new(8.0);
        let d : Real<f64>= Real::new(0.0);
        assert_eq!(d, &(&a-&b) - &c);
    }

    #[test]
    fn sub_assign()
    {
        let a: Real<f64>= Real::one();
        let refer: Real<f64> = Real64!(1.0);
        let mut b: Real<f64>= Real::new(2.0);
        b -= a;

        assert_eq!(refer, b);
    }

    #[test]
    fn div()
    {
        let a : Real<f64>= Real::new(4.0);
        let b : Real<f64>= Real::new(2.0);
        let c : Real<f64>= Real::new(2.0);
        assert_eq!(c, a / b);
    }

    #[test]
    fn div_ref()
    {
        let a : Real<f64>= Real::new(4.0);
        let b : Real<f64>= Real::new(2.0);
        let c : Real<f64>= Real::new(2.0);
        assert_eq!(c, &a / &b);
    }

    #[test]
    fn div_assign()
    {
        let a: Real<f64>= Real::one();
        let refer: Real<f64> = Real!(1.0);
        let mut b: Real<f64>= Real::new(1.0);
        b /= a;

        assert_eq!(refer, b);
    }

    #[test]
    fn neg()
    {
        let a : Real<f64>= Real::new(2.0);
        let b : Real<f64>= Real::new(-2.0);
        assert_eq!(b, a.neg());
    }
//
//    #[test]
//    fn inv()
//    {
//        let a : Real<f64>= Real::new(2.0);
//        let b : Real<f64>= Real::new(0.5);
//        assert_eq!(b, a.inv());
//    }

    #[test]
    fn mul0()
    {
        let a : Real<f64>= Real::new(12.0);
        let b : Real<f64>= Real::new(-6.0);
        let c : Real<f64>= Real::new(-72.0);
        assert_eq!(c, a * b);
    }

    #[test]
    fn mul0_ref()
    {
        let a : Real<f64>= Real::new(12.0);
        let b : Real<f64>= Real::new(-6.0);
        let c : Real<f64>= Real::new(-72.0);
        assert_eq!(c, &a * &b);
    }

    #[test]
    fn mul_assign()
    {
        let a: Real<f64>= Real::new(2.0);
        let refer: Real<f64> = Real!(2.0);
        let mut b: Real<f64>= Real::one();

        b *= a;

        assert_eq!(refer, b);
    }

    #[test]
    fn pi()
    {
        let pi: Real<f64> = Real::pi();
        let refer: Real<f64> = Real!(std::f64::consts::PI);

        assert_eq!(refer, pi);
    }

    #[test]
    fn sin0()
    {
        let sin_0: Real<f64> = Real::zero();

        assert_eq!(Real::zero().sin(), sin_0);
    }

    #[test]
    fn sin1()
    {
        let sin_pihalf: Real<f64> = Real::pi() / Real!(2.0);

        assert_eq!(Real::one(), sin_pihalf.sin());
    }

     #[test]
    fn sin2()
    {
        let sin_pihalf: Real<f64> = -Real::pi() / Real!(2.0);

        assert_eq!(-Real::one(), sin_pihalf.sin());
    }


    #[test]
    fn cos0()
    {
        let cos_0: Real<f64> = Real::one();

        assert_eq!(Real::zero().cos(), cos_0);
    }

    #[test]
    fn cos1()
    {
        let pi_half: Real<f64> = Real::pi() / Real!(2.0);

        assert!(Real!(0.0000000000000001) > pi_half.cos());
    }

     #[test]
    fn cos2()
    {
        let pi_half: Real<f64> = -Real::pi() / Real!(2.0);

        assert!(Real!(0.0000000000000001) > pi_half.cos());
    }

    #[test]
    fn tan0()
    {
        let phi_0: Real<f64> = Real::zero();

        assert_eq!(Real::zero(), phi_0.tan());
    }

    #[test]
    fn tan1()
    {
        let phi: Real<f64> = Real::pi() / Real!(4.0);

        assert_eq!(Real!(0.9999999999999999), phi.tan());
    }

    #[test]
    fn tan2()
    {
        let phi: Real<f64> = -Real::pi() / Real!(4.0);

        assert_eq!(Real!(-0.9999999999999999), phi.tan());
    }

    #[test]
    fn cot()
    {
        let phi: Real<f64> = Real::pi() / Real!(2.0);
        let cot_phi: Real<f64> = phi.cot();

        assert!(Real::zero() <= cot_phi);
        assert!(Real!(0.0000000000000001) > cot_phi);
    }

    #[test]
    fn csc()
    {
        let phi: Real<f64> = Real::pi() / Real!(2.0);

        assert_eq!(Real::one(), (Real::pi() / Real!(2.0)).sin() * phi.csc());
    }

     #[test]
    fn sec()
    {
        let phi: Real<f64> = Real::pi();

        assert_eq!(Real::one(), Real::pi().cos() * phi.sec());
    }

    #[test]
    fn arcsin0()
    {
        let phi: Real<f64> = Real::zero();

        assert_eq!(phi, Real::zero().arcsin());
    }

    #[test]
    fn arcsin1()
    {
        let phi: Real<f64> = Real::pi() / Real!(2.0);

        assert_eq!(phi, Real::one().arcsin());
    }

    #[test]
    fn arcsin2()
    {
        let phi: Real<f64> = -Real::pi() / Real!(2.0);

        assert_eq!(phi, (-Real::one()).arcsin());
    }

    #[test]
    fn arccos0()
    {
        let phi: Real<f64> = Real::zero();

        assert_eq!(phi, Real::one().arccos());
    }

    #[test]
    fn arccos1()
    {
        let phi: Real<f64> = Real::pi() / Real!(2.0);

        assert_eq!(phi, Real::zero().arccos());
    }

    #[test]
    fn arccos2()
    {
        let phi: Real<f64> = Real::pi();

        assert_eq!(phi, (-Real::one()).arccos());
    }

    #[test]
    fn arctan()
    {
        let phi: Real<f64> = Real::pi() / Real!(4.0);

        assert_eq!(phi, (Real::pi() / Real!(4.0)).tan().arctan());
    }

       #[test]
    fn arccot()
    {
        let phi: Real<f64> = Real::pi() / Real!(2.0);
        let cot_phi: Real<f64> = phi.arccot();

        assert_eq!(Real::new(0.5669115049410094), cot_phi);
    }

    #[test]
    fn arccsc()
    {
        let phi: Real<f64> = Real::pi() / Real!(4.0);

        assert_eq!(Real64!(0.7853981633974482), phi.csc().arccsc());
    }


     #[test]
    fn arcsec()
    {
        let phi: Real<f64> = Real::pi();
        let refer: Real<f64> = Real::pi();

        assert_eq!(refer, phi.sec().arcsec());
    }


    #[test]
    fn exp()
    {
        let a : Real<f64> = Real::zero();
        let c : Real<f64> = Real::one();
        assert_eq!(c, a.exp());
    }

    #[test]
    fn ln()
    {
        let a: Real<f64> = Real::one();
        let c: Real<f64> = Real::zero();
        assert_eq!(c, a.ln());
    }

    #[test]
    fn pow0()
    {
        let base: Real<f64>= Real::one();
        let exp: Real<f64>= Real::zero();
        assert_eq!(base, base.pow(&exp));
    }

    #[test]
    fn pow1()
    {
        let base: Real<f64> = Real::new(2.0);
        let exp: Real<f64> = Real::new(3.0);
        let res: Real<f64> = Real::new(8.0);
        assert_eq!(res, base.pow(&exp));
    }

     #[test]
    fn root()
    {
        let base: Real<f64> = Real::new(2.0);
        let exp: Real<f64> = Real::new(3.0);
        let res: Real<f64> = Real::new(8.0);
        assert_eq!(res, base.pow(&exp));
    }

    #[test]
    fn sinh()
    {
        let e: Real<f64> = Real::e();
        let x: Real<f64> = Real!(1.0_f64);

        let f: Real<f64> = x.sinh();
        // Solving sinh() at 1 gives `(e^2-1)/(2e)`
        let g: Real<f64> = (e * e - Real!(1.0)) / (Real!(2.0) * e);
        let abs_difference : Real<f64> = (f - g).abs();

        assert!(abs_difference < Real!(1e-10));
    }

    #[test]
    fn cosh()
    {
        let e: Real<f64> = Real::e();
        let x: Real<f64> = Real!(1.0_f64);
        let f: Real<f64> = x.cosh();
        // Solving cosh() at 1 gives this result
        let g: Real<f64> = (e * e + Real!(1.0)) / (Real!(2.0) * e);
        let abs_difference: Real<f64> = (f - g).abs();
        // Same result
        assert!(abs_difference < Real!(1.0e-10));
    }

    #[test]
    fn tanh()
    {

        let e: Real<f64> = Real::e();
        let x: Real<f64> = Real!(1.0_f64);

        let f: Real<f64> = x.tanh();
        // Solving tanh() at 1 gives `(1 - e^(-2))/(1 + e^(-2))`
        let g: Real<f64> = (Real::one() - e.pow(&Real!(-2.0)))/(Real::one() + e.pow(&Real!(-2.0)));
        let abs_difference: Real<f64> = (f - g).abs();

        assert!(abs_difference < Real!(1.0e-10));
    }

    #[test]
    fn sech()
    {
        let x: Real<f64> = Real!(0.0_f64);

        let f: Real<f64> = x.sech();
        let g: Real<f64> = Real::one();
        let abs_difference: Real<f64> = (f - g).abs();

        assert!(abs_difference < Real!(1.0e-10));
    }

    #[test]
    fn csch()
    {
        let x: Real<f64> = Real!(1.0_f64);

        let f: Real<f64> = x.csch();
        let g: Real<f64> = Real::one() / x.sinh();
        let abs_difference: Real<f64> = (f - g).abs();

        assert!(abs_difference < Real!(1.0e-10));
    }

    #[test]
    fn arcoth()
    {
        let x: Real<f64> = Real!(2.0_f64);

        let f: Real<f64> = x.arcoth();
        let g: Real<f64> = ((x + Real::one()) / ( x - Real::one())).ln() / Real::new(2.0);
        let abs_difference: Real<f64> = (f - g).abs();

        assert!(abs_difference < Real!(1.0e-10));
    }

    #[test]
    fn arsech()
    {
        let x: Real<f64> = Real!(0.5_f64);

        let f: Real<f64> = x.arsech();
        let g: Real<f64> = (Real::one() / x).arcosh();
        let abs_difference: Real<f64> = (f - g).abs();

        assert!(abs_difference < Real!(1.0e-10));
    }

    #[test]
    fn arcsch()
    {
        let x: Real<f64> = Real::new(2.0_f64);
	    let f: Real<f64> = x.arcsch();
	    let g: Real<f64> = (Real::new(1.0) / x).arsinh();
	    let abs_difference: Real<f64> = (f - g).abs();

	    assert!(abs_difference < Real::new(1.0e-10));
    }
}
