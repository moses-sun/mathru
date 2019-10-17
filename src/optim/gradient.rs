use crate::algebra::linear::{Vector};
use crate::optim::{Jacobian};
extern crate rand;
//use self::rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Gradient
{
    /// Learningrate
    alpha: f64,
    // Momentum weight
    beta: f64,
    /// The number of iterations to run.
    iters: usize,
}


impl Gradient
{
    /// Construct an instance of gradient algorithm.
    ///
    /// # Parameters
    ///
    /// alpha: learning rate > 0.0
    /// beta:  momentum >= 0.0
    ///
    /// # Examples
    ///
    /// ```
    /// use mathru::optim::Gradient;
    ///
    /// let gd = Gradient::new(0.3, 0.8, 10000);
    /// ```
    pub fn new(alpha: f64, beta: f64, iters: usize) -> Gradient
    {
        assert!(alpha > 0.0f64, "The step size must be greater than 0.0.");
        assert!(beta <= 1.0 && beta >= 0.0f64, "The momentum must be smaller or equalt to 1.0 and greater or equal to \
        .0.0");

        Gradient
        {
            alpha: alpha,
            beta: beta,
            iters: iters,

        }
    }

//    fn shuffle(mut vec: Vec<usize>) -> Vec<usize>
//    {
//        let mut rng = rand::thread_rng();
//        let n = vec.len();
//
//        if n > 0
//        {
//            for i in 0..(n - 1)
//            {
//                let j: usize = rng.gen_range(i, n);
//                let temp = vec[i];
//                vec[i] =  vec[j];
//                vec[j] = temp;
//            }
//            return vec;
//        }
//        else
//        {
//            vec
//        }
//    }
}

impl Gradient
{
    pub fn minimize<F: Jacobian<f64>>(self: &Self, func: &F, x_0: &Vector<f64>) -> Vector<f64>
    {
        let mut x_n: Vector<f64> = x_0.clone();


        for _i in 0..self.iters
        {
            let grad: Vector<f64> = func.jacobian(&x_n).get_row(0).transpose();

            //Make step
            x_n = &x_n - &(&grad * &self.alpha);

        }
        return x_n;
    }

    pub fn maximize<F: Jacobian<f64>>(self: &Self, func: &F, x_0: &Vector<f64>) -> Vector<f64>
    {
        let mut x_n: Vector<f64> = x_0.clone();


        for _i in 0..self.iters
        {
            let grad: Vector<f64> = func.jacobian(&x_n).get_row(0).transpose();

            // Make step
            x_n = &x_n + &(&grad * &self.alpha);

        }
        return x_n;
    }
}
