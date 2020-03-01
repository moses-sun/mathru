use crate::statistics::distrib::{Continuous, ChiSquared};
use crate::algebra::abstr::Real;

/// G-Test
///
/// Fore more information:
/// <a href="https://de.wikipedia.org/wiki/G-Test">https://de.wikipedia.org/wiki/G-Test</a>
///
pub struct G<T>
{
	df: u32,
	g: T
}

impl<T> G<T>
	where T: Real
{
	///
	/// \sum_{i}{y_{i}} = n = \sum_i{xs_i}
	/// b = \sum_{i}{x_{i}}
	/// k = n/b
	/// xs_{i} = x{i}*k
	///
	/// x: observation
	/// y: expectation
	pub fn test_vector(x: &Vec<T>, y: &Vec<T>) -> G<T>
	{
		if x.len() != y.len()
		{
			panic!();
		}

		let df: u32 = (x.len() - 1) as u32;

		let mut n: T = T::zero();
		for y_i in y.iter()
		{
			n += *y_i;
		}

		let mut b: T = T::zero();
		for x_i in x.iter()
		{
			b += *x_i;
		}

		let k: T = n / b;

		let mut g: T = T::zero();

		for i in 0..x.len()
		{
			g += x[i] * (x[i]/ (y[i]/k)).ln()
		}

		G
		{
			df: df,
			g: T::from_f64(2.0) * g
		}
	}

	pub fn df(self: &Self) -> u32
	{
		self.df
	}

	pub fn g(self: &Self) -> T
	{
		self.g
	}

	pub fn p_value(self: &Self) -> T
	{
		let distrib: ChiSquared<T> = ChiSquared::new(self.df);
		T::one() - distrib.cdf(self.g)
	}
}