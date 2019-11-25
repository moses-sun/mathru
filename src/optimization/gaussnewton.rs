use crate::algebra::linear::{Vector, Matrix};
use crate::optimization::{OptimResult, Jacobian};
use std::marker::PhantomData;
use crate::algebra::abstr::Real;


/// Gauss-Newton method
///
///
pub struct GaussNewton<T>
{
    iters: u64,
	__phantom: PhantomData<T>
}

impl<T> GaussNewton<T>
{
    pub fn new(iters: u64) -> GaussNewton<T>
    {
        GaussNewton
        {
            iters: iters,
            __phantom: PhantomData
        }
    }

}

impl<T> GaussNewton<T>
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
    pub fn minimize<F: Jacobian<T>>(self: &Self, func: &F, x_0: &Vector<T>) -> OptimResult<Vector<T>>
    {
        let mut x_n: Vector<T> = x_0.clone();

        for _i in 0..self.iters
        {
            let jacobian_x_n: Matrix<T> = func.jacobian(&x_n);
            let f_x_n: Vector<T> = func.eval(&x_n);
            let delta_x_n: Vector<T> = jacobian_x_n.pinv() * f_x_n;
            x_n = x_n - delta_x_n;

        }

        return OptimResult::new(x_n);
    }
}