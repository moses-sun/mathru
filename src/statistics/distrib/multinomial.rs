use crate::statistics::distrib::Discrete;
use crate::statistics::combins;
use crate::algebra::linear::Vector;

/// Multinomial distribution
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Multinomial_distribution">https://en.wikipedia.org/wiki/Multinomial_distribution</a>
///
pub struct Multinomial
{
    p: Vector<f64>,
}

impl Multinomial
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
    /// use mathru::*;
    /// use mathru::algebra::linear::Vector;
    /// use mathru::statistics::distrib::Multinomial;
    ///
    /// let distrib: Multinomial = Multinomial::new(vector![0.3; 0.2; 0.5]);
    /// ```
	 pub fn new(p: Vector<f64>) -> Multinomial
    {
        Multinomial
        {
            p: p
        }
    }
}

impl Discrete<Vector<u32>, Vector<f64>> for Multinomial
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
    /// use mathru::*;
    /// use mathru::statistics::distrib::{Discrete, Multinomial};
    /// use mathru::algebra::linear::Vector;
    ///
    /// let p: Vector<f64> = vector![0.3; 0.7];
    /// let distrib: Multinomial = Multinomial::new(p);
    /// let x: Vector<u32> = vector![1; 2];
    /// let p: f64 = distrib.pmf(x);
    /// ```
	fn pmf<'a>(self: &'a Self, x: Vector<u32>) -> f64
    {
        assert_eq!(self.p.dim(), x.dim());
        let (m, _n) = x.dim();

        let mut prod: f64 = 1.0;
        let mut n: u32 = 0;
        for k in 0..m
        {
            let p_k: f64 = *self.p.get(k);
            let n_k: u32 = *x.get(k);
            n = n + n_k;
            prod = prod * p_k.powf(n_k as f64) / (combins::factorial(n_k) as f64);
        }
        return prod * (combins::factorial(n) as f64);
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
    /// let distrib: Binomial = Binomial::new(&5, &0.3);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
	fn cdf<'a>(self: &'a Self, _x: Vector<f64>) -> f64
    {
    /*
        let x_supremum : u32 = x.floor() as u32;
        let mut prob : f64 = 0.0;

        for k in 0..x_supremum + 1
        {
            prob += self.pmf(k);
        }
        prob
        */
        unimplemented!();
    }

    /// Expected value
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Discrete, Binomial};
    ///
    /// let distrib: Binomial = Binomial::new(&5, &0.3);
    /// let mean: f64 = distrib.mean();
    /// ```
	fn mean<'a>(self: &'a Self) -> f64
    {
        unimplemented!();
        //return &(self.n as f64) * &self.p
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Discrete, Binomial};
    ///
    /// let distrib: Binomial = Binomial::new(&5, &0.3);
    /// let var: f64 = distrib.variance();
    /// ```
	fn variance<'a>(self: &'a Self) -> f64
    {
        unimplemented!();
        //return self.mean() * (1.0 - self.p)
    }
}