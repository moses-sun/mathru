use crate::algebra::linear::{Vector, Matrix};
use super::Jacobian;
use std::marker::PhantomData;
use crate::algebra::abstr::Real;
use std::f64;

pub struct Rosenbrock<T>
{
	__phantom: PhantomData<T>
}



impl<T> Rosenbrock<T>
{
	pub fn new() -> Rosenbrock<T>
	{
		Rosenbrock
		{
			__phantom: PhantomData
		}
	}
}

impl<T> Jacobian<T> for Rosenbrock<T>
	where T: Real
{
   	fn eval(self: &Self, input: &Vector<T>) -> Vector<T>
   	{
   		let x_1: T = *input.get(0);
   		let x_2: T = *input.get(1);

   		return vector![	T::from_f64(f64::sqrt(2.0)).unwrap() * (T::one() - x_1);
   						T::from_f64(f64::sqrt(200.)).unwrap() * (x_2 - x_1 * x_1)];
   	}

  	fn jacobian(self: &Self, input: &Vector<T>) -> Matrix<T>
	{
		return matrix![	-T::from_f64(f64::sqrt(2.0)).unwrap(), T::zero();
						-T::from_f64(f64::sqrt(2.0)).unwrap() * *input.get(0) * T::from_f64(f64::sqrt(200.0)).unwrap(), T::from_f64(f64::sqrt(200.0)).unwrap()];
	}
}