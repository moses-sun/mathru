use crate::algebra::linear::{Vector, Matrix};
use crate::algebra::abstr::Real;
use super::Solver;

/// Solves an ordinary differential equation using Heun's method.
///
/// <a href="https://en.wikipedia.org/wiki/Heun%27s_method">https://en.wikipedia.org/wiki/Heun's_method</a>
pub struct Heun<T>
{
    step_size: T
}


impl<T> Heun<T>
{
    /// Creates a Heun instance with step size 'step_size'
    ///
    /// # Argument
    ///
    /// * 'step_size'
    ///
    /// # Panics
    ///
    /// 'step_size' <= 0.0
    ///
    pub fn new(step_size: T) -> Heun<T>
    {
        Heun
        {
            step_size: step_size,
        }
    }
}

impl<T> Solver<T> for Heun<T>
    where T: Real
{

    /// Solves `func` using Heun's method.
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
    /// use mathru::algebra::linear::{Vector, Matrix};
    /// use mathru::analysis::ode::{Solver, Heun};
    ///
    /// let f = |t: &f64, _x: &Vector<f64> | -> Vector<f64> { return Vector::new_row(1, vec![1.0]) * (t * &2.0f64); };
    ///
    ///	let init: Vector<f64> = vector![1.0];
    ///	let solver: Heun<f64> = Heun::new(0.01);
    ///
    ///	let (t, y): (Vector<f64>, Matrix<f64>) = solver.solve(f, init, 0.0, 2.0);
    /// ```
	fn solve<F>(self: &Self, func: F, init: Vector<T>, t_start: T, t_end: T) -> (Vector<T>, Matrix<T>)
        where F: Fn(&T, &Vector<T>) -> Vector<T>
    {
        let mut x_n: Vector<T> = init.clone();
        let mut t_n: T = t_start;

        let limit = ((t_end - t_start) / self.step_size).ceil() + T::one();
        let steps: usize = limit.to_usize().unwrap();
        let mut t_vec: Vector<T> = Vector::zero(steps);
        let (m, _n) = init.dim();
        let mut res_mat: Matrix<T> = Matrix::zero(steps, m);

        for i in 0..steps
        {
            *t_vec.get_mut(&i) = t_n;
            res_mat = res_mat.set_row(&x_n.transpose(), &i);

            let slope_left: Vector<T> = func(&t_n, &x_n);
            let slope_right: Vector<T> = func(&(t_n + self.step_size), &(&x_n + &(&slope_left * &self.step_size)));

            let slope_ideal: Vector<T> = (&slope_left + &slope_right) / T::from_f64(2.0).unwrap();

            // Update
            x_n = &x_n + &(&slope_ideal * &self.step_size);
            t_n = t_n + self.step_size;
        }

        return (t_vec, res_mat);
    }
}