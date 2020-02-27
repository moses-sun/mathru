use crate::analysis::ode::explicit_ode::{ExplicitODE};
use crate::algebra::linear::vector::vector::Vector;
use crate::algebra::abstr::Real;

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
		if k == 0 || k > 5
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

impl<T> AdamsBashforth<T>
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
    /// use mathru::analysis::ode::{ExplicitODE, AdamsBashforth};
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
	pub fn solve<F>(self: &Self, prob: &F) -> Result<(Vec<T>, Vec<Vector<T>>), ()>
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

		t_vec.push(t_n);
     	res_vec.push(x_n.clone());

		//calculate initial steps
		if self.k >= 2
		{
			let h: T = self.step_size.min(t_stop - t_n);
			let x_n = AdamsBashforth::step_s1(prob, &t_vec, &res_vec, h);
			t_n = t_n + h;

			t_vec.push(t_n);
            res_vec.push(x_n.clone());
		}

		if self.k >= 3
		{
			let h: T = self.step_size.min(t_stop - t_n);
			let x_n = AdamsBashforth::step_s2(prob, &t_vec, &res_vec, h);
			t_n = t_n + h;

			t_vec.push(t_n);
            res_vec.push(x_n.clone());
		}

		if self.k >= 4
		{
			let h: T = self.step_size.min(t_stop - t_n);
			let x_n = AdamsBashforth::step_s3(prob, &t_vec, &res_vec, h);
			t_n = t_n + h;

			t_vec.push(t_n);
            res_vec.push(x_n.clone());
		}

		if self.k >= 5
		{
			let h: T = self.step_size.min(t_stop - t_n);
			let x_n = AdamsBashforth::step_s4(prob, &t_vec, &res_vec, h);
			t_n = t_n + h;

			t_vec.push(t_n);
            res_vec.push(x_n.clone());
		}

		let step = match self.k
		{
			1 => AdamsBashforth::step_s1,
			2 => AdamsBashforth::step_s2,
			3 => AdamsBashforth::step_s3,
			4 => AdamsBashforth::step_s4,
			5 => AdamsBashforth::step_s5,
			_ => panic!(),
		};

		println!("{}", t_vec.len());

		while (t_n - t_stop).abs() > T::from_f64(0.0000000001).unwrap()
        {
 			//Step size
            let h: T = self.step_size.min(t_stop - t_n);

			x_n = step(prob, &t_vec, &res_vec, h);
            t_n = t_n + h;

        	t_vec.push(t_n);
            res_vec.push(x_n.clone());
		}

    	return Ok((t_vec, res_vec));
	}
}

impl<T> AdamsBashforth<T>
	where T: Real
{
	fn step_s1<F>(prob: &F, t: &Vec<T>, x: &Vec<Vector<T>>, h: T) -> Vector<T>
		where F: ExplicitODE<T>
	{
		let n: usize = x.len() - 1;
		let x_n: &Vector<T> = &x[n];
		let t_n: &T = &t[n];
		return x_n + &(&prob.func(t_n, x_n) * &h);
	}

	fn step_s2<F>(prob: &F, t: &Vec<T>, x: &Vec<Vector<T>>, h: T) -> Vector<T>
		where F: ExplicitODE<T>
	{
		let n: usize = x.len() - 1;
		let x_n: &Vector<T> = &x[n];
		let t_n: &T = &t[n];
		let x_n1: &Vector<T> = &x[n - 1];
		let t_n1: &T = &t[n - 1];
		return x_n + &((prob.func(t_n, x_n) * T::from_f64(3.0/2.0).unwrap()  + prob.func(&t_n1, x_n1) * T::from_f64(-0.5).unwrap()) * h);
	}

	fn step_s3<F>(prob: &F, t: &Vec<T>, x: &Vec<Vector<T>>, h: T) -> Vector<T>
		where F: ExplicitODE<T>
	{
		let n: usize = x.len() - 1;
		let x_n: &Vector<T> = &x[n];
		let t_n: &T = &t[n];
		let x_n1: &Vector<T> = &x[n - 1];
		let t_n1: &T = &t[n - 1];
		let x_n2: &Vector<T> = &x[n - 2];
		let t_n2: &T = &t[n - 2];
		return x_n + &((prob.func(t_n, x_n) * T::from_f64(23.0/12.0).unwrap() + prob.func(t_n1, x_n1) * T::from_f64(-16.0/12.0).unwrap()
		+ prob.func(t_n2, x_n2) * T::from_f64(5.0/12.0).unwrap()) * h);
	}

	fn step_s4<F>(prob: &F, t: &Vec<T>, x: &Vec<Vector<T>>, h: T) -> Vector<T>
		where F: ExplicitODE<T>
	{
		let n: usize = x.len() - 1;
		let x_n: &Vector<T> = &x[n];
		let t_n: &T = &t[n];
		let x_n1: &Vector<T> = &x[n - 1];
		let t_n1: &T = &t[n - 1];
		let x_n2: &Vector<T> = &x[n - 2];
		let t_n2: &T = &t[n - 2];
		let x_n3: &Vector<T> = &x[n - 3];
		let t_n3: &T = &t[n - 3];
		return x_n + &((prob.func(t_n, x_n) * T::from_f64(55.0/24.0).unwrap() + prob.func(t_n1, x_n1) * T::from_f64(-59.0/24.0).unwrap
		() + prob.func(t_n2, x_n2) * T::from_f64(37.0/24.0).unwrap() + prob.func(t_n3, x_n3) * T::from_f64(-9.0/24.0).unwrap()) * h);
	}

	fn step_s5<F>(prob: &F, t: &Vec<T>, x: &Vec<Vector<T>>, h: T) -> Vector<T>
		where F: ExplicitODE<T>
	{
		let n: usize = x.len() - 1;
		let x_n: &Vector<T> = &x[n];
		let t_n: &T = &t[n];
		let x_n1: &Vector<T> = &x[n - 1];
		let t_n1: &T = &t[n - 1];
		let x_n2: &Vector<T> = &x[n - 2];
		let t_n2: &T = &t[n - 2];
		let x_n3: &Vector<T> = &x[n - 3];
		let t_n3: &T = &t[n - 3];
		let x_n4: &Vector<T> = &x[n - 4];
		let t_n4: &T = &t[n - 4];
		return x_n + &((prob.func(t_n, x_n) * T::from_f64(1901.0/720.0).unwrap()  + prob.func(t_n1, x_n1) * T::from_f64(-2774.0/720.0)
		.unwrap() + prob.func(t_n2, x_n2) * T::from_f64(2616.0/720.0).unwrap() + prob.func(t_n3, x_n3) * T::from_f64(-1274.0/720.0)
		.unwrap() + prob.func(t_n4, x_n4) * T::from_f64(251.0/720.0).unwrap()) * h);
	}
}