//! Ordinary differential equation
//!
//! Fore more information:<br>
//! <a href="https://en.wikipedia.org/wiki/Ordinary_differential_equation">https://en.wikipedia.org/wiki/Ordinary_differential_equation</a>

//! Because ODE higher order can always be reduced to a system of first order ODEs,  the implemented algorithms only support to solve
//! first order ODEs.
//!
//! ```math
//! \frac{dy}{dt}=f(t, y)
//! ```

mod euler;
mod implicit_euler;
mod midpoint;
mod heun;
mod kutta3;
mod bogackishampine32;
mod ralston;
mod rungekutta4;
mod rungekuttafehlberg54;
mod dormandprince54;
mod cashkarp54;
//mod tsitouras54;
mod adamsbashforth;
//mod adamsmoulton;
mod explicit_ode;
mod implicit_ode;
mod explicit_method;
mod implicit_method;
mod fixed_stepper;
mod adaptive_stepper;

pub use euler::Euler;
pub use implicit_euler::ImplicitEuler;
pub use midpoint::Midpoint;


pub use heun::Heun;

pub use kutta3::Kutta3;
pub use rungekutta4::RungeKutta4;
pub use ralston::Ralston;
pub use bogackishampine32::BogackiShampine32;
pub use rungekuttafehlberg54::RungeKuttaFehlberg54;
pub use dormandprince54::DormandPrince54;
pub use cashkarp54::CashKarp54;

pub use adamsbashforth::AdamsBashforth;
//pub use adamsmoulton::AM;
//pub use tsitouras54::Tsitouras54;
pub use explicit_ode::ExplicitODE;
pub use implicit_ode::ImplicitODE;

