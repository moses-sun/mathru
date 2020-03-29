//! Solves an ODE using the 4th order Runge-Kutta-Fehlberg algorithm.
use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::explicit_method::{ExplicitAdaptiveMethod};
use super::ExplicitODE;
use std::default::Default;
use super::adaptive_stepper::AdaptiveStepper;

/// Solves an ODE using the 4th order Runge-Kutta-Fehlberg algorithm.
///
/// ```math
/// order \mathcal{O}(h^4) with an error estimator of order \mathcal{O}(h^5)
/// ```
///
///<a href="https://en.wikipedia.org/wiki/Runge-Kutta-Fehlberg_method">https://en.wikipedia
/// .org/wiki/Runge-Kutta-Fehlberg_method</a>
///
/// # Example
///
/// For this example, we want to solve the following ordinary differiential equation:
/// ```math
/// \frac{dy}{dt} = ay = f(t, y)
/// ```
/// The inial condition is $`y(0) = 0.5`$ and we solve it in the interval $`\lbrack 0, 2\rbrack`$
/// The following equation is the closed solution for this ODE:
/// ```math
/// y(t) = C a e^{at}
/// ```
/// $`C`$ is a parameter and depends on the initial condition $`y(t_{0})`$
/// ```math
/// C = \frac{y(t_{0})}{ae^{at_{0}}}
/// ```
///
/// In this example, we set $`a=2`$
/// ```
/// # #[macro_use]
/// # extern crate mathru;
/// # fn main()
/// # {
///	use mathru::algebra::linear::{Vector};
///	use mathru::analysis::ode::{ExplicitODE, RungeKuttaFehlberg54};
///
/// pub struct ExplicitODE1
/// {
///	    time_span: (f64, f64),
///     init_cond: Vector<f64>
/// }
///
/// impl Default for ExplicitODE1
/// {
///	    fn default() -> ExplicitODE1
///	    {
///         ExplicitODE1
///         {
///             time_span: (0.0, 2.0),
///             init_cond: vector![0.5]
///	        }
///     }
/// }
///
/// impl ExplicitODE<f64> for ExplicitODE1
/// {
/// 	fn func(self: &Self, _t: &f64, x: &Vector<f64>) -> Vector<f64>
///     {
///		    return x * &2.0f64;
///     }
///
///     fn time_span(self: &Self) -> (f64, f64)
///     {
///     	return self.time_span;
///     }
///
///     fn init_cond(self: &Self) -> Vector<f64>
///	    {
///	        return self.init_cond.clone();
///     }
/// }
///
/// // We instanciate CashKarp algorithm
/// let h_0: f64 = 0.0001;
/// let fac: f64 = 0.9;
/// let fac_min: f64 = 0.01;
///	let fac_max: f64 = 2.0;
///	let n_max: u32 = 100;
/// let abs_tol: f64 = 10e-6;
///	let rel_tol: f64 = 10e-3;
///
///	let solver: RungeKuttaFehlberg54<f64> = RungeKuttaFehlberg54::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);
///
/// let problem: ExplicitODE1 = ExplicitODE1::default();
///
/// // Solve the ODE
/// let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();
///
/// # }
/// ```
pub struct RungeKuttaFehlberg54<T>
{
    stepper: AdaptiveStepper<T>
}

impl<T> RungeKuttaFehlberg54<T>
    where T: Real
{
    /// Creates a Runge Kuttea Fehlberg 54 instance, also known as Runge-Kutta-Fehlberg
    ///
    pub fn new(n_max: u32, h_0: T, fac: T, fac_min: T, fac_max: T, abs_tol: T, rel_tol: T) -> RungeKuttaFehlberg54<T>
    {
        return RungeKuttaFehlberg54
        {
            stepper: AdaptiveStepper::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol)
        }
    }

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

impl<T> Default for RungeKuttaFehlberg54<T>
    where T: Real
{

    fn default() -> RungeKuttaFehlberg54<T>
    {
        let h_0: T = T::from_f64(0.0001);
        let fac: T = T::from_f64(0.9);
        let fac_min: T = T::from_f64(0.01);
        let fac_max: T = T::from_f64(2.0);
        let n_max: u32 = 100;
        let abs_tol: T = T::from_f64(10e-6);
        let rel_tol: T = T::from_f64(10e-3);
        return RungeKuttaFehlberg54::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);
    }
}

