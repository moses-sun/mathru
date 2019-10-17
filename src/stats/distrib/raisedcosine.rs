use crate::stats::distrib::Continuous;
use std::f64::consts::PI;
use std::f64;

/// Raised Cosine distribution
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Raised_cosine_distribution">https://en.wikipedia.org/wiki/Raised_cosine_distribution</a>
///
pub struct RaisedCosine
{
    mu: f64,
    s: f64
}



impl RaisedCosine
{
    /// Creates a probability distribution
    ///
    /// # Arguments
    ///
    /// * `mu`
    /// * `s` > 0.0
    ///
    /// # Panics
    ///
    /// if s < 0.0
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::stats::distrib::RaisedCosine;
    /// use std::f64::consts::PI;
    ///
    /// let mu: f64 = PI;
    /// let s: f64 = 0.5 * PI;
    /// let distrib: RaisedCosine = RaisedCosine::new(mu, s);
    /// ```
    pub fn new(mu: f64, s: f64) -> RaisedCosine
    {
        if s < 0.0
        {
            panic!();
        }
        RaisedCosine
        {
            mu: mu,
            s: s
        }
    }
}

impl Continuous<f64, f64> for RaisedCosine
{

    /// Probability density function
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x
    ///
    /// # Panics
    ///
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::stats::distrib::{Continuous, RaisedCosine};
    ///
    /// let distrib: RaisedCosine = RaisedCosine::new(-1.2, 1.5);
    /// let x: f64 = 5.0;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf(self: & Self, x: f64) -> f64
    {
        if (self.mu - self.s) <= x && x < (self.mu + self.s)
        {
            return (1.0 + (PI*(x - self.mu)/self.s).cos()) / (2.0 * self.s)
        }

        return 0.0;
    }

    /// Cumulative distribution function
    ///
    /// # Arguments
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::stats::distrib::{Continuous, RaisedCosine};
    /// use std::f64::consts::PI;
    ///
    /// let distrib: RaisedCosine = RaisedCosine::new(1.0, PI);
    /// let x: f64 = PI/2.0;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf(self: &Self, x: f64) -> f64
    {
        if (self.mu - self.s) <= x && x <= (self.mu + self.s)
        {
            let k: f64 = (x - self.mu) / self.s;
            return (1.0 + k + 1.0 / PI * (k * PI).sin()) / 2.0;
        }
        else
        {
            if x < (self.mu - self.s)
            {
                return 0.0;
            }
            else
            {
                return 1.0
            }
        }
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
    /// use mathru::stats::distrib::{Continuous, RaisedCosine};
    ///
    /// let distrib: RaisedCosine = RaisedCosine::new(-2.0, 0.5);
    /// let mean: f64 = distrib.mean();
    /// ```
	fn mean<'a>(self: &'a Self) -> f64
    {
        return self.mu
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::stats::distrib::{Continuous, RaisedCosine};
    /// use std::f64::consts::PI;
    ///
    /// let distrib: RaisedCosine = RaisedCosine::new(2.0, PI);
    /// let var: f64 = distrib.variance();
    /// ```
	fn variance(self: & Self) -> f64
    {
        return self.s * self.s * (1.0 / 3.0 - 2.0 / (PI * PI));
    }
}