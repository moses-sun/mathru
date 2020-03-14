extern crate mathru;

#[cfg(test)]
mod gaussnewton
{
	use mathru::algebra::linear::{Vector, Matrix};
	use mathru::optimization::{GaussNewton};
	use mathru::analysis::{Function, Jacobian};

	pub struct Rosenbrock
	{
	}



	impl Rosenbrock
	{
		pub fn new() -> Rosenbrock
		{
			Rosenbrock
			{
			}
		}
	}

	impl Function<Vector<f64>> for Rosenbrock
	{
		type Codomain = Vector<f64>;

		fn eval(self: &Self, input: &Vector<f64>) -> Vector<f64>
		{
			let x_1: f64 = *input.get(0);
			let x_2: f64 = *input.get(1);

			return vector![	f64::sqrt(2.0) * (1.0 - x_1);
							f64::sqrt(200.0) * (x_2 - x_1 * x_1)];
		}
	}

	impl Jacobian<f64> for Rosenbrock
	{
		fn jacobian(self: &Self, input: &Vector<f64>) -> Matrix<f64>
		{
			return matrix![	-f64::sqrt(2.0), 0.0;
							-f64::sqrt(2.0) * *input.get(0) * f64::sqrt(200.0), f64::sqrt(200.0)];
		}
	}


	#[test]
	fn test_minimization()
	{
		let rosenbrock: Rosenbrock = Rosenbrock::new();

		let gaussnewton: GaussNewton<f64> = GaussNewton::new(5);
		let x_0: Vector<f64> = vector![0.0; -0.1];
		let x_opt: Vector<f64> = gaussnewton.minimize(&rosenbrock, &x_0).arg();

		let x_opt_ref: Vector<f64> = vector![1.0; 1.0];

		assert_eq!(x_opt_ref, x_opt);
	}
}