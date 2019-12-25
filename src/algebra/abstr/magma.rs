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

macro_rules! impl_magma
(
    ($O:ty; $op: ident; $($T:ty),*) =>
    {
        $(
            impl Magma<$O> for $T
            {
                fn operate(self, rhs: Self) -> Self
                {
                    self.$op(rhs)
                }
            }
        )*
    }
);

impl_magma!(Addition; add; u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
impl_magma!(Multiplication; mul; u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
