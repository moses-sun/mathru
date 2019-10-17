use crate::algebra::linear::{Vector, Matrix};
use crate::optim::{Jacobian};


pub struct GaussNewton
{
    iters: u64
}

impl GaussNewton
{
    pub fn new(iters: u64) -> GaussNewton
    {
        GaussNewton
        {
            iters: iters
        }
    }

}

impl GaussNewton
{
    /// Minimize function func
    ///
    /// # Arguments
    ///
    /// * 'func0': Function to be minimized
    /// * 'x_0': Initial guess for the minimum
    ///
    /// # Return
    ///
    /// local minimum
    pub fn minimize<F: Jacobian<f64>>(self: &Self, func: &F, x_0: &Vector<f64>) -> Vector<f64>
    {
        let mut x_n: Vector<f64> = x_0.clone();

        for _i in 0..self.iters
        {
            let jacobian_x_n: Matrix<f64> = func.jacobian(&x_n);
            let f_x_n: Vector<f64> = func.eval(&x_n);
            let delta_x_n: Vector<f64> = jacobian_x_n.pinv() * f_x_n;
            x_n = x_n - delta_x_n;

        }

        return x_n;
    }

    pub fn maximize<F: Jacobian<f64>>(self: &Self, func: &F, x_0: &Vector<f64>) -> Vector<f64>
    {
        let mut x_n: Vector<f64> = x_0.clone();

        for _i in 0..self.iters
        {
            let jacobian_x_n: Matrix<f64> = func.jacobian(&x_n);
            let f_x_n: Vector<f64> = func.eval(&x_n);
            let delta_x_n: Vector<f64> = jacobian_x_n.pinv() * f_x_n;

            x_n = x_n + delta_x_n;
        }

        return x_n;
    }
}