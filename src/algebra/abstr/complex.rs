use super::Real;

pub trait Complex: Real
{
	fn conj(self: Self) -> Self;

//   	pub fn inv<'a>(self: &'a Self) -> Self
//    {
//        &Complex::one() / self
//    }

	fn arg(self: Self) -> Self;
}