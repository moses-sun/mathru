#[cfg(test)]
mod uniformdistrib
{
	use mathru::statistics::distrib::Uniform;
	use mathru::statistics::distrib::Continuous;

	#[test]
	fn pdf_lower_a()
    {
    	let a: f64 = -0.2;
        let b: f64 = 0.4;
		let x: f64 = -0.3;

        let distrib: Uniform = Uniform::new(a, b);

        assert_eq!(0.0, distrib.pdf(x));
    }

    #[test]
	fn pdf_higher_b()
    {
    	let a: f64 = -0.2;
        let b: f64 = 0.4;
		let x: f64 = 0.5;

        let distrib: Uniform = Uniform::new(a, b);

        assert_eq!(0.0, distrib.pdf(x));
    }

    #[test]
	fn pdf()
    {
    	let a: f64 = -0.2;
        let b: f64 = 0.4;
		let x: f64 = 0.3;

        let distrib: Uniform = Uniform::new(a, b);

        assert_eq!(1.6666666666666665, distrib.pdf(x));
    }

   	#[test]
	fn cdf_lower_a()
    {
    	let a: f64 = -0.2;
        let b: f64 = 0.4;
		let x: f64 = -0.3;

        let distrib: Uniform = Uniform::new(a, b);

        assert_eq!(0.0, distrib.cdf(x));
    }

    #[test]
	fn cdf_higher_b()
    {
    	let a: f64 = -0.2;
        let b: f64 = 0.4;
		let x: f64 = 0.5;

        let distrib: Uniform = Uniform::new(a, b);

        assert_eq!(1.0, distrib.cdf(x));
    }

    #[test]
	fn cdf()
    {
    	let a: f64 = -0.2;
        let b: f64 = 0.4;
		let x: f64 = 0.3;

        let distrib: Uniform = Uniform::new(a, b);

        assert_eq!(0.8333333333333333, distrib.cdf(x));
    }

	#[test]
	fn mean()
    {
    	let a: f64 = -0.2;
        let b: f64 = 0.4;

        let distrib: Uniform = Uniform::new(a, b);

        assert_eq!((a + b) / 2.0, distrib.mean());
    }

    #[test]
	fn variance()
    {
    	let a: f64 = -0.2;
        let b: f64 = 0.4;
		let diff: f64 = b - a;

        let distrib: Uniform = Uniform::new(a, b);

        assert_eq!(diff * diff / 12.0, distrib.variance());
    }

}