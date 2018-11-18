mod normal;
mod bernoulli;
mod binomial;
mod exponential;
mod chisquared;
mod distrib;
mod poisson;
mod beta;
mod gamma;

pub use self::distrib::{Discrete, Continuous};
pub use self::binomial::Binomial;
pub use self::bernoulli::Bernoulli;
pub use self::normal::Normal;
pub use self::exponential::Exponential;
pub use self::chisquared::ChiSquared;
pub use self::poisson::Poisson;
pub use self::beta::Beta;
pub use self::gamma::Gamma;