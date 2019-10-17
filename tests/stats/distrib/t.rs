#[cfg(test)]
mod tdistrib
{
	use mathru::stats::distrib::T;
	use mathru::stats::distrib::Continuous;

    #[test]
	fn pdf0()
    {
        let n: f64 = 2.0;
		let x: f64 = 0.0;

        let t: T = T::new(n);

        assert_eq!(0.3535533905932741, t.pdf(x));
    }

	#[test]
	fn pdf1()
    {
        let n: f64 = 5.0;
		let x: f64 = -1.0;

        let t: T = T::new(n);

        assert_eq!(0.21967979735098045, t.pdf(x));
    }

    #[test]
	fn cdf0()
    {
        let n: f64 = 2.0;
		let x: f64 = 0.0;

        let t: T = T::new(n);

       	assert_eq!(0.5, t.cdf(x));
    }

	#[test]
	fn cdf1()
    {
        let n: f64 = 2.0;
		let x: f64 = -0.5;

        let t: T = T::new(n);

       	assert_eq!(0.33333333333333315, t.cdf(x));
    }

    #[test]
	fn cdf2()
    {
        let n: f64 = 7.753;
		let x: f64 = -0.7559;

        let t: T = T::new(n);

       	assert!((t.pdf(x) - 0.283).abs() < 0.001);
    }
}