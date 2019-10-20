use crate::algebra::linear::{Vector, Matrix};
use crate::optim::{Jacobian, Hessian};
use std::marker::PhantomData;
use crate::algebra::abstr::Real;

pub struct Newton<T>
{
    iters: u64,
    __phantom: PhantomData<T>
}

impl<T> Newton<T>
{
    /// Creates an instance of newtons method
    ///
    /// # Arguments
    ///
    /// * 'iters': Number of iterations
    pub fn new(iters: u64) -> Newton<T>
    {
        Newton
        {
            iters: iters,
            __phantom: PhantomData
        }
    }

}

impl<T> Newton<T>
    where T: Real
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
    pub fn minimize<F: Jacobian<f64> + Hessian<f64>>(self: &Self, func: &F, x_0: &Vector<f64>) -> Vector<f64>
    {
        let mut x_n: Vector<f64> = x_0.clone();

        for _i in 0..self.iters
        {
            let hessian_x_n: Matrix<f64> = func.hessian(&x_n);
            let grad_x_n: Vector<f64> = func.jacobian(&x_n).get_row(0).transpose();
            let delta_x_n: Vector<f64> = hessian_x_n.solve_vector(&grad_x_n);
            x_n = x_n - delta_x_n;

        }

        return x_n;
    }

//    pub fn maximize<F: Jacobian<f64>>(self: &Self, func: &F, x_0: &Vector<f64>) -> Vector<f64>
//    {
//        let mut x_n: Vector<f64> = x_0.clone();
//
//        for _i in 0..self.iters
//        {
//            let jacobian_x_n: Matrix<f64> = func.jacobian(&x_n);
//            let f_x_n: Vector<f64> = func.eval(&x_n);
//            let delta_x_n: Vector<f64> = jacobian_x_n.pinv() * f_x_n;
//
//            x_n = x_n + delta_x_n;
//        }
//
//        return x_n;
//    }
}