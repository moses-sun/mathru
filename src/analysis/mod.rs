//! Analysis
//!
//! Fore more information: <br>
//! <a href="https://en.wikipedia.org/wiki/Analysis">https://en.wikipedia.org/wiki/Analysis</a>

//pub mod interpolation;
#[macro_use]
mod function;
mod jacobian;
mod hessian;

mod newton_raphson;

pub mod differential_equation;

pub use newton_raphson::NewtonRaphson;
pub use function::Function;
pub use jacobian::Jacobian;
pub use hessian::Hessian;
