#[cfg(test)]
mod euler
{
	extern crate mathru;

	use mathru::algebra::abstr::{Real};
	use mathru::algebra::linear::{Vector, Matrix};
	use mathru::analysis::ode::Solver;
	use mathru::analysis::ode::Euler;


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
		let f = |t: &f64, _x: &Vector<f64> | -> Vector<f64> { return vector![1.0] * (t * &2.0f64); };

		let init: Vector<f64> = vector![1.0];
		let solver: Euler<f64> = Euler::new(0.01);

		// y = 2 * x
		let (t, y): (Vector<f64>, Matrix<f64>) = solver.solve(f, init, 0.0, 2.0);

		let (m, _n): (usize, usize) = y.dim();

		assert!(compare_real(&2.0, &t.get(m - 1 ), 0.00000001));
		assert!(compare_real(&5.00, &y.get(m - 1, 0), 0.00000001));
	}

	#[test]
	fn fn2()
	{
		// x' = 1 + x^2
		let f = |_t: &f64, x: &Vector<f64> | -> Vector<f64> { return x.clone(); };

		let init: Vector<f64> = vector![1.0];
		let solver: Euler<f64> = Euler::new(0.0001);


		let (_t, y): (Vector<f64>, Matrix<f64>) = solver.solve(f, init, 0.0, 1.5);

		let (m, _n): (usize, usize) = y.dim();
		//println!("{}, {}",  &y.get(&(m-1), &0), 1.5_f64.exp());
		//println!("{}",  &y.get(&(m-1), &0));
		assert!(compare_real(&1.5_f64.exp(), &y.get(m - 1, 0), 0.01));
	}

}