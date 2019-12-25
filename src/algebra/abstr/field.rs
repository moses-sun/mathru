/// Field

use crate::algebra::abstr::{Operator, Addition, Multiplication, AbelianGroup, CommutativeRing};
use std::{f32, f64};

/// Field
///
/// A field is a commutative ring, and an Abelian group under both operators.
///
///
pub trait Field<A: Operator = Addition, M: Operator = Multiplication>:
    CommutativeRing<A, M> + AbelianGroup<M>
{

}

macro_rules! impl_field
{
    ($($t:ty),*) =>
    {
        $(
        impl Field for $t
        {

        }
        )*
    };
}

impl_field!(f32, f64);
