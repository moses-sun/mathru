use std::fmt::Display;
use std::fmt::Debug;
use crate::algebra::abstr::cast::{ToPrimitive , FromPrimitive, NumCast};
use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use super::{Zero, One};


/// comparisons, basic numeric operations, and string conversion.
pub trait Scalar<Rhs = Self, Output = Self>: Sized + ScalarOps + PartialEq<Rhs> + PartialOrd + Display + ToPrimitive +
FromPrimitive +
NumCast +
Copy +
Clone + Debug
+ Zero + One
{
}

macro_rules! impl_scalar
{
    ($($t:ty),+) =>
    {
    	$(
        impl Scalar for $t
        {

        }
        )*
    };
}

impl_scalar!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);


/// The trait for types implementing basic numeric operations
pub trait ScalarOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    //+ Rem<Rhs, Output = Output>
    + AddAssign<Rhs>
    + SubAssign<Rhs>
    + MulAssign<Rhs>
    + DivAssign<Rhs>
    //+ RemAssign<Rhs>
{
}


macro_rules! impl_scalar_ops
{
    ($($t:ty),+) =>
    {
    	$(
        impl ScalarOps for $t
        {

        }
        )*
    };
}

impl_scalar_ops!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);


