use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::{ExplicitODE, ExplicitAdaptiveMethod};
use std::marker::PhantomData;

/// Solves an ordinary differential equation using the 4th order Runge-Kutta-Dormand-Prince algorithm.
///
///<a href="https://en.wikipedia.org/wiki/Dormand-Prince_method">https://en.wikipedia.org/wiki/Dormand-Prince_method</a>
pub struct DormandPrince45<T>
{
    phantom: PhantomData<T>,
}

impl<T> DormandPrince45<T>
    where T: Real
{
    /// Creates a DormandPrince45 instance, also known as explicit Runge-Kutta method of order (4)5 with step-size control
    ///
    ///
    pub fn new() -> DormandPrince45<T>
    {

        return DormandPrince45
        {
            phantom: PhantomData
        }
    }
}

impl<T> ExplicitAdaptiveMethod<T> for DormandPrince45<T>
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

        // k_4 = h f(t_n + 4/5h, x_n + 44/45k_1 - 56/15k_2 + 32/9k_3)
        let k_41: Vector<T> = x_n + &(&k_1 * &T::from_f64(44.0 / 45.0).unwrap());
        let k_42: Vector<T> = k_41 - (&k_2 * &T::from_f64(56.0 / 15.0).unwrap());
        let k_43: Vector<T> = k_42 + (&k_3 * &T::from_f64(32.0 / 9.0).unwrap());
        let k_4: Vector<T> = &prob.func(&(*t_n + *h * T::from_f64(4.0 / 5.0).unwrap()), &k_43) * h;

        // k_5 = h f(t_n + h, x_n + 19372/6561k_1 - 25360/2187k_2 + 64448/6561k_3 - 212/729k_4)
        let k_51: Vector<T> = x_n + &(&k_1 * &T::from_f64(19372.0 / 6561.0).unwrap());
        let k_52: Vector<T> = k_51 - (&k_2 * &T::from_f64( 25360.0 / 2187.0).unwrap());
        let k_53: Vector<T> = k_52 + (&k_3 * &T::from_f64(64448.0 / 6561.0).unwrap());
        let k_54: Vector<T> = k_53 - (&k_4 * &T::from_f64(212.0 / 729.0).unwrap());
        let k_5: Vector<T> = &prob.func(&(*t_n + *h * T::from_f64(8.0 / 9.0).unwrap()) , &k_54) * h;

        // k_6 = h f(t_n + h, x_n + 9017/3168k_1 - 355/33k_2 + 46732/5247k_3 + 49/176k_4 - 5103/18656k_5)
        let k_61: Vector<T> = x_n + &(&k_1 * &T::from_f64(9017.0 / 3168.0).unwrap());
        let k_62: Vector<T> = k_61 - (&k_2 * &T::from_f64(355.0 / 33.0).unwrap());
        let k_63: Vector<T> = k_62 + (&k_3 * &T::from_f64(46732.0 / 5247.0).unwrap());
        let k_64: Vector<T> = k_63 + (&k_4 * &T::from_f64(49.0 / 176.0).unwrap());
        let k_65: Vector<T> = k_64 - (&k_5 * &T::from_f64(5103.0 / 18656.0).unwrap());
        let k_6: Vector<T> = &prob.func(&(*t_n + *h), &k_65) * h;

        // k_7 = h f(t_n + h, x_n + 35/384k_1 + 500/1113k_3 + 125/192k_4 - 2187/6784k_5 + 11/84k_6)
        let k_71: Vector<T> = x_n + &(&k_1 * &T::from_f64(35.0 / 384.0).unwrap());
        let k_72: Vector<T> = k_71 + (&k_3 * &T::from_f64(500.0 / 1113.0).unwrap());
        let k_73: Vector<T> = k_72 + (&k_4 * &T::from_f64(125.0 / 192.0).unwrap());
        let k_74: Vector<T> = k_73 - (&k_5 * &T::from_f64(2187.0 / 6784.0).unwrap());
        let k_75: Vector<T> = k_74 + (&k_6 * &T::from_f64(11.0 / 84.0).unwrap());
        let k_7: Vector<T> = &prob.func(&(*t_n + *h), &k_75) * h;

        let rkdp4: Vector<T> = k_75;

        // y_(n +1) = x_n + 5179/57600k_1 + 7571/16695k_3 + 393/640k_4 - 92097/339200k_5 + 187/2100k_6 + 1/40k_7
        let rkdp5_1: Vector<T> = x_n + &(&k_1 * &T::from_f64(5179.0 / 57600.0).unwrap());
        let rkdp5_2: Vector<T> = rkdp5_1 + (&k_3 * &T::from_f64(7571.0 / 16695.0).unwrap());
        let rkdp5_3: Vector<T> = rkdp5_2 + (&k_4 * &T::from_f64(393.0 / 640.0).unwrap());
        let rkdp5_4: Vector<T> = rkdp5_3 - (&k_5 * &T::from_f64(92097.0 / 339200.0).unwrap());
        let rkdp5_5: Vector<T> = rkdp5_4 + (&k_6 * &T::from_f64(187.0 / 2100.0).unwrap());
        let rkdp5: Vector<T> = rkdp5_5 + (&k_7 * &T::from_f64(1.0 / 40.0).unwrap());
        return (rkdp4, rkdp5);
    }

}