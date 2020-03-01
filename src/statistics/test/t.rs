use crate::statistics::distrib::{Continuous, Normal};
use crate::algebra::linear::{Vector};
use crate::statistics::distrib::T as TD;
use crate::algebra::abstr::Real;

/// T-Test
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Student%27s_t-test">https://en.wikipedia.org/wiki/Student%27s_t-test</a>
///
/// # Example
/// ```
/// use mathru;
/// use mathru::statistics::distrib::{Distribution, Normal};
/// use mathru::statistics::test::T;
///
/// let rv1 = Normal::new(1.0, 0.5).random_vector(100);
/// let rv2 = Normal::new(1.0, 0.5).random_vector(100);
///
/// //Test with sample with identical means
/// let mut measure: T<f64> = T::test_independence_unequal_variance(&rv1, &rv2);
/// println!("{}", measure.t());
/// measure = T::test_independence_equal_variance(&rv1, &rv2);
/// println!("{}", measure.t());
///
/// // Test with different equal mean, but unequal variances
/// let rv3 = Normal::new(1.0, 1.5).random_vector(100);
/// measure = T::test_independence_unequal_variance(&rv1, &rv3);
/// println!("{}", measure.t());
/// measure = T::test_independence_equal_variance(&rv1, &rv3);
/// println!("{}", measure.t());
///
/// // When the sample size is not equal anymore
/// //the equal variance t-statistic is no longer equal to the unequal variance t-statistic:
///	let rv4 = Normal::new(2.0, 0.5).random_vector(300);
/// measure = T::test_independence_unequal_variance(&rv1, &rv4);
/// println!("{}", measure.t());
/// measure = T::test_independence_equal_variance(&rv1, &rv4);
/// println!("{}", measure.t());
///
/// //t-Test with different mean, variance and sample size
///	let rv5 = Normal::new(2.0, 1.0).random_vector(300);
/// measure = T::test_independence_unequal_variance(&rv1, &rv5);
/// println!("{}", measure.t());
/// measure = T::test_independence_equal_variance(&rv1, &rv5);
/// println!("{}", measure.t());
/// ```
pub struct T<K>
{
	p: K,
	t: K
}

impl<K> T<K>
	where K: Real
{
	pub fn t(self: &Self) -> K
	{
		self.t
	}

	pub fn p_value(self: &Self) -> K
	{
		return self.p
	}


	/// Calculates the T-test for the means of two independent samples of scores
	///
	/// This is a two-sided test for the null hypothesis that two independent samples have identical expected values.
	/// It is assumed, that the populations have identical variances.
	pub fn test_independence_equal_variance(x: &Vector<K>, y: &Vector<K>) -> T<K>
	{
		let (_, n_x) : (usize, usize) = x.dim();
		let (_, n_y) : (usize, usize) = y.dim();

		let x_dist: Normal<K> = Normal::from_data(&x);
		let y_dist: Normal<K> = Normal::from_data(&y);

		let mean_x: K = x_dist.mean();
		let mean_y: K = y_dist.mean();

		let df: usize = n_x + n_y - 2;

		let s_x_squared: K = x_dist.variance();
		let s_y_squared: K = y_dist.variance();

		let nomin: K = K::from_f64((n_x - 1) as f64) * s_x_squared + K::from_f64((n_y - 1) as f64) * s_y_squared;
		let denom: K = K::from((df) as f64);

		let s_p: K = (nomin / denom).sqrt();

		let t: K = (mean_x - mean_y) / (s_p * K::from_f64((1.0 / (n_x as f64) + 1.0 / (n_y as f64)).sqrt()));
		T
		{
			p: K::zero(),
			t: t
		}
	}

	/// Calculates the T-test for the means of two independent samples of scores
	///
	/// This is a two-sided test for the null hypothesis that two independent samples have identical expected values.
	/// It is assumed, that the populations have NOT identical variances. It performs the Welch’s t-test
	pub fn test_independence_unequal_variance(x: &Vector<K>, y: &Vector<K>) -> T<K>
	{
		let (_, n_x) : (usize, usize) = x.dim();
		let (_, n_y) : (usize, usize) = y.dim();

		let x_dist: Normal<K> = Normal::from_data(&x);
		let y_dist: Normal<K> = Normal::from_data(&y);

		let mean_x: K = x_dist.mean();
		let mean_y: K = y_dist.mean();

		let s_x_squared: K = x_dist.variance();
		let s_y_squared: K = y_dist.variance();

		let term1: K = s_x_squared / K::from_f64(n_x as f64) + s_y_squared / K::from_f64(n_y as f64);

		let df: K =  term1 * term1 / (s_x_squared * s_x_squared /
		 K::from_f64((n_x * n_x * (n_x - 1)) as f64) +	s_y_squared * s_y_squared / K::from_f64((n_y * n_y * (n_y - 1)) as
		 f64));

		let s_p: K = term1.sqrt();

		let t: K = (mean_x - mean_y) / s_p ;

		let p: K = K::from_f64(2.0) * TD::new(df).cdf(-t.abs());
		T
		{
			p: p,
			t: t
		}
	}

}
