#[cfg(test)]
mod adamsbashforth
{
	use mathru::algebra::linear::{Vector};
	use mathru::analysis::differential_equation::ordinary::{ExplicitODE, AdamsBashforth};

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
	fn fn1_1_step()
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
	fn fn1_2_steps()
	{
		let problem: ExplicitODE1 = ExplicitODE1::default();
		let solver: AdamsBashforth<f64> = AdamsBashforth::new(2, 0.001);

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

		let len: usize = y.len();

		let time_span: (f64, f64) = problem.time_span();
		let init_cond: Vector<f64> = problem.init_cond();

		assert!(compare_epsilon(time_span.1, t[len - 1], 0.000000001));
		assert!(compare_epsilon(*init_cond.get(0) * (2.0 * time_span.1).exp() , *y[len - 1].get(0), 0.0003));
	}

	#[test]
	fn fn1_3_steps()
	{
		let problem: ExplicitODE1 = ExplicitODE1::default();
		let solver: AdamsBashforth<f64> = AdamsBashforth::new(3, 0.001);

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

		let len: usize = y.len();

		let time_span: (f64, f64) = problem.time_span();
		let init_cond: Vector<f64> = problem.init_cond();

		assert!(compare_epsilon(time_span.1, t[len - 1], 0.000000001));
		assert!(compare_epsilon(*init_cond.get(0) * (2.0 * time_span.1).exp() , *y[len - 1].get(0), 0.00006));
	}

	#[test]
	fn fn1_4_steps()
	{
		let problem: ExplicitODE1 = ExplicitODE1::default();
		let solver: AdamsBashforth<f64> = AdamsBashforth::new(4, 0.001);

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

		let len: usize = y.len();

		let time_span: (f64, f64) = problem.time_span();
		let init_cond: Vector<f64> = problem.init_cond();

		assert!(compare_epsilon(time_span.1, t[len - 1], 0.000000001));
		assert!(compare_epsilon(*init_cond.get(0) * (2.0 * time_span.1).exp() , *y[len - 1].get(0), 0.000055));
	}

	#[test]
	fn fn1_5_steps()
	{
		let problem: ExplicitODE1 = ExplicitODE1::default();
		let solver: AdamsBashforth<f64> = AdamsBashforth::new(5, 0.001);

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

		let len: usize = y.len();

		let time_span: (f64, f64) = problem.time_span();
		let init_cond: Vector<f64> = problem.init_cond();

		assert!(compare_epsilon(time_span.1, t[len - 1], 0.000000001));
		assert!(compare_epsilon(*init_cond.get(0) * (2.0 * time_span.1).exp() , *y[len - 1].get(0), 0.000055));
	}

//	#[test]
//	fn fn3_k5()
//	{
//		let problem: ExplicitODE3 = ExplicitODE3::default();
//		let solver: AdamsBashforth<f64> = AdamsBashforth::new(5, 0.00001);
//
//		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();
//
//		let len: usize = y.len();
//
//		let time_span: (f64, f64) = problem.time_span();
//
//		assert!(compare_epsilon(time_span.1, t[len - 1], 0.000000001));
//		assert!(compare_epsilon(1.0 / (2.0 - 1.8) - *y[len -1].get(0), 1.89756, 0.00006));
//	}

	#[test]
	fn fn2_1_step()
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
