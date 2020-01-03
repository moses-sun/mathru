#[cfg(test)]
mod euler
{
	extern crate mathru;
	use mathru::algebra::linear::{Vector};
	use mathru::analysis::ode::Solver;
	use mathru::analysis::ode::Euler;


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
		let t_start: f64 = 0.0;
		let t_stop: f64 = 2.0;

		let f = |t: &f64, _x: &Vector<f64> | -> Vector<f64> { return vector![1.0] * (t * &2.0f64); };

		let init: Vector<f64> = vector![1.0];
		let solver: Euler<f64> = Euler::new(0.01);

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(f, init, (t_start, t_stop)).unwrap();

		let len: usize = y.len();

		assert!(compare_epsilon(t_stop, t[len - 1], 0.000000001));
		assert!(compare_epsilon(5.0, *y[len - 1].get(0), 0.000000001));
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
		let t_start: f64 = 0.0;
		let t_stop: f64 = 1.0;
		let init: Vector<f64> = vector![0.0];
		let solver: Euler<f64> = Euler::new(0.1);

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(f, init, (t_start, t_stop)).unwrap();

		let len: usize = y.len();

		assert!(compare_epsilon(t_stop, t[len - 1], 0.00000001));
		assert!(compare_epsilon(t_stop.tan(), *y[len - 1].get(0), 0.02));
	}
}
