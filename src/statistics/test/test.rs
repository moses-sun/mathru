

pub trait Test
{
	///Test value
	fn value(self: &Self) -> f64;

	/// Degree of freedom
	fn df(self: &Self) -> u32;

	///
	fn p_value(self: &Self) -> f64;
}