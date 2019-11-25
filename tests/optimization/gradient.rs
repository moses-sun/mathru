extern crate mathru;

#[cfg(test)]
mod gradient
{
	use mathru::algebra::linear::{Vector, Matrix};
	use mathru::optimization::Gradient;
	use mathru::optimization::{Jacobian};


	struct QuadraticFunctionMinimization
	{

	}


	//F(x) = 0.5 (x1^2 + x2^2)^2
	impl QuadraticFunctionMinimization
	{
		pub fn new() -> QuadraticFunctionMinimization
		{
			QuadraticFunctionMinimization{}
		}
	}

	// Ix
	impl Jacobian<f64> for QuadraticFunctionMinimization
	{

		fn jacobian(&self, input: &Vector<f64>) -> Matrix<f64>
		{
			let mut jacobian: Matrix<f64> = Matrix::zero(1, 2);

			let quadratic: f64 = input.dotp(input);
			*jacobian.get_mut(0, 0) = *input.get(0) * quadratic;
			*jacobian.get_mut(0, 1) = *input.get(1) * quadratic;

			return jacobian;
		}

		fn eval(&self, x: &Vector<f64>) -> Vector<f64>
		{
			return vector![x.dotp(x) * x.dotp(x) * 0.5];
		}
	}


	#[test]
	fn test_minimization()
	{
		let optim: Gradient = Gradient::new(0.1, 15);
		let function: QuadraticFunctionMinimization = QuadraticFunctionMinimization::new();

		let x_0: Vector<f64> = Vector::new_column(2, vec![1.0, 1.0]);

		let x_min: Vector<f64> = optim.minimize(&function, &x_0).arg();

		assert!(*x_min.get(0) < 0.005);
		assert!(*x_min.get(1) < 0.005);
	}

}