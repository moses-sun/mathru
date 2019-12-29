use super::{Field, Lattice, Scalar};
use crate::elementary::{Exponential, Trigonometry, Power, Hyperbolic};



macro_rules! impl_real
{
    ($($t:ty, $id:ident);*) =>
    {
    	$(
        impl Real for $t
        {
			/// Returns the smallest integer greater than or equal to a number.
			fn ceil(self: &Self) -> Self
			{
				(*self).ceil()
			}

			/// Returns the largest integer less than or equal to a number.
			fn floor(self: &Self) -> Self
			{
				(*self).floor()
			}

			fn epsilon() -> Self
			{
				return std::$id::EPSILON;
			}
        }
        )*
    }
}

impl_real!(f32, f32; f64, f64);


/// Real number
///
///<a href="https://en.wikipedia.org/wiki/Real_number">https://en.wikipedia.org/wiki/Real_number</a>
pub trait Real: Field + Lattice + Scalar + Exponential + Trigonometry + Power + Hyperbolic
{
	/// Returns the smallest integer greater than or equal to a number.
	fn ceil(self: &Self) -> Self;

	/// Returns the largest integer less than or equal to a number.
	fn floor(self: &Self) -> Self;

	fn min(self: Self, a: Self) -> Self
    {
    	if self <= a
		{
			return self
		}
		else
		{
			return a
		}
    }

    fn max(self: Self, a: Self) -> Self
    {
    	if self >= a
		{
			return self
		}
		else
		{
			return a
		}
    }

    fn epsilon() -> Self;
}

