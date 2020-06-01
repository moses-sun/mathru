//! Newton-Raphson's root finding algorithm
use crate::algebra::abstr::Real;
use crate::algebra::linear::matrix::Solve;
use crate::algebra::linear::{Matrix, Vector};
use crate::analysis::{Function, Jacobian};
use std::default::Default;

/// Newton Raphson
pub struct NewtonRaphson<T>
{
    iters: u64,
    tolerance_abs: T,
}

impl<T> NewtonRaphson<T>
{
    /// Creates an instance of newtons method
    ///
    /// # Arguments
    ///
    /// * 'iters': Number of iterations
    pub fn new(iters: u64, tolerance_abs: T) -> NewtonRaphson<T>
    {
        NewtonRaphson { iters,
                        tolerance_abs }
    }
}

impl<T> Default for NewtonRaphson<T> where T: Real
{
    fn default() -> NewtonRaphson<T>
    {
        return NewtonRaphson::new(100, T::from_f64(10e-3));
    }
}

impl<T> NewtonRaphson<T> where T: Real
{
    pub fn find_root<F>(self: &Self, func: &F, x_0: &Vector<T>) -> Result<Vector<T>, &'static str>
        where F: Function<Vector<T>, Codomain = Vector<T>> + Jacobian<T>
    {
        let mut x = x_0.clone();

        for _i in 0..self.iters
        {
            let func_x: Vector<T> = func.eval(&x);

            let jacobian_x: Matrix<T> = func.jacobian(&x);

            let b: Vector<T> = jacobian_x.solve(&func_x).unwrap();

            let x_current: Vector<T> = &x - &b;

            if (&x - &x_current).p_norm(&T::from_f64(2.0)) < self.tolerance_abs
            {
                //println!("{}", x);
                return Ok(x);
            }
            //println!("{}", x);
            x = x_current;
        }

        return Err("Maxmimum number of iterations reached");
    }
}
