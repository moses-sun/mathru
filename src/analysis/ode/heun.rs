use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::{ExplicitFixedStepSizeMethod, ExplicitODE};
use std::marker::PhantomData;

/// Solves an ordinary differential equation using Heun's method.
///
/// <a href="https://en.wikipedia.org/wiki/Heun's_method">https://en.wikipedia.org/wiki/Heun's_method</a>
pub struct Heun<T>
{
    phantom: PhantomData<T>
}


impl<T> Heun<T>
    where T: Real
{
    /// Creates a Heun instance with step size 'step_size'
    ///
    pub fn new() -> Heun<T>
    {
        return Heun
        {
            phantom: PhantomData
        }
    }
}

impl<T> ExplicitFixedStepSizeMethod<T> for Heun<T>
    where T: Real
{
     fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> (Vector<T>)
        where F: ExplicitODE<T>
     {
            let k_1: Vector<T> = prob.func(t_n, x_n);
            let k_2: Vector<T> = prob.func(&(*t_n + *h), &(x_n + &(k_1.clone() * *h)));

            return x_n.clone() + (&k_1 + &k_2) * *h / T::from_f64(2.0).unwrap();
    }
}