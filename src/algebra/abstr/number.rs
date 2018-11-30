use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign};
//use algebra::abstr::identity::{Zero, One};
//use std::clone::Clone;
use std::fmt::Display;
use algebra::abstr::cast::{ToPrimitive, FromPrimitive, NumCast};



/// The base trait for numeric types, covering `0` and `1` values,
/// comparisons, basic numeric operations, and string conversion.
pub trait Number: Sized + PartialEq + PartialOrd + Display + ToPrimitive + Copy + Clone
{

	fn min(self: Self, a: Self) -> Self;

	fn max(self: Self, a: Self) -> Self;
}

/// The trait for types implementing basic numeric operations
///
/// This is automatically implemented for types which implement the operators.
pub trait Field<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
{
}

impl<T, Rhs, Output> Field<Rhs, Output> for T
    where  T: Add<Rhs, Output = Output>
            + Sub<Rhs, Output = Output>
            + Mul<Rhs, Output = Output>
            + Div<Rhs, Output = Output>
            + Rem<Rhs, Output = Output>,
{

}

/// The trait for types implementing numeric assignment operators (like `+=`).
///
/// This is automatically implemented for types which implement the operators.
pub trait NumberAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{
}

impl<T, Rhs> NumberAssignOps<Rhs> for T
    where   T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>,
{
}

// The trait for `Num` types which also implement assignment operators.
//
// This is automatically implemented for types which implement the operators.
//pub trait NumberAssign: Number + NumberAssignOps {}
//
//impl<T> NumberAssign for T
//where
//    T: Number + NumberAssignOps,
//{
//}