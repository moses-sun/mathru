#[cfg(test)]
mod raisedcosine
{
    use mathru;
    use mathru::statistics::distrib::{Continuous, RaisedCosine};
    use std::f64::consts::PI;

    #[test]
    fn pdf0()
    {
        let mu: f64 = 0.0;
        let s: f64 = PI;
        let distrib : RaisedCosine<f64> = RaisedCosine::new(mu, s);
        let x: f64 = PI;
        let prob : f64 = distrib.pdf(x);

        assert_eq!(0.0, prob);
    }

    #[test]
    fn pdf1()
    {
        let mu: f64 = 0.0;
        let s: f64 = PI;
        let distrib : RaisedCosine<f64> = RaisedCosine::new(mu, s);
        let x: f64 = 0.0;
        let prob : f64 = distrib.pdf(x);

        assert_eq!(1.0 / PI, prob);
    }

    #[test]
    fn pdf2()
    {
        let mu: f64 = PI;
        let s: f64 = 1.0;
        let distrib : RaisedCosine<f64> = RaisedCosine::new(mu, s);
        let x: f64 = PI;
        let prob : f64 = distrib.pdf(x);

        assert_eq!(1.0, prob);
    }

    #[test]
    fn cdf0()
    {
        let mu: f64 = PI;
        let s: f64 = 1.0;
        let distrib : RaisedCosine<f64> = RaisedCosine::new(mu, s);
        let x: f64 = PI;
        let prob: f64 = distrib.cdf(x);

        assert_eq!(0.5, prob);
    }

    #[test]
    fn cdf1()
    {
        let mu: f64 = PI;
        let s: f64 = 0.1;
        let distrib : RaisedCosine<f64> = RaisedCosine::new(mu, s);
        let x: f64 = mu - s;
        let prob: f64 = distrib.cdf(x);

        assert!((prob - 0.0).abs() < 0.000000000001);
    }

    #[test]
    fn cdf2()
    {
        let mu: f64 = PI;
        let s: f64 = 0.1;
        let distrib: RaisedCosine<f64> = RaisedCosine::new(mu, s);
        let x: f64 = PI + s;
        let prob: f64 = distrib.cdf(x);

        assert_eq!(1.0, prob);
    }

     #[test]
    fn mean()
    {
        let mu: f64 = PI;
        let s: f64 = 2.0 * PI;
        let distrib: RaisedCosine<f64> = RaisedCosine::new(mu, s);
        let mean: f64 = distrib.mean();

        assert_eq!(mu, mean);
    }

     #[test]
    fn variance()
    {
        let mu: f64 = 0.0;
        let s: f64 = PI;
        let distrib: RaisedCosine<f64> = RaisedCosine::new(mu, s);
        let variance: f64 = distrib.variance();

        assert_eq!(s * s * (1.0 / 3.0  - 2.0 / PI / PI), variance);
    }
}