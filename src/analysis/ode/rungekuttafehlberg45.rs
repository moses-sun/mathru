use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::{ExplicitODE, ExplicitAdaptiveMethod};
use std::marker::PhantomData;

/// Solves an ordinary differential equation using the 4th order Runge-Kutta-Fehlberg algorithm.
///
/// ```math
/// order \mathcal{O}(h^4) with an error estimator of order \mathcal{O}(h^5)
/// ```
///
///<a href="https://en.wikipedia.org/wiki/Runge-Kutta-Fehlberg_method">https://en.wikipedia
/// .org/wiki/Runge-Kutta-Fehlberg_method</a>
///
pub struct RungeKuttaFehlberg45<T>
{
    phantom: PhantomData<T>,
}

impl<T> RungeKuttaFehlberg45<T>
    where T: Real
{
    /// Creates a RKF45 instance, also known as Runge-Kutta-Fehlberg
    ///
    pub fn new() -> RungeKuttaFehlberg45<T>
    {
        return
        RungeKuttaFehlberg45
        {
            phantom: PhantomData
        }
    }
}

impl<T> ExplicitAdaptiveMethod<T> for RungeKuttaFehlberg45<T>
    where T: Real
{

    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> (Vector<T>, Vector<T>)
        where F: ExplicitODE<T>
    {
        // k_1 = hf(t_n, x_n)
        let k_1: Vector<T> = &prob.func(t_n, x_n) * h;

        // k_2 = h f(t_n + h/4, x_n + k1/4);
        let k_2: Vector<T> = &prob.func(&(*t_n + *h / T::from_f64(4.0).unwrap()), &(x_n + &(&k_1 / &T::from_f64(4.0).unwrap()
        ))) * h;

        //k_3 = h f(t_n + h3/8, x_n + k_1*3/32 + k2*9/32)
        let k_31: Vector<T> = x_n + &(&k_1 * &T::from_f64(3.0 / 32.0).unwrap());
        let k_32: Vector<T> = k_31 + (&k_2 * &T::from_f64(9.0 / 32.0).unwrap());
        let k_3: Vector<T> = &prob.func(&(*t_n + *h * T::from_f64(3.0 / 8.0).unwrap()),  &k_32) * h;

        // k_4 = h f(t_n + 12/13h, x_n + 1932/2197k_1 - 7200/2197k_2 + 7296/2197k_3)
        let k_41: Vector<T> = x_n + &(&k_1 * &T::from_f64(1932.0 / 2197.0).unwrap());
        let k_42: Vector<T> = k_41 - (&k_2 * &T::from_f64(7200.0 / 2197.0).unwrap());
        let k_43: Vector<T> = k_42 + (&k_3 * &T::from_f64(7296.0 / 2197.0).unwrap());
        let k_4: Vector<T> = &prob.func(&(*t_n + *h * T::from_f64(12.0 / 13.0).unwrap()), &k_43) * h;

        // k_5 = h f(t_n + h, x_n + 439/216k_1 - 8k_2 + 3680/513k_3 - 845/4104k_4)
        let k_51: Vector<T> = x_n + &(&k_1 * &T::from_f64(439.0 / 216.0).unwrap());
        let k_52: Vector<T> = k_51 - (&k_2 * &T::from_f64(8.0).unwrap());
        let k_53: Vector<T> = k_52 + (&k_3 * &T::from_f64(3680.0 / 513.0).unwrap());
        let k_54: Vector<T> = k_53 - (&k_4 * &T::from_f64(845.0 / 4104.0).unwrap());
        let k_5: Vector<T> = &prob.func(&(*t_n + *h) , &k_54) * h;


        // k_6 = h f(t_n + h/2, x_n - 8/27k_1 + 2k_2 - 3544/2565k_3 + 1859/4104k_4 - 11/40k_5)
        let k_61: Vector<T> = x_n - &(&k_1 * &T::from_f64(8.0 / 27.0).unwrap());
        let k_62: Vector<T> = k_61 + (&k_2 * &T::from_f64(2.0).unwrap());
        let k_63: Vector<T> = k_62 - (&k_3 * &T::from_f64(3544.0 / 2565.0).unwrap());
        let k_64: Vector<T> = k_63 + (&k_4 * &T::from_f64(1859.0 / 4104.0).unwrap());
        let k_65: Vector<T> = k_64 - (&k_5 * &T::from_f64(11.0 / 40.0).unwrap());
        let k_6: Vector<T> = &prob.func(&(*t_n + *h * T::from_f64(0.5).unwrap()), &k_65) * h;

        // order 4
        // x_(n +1) = x_n + 25/216k_1 + 1408/2565k_3 + 2197/4104k_4 - 1/5k_5
        let rkf4_1: Vector<T> = x_n + &(&k_1 * &T::from_f64(25.0 / 216.0).unwrap());
        let rkf4_2: Vector<T> = rkf4_1 + (&k_3 * &T::from_f64(1408.0 / 2565.0).unwrap());
        let rkf4_3: Vector<T> = rkf4_2 + (&k_4 * &T::from_f64(2197.0 / 4104.0).unwrap());
        let rkf4: Vector<T> = rkf4_3 - (&k_5 * &T::from_f64(0.2).unwrap());

        // order 5
        // y_(n +1) = x_n + 16/135k_1 + 6656/12825k_3 + 28561/56430k_4 - 9/50k_5 + 2/55k6
        let rkf5_1: Vector<T> = x_n + &(&k_1 * &T::from_f64(16.0 / 135.0).unwrap());
        let rkf5_2: Vector<T> = rkf5_1 + (&k_3 * &T::from_f64(6656.0 / 12825.0).unwrap());
        let rkf5_3: Vector<T> = rkf5_2 + (&k_4 * &T::from_f64(28561.0 / 56430.0).unwrap());
        let rkf5_4: Vector<T> = rkf5_3 - (&k_5 * &T::from_f64(9.0 / 50.0).unwrap());
        let rkf5: Vector<T> = rkf5_4 + (&k_6 * &T::from_f64(2.0 / 55.0).unwrap());

        return (rkf4, rkf5);
    }

}