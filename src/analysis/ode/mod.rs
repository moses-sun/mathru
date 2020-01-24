//! Ordinary differential equation
//!
//! Fore more information:<br>
//! <a href="https://en.wikipedia.org/wiki/Ordinary_differential_equation">https://en.wikipedia.org/wiki/Ordinary_differential_equation</a>

mod rungekutta4;
mod euler;
mod midpoint;
mod heun;
mod kutta3;
mod ralston;
mod rungekuttafehlberg45;
mod dormandprince45;
//mod adamsbashforth;
//mod adamsmoulton;
mod explicit_ode;
mod explicit_method;
mod fixed_stepper;
mod adaptive_stepper;
//mod step_size_controller;

pub use rungekutta4::RungeKutta4;
pub use kutta3::Kutta3;
pub use euler::Euler;
pub use midpoint::Midpoint;
pub use heun::Heun;
pub use ralston::Ralston;
pub use rungekuttafehlberg45::RungeKuttaFehlberg45;
pub use dormandprince45::DormandPrince45;
//pub use adamsbashforth::AdamsBashforth;
//pub use adamsmoulton::AM;
pub use explicit_ode::{ExplicitODE};
pub use explicit_method::{ExplicitFixedStepSizeMethod, ExplicitAdaptiveMethod};
pub use fixed_stepper::FixedStepper;
pub use adaptive_stepper::AdaptiveStepper;
//use step_size_controller::StepSizeController;