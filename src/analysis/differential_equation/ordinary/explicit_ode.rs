//! Explicit ODE
use crate::algebra::linear::Vector;

/// Explicit ODE algorithm interface
///
/// This trait has to be implemented by every ODE which shall be solved with
/// and explicit ODE solving algorithm.
pub trait ExplicitODE<T>
{
    fn func(self: &Self, t: &T, x: &Vector<T>) -> Vector<T>;
    fn time_span(self: &Self) -> (T, T);
    fn init_cond(self: &Self) -> Vector<T>;
}
