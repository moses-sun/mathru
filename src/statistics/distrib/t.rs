use crate::statistics::distrib::Continuous;
use crate::special::gamma;
use crate::special::hypergeometrical;
use crate::algebra::abstr::Real;

/// T distribution
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/T_distribution">https://en.wikipedia.org/wiki/T_distribution</a>
///
pub struct T<K>
{
    n: K,
}

impl<K> T<K>
    where K: Real
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
    /// use mathru::statistics::distrib::{Continuous, T};
    ///
    /// let distrib: T<f64> = T::new(1.2);
    /// ```
    pub fn new(n: K) -> T<K>
    {
        if n < K::zero()
        {
            panic!()
        }
        T
        {
            n: n,
        }
    }
}

impl<K> Continuous<K> for T<K>
    where K: Real
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
    /// use mathru::statistics::distrib::{Continuous, T};
    ///
    /// let distrib: T<f64> = T::new(1.2);
    /// let x: f64 = 0.5;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf(self: &Self, x: K) -> K
    {
        gamma::gamma((self.n + K::one()) / K::from_f64(2.0).unwrap()) * (K::one() + x.pow(&K::from_f64(2.0).unwrap()) / self.n).pow(&(-
        (self.n + K::one()) / K::from_f64(2.0).unwrap())) / ((self.n * K::pi()).sqrt() * gamma::gamma(self.n / K::from_f64(2.0).unwrap()))
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
    /// use mathru::statistics::distrib::{Continuous, T};
    ///
    /// let distrib: T<f64> = T::new(1.3);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf(self: &Self, x: K) -> K
    {
        let k: K = (self.n + K::one()) / K::from_f64(2.0).unwrap();
        let f21: K = hypergeometrical::f21(K::from_f64(0.5).unwrap(), k, K::from_f64(1.5).unwrap(), -(x.pow(&K::from_f64(2.0).unwrap())) /
        self.n);
        return K::from_f64(0.5).unwrap() + x * gamma::gamma(k) * f21 / ((self.n * K::pi()).sqrt() * gamma::gamma(self.n /
        K::from_f64(2.0).unwrap()))
    }


    /// Quantile function of inverse cdf
    ///
    fn quantile(self: &Self, _p: K) -> K
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
    /// use mathru::statistics::distrib::{Continuous, T};
    ///
    /// let distrib: T<f64> = T::new(1.2);
    /// let mean: f64 = distrib.mean();
    /// ```
	fn mean(self: &Self) -> K
    {
        if self.n > K::one()
        {
            return K::zero();
        }
        panic!();
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, T};
    ///
    /// let distrib: T<f64> = T::new(2.2);
    /// let var: f64 = distrib.variance();
    /// ```
	fn variance(self: &Self) -> K
    {
        if self.n > K::from_f64(2.0).unwrap()
        {
            return self.n / (self.n - K::from_f64(2.0).unwrap())
        }
        if self.n > K::one()
        {
            return K::from_f64(std::f64::INFINITY).unwrap();
        }
        else
        {
            panic!();
        }
    }
}