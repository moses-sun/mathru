//! Semigroup
use super::magma::Magma;
use super::operator::{Operator, Addition, Multiplication};

/// A Semigroup is a pair $`(\mathbb{S}, \circ)`$, composed by a set $`\mathbb{S}`$ and a binary inner operation $`\circ`$:
/// # Definition
///
/// ```math
/// \circ: \mathbb{S} \times \mathbb{S} \rightarrow \mathbb{S} , (x, y) \mapsto x \circ y
/// ```
/// and is associative
/// $`x, y, z \in \mathbb{S}`$
/// $`x \circ (y \circ z) = (x \circ y) \circ z`$
pub trait Semigroup<O: Operator + Copy>: Magma<O>
{
    fn is_associative(self: Self, y: Self, z: Self) -> bool
    {
        return self.clone().operate(y.clone()).operate(z.clone()) == self.operate(y.operate(z));
    }
}

/// Blanket implementation
impl<T> Semigroup<Addition> for T
    where T: Magma<Addition>
{
}

/// Blanket implementation
impl<T> Semigroup<Multiplication> for T
    where T: Magma<Multiplication>
{
}

