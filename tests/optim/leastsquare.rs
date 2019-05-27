
#[cfg(test)]
mod leastsquare
{
	extern crate mathru;

	use mathru::algebra::linear::{Vector, Matrix};
	use mathru::optim::LeastSquare;
	use mathru::optim::{StochasticGradientDesc, BatchGradientDesc};
	use mathru::optim::{Optimizable};

	struct Function1
	{

	}

	impl Function1
	{
		fn new() -> Function1
		{
			Function1
			{

			}
		}
	}

	impl Optimizable for Function1
	{
	    type Input = Matrix<f64>;

    	/// The target data type to the model.
    	type Target = Vector<f64>;

    	/// Compute the gradient for the model.
    	fn value(&self, param: &Vector<f64>, input: &Self::Input, target: &Self::Target) -> f64
		{
			unimplemented!();
//			let target_hat: Vector<f64> = input * param;
//
//        	let diff: Vector<f64> = &target_hat - target;
//
//        	let cost: f64 = (&target_hat - target).dotp(&diff);
//
//			return cost;
		}

		fn gradient(&self, param: &Vector<f64>, input: &Self::Input, target: &Self::Target) -> Vector<f64>
		{
			unimplemented!();
		}
	}

	#[test]
	fn fn0()
	{
		let epsilon: f64 = 0.01;
		let m: usize = 2;
		let n: usize = 1;

		let func1: Function1 = Function1::new();

		let x: Matrix<f64> = Matrix::new(m, n, vec![2.0, 6.0]);
		let y: Vector<f64> = Vector::new_column(m, vec![2.0, 4.0]);

		let optimizer: BatchGradientDesc = BatchGradientDesc::new(0.1, 0.8, 100);

		let mut model: LeastSquare<BatchGradientDesc> = LeastSquare::new(optimizer);
		model.train(&x, &y);

		let y_hat: Vector<f64> = model.predict(&x);

		let diff: Vector<f64> = y - y_hat;
		let sum_sqared_diff : f64 = diff.dotp( &diff);
		assert!(sum_sqared_diff.sqrt() < epsilon);;
	}


	#[test]
	fn fn1()
	{
		let epsilon: f64 = 0.01;
		let m: usize = 2;
		let n: usize = 1;


		let x: Matrix<f64> = Matrix::new(m, n, vec![2.0, 6.0]);
		let y: Vector<f64> = Vector::new_column(m, vec![2.0, 4.0]);

		let optimizer: StochasticGradientDesc = StochasticGradientDesc::new(0.1, 0.8, 100);
		let mut model: LeastSquare<StochasticGradientDesc> = LeastSquare::new(optimizer);
		model.train(&x, &y);

		let y_hat: Vector<f64> = model.predict(&x);

		let diff: Vector<f64> = y-y_hat;
		let sum_sqared_diff : f64 = diff.dotp( &diff);
		assert!(sum_sqared_diff.sqrt() < epsilon);;
	}

	#[test]
	fn fn2()
	{
		let epsilon: f64 = 0.01;
		let m: usize = 3;
		let n: usize = 1;

		let x: Matrix<f64> = Matrix::new(m, n, vec![2.0, 2.0, 6.0]);
		let y: Vector<f64> = Vector::new_column(m, vec![2.0, 2.0, 4.0]);

		let optimizer: StochasticGradientDesc = StochasticGradientDesc::new(0.1, 0.8, 100);
		let mut model: LeastSquare<StochasticGradientDesc> = LeastSquare::new(optimizer);
		model.train(&x, &y);

		let y_hat: Vector<f64> = model.predict(&x);

		let diff: Vector<f64> = y-y_hat;
		let sum_sqared_diff: f64= diff.dotp( &diff);
		assert!(sum_sqared_diff.sqrt() < epsilon);
	}
}
