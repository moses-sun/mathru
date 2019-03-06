extern crate mathru;

#[cfg(test)]
mod special_test
{
    use mathru::special;
    use mathru::num::Real;
    use mathru::num::Complex;
    //use mathru::num::primitives;

    #[test]
    fn beta0()
    {
        let x: f64 = 5.0;
        let y: f64 = 6.5;

        let beta: f64 = special::beta::beta(x, y);

        assert_eq!(0.0005806371131448529, beta);
    }


    #[test]
    fn gamma0()
    {
        let x: f64 = 0.5;

        let gamma: f64 = special::gamma::gamma(x);

        assert_eq!(1.7724538509055163, gamma);
    }

    #[test]
    fn gamma1()
    {
        let x: f64 = -0.5;

        let gamma: f64 = special::gamma::gamma(x);

        assert_eq!(-3.544907701811029, gamma);
    }

    #[test]
    fn gamma2()
    {
         let x: f32 = -0.5;

        let gamma: f32 = special::gamma::gamma(x);

        assert_eq!(-3.5449073, gamma);
    }

    #[test]
    fn gamma_real()
    {
         let x: Real<f64> = Real::new(-0.5);

        let gamma: Real<f64> = special::gamma::gamma(x);

        assert_eq!(Real::new(-3.544907701811029), gamma);
    }

    #[test]
    fn gamma_complex0()
    {
         let x: Complex<f64> = Complex::new(-0.5, 0.0);

        let gamma: Complex<f64> = special::gamma::gamma(x);

        assert_eq!(Complex::new(-3.5449077018110304, 0.0), gamma);
    }

    #[test]
    fn gamma_complex1()
    {
         let x: Complex<f64> = Complex::new(-0.5, 0.8);

        let gamma: Complex<f64> = special::gamma::gamma(x);

        assert_eq!(Complex::new(-0.7505627158730652, -0.06918543147121949), gamma);
    }

    #[test]
    fn gamma_complex2()
    {
         let x: Complex<f64> = Complex::new(-0.2, 0.8);

        let gamma: Complex<f64> = special::gamma::gamma(x);

        assert_eq!(Complex::new(-0.5500387100945794, -0.5773132048330409), gamma);
    }

       #[test]
    fn gamma_complex3()
    {
         let x: Complex<f64> = Complex::new(0.98, 0.8);

        let gamma: Complex<f64> = special::gamma::gamma(x);

        assert_eq!(Complex::new(0.6078826572404026, -0.2039648953365259), gamma);
    }

    #[test]
    fn digamma0()
    {
        let x: f64 = 0.1;

        let gamma: f64 = special::gamma::digamma(x);

        assert_eq!(-10.423754940411076, gamma);
    }

    #[test]
    fn digamma1()
    {
        let x: f64 = 1.7;

        let gamma: f64 = special::gamma::digamma(x);

        assert_eq!(0.20854787487349322, gamma);
    }

    #[test]
    fn digamma_complex0()
    {
         let x: Complex<f64> = Complex::new(0.5, 0.0);

        let gamma: Complex<f64> = special::gamma::digamma(x);

        assert_eq!(Complex::new(-1.9635100260214235, 0.0), gamma);
    }

    #[test]
    fn digamma_complex1()
    {
         let x: Complex<f64> = Complex::new(0.5, 0.8);

        let gamma: Complex<f64> = special::gamma::digamma(x);

        assert_eq!(Complex::new(-0.3070040353355371, 1.5503173890363635), gamma);
    }

    #[test]
    fn digamma_complex2()
    {
         let x: Complex<f64> = Complex::new(-0.5, -0.8);

        let gamma: Complex<f64> = special::gamma::digamma(x);

        assert_eq!(Complex::new(0.2547937174734515, -2.4491937935307453), gamma);
    }

    #[test]
    fn gamma_ur0()
    {
        let x: f64 = 1.5;
        let a: f64 = 1.0;

        let gamma: f64 = special::gamma::gamma_ur(a, x);

        assert_eq!(0.2231301601484299, gamma);
    }

    #[test]
    fn gamma_lr0()
    {
        let x: f64 = 1.5;
        let a: f64 = 1.0;

        let gamma: f64 = special::gamma::gamma_lr(a, x);

        assert_eq!(0.7768698398515701, gamma);
    }

     #[test]
    fn gamma_lr1()
    {
        let x: f64 = 1.5;
        let a: f64 = 0.5;

        let gamma: f64 = special::gamma::gamma_lr(a, x);

        assert_eq!(0.9167354833364494, gamma);
    }

	#[test]
    fn gamma_lr2()
    {
        let x: Real<f64> = Real::new(1.5_f64);
        let a: Real<f64> = Real::new(0.5_f64);

        let gamma: Real<f64> = special::gamma::gamma_lr(a, x);

        assert_eq!(Real::new(0.9167354833364494), gamma);
    }

    #[test]
    fn gamma_lr3()
    {
        let x: Complex<f64> = Complex::new(1.5, 0.0);
        let a: Complex<f64> = Complex::new(0.5, 0.0);

        let gamma: Complex<f64> = special::gamma::gamma_lr(a, x);

        assert_eq!(Complex::new(0.9167354833364494, 0.0), gamma);
    }

