//! Adaptive step size stepper

use std::default::Default;
use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::{ExplicitODE, ExplicitAdaptiveMethod};

/// Adaptive step size stepper
pub struct AdaptiveStepper<T>
{
     /// Step size
    e_max: T,
    n_max: u32,
    h_0: T,
    h_max: T,
    h_min: T,
}

impl<T> Default for AdaptiveStepper<T>
    where T: Real
{
    fn default() -> AdaptiveStepper<T>
    {
        return AdaptiveStepper::new(1000, T::from_f64(0.02).unwrap(), T::from_f64(0.01).unwrap(), T::from_f64(0.001).unwrap(), T::from_f64
        (0.001).unwrap());
    }
}

impl<T> AdaptiveStepper<T>
    where T: Real
{
    /// Creates an instance with the given step siz
    ///
    pub fn new(n_max: u32, e_max: T, h_0: T, h_min: T, h_max: T) -> AdaptiveStepper<T>
    {
        return AdaptiveStepper
        {
            e_max: e_max,
            n_max: n_max,
            h_0: h_0,
            h_max: h_max,
            h_min: h_min,
        }
    }

    /// Solves `func` using the 4th order Runge-Kutta-Fehlberg algorithm.
    ///
    /// # Arguments
    ///
    /// * 'func' is an explict oridnary diffential equation
    /// * 'init' is the initial value at the time 't_start'
    /// * 't_span' Time span t_span.0 = t_start, t_span.1 = t_stop
    ///
    /// # Return
    ///
    /// The solver returns a vector and a matrix, containing the times used in each step of the
    /// algorithm and the respectful values for that time.
    ///
    /// # Panic
    ///
    /// if t_span.0 > t_span.1
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::*;
    /// use mathru::algebra::linear::{Vector, Matrix};
    /// use mathru::analysis::ode::{AdaptiveStepper, ExplicitODE, RungeKuttaFehlberg45};
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
   	/// let h_0: f64 = 0.02;
	///	let h_min: f64 = 0.001;
	///	let h_max: f64 = 1.0;
	///	let e_max: f64 = 0.00001;
	///	let n_max: u32 = 100;
    ///
	///	let method: RungeKuttaFehlberg45<f64> = RungeKuttaFehlberg45::new();
    /// let solver: AdaptiveStepper<f64> = AdaptiveStepper::new(n_max, e_max, h_0, h_min, h_max);
    ///
    /// let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem, &method).unwrap();
    /// ```
    pub fn solve<F, M>(self: &Self, prob: &F, method: &M) -> Result<(Vec<T>, Vec<Vector<T>>), ()>
        where F: ExplicitODE<T>,
               M: ExplicitAdaptiveMethod<T>
    {
        let t_span: (T, T) = prob.time_span();
        let t_start: T = t_span.0;
        let t_stop: T = t_span.1;
        if t_start > t_stop
        {
            panic!();
        }

        let mut x_n: Vector<T> = prob.init_cond();
        let mut t_n: T = t_start;
        let mut h: T = self.h_0;

        let mut t_vec: Vec<T> = Vec::new();
        t_vec.push(t_n);

        let mut res_vec: Vec<Vector<T>> = Vec::new();
        res_vec.push(x_n.clone());

        let mut n: u32 = 0;
        while n < self.n_max && t_n < t_stop
        {
            h = h.min(t_stop - t_n);
            //
            let (x_n_new, x_ne): (Vector<T>, Vector<T>) = method.do_step(prob, &t_n, &x_n, &h);
            let e: T = (x_n_new.clone() - x_ne.clone()).p_norm(&T::from_f64(2.0).unwrap());

            let mut s: T = T::one();

            if e != T::zero()
            {
                s = (self.e_max * h / (T::from_f64(2.0).unwrap() * e)).pow(&T::from_f64(0.25).unwrap());
            }

            if e <= self.e_max
            {
                t_n = t_n + h;
                x_n = x_n_new;
                t_vec.push(t_n);
                res_vec.push(x_n.clone());
				n = n + 1;
				h = s * h;
            }
            else
            {
                h = s * h;
            }


            if h < self.h_min
            {
                h = self.h_min;
            }
            if h > self.h_max
            {
                h = self.h_max;
            }
        }
        return Ok((t_vec, res_vec));
    }
}