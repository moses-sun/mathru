use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;

pub trait Distribution<T>
	where T: Real
{
    fn random(self: &Self) -> T;

    fn random_vector(self: &Self, size: usize) -> Vector<T>
	{
        let v: Vector<T> = Vector::zero(size).transpose().apply(&|_x| self.random());

        return v;
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
   	/// *`x`: random variable
   	///
    fn pdf(self: &Self, x: T) -> T;

    /// Cumulative distribution function
    ///
    /// # Arguments
    ///
    /// *`x`: random variable
    ///
    fn cdf(self: &Self, x: T) -> T;

	/// Quantile function, inverse cdf
    fn quantile(self: &Self, p: T) -> T;

	/// Mean
	fn mean(self: &Self) -> T;

	/// Variance
	fn variance(self: &Self) -> T;

}

/// Discrete distribution
pub trait Discrete<T, A, B>
{
   	/// Probability mass function
   	///
   	/// # Arguments
   	///
   	/// *`x`: random variable
   	///
    fn pmf<'a, 'b>(self: &'a Self, x: A) -> T;

    ///Cumulative distribution function
    ///
    /// # Arguments
    ///
    /// * `x`: random variable
    ///
    fn cdf<'a, 'b>(self: &'a Self, x: B) -> T;

    /// Mean
    ///
	fn mean<'a>(self: &'a Self) -> T;

   	/// Variance
   	///
	fn variance<'a>(self: &'a Self) -> T;

}
