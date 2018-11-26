#[macro_use]
extern crate mathru;

#[cfg(test)]
mod natural_test
{
    use std::ops::Rem;
    use mathru::num::{Natural, Natural8}; //, Integer, Real};
    use mathru::algebra::abstr::identity::{Zero, One};
    use std::ops::{Add, AddAssign, Mul, MulAssign};
    use std::cmp::{Ordering, PartialOrd};
    use mathru::algebra::abstr::Number;
    //use mathru::analysis::Interval;
    //use mathru::set::Set;

    #[test]
    fn new0()
    {
        let refer: Natural<u8> = Natural::zero();
        let a: Natural<u8> = Natural::new(0);
        assert_eq!(a, refer);
    }

    #[test]
    fn new1()
    {
        let refer: Natural8 = Natural!(255);
        let uut: Natural<u8> = Natural!(255);

        assert_eq!(refer, uut);
    }

    #[test]
    fn one0()
    {
        let refer: Natural<u16> = Natural::one();
        assert_eq!(Natural::new(1), refer);
    }

    #[test] fn one1()
    {
        let refer: Natural<u8> = Natural::one();
        assert_eq!(Natural::new(1), refer);
    }

    #[test]
    fn zero0() {
        let refer: Natural<u8> = Natural::zero();
        assert_eq!(Natural::new(0), refer);
    }

    #[test]
    fn cmpeq() {
        let a: Natural<u8> = Natural::zero();
        let b: Natural<u8> = Natural::zero();
        assert_eq!(Ordering::Equal, a.partial_cmp(&b).unwrap());
    }

    #[test]
    fn cmpgr() {
        let a: Natural<u8> = Natural::one();
        let b: Natural<u8> = Natural::zero();
        assert_eq!(Ordering::Greater, a.partial_cmp(&b).unwrap());
    }

    #[test]
    fn cmplo() {
        let a: Natural<u8> = Natural::zero();
        let b: Natural<u8> = Natural::one();
        assert_eq!(Ordering::Less, a.partial_cmp(&b).unwrap());
    }

    #[test]
    fn eq0() {
        let a: Natural<u8> = Natural::zero();
        let b: Natural<u8> = Natural::zero();
        assert_eq!(true, a == b);
    }

    #[test]
    fn eq1() {
        let a: Natural<u8> = Natural::zero();
        let b: Natural<u8> = Natural::one();
        assert_eq!(false, a == b);
    }

//   #[test]
//    fn cast0()
//    {
//        let a: Natural<u8> = Natural::new(2);
//        let b: Real<f32> = a.cast();
//        assert_eq!(Real::new(2.0), b)
//    }
//
//    #[test]
//    fn cast1()
//    {
//        let a: Natural<u16> = Natural::new(2);
//        let b: Real<f32> = a.cast();
//        assert_eq!(Real::new(2.0), b)
//    }
//
//    #[test]
//    fn cast2()
//    {
//        let a: Real<f32> =  Real::new(2.2);
//        let b: Natural<u16> = a.cast();
//        assert_eq!(Natural::new(2), b)
//    }
//
//    #[test]
//    fn cast3()
//    {
//        let a: Integer<i8> =  Integer::new(1);
//        let b: Natural<u16> = a.cast();
//        assert_eq!(Natural::new(1), b)
//    }
//
//     #[test]
//    fn cast4()
//    {
//        let a: Natural<u8> =  Natural::new(1);
//        let b: Integer<i16> = a.cast();
//        assert_eq!(Integer::new(1), b)
//    }

    #[test]
    fn add()
    {
        let a: Natural<u8> = Natural::zero();
        let b: Natural<u8> = Natural::one();
        assert_eq!(b, a + b);
    }

    #[test]
    fn add_ref()
    {
        let a: Natural<u8> = Natural::zero();
        let b: Natural<u8> = Natural::one();
        assert_eq!(b, &a + &b);
    }

    #[test]
    fn add_assign() {
        let mut a: Natural<u8> = Natural::zero();
        let b: Natural<u8> = Natural::one();
        a += b;
        assert_eq!(b, a);
    }

