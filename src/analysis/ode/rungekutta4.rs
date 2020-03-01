use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::explicit_method::{ExplicitFixedStepSizeMethod};
use super::ExplicitODE;
use crate::analysis::ode::fixed_stepper::ExplicitFixedStepper;

/// Solves an ordinary differential equation using the 4th order Runge-Kutta algorithm.
///
///<a href="https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta_methods">https://en.wikipedia
/// .org/wiki/Rung-Kutta_methods</a>
pub struct RungeKutta4<T>
{
    stepper: ExplicitFixedStepper<T>
}

impl<T> RungeKutta4<T>
    where T: Real
{
    /// Creates a RungeKutta4 instance
    ///
    pub fn new(step_size: T) -> RungeKutta4<T>
    {
        return RungeKutta4
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

impl<T> ExplicitFixedStepSizeMethod<T> for RungeKutta4<T>
    where T: Real
{
    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> Vector<T>
        where F: ExplicitODE<T>
    {
           // k1 = f(t_n, x_n)
            let k1: Vector<T> = prob.func(t_n, x_n);

            // k2 = func(t_n + h / 2, x_n + h / 2 k1)
            let k2: Vector<T> = prob.func(&(*t_n + (*h / T::from_f64(2.0))), &(x_n + &((&k1 * h) /
            T::from_f64(2.0))));

            // k3 = func(t_n + h / 2, x_n + h / 2 * k2)
            let k3: Vector<T> = prob.func(&(*t_n + *h / T::from_f64(2.0)), &(x_n + &(&k2 * &(*h / T::from_f64(2.0))))) ;

            // k4 = h * func(t_n + h, x_n + h * k3)
            let k4: Vector<T> = prob.func(&(*t_n + *h), &(x_n + &(&k3 * h)));

            // x[n+1] = x[n] + h*(k1 + 2*k2 + 2*k3 + k4)/6
            return x_n + &((k1 + ((k2 + k3) * T::from_f64(2.0)) + k4) * *h / T::from_f64(6.0));
    }

    /// Runge-Kutta 4 is a fourth order method
    fn order(self: &Self) -> u8
    {
        return 4;
    }
}