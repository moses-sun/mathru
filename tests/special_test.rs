extern crate mathru;

#[cfg(test)]
mod special_test
{
    use mathru::special;

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
    fn digamma0()
    {
        let x: f64 = 0.1;

        let gamma: f64 = special::gamma::digamma(x);

        assert_eq!(-10.423754940411138, gamma);
    }

    #[test]
    fn digamma1()
    {
        let x: f64 = 1.7;

        let gamma: f64 = special::gamma::digamma(x);

        assert_eq!(0.20854787487338688, gamma);
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