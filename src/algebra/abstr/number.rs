use std::fmt::Display;
use std::fmt::Debug;
use crate::algebra::abstr::cast::{ToPrimitive , FromPrimitive, NumCast};



/// comparisons, basic numeric operations, and string conversion.
pub trait Number: Sized + PartialEq<Self> + PartialOrd + Display + ToPrimitive + FromPrimitive + NumCast + Copy +
Clone +
Debug
{
}

