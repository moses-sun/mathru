use crate::algebra::abstr::Real;
use std::iter;

pub trait Distribution<T>
    where T: Real
{
    fn random(self: &Self) -> T;

    fn random_sequence(self: &Self, size: u32) -> Vec<T>
    {
        let mut v: Vec<T> = Vec::new();
        v.extend(iter::repeat_with(&|| self.random()).take(size as usize));

        v
    }
}

/// Continuous distribution
pub trait Continuous<T>
    where T: Real
{
    /// Probability density function
    ///
    /// # Arguments
    ///
    /// *`x`:
    fn pdf(self: &Self, x: T) -> T;

    /// Cumulative distribution function
    ///
    /// # Arguments
    ///
    /// *`x`:
    fn cdf(self: &Self, x: T) -> T;

    /// Quantile function, inverse cdf
    fn quantile(self: &Self, p: T) -> T;

    /// Mean
    fn mean(self: &Self) -> T;

    /// Variance
    fn variance(self: &Self) -> T;

    /// Skewness is a measure of the asymmetry of the probability distribution
    /// of a real-valued random variable about its mean
    fn skewness(self: &Self) -> T;

    /// Median is the value separating the higher half from the lower half of a
    /// probability distribution.
    fn median(self: &Self) -> T;

    ///
    fn entropy(self: &Self) -> T;
}

/// Discrete distribution
pub trait Discrete<T, A, B>
{
    /// Probability mass function
    ///
    /// # Arguments
    ///
    /// *`x`:
    fn pmf(self: &Self, x: A) -> T;

    ///Cumulative distribution function
    ///
    /// # Arguments
    ///
    /// * `x`:
    fn cdf(self: &Self, x: B) -> T;

    /// Mean
    fn mean(self: &Self) -> T;

    /// Variance
    fn variance(self: & Self) -> T;
}
