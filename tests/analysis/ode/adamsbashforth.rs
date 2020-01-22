#[cfg(test)]
mod adamsbashforth
{
	extern crate mathru;
	use mathru::algebra::linear::{Vector};
	use mathru::analysis::ode::{Solver, ExplicitODE};
	use mathru::analysis::ode::AdamsBashforth;

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
	fn fn1_k1()
	{
		let problem: ExplicitODE1 = ExplicitODE1::default();
		let solver: AdamsBashforth<f64> = AdamsBashforth::new(1, 0.001);

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

		let len: usize = y.len();

		let time_span: (f64, f64) = problem.time_span();
		let init_cond: Vector<f64> = problem.init_cond();

		assert!(compare_epsilon(time_span.1, t[len - 1], 0.000000001));
		assert!(compare_epsilon(*init_cond.get(0) * (2.0 * time_span.1).exp() , *y[len - 1].get(0), 0.2));
	}

	#[test]
	fn fn2_k1()
	{
		let problem: ExplicitODE2 = ExplicitODE2::default();
		let solver: AdamsBashforth<f64> = AdamsBashforth::new(1, 0.001);

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

		let len: usize = y.len();

		let time_span: (f64, f64) = problem.time_span();

		assert!(compare_epsilon(time_span.1, t[len - 1], 0.00000001));
		assert!(compare_epsilon(time_span.1.tan(), *y[len - 1].get(0), 0.07));
	}
}