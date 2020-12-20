//! Statistical hypothesis tests
//!
//! Fore more information:
//! <a href="https://en.wikipedia.org/wiki/Statistical_hypothesis_testing">https://en.wikipedia
//! .org/wiki/Statistical_hypothesis_testing</a>

mod chisquare;
mod g;
mod t;
mod test;

pub use self::{chisquare::ChiSquare, g::G, t::T, test::Test};
