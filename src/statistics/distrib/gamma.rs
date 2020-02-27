use crate::statistics::distrib::Continuous;
use crate::special::gamma;
use crate::algebra::abstr::Real;

/// Gamma distribution
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Gamma_distribution">https://en.wikipedia.org/wiki/Gamma_distribution</a>
///
pub struct Gamma<T>
{
    p: T,
    b: T,
}



impl<T> Gamma<T>
    where T: Real
{
    /// Creates a probability distribution
    ///
    /// # Arguments
    ///
    /// * `p` (alpha) p > 0.0
    /// * `b` (beta)  b > 0.0
    ///
    /// # Panics
    ///
    /// if p <= 0.0 || b <= 0.0
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::Gamma;
    ///
    /// let distrib: Gamma<f64> = Gamma::new(0.3, 0.2);
    /// ```
    pub fn new(p: T, b: T) -> Gamma<T>
    {
        if p <= T::zero() || b <= T::zero()
        {
            panic!()
        }
        Gamma
        {
            p: p,
            b: b
        }
    }
}

impl<T> Continuous<T> for Gamma<T>
    where T: Real
{

    /// Probability density function
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin; &#x2115 | x > 0.0
    ///
    /// # Panics
    ///
    /// if x <= 0.0
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Gamma};
    ///
    /// let distrib: Gamma<f64> = Gamma::new(0.3, 0.2);
    /// let x: f64 = 5.0;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf(self: &Self, x: T) -> T
    {
        if x <= T::zero()
        {
            panic!();
        }
        self.b.pow(&self.p) / gamma::gamma(self.p) * x.pow(&(self.p - T::one())) * (-self.b * x).exp()
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
    /// use mathru::statistics::distrib::{Continuous, Gamma};
    ///
    /// let distrib: Gamma<f64> = Gamma::new(0.3, 0.2);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf(self: &Self, x: T) -> T
    {
        if x == T::zero()
        {
            return T::zero()
        }
        return gamma::gamma_lr(self.p, self.b * x)
    }


    /// Quantile function of inverse cdf
    ///
    fn quantile(self: &Self, _p: T) -> T
    {
        unimplemented!();
    }

    /// Expected value
    ///
	fn mean(self: &Self) -> T
    {
        return self.p / self.b;
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, Gamma};
    ///
    /// let distrib: Gamma<f64> = Gamma::new(0.2, 0.5);
    /// let var: f64 = distrib.variance();
    /// ```
	fn variance(self: &Self) -> T
    {
        return self.p / self.b.pow(&T::from_f64(2.0).unwrap());
    }
}