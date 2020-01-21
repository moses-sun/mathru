use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::{Solver, ExplicitODE};

/// Solves an ordinary differential equation using Ralston's method.
///
/// Ralston's method is a second-order metho
///
/// <a href="https://en.wikipedia.org/wiki/List_of_Runge-Kutta_methods#Forward_Euler">https://en.wikipedia.org/wiki/List_of_Runge-Kutta_methods#Forward_Euler</a>
pub struct Ralston<T>
{
     /// Step size
    step_size: T
}

impl<T> Ralston<T>
    where T: Real
{
    /// Creates a Ralstons instance with step size 'step_size'
    ///
    /// # Argument
    ///
    /// * 'step_size'
    ///
    /// # Panics
    ///
    /// 'step_size' <= 0.0
    ///
    pub fn new(step_size: T) -> Ralston<T>
    {
        if step_size <= T::zero()
        {
            panic!();
        }
        Ralston
        {
            step_size: step_size,
        }
    }
}

impl<T> Solver<T> for Ralston<T>
    where T: Real
{

    /// Solves `func` using Ralston's method.
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
    /// use mathru::algebra::linear::{Vector};
    /// use mathru::analysis::ode::{Solver, ExplicitODE, Ralston};
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
    ///
    ///	let solver: Ralston<f64> = Ralston::new(0.001);
    ///
    /// let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();
    ///
    /// ```
    fn solve<F>(self: &Self, prob: &F) -> Result<(Vec<T>, Vec<Vector<T>>), ()>
        where F: ExplicitODE<T>
    {
        let t_span =  prob.time_span();
        let init = prob.init_cond();
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

            let k_1: Vector<T> = prob.func(&t_n, &x_n);
            let factor: T = h * T::from_f64(2.0/3.0).unwrap();
            let k_2: Vector<T> = prob.func(&(t_n + factor), &(&x_n + &(&k_1 * &factor )));

            // Update
            x_n = x_n + ((k_1 * T::from_f64(1.0/4.0).unwrap()) + (k_2 * T::from_f64(3.0/4.0).unwrap())) * h;
            t_n = t_n + h;
        }

        return Ok((t_vec, res_vec));
    }
}