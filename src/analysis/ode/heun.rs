use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::explicit_method::{ExplicitFixedStepSizeMethod};
use super::ExplicitODE;
use crate::analysis::ode::fixed_stepper::ExplicitFixedStepper;

/// Solves an ordinary differential equation using Heun's method.
///
/// <a href="https://en.wikipedia.org/wiki/Heun's_method">https://en.wikipedia.org/wiki/Heun's_method</a>
pub struct Heun<T>
{
    stepper: ExplicitFixedStepper<T>
}


impl<T> Heun<T>
    where T: Real
{
    /// Creates a Heun instance with step size 'step_size'
    ///
    pub fn new(step_size: T) -> Heun<T>
    {
        return Heun
        {
            stepper: ExplicitFixedStepper::new(step_size)
        }
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

impl<T> ExplicitFixedStepSizeMethod<T> for Heun<T>
    where T: Real
{
     fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> Vector<T>
        where F: ExplicitODE<T>
     {
            let k_1: Vector<T> = prob.func(t_n, x_n);
            let k_2: Vector<T> = prob.func(&(*t_n + *h), &(x_n + &(k_1.clone() * *h)));

            return x_n.clone() + (&k_1 + &k_2) * *h / T::from_f64(2.0);
    }

    ///
    fn order(self: &Self) -> u8
    {
        return 2;
    }
}