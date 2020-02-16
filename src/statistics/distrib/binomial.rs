use crate::statistics::distrib::Discrete;
use crate::statistics::combins;
use crate::algebra::abstr::Real;

/// Binomial distribution
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Binomial_distribution">https://en.wikipedia.org/wiki/Binomial_distribution</a>
///
pub struct Binomial<T>
{
    p: T,
    n: u32
}

impl<T> Binomial<T>
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
    /// use mathru::statistics::distrib::Binomial;
    ///
    /// let distrib: Binomial<f64> = Binomial::new(5, 0.3);
    /// ```
    pub fn new(n: u32, p: T) -> Binomial<T>
    {
        Binomial
        {
            p: p,
            n: n
        }
    }
}

impl<T> Discrete<T, u32, T> for Binomial<T>
    where T: Real
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
    /// use mathru::statistics::distrib::{Discrete, Binomial};
    ///
    /// let distrib: Binomial<f64> = Binomial::new(5, 0.3);
    /// let x: u32 = 0;
    /// let p: f64 = distrib.pmf(x);
    /// ```
    fn pmf<'a>(self: &'a Self, x: u32) -> T
    {
        if x > self.n
        {
            return T::zero();
        }
        let f: T = T::from_u32(combins::binom(self.n, x)).unwrap();
        let diff: i32 = (self.n as i32) - (x as i32);
        let pdf: T = f * (self.p.pow(&T::from_u32(x).unwrap())) * ((T::one() - self.p).pow(&T::from_i32(diff).unwrap()));
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
    /// use mathru::statistics::distrib::{Discrete, Binomial};
    ///
    /// let distrib: Binomial<f64> = Binomial::new(5, 0.3);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf<'a>(self: &'a Self, x: T) -> T
    {
        let x_supremum: u32 = x.floor().to_u32().unwrap();
        let mut prob: T = T::zero();

        for k in 0..x_supremum + 1
        {
            prob += self.pmf(k);
        }
        return prob;
    }

    /// Expected value
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Discrete, Binomial};
    ///
    /// let distrib: Binomial<f64> = Binomial::new(5, 0.3);
    /// let mean: f64 = distrib.mean();
    /// ```
	fn mean<'a>(self: &'a Self) -> T
    {
        return T::from_u32(self.n).unwrap() * self.p
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Discrete, Binomial};
    ///
    /// let distrib: Binomial<f64> = Binomial::new(5, 0.3);
    /// let var: f64 = distrib.variance();
    /// ```
	fn variance<'a>(self: &'a Self) -> T
    {
        return self.mean() * (T::one() - self.p)
    }
}
