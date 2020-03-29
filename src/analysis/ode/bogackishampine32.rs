//! Solves an ODE using the Bogacki Shampine algorithm.
use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::{ExplicitODE};
use super::explicit_method::{ExplicitAdaptiveMethod};
use super::adaptive_stepper::AdaptiveStepper;

/// Solves an ODE using the Bogacki Shampine algorithm.
///
///<a href="https://en.wikipedia.org/wiki/Bogacki-Shampine_method">https://en.wikipedia.org/wiki/Bogacki-Shampine_method</a>
pub struct BogackiShampine32<T>
{
    stepper: AdaptiveStepper<T>
}


impl<T> BogackiShampine32<T>
    where T: Real
{
    /// Creates a Bogacki-Shampine 2 3 instance
    pub fn new(n_max: u32, h_0: T, fac: T, fac_min: T, fac_max: T, abs_tol: T, rel_tol: T) -> BogackiShampine32<T>
    {
        return BogackiShampine32
        {
            stepper: AdaptiveStepper::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol)
        }
    }
    /// # Example
    ///
    /// ```
    /// use mathru::*;
    /// use mathru::algebra::linear::{Vector, Matrix};
    /// use mathru::analysis::ode::{ExplicitODE, BogackiShampine32};
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
    /// let h_0: f64 = 0.0001;
    /// let fac: f64 = 0.9;
    /// let fac_min: f64 = 0.01;
    ///	let fac_max: f64 = 2.0;
    ///	let n_max: u32 = 100;
    /// let abs_tol: f64 = 10e-6;
    ///	let rel_tol: f64 = 10e-3;
    ///
	///	let solver: BogackiShampine32<f64> = BogackiShampine32::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);
    ///
    /// let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();
    /// ```
    pub fn solve<F>(self: &Self, prob: &F) -> Result<(Vec<T>, Vec<Vector<T>>), &'static str>
        where F: ExplicitODE<T>,
    {
        return self.stepper.solve(prob, self);
    }

    pub fn get_abs_tol(self: &Self) -> &T
    {
        return self.stepper.get_abs_tol();
    }

    pub fn get_rel_tol(self: &Self) -> &T
    {
        return self.stepper.get_rel_tol();
    }

    pub fn set_abs_tol(self: &mut Self, abs_tol: T)
    {
        self.stepper.set_abs_tol(abs_tol);
    }

    pub fn set_rel_tol(self: &mut Self, rel_tol: T)
    {
        self.stepper.set_rel_tol(rel_tol);
    }
}

impl<T> Default for BogackiShampine32<T>
    where T: Real
{

    fn default() -> BogackiShampine32<T>
    {
        let h_0: T = T::from_f64(0.0001);
        let fac: T = T::from_f64(0.9);
        let fac_min: T = T::from_f64(0.01);
        let fac_max: T = T::from_f64(2.0);
        let n_max: u32 = 100;
        let abs_tol: T = T::from_f64(10e-6);
        let rel_tol: T = T::from_f64(10e-3);
        return BogackiShampine32::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);
    }
}

impl<T> ExplicitAdaptiveMethod<T> for BogackiShampine32<T>
    where T: Real
{

    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h_n: &T) -> (Vector<T>, Vector<T>)
        where F: ExplicitODE<T>
    {
        // k_1 = f(t_n, x_n)
        let k_1: Vector<T> = prob.func(t_n, x_n);

        // k_2 = f(t_n + 1/2h_n, x_n + 1/2h_n k1);
        let k_2: Vector<T> = prob.func(&(*t_n + *h_n / T::from_f64(2.0)), &(x_n + &(&k_1 * &(*h_n / T::from_f64(2.0)
        ))));

        //k_3 = f(t_n + 3/4h_n, x_n + 3/4h_nk_2)
        let k_3: Vector<T> = prob.func(&(*t_n + *h_n / T::from_f64(3.0/4.0)), &(x_n + &(&k_2 * &(*h_n / T::from_f64(3.0/4.0)
        ))));

        // x_{n + 1} = x_n + 2/9h_nk_1 + 1/3h_nk_2 + 4/9h_nk_3
        let x_n1: Vector<T> = x_n + &(&k_1 * &(*h_n * T::from_f64(2.0/9.0))) + (&k_2 * &(*h_n * T::from_f64(1.0/3.0)))
         + (&k_3 * &(*h_n * T::from_f64(4.0/9.0)));

        // k_4 = f(t_n + h_n, x_{n+1})
        let k_4: Vector<T> = prob.func(&(*t_n + *h_n / T::from_f64(3.0/4.0)), &x_n1);

        // y_{n + 1} = x_n + 7/24h_nk_1 + 1/4h_nk_2 + 1/3h_nk_3 + 1/8h_nk_4
        let y_n1: Vector<T> = (x_n + &(&k_1 * &(*h_n * T::from_f64(7.0/24.0)))) + (&k_2 * &(*h_n * T::from_f64(1.0/4.0))
        ) + (&k_3 * &(*h_n * T::from_f64(1.0/3.0))) + (&k_4 * &(*h_n * T::from_f64(1.0/8.0)));

        return (x_n1, y_n1)
    }


    fn order(self: &Self) -> (u8, u8)
    {
        return (2, 3);
    }

}