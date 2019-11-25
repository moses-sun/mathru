use crate::statistics::distrib::Discrete;

/// Bernoulli distribution
///
/// Fore more information:
///
/// <a href="https://en.wikipedia.org/wiki/Bernoulli_distribution">https://en.wikipedia.org/wiki/Bernoulli_distribution</a>
pub struct Bernoulli
{
    p: f64
}

impl Bernoulli
{
    /// Create a probability distribution with p(X=1) = p
    ///
    /// # Arguments
    ///
    /// * `p` Probability that random varibale X=1, 0.0 <= p <= 1.0
    ///
    /// # Panics
    ///
    /// if p < 0 || p > 1.0
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::Bernoulli;
    ///
    /// let distrib: Bernoulli = Bernoulli::new(0.2);
    /// ```
    pub fn new(p: f64) -> Bernoulli
    {
        if p < 0.0 || p > 1.0
        {
            panic!()
        }
        Bernoulli
        {
            p: p
        }
    }
}

impl Discrete<u8, f64> for Bernoulli
{
    /// Probability mass function of the Bernoulli distribution
    ///
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin; {0, 1}
    ///
    /// # Panics
    ///
    /// if x &notin; {0, 1}
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Discrete, Bernoulli};
    ///
    /// let distrib: Bernoulli = Bernoulli::new(0.2);
    /// let x: u8 = 0;
    /// let p: f64 = distrib.pmf(x);
    /// ```
    fn pmf<'a>(self: &'a Self, x: u8) -> f64
    {
        if (x == 1) || (x ==  0)
        {
            if x == 0
            {
                return 1.0 - self.p
            }
            else
            {
                return self.p
            }
        }
        else
        {
            panic!()
        }
    }

    /// Cumulative distribution function of the Bernoulli distribution
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin; {0, 1}
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Discrete, Bernoulli};
    ///
    /// let distrib: Bernoulli = Bernoulli::new(0.2);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf<'a>(self: &'a Self, x: f64) -> f64
    {
        if x >= 1.0
        {
            return 1.0
        }

        if x <= 0.0
        {
            return 0.0
        }
        else
        {
            return 1.0 - self.p
        }
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
        self.p
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
        self.p * (1.0 - self.p)
    }
}
