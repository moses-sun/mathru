#[cfg(test)]
mod bogackishampine
{
	extern crate mathru;
	use mathru::algebra::linear::{Vector};
	use mathru::analysis::ode::{ExplicitODE, BogackiShampine32};

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
	fn setter_getter_test()
	{
		let h_0: f64 = 0.0001;
		let fac: f64 = 0.9;
		let fac_min: f64 = 0.01;
		let fac_max: f64 = 2.0;
		let n_max: u32 = 100;
		let abs_tol: f64 = 10e-6;
		let rel_tol: f64 = 10e-6;

		let mut solver: BogackiShampine32<f64> = BogackiShampine32::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);

		assert_eq!(abs_tol, *solver.get_abs_tol());

		let abs_tol_new: f64 = 0.2;
		solver.set_abs_tol(abs_tol_new);
		assert_eq!(abs_tol_new, *solver.get_abs_tol());

		assert_eq!(rel_tol, *solver.get_rel_tol());

		let rel_tol_new: f64 = 0.2;
		solver.set_rel_tol(abs_tol_new);
		assert_eq!(rel_tol_new, *solver.get_rel_tol());

	}

	#[test]
	fn fn1()
	{
		let problem: ExplicitODE1 = ExplicitODE1::default();

		let h_0: f64 = 0.0001;
		let fac: f64 = 0.9;
		let fac_min: f64 = 0.01;
		let fac_max: f64 = 2.0;
		let n_max: u32 = 1000;
		let abs_tol: f64 = 10e-6;
		let rel_tol: f64 = 10e-6;

		let solver: BogackiShampine32<f64> = BogackiShampine32::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

		let len: usize = y.len();

		let time_span: (f64, f64) = problem.time_span();
		let init_cond: Vector<f64> = problem.init_cond();

		assert!(compare_epsilon(time_span.1, t[len - 1], 0.001));
		assert!(compare_epsilon(*init_cond.get(0) * (2.0 * time_span.1).exp() , *y[len - 1].get(0), 0.4));
	}

	#[test]
	fn fn2()
	{
		let problem: ExplicitODE2 = ExplicitODE2::default();

		let h_0: f64 = 0.0001;
		let fac: f64 = 0.9;
		let fac_min: f64 = 0.01;
		let fac_max: f64 = 2.0;
		let n_max: u32 = 1000;
		let abs_tol: f64 = 10e-7;
		let rel_tol: f64 = 10e-7;

		let solver: BogackiShampine32<f64> = BogackiShampine32::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

		let len: usize = y.len();

		let time_span: (f64, f64) = problem.time_span();

		assert!(compare_epsilon(time_span.1, t[len - 1], 0.0001));
		assert!(compare_epsilon(time_span.1.tan(), *y[len - 1].get(0), 0.05));
	}
}