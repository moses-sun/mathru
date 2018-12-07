
pub trait Power
{
	fn pow(self: &Self, exp: &Self) -> Self;

	fn root(self: &Self, root: &Self) -> Self;
}