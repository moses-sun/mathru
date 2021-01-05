//! Solves an ODE using Heun's method.
use super::{explicit_method::ExplicitFixedStepSizeMethod, ExplicitODE};
use crate::{
    algebra::{abstr::Real, linear::Vector},
    analysis::differential_equation::ordinary::fixed_stepper::ExplicitFixedStepper,
};
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Solves an ODE using Heun's method.
///
/// <a href="https://en.wikipedia.org/wiki/Heun's_method">https://en.wikipedia.org/wiki/Heun's_method</a>
///
/// # Example
///
/// For this example, we want to solve the following ordinary differiential
/// equation:
/// ```math
/// \frac{dy}{dt} = ay = f(t, y)
/// ```
/// The inial condition is $`y(0) = 0.5`$ and we solve it in the interval
/// $`\lbrack 0, 2\rbrack`$ The following equation is the closed solution for
/// this ODE:
/// ```math
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
/// use mathru::{
///     algebra::linear::Vector,
///     analysis::differential_equation::ordinary::{ExplicitODE, Heun},
/// };
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
/// // We instantiate Heun's algorithm with a step size of 0.001
/// let step_size: f64 = 0.001;
/// let solver: Heun<f64> = Heun::new(step_size);
///
/// let problem: ExplicitODE1 = ExplicitODE1::default();
///
/// // Solve the ODE
/// let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();
///
/// # }
/// ```
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Heun<T>
{
    stepper: ExplicitFixedStepper<T>,
}

impl<T> Heun<T> where T: Real
{
    /// Creates a Heun instance with step size 'step_size'
    pub fn new(step_size: T) -> Heun<T>
    {
        return Heun { stepper: ExplicitFixedStepper::new(step_size) };
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

impl<T> ExplicitFixedStepSizeMethod<T> for Heun<T> where T: Real
{
    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> Vector<T>
        where F: ExplicitODE<T>
    {
        let k_1: Vector<T> = prob.func(t_n, x_n);
        let k_2: Vector<T> = prob.func(&(*t_n + *h), &(x_n + &(&k_1 * h)));

        return x_n + &((&k_1 + &k_2) * *h / T::from_f64(2.0));
    }

    ///
    fn order(self: &Self) -> u8
    {
        return 2;
    }
}
