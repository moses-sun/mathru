use crate::statistics::distrib::{Distribution, Continuous};
use rand::{Rng};
use rand::rngs::ThreadRng;

/// Uniform distribution
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)">https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)</a>
///
pub struct Uniform
{
	a: f64,
	b: f64
}

impl Uniform
{
    ///
    /// # Arguments
    ///
    /// a: lower bound
    /// b: upper bound
    ///
    /// a < b
    ///
    /// # Panic
    ///
    /// a >= b
    ///
    pub fn new(a: f64, b: f64) -> Uniform
    {
        if a >= b
        {
            panic!();
        }

        Uniform
        {
            a,
            b
        }
    }
}

impl Distribution for Uniform
{
    fn random(self: &Self) -> f64
    {
        let mut rng: ThreadRng = rand::thread_rng();
        return rng.gen_range(self.a, self.b);
    }
}

impl Continuous<f64, f64> for Uniform
{

    /// Probability density function
    ///
    /// # Arguments
    ///
    /// x: random variable
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Uniform};
    ///
    /// let distrib: Uniform = Uniform::new(-0.1, 0.3);
    /// let x: f64 = 5.0;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf<'a>(self: &'a Self, x: f64) -> f64
    {
        if self.a <= x && x <= self.b
        {
            return 1.0 / (self.b - self.a);
        }
        else
        {
            return 0.0;
        }
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
    /// use mathru::statistics::distrib::{Continuous, Uniform};
    ///
    /// let distrib: Uniform = Uniform::new(0.0, 0.5);
    /// let x: f64 = 0.3;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf<'a>(self: &'a Self, x: f64) -> f64
    {
        if x < self.a
        {
            return 0.0;
        }
        else
        {
            if x > self.b
            {
                return 1.0;
            }
            else
            {
                return (x - self.a) / (self.b - self.a);
            }
        }
    }


    /// Quantile function of inverse cdf
    ///
    fn quantile<'a, 'b>(self: &'a Self, _p: f64) -> f64
    {
        unimplemented!();
    }

	fn mean<'a>(self: &'a Self) -> f64
    {
        return (self.a +  self.b) / 2.0;
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Uniform};
    ///
    /// let distrib: Uniform = Uniform::new(0.2, 0.5);
    /// let var: f64 = distrib.variance();
    /// ```
	fn variance<'a>(self: &'a Self) -> f64
    {
        return (self.b - self.a) * (self.b - self.a) / 12.0;
    }
}