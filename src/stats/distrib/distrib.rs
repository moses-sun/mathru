
/// Coontinuous distribution
pub trait Continuous<A, B>
{
   	/// Probability density function
   	///
   	/// # Arguments
   	///
   	/// *`x`: random variable
   	///
    fn pdf<'a, 'b>(self: &'a Self, x: A) -> f64;

    /// Cumulative distribution function
    ///
    /// # Arguments
    ///
    /// *`x`: random variable
    ///
    fn cdf<'a, 'b>(self: &'a Self, x: B) -> f64;

	/// Quantile function, inverse cdf
    fn quantile<'a, 'b>(self: &'a Self, p: B) -> f64;

	/// Mean
	fn mean<'a>(self: &'a Self) -> f64;

	/// Variance
	fn variance<'a>(self: &'a Self) -> f64;

}

/// Discrete distribution
pub trait Discrete<A, B>
{
   	/// Probability mass function
   	///
   	/// # Arguments
   	///
   	/// *`x`: random variable
   	///
    fn pmf<'a, 'b>(self: &'a Self, x: A) -> f64;

    ///Cumulative distribution function
    ///
    /// # Arguments
    ///
    /// * `x`: random variable
    ///
    fn cdf<'a, 'b>(self: &'a Self, x: B) -> f64;

    /// Mean
    ///
	fn mean<'a>(self: &'a Self) -> f64;

   	/// Variance
   	///
	fn variance<'a>(self: &'a Self) -> f64;

}
