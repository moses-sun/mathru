//! Magma
use std::cmp::PartialEq;
use super::operator::{Addition, Multiplication};
use super::operator::Operator;
use std::ops::{Add, Mul};

/// A Magma is a pair $`(\mathbb{M}, \circ)`$, composed by a set $`\mathbb{M}`$ and a binary inner operation $`\circ`$:
/// # Definition
///
/// ```math
/// \circ: \mathbb{M} \times \mathbb{M} \rightarrow \mathbb{M} , (x, y) \mapsto x \circ y
/// ```
pub trait Magma<O: Operator>: Sized + PartialEq + Clone
{
    /// binary operation
    fn operate(self, rhs: Self) -> Self;

}

impl<T> Magma<Addition> for T
	where T: Add<T, Output=T> + PartialEq + Clone
{
	fn operate(self: Self, rhs: Self) -> Self
    {
   		return self + rhs;
    }
}

impl<T> Magma<Multiplication> for T
	where T: Mul<T, Output=T> + PartialEq + Clone
{
	fn operate(self: Self, rhs: Self) -> Self
    {
   		return self * rhs;
    }
}

