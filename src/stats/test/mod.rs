//! Statistical hypothesis tests
//!
//! Fore more information:
//! <a href="https://en.wikipedia.org/wiki/Statistical_hypothesis_testing">https://en.wikipedia
//! .org/wiki/Statistical_hypothesis_testing</a>

mod test;
mod chisquared;
mod g;
mod t;

pub use self::test::Test;
pub use self::chisquared::ChiSquared;
pub use self::g::G;
pub use self::t::T;