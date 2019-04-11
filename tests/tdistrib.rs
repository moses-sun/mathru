extern crate mathru;

#[cfg(test)]
mod tdistrib
{
	use mathru::stats::distrib::T;
	use mathru::stats::distrib::Continuous;
	use std::f64;


    #[test]
	fn pdf0()
    {
        let n: f64 = 2.0;
		let x: f64 = 0.0;

        let t: T = T::new(&n);

        assert_eq!(0.3535533905932741, t.pdf(x));
    }


	#[test]
	fn pdf1()
    {
        let n: f64 = 5.0;
		let x: f64 = -1.0;

        let t: T = T::new(&n);

        assert_eq!(0.21967979735098045, t.pdf(x));
    }

    #[test]
	fn cdf0()
    {
        let n: f64 = 2.0;
		let x: f64 = 0.0;

        let t: T = T::new(&n);

       	assert_eq!(0.5, t.cdf(x));
    }

	#[test]
	fn cdf1()
    {
        let n: f64 = 2.0;
		let x: f64 = -0.5;

        let t: T = T::new(&n);

       	assert_eq!(0.33333333333333315, t.cdf(x));
    }

//    #[test]
//	fn cdf2()
//    {
//        let n: f64 = 2.0;
//		let x: f64 = 2.0;
//
//        let t: T = T::new(&n);
//
//       	assert_eq!(0.6666666824730494, t.cdf(x));
//    }

}