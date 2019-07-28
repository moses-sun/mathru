use crate::algebra::linear::Vector;

/// Trait for models which can be gradient-optimized.
pub trait Optimizable
{
    /// The input data type to the model.
    type Input;

    /// The target data type to the model.
    type Target;

    /// Compute the gradient for the model.
    fn value(&self, param: &Vector<f64>, input: &Self::Input, target: &Self::Target) -> f64;

    fn gradient(&self, param: &Vector<f64>, input: &Self::Input, target: &Self::Target) -> Vector<f64>;
}

