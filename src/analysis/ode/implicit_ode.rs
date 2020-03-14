//! Explicit Ordinary Differential Equation

use crate::algebra::linear::{Vector};
use crate::analysis::{Function, Jacobian};
use crate::algebra::abstr::Real;

/// Implicit ordinary differential equation
///
/// $`0 = f(t, x(t), x^{'}(t), \dots, x^{n}(t))`$
pub trait ImplicitODE<T> : Jacobian<T>
    where T: Real
{
    fn func(self: &Self, t: &T, x: &Vector<T>) -> Vector<T>;
    fn time_span(self: &Self) -> (T, T);
    fn init_cond(self: &Self) -> Vector<T>;
}


