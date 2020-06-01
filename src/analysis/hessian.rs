use crate::algebra::abstr::Real;
use crate::algebra::linear::{Matrix, Vector};

pub trait Hessian<T>
    where T: Real
{
    fn hessian(self: &Self, input: &Vector<T>) -> Matrix<T>;
}
