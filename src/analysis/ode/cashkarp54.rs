use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::{ExplicitODE, ExplicitAdaptiveMethod};
use std::marker::PhantomData;

/// Solves an ordinary differential equation using the 4th order Cash-Karp algorithm.
///
///<a href="https://en.wikipedia.org/wiki/Cash-Karp_method">https://en.wikipedia.org/wiki/Cash-Karp_method</a>
pub struct CashKarp45<T>
{
    phantom: PhantomData<T>,
}

impl<T> CashKarp45<T>
    where T: Real
{
    /// Creates a CashKarp45 instance
    pub fn new() -> CashKarp45<T>
    {

        return CashKarp45
        {
            phantom: PhantomData
        }
    }
}

impl<T> ExplicitAdaptiveMethod<T> for CashKarp45<T>
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

        // y(n + 1) = x_n + 37/378k_1 + 250/621k_3 + 125/594k_4 + 512/1771k_6)
        let ck4_1: Vector<T> = x_n + &(&k_1 * &T::from_f64(37.0 / 378.0).unwrap());
        let ck4_2: Vector<T> = ck4_1 + (&k_3 * &T::from_f64(250.0 / 621.0).unwrap());
        let ck4_3: Vector<T> = ck4_2 + (&k_4 * &T::from_f64(125.0 / 594.0).unwrap());
        let ck4: Vector<T> = ck4_3 + (&k_6 * &T::from_f64(512.0 / 1771.0).unwrap());

        // y_(n +1) = x_n + 2825/27648k_1 + 18575/48384k_3 + 13525/55296k_4 + 277/14336k_5 + 1/4k_6
        let ck5_1: Vector<T> = x_n + &(&k_1 * &T::from_f64(2825.0 / 27648.0).unwrap());
        let ck5_2: Vector<T> = ck5_1 + (&k_3 * &T::from_f64(18575.0 / 48384.0).unwrap());
        let ck5_3: Vector<T> = ck5_2 + (&k_4 * &T::from_f64(13525.0 / 55296.0).unwrap());
        let ck5_4: Vector<T> = ck5_3 + (&k_5 * &T::from_f64(277.0 / 14336.0).unwrap());
        let ck5: Vector<T> = ck5_4 + (&k_6 * &T::from_f64(1.0 / 4.0).unwrap());
        return (ck4, ck5);

    }

    fn order(self: &Self) -> (u8, u8)
    {
        return (4, 5);
    }

}