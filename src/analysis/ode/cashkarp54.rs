use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::explicit_method::{ExplicitAdaptiveMethod};
use super::ExplicitODE;
use std::default::Default;
use super::adaptive_stepper::AdaptiveStepper;

/// Solves an ordinary differential equation using the 4th order Cash-Karp algorithm.
///
///<a href="https://en.wikipedia.org/wiki/Cash-Karp_method">https://en.wikipedia.org/wiki/Cash-Karp_method</a>
pub struct CashKarp54<T>
{
    stepper: AdaptiveStepper<T>
}

impl<T> CashKarp54<T>
    where T: Real
{
     /// Creates a Cash-Karp 4 (5) instance
    ///
    pub fn new(n_max: u32, h_0: T, fac: T, fac_min: T, fac_max: T, abs_tol: T, rel_tol: T) -> CashKarp54<T>
    {
        return CashKarp54
        {
            stepper: AdaptiveStepper::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol)
        }
    }
    /// # Example
    ///
    /// ```
    /// use mathru::*;
    /// use mathru::algebra::linear::{Vector, Matrix};
    /// use mathru::analysis::ode::{ExplicitODE, CashKarp54};
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
	///	let solver: CashKarp54<f64> = CashKarp54::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);
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

impl<T> Default for CashKarp54<T>
    where T: Real
{

    fn default() -> CashKarp54<T>
    {
        let h_0: T = T::from_f64(0.0001).unwrap();
        let fac: T = T::from_f64(0.9).unwrap();
        let fac_min: T = T::from_f64(0.01).unwrap();
        let fac_max: T = T::from_f64(2.0).unwrap();
        let n_max: u32 = 100;
        let abs_tol: T = T::from_f64(10e-6).unwrap();
        let rel_tol: T = T::from_f64(10e-3).unwrap();
        return CashKarp54::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);
    }
}

impl<T> ExplicitAdaptiveMethod<T> for CashKarp54<T>
    where T: Real
{

    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> (Vector<T>, Vector<T>)
        where F: ExplicitODE<T>
    {
        // k_1 = hf(t_n, x_n)
        let k_1: Vector<T> = &prob.func(t_n, x_n) * h;

        // k_2 = h f(t_n + h/5, x_n + k1/5);
        let k_2: Vector<T> = &prob.func(&(*t_n + *h / T::from_f64(1.0 / 5.0).unwrap()), &(x_n + &(&k_1 / &T::from_f64(1.0
        / 5.0).unwrap()))) * h;

        //k_3 = h f(t_n + h3/10, x_n + k_1*3/40 + k2*9/40)
        let k_31: Vector<T> = x_n + &(&k_1 * &T::from_f64(3.0 / 40.0).unwrap());
        let k_32: Vector<T> = k_31 + (&k_2 * &T::from_f64(9.0 / 40.0).unwrap());
        let k_3: Vector<T> = &prob.func(&(*t_n + *h * T::from_f64(3.0 / 10.0).unwrap()),  &k_32) * h;

        // k_4 = h f(t_n + 3/5h, x_n + 3/10k_1 - 9/10k_2 + 6/5k_3)
        let k_41: Vector<T> = x_n + &(&k_1 * &T::from_f64(3.0 / 10.0).unwrap());
        let k_42: Vector<T> = k_41 - (&k_2 * &T::from_f64(9.0 / 10.0).unwrap());
        let k_43: Vector<T> = k_42 + (&k_3 * &T::from_f64(6.0 / 5.0).unwrap());
        let k_4: Vector<T> = &prob.func(&(*t_n + *h * T::from_f64(3.0 / 5.0).unwrap()), &k_43) * h;

        // k_5 = h f(t_n + h, x_n - 11/54k_1 + 5/2k_2 - 70/27k_3 + 35/27k_4)
        let k_51: Vector<T> = x_n - &(&k_1 * &T::from_f64(11.0 / 54.0).unwrap());
        let k_52: Vector<T> = k_51 + (&k_2 * &T::from_f64(5.0 / 2.0).unwrap());
        let k_53: Vector<T> = k_52 - (&k_3 * &T::from_f64(70.0 / 27.0).unwrap());
        let k_54: Vector<T> = k_53 + (&k_4 * &T::from_f64(35.0 / 27.0).unwrap());
        let k_5: Vector<T> = &prob.func(&(*t_n + *h) , &k_54) * h;

        // k_6 = h f(t_n + 7/8h, x_n + 1631/55296k_1 + 175/512k_2 + 575/13824k_3 + 44275/110592k_4 - 253/4096k_5)
        let k_61: Vector<T> = x_n + &(&k_1 * &T::from_f64(1631.0 / 55296.0).unwrap());
        let k_62: Vector<T> = k_61 + (&k_2 * &T::from_f64(175.0 / 512.0).unwrap());
        let k_63: Vector<T> = k_62 + (&k_3 * &T::from_f64(575.0 / 13824.0).unwrap());
        let k_64: Vector<T> = k_63 + (&k_4 * &T::from_f64(44275.0 / 110592.0).unwrap());
        let k_65: Vector<T> = k_64 + (&k_5 * &T::from_f64(253.0 / 4096.0).unwrap());
        let k_6: Vector<T> = &prob.func(&(*t_n + *h * T::from_f64(7.0 / 8.0).unwrap()), &k_65) * h;

        // order 4
        // y(n +1) = x_n + 2825/27648k_1 + 18575/48384k_3 + 13525/55296k_4 + 277/14336k_5 + 1/4k_6
        let ck4_1: Vector<T> = x_n + &(&k_1 * &T::from_f64(2825.0 / 27648.0).unwrap());
        let ck4_2: Vector<T> = ck4_1 + (&k_3 * &T::from_f64(18575.0 / 48384.0).unwrap());
        let ck4_3: Vector<T> = ck4_2 + (&k_4 * &T::from_f64(13525.0 / 55296.0).unwrap());
        let ck4_4: Vector<T> = ck4_3 + (&k_5 * &T::from_f64(277.0 / 14336.0).unwrap());
        let ck4: Vector<T> = ck4_4 + (&k_6 * &T::from_f64(1.0 / 4.0).unwrap());

        // order 5
        // y_(n + 1) = x_n + 37/378k_1 + 250/621k_3 + 125/594k_4 + 512/1771k_6)
        let ck5_1: Vector<T> = x_n + &(&k_1 * &T::from_f64(37.0 / 378.0).unwrap());
        let ck5_2: Vector<T> = ck5_1 + (&k_3 * &T::from_f64(250.0 / 621.0).unwrap());
        let ck5_3: Vector<T> = ck5_2 + (&k_4 * &T::from_f64(125.0 / 594.0).unwrap());
        let ck5: Vector<T> = ck5_3 + (&k_6 * &T::from_f64(512.0 / 1771.0).unwrap());

        return (ck4, ck5);

    }

    fn order(self: &Self) -> (u8, u8)
    {
        return (4, 5);
    }

}