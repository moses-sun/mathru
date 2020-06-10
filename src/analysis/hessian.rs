use crate::algebra::{
    abstr::Real,
    linear::{Matrix, Vector},
};

pub trait Hessian<T>
    where T: Real
{
    fn hessian(self: &Self, input: &Vector<T>) -> Matrix<T>;
}
