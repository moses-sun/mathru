use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::{ExplicitFixedStepSizeMethod, ExplicitODE};
use std::marker::PhantomData;

/// Solves an ordinary differential equation using Ralston's method.
///
/// Ralston's method is a second-order metho
///
/// <a href="https://en.wikipedia.org/wiki/List_of_Runge-Kutta_methods#Forward_Euler">https://en.wikipedia.org/wiki/List_of_Runge-Kutta_methods#Forward_Euler</a>
pub struct Ralston<T>
{
    phantom: PhantomData<T>
}

impl<T> Ralston<T>
    where T: Real
{
    /// Creates a Ralstons instance with step size 'step_size'
    ///
    pub fn new() -> Ralston<T>
    {
        Ralston
        {
            phantom: PhantomData
        }

    }
}

impl<T> ExplicitFixedStepSizeMethod<T> for Ralston<T>
    where T: Real
{
    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> (Vector<T>)
        where F: ExplicitODE<T>
    {
        let k_1: Vector<T> = prob.func(&t_n, &x_n);
        let factor: T = *h * T::from_f64(2.0/3.0).unwrap();
        let k_2: Vector<T> = prob.func(&(*t_n + factor), &(x_n + &(&k_1 * &factor)));

        // Update
        return x_n + &(((k_1 * T::from_f64(1.0/4.0).unwrap()) + (k_2 * T::from_f64(3.0/4.0).unwrap())) * *h);
    }
}