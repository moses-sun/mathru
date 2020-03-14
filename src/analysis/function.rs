use crate::algebra::linear::{Vector, Matrix};
use crate::algebra::abstr::Real;

pub trait Function<Domain>
{
    type Codomain;
    fn eval(self: &Self, input: &Domain) -> Self::Codomain;
}