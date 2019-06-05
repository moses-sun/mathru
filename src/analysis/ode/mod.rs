//! Ordinary differential equation
//!
//! Fore more information:<br>
//! <a href="https://en.wikipedia.org/wiki/Ordinary_differential_equation">https://en.wikipedia.org/wiki/Ordinary_differential_equation</a>

mod rk4;
mod euler;
mod heun;

mod solver;
pub use rk4::RK4;
pub use euler::Euler;
pub use heun::Heun;
pub use solver::Solver;