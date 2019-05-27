pub mod polynom;
pub mod spline;


pub trait Interpolation
{
	fn eval(self: &Self, x: f64) -> f64;
}