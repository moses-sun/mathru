pub mod beta;
pub mod gamma;

use std::f64::consts::PI;

/// Error Function
pub fn erf<'a>(x: f64) -> f64
{
	let x_squared: f64 =  x.powi(2);
	let a: f64 = 0.140012;
	let b: f64 = -x_squared * (4.0 / PI + a * x_squared) / ( 1.0 + a * x_squared);
	let error: f64 = x.signum() * (1.0 - b.exp()).sqrt();
	error
}





