/// Field

use crate::algebra::abstr::Ring;
use std::ops::{Sub, SubAssign, Div, DivAssign};


/// Field
///
/// <a href="https://en.wikipedia.org/wiki/Field_(mathematics)">https://en.wikipedia.org/wiki/Field_(mathematics)</a>
///
pub trait Field : Ring + Sub<Self, Output = Self> + SubAssign<Self> + Sign + Div<Self, Output = Self> +
DivAssign<Self> + Abs
{
	fn epsilon() -> Self;
}


/// Sign trait
///
pub trait Sign
{
	/// Returns the sign of a number
	fn sgn(self: &Self) -> Self;
}

///
/// Abs trait
///
pub trait Abs
{
	/// returns the absolute value of a field
	fn abs(self: &Self) -> Self;
}

