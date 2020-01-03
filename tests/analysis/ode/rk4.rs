#[cfg(test)]
mod rk4
{
	extern crate mathru;
	use mathru::algebra::linear::{Vector};
	use mathru::analysis::ode::Solver;
	use mathru::analysis::ode::RK4;


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
		let f = |t: &f64, _x: &Vector<f64> | -> Vector<f64> { return vector![1.0] * (t * &2.0f64); };

		let init: Vector<f64> = vector![1.0];
		let solver: RK4<f64> = RK4::new(0.01);

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(f, init, (0.0f64, 2.0f64)).unwrap();

		let len: usize = y.len();

		assert!(compare_epsilon(2.0, t[len - 1], 0.000000001));
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
		let init: Vector<f64> = vector![0.0];
		let solver: RK4<f64> = RK4::new(0.1);

		let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(f, init, (0.0, 1.4)).unwrap();

		let len: usize = y.len();

		assert!(compare_epsilon(1.40, t[len - 1], 0.00000001));
		assert!(compare_epsilon(1.4_f64.tan(), *y[len - 1].get(0), 0.006));
	}
}
