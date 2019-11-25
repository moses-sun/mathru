extern crate mathru;

#[cfg(test)]
mod levenbergmarquardt
{
	use mathru::algebra::linear::{Vector, Matrix};
	use mathru::optimization::{Jacobian, LevenbergMarquardt};
	use mathru::algebra::abstr::Real;

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

	impl Jacobian<f64> for Rosenbrock
	{
		fn eval(self: &Self, input: &Vector<f64>) -> Vector<f64>
		{
			let x_1: f64 = *input.get(0);
			let x_2: f64 = *input.get(1);

			return vector![	f64::sqrt(2.0) * (1.0 - x_1);
							f64::sqrt(200.0) * (x_2 - x_1 * x_1)];
		}

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

		let lm: LevenbergMarquardt<f64> = LevenbergMarquardt::new(150, 0.3, 0.95);
		let x_0: Vector<f64> = vector![0.0; -0.1];
		let x_opt: Vector<f64> = lm.minimize(&rosenbrock, &x_0).arg();

		let x_opt_ref: Vector<f64> = vector![1.0; 1.0];

		assert!(compare_vector_epsilon(&x_opt_ref, &x_opt, 0.01f64));
	}

	fn compare_vector_epsilon<T: Real>(a: &Vector<T>, b: &Vector<T>, epsilon: T) -> bool
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