use stats::distrib::Continuous;
use special::gamma;
use special::hypergeometrical;

/// T distribution
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/T_distribution">https://en.wikipedia.org/wiki/T_distribution</a>
///
pub struct T
{
    n: f64,
}

impl T
{

    /// Create a probability distribution
    ///
    /// # Arguments
    ///
    /// * `n`: > 0.0
    ///
    /// # Panics
    ///
    /// if n <= 0.0
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Continuous, T};
    ///
    /// let distrib: T = T::new(&1.2);
    /// ```
    pub fn new(n: &f64) -> T
    {
        if *n < 0.0_f64
        {
            panic!()
        }
        T
        {
            n: *n,
        }
    }
}

impl Continuous<f64, f64> for T
{
    /// Probability density function
    ///
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin &#2115;
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Continuous, T};
    ///
    /// let distrib: T = T::new(&1.2);
    /// let x: f64 = 0.5;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf<'a>(self: &'a Self, x: f64) -> f64
    {
        gamma::gamma((self.n + 1.0) / 2.0) * (1.0 + x.powi(2) / self.n).powf(-(self.n + 1.0) / 2.0) / ((self.n *
        std::f64::consts::PI).sqrt() * gamma::gamma(self.n / 2.0))
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
    /// use mathru::stats::distrib::{Continuous, T};
    ///
    /// let distrib: T = T::new(&1.3);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf<'a>(self: &'a Self, x: f64) -> f64
    {
        let k: f64 = (self.n + 1.0) / 2.0;
        let f21: f64 = hypergeometrical::f21(0.5, k, 1.5, -(x.powi(2)) / self.n);
        0.5 + x * gamma::gamma(k) * f21 / ((self.n *
        std::f64::consts::PI).sqrt() * gamma::gamma(self.n / 2.0))
    }


    /// Quantile function of inverse cdf
    ///
    fn quantile<'a, 'b>(self: &'a Self, _p: f64) -> f64
    {
        unimplemented!();
    }

    /// Expected value
    ///
    /// # Panics
    ///
    /// if self.n <= 1.0
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Continuous, T};
    ///
    /// let distrib: T = T::new(&1.2);
    /// let mean: f64 = distrib.mean();
    /// ```
	fn mean<'a>(self: &'a Self) -> f64
    {
        if self.n > 1.0
        {
            return 0.0;
        }
        panic!();
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Continuous, T};
    ///
    /// let distrib: T = T::new(&2.2);
    /// let var: f64 = distrib.variance();
    /// ```
	fn variance<'a>(self: &'a Self) -> f64
    {
        if self.n > 2.0
        {
            return self.n / (self.n - 2.0)
        }
        if self.n > 1.0
        {
            return std::f64::INFINITY;
        }
        else
        {
            panic!();
        }
    }
}