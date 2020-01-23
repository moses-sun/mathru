use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::{ExplicitODE, ExplicitFixedStepSizeMethod};
use std::marker::PhantomData;

/// Solves an ordinary differential equation using the 3th order Runge-Kutta algorithm.
///
///<a href="https://en.wikipedia.org/wiki/Runge-Kutta_methods">https://en.wikipedia
/// .org/wiki/Rung-Kutta_methods</a>
pub struct Kutta3<T>
{
    phantom: PhantomData<T>,
}

impl<T> Kutta3<T>
    where T: Real
{
    /// Creates a Kutta3 instance with step size 'step_size'
    ///
    pub fn new() -> Kutta3<T>
    {
        Kutta3
        {
            phantom: PhantomData
        }
    }
}

impl<T> ExplicitFixedStepSizeMethod<T> for Kutta3<T>
    where T: Real
{
    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> (Vector<T>)
        where F: ExplicitODE<T>
    {
        // k1 = f(t_n, x_n)
        let k1: Vector<T> = prob.func(&t_n, &x_n);

        // k2 = func(t_n + h / 2, x_n + h / 2 k1)
        let k2: Vector<T> = prob.func(&(*t_n + (*h / T::from_f64(2.0).unwrap())), &(x_n + &((&k1 * h) / T::from_f64(2.0).unwrap())));

        // k3 = func(t_n + h, x_n + - h k1 + 2h *k2)
        let k3: Vector<T> = prob.func(&(*t_n + *h), &(x_n + &(&(&(&k2 * &T::from_f64(2.0).unwrap()) - &k1) * h))) ;

        // x[n+1] = xn + h*(k1 + 4*k2 + k3)/6
        return x_n + &((k1 + k2 * T::from_f64(4.0).unwrap() + k3) * *h / T::from_f64(6.0).unwrap());
    }
}
