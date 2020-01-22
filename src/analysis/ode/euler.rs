use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::{ExplicitFixedStepSizeMethod};
use super::ExplicitODE;
use std::marker::PhantomData;

/// Solves an ordinary differential equation using Euler's method.
///
/// <a href="https://en.wikipedia.org/wiki/Euler_method">https://en.wikipedia.org/wiki/Euler_method</a>
pub struct Euler<T>
{
    phantom: PhantomData<T>,
    //step_size: T
}

impl<T> Euler<T>
    where T: Real
{
    /// Creates a Euler instance with step size 'step_size'
    ///
    /// # Argument
    ///
    /// * 'step_size'
    ///
    /// # Panics
    ///
    /// 'step_size' <= 0.0
    ///
    pub fn new() -> Euler<T>
    {
//        if step_size <= T::zero()
//        {
//            panic!();
//        }
        Euler
        {
            phantom: PhantomData
            //step_size: step_size,
        }
    }
}

impl<T> ExplicitFixedStepSizeMethod<T> for Euler<T>
    where T: Real
{
     fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> (Vector<T>)
        where F: ExplicitODE<T>
     {
        return x_n + &(&prob.func(t_n, x_n) * h);
    }
}