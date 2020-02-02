//! Often used ODEs
extern crate mathru;
use mathru::algebra::linear::{Vector};
use mathru::analysis::ode::ExplicitODE;
use std::default::Default;

/// Define ODE
/// $`y^{'} = ay = f(x, y) `$
/// $`y = C a e^{at}`$
/// $'y(t_{s}) = C a e^{at_s} => C = \frac{y(t_s)}{ae^{at_s}}`$
pub struct ExplicitODE1
{
	time_span: (f64, f64),
	init_cond: Vector<f64>

}

impl Default for ExplicitODE1
{
	fn default() -> ExplicitODE1
	{
		ExplicitODE1
		{
			time_span: (0.0, 2.0),
			init_cond: vector![0.5]
		}
	}
}

impl ExplicitODE<f64> for ExplicitODE1
{
   	fn func(self: &Self, _t: &f64, x: &Vector<f64>) -> Vector<f64>
	{
		return x * &2.0f64;
	}

    fn time_span(self: &Self) -> (f64, f64)
	{
		return self.time_span;
	}

    fn init_cond(self: &Self) -> Vector<f64>
	{
		return self.init_cond.clone();
	}
}

/// Define ODE
/// $`y^{'} = y^2 + 1 = f(x, y) `$
/// $`y = tan(c+x)`$
pub struct ExplicitODE2
{
	time_span: (f64, f64),
	init_cond: Vector<f64>
}

impl Default for ExplicitODE2
{
	fn default() -> ExplicitODE2
	{
		ExplicitODE2
		{
			time_span: (0.0, 1.4),
			init_cond: vector![0.0],
		}
	}
}

impl ExplicitODE<f64> for ExplicitODE2
{
   	fn func(self: &Self, _t: &f64, x: &Vector<f64>) -> Vector<f64>
	{
		return x.clone().apply(&|e: &f64| -> f64 {return e * e + 1.0;}) ;
	}

    fn time_span(self: &Self) -> (f64, f64)
	{
		return self.time_span;
	}

    fn init_cond(self: &Self) -> Vector<f64>
	{
		return self.init_cond.clone();
	}
}
