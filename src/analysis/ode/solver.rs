//!

use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Field;


/// Explicit ordinary differential equation
///
/// $`x^{n}(t) = f(t, x(t), x^{'}(t), \dots, x^{n-1}(t))`$
pub trait ExplicitODE<T>
{
    fn func(self: &Self, t: &T, x: &Vector<T>) -> Vector<T>;
    fn time_span(self: &Self) -> (T, T);
    fn init_cond(self: &Self) -> Vector<T>;
}

/// Trait which is implemented by every ode algorithm
pub trait Solver<T>
    where T: Field,
{
    /// ```
    /// use mathru::*;
    /// use mathru::algebra::linear::{Vector, Matrix};
    /// use mathru::analysis::ode::{Solver, ExplicitODE, RK4};
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
    ///	let init: Vector<f64> = vector![1.0];
    ///	let solver: RK4<f64> = RK4::new(0.01);
    /// let t_start: f64 = 0.0;
    /// let t_stop: f64 = 2.0;
    ///
    ///	let problem: ExplicitODEProblem = ExplicitODEProblem::default();
    ///	let solver: RK4<f64> = RK4::new(0.1);
    ///
    /// let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();
    ///
    /// ```
    fn solve<F>(self: &Self, prob: &F) -> Result<(Vec<T>, Vec<Vector<T>>), ()>
        where F: ExplicitODE<T>;
}