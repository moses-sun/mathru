//! Analysis
//!
//! Fore more information: <br>
//! <a href="https://en.wikipedia.org/wiki/Analysis">https://en.wikipedia.org/wiki/Analysis</a>

//pub mod interpolation;
#[macro_use]
mod function;
mod hessian;
mod jacobian;

mod newton_raphson;

pub mod differential_equation;

pub use function::Function;
pub use hessian::Hessian;
pub use jacobian::Jacobian;
pub use newton_raphson::NewtonRaphson;
