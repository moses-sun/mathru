use mathru::{
    algebra::{
        linear::{Matrix, Vector},
    },
    optimization::{ConjugateGradient, Optim},
};

struct LinearEquation
{
    a: Matrix<f64>,
    b: Vector<f64>,
}

//Ax = b
impl LinearEquation
{
    pub fn new() -> LinearEquation
    {
        LinearEquation { a: matrix![1.0, 3.0; 3.0, 5.0],
                         b: vector![-7.0; 7.0] }
    }
}

/// f(x) = b-Ax
impl Optim<f64> for LinearEquation
{
    fn eval(&self, x: &Vector<f64>) -> Vector<f64>
    {
        return self.b.clone() - self.a.clone() * x.clone();
    }

    // A
    fn jacobian(&self, _input: &Vector<f64>) -> Matrix<f64>
    {
        return self.a.clone();
    }
}

#[test]
fn test_minimization()
{
    let optim: ConjugateGradient<f64> = ConjugateGradient::new(10, 0.01);
    let leq: LinearEquation = LinearEquation::new();

    let x_0: Vector<f64> = vector![1.0; 1.0];

    let x_min: Vector<f64> = optim.minimize(&leq, &x_0).arg();

    let x_opt: Vector<f64> = vector![14.0; -7.0];

    assert_relative_eq!(x_opt, x_min, epsilon=0.0001);
}

