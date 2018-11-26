extern crate mathru;

#[cfg(test)]
mod beta_test
{
	use mathru::special::beta;

	#[test]
	fn beta0()
    {
        let x: f64 = 5.0;
        let y: f64 = 6.5;

        let beta: f64 = beta::beta(x, y);

        assert_eq!(0.0005806371131448529, beta);
    }

  	#[test]
	fn beta_inc0()
    {
        let x: f64 = 0.0;
        let p: f64 = 0.5;
        let q: f64 = 0.3;

        let beta: f64 = beta::beta_inc_reg(x, p, q);

        assert_eq!(0.0, beta);
    }

   	#[test]
	fn beta_inc1()
    {
        let x: f64 = 1.0;
        let p: f64 = 0.5;
        let q: f64 = 0.3;

        let beta: f64 = beta::beta_inc_reg(x, p, q);

        assert_eq!(1.0, beta);
    }

    #[test]
	fn beta_inc2()
    {
        let x: f64 = 0.7;
        let p: f64 = 0.5;
        let q: f64 = 0.3;

        let beta: f64 = beta::beta_inc_reg(x, p, q);

        assert_eq!(0.4695967039528893, beta);
    }
}