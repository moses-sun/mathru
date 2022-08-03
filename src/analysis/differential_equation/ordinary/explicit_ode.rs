//! Explicit Ordinary Differential Equation

use crate::algebra::{abstr::Real, linear::Vector};

/// Explicit ordinary differential equation
pub trait ExplicitODE<T>
where
    T: Real,
{
    fn ode(&self, t: &T, x: &Vector<T>) -> Vector<T>;
}
