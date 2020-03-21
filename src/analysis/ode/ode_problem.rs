//! Explicit Ordinary Differential Equation

use crate::algebra::linear::{Vector, Matrix};
use crate::algebra::abstr::Real;

///
pub trait ODEProblem<T>
    where T: Real
{
    fn time_span(self: &Self) -> (T, T);

    fn init_cond(self: &Self) -> Vector<T>;

    fn eval(&self, t: &T, _x: &Vector<T>) -> Vector<T>
    {
        unimplemented!();
    }

    // Computes the Jacobian at the given x
    fn jacobian(&self, t: &T, _x: &Vector<T>) -> Matrix<T>
    {
        unimplemented!();
    }

    /// Computes the Hessian at the given value x
    fn hessian(&self, t: &T, _x: &Vector<T>) -> Matrix<T>
    {
        unimplemented!();
    }
}