     #[test]
    fn gamma_lr4()
    {
        let x: Complex<f64> = Complex::new(1.5, 0.0);
        let a: Complex<f64> = Complex::new(0.5, 1.0);

        let gamma: Complex<f64> = special::gamma::gamma_lr(a, x);

        assert_eq!(Complex::new(1.0484003068056669, -0.26574880173515353), gamma);
    }

     #[test]
    fn gamma_lr5()
    {
        let x: Complex<f64> = Complex::new(1.5, -0.85);
        let a: Complex<f64> = Complex::new(0.5, 1.0);

        let gamma: Complex<f64> = special::gamma::gamma_lr(a, x);

        assert_eq!(Complex::new(1.3724562080154135, -0.1109652182333), gamma);
    }

  	#[test]
    fn gamma_l0()
    {
        let x: f64 = 1.5;
        let a: f64 = 0.5;

        let gamma: f64 = special::gamma::gamma_l(a, x);

        assert_eq!(1.6248713377014194, gamma);
    }

   	#[test]
    fn gamma_l1()
    {
        let x: f64 = 1.5;
        let a: f64 = 1.0;

        let gamma: f64 = special::gamma::gamma_l(a, x);

        assert_eq!(0.77686983985157, gamma);
    }
//    #[test]
//    fn gamma_lr_c0()
//    {
//        let x: Complex<f64> = Complex::new(0.5, 0.2);
//        let a: Complex<f64> = Complex::new(2.0, 0.0);
//
//        let gamma: Complex<f64> = special::gamma::gamma_lr(a, x);
//
//        assert_eq!(Complex::new(0.7768698398515701, 0.0), gamma);
//    }

    #[test]
    fn hypergeometrical_f21_0()
    {
        let a: f64 = 0.5_f64;
        let b: f64 = 0.3_f64;
        let c: f64 = 0.7_f64;
        let z: f64 = -5.8_f64;

        let h: f64 = special::hypergeometrical::f21(a, b, c, z);

        assert_eq!(0.6463419208795624, h);
    }

    #[test]
    fn hypergeometrical_f21_1()
    {
        let a: f64 = 0.5_f64;
        let b: f64 = 0.3_f64;
        let c: f64 = 0.7_f64;
        let z: f64 = -0.5_f64;

        let h: f64 = special::hypergeometrical::f21(a, b, c, z);

        assert_eq!(0.9157005646955769, h);
    }

    #[test]
    fn hypergeometrical_f21_2()
    {
        let a: f64 = 0.5_f64;
        let b: f64 = 0.3_f64;
        let c: f64 = 0.7_f64;
        let z: f64 = 0.5_f64;

        let h: f64 = special::hypergeometrical::f21(a, b, c, z);

        assert_eq!(1.1561182056693702, h);
    }

    #[test]
    fn hypergeometrical_f21_3()
    {
        let a: f64 = 0.5_f64;
        let b: f64 = 0.3_f64;
        let c: f64 = 0.7_f64;
        let z: f64 = 0.8_f64;

        let h: f64 = special::hypergeometrical::f21(a, b, c, z);

        assert_eq!(1.3858902990044464, h);
    }

     #[test]
    fn hypergeometrical_f21_4()
    {
        let a: f64 = 0.5_f64;
        let b: f64 = 0.3_f64;
        let c: f64 = 0.7_f64;
        let z: f64 = 0.9_f64;

        let h: f64 = special::hypergeometrical::f21(a, b, c, z);

        assert_eq!(1.577586297244991, h);
    }

//    #[test]
//    fn hypergeometrical_f21_5()
//    {
//        let a: f64 = 0.5_f64;
//        let b: f64 = 0.3_f64;
//        let c: f64 = 0.7_f64;
//        let z: f64 = 3.9_f64;
//
//        let h: f64 = special::hypergeometrical::f21(a, b, c, z);
//
//        assert_eq!(1.3858900032709065, h);
//    }

//    #[test]
//    fn hypergeometrical_f21_6()
//    {
//        let a: f64 = 0.5_f64;
//        let b: f64 = 1.5_f64;
//        let c: f64 = 1.5_f64;
//        let z: f64 = -2.0_f64;
//
//        let h: f64 = special::hypergeometrical::f21(a, b, c, z);
//
//        assert_eq!(1.577586290185282, h);
//    }

    #[test]
    fn hypergeometrical_f21_7()
    {
        let a: f64 = 0.5_f64;
        let b: f64 = 0.3_f64;
        let c: f64 = 0.7_f64;
        let z: f64 = -2.0_f64;

        let h: f64 = special::hypergeometrical::f21(a, b, c, z);

        assert_eq!(0.7834881981920094, h);
    }

     #[test]
    fn hypergeometrical_f21_8()
    {
        let a: f64 = 0.1_f64;
        let b: f64 = 0.2_f64;
        let c: f64 = 0.3_f64;
        let z: f64 = 0.5_f64;

        let h: f64 = special::hypergeometrical::f21(a, b, c, z);

        assert_eq!(1.0464328112173522, h);
    }

}