//! Distributions
//!
//! Each univariate distribution implements the trait Continuous or Discrete for
//! discrete distributions.
//!
//! Fore more information:
//! <a href="https://en.wikipedia.org/wiki/Probability_distribution">https://en.wikipedia
//! .org/wiki/Probability_distribution</a>
//!
mod bernoulli;
mod beta;
mod binomial;
mod chisquared;
mod distrib;
mod exponential;
mod gamma;
mod normal;
mod poisson;
mod t;
//mod multinomial;
mod raisedcosine;
mod uniform;

pub use self::{
    bernoulli::Bernoulli,
    beta::Beta,
    binomial::Binomial,
    chisquared::ChiSquared,
    distrib::{Continuous, Discrete, Distribution},
    exponential::Exponential,
    gamma::Gamma,
    normal::Normal,
    poisson::Poisson,
    t::T,
};
//pub use self::multinomial::Multinomial;
pub use self::{raisedcosine::RaisedCosine, uniform::Uniform};
