use crate::algebra::linear::{Vector};

pub trait Hessian
{
    /// The input data type to the model.
    type Input;

    /// The target data type to the model.
    type Target;

    fn value(&self, param: &Vector<f64>, input: &Self::Input, target: &Self::Target) -> f64;

    fn hessian(&self, param: &Vector<f64>, input: &Self::Input, target: &Self::Target) -> Vector<f64>;
}