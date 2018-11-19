use stats::distrib::Discrete;
use stats::combins;

/// Binomial distribution
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Binomial_distribution">https://en.wikipedia.org/wiki/Binomial_distribution</a>
///
pub struct Binomial
{
    p: f64,
    n: u32
}

impl Binomial
{
     /// Create a probability distribution with
    ///
    /// # Arguments
    ///
    /// * `p` Probability that random variable, p &isin; [0, 1]
    /// * `n` number of trials, n &isin; &#x2115;
    ///
    /// # Panics
    ///
    /// if p < 0 || p > 1.0
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::Binomial;
    ///
    /// let dsitrib: Binomial = Binomial::new(&5, &0.3);
    /// ```
    pub fn new(n: &u32, p: &f64) -> Binomial
    {
        Binomial
        {
            p: *p,
            n: *n
        }
    }
}

impl Discrete<u32, f64> for Binomial
{
    /// Probability mass function
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin &#2115;
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Discrete, Binomial};
    ///
    /// let distrib: Binomial = Binomial::new(&5, &0.3);
    /// let x: u32 = 0;
    /// let p: f64 = distrib.pmf(x);
    /// ```
    fn pmf<'a>(self: &'a Self, x: u32) -> f64
    {
        if x > self.n
        {
            return 0.0;
        }
        let f: f64 = combins::binom(self.n, x) as f64;
        let diff : i32 = (self.n as i32) - (x as i32);
        let pdf: f64 = f * (self.p.powi(x as i32)) * ((1.0 - self.p).powi(diff));
        pdf
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
    /// use mathru::stats::distrib::{Discrete, Binomial};
    ///
    /// let distrib: Binomial = Binomial::new(&5, &0.3);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf<'a>(self: &'a Self, x: f64) -> f64
    {
        let x_supremum : u32 = x.floor() as u32;
        let mut prob : f64 = 0.0;

        for k in 0..x_supremum + 1
        {
            prob += self.pmf(k);
        }
        prob
    }

    /// Expected value
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Discrete, Binomial};
    ///
    /// let distrib: Binomial = Binomial::new(&5, &0.3);
    /// let mean: f64 = distrib.mean();
    /// ```
	fn mean<'a>(self: &'a Self) -> f64
    {
        return &(self.n as f64) * &self.p
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Discrete, Binomial};
    ///
    /// let distrib: Binomial = Binomial::new(&5, &0.3);
    /// let var: f64 = distrib.variance();
    /// ```
	fn variance<'a>(self: &'a Self) -> f64
    {
        return self.mean() * (1.0 - self.p)
    }
}
