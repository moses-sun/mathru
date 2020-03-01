use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::explicit_method::{ExplicitFixedStepSizeMethod};
use super::ExplicitODE;
use crate::analysis::ode::fixed_stepper::ExplicitFixedStepper;

/// Solves an ordinary differential equation using the 3th order Runge-Kutta algorithm.
///
///<a href="https://en.wikipedia.org/wiki/Runge-Kutta_methods">https://en.wikipedia
/// .org/wiki/Rung-Kutta_methods</a>
pub struct Kutta3<T>
{
    stepper: ExplicitFixedStepper<T>
}

impl<T> Kutta3<T>
    where T: Real
{
    /// Creates a Kutta3 instance with step size 'step_size'
    ///
    pub fn new(step_size: T) -> Kutta3<T>
    {
        return Kutta3
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

impl<T> ExplicitFixedStepSizeMethod<T> for Kutta3<T>
    where T: Real
{
    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> Vector<T>
        where F: ExplicitODE<T>
    {
        // k1 = f(t_n, x_n)
        let k1: Vector<T> = prob.func(&t_n, &x_n);

        // k2 = func(t_n + h / 2, x_n + h / 2 k1)
        let k2: Vector<T> = prob.func(&(*t_n + (*h / T::from_f64(2.0))), &(x_n + &((&k1 * h) / T::from_f64(2.0))));

        // k3 = func(t_n + h, x_n + - h k1 + 2h *k2)
        let k3: Vector<T> = prob.func(&(*t_n + *h), &(x_n + &(&(&(&k2 * &T::from_f64(2.0)) - &k1) * h))) ;

        // x[n+1] = xn + h*(k1 + 4*k2 + k3)/6
        return x_n + &((k1 + k2 * T::from_f64(4.0) + k3) * *h / T::from_f64(6.0));
    }

    /// Kuttas method is a 3rd order method
    fn order(self: &Self) -> u8
    {
        return 3;
    }
}
