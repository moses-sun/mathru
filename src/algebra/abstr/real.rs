use super::field::Field;
use elementary::{Exponential, Trigonometry, Power, Hyperbolic};


/// Real number
///
///<a href="https://en.wikipedia.org/wiki/Real_number">https://en.wikipedia.org/wiki/Real_number</a>
pub trait Real: Field + Exponential + Trigonometry + Power + Hyperbolic
{
	/// Returns the smallest integer greater than or equal to a number.
	fn ceil(self: &Self) -> Self;

	/// Returns the largest integer less than or equal to a number.
	fn floor(self: &Self) -> Self;
}