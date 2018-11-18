use stats::distrib::Continuous;
extern crate rand;
use std::f64;

/// Exponential distribution
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Exponential_distribution">https://en.wikipedia.org/wiki/Exponential_distribution</a>
///
pub struct Exponential
{
	lambda: f64
}

impl Exponential
{
	/// Creates a probability distribution
    ///
    /// # Arguments
    ///
    /// * `lambda`  > 0.0
    ///
    /// # Panics
    ///
    /// if lambda <= 0.0
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::Exponential;
    ///
    /// let distrib: Exponential = Exponential::new(&0.3);
    /// ```
	pub fn new(lambda: &f64) -> Exponential
	{
		if *lambda <= 0.0
		{
			panic!()
		}

		Exponential
		{
			lambda: *lambda
		}
	}

	pub fn from_data<'a>(data: &'a Vec<f64>) -> Self
    {
        let lambda : f64 = 1.0 / Exponential::calc_mean(data);

        return Exponential::new(&lambda)
    }

    fn calc_mean<'a>(data: &'a Vec<f64>) -> f64
    {
        let mut sum: f64 = 0.0;

        for x in data.iter()
        {
            sum += x;
        }

        return sum / (data.len() as f64)
    }
}

impl Continuous<f64, f64> for Exponential
{
    /// Probability density function
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin; &#x2115 | x > 0.0
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Continuous, Exponential};
    ///
    /// let distrib: Exponential = Exponential::new(&0.3);
    /// let x: f64 = 5.0;
    /// let p: f64 = distrib.pdf(x);
    /// ```
    fn pdf<'a>(self: &'a Self, x: f64) -> f64
	{
		if x < 0.0
		{
			return 0.0
		}

		let p: f64 = self.lambda * (-self.lambda * x).exp();
		p
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
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Continuous, Exponential};
    ///
    /// let distrib: Exponential = Exponential::new(&0.3);
    /// let x: f64 = 0.4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
    fn cdf<'a>(self: &'a Self, x: f64) -> f64
	{
		if x < 0.0
		{
			return 0.0
		}

		let p: f64 = 1.0 - (-x * self.lambda).exp();
		p
	}

	/// Quantile function of inverse cdf
    fn quantile<'a>(self: &'a Self, p: f64) -> f64
    {
    	let q: f64 = -(1.0 - p).ln() / self.lambda;

    	q
    }

  	/// Expected value
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Discrete, Bernoulli};
    ///
    /// let distrib: Bernoulli = Bernoulli::new(0.2);
    /// let mean: f64 = distrib.mean();
    /// ```
	fn mean<'a>(self: &'a Self) -> f64
	{
		1.0 / self.lambda
	}

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Discrete, Bernoulli};
    ///
    /// let distrib: Bernoulli = Bernoulli::new(0.2);
    /// let var: f64 = distrib.variance();
    /// ```
	fn variance<'a>(self: &'a Self) -> f64
	{
		1.0 / self.lambda.powi(2)
	}
}


impl Exponential
{
 	pub fn random(self: &Self) -> f64
    {
  		let y: f64 = rand::random::<f64>();
   		let p: f64 = self.quantile(y);

   		p
	}
}