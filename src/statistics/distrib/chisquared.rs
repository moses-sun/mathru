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
        if T::from_u32(df).unwrap() < T::one()
        {
            panic!()
        }
        ChiSquared
        {
            k: T::from_u32(df).unwrap()
        }
    }
}


impl<T> Continuous<T, T, T> for ChiSquared<T>
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
    fn pdf<'a>(self: &'a Self, x: T) -> T
    {
        if x < T::zero()
        {
            return T::zero()
        }
        let t1: T = T::one() / (T::from_f64(2.0).unwrap().pow(&(self.k / T::from_f64(2.0).unwrap())) * gamma::gamma(self.k / T::from_f64(2.0).unwrap()));
        let t2: T = x.pow(&(self.k / T::from_f64(2.0).unwrap() - T::one())) * (-x / T::from_f64(2.0).unwrap()).exp();
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
    fn cdf<'a>(self: &'a Self, x: T) -> T
    {
        let t1: T = (-x / T::from_f64(2.0).unwrap()).exp();

        let k_natural: u32 = self.k.to_u32().unwrap();
        let p: T;

        if k_natural % 2 == 0
        {
            let mut sum: T = T::zero();
            for i in 0..(self.k / T::from_f64(2.0).unwrap()).to_u32().unwrap()
            {
                sum += (x / T::from_f64(2.0).unwrap()).pow(&T::from_u32(i).unwrap()) / gamma::gamma(T::from_u32(i + 1).unwrap())
            }

            p = T::one() - t1 * sum;
        }
        else
        {
            let mut sum: T = T::zero();
            for i in 0..(self.k / T::from_f64(2.0).unwrap()).to_u32().unwrap()
            {
                sum += (x / T::from_f64(2.0).unwrap()).pow(&T::from_f64((i as f64) + 0.5).unwrap()) / gamma::gamma(T::from_f64((i as f64) +
                1.5).unwrap());
            }

            p = special::erf((x / T::from_f64(2.0).unwrap()).sqrt()) - t1 * sum;
        }

        p
    }

    /// Quantile function of inverse cdf
    ///
    fn quantile<'a, 'b>(self: &'a Self, p: T) -> T
    {
        let std_distrib: Normal<T> = Normal::new(T::zero(), T::one());
        let q: T = T::from_f64(0.5).unwrap() * (std_distrib.quantile(p) + (T::from_f64(2.0).unwrap() * self.k - T::one()).sqrt()).pow
        (&T::from_f64(2.0).unwrap());
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
	fn mean<'a>(self: &'a Self) -> T
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
	fn variance<'a>(self: &'a Self) -> T
    {
        return T::from_f64(2.0).unwrap() * self.k
    }
}