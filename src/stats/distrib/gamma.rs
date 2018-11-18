use stats::distrib::Continuous;
use special::gamma;

/// Gamma distribution
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Gamma_distribution">https://en.wikipedia.org/wiki/Gamma_distribution</a>
///
pub struct Gamma
{
    p: f64,
    b: f64,
}



impl Gamma
{
    /// Creates a probability distribution
    ///
    /// # Arguments
    ///
    /// * `p` (alpha) p > 0.0
    /// * `b` (beta)  b > 0.0
    ///
    /// # Panics
    ///
    /// if p <= 0.0 || b <= 0.0
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::Gamma;
    ///
    /// let distrib: Gamma = Gamma::new(&0.3, &0.2);
    /// ```
    pub fn new(p: &f64, b: &f64) -> Gamma
    {
        if *p <= 0.0_f64 || *b <= 0.0_f64
        {
            panic!()
        }
        Gamma
        {
            p: *p,
            b: *b
        }
    }
}

impl Continuous<f64, f64> for Gamma
{

    /// Probability density function
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin; &#x2115 | x > 0.0
    ///
    /// # Panics
    ///
    /// if x <= 0.0
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Continuous, Gamma};
    ///
    /// let distrib: Gamma = Gamma::new(&0.3, &0.2);
    /// let x: f64 = 5.0;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf<'a>(self: &'a Self, x: f64) -> f64
    {
        if x <= 0.0_f64
        {
            panic!();
        }
        self.b.powf(self.p) / gamma::gamma(self.p) * x.powf(self.p - 1.0_f64) * (-self.b * x).exp()
    }

    /// Cumulative distribution function
    ///
    /// # Arguments
    ///
    /// * `x` Random variable
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Continuous, Gamma};
    ///
    /// let distrib: Gamma = Gamma::new(&0.3, &0.2);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf<'a>(self: &'a Self, x: f64) -> f64
    {
        if x == 0.0_f64
        {
            return 0.0
        }
        gamma::gamma_lr(self.p, self.b * x)
    }


    /// Quantile function of inverse cdf
    ///
    fn quantile<'a, 'b>(self: &'a Self, _p: f64) -> f64
    {
        unimplemented!();
    }

    /// Expected value
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Discrete, Bernoulli};
    ///
    /// let distrib: Bernoulli = Bernoulli::new(0.2);
    /// let mean: f64 = distrib.mean();
    /// ```
	fn mean<'a>(self: &'a Self) -> f64
    {
        unimplemented!();
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Discrete, Bernoulli};
    ///
    /// let distrib: Bernoulli = Bernoulli::new(0.2);
    /// let var: f64 = distrib.variance();
    /// ```
	fn variance<'a>(self: &'a Self) -> f64
    {
        self.p / self.b.powi(2)
    }
}