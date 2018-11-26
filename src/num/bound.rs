
/// Numbers which have upper and lower bounds
pub trait Bound
{
	/// returns the smallest finite number this type can represent
	fn lower_bound() -> Self;
	/// returns the largest finite number this type can represent
	fn upper_bound() -> Self;
}


