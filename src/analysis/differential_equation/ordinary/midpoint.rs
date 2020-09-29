//! Solves an ODE using midpoint method.
use super::{explicit_method::ExplicitFixedStepSizeMethod, ExplicitODE};
use crate::{
    algebra::{abstr::Real, linear::Vector},
    analysis::differential_equation::ordinary::fixed_stepper::ExplicitFixedStepper,
};
use serde::{Deserialize, Serialize};
use std::clone::Clone;

/// Solves an ODE using midpoint method.
///
/// <a href="https://en.wikipedia.org/wiki/Midpoint_method">https://en.wikipedia.org/wiki/Midpoint_method</a>
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Midpoint<T>
{
    stepper: ExplicitFixedStepper<T>,
}

impl<T> Midpoint<T> where T: Real
{
    /// Creates a Euler instance with step size 'step_size'
    pub fn new(step_size: T) -> Midpoint<T>
    {
        return Midpoint { stepper: ExplicitFixedStepper::new(step_size) };
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

impl<T> ExplicitFixedStepSizeMethod<T> for Midpoint<T> where T: Real
{
    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> Vector<T>
        where F: ExplicitODE<T>
    {
        let x_n_1_2: Vector<T> = x_n + &(&prob.func(t_n, x_n) * &(*h / T::from_f64(2.0)));
        return x_n + &(&prob.func(&(*t_n + *h / T::from_f64(2.0)), &x_n_1_2) * h);
    }

    /// The mitdpoint methods is a 2nd order method
    fn order(self: &Self) -> u8
    {
        return 2;
    }
}
