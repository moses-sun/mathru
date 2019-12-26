use std::fmt::Display;
use std::fmt::Debug;
use crate::algebra::abstr::cast::{ToPrimitive , FromPrimitive, NumCast};
use crate::algebra::abstr::Sign;
#[cfg(feature = "blaslapack")]
use crate::algebra::abstr::{Blas, Lapack};

/// comparisons, basic numeric operations, and string conversion.
#[cfg(feature = "native")]
pub trait Scalar<Rhs = Self, Output = Self>: Sized + Display + ToPrimitive + FromPrimitive + NumCast + Debug + Copy + PartialOrd + Sign
{
    fn epsilon() -> Self;
}

/// comparisons, basic numeric operations, and string conversion.
#[cfg(feature = "blaslapack")]
pub trait Scalar<Rhs = Self, Output = Self>: Sized + Display + ToPrimitive + FromPrimitive + NumCast + Debug + Copy + PartialOrd + Sign +
 Blas + Lapack
{
    fn epsilon() -> Self;
}

macro_rules! impl_scalar
{
    ($t:ty, $eps:expr) =>
    {
        impl Scalar for $t
        {
            fn epsilon() -> Self
            {
                return $eps;
            }
        }
    };
}

impl_scalar!(/*u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, */ f32, std::f32::EPSILON);
impl_scalar!(/*u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, */ f64, std::f64::EPSILON);
