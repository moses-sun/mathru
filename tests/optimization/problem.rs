use mathru::{
    algebra::linear::{matrix::General, vector::Vector},
    optimization::Optim,
};

pub struct LinearEquation {
    a: General<f64>,
    b: Vector<f64>,
}

//Ax = b
impl LinearEquation {
    pub fn new() -> LinearEquation {
        LinearEquation {
            a: matrix![1.0, 3.0; 3.0, 5.0],
            b: vector![-7.0; 7.0],
        }
    }
}

/// f(x) = b-Ax
impl Optim<f64> for LinearEquation {
    fn eval(&self, x: &Vector<f64>) -> Vector<f64> {
        return self.b.clone() - self.a.clone() * x.clone();
    }

    // A
    fn jacobian(&self, _input: &Vector<f64>) -> General<f64> {
        return self.a.clone();
    }

    fn hessian(&self, _x: &Vector<f64>) -> General<f64> {
        unimplemented!()
    }
}

pub struct Rosenbrock {}

impl Rosenbrock {
    pub fn new() -> Rosenbrock {
        Rosenbrock {}
    }
}

impl Optim<f64> for Rosenbrock {
    fn eval(&self, input: &Vector<f64>) -> Vector<f64> {
        let x_1: f64 = input[0];
        let x_2: f64 = input[1];

        return vector![(1.0 - x_1) * (1.0 - x_1) + 100.0 * (x_2 - x_1 * x_1) * (x_2 - x_1 * x_1)];
    }

    fn jacobian(&self, input: &Vector<f64>) -> General<f64> {
        let x_1: f64 = input[0];
        let x_2: f64 = input[1];

        return matrix![
            -2.0 * (1.0 - x_1) - 400.0 * (x_2 - x_1 * x_1) * x_1,
            200.0 * (x_2 - x_1 * x_1)
        ];
    }

    fn hessian(&self, input: &Vector<f64>) -> General<f64> {
        let x_1: f64 = input[0];
        let x_2: f64 = input[1];
        return matrix![1200.0 * x_1 * x_1 - 400.0 * x_2  + 2.0, -400.0 * x_1
            ; -400.0 * x_1, 200.0];
    }
}

pub struct QuadraticFunction {}

//F(x) = 0.5 (x1^2 + x2^2)^2
impl QuadraticFunction {
    pub fn new() -> QuadraticFunction {
        QuadraticFunction {}
    }
}

impl Optim<f64> for QuadraticFunction {
    fn eval(&self, x: &Vector<f64>) -> Vector<f64> {
        return vector![x.dotp(x) * x.dotp(x) * 0.5];
    }

    fn jacobian(&self, input: &Vector<f64>) -> General<f64> {
        let mut jacobian: General<f64> = General::zero(1, 2);

        let quadratic: f64 = input.dotp(input);
        jacobian[[0, 0]] = input[0] * quadratic;
        jacobian[[0, 1]] = input[1] * quadratic;

        return jacobian;
    }

    fn hessian(&self, _x: &Vector<f64>) -> General<f64> {
        unimplemented!()
    }
}
