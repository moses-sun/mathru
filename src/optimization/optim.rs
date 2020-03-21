
use crate::algebra::linear::{Vector, Matrix};
use crate::algebra::abstr::Real;

pub trait Optim<T>
    where T: Real
{

    fn eval(&self, _x: &Vector<T>) -> Vector<T>
    {
        unimplemented!();
    }

    // Computes the Jacobian at the given x
    fn jacobian(&self, _x: &Vector<T>) -> Matrix<T>
    {
        unimplemented!();
    }

    /// Computes the Hessian at the given value x
    fn hessian(&self, _x: &Vector<T>) -> Matrix<T>
    {
        unimplemented!();
    }
}