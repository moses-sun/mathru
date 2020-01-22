use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::{ExplicitODE, ExplicitFixedStepSizeMethod};
use std::marker::PhantomData;

/// Solves an ordinary differential equation using midpoint method.
///
/// <a href="https://en.wikipedia.org/wiki/Midpoint_method">https://en.wikipedia.org/wiki/Midpoint_method</a>
pub struct Midpoint<T>
{
    phantom: PhantomData<T>,
}

impl<T> Midpoint<T>
    where T: Real
{
    /// Creates a Euler instance with step size 'step_size'
    ///
    pub fn new() -> Midpoint<T>
    {
        return Midpoint
        {
            phantom: PhantomData
        }
    }
}

impl<T> ExplicitFixedStepSizeMethod<T> for Midpoint<T>
    where T: Real
{
     fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> (Vector<T>)
        where F: ExplicitODE<T>
     {
        let x_n_1_2: Vector<T> = x_n + &(&prob.func(t_n, x_n) * &(*h / T::from_f64(2.0).unwrap()));
        return x_n + &(&prob.func(&(*t_n + *h / T::from_f64(2.0).unwrap()), &x_n_1_2) * h);
    }
}
