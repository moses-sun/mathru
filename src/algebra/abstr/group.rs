//! Group
use super::operator::{Operator, Addition, Multiplication};
use super::monoid::{Monoid, MonoidAdd, MonoidMul};
use super::Loop;
use std::ops::{Sub, SubAssign, Neg, Div, DivAssign};

/// A Group is a triple $`(\mathbb{M}, \circ, e)`$, composed by a set $`\mathbb{M}`$ and a binary inner operation $`\circ`$
/// and the element $`e \in \mathbb{M}`$
/// # Definition
///
/// ```math
/// \circ: \mathbb{M} \times \mathbb{M} \rightarrow \mathbb{M} , (x, y) \mapsto x \circ y
/// ```
/// 1. associativity <br>
/// $`\forall x, y, z \in \mathbb{M}`$: $`x \circ (y \circ z) = (x \circ y) \circ z`$
/// 2. $`e`$ neutral element(identity) <br>
/// $`\forall x \in \mathbb{M}`$: $`x \circ e = e \circ x = x`$
/// 3.
/// $`x^-1 \in \mathbb{M}: x^⁻1 \circ x = x \circ x^-1 ` = e$
pub trait Group<O: Operator>: Loop<O> + Monoid<O>
{

}

macro_rules! impl_group(
    ($T:ty, $($S:ty),*) =>
    {
        $(
        impl Group<$T> for $S
        {
        }
        )*
    }
);

impl_group!(Addition, i8, i16, i32, i64, i128, f32, f64);
impl_group!(Multiplication, f32, f64);

pub trait GroupAdd: Group<Addition> + MonoidAdd + Sub + SubAssign + Neg<Output = Self>
{

}

macro_rules! impl_groupadd
(
    ($($T:ty),*) =>
    {
        $(
            impl GroupAdd for $T
            {

            }
        )*
    }
);

impl_groupadd!(i8, i16, i32, i64, i128, f32, f64);


pub trait GroupMul: Group<Multiplication> + MonoidMul + Div + DivAssign
{

}

macro_rules! impl_groupmul
(
    ($($T:ty),*) =>
    {
        $(
            impl GroupMul for $T
            {

            }
        )*
    }
);

impl_groupmul!(f32, f64);