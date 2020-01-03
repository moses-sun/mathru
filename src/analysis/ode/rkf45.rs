use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Real;
use super::Solver;

/// Solves an ordinary differential equation using the 4th order Runge-Kutta-Fehlberg algorithm.
///
///<a href="https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta%E2%80%93Fehlberg_method">https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta%E2%80%93Fehlberg_method</a>
pub struct RKF45<T>
{
    h_0: T,

    /// Step size min
    h_min: T,

    /// Step size max
    h_max: T,

    /// Maximum error
    e_max: T,

    n_max: u32
}

impl<T> RKF45<T>
    where T: Real
{
    /// Creates a RKF45 instance, also known as Runge-Kutta-Fehlberg
    ///
    /// # Argument
    ///
    /// * 'step_size'
    /// * 'h_min': Minimum step size
    /// * 'h_max': Maximum step size
    /// * 'e_max': Maximum error
    ///
    /// # Panics
    ///
    /// 'h_min' <= 0.0
    /// 'h_max' <= 'h_min'
    /// 'n_max' <= 0
    ///
    pub fn new(h_0: T, h_min: T, h_max: T, e_max: T, n_max: u32) -> RKF45<T>
    {
        if h_0 < h_min || h_0 > h_max || h_min <= T::zero() || h_max <= h_min || e_max <= T::zero() || n_max <= 0
        {
            panic!();
        }

        RKF45
        {
            h_0: h_0,
            h_min: h_min,
            h_max: h_max,
            e_max: e_max,
            n_max: n_max
        }
    }
}

impl<T> Solver<T> for RKF45<T>
    where T: Real
{

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
    /// use mathru::algebra::linear::{Vector};
    /// use mathru::analysis::ode::{Solver, RKF45};
    ///
    /// let f = |t: &f64, _x: &Vector<f64> | -> Vector<f64> { return Vector::new_row(1, vec![1.0]) * (t * &2.0f64); };
    ///
    ///	let init: Vector<f64> = vector![1.0];
    ///	let solver: RKF45<f64> = RKF45::new(0.001, 0.001, 0.01, 0.1, 3000);
    /// let t_start: f64 = 0.0;
    /// let t_stop: f64 = 2.0;
    ///
    ///	let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(f, init, (t_start, t_stop)).unwrap();
    /// ```
	fn solve<F>(self: &Self, func: F, init: Vector<T>, t_span: (T, T)) -> Result<(Vec<T>, Vec<Vector<T>>), ()>
        where F: Fn(&T, &Vector<T>) -> Vector<T>
    {
        let t_start: T = t_span.0;
        let t_stop: T = t_span.1;
        if t_start > t_stop
        {
            panic!();
        }

        let mut x_n: Vector<T> = init.clone();
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
            let (rkf4, rkf5): (Vector<T>, Vector<T>) = RKF45::calc_rkf4_rkf5(&t_n, &x_n, &func, &h);
            let e: T = (rkf4.clone() - rkf5.clone()).p_norm(&T::from_f64(2.0).unwrap());

            let mut s: T = T::one();

            if e != T::zero()
            {
                s = (self.e_max * h / (T::from_f64(2.0).unwrap() * e)).pow(&T::from_f64(0.25).unwrap());
            }

            if e <= self.e_max
            {
                t_n = t_n + h;
                x_n = rkf4;
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


//        //
//        let mut t_vector: Vector<T> = Vector::zero(t_vec.len());
//
//        for i in 0..t_vec.len()
//        {
//            *t_vector.get_mut(i) = t_vec[i];
//        }
//
//        //
//        let mut res_vec: Vec<Vector<T>> = Vec::with_capacity(t_vec.len());
//
//        for i in 0..t_vec.len()
//        {
//            //res_matrix = res_matrix.set_row(&res_mat[i].clone().transpose(), i);
//            res_vec.set_row(&res_mat[i].clone().transpose(), i);
//        }

        return Ok((t_vec, res_vec));
    }
}

impl<T> RKF45<T>
    where T: Real
{

    fn calc_rkf4_rkf5<F>(t_n: &T, x_n: &Vector<T>, func: F, h: &T) -> (Vector<T>, Vector<T>)
        where F: Fn(&T, &Vector<T>) -> Vector<T>
    {
        // k_1 = hf(t_n, x_n)
        let k_1: Vector<T> = &func(t_n, x_n) * h;

        // k_2 = h f(t_n + h/4, x_n + k1/4);
        let k_2: Vector<T> = &func(&(*t_n + *h / T::from_f64(4.0).unwrap()), &(x_n + &(&k_1 / &T::from_f64(4.0).unwrap()
        ))) * h;

        //k_3 = h f(t_n + h3/8, x_n + k_1*3/32 + k2*9/32)
        let k_31: Vector<T> = x_n + &(&k_1 * &T::from_f64(3.0 / 32.0).unwrap());
        let k_32: Vector<T> = k_31 + (&k_2 * &T::from_f64(9.0 / 32.0).unwrap());
        let k_3: Vector<T> = &func(&(*t_n + *h * T::from_f64(3.0 / 8.0).unwrap()),  &k_32) * h;

        // k_4 = h f(t_n + 12/13h, x_n + 1932/2197k_1 - 7200/2197k_2 + 7296/2197k_3)
        let k_41: Vector<T> = x_n + &(&k_1 * &T::from_f64(1932.0 / 2197.0).unwrap());
        let k_42: Vector<T> = k_41 - (&k_2 * &T::from_f64(7200.0 / 2197.0).unwrap());
        let k_43: Vector<T> = k_42 + (&k_3 * &T::from_f64(7296.0 / 2197.0).unwrap());
        let k_4: Vector<T> = &func(&(*t_n + *h * T::from_f64(12.0 / 13.0).unwrap()), &k_43) * h;

        // k_5 = h f(t_n + h, x_n + 439/216k_1 - 8k_2 + 3680/513k_3 - 845/4104k_4)
        let k_51: Vector<T> = x_n + &(&k_1 * &T::from_f64(439.0 / 216.0).unwrap());
        let k_52: Vector<T> = k_51 - (&k_2 * &T::from_f64(8.0).unwrap());
        let k_53: Vector<T> = k_52 + (&k_3 * &T::from_f64(3680.0 / 513.0).unwrap());
        let k_54: Vector<T> = k_53 - (&k_4 * &T::from_f64(845.0 / 4104.0).unwrap());
        let k_5: Vector<T> = &func(&(*t_n + *h) , &k_54) * h;


        // k_6 = h f(t_n + h/2, x_n - 8/27k_1 + 2k_2 - 3544/2565k_3 + 1859/4104k_4 - 11/40k_5)
        let k_61: Vector<T> = x_n - &(&k_1 * &T::from_f64(8.0 / 27.0).unwrap());
        let k_62: Vector<T> = k_61 + (&k_2 * &T::from_f64(2.0).unwrap());
        let k_63: Vector<T> = k_62 - (&k_3 * &T::from_f64(3544.0 / 2565.0).unwrap());
        let k_64: Vector<T> = k_63 + (&k_4 * &T::from_f64(1859.0 / 4104.0).unwrap());
        let k_65: Vector<T> = k_64 - (&k_5 * &T::from_f64(11.0 / 40.0).unwrap());
        let k_6: Vector<T> = &func(&(*t_n + *h * T::from_f64(0.5).unwrap()), &k_65) * h;

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