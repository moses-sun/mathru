use crate::algebra::linear::{Vector};
use crate::optim::{Optimizable};


/// Batch Gradient Descent algorithm
pub trait GradientDesc<M: Optimizable>
{
    fn minimize(self: &Self, model: &M, param_start: &Vector<f64>, inputs: &M::Input, targets: &M::Target) ->
    M::Target;
}

