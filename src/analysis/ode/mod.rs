//! Ordinary differential equation
//!
//! Fore more information:<br>
//! <a href="https://en.wikipedia.org/wiki/Ordinary_differential_equation">https://en.wikipedia.org/wiki/Ordinary_differential_equation</a>

mod rk4;
mod euler;
mod midpoint;
mod heun;
mod rkf45;
mod dopri5;
mod adamsbashforth;
mod adamsmoulton;
mod solver;

pub use rk4::RK4;
pub use euler::Euler;
pub use midpoint::Midpoint;
pub use heun::Heun;
pub use rkf45::RKF45;
pub use dopri5::Dopri5;
pub use adamsbashforth::AdamsBashforth;
pub use adamsmoulton::AM;
pub use solver::{Solver, ExplicitODE};