use mathru::{
    algebra::linear::{Matrix, Vector},
    optimization::{Newton, Optim},
};

pub struct Rosenbrock {}

impl Rosenbrock
{
    pub fn new() -> Rosenbrock
    {
        Rosenbrock {}
    }
}

impl Optim<f64> for Rosenbrock
{
    fn eval(self: &Self, input: &Vector<f64>) -> Vector<f64>
    {
        let x_1: f64 = *input.get(0);
        let x_2: f64 = *input.get(1);

        return vector![(1.0 - x_1) * (1.0 - x_1)
                       + 100.0 * (x_2 - x_1 * x_1) * (x_2 - x_1 * x_1)];
    }

    fn jacobian(self: &Self, input: &Vector<f64>) -> Matrix<f64>
    {
        let x_1: f64 = *input.get(0);
        let x_2: f64 = *input.get(1);

        return matrix![-2.0 * (1.0 - x_1) - 400.0 * (x_2 - x_1 * x_1) * x_1,
                       200.0 * (x_2 - x_1 * x_1)];
    }

    fn hessian(self: &Self, input: &Vector<f64>) -> Matrix<f64>
    {
        let x_1: f64 = *input.get(0);
        let x_2: f64 = *input.get(1);
        return matrix![1200.0 * x_1 * x_1 - 400.0 * x_2  + 2.0, -400.0 * x_1
            ; -400.0 * x_1, 200.0];
    }
}

#[test]
fn test_minimization()
{
    let rosenbrock: Rosenbrock = Rosenbrock::new();

    let optimizer: Newton<f64> = Newton::new(15, 0.1, 0.00001);
    let x_0: Vector<f64> = vector![0.0; -0.1];
    let x_opt: Vector<f64> = optimizer.minimize(&rosenbrock, &x_0).arg();

    let x_opt_ref: Vector<f64> = vector![1.0; 1.0];

    assert_eq!(x_opt_ref, x_opt);
}
