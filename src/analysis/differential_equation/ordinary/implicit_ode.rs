//! Implicit Ordinary Differential Equation

use crate::algebra::{
    abstr::Real,
    linear::{Matrix, Vector},
};

/// Implicit ordinary differential equation
pub trait ImplicitODE<T>
    where T: Real
{
    fn func(self: &Self, t: &T, x: &Vector<T>) -> Vector<T>;

    fn jacobian(self: &Self, t: &T, x: &Vector<T>) -> Matrix<T>;

    fn time_span(self: &Self) -> (T, T)
    {
        unimplemented!();
    }

    fn init_cond(self: &Self) -> Vector<T>
    {
        unimplemented!();
    }
}
