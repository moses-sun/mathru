use stats::distrib::Continuous;
use special;
use special::beta;


/// Beta distribution
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Beta_distribution">https://en.wikipedia.org/wiki/Beta_distribution</a>
///
pub struct Beta
{
    p: f64,
    q: f64,
}

impl Beta
{

    /// Create a probability distribution
    ///
    /// # Arguments
    ///
    /// * `p`: &alpha; > 0.0
    /// * `q`: &beta; > 0.0
    ///
    /// # Panics
    /// if p <= 0.0 || q <= 0.0
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Continuous, Beta};
    ///
    /// let distrib: Beta = Beta::new(&0.2, &0.3);
    /// ```
    pub fn new(p: &f64, q: &f64) -> Beta
    {
        if *p < 0.0_f64 || *q <= 0.0_f64
        {
            panic!()
        }
        Beta
        {
            p: *p,
            q: *q
        }
    }
}

impl Continuous<f64, f64> for Beta
{
    fn pdf<'a>(self: &'a Self, x: f64) -> f64
    {
        if 0.0_f64 > x || x > 1.0_f64
        {
            panic!();
        }
        x.powf(self.p - 1.0_f64) * (1.0_f64 - x).powf(self.q - 1.0_f64) / special::beta::beta(self.p, self.q)
    }

    fn cdf<'a>(self: &'a Self, x: f64) -> f64
    {
        beta::beta_inc_reg(x, self.p, self.q)
    }


    /// Quantile function of inverse cdf
    ///
    fn quantile<'a, 'b>(self: &'a Self, _p: f64) -> f64
    {
        unimplemented!();
    }

    /// Expected value
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Continuous, Beta};
    ///
    /// let distrib: Beta = Beta::new(&0.2, &0.3);
    /// let mean: f64 = distrib.mean();
    /// ```
	fn mean<'a>(self: &'a Self) -> f64
    {
        self.p  / (self.p + self.q)
    }

    /// Variance
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::stats::distrib::{Continuous, Beta};
    ///
    /// let distrib: Beta = Beta::new(&0.2, &0.3);
    /// let var: f64 = distrib.variance();
    /// ```
	fn variance<'a>(self: &'a Self) -> f64
    {
        self.p * self.q / ((self.p + self.q + 1.0_f64) * (self.p + self.q).powi(2))
    }
}