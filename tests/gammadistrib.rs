extern crate mathru;

#[cfg(test)]
mod gammadistrib
{
	use mathru::stats::distrib::Gamma;
	use mathru::stats::distrib::Continuous;
	use std::f64;

	#[test]
	#[should_panic]
	fn pdf_lower_0()
    {
        let p: f64 = 2.0;
        let b: f64 = 5.0;
		let x: f64 = 0.0 - f64::EPSILON;

        let beta: Gamma = Gamma::new(&p, &b);

        beta.pdf(x);
    }

    #[test]
    #[should_panic]
	fn pdf0()
    {
        let p: f64 = 2.0;
        let b: f64 = 2.0;
		let x: f64 = 0.0;

        let beta: Gamma = Gamma::new(&p, &b);

        beta.pdf(x);
    }


	#[test]
	fn pdf1()
    {
        let p: f64 = 2.0;
        let b: f64 = 2.0;
		let x: f64 = 1.0;

        let beta: Gamma = Gamma::new(&p, &b);

        beta.pdf(x);
    }

	#[test]
	fn pdf2()
    {
        let p: f64 = 2.0;
        let b: f64 = 2.0;
		let x: f64 = 0.4;

        let beta: Gamma = Gamma::new(&p, &b);

        assert_eq!(0.7189263425875543, beta.pdf(x));
    }

    #[test]
	#[should_panic]
	fn cdf_lower_0()
    {
        let p: f64 = 2.0;
        let q: f64 = 5.0;
		let x: f64 = 0.0 - f64::EPSILON;

        let beta: Gamma = Gamma::new(&p, &q);

       	beta.cdf(x);
    }

    #[test]
	fn cdf0()
    {
        let p: f64 = 2.0;
        let q: f64 = 5.0;
		let x: f64 = 0.0;

        let beta: Gamma = Gamma::new(&p, &q);

       	assert_eq!(0.0_f64, beta.cdf(x));
    }

	#[test]
	fn cdf1()
    {
        let p: f64 = 2.0;
        let q: f64 = 5.0;
		let x: f64 = 1.0;

        let beta: Gamma = Gamma::new(&p, &q);

        assert_eq!(0.9595723180054873_f64, beta.cdf(x));
    }

    #[test]
	fn cdf2()
    {
        let p: f64 = 2.0_f64;
        let q: f64 = 5.0_f64;
		let x: f64 = 0.3_f64;

        let beta: Gamma = Gamma::new(&p, &q);

        assert_eq!(0.44217459962892536_f64, beta.cdf(x));
    }
}