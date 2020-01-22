//! Ordinary differential equation
//!
//! Fore more information:<br>
//! <a href="https://en.wikipedia.org/wiki/Ordinary_differential_equation">https://en.wikipedia.org/wiki/Ordinary_differential_equation</a>

mod rungekutta4;
mod euler;
mod midpoint;
mod heun;
mod kutta3;
mod rkf45;
mod ralston;
mod dopri5;
mod adamsbashforth;
mod adamsmoulton;
mod solver;

pub use rungekutta4::RungeKutta4;
pub use kutta3::Kutta3;
pub use euler::Euler;
pub use midpoint::Midpoint;
pub use heun::Heun;
pub use ralston::Ralston;
pub use rkf45::RKF45;
pub use dopri5::Dopri5;
pub use adamsbashforth::AdamsBashforth;
pub use adamsmoulton::AM;
pub use solver::{Solver, ExplicitODE};