//! AbelianGroup
use super::operator::{Operator, Addition, Multiplication};
use super::monoid::Monoid;
use super::Loop;


/// An Abelian group is a commutative group.
///
/// A Group is a triple $`(\mathbb{A}, \circ, e)`$, composed by a set $`\mathbb{A}`$ and a binary inner operation $`\circ`$
/// and the element $`e \in \mathbb{A}`$
///
/// # Definition
/// ```math
/// \circ: \mathbb{A} \times \mathbb{A} \rightarrow \mathbb{A} , (x, y) \mapsto x \circ y
/// ```
/// 1. Closure
/// $`\forall x, y \in \mathbb{A},: x \circ y \in \mathbb{A}`$
/// 2. associativity <br>
/// $`\forall x, y, z \in \mathbb{A}`$: $`x \circ (y \circ z) = (x \circ y) \circ z`$
/// 3. $`e`$ neutral element(identity) <br>
/// $`\forall x \in \mathbb{A}`$: $`x \circ e = e \circ x = x`$
/// 4. Inverse element
/// $`x^{-1} \in \mathbb{A}: x^{-1} \circ x = x \circ x^{-1} = e`$
/// 5. Commutativity
/// $`\forall x, y, \in \mathbb{A}: x \circ y = y \circ x`$
pub trait AbelianGroup<O: Operator>: Loop<O> + Monoid<O>
{

}

macro_rules! impl_abeliangroup(
    ($T:ty, $($S:ty),*) =>
    {
        $(
        impl AbelianGroup<$T> for $S
        {
        }
        )*
    }
);

impl_abeliangroup!(Addition, i8, i16, i32, i64, i128, f32, f64);
impl_abeliangroup!(Multiplication, f32, f64);
