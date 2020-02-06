use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::explicit_method::{ExplicitAdaptiveMethod};
use super::ExplicitODE;

/// Solves an ordinary differential equation using the 4th order Tsitouras algorithm
///
///<a href="http://users.ntua.gr/tsitoura/RK54_new_v2.pdf">http://users.ntua.gr/tsitoura/RK54_new_v2.pdf</a>
pub struct Tsitouras54<T>
{
    abs_tol: T,
    h_0: T,
    n_max: u32,
}

impl<T> Tsitouras54<T>
    where T: Real
{
    /// Creates a CashKarp45 instance
    ///
    /// # Arguments
    ///
    /// *'abs_tol': Absolut tolerance
    pub fn new(abs_tol: T, begin_step_size: T, n_max: u32) -> Tsitouras54<T>
    {

        return Tsitouras54
        {
            abs_tol: abs_tol,
            h_0: begin_step_size,
            n_max: n_max,
        }
    }

    pub fn get_begin_step_size(self: &Self) -> &T
    {
        return &self.h_0;
    }

    pub fn set_begin_step_size(self: &mut Self, step_size: T)
    {
        self.h_0 = step_size
    }

    pub fn solve<F>(self: &Self, prob: &F) -> Result<(Vec<T>, Vec<Vector<T>>), &'static str>
        where F: ExplicitODE<T>,
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
        let mut h_n: T = self.h_0;

        let mut t_vec: Vec<T> = Vec::new();
        t_vec.push(t_n);

        let mut res_vec: Vec<Vector<T>> = Vec::new();
        res_vec.push(x_n.clone());

        let mut n: u32 = 0;

        while n < self.n_max && t_n < t_stop
        {
            h_n = h_n.min(t_stop - t_n);
            //
            let (x_n_new, x_ne): (Vector<T>, Vector<T>) = self.do_step(prob, &t_n, &x_n, &h_n);
            let error_n: T = self.calc_error(&x_n_new, &x_ne);

            if error_n <= self.abs_tol
            {
                t_n = t_n + h_n;
                x_n = x_n_new;
                t_vec.push(t_n);
                res_vec.push(x_n.clone());
				n = n + 1;
            }

            //Update step size
            h_n = self.calc_step_size(h_n, error_n);

        }

        if !(t_n < t_stop)
        {
            return Err("Maxmimum number of iterations reached");
        }
        unimplemented!("Koeffizienten sind noch nicht angepasst");
        return Ok((t_vec, res_vec));
    }

    /// calculate new step size
    fn calc_step_size(self: &Self, h_n: T, error_n: T) -> T
    {
        return T::from_f64(0.9).unwrap() * h_n * (self.abs_tol / error_n).pow(&(T::one() / T::from_f64(5.0).unwrap()))
    }


    fn calc_error(self: &Self, y: &Vector<T>, y_hat: &Vector<T>) -> T
    {
        return (y - y_hat).p_norm(&T::from_f64(2.0).unwrap());
    }
}

impl<T> ExplicitAdaptiveMethod<T> for Tsitouras54<T>
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
        return (5, 4);
    }

}