use crate::stats::distrib::Discrete;
use crate::stats::combins;
use crate::special;


/// Poisson distribution
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Poisson_distribution">https://en.wikipedia.org/wiki/Poisson_distribution</a>
///
pub struct Poisson
{
	gamma: f64
}

impl Poisson
{
	/// Creates a probability distribution
    ///
    /// # Arguments
    ///
    /// * `gamma` gamma > 0.0
    ///
    /// # Panics
    ///
    /// if gamma <= 0.0
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::Poisson;
    ///
    /// let distrib: Poisson = Poisson::new(&0.2);
    /// ```
	pub fn new(gamma: &f64) -> Poisson
	{
		if *gamma <= 0.0
		{
			panic!();
		}

		Poisson
		{
			gamma: *gamma
		}
	}
}

impl Discrete<u32, u32> for Poisson
{
	/// Probability mass function
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin; &#x2115;
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Discrete, Poisson};
    ///
    /// let distrib: Poisson = Poisson::new(&0.2);
    /// let x: u32 = 5;
    /// let p: f64 = distrib.pmf(x);
    /// ```
    fn pmf<'a>(self: &'a Self, x: u32) -> f64
	{
		let k_fact: f64 = combins::factorial(x) as f64;
		self.gamma.powf(x as f64) * (-self.gamma).exp() / k_fact
	}

	/// Cumulative distribution function of the Bernoulli distribution
    ///
    /// # Arguments
    ///
    /// * `x` Random variable x &isin; &#x2115;
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Discrete, Poisson};
    ///
    /// let distrib: Poisson = Poisson::new(&0.2);
    /// let x: u32 = 4;
    /// let p: f64 = distrib.cdf(x);
    /// ```
	fn cdf<'a>(self: &'a Self, x: u32) -> f64
    {
		special::gamma::gamma_ur( (x + 1) as f64,self.gamma)
	}

  	/// Expected value
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Discrete, Poisson};
    ///
    /// let distrib: Poisson = Poisson::new(&0.2);
    /// let mean: f64 = distrib.mean();
    /// ```
	fn mean<'a>(self: &'a Self) -> f64
    {
        return self.gamma
    }

   	/// Variance
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Discrete, Poisson};
    ///
    /// let distrib: Poisson = Poisson::new(&0.2);
    /// let var: f64 = distrib.variance();
    /// ```
	fn variance<'a>(self: &'a Self) -> f64
    {
        return self.gamma
    }
}