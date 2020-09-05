use mathru::algebra::linear::{Vector};
use mathru::analysis::ode::{ExplicitODE, Tsitouras54};

use super::super::problem::{ExplicitODE1, ExplicitODE2};

#[test]
fn max_number_iterations_too_low()
{
	let problem: ExplicitODE1 = ExplicitODE1::default();

	let h_0: f64 = 0.0001;
	let n_max: u32 = 10;
	let abs_tol: f64 = 10e-6;

	let solver: Tsitouras54<f64> = Tsitouras54::new(abs_tol, h_0, n_max);

	let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

	let len: usize = y.len();

	let time_span: (f64, f64) = problem.time_span();
	let init_cond: Vector<f64> = problem.init_cond();

	assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.001);
	assert_relative_eq!(*init_cond.get(0) * (2.0 * time_span.1).exp() , *y[len - 1].get(0), epsilon=0.002);
}

#[test]
fn fn1()
{
	let problem: ExplicitODE1 = ExplicitODE1::default();

	let h_0: f64 = 0.0001;
	let n_max: u32 = 100;
	let abs_tol: f64 = 10e-6;

	let solver: Tsitouras54<f64> = Tsitouras54::new(abs_tol, h_0, n_max);

	let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

	let len: usize = y.len();

	let time_span: (f64, f64) = problem.time_span();
	let init_cond: Vector<f64> = problem.init_cond();

	assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.001);
	assert_relative_eq!(*init_cond.get(0) * (2.0 * time_span.1).exp() , *y[len - 1].get(0), epsilon=0.002);
}

#[test]
fn fn2()
{
	let problem: ExplicitODE2 = ExplicitODE2::default();

	let h_0: f64 = 0.0001;
	let n_max: u32 = 400;
	let abs_tol: f64 = 10e-7;

	let solver: Tsitouras54<f64> = Tsitouras54::new(abs_tol, h_0, n_max);

	let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

	let len: usize = y.len();

	let time_span: (f64, f64) = problem.time_span();

	assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.0001);
	assert_relative_eq!(time_span.1.tan(), *y[len - 1].get(0), epsilon=0.001);
}
