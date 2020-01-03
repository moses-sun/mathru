// https://web.archive.org/web/20150907215914/http://adorio-research.org/wordpress/?p=6565

#[cfg(test)]
mod rkdp
{
	extern crate mathru;

	use mathru::algebra::linear::{Vector};
	use mathru::analysis::ode::{Solver, Dopri5};

	fn compare_epsilon(a: f64, b: f64, epsilon: f64) -> bool
    {
    	if (a - b).abs() > epsilon
        {
        	return false;
        }

        return true;
    }

	#[test]
	fn fn1()
	{
		let h_0: f64 = 0.02;
		let e_max: f64 = 0.00001;
		let n_max: u32 = 100;

		let f = |t: &f64, _x: &Vector<f64> | -> Vector<f64> { return vector![1.0] * (t * &2.0f64); };

		let init: Vector<f64> = vector![1.0];
		let solver: Dopri5<f64> = Dopri5::new(h_0, e_max, n_max);
		let t_start: f64 = 0.0;
		let t_stop: f64 = 2.0;

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(f, init, (t_start, t_stop)).unwrap();

		let len: usize = y.len();

		assert!(compare_epsilon(t_stop, t[len - 1], 0.001));
		assert!(compare_epsilon(5.0, *y[len - 1].get(0), 0.002));
	}


	// x' = 1 + x^2
	fn f(_t: &f64, x: &Vector<f64>) -> Vector<f64>
	{
		let result  = vector![1.0] + x.clone().apply(&|e: &f64| -> f64 {return e * e;}) ;

		return result;
	}

	#[test]
	fn fn2()
	{
		let h_0: f64 = 0.0001;
		let e_max: f64 = 0.000001;
		let n_max: u32 = 500;

		let init: Vector<f64> = vector![0.0];
		let solver: Dopri5<f64> = Dopri5::new(h_0, e_max, n_max);
		let t_start: f64 = 0.0;
		let t_stop: f64 = 1.4;

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(f, init, (t_start, t_stop)).unwrap();

		let len: usize = y.len();

		assert!(compare_epsilon(t_stop, t[len - 1], 0.001));
		assert!(compare_epsilon(t_stop.tan(), *y[len - 1].get(0), 0.0001));
	}
}