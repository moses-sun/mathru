use mathru::{
    algebra::{
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

    assert_relative_eq!(x_opt_ref, x_opt, epsilon=0.001f64);
}