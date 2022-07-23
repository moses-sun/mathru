//! Elementary functions
//!
//! Fore more information:
//! <https://en.wikipedia.org/wiki/Elementary_function>

pub mod exponential;
pub mod hyperbolic;
pub mod power;
pub mod trigonometry;

pub use self::{
    exponential::Exponential, hyperbolic::Hyperbolic, power::Power, trigonometry::Trigonometry,
};
