use std::fmt::Display;
use algebra::abstr::cast::{ToPrimitive /*, FromPrimitive, NumCast*/};



/// comparisons, basic numeric operations, and string conversion.
pub trait Number: Sized + PartialEq + PartialOrd + Display + ToPrimitive + Copy + Clone
{

}