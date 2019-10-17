#[cfg(test)]
mod rkf45
{
	extern crate mathru;

	use mathru::algebra::abstr::{Real};
	use mathru::algebra::linear::{Vector, Matrix};
	use mathru::analysis::ode::Solver;
	use mathru::analysis::ode::RKF45;


	fn compare_real<T: Real>(a: &T, b: &T, epsilon: T) -> bool
    {
    	if (*a - *b).abs() > epsilon
        {
        	return false;
        }

        return true;
    }

	#[test]
	fn fn1()
	{
		let h_0: f64 = 0.02;
		let h_min: f64 = 0.001;
		let h_max: f64 = 1.0;
		let e_max: f64 = 0.00001;
		let n_max: u32 = 100;

		let f = |t: &f64, _x: &Vector<f64> | -> Vector<f64> { return vector![1.0] * (t * &2.0f64); };

		let init: Vector<f64> = vector![1.0];
		let solver: RKF45<f64> = RKF45::new(h_0, h_min, h_max, e_max, n_max);

		let (t, y): (Vector<f64>, Matrix<f64>) = solver.solve(f, init, 0.0, 2.0);

		let (m, _n): (usize, usize) = y.dim();

		assert!(compare_real(&2.0, &t.get(m - 1), 0.001));
		assert!(compare_real(&5.0, &y.get(m - 1, 0), 0.002));
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
		let h_min: f64 = 0.0001;
		let h_max: f64 = 1.0;
		let e_max: f64 = 0.00001;
		let n_max: u32 = 1000;

		let init: Vector<f64> = vector![0.0];
		let solver: RKF45<f64> = RKF45::new(h_0, h_min, h_max, e_max, n_max);

		let (t, y): (Vector<f64>, Matrix<f64>) = solver.solve(f, init, 0.0, 1.4);

		let (m, _n): (usize, usize) = y.dim();

		assert!(compare_real(&1.40, &t.get(m - 1), 0.0001));
		assert!(compare_real(&1.4_f64.tan(), &y.get(m - 1, 0), 0.0007));
	}
}
