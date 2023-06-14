use crate::algebra::{
    abstr::Real,
    linear::{matrix::General, Vector},
};

pub trait Optim<T>
where
    T: Real,
{
    fn eval(&self, _x: &Vector<T>) -> Vector<T> {
        unimplemented!();
    }

    // Computes the Jacobian at the given x
    fn jacobian(&self, _x: &Vector<T>) -> General<T> {
        unimplemented!();
    }

    /// Computes the Hessian at the given value x
    fn hessian(&self, _x: &Vector<T>) -> General<T> {
        unimplemented!();
    }
}
