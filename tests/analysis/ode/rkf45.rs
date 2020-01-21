#[cfg(test)]
mod rkf45
{
	extern crate mathru;
	use mathru::algebra::linear::{Vector};
	use mathru::analysis::ode::{Solver, ExplicitODE, RKF45};

	use super::super::problem::{ExplicitODE1, ExplicitODE2};

	fn compare_epsilon(a: f64, b: f64, epsilon: f64) -> bool
    {
    	if (a - b).abs() > epsilon
        {
        	println!("|a-b|: {}", (a-b).abs());
        	return false;
        }

        return true;
    }

	#[test]
	fn fn1()
	{
		let problem: ExplicitODE1 = ExplicitODE1::default();

		let h_0: f64 = 0.02;
		let h_min: f64 = 0.001;
		let h_max: f64 = 1.0;
		let e_max: f64 = 0.00001;
		let n_max: u32 = 100;

		let solver: RKF45<f64> = RKF45::new(h_0, h_min, h_max, e_max, n_max);

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

		let len: usize = y.len();

		let time_span: (f64, f64) = problem.time_span();
		let init_cond: Vector<f64> = problem.init_cond();

		assert!(compare_epsilon(time_span.1, t[len - 1], 0.001));
		assert!(compare_epsilon(*init_cond.get(0) * (2.0 * time_span.1).exp() , *y[len - 1].get(0), 0.002));
	}

	#[test]
	fn fn2()
	{
		let problem: ExplicitODE2 = ExplicitODE2::default();

		let h_0: f64 = 0.0001;
		let h_min: f64 = 0.0001;
		let h_max: f64 = 1.0;
		let e_max: f64 = 0.00001;
		let n_max: u32 = 1000;

		let solver: RKF45<f64> = RKF45::new(h_0, h_min, h_max, e_max, n_max);

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

		let len: usize = y.len();

		let time_span: (f64, f64) = problem.time_span();

		assert!(compare_epsilon(time_span.1, t[len - 1], 0.0001));
		assert!(compare_epsilon(time_span.1.tan(), *y[len - 1].get(0), 0.0007));
	}
}
