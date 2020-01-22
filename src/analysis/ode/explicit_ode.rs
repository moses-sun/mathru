//! Explicit Ordinary Differential Equation

use crate::algebra::linear::{Vector};


/// Explicit ordinary differential equation
///
/// $`x^{n}(t) = f(t, x(t), x^{'}(t), \dots, x^{n-1}(t))`$
pub trait ExplicitODE<T>
{
    fn func(self: &Self, t: &T, x: &Vector<T>) -> Vector<T>;
    fn time_span(self: &Self) -> (T, T);
    fn init_cond(self: &Self) -> Vector<T>;
}


