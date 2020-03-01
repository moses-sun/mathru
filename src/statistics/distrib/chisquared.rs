use crate::statistics::distrib::Continuous;
use crate::special::gamma;
use crate::special;
use super::Normal;
use crate::algebra::abstr::Real;


/// Chi-Squared distribution
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Chi-squared_distribution">https://en.wikipedia.org/wiki/Chi-squared_distribution</a>
///
pub struct ChiSquared<T>
{
    k: T //degree of freedom
}

impl<T> ChiSquared<T>
    where T: Real
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
    /// let distrib: ChiSquared<f64> = ChiSquared::new(3);
    /// ```
    pub fn new(df: u32) -> ChiSquared<T>
    {
        if T::from_u32(df) < T::one()
        {
            panic!()
        }
        ChiSquared
        {
            k: T::from_u32(df)
        }
    }
}


impl<T> Continuous<T> for ChiSquared<T>
    where T: Real
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
    /// let distrib: ChiSquared<f64> = ChiSquared::new(2);
    /// let x: f64 = 5.0;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf(self: &Self, x: T) -> T
    {
        if x < T::zero()
        {
            return T::zero()
        }
        let t1: T = T::one() / (T::from_f64(2.0).pow(&(self.k / T::from_f64(2.0))) * gamma::gamma(self.k / T::from_f64(2.0)));
        let t2: T = x.pow(&(self.k / T::from_f64(2.0) - T::one())) * (-x / T::from_f64(2.0)).exp();
        let chisquared: T = t1 * t2;

        return chisquared;
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
    /// let distrib: ChiSquared<f64> = ChiSquared::new(3);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf(self: &Self, x: T) -> T
    {
        let t1: T = (-x / T::from_f64(2.0)).exp();

        let k_natural: u32 = self.k.to_u32();
        let p: T;

        if k_natural % 2 == 0
        {
            let mut sum: T = T::zero();
            for i in 0..(self.k / T::from_f64(2.0)).to_u32()
            {
                sum += (x / T::from_f64(2.0)).pow(&T::from_u32(i)) / gamma::gamma(T::from_u32(i + 1))
            }

            p = T::one() - t1 * sum;
        }
        else
        {
            let mut sum: T = T::zero();
            for i in 0..(self.k / T::from_f64(2.0)).to_u32()
            {
                sum += (x / T::from_f64(2.0)).pow(&T::from_f64((i as f64) + 0.5)) / gamma::gamma(T::from_f64((i as f64) +
                1.5));
            }

            p = special::erf((x / T::from_f64(2.0)).sqrt()) - t1 * sum;
        }

        p
    }

    /// Quantile function of inverse cdf
    ///
    fn quantile(self: &Self, p: T) -> T
    {
        let std_distrib: Normal<T> = Normal::new(T::zero(), T::one());
        let q: T = T::from_f64(0.5) * (std_distrib.quantile(p) + (T::from_f64(2.0) * self.k - T::one()).sqrt()).pow
        (&T::from_f64(2.0));
        return q;
    }

    /// Expected value
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, ChiSquared};
    ///
    /// let distrib: ChiSquared<f64> = ChiSquared::new(2);
    /// let mean: f64 = distrib.mean();
    /// ```
	fn mean(self: &Self) -> T
    {
        return self.k
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::statistics::distrib::{Continuous, ChiSquared};
    ///
    /// let distrib: ChiSquared<f64> = ChiSquared::new(2);
    /// let var: f64 = distrib.variance();
    /// ```
	fn variance(self: &Self) -> T
    {
        return T::from_f64(2.0) * self.k
    }
}