

pub trait Exponential
{
	fn e() -> Self;

	///Exponential function
	fn exp(self: Self) -> Self;

	///Logiarithm function
	fn ln(self: Self) -> Self;

}