use crate::analysis::ode::solver::{Solver, ExplicitODE};
use crate::algebra::linear::vector::vector::Vector;
use crate::algebra::abstr::Real;
use crate::algebra::abstr::Zero;

///
/// Adams-Bashforth method
pub struct AdamsBashforth<T>
{
	k: u8,
	step_size: T
}

impl<T> AdamsBashforth<T>
	where T: Real
{
	///
	pub fn new(k: u8, step_size: T) -> AdamsBashforth<T>
	{
		if k == 0 || k >= 6
		{
			panic!();
		}
		if step_size <= T::zero()
        {
            panic!();
        }

		return
		AdamsBashforth
		{
			k: k,
			step_size: step_size
		}
	}
}

impl<T> Solver<T> for AdamsBashforth<T>
    where T: Real
{

    /// Solves `func` using Adams' method.
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
    /// if t_span.0 > t_span.1
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::*;
    /// use mathru::algebra::linear::{Vector, Matrix};
    /// use mathru::analysis::ode::{Solver, ExplicitODE, AdamsBashforth};
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
    /// let problem: ExplicitODEProblem = ExplicitODEProblem::default();
    ///	let solver: AdamsBashforth<f64> = AdamsBashforth::new(1, 0.001);
    ///
    /// let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();
    ///
    /// ```
	fn solve<F>(self: &Self, prob: &F) -> Result<(Vec<T>, Vec<Vector<T>>), ()>
        where F: ExplicitODE<T>
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

        let limit = ((t_stop - t_start) / self.step_size).ceil() + T::one();
        let steps: usize = limit.to_u64().unwrap() as usize;
        let mut t_vec: Vec<T> = Vec::with_capacity(steps);
        let mut res_vec: Vec<Vector<T>> = Vec::with_capacity(steps);

     	let mut res_vec: Vec<Vector<T>> = Vec::with_capacity(steps);

        for _i in 0..steps
        {
            //Step size
            let h: T = self.step_size.min(t_stop - t_n);

            t_vec.push(t_n);
            res_vec.push(x_n.clone());

            x_n = &x_n + &(&prob.func(&t_n, &x_n) * &h);

            t_n = t_n + h;
		}
    	return Ok((t_vec, res_vec));
	}
}

impl<T> AdamsBashforth<T>
	where T: Real
{
	///
	/// J \in \[0, 8\]
	///
	fn coefficient(j: u32) -> T
	{
		match j
		{
			0 => return T::from_f64(1.0).unwrap(),
			1 => return T::from_f64(0.5).unwrap(),
			2 => return T::from_f64(5.0/12.0).unwrap(),
			3 => return T::from_f64(3.0/8.0).unwrap(),
			4 => return T::from_f64(251.0/720.0).unwrap(),
			5 => return T::from_f64(95.0/288.0).unwrap(),
			6 => return T::from_f64(19087.0/60480.0).unwrap(),
			7 => return T::from_f64(5257.0/17280.0).unwrap(),
			7 => return T::from_f64(1070017.0/3628800.0).unwrap(),
			_ => panic!(),
		}
	}
}