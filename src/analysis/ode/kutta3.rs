use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::{Solver, ExplicitODE};

/// Solves an ordinary differential equation using the 4th order Runge-Kutta algorithm.
///
///<a href="https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta_methods">https://en.wikipedia
/// .org/wiki/Rung-Kutta_methods</a>
pub struct Kutta3<T>
{
    /// Step size
    step_size: T
}

impl<T> Kutta3<T>
    where T: Real
{
    /// Creates a Kutta3 instance with step size 'step_size'
    ///
    /// # Argument
    ///
    /// * 'step_size'
    ///
    /// # Panics
    ///
    /// 'step_size' <= 0.0
    ///
    pub fn new(step_size: T) -> Kutta3<T>
    {
        if step_size <= T::zero()
        {
            panic!("Step is lower or equal to zero");
        }
        Kutta3
        {
            step_size: step_size,
        }
    }
}

impl<T> Solver<T> for Kutta3<T>
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
    /// use mathru::analysis::ode::{Solver, ExplicitODE, Kutta3};
    ///
    /// // Define ODE
    /// // $`y^{'} = ay = f(x, y) `$
    /// // $`y = C a e^{at}`$
    /// // $'y(t_{s}) = C a e^{at_s} => C = \frac{y(t_s)}{ae^{at_s}}`$
    /// pub struct ExplicitODEProblem
    /// {
    ///	    time_span: (f64, f64),
    ///	    init_cond: Vector<f64>
    /// }
    ///
    /// impl Default for ExplicitODEProblem
    /// {
    ///	    fn default() -> ExplicitODEProblem
    ///	    {
    ///		    ExplicitODEProblem
    ///		    {
    ///			    time_span: (0.0, 2.0),
    ///			    init_cond: vector![0.5],
    ///		    }
    ///	    }
    /// }
    ///
    /// impl ExplicitODE<f64> for ExplicitODEProblem
    /// {
    ///   	fn func(self: &Self, t: &f64, x: &Vector<f64>) -> Vector<f64>
    ///     {
    ///		    return x * &2.0f64;
    ///	    }
    ///
    ///     fn time_span(self: &Self) -> (f64, f64)
    ///     {
    ///		    return self.time_span;
    ///     }
    ///
    ///    fn init_cond(self: &Self) -> Vector<f64>
    ///    {
    ///	        return self.init_cond.clone();
    ///    }
    /// }
    ///
    ///	let problem: ExplicitODEProblem = ExplicitODEProblem::default();
    ///	let solver: Kutta3<f64> = Kutta3::new(0.01);
    ///
    /// let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();
    ///
    /// ```
	 fn solve<F>(self: &Self, prob: &F) -> Result<(Vec<T>, Vec<Vector<T>>), ()>
        where F: ExplicitODE<T>
    {
        let mut x_n: Vector<T> = prob.init_cond();

        let t_span: (T, T) = prob.time_span();
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
            let k1: Vector<T> = prob.func(&t_n, &x_n);

            // k2 = func(t_n + h / 2, x_n + h / 2 k1)
            let k2: Vector<T> = prob.func(&(t_n + (h / T::from_f64(2.0).unwrap())), &(&x_n + &((&k1 * &h) /
            T::from_f64(2.0).unwrap())));

            // k3 = func(t_n + h, x_n + - h k1 + 2h *k2)
            let k3: Vector<T> = prob.func(&(t_n + h), &(&x_n + &(&(&(&k2 * &T::from_f64(2.0).unwrap()) - &k1) * &h))) ;

            // x[n+1] = xn + h*(k1 + 4*k2 + k3)/6
            x_n = x_n + (k1 + k2 * T::from_f64(4.0).unwrap() + k3) * h / T::from_f64(6.0).unwrap();
            t_n = t_n + h;
        }

        return Ok((t_vec, res_vec));
    }
}