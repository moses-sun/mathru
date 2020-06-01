//! Solves an ODE using Euler's method.
use super::explicit_method::ExplicitFixedStepSizeMethod;
use super::ExplicitODE;
use crate::algebra::abstr::Real;
use crate::algebra::linear::Vector;
use crate::analysis::differential_equation::ordinary::fixed_stepper::ExplicitFixedStepper;

/// Solves an ODE using Euler's method.
///
/// <a href="https://en.wikipedia.org/wiki/Euler_method">https://en.wikipedia.org/wiki/Euler_method</a>
///
/// # Example
///
/// For this example, we want to solve the following ordinary differiential
/// equation: ```math
/// \frac{dy}{dt} = ay = f(t, y)
/// ```
/// The inial condition is $`y(0) = 0.5`$ and we solve it in the interval
/// $`\lbrack 0, 2\rbrack`$ The following equation is the closed solution for
/// this ODE: ```math
/// y(t) = C a e^{at}
/// ```
/// $`C`$ is a parameter and depends on the initial condition $`y(t_{0})`$
/// ```math
/// C = \frac{y(t_{0})}{ae^{at_{0}}}
/// ```
///
/// In this example, we set $`a=2`$
/// ```
/// # #[macro_use]
/// # extern crate mathru;
/// # fn main()
/// # {
/// use mathru::algebra::linear::Vector;
/// use mathru::analysis::differential_equation::ordinary::{Euler, ExplicitODE};
///
/// pub struct ExplicitODE1
/// {
///     time_span: (f64, f64),
///     init_cond: Vector<f64>,
/// }
///
/// impl Default for ExplicitODE1
/// {
///     fn default() -> ExplicitODE1
///     {
///         ExplicitODE1 { time_span: (0.0, 2.0),
///                        init_cond: vector![0.5] }
///     }
/// }
///
/// impl ExplicitODE<f64> for ExplicitODE1
/// {
///     fn func(self: &Self, _t: &f64, x: &Vector<f64>) -> Vector<f64>
///     {
///         return x * &2.0f64;
///     }
///
///     fn time_span(self: &Self) -> (f64, f64)
///     {
///         return self.time_span;
///     }
///
///     fn init_cond(self: &Self) -> Vector<f64>
///     {
///         return self.init_cond.clone();
///     }
/// }
///
/// // We instanciate Eulers algorithm with a stepsize of 0.001
/// let step_size: f64 = 0.001;
/// let solver: Euler<f64> = Euler::new(step_size);
///
/// let problem: ExplicitODE1 = ExplicitODE1::default();
///
/// // Solve the ODE
/// let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();
/// # }
/// ```
pub struct Euler<T>
{
    stepper: ExplicitFixedStepper<T>,
}

impl<T> Euler<T> where T: Real
{
    /// Creates a Euler instance
    pub fn new(step_size: T) -> Euler<T>
    {
        return Euler { stepper: ExplicitFixedStepper::new(step_size) };
    }

    pub fn solve<F>(self: &Self, prob: &F) -> Result<(Vec<T>, Vec<Vector<T>>), ()>
        where F: ExplicitODE<T>
    {
        return self.stepper.solve(prob, self);
    }

    pub fn get_step_size(self: &Self) -> &T
    {
        return self.stepper.get_step_size();
    }

    pub fn set_step_size(self: &mut Self, step_size: T)
    {
        self.stepper.set_step_size(step_size)
    }
}

impl<T> ExplicitFixedStepSizeMethod<T> for Euler<T> where T: Real
{
    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> Vector<T>
        where F: ExplicitODE<T>
    {
        return x_n + &(&prob.func(t_n, x_n) * h);
    }

    /// Euler's method is a first order method
    fn order(self: &Self) -> u8
    {
        return 1;
    }
}
