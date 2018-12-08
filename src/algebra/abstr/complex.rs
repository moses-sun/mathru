use super::field::Field;
use elementary::{Exponential, Trigonometry, Power, Hyperbolic};

/// Complex number
///
///<a href="https://en.wikipedia.org/wiki/Complex_number">https://en.wikipedia.org/wiki/Complex_numberr</a>
pub trait Complex: Field + Exponential + Trigonometry + Power + Hyperbolic
{
	/// Returns the complex conjuagte
	/// conj(self) = Re(self) - i Im(self)
	fn conj(self: Self) -> Self;

	fn arg(self: Self) -> Self;
}