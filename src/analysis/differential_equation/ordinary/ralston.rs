//! Solves an ODE using Ralston's method.
use super::{explicit_method::ExplicitFixedStepSizeMethod, ExplicitODE};
use crate::{
    algebra::{abstr::Real, linear::Vector},
    analysis::differential_equation::ordinary::fixed_stepper::ExplicitFixedStepper,
};

/// Solves an ODE using Ralston's method.
///
/// Ralston's method is a second-order metho
///
/// <a href="https://en.wikipedia.org/wiki/List_of_Runge-Kutta_methods#Forward_Euler">https://en.wikipedia.org/wiki/List_of_Runge-Kutta_methods#Forward_Euler</a>
pub struct Ralston<T>
{
    stepper: ExplicitFixedStepper<T>,
}

impl<T> Ralston<T> where T: Real
{
    /// Creates a Ralstons instance with step size 'step_size'
    pub fn new(step_size: T) -> Ralston<T>
    {
        return Ralston { stepper: ExplicitFixedStepper::new(step_size) };
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

impl<T> ExplicitFixedStepSizeMethod<T> for Ralston<T> where T: Real
{
    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> Vector<T>
        where F: ExplicitODE<T>
    {
        let k_1: Vector<T> = prob.func(&t_n, &x_n);
        let factor: T = *h * T::from_f64(2.0 / 3.0);
        let k_2: Vector<T> = prob.func(&(*t_n + factor), &(x_n + &(&k_1 * &factor)));

        // Update
        return x_n + &(((k_1 * T::from_f64(1.0 / 4.0)) + (k_2 * T::from_f64(3.0 / 4.0))) * *h);
    }

    /// Ralston's method is a 3rd order method
    fn order(self: &Self) -> u8
    {
        return 3;
    }
}
