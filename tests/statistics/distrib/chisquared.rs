#[cfg(test)]
mod chisquareddistrib
{
    use mathru::statistics::distrib::{Continuous, ChiSquared};

    #[test]
    fn pdf0()
    {
        let df: u32 = 4;
        let distrib: ChiSquared<f64> = ChiSquared::new(df);
        let x : f64 = 0.0;
        let prob : f64 = distrib.pdf(x);

        assert_eq!(0.0, prob);
    }

    #[test]
    fn pdf1()
    {
        let df: u32 = 4;
        let distrib : ChiSquared<f64> = ChiSquared::new(df);
        let x : f64 = 1.0;
        let prob : f64 = distrib.pdf(x);

        assert_eq!(0.1516326649281583, prob);
    }

    #[test]
    fn cdf0()
    {
        let df: u32 = 4;
        let distrib: ChiSquared<f64> = ChiSquared::new(df);

        assert_eq!(0.09020401043104986, distrib.cdf(1.0))
    }

    #[test]
    fn cdf1()
    {
        let df: u32 = 4;
        let distrib: ChiSquared<f64> = ChiSquared::new(df);

        assert_eq!(0.001209104274250028, distrib.cdf(0.1))
    }

    #[test]
    fn cdf2()
    {
        let df: u32 = 4;
        let distrib: ChiSquared<f64> = ChiSquared::new(df);

        assert_eq!(0.7689217620241717, distrib.cdf(5.6))
    }

    //Very inaccurate
    #[test]
    fn cdf3()
    {
        let df: u32 = 3;

        let distrib: ChiSquared<f64> = ChiSquared::new(df);

        assert_eq!(0.0811279995005158, distrib.cdf(0.5))
    }

    //Very inaccurate
    #[test]
    fn cdf4()
    {
        let df: u32 = 3;

        let distrib: ChiSquared<f64> = ChiSquared::new(df);

        assert_eq!(0.9541107096801389, distrib.cdf(8.0))
    }

    #[test]
    fn quantile()
    {
        let df: u32 = 4;

        let distrib: ChiSquared<f64> = ChiSquared::new(df);

        assert_eq!(7.711853942311415, distrib.quantile(0.9))
    }
}