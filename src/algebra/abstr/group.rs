//! Group
use super::operator::{Operator, Addition};
use super::monoid::Monoid;
use super::Loop;

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

impl_group!(Addition, i8, i16, i32, i64, i128);