extern crate mathru;

#[cfg(test)]
mod beta
{
	use mathru::stats::distrib::Beta;
	use mathru::stats::distrib::Continuous;
	use std::f64;

	#[test]
	#[should_panic]
	fn pdf_lower_0()
    {
        let p: f64 = 2.0;
        let q: f64 = 5.0;
		let x: f64 = 0.0 - f64::EPSILON;

        let beta: Beta = Beta::new(&p, &q);

        beta.pdf(x);
    }

    #[test]
	fn pdf0()
    {
        let p: f64 = 2.0;
        let q: f64 = 5.0;
		let x: f64 = 0.0;

        let beta: Beta = Beta::new(&p, &q);

        assert_eq!(0.0, beta.pdf(x));
    }


	#[test]
	fn pdf1()
    {
        let p: f64 = 2.0;
        let q: f64 = 5.0;
		let x: f64 = 1.0;

        let beta: Beta = Beta::new(&p, &q);

        beta.pdf(x);
    }

    #[test]
    #[should_panic]
	fn pdf_greater_1()
    {
        let p: f64 = 2.0;
        let q: f64 = 5.0;
		let x: f64 = 1.0 + f64::EPSILON;

        let beta: Beta = Beta::new(&p, &q);

        beta.pdf(x);
    }

	#[test]
	fn pdf2()
    {
        let p: f64 = 2.0;
        let q: f64 = 5.0;
		let x: f64 = 0.4;

        let beta: Beta = Beta::new(&p, &q);

        assert_eq!(1.5552000000000026, beta.pdf(x));
    }

    #[test]
	#[should_panic]
	fn cdf_lower_0()
    {
        let p: f64 = 2.0;
        let q: f64 = 5.0;
		let x: f64 = 0.0 - f64::EPSILON;

        let beta: Beta = Beta::new(&p, &q);

       	beta.cdf(x);
    }

    #[test]
	fn cdf0()
    {
        let p: f64 = 2.0;
        let q: f64 = 5.0;
		let x: f64 = 0.0;

        let beta: Beta = Beta::new(&p, &q);

       	assert_eq!(0.0, beta.cdf(x));
    }

	#[test]
	fn cdf1()
    {
        let p: f64 = 2.0;
        let q: f64 = 5.0;
		let x: f64 = 1.0;

        let beta: Beta = Beta::new(&p, &q);

        assert_eq!(1.0, beta.cdf(x));
    }

   	#[test]
   	#[should_panic]
	fn cdf_greater_1()
    {
        let p: f64 = 2.0;
        let q: f64 = 5.0;
		let x: f64 = 1.0 + f64::EPSILON;

        let beta: Beta = Beta::new(&p, &q);

        beta.cdf(x);
    }

    #[test]
	fn cdf2()
    {
        let p: f64 = 2.0_f64;
        let q: f64 = 5.0_f64;
		let x: f64 = 0.3_f64;

        let beta: Beta = Beta::new(&p, &q);

        assert_eq!(0.5798249999999994_f64, beta.cdf(x));
    }
}