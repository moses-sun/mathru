//! Special functions
//!
//! Fore more information:
//! <a href="https://en.wikipedia.org/wiki/List_of_mathematical_functions">https://en.wikipedia
//! .org/wiki/List_of_mathematical_functions</a>

pub mod beta;
pub mod gamma;
pub mod hypergeometrical;
use crate::algebra::abstr::Real;

/// Error Function
pub fn erf<T>(x: T) -> T
	where T: Real
{
	let x_squared: T =  x * x;
	let a: T = T::from_f64(0.140012);
	let b: T = -x_squared * (T::from_f64(4.0) / T::pi() + a * x_squared) / (T::one() + a * x_squared);
	let error: T = x.sign() * (T::one() - b.exp()).sqrt();

	return error;
}