    #[test]
    fn mul0() {
        let a: Natural<u8> = Natural::zero();
        let b: Natural<u8> = Natural::one();
        assert_eq!(a, a * b);
    }

    #[test]
    fn mul1() {
        let a: Natural<u8> = Natural::one();
        let b: Natural<u8> = Natural::one();
        assert_eq!(a, a * b);
    }

    #[test]
    fn mul_assign() {
        let mut a: Natural<u8> = Natural::one();
        let b: Natural<u8> = Natural::one();
        a *= b;
        assert_eq!(b, a);
    }

    #[test]
    fn factorial0()
    {
        let fact_ref: Natural<u8> = Natural::one();
        let fact: Natural<u8> = Natural::new(0).factorial();
        assert_eq!(fact_ref, fact);
    }

    #[test]
    fn factorial1()
    {
        let fact_ref: Natural<u8> = Natural::one();
        let fact: Natural<u8> = Natural::one().factorial();
        assert_eq!(fact_ref, fact);
    }

    #[test]
    fn factorial2()
    {
        let fact_ref: Natural<u16>= Natural::new(720);
        let fact: Natural<u16> = Natural::new(6).factorial();
        assert_eq!(fact_ref, fact);
    }

    #[test]
    fn rem0()
    {
        let a: Natural<u8> = Natural::new(7);
        let b: Natural<u8> = Natural::new(11);
        let c: Natural<u8> = Natural::new(4);

        assert_eq!(b.rem(a), c);
    }

    #[test]
    fn gcd0()
    {
        let a: Natural<u8> = Natural::new(11);
        let b: Natural<u8> = Natural::new(4);
        let c: Natural<u8> = Natural::one();

        assert_eq!(a.gcd(b), c);
    }

    #[test]
    fn gcd1()
    {
        let a: Natural<u8> = Natural::new(56);
        let b: Natural<u8> = Natural::new(7);
        let c: Natural<u8> = Natural::new(7);

        assert_eq!(a.gcd(b), c);
    }

    #[test]
    fn gcd2()
    {
        let a: Natural<u8> = Natural::new(68);
        let b: Natural<u8> = Natural::new(8);
        let c: Natural<u8> = Natural::new(4);

        assert_eq!(a.gcd(b), c);
    }

    #[test]
    fn max0()
    {
        let a: Natural<u8> = Natural::new(5);
        let b: Natural<u8> = Natural::new(39);

        assert_eq!(b, b.max(a));
    }

    #[test]
    fn max1()
    {
        let a: Natural<u8> = Natural::new(3);
        let b: Natural<u8> = Natural::new(5);
        let c: Natural<u8> = a.max(b);

        assert_eq!(c, b);
    }

    #[test]
    fn min()
    {
        let a: Natural<u8> = Natural::new(3);
        let b: Natural<u8> = Natural::new(5);
        let c: Natural<u8> = a.min(b);

        assert_eq!(c, a);
    }
//
//    #[test]
//    fn get_primitive()
//    {
//        let a: Natural<u8> = Natural::new(8);
//
//        assert_eq!(a.get_primitive(), 8);
//    }
//
//    #[test]
//    fn interval_closed0()
//    {
//        let interval: Interval<Natural<u8>> = Interval::closed(Natural::zero(), Natural::one());
//        let mut ref_value: Natural<u8> = Natural::zero();
//        for i in interval
//        {
//            assert_eq!(ref_value, i);
//            ref_value += Natural::one();
//        }
//        assert_eq!(Natural::zero(), interval.clone().next().unwrap());
//    }
//
//    #[test]
//    fn interval_open0()
//    {
//        let interval: Interval<Natural<u8>> = Interval::open(Natural::zero(), Natural::one());
//        let mut ref_value: Natural<u8> = Natural::zero();
//        for i in interval
//        {
//            assert!(false);
//        }
//    }
//
//    #[test]
//    fn range()
//    {
//        let interval: Interval<Natural<u8>> = Interval::range(Natural::zero(), Natural::one());
//        let mut ref_value: Natural<u8> = Natural::zero();
//        for i in interval
//        {
//            assert_eq!(Natural::zero(), i);
//        }
//    }
}
