use stats::distrib::ChiSquared as ChiSquaredDistrib;
use stats::distrib::Continuous;
use stats::test::Test;

/// Chi-Squared Ttest
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Chi-squared_test">https://en.wikipedia.org/wiki/Chi-squared_test</a>
///
pub struct ChiSquared
{
	df: u32,
	chi_squared: f64
}


impl ChiSquared
{
	///
	/// alpha: significance level
	pub fn test_vector(x: &Vec<f64>, y: &Vec<f64>) -> ChiSquared
	{
		if x.len() != y.len()
		{
			panic!();
		}

		let df: u32 = (y.len() -1) as u32;

		let mut sum_x: f64 = 0.0;
		for n_i in x.iter()
		{
			sum_x += n_i;
		}

		let mut sum_y: f64 = 0.0;
		for n_j in y.iter()
		{
			sum_y += n_j;
		}

		let n: f64 = sum_x + sum_y;

		let mut chi_squared: f64 = 0.0;
		let m: usize = x.len();
		for j in 0..m
		{
			for k in 0..2
			{
				let mut n_jk: f64;
				let mut n_k: f64 = 0.0;
				if k == 0
				{
					n_jk = x[j];
					for l in 0..m
					{
						n_k += x[l];
					}
				}
				else
				{
					n_jk = y[j];
					for l in 0..m
					{
						n_k += y[l];
					}
				}

				let n_j_: f64 = x[j] + y[j];

				let n_jks: f64 = (n_k * n_j_) / (n);
				chi_squared += (n_jk - n_jks).powi(2)/n_jks
			}
		}

		ChiSquared
		{
			chi_squared: chi_squared,
			df: df
		}
	}
}

impl Test for ChiSquared
{

	fn df(self: &Self) -> u32
	{
		self.df
	}

	fn value(self: &Self) -> f64
	{
		self.chi_squared
	}

	fn p_value(self: &Self) -> f64
	{
		let distrib: ChiSquaredDistrib = ChiSquaredDistrib::new(&self.df);
		1.0 - distrib.cdf(self.chi_squared)
	}
}