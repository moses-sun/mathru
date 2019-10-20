use crate::algebra::linear::{Vector, Matrix};
use crate::algebra::abstr::Real;

pub trait Hessian<T>
    where T: Real
{

    fn hessian(self: &Self, input: &Vector<T>) -> Matrix<T>;
}