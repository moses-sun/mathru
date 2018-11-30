

pub trait Hyperbolic
{
	/// Hyperbolic sine
	fn sinh(self: Self) -> Self;

	/// Hyperbolic cosine
	fn cosh(self: Self) -> Self;

	/// Hyperbolic tangens
	fn tanh(self: Self) -> Self;

	/// Hyperbolic cotangens
	fn coth(self: Self) -> Self;

	/// Hyperbolic secant
	fn sech(self: Self) -> Self;

	/// Hyperbolic cosecant
	fn csch(self: Self) -> Self;

	/// Hyperbolic inverse sine
	fn arsinh(self: Self) -> Self;

	/// Hyperbolic inverse cosine
	fn arcosh(self: Self) -> Self;

	/// Hyperbolic inverse tangens
	fn artanh(self: Self) -> Self;

	/// Hyperbolic inverse cosecant
	fn arcoth(self: Self) -> Self;

	/// Hyperbolic inverse secant
	fn arsech(self: Self) -> Self;

	// Hyperbolic inverse cosecant
	fn arcsch(self: Self) -> Self;
}