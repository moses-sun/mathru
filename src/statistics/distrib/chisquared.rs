use crate::statistics::distrib::Continuous;
use crate::special::gamma;
use crate::special;
use std::f64;
use super::Normal;


/// Chi-Squared distribution
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Chi-squared_distribution">https://en.wikipedia.org/wiki/Chi-squared_distribution</a>
///
pub struct ChiSquared
{
    k: f64 //degree of freedom
}

impl ChiSquared
{
    /// Creates a probability distribution
    ///
    /// # Arguments
    ///
    /// * `df`: Degree of freedom, df >= 1
    ///
    /// # Panics
    ///
    /// if df < 1
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::ChiSquared;
    ///
    /// let distrib: ChiSquared = ChiSquared::new(&3);
    /// ```
    pub fn new(df: &u32) -> ChiSquared
    {
        if *df < 1
        {
            panic!()
        }
        ChiSquared
        {
            k: *df as f64
        }
    }
}


impl Continuous<f64, f64> for ChiSquared
{

    /// Probability density function
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin; &#x2115
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, ChiSquared};
    ///
    /// let distrib: ChiSquared = ChiSquared::new(&2);
    /// let x: f64 = 5.0;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf<'a>(self: &'a Self, x: f64) -> f64
    {
        if x < 0.0
        {
            return 0.0
        }
        let t1: f64 = 1.0 / ((2.0_f64).powf(self.k  / 2.0) * gamma::gamma(self.k / 2.0));
        let t2: f64 = x.powf(self.k / 2.0 - 1.0) * (-x / 2.0).exp();
        let chisquared: f64 = t1 * t2;

        chisquared
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
    /// use mathru::statistics::distrib::{Continuous, ChiSquared};
    ///
    /// let distrib: ChiSquared = ChiSquared::new(&3);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf<'a>(self: &'a Self, x: f64) -> f64
    {
        let t1: f64 = (-x / 2.0).exp();

        let k_natural: u32 = self.k as u32;
        let p: f64;

        if k_natural % 2 == 0
        {
            let mut sum: f64 = 0.0;
            for i in 0..(self.k / 2.0) as u32
            {
                sum += (x / 2.0).powf(i as f64) / gamma::gamma((i + 1) as f64)
            }

            p = 1.0 - t1 * sum;
        }
        else
        {
            let mut sum: f64 = 0.0;
            for i in 0..(self.k / 2.0) as u32
            {
                sum += (x / 2.0).powf((i as f64) + 0.5) / gamma::gamma((i as f64) + 1.5)
            }

            p = special::erf((x / 2.0).sqrt()) - t1 * sum;
        }

        p
    }

    /// Quantile function of inverse cdf
    ///
    fn quantile<'a, 'b>(self: &'a Self, p: f64) -> f64
    {
        let std_distrib: Normal = Normal::new(0.0, 1.0);
        let q: f64 = 0.5 * (std_distrib.quantile(p) + (2.0 * self.k as f64 - 1.0).sqrt()).powi(2);
        q
    }

    /// Expected value
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Discrete, Bernoulli};
    ///
    /// let distrib: Bernoulli = Bernoulli::new(0.2);
    /// let mean: f64 = distrib.mean();
    /// ```
	fn mean<'a>(self: &'a Self) -> f64
    {
        return self.k as f64
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Discrete, Bernoulli};
    ///
    /// let distrib: Bernoulli = Bernoulli::new(0.2);
    /// let var: f64 = distrib.variance();
    /// ```
	fn variance<'a>(self: &'a Self) -> f64
    {
        return 2.0 * (self.k as f64)
    }
}