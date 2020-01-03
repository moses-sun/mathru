use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::Solver;

/// Solves an ordinary differential equation using the 4th order Runge-Kutta algorithm.
///
///<a href="https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta_methods">https://en.wikipedia
/// .org/wiki/Rung-Kutta_methods</a>
pub struct RK4<T>
{
    /// Step size
    step_size: T
}

impl<T> RK4<T>
    where T: Real
{
    /// Creates a RK4 instance with step size 'step_size'
    ///
    /// # Argument
    ///
    /// * 'step_size'
    ///
    /// # Panics
    ///
    /// 'step_size' <= 0.0
    ///
    pub fn new(step_size: T) -> RK4<T>
    {
        if step_size <= T::zero()
        {
            panic!("Step is lower or equal to zero");
        }
        RK4
        {
            step_size: step_size,
        }
    }
}

impl<T> Solver<T> for RK4<T>
    where T: Real
{

    /// Solves `func` using the 4th order Runge-Kutta algorithm.
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
    /// use mathru::analysis::ode::{Solver, RK4};
    ///
    /// let f = |_x: &f64, y: &Vector<f64> | -> Vector<f64> { return y * &2.0f64; };
    ///
    ///	let init: Vector<f64> = vector![1.0];
    ///	let solver: RK4<f64> = RK4::new(0.01);
    /// let t_start: f64 = 0.0;
    /// let t_stop: f64 = 2.0;
    ///
    ///	let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(f, init, (t_start, t_stop)).unwrap();
    ///
    /// ```
	fn solve<F>(self: &Self, func: F, init: Vector<T>, t_span: (T, T)) -> Result<(Vec<T>, Vec<Vector<T>>), ()>
        where   F: Fn(&T, &Vector<T>) -> Vector<T>
    {
        let mut x_n: Vector<T> = init.clone();

        let t_start: T = t_span.0;
        let t_stop: T = t_span.1;

        let mut t_n: T = t_start;

        let limit: T = ((t_stop - t_start) / self.step_size).ceil() + T::one();

        let steps: usize = limit.to_u64().unwrap() as usize;

        let mut t_vec: Vec<T> = Vec::with_capacity(steps);
        let mut res_vec: Vec<Vector<T>> = Vec::with_capacity(steps);

        let h: T = self.step_size;

        for _i in 0..steps
        {
            t_vec.push(t_n);
            res_vec.push(x_n.clone());

            // k1 = f(t_n, x_n)
            let k1: Vector<T> = func(&t_n, &x_n);

            // k2 = func(t_n + h / 2, x_n + h / 2 k1)
            let k2: Vector<T> = func(&(t_n + (h / T::from_f64(2.0).unwrap())), &(&x_n + &((&k1 * &h) /
            T::from_f64(2.0).unwrap())));

            // k3 = func(t_n + h / 2, x_n + h / 2 * k2)
            let k3: Vector<T> = func(&(t_n + h / T::from_f64(2.0).unwrap()), &(&x_n + &(&k2 * &(h /
            T::from_f64(2.0).unwrap())))) ;

            // k4 = h * func(t_n + h, x_n + h * k3)
            let k4: Vector<T> = func(&(t_n + h), &(&x_n + &(&k3 * &h)));

            // x[n+1] = x[n] + h*(k1 + 2*k2 + 2*k3 + k4)/6
            x_n = x_n + (k1 + ((k2 + k3) * T::from_f64(2.0).unwrap()) + k4) * h / T::from_f64(6.0).unwrap();
            t_n = t_n + h;
        }

        return Ok((t_vec, res_vec));
    }
}