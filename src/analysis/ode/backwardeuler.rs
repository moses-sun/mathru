use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::implicit_method::{ImplicitFixedStepSizeMethod};
use super::ImplicitODE;
use crate::analysis::ode::fixed_stepper::ImplicitFixedStepper;

/// Solves an ordinary differential equation using Euler's method.
///
/// <a href="https://en.wikipedia.org/wiki/Euler_method">https://en.wikipedia.org/wiki/Euler_method</a>
pub struct BackwardEuler<T>
{
    stepper: ImplicitFixedStepper<T>
}

impl<T> BackwardEuler<T>
    where T: Real
{
    /// Creates a backward Euler instance
    ///
    pub fn new(step_size: T) -> BackwardEuler<T>
    {
        return BackwardEuler
        {
            stepper: ImplicitFixedStepper::new(step_size)
        };
    }

    pub fn solve<F>(self: &Self, prob: &F) -> Result<(Vec<T>, Vec<Vector<T>>), ()>
        where F: ImplicitODE<T>
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

impl<T> ImplicitFixedStepSizeMethod<T> for BackwardEuler<T>
    where T: Real
{
    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> Vector<T>
        where F: ImplicitODE<T>
    {
        let y_n = prob.func(t_n, x_n);

        return y_n.apply(&|y: &T| -> T {return (-T::one() + (T::one() + T::from_f64(4.0) * *y).sqrt())/(T::from_f64(2.0) *
        *h)});

    }

    /// Euler's method is a first order method
    fn order(self: &Self) -> u8
    {
        return 1;
    }
}