#[cfg(test)]
mod levenbergmarquardt
{
    use mathru::{
        algebra::{
            abstr::Real,
            linear::{Matrix, Vector},
        },
        optimization::{LevenbergMarquardt, Optim},
    };

    struct QuadraticFunction {}

    //F(x) = 0.5 (x1^2 + x2^2)^2
    impl QuadraticFunction
    {
        pub fn new() -> QuadraticFunction
        {
            QuadraticFunction {}
        }
    }

    impl Optim<f64> for QuadraticFunction
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
    fn minimization()
    {
        let problem: QuadraticFunction = QuadraticFunction::new();

        let lm: LevenbergMarquardt<f64> = LevenbergMarquardt::new(30, 0.3, 0.95);
        let x_0: Vector<f64> = vector![1.0; -2.1];
        let x_opt: Vector<f64> = lm.minimize(&problem, &x_0).arg();

        let x_opt_ref: Vector<f64> = vector![0.0; 0.0];

        assert!(compare_vector_epsilon(&x_opt_ref, &x_opt, 0.001f64));
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
                println!("a: {}, b: {} a-b: {}", a, b, a - b);
                return false;
            }
        }

        return true;
    }
}
