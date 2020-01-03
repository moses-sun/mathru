use crate::algebra::linear::{Vector};
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
    where T: Real
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
        if step_size <= T::zero()
        {
            panic!();
        }
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
    /// * 't_span' Time span
    ///
    /// # Return
    ///
    /// The solver returns a vector and a matrix, containing the times used in each step of the
    /// algorithm and the respectful values for that time.
    ///
    /// # Panic
    ///
    /// t_span.0 > t_span 1
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::*;
    /// use mathru::algebra::linear::{vector, Vector};
    /// use mathru::analysis::ode::{Solver, Euler};
    ///
    /// let f = |t: &f64, _x: &Vector<f64> | -> Vector<f64> { return Vector::new_row(1, vec![1.0]) * (t * &2.0f64); };
    ///
    ///	let init: Vector<f64> = vector![1.0];
    ///	let solver: Euler<f64> = Euler::new(0.01);
    /// let t_start: f64 = 0.0;
    /// let t_stop: f64 = 2.0;
    ///
    ///	let (t, x): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(f, init, (t_start, t_stop)).unwrap();
    /// ```
	fn solve<F>(self: &Self, func: F, init: Vector<T>, t_span: (T, T)) -> Result<(Vec<T>, Vec<Vector<T>>),()>
        where F: Fn(&T, &Vector<T>) -> Vector<T>
    {
        let t_start = t_span.0;
        let t_stop = t_span.1;

        if t_start > t_stop
        {
            panic!()
        }

        let mut x_n: Vector<T> = init.clone();

        let mut t_n: T = t_start;

        let limit = ((t_stop - t_start) / self.step_size).ceil() + T::one();

        let steps: usize = limit.to_u64().unwrap() as usize;
        let mut t_vec: Vec<T> = Vec::with_capacity(steps);
        let mut res_vec: Vec<Vector<T>> = Vec::with_capacity(steps);

        for _i in 0..steps
        {
            let h: T = self.step_size.min(t_stop - t_n);

            t_vec.push(t_n);
            res_vec.push(x_n.clone());

            let x_n_1_2: Vector<T> = &x_n + &(&func(&t_n, &x_n) * &(h / T::from_f64(2.0).unwrap()));
            x_n = &x_n + &(&func(&(t_n + h / T::from_f64(2.0).unwrap()), &x_n_1_2) * &h);

            t_n = t_n + h;
        }

        return Ok((t_vec, res_vec));
    }
}