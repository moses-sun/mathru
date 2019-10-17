use crate::algebra::linear::{Vector, Matrix};
use crate::algebra::abstr::Real;

pub trait Jacobian<T>
    where T: Real
{
    fn eval(self: &Self, input: &Vector<T>) -> Vector<T>;
    fn jacobian(self: &Self, input: &Vector<T>) -> Matrix<T>;
}