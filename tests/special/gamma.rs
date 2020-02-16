extern crate mathru;

#[cfg(test)]
mod gamma
{
    use mathru::special::gamma;
    //use mathru::num::Complex;
    
    #[test]
    fn gamma0()
    {
        let x: f64 = 0.5;

        let gamma: f64 = gamma::gamma(x);

        assert_eq!(1.7724538509055163, gamma);
    }

    #[test]
    fn gamma1()
    {
        let x: f64 = -0.5;

        let gamma: f64 = gamma::gamma(x);

        assert_eq!(-3.544907701811029, gamma);
    }

    #[test]
    fn gamma2()
    {
         let x: f32 = -0.5;

        let gamma: f32 = gamma::gamma(x);

        assert_eq!(-3.5449073, gamma);
    }

    #[test]
    fn gamma_real()
    {
         let x: f64 = -0.5;

        let gamma: f64 = gamma::gamma(x);

        assert_eq!((-3.544907701811029), gamma);
    }

// TOOD
//    #[test]
//    fn gamma_complex0()
//    {
//         let x: Complex<f64> = Complex::new(-0.5, 0.0);
//
//        let gamma: Complex<f64> = gamma::gamma(x);
//
//        assert_eq!(Complex::new(-3.5449077018110304, 0.0), gamma);
//    }
//
//    #[test]
//    fn gamma_complex1()
//    {
//         let x: Complex<f64> = Complex::new(-0.5, 0.8);
//
//        let gamma: Complex<f64> = gamma::gamma(x);
//
//        assert_eq!(Complex::new(-0.7505627158730652, -0.06918543147121949), gamma);
//    }
//
//    #[test]
//    fn gamma_complex2()
//    {
//         let x: Complex<f64> = Complex::new(-0.2, 0.8);
//
//        let gamma: Complex<f64> = gamma::gamma(x);
//
//        assert_eq!(Complex::new(-0.5500387100945794, -0.5773132048330409), gamma);
//    }
//
//       #[test]
//    fn gamma_complex3()
//    {
//         let x: Complex<f64> = Complex::new(0.98, 0.8);
//
//        let gamma: Complex<f64> = gamma::gamma(x);
//
//        assert_eq!(Complex::new(0.6078826572404026, -0.2039648953365259), gamma);
//    }

      #[test]
    fn digamma0()
    {
        let x: f64 = 0.1;

        let gamma: f64 = gamma::digamma(x);

        assert_eq!(-10.423754940411076, gamma);
    }

    #[test]
    fn digamma1()
    {
        let x: f64 = 1.7;

        let gamma: f64 = gamma::digamma(x);

        assert_eq!(0.20854787487349322, gamma);
    }

//TODO
//    #[test]
//    fn digamma_complex0()
//    {
//         let x: Complex<f64> = Complex::new(0.5, 0.0);
//
//        let gamma: Complex<f64> = gamma::digamma(x);
//
//        assert_eq!(Complex::new(-1.9635100260214235, 0.0), gamma);
//    }
//
//    #[test]
//    fn digamma_complex1()
//    {
//         let x: Complex<f64> = Complex::new(0.5, 0.8);
//
//        let gamma: Complex<f64> = gamma::digamma(x);
//
//        assert_eq!(Complex::new(-0.3070040353355371, 1.5503173890363635), gamma);
//    }
//
//    #[test]
//    fn digamma_complex2()
//    {
//         let x: Complex<f64> = Complex::new(-0.5, -0.8);
//
//        let gamma: Complex<f64> = gamma::digamma(x);
//
//        assert_eq!(Complex::new(0.2547937174734515, -2.4491937935307453), gamma);
//    }

    #[test]
    fn gamma_ur0()
    {
        let x: f64 = 1.5;
        let a: f64 = 1.0;

        let gamma: f64 = gamma::gamma_ur(a, x);

        assert_eq!(0.2231301601484299, gamma);
    }

    #[test]
    fn gamma_lr0()
    {
        let x: f64 = 1.5;
        let a: f64 = 1.0;

        let gamma: f64 = gamma::gamma_lr(a, x);

        assert_eq!(0.7768698398515701, gamma);
    }

     #[test]
    fn gamma_lr1()
    {
        let x: f64 = 1.5;
        let a: f64 = 0.5;

        let gamma: f64 = gamma::gamma_lr(a, x);

        assert_eq!(0.9167354833364494, gamma);
    }

	#[test]
    fn gamma_lr2()
    {
        let x: f64 = 1.5_f64;
        let a: f64 = 0.5_f64;

        let gamma: f64 = gamma::gamma_lr(a, x);

        assert_eq!(0.9167354833364494, gamma);
    }

//TODO
//    #[test]
//    fn gamma_lr3()
//    {
//        let x: Complex<f64> = Complex::new(1.5, 0.0);
//        let a: Complex<f64> = Complex::new(0.5, 0.0);
//
//        let gamma: Complex<f64> = gamma::gamma_lr(a, x);
//
//        assert_eq!(Complex::new(0.9167354833364494, 0.0), gamma);
//    }
//
//     #[test]
//    fn gamma_lr4()
//    {
//        let x: Complex<f64> = Complex::new(1.5, 0.0);
//        let a: Complex<f64> = Complex::new(0.5, 1.0);
//
//        let gamma: Complex<f64> = gamma::gamma_lr(a, x);
//
//        assert_eq!(Complex::new(1.0484003068056669, -0.26574880173515353), gamma);
//    }
//
//     #[test]
//    fn gamma_lr5()
//    {
//        let x: Complex<f64> = Complex::new(1.5, -0.85);
//        let a: Complex<f64> = Complex::new(0.5, 1.0);
//
//        let gamma: Complex<f64> = gamma::gamma_lr(a, x);
//
//        assert_eq!(Complex::new(1.3724562080154135, -0.1109652182333), gamma);
//    }

  	#[test]
    fn gamma_l0()
    {
        let x: f64 = 1.5;
        let a: f64 = 0.5;

        let gamma: f64 = gamma::gamma_l(a, x);

        assert_eq!(1.6248713377014194, gamma);
    }

   	#[test]
    fn gamma_l1()
    {
        let x: f64 = 1.5;
        let a: f64 = 1.0;

        let gamma: f64 = gamma::gamma_l(a, x);

        assert_eq!(0.77686983985157, gamma);
    }

}