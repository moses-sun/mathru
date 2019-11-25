use crate::statistics::distrib::{Continuous, ChiSquared};

/// G-Test
///
/// Fore more information:
/// <a href="https://de.wikipedia.org/wiki/G-Test">https://de.wikipedia.org/wiki/G-Test</a>
///
pub struct G
{
	df: u32,
	g: f64
}

impl G
{
	///
	/// \sum_{i}{y_{i}} = n = \sum_i{xs_i}
	/// b = \sum_{i}{x_{i}}
	/// k = n/b
	/// xs_{i} = x{i}*k
	///
	/// x: observation
	/// y: expectation
	pub fn test_vector(x: &Vec<f64>, y: &Vec<f64>) -> G
	{
		if x.len() != y.len()
		{
			panic!();
		}

		let df: u32 = (x.len() - 1) as u32;

		let mut n: f64 = 0.0;
		for y_i in y.iter()
		{
			n += y_i;
		}

		let mut b: f64 = 0.0;
		for x_i in x.iter()
		{
			b += x_i;
		}

		let k: f64 = n / b;

		let mut g: f64 = 0.0;

		for i in 0..x.len()
		{
			g += x[i] * (x[i]/ (y[i]/k)).ln()
		}

		G
		{
			df: df,
			g: 2.0 * g
		}
	}

	pub fn df(self: &Self) -> u32
	{
		self.df
	}

	pub fn g(self: &Self) -> f64
	{
		self.g
	}

	pub fn p_value(self: &Self) -> f64
	{
		let distrib: ChiSquared = ChiSquared::new(&self.df);
		1.0 - distrib.cdf(self.g)
	}
}