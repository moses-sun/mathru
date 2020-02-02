use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::explicit_method::{ExplicitFixedStepSizeMethod};
use super::ExplicitODE;
use crate::analysis::ode::fixed_stepper::FixedStepper;

/// Solves an ordinary differential equation using Euler's method.
///
/// <a href="https://en.wikipedia.org/wiki/Euler_method">https://en.wikipedia.org/wiki/Euler_method</a>
pub struct Euler<T>
{
    stepper: FixedStepper<T>
}

impl<T> Euler<T>
    where T: Real
{
    /// Creates a Euler instance
    ///
    pub fn new(step_size: T) -> Euler<T>
    {
        return Euler
        {
            stepper: FixedStepper::new(step_size)
        };
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

impl<T> ExplicitFixedStepSizeMethod<T> for Euler<T>
    where T: Real
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