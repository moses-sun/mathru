///
///
use std::ops::{Add, AddAssign, Mul, MulAssign};

///
///
pub trait Operator : Copy
{
	fn operator() -> Self;
}

/// The addition operator, commonly symbolized by $`+`$.
#[derive(Copy, Clone)]
pub struct Addition;

impl Operator for Addition
{
    fn operator() -> Self
    {
       	Addition
    }
}

/// The addition operator, commonly symbolized by $`*`$.
#[derive(Copy, Clone)]
pub struct Multiplication;

impl Operator for Multiplication
{
	fn operator() -> Self
	{
		Multiplication
	}
}




/// Trait alias for `Add` and `AddAssign` with result of type `Self`.
trait ClosedAdd<Rhs = Self>: Sized + Add<Rhs, Output = Self> + AddAssign<Rhs>
{

}

// blanked implementation
impl<T, Rhs> ClosedAdd<Rhs> for T where T: Add<Rhs, Output = T> + AddAssign<Rhs>
{

}


/// Trait alias for `Mul` and `MulAssign` with result of type `Self`.
trait ClosedMul<Rhs = Self>: Sized + Mul<Rhs, Output = Self> + MulAssign<Rhs>
{

}

// blanked implementation
impl<T, Rhs> ClosedMul<Rhs> for T where T: Mul<Rhs, Output = T> + MulAssign<Rhs>
{

}
