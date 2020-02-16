
/// Power functions
///
///<a href="https://en.wikipedia.org/wiki/Exponentiation#Power_functions">https://en.wikipedia.org/wiki/Exponentiation#Power_functions</a>
pub trait Power
{
	fn pow(self: &Self, exp: &Self) -> Self;

	fn root(self: &Self, root: &Self) -> Self;

	fn sqrt(self: &Self) -> Self;
}