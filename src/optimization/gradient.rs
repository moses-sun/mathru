use crate::algebra::linear::{Vector};
use crate::optimization::{OptimResult, Jacobian};
extern crate rand;


/// Gradient method
///
/// It is assumed that $`f \colon D \in \mathbb{R}^n \to \mathbb{R}`$
/// The idea is, that in every iteration a step is made in direction of
/// the anti gradient.
///
/// ```math
/// x_{k + 1} := x_{k} - \alpha_{k} \nabla f(x_{k})
/// ```
/// in order that $` f(x_{k + 1}) < f(x_{k}) `$.
///
/// input: Function $` f: \mathbb{R}^n \to \mathbb{R} `$, and initial approximation $` x_{0} \in \mathbb{R}^{n} `$
///
/// output: $` x_{k} `$
///
/// 1. Initialization: choose $`\sigma \in (0, 1) `$
///
///     set  $`k := 0 `$
/// 2. calculate antigradient $`d_{k} := -\nabla f(x_{k}) `$
///
///     set $` \alpha_{k} := 1 `$
/// 3. while $`f(x_{k} + \alpha_{k} d_{k}) > f(x_k) - \sigma \alpha_{k} \lvert \lvert d_{k} \rvert \rvert_{2}^{2} `$
///
///     set  $` \alpha_{k} := \alpha_{k} /2 `$
/// 4. $` x_{k + 1} := x_{k} + \alpha_{k} d_{k} `$
/// 5. $` k := k + 1 `$ go to 2.
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Gradient
{
    /// Learningrate
    sigma: f64,
    /// The number of iterations to run.
    iters: usize,
}


impl Gradient
{
    /// Construct an instance of gradient algorithm.
    ///
    /// # Parameters
    ///
    /// sigma: learning rate > 0.0
    ///
    /// # Examples
    ///
    /// ```
    /// use mathru::optimization::Gradient;
    ///
    /// let gd = Gradient::new(0.3, 10000);
    /// ```
    pub fn new(sigma: f64, iters: usize) -> Gradient
    {
        assert!(sigma <= 1.0 && sigma > 0.0f64);
        assert!(iters > 0);
        Gradient
        {
            sigma: sigma,
            iters: iters,

        }
    }

}

impl Gradient
{
    pub fn minimize<F: Jacobian<f64>>(self: &Self, func: &F, x_0: &Vector<f64>) -> OptimResult<Vector<f64>>
    {
        let mut x_n: Vector<f64> = x_0.clone();


        for _i in 0..self.iters
        {
            let mut alpha_k: f64 = 1.0;
            let anti_grad: Vector<f64> = -func.jacobian(&x_n).get_row(0).transpose();

            //Backtracking line search
            //Armijo–Goldstein condition
            let mut anti_grad_alpha: Vector<f64> = &anti_grad * &alpha_k;
            let mut r: Vector<f64> = &x_n + &anti_grad_alpha;
            let mut k: f64 = self.sigma * anti_grad_alpha.dotp(&anti_grad.clone());
            let mut f_r: f64 = *func.eval(&r).get(0);
            let f_x_n: f64 = *func.eval(&x_n).get(0);

            while f_r > f_x_n - k
            {
                alpha_k = alpha_k / 2.0;
                anti_grad_alpha  = &anti_grad * &alpha_k;
                r = &x_n + &anti_grad_alpha;
                k = self.sigma * anti_grad_alpha.dotp(&anti_grad.clone());
                f_r = *func.eval(&r).get(0);
            }
            //Make step
            x_n = x_n + anti_grad * alpha_k;

        }
        return OptimResult::new(x_n);
    }
}
