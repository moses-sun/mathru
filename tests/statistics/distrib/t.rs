#[cfg(test)]
mod tdistrib
{
	use mathru::statistics::distrib::{Continuous, T};

    #[test]
	fn pdf0()
    {
        let n: f64 = 2.0;
		let x: f64 = 0.0;

        let t: T<f64> = T::new(n);

        assert_eq!(0.3535533905932741, t.pdf(x));
    }

	#[test]
	fn pdf1()
    {
        let n: f64 = 5.0;
		let x: f64 = -1.0;

        let t: T<f64> = T::new(n);

        assert_eq!(0.21967979735098045, t.pdf(x));
    }

    #[test]
	fn cdf0()
    {
        let n: f64 = 2.0;
		let x: f64 = 0.0;

        let t: T<f64> = T::new(n);

       	assert_eq!(0.5, t.cdf(x));
    }

	#[test]
	fn cdf1()
    {
        let n: f64 = 2.0;
		let x: f64 = -0.5;

        let t: T<f64> = T::new(n);

       	assert_eq!(0.33333333333333315, t.cdf(x));
    }

    #[test]
	fn cdf2()
    {
        let n: f64 = 7.753;
		let x: f64 = -0.7559;

        let t: T<f64> = T::new(n);

       	assert!((t.pdf(x) - 0.283).abs() < 0.001);
    }

 	#[test]
  	#[should_panic(expected = "Skewness is not defined if degrees of freedom is smaller or equal 3")]
	fn skewness()
    {
        let n: f64 = 3.0;

        let t: T<f64> = T::new(n);

       	assert_eq!(t.skewness(), 0.0);
    }

    #[test]
	fn median()
    {
        let n: f64 = 3.0;

        let t: T<f64> = T::new(n);

       	assert_eq!(t.median(), 0.0);
    }
}