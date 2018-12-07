

pub trait Trigonometry
{
	/// Returns the mathematic constant PI
	fn pi() -> Self;

	/// Sinus function
	fn sin(self: &Self) -> Self;

	/// Cosinus function
	fn cos(self: &Self) -> Self;

	/// Tangens function
	fn tan(self: &Self) -> Self;

	/// Cotangens function
	fn cot(self: &Self) -> Self;

	/// Secant function
	fn sec(self: &Self) -> Self;

	/// Cosecant function
	fn csc(self: &Self) -> Self;

	/// Inverse sinus function
	fn arcsin(self: &Self) -> Self;

	/// Inverse cosinus function
	fn arccos(self: &Self) -> Self;

	/// Inverse tangens function
	fn arctan(self: &Self) -> Self;

	fn arctan2(self: &Self,  other: &Self) -> Self;

	/// Inverse cosecant function
	fn arccot(self: &Self) -> Self;

	/// Inverse secant function
	fn arcsec(self: &Self) -> Self;

	// Inverse cosecant function
	fn arccsc(self: &Self) -> Self;
}