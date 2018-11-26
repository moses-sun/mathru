//#[macro_use]
//extern crate mathru;
//
//#[cfg(test)]
//mod complex_test
//{
//    use mathru::num::{Complex, Complex32, Complex64};
//    use mathru::algebra::abstr::identity::{One, Zero};
//    //use mathru::geometry::Trigonometry;
//
////    #[test]
////    fn abs()
////    {
////        let a : Complex32 = Complex::new_cart(-3.0, 4.0);
////        let abs: Complex32 = Complex::new_cart(5.0, 0.0);
////        assert_eq!(abs, a.abs());
////    }
////
////    #[test]
////    fn arg0()
////    {
////        let a: Complex32 = Complex::new_cart(-3.5, 6.0);
////        let mut angle : Real<f32> = Real::new(6.0/-3.5);
////        angle = angle.atan();
////        assert_eq!(angle, a.arg());
////    }
////
////    #[test]
////    fn arg1()
////    {
////        let phi: Real<f32> = Real::new(0.5);
////        let a: Complex32= Complex::new_polar(1.0, 0.5);
////        assert_eq!(phi, a.arg());
////    }
////
////    #[test]
////    fn add()
////    {
////        let a : Complex32= Complex::new_cart(-3.5, 6.5);
////        let b : Complex32= Complex::new_cart(2.0, -3.0);
////        let sum: Complex32= Complex::new_cart(-1.5, 3.5);
////        assert_eq!(sum, a + b);
////    }
////
////    #[test]
////    fn mul()
////    {
////        let a : Complex32= Complex::new_cart(-3.5, 6.5);
////        let b : Complex32= Complex::new_cart(2.0, -3.0);
////        let prod: Complex32= Complex::new_cart(-3.5 * 2.0 - 6.5 * -3.0, 6.5 * 2.0 + -3.5 * -3.0);
////        assert_eq!(prod, a * b);
////    }
////
////    #[test]
////    fn mulassign()
////    {
////        let mut a : Complex32= Complex::new_cart(-3.5, 6.5);
////        let b : Complex32= Complex::new_cart(2.0, -3.0);
////        let prod : Complex32= a * b;
////        a *= b;
////        assert_eq!(prod, a);
////    }
////
//    #[test]
//    fn zero()
//    {
//        let reference : Complex32 = Complex!(0.0, 0.0);
//        assert_eq!(reference, Complex32::zero());
//    }
//
//    #[test]
//    fn one()
//    {
//        let reference : Complex64 = Complex!(1.0, 0.0);
//        assert_eq!(reference, Complex64::one());
//    }
//
////    #[test]
////    fn conj()
////    {
////        let a_real : f32 = -3.5;
////        let a_imag : f32 = 6.5;
////        let a : Complex32 = Complex::new_cart(a_real, a_imag);
////        let conj : Complex32 = Complex::new_cart(a_real, -a_imag);
////        assert_eq!(conj, a.conj());
////    }
////
////    #[test]
////    fn div()
////    {
////        let a_real : f32 = -3.5;
////        let a_imag : f32 = 6.5;
////        let b_real : f32 = 2.0;
////        let b_imag : f32 = -3.0;
////        let a : Complex32= Complex::new_cart(a_real, a_imag);
////        let b : Complex32= Complex::new_cart(b_real, b_imag);
////        let prod: Complex32= Complex::new_cart((a_real * b_real + a_imag * b_imag)/(b_real* b_real +
////            b_imag * b_imag), (a_imag * b_real - a_real * b_imag)/(b_real * b_real + b_imag * b_imag));
////
////        assert_eq!(prod, a / b);
////    }
////
////    #[test]
////    fn sub()
////    {
////        let a : Complex32= Complex::new_cart(-3.5, 6.5);
////        let b : Complex32= Complex::new_cart(2.0, -3.0);
////        let diff: Complex32= Complex::new_cart(-5.5, 9.5);
////        assert_eq!(diff, a - b);
////    }
////
//////    #[test]
//////    fn inv()
//////    {
//////        let a_real : f32 = 1.0;
//////        let a_imag : f32 = 1.0;
//////        let b_real : f32 = 2.0;
//////        let b_imag : f32= -3.0;
//////        let a : Complex32= Complex::new_cart(a_real, a_imag);
//////        let b : Complex32= Complex::new_cart(b_real, b_imag);
//////        let c : Complex32= b.clone();
//////        assert_eq!(a/b,  c.inv());
//////    }
////
////    #[test]
////    fn neg0()
////    {
////        let a_real : f32 = 1.0;
////        let a_imag : f32 = 2.0;
////        let uut : Complex32= Complex::new_cart(a_real, a_imag);
////        assert_eq!(Complex::new_cart(-a_real, -a_imag), -uut);
////    }
//
//}
