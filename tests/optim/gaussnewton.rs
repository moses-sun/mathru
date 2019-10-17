extern crate mathru;

#[cfg(test)]
mod gaussnewton
{
	use mathru::algebra::linear::{Vector};
	use mathru::optim::{GaussNewton};
	use mathru::optim::{Rosenbrock};



	#[test]
	fn test_minimization()
	{
		let rosenbrock: Rosenbrock<f64> = Rosenbrock::new();

		let gaussnewton: GaussNewton = GaussNewton::new(10);
		let x_0: Vector<f64> = vector![0.0; -0.1];
		let x_opt: Vector<f64> = gaussnewton.minimize(&rosenbrock, &x_0);

		let x_opt_ref: Vector<f64> = vector![1.0; 1.0];

		assert_eq!(x_opt_ref, x_opt);
	}
}