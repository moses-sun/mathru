//! Solves an ODE using the 3th order Runge-Kutta algorithm.
use super::{explicit_method::ExplicitFixedStepSizeMethod, ExplicitODE};
use crate::{
    algebra::{abstr::Real, linear::Vector},
    analysis::differential_equation::ordinary::fixed_stepper::ExplicitFixedStepper,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Solves an ODE using the 3th order Runge-Kutta algorithm.
///
///<a href="https://en.wikipedia.org/wiki/Runge-Kutta_methods">https://en.wikipedia.org/wiki/Rung-Kutta_methods</a>
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
///     analysis::differential_equation::ordinary::{ExplicitODE, Kutta3},
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
/// // We instantiate CashKarp algorithm
/// let step_size: f64 = 0.001;
/// let solver: Kutta3<f64> = Kutta3::new(step_size);
///
/// let problem: ExplicitODE1 = ExplicitODE1::default();
///
/// // Solve the ODE
/// let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();
///
/// # }
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct Kutta3<T>
{
    stepper: ExplicitFixedStepper<T>,
}

impl<T> Kutta3<T> where T: Real
{
    /// Creates a Kutta3 instance with step size 'step_size'
    pub fn new(step_size: T) -> Kutta3<T>
    {
        return Kutta3 { stepper: ExplicitFixedStepper::new(step_size) };
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

impl<T> ExplicitFixedStepSizeMethod<T> for Kutta3<T> where T: Real
{
    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> Vector<T>
        where F: ExplicitODE<T>
    {
        // k1 = f(t_n, x_n)
        let k1: Vector<T> = prob.func(&t_n, &x_n);

        // k2 = func(t_n + h / 2, x_n + h / 2 k1)
        let k2: Vector<T> = prob.func(&(*t_n + (*h / T::from_f64(2.0))),
                                      &(x_n + &((&k1 * h) / T::from_f64(2.0))));

        // k3 = func(t_n + h, x_n + - h k1 + 2h *k2)
        let k3: Vector<T> = prob.func(&(*t_n + *h),
                                      &(x_n + &(&(&(&k2 * &T::from_f64(2.0)) - &k1) * h)));

        // x[n+1] = xn + h*(k1 + 4*k2 + k3)/6
        return x_n + &((k1 + k2 * T::from_f64(4.0) + k3) * *h / T::from_f64(6.0));
    }

    /// Kuttas method is a 3rd order method
    fn order(self: &Self) -> u8
    {
        return 3;
    }
}
