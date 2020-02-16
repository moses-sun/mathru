use crate::statistics::distrib::{Distribution, Continuous};
use rand::{Rng};
use rand::rngs::ThreadRng;
use crate::algebra::abstr::Real;

/// Uniform distribution
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)">https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)</a>
///
pub struct Uniform<T>
{
	a: T,
	b: T
}

impl<T> Uniform<T>
    where T: Real
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
    pub fn new(a: T, b: T) -> Uniform<T>
    {
        if a >= b
        {
            panic!();
        }

        return
        Uniform
        {
            a,
            b
        }
        ;
    }
}

impl<T> Distribution<T> for Uniform<T>
    where T: Real
{
    fn random(self: &Self) -> T
    {
        let mut rng: ThreadRng = rand::thread_rng();
        return T::from_f64(rng.gen_range(self.a.to_f64().unwrap(), self.b.to_f64().unwrap())).unwrap();
    }
}

impl<T> Continuous<T, T, T> for Uniform<T>
    where T: Real
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
    /// let distrib: Uniform<f64> = Uniform::new(-0.1, 0.3);
    /// let x: f64 = 5.0;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf<'a>(self: &'a Self, x: T) -> T
    {
        if self.a <= x && x <= self.b
        {
            return T::one() / (self.b - self.a);
        }
        else
        {
            return T::zero();
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
    /// let distrib: Uniform<f64> = Uniform::new(0.0, 0.5);
    /// let x: f64 = 0.3;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf<'a>(self: &'a Self, x: T) -> T
    {
        if x < self.a
        {
            return T::zero();
        }
        else
        {
            if x > self.b
            {
                return T::one();
            }
            else
            {
                return (x - self.a) / (self.b - self.a);
            }
        }
    }


    /// Quantile function of inverse cdf
    ///
    fn quantile<'a, 'b>(self: &'a Self, _p: T) -> T
    {
        unimplemented!();
    }

	fn mean<'a>(self: &'a Self) -> T
    {
        return (self.a +  self.b) / T::from_f64(2.0).unwrap();
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Uniform};
    ///
    /// let distrib: Uniform<f64> = Uniform::new(0.2, 0.5);
    /// let var: f64 = distrib.variance();
    /// ```
	fn variance<'a>(self: &'a Self) -> T
    {
        return (self.b - self.a) * (self.b - self.a) / T::from_f64(12.0).unwrap();
    }
}