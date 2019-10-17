use crate::algebra::linear::{Vector, Matrix};
use crate::algebra::abstr::Real;
use super::Solver;

/// Solves an ordinary differential equation using Euler's method.
///
/// <a href="https://en.wikipedia.org/wiki/Euler_method">https://en.wikipedia.org/wiki/Euler_method</a>
pub struct Euler<T>
{
     /// Step size
    step_size: T
}

impl<T> Euler<T>
{
    /// Creates a Euler instance with step size 'step_size'
    ///
    /// # Argument
    ///
    /// * 'step_size'
    ///
    /// # Panics
    ///
    /// 'step_size' <= 0.0
    ///
    pub fn new(step_size: T) -> Euler<T>
    {
        Euler
        {
            step_size: step_size,
        }
    }
}

impl<T> Solver<T> for Euler<T>
    where T: Real
{

    /// Solves `func` using Euler's method.
    ///
    /// # Arguments
    ///
    /// * 'func' is an explict oridnary diffential equation
    /// * 'init' is the initial value at the time 't_start'
    /// * 't_start' initial time
    /// * 't_end'
    ///
    /// # Return
    ///
    /// The solver returns a vector and a matrix, containing the times used in each step of the
    /// algorithm and the respectful values for that time.
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::*;
    /// use mathru::algebra::linear::{vector, Vector, Matrix};
    /// use mathru::analysis::ode::{Solver, Euler};
    ///
    /// let f = |t: &f64, _x: &Vector<f64> | -> Vector<f64> { return Vector::new_row(1, vec![1.0]) * (t * &2.0f64); };
    ///
    ///	let init: Vector<f64> = vector![1.0];
    ///	let solver: Euler<f64> = Euler::new(0.01);
    ///
    ///	let (t, y): (Vector<f64>, Matrix<f64>) = solver.solve(f, init, 0.0, 2.0);
    /// ```
	fn solve<F>(self: &Self, func: F, init: Vector<T>, t_start: T, t_end: T) -> (Vector<T>, Matrix<T>)
        where F: Fn(&T, &Vector<T>) -> Vector<T>
    {

        let mut x_n: Vector<T> = init.clone();
        let mut t_n: T = t_start;

        let limit = ((t_end - t_start) / self.step_size).ceil() + T::one();

        let mut t_vec: Vector<T> = Vector::zero(limit.to_usize().unwrap());
        let (m, _n) = init.dim();
        let mut res_mat: Matrix<T> = Matrix::zero(limit.to_usize().unwrap(), m);

        for i in 0..limit.to_usize().unwrap()
        {
            let h: T = self.step_size.min(t_end - t_n);

            *t_vec.get_mut(i) = t_n;
            //res_mat = res_mat.set_row(&x_n.clone().transpose(), i);
            res_mat.set_row(&x_n.clone().transpose(), i);

            let x_n_1_2: Vector<T> = &x_n + &(&func(&t_n, &x_n) * &(h / T::from_f64(2.0).unwrap()));
            x_n = &x_n + &(&func(&(t_n + h / T::from_f64(2.0).unwrap()), &x_n_1_2) * &h);

            t_n = t_n + h;
        }

        return (t_vec, res_mat);
    }
}