use crate::algebra::linear::{Vector, Matrix};
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
    /// let f = |t: &f64, _x: &Vector<f64> | -> Vector<f64> { return Vector::new_row(1, vec![1.0]) * (t * &2.0f64); };
    ///
    ///	let init: Vector<f64> = vector![1.0];
    ///	let solver: RK4<f64> = RK4::new(0.01);
    ///
    ///	let (t, y): (Vector<f64>, Matrix<f64>) = solver.solve(f, init, 0.0, 2.0);
    ///
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
            *t_vec.get_mut(&i) = t_n;
            res_mat = res_mat.set_row(&x_n.transpose(), &i);

            // k1 = f(t_n, x_n)
            let k1: Vector<T> = func(&t_n, &x_n);

            // k2 = func(t_n + dt/2, x_n + dt *k1/2)
            let k2: Vector<T> = func(&(t_n + (self.step_size / T::from_f64(2.0).unwrap())), &(&x_n + &((&k1 * &self.step_size) /
            T::from_f64(2.0).unwrap())));

            // k3 = func(t + dt/2, x + dt * k2/2)
            let k3: Vector<T> = func(&(t_n + self.step_size / T::from_f64(2.0).unwrap()), &(&x_n + &(&k2 * &(self.step_size /
            T::from_f64(2.0).unwrap())))) ;

            // k4 = dt * func(t + dt, x + dt * k3)
            let k4: Vector<T> = func(&(t_n + self.step_size), &(&x_n + &(&k3 * &self.step_size)));


            // x[n+1] = x[n] + h*(k1 + 2*k2 + 2*k3 + k4)/6
            x_n = x_n + (k1 + ((k2 + k3) * T::from_f64(2.0).unwrap()) + k4) * self.step_size / T::from_f64(6.0).unwrap();
            t_n = t_n + self.step_size;
        }

        return (t_vec, res_mat);
    }
}