impl<T> ExplicitAdaptiveMethod<T> for RungeKuttaFehlberg54<T>
    where T: Real
{

    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> (Vector<T>, Vector<T>)
        where F: ExplicitODE<T>
    {
        // k_1 = hf(t_n, x_n)
        let k_1: Vector<T> = &prob.func(t_n, x_n) * h;

        // k_2 = h f(t_n + h/4, x_n + k1/4);
        let k_2: Vector<T> = &prob.func(&(*t_n + *h / T::from_f64(4.0)), &(x_n + &(&k_1 / &T::from_f64(4.0)
        ))) * h;

        //k_3 = h f(t_n + h3/8, x_n + k_1*3/32 + k2*9/32)
        let k_31: Vector<T> = x_n + &(&k_1 * &T::from_f64(3.0 / 32.0));
        let k_32: Vector<T> = k_31 + (&k_2 * &T::from_f64(9.0 / 32.0));
        let k_3: Vector<T> = &prob.func(&(*t_n + *h * T::from_f64(3.0 / 8.0)),  &k_32) * h;

        // k_4 = h f(t_n + 12/13h, x_n + 1932/2197k_1 - 7200/2197k_2 + 7296/2197k_3)
        let k_41: Vector<T> = x_n + &(&k_1 * &T::from_f64(1932.0 / 2197.0));
        let k_42: Vector<T> = k_41 - (&k_2 * &T::from_f64(7200.0 / 2197.0));
        let k_43: Vector<T> = k_42 + (&k_3 * &T::from_f64(7296.0 / 2197.0));
        let k_4: Vector<T> = &prob.func(&(*t_n + *h * T::from_f64(12.0 / 13.0)), &k_43) * h;

        // k_5 = h f(t_n + h, x_n + 439/216k_1 - 8k_2 + 3680/513k_3 - 845/4104k_4)
        let k_51: Vector<T> = x_n + &(&k_1 * &T::from_f64(439.0 / 216.0));
        let k_52: Vector<T> = k_51 - (&k_2 * &T::from_f64(8.0));
        let k_53: Vector<T> = k_52 + (&k_3 * &T::from_f64(3680.0 / 513.0));
        let k_54: Vector<T> = k_53 - (&k_4 * &T::from_f64(845.0 / 4104.0));
        let k_5: Vector<T> = &prob.func(&(*t_n + *h) , &k_54) * h;


        // k_6 = h f(t_n + h/2, x_n - 8/27k_1 + 2k_2 - 3544/2565k_3 + 1859/4104k_4 - 11/40k_5)
        let k_61: Vector<T> = x_n - &(&k_1 * &T::from_f64(8.0 / 27.0));
        let k_62: Vector<T> = k_61 + (&k_2 * &T::from_f64(2.0));
        let k_63: Vector<T> = k_62 - (&k_3 * &T::from_f64(3544.0 / 2565.0));
        let k_64: Vector<T> = k_63 + (&k_4 * &T::from_f64(1859.0 / 4104.0));
        let k_65: Vector<T> = k_64 - (&k_5 * &T::from_f64(11.0 / 40.0));
        let k_6: Vector<T> = &prob.func(&(*t_n + *h * T::from_f64(0.5)), &k_65) * h;

        // order 4
        // x_(n +1) = x_n + 25/216k_1 + 1408/2565k_3 + 2197/4104k_4 - 1/5k_5
        let rkf4_1: Vector<T> = x_n + &(&k_1 * &T::from_f64(25.0 / 216.0));
        let rkf4_2: Vector<T> = rkf4_1 + (&k_3 * &T::from_f64(1408.0 / 2565.0));
        let rkf4_3: Vector<T> = rkf4_2 + (&k_4 * &T::from_f64(2197.0 / 4104.0));
        let rkf4: Vector<T> = rkf4_3 - (&k_5 * &T::from_f64(0.2));

        // order 5
        // y_(n +1) = x_n + 16/135k_1 + 6656/12825k_3 + 28561/56430k_4 - 9/50k_5 + 2/55k6
        let rkf5_1: Vector<T> = x_n + &(&k_1 * &T::from_f64(16.0 / 135.0));
        let rkf5_2: Vector<T> = rkf5_1 + (&k_3 * &T::from_f64(6656.0 / 12825.0));
        let rkf5_3: Vector<T> = rkf5_2 + (&k_4 * &T::from_f64(28561.0 / 56430.0));
        let rkf5_4: Vector<T> = rkf5_3 - (&k_5 * &T::from_f64(9.0 / 50.0));
        let rkf5: Vector<T> = rkf5_4 + (&k_6 * &T::from_f64(2.0 / 55.0));

        return (rkf4, rkf5);
    }


    fn order(self: &Self) -> (u8, u8)
    {
        return (4, 5);
    }

}