use std::fmt::Display;
use std::fmt::Debug;
use crate::algebra::abstr::cast::{ToPrimitive , FromPrimitive, NumCast};
use crate::algebra::abstr::Sign;


/// comparisons, basic numeric operations, and string conversion.
pub trait Scalar<Rhs = Self, Output = Self>: Sized + Display + ToPrimitive + FromPrimitive + NumCast + Debug + Copy + PartialOrd + Sign
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

impl_scalar!(/*u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, */ f32, f64);
