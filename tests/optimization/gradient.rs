#[cfg(test)]
mod gradient
{
	use mathru::algebra::linear::{Vector, Matrix};
	use mathru::optimization::{Optim, Gradient};

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

	impl Optim<f64> for Rosenbrock
	{

		fn eval(self: &Self, input: &Vector<f64>) -> Vector<f64>
		{
			let x_1: f64 = *input.get(0);
			let x_2: f64 = *input.get(1);

			return vector![(1.0 - x_1) * (1.0 - x_1) + 100.0 * (x_2 - x_1 * x_1) * (x_2 - x_1 * x_1)];
		}

		fn jacobian(self: &Self, input: &Vector<f64>) -> Matrix<f64>
		{
			let x_1: f64 = *input.get(0);
			let x_2: f64 = *input.get(1);

			return matrix![-2.0 * (1.0 - x_1) - 400.0 * (x_2 - x_1 * x_1) * x_1, 200.0 * (x_2 - x_1 * x_1)];
		}
	}

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

	impl Optim<f64> for QuadraticFunctionMinimization
	{

		fn eval(&self, x: &Vector<f64>) -> Vector<f64>
		{
			return vector![x.dotp(x) * x.dotp(x) * 0.5];
		}

		fn jacobian(&self, input: &Vector<f64>) -> Matrix<f64>
		{
			let mut jacobian: Matrix<f64> = Matrix::zero(1, 2);

			let quadratic: f64 = input.dotp(input);
			*jacobian.get_mut(0, 0) = *input.get(0) * quadratic;
			*jacobian.get_mut(0, 1) = *input.get(1) * quadratic;

			return jacobian;
		}
	}

	#[test]
	fn minimization_quadratic()
	{
		let optim: Gradient<f64> = Gradient::new(0.1, 100);
		let function: QuadraticFunctionMinimization = QuadraticFunctionMinimization::new();

		let x_0: Vector<f64> = Vector::new_column(2, vec![1.0, -1.0]);

		let x_min: Vector<f64> = optim.minimize(&function, &x_0).arg();

		assert!(*x_min.get(0) < 0.05);
		assert!(*x_min.get(1) < 0.05);
	}

	#[test]
	fn minimization_rosenbrock()
	{
		let rosenbrock: Rosenbrock = Rosenbrock::new();

		let optim: Gradient<f64> = Gradient::new(0.1, 1500);
		let x_0: Vector<f64> = vector![-2.0; -1.0];
		let x_opt: Vector<f64> = optim.minimize(&rosenbrock, &x_0).arg();

		let x_opt_ref: Vector<f64> = vector![1.0; 1.0];

		assert!(compare_vector_epsilon(&x_opt_ref, &x_opt, 0.1f64));
	}

	fn compare_vector_epsilon(a: &Vector<f64>, b: &Vector<f64>, epsilon: f64) -> bool
    {
        let (a_m, a_n): (usize, usize) = a.dim();
        let (b_m, b_n): (usize, usize) = b.dim();

        if a_m != b_m || a_n != b_n
        {
            println!("dimension mismatch");
            return false;
        }

        for i in 0..a_m
        {
            if (*a.get(i) - *b.get(i)).abs() > epsilon
            {
                println!("a: {}, b: {} a-b: {}", a, b, a-b);
                return false;
            }
        }

        return true;
    }

}