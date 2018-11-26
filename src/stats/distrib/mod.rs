/// Distributions
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Probability_distribution">https://en.wikipedia.org/wiki/Probability_distribution</a>
mod normal;
mod bernoulli;
mod binomial;
mod exponential;
mod chisquared;
mod distrib;
mod poisson;
mod beta;
mod gamma;
mod t;

pub use self::distrib::{Discrete, Continuous};
pub use self::binomial::Binomial;
pub use self::bernoulli::Bernoulli;
pub use self::normal::Normal;
pub use self::exponential::Exponential;
pub use self::chisquared::ChiSquared;
pub use self::poisson::Poisson;
pub use self::beta::Beta;
pub use self::gamma::Gamma;
pub use self::t::T;