#[cfg(test)]
mod exponential
{
    use mathru::statistics::distrib::{Continuous, Exponential};

    #[test]
    fn pdf0()
    {
        let lambda: f64 = 1.0;
        let distrib: Exponential<f64> = Exponential::new(lambda);
        let x: f64 = 1.0;
        let prob: f64 = distrib.pdf(x);

        assert_eq!((-1.0_f64).exp(), prob);
    }

    // Does not work all the time, because the used function random is not mocked.
    //    #[test]
    //    fn random()
    //    {
    //        let lambda_1: f64 = 1.0;
    //        let distrib_1 : ExponentialDistrib =
    // ExponentialDistrib::new(&lambda_1);        let mut data: Vec<f64> =
    // Vec::new();
    //
    //        for _i in 0..10000
    //        {
    //            data.push(distrib_1.random());
    //        }
    //
    //        let distrib_2: ExponentialDistrib =
    // ExponentialDistrib::from_data(&data);
    //
    //
    //        assert!(distrib_2.mean() < 1.01);
    //        assert!(distrib_2.mean() > 0.99);
    //        //assert!(distrib_2.variance() < 1.01 * variance_1);
    //        //assert!(distrib_2.variance() > 0.99 * variance_1);
    //    }

    #[test]
    fn cdf0()
    {
        let lambda: f64 = 0.5;
        let distrib: Exponential<f64> = Exponential::new(lambda);

        assert_eq!(1.0 - (-1.0_f64).exp(), distrib.cdf(2.0))
    }

    #[test]
    fn quantile()
    {
        let lambda: f64 = 0.5;
        let distrib: Exponential<f64> = Exponential::new(lambda);

        assert_eq!(1.3862943611198906, distrib.quantile(0.5))
    }

    #[test]
    fn skewnes()
    {
        let lambda: f64 = 0.5;
        let distrib: Exponential<f64> = Exponential::new(lambda);

        assert_eq!(2.0, distrib.skewness());
    }

    #[test]
    fn median()
    {
        let lambda: f64 = 0.5;
        let distrib: Exponential<f64> = Exponential::new(lambda);

        assert_eq!((2.0f64).ln() / lambda, distrib.median());
    }

    #[test]
    fn entropy()
    {
        let lambda: f64 = 0.5;
        let distrib: Exponential<f64> = Exponential::new(lambda);

        assert_eq!(1.0 - lambda.ln(), distrib.entropy());
    }
}
