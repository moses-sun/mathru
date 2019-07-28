use crate::optim::Optimizable;
use crate::algebra::linear::Vector;

/// Trait for optimization algorithms.
///
/// This trait is implemented by every optimization algorithm
pub trait OptimAlgorithm<M: Optimizable>
{
    /// Takes in a set of starting parameters and related model data.
    fn minimize(self: &Self, model: &M, param_start: &Vector<f64>, input: &M::Input, targets: &M::Target) ->
    Vector<f64>;
}