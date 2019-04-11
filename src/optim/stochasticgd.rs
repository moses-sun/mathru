use algebra::linear::{Vector, Matrix};
use optim::{OptimAlgorithm, Optimizable};

extern crate rand;
use self::rand::Rng;


const LEARNING_EPS: f64 = 1e-10;


#[derive(Clone, Copy, Debug)]
pub struct StochasticGradientDesc
{
    /// Learningrate
    alpha: f64,
    // Momentum weight
    beta: f64,
    /// The number of iterations to run.
    iters: usize,
}


impl StochasticGradientDesc
{
    /// Construct an instance of stochastic gradient descent algorithm.
    ///
    /// # Parameters
    ///
    /// alpha: learning rate > 0.0
    /// beta:  momentum >= 0.0
    ///
    /// # Examples
    ///
    /// ```
    /// use mathru::optim::StochasticGradientDesc;
    ///
    /// let gd = StochasticGradientDesc::new(0.3, 0.8, 10000);
    /// ```
    pub fn new(alpha: f64, beta: f64, iters: usize) -> StochasticGradientDesc
    {
        assert!(alpha > 0.0f64, "The step size must be greater than 0.0.");
        assert!(beta <= 1.0 && beta >= 0.0f64, "The momentum must be smaller or equalt to 1.0 and greater or equal to \
        .0.0");

        StochasticGradientDesc
        {
            alpha: alpha,
            beta: beta,
            iters: iters,

        }
    }

    fn shuffle(mut vec: Vec<usize>) -> Vec<usize>
    {
        let mut rng = rand::thread_rng();
        let n = vec.len();

        if n > 0
        {
            for i in 0..(n - 1)
            {
                let j: usize = rng.gen_range(i, n);
                let temp = vec[i];
                vec[i] =  vec[j];
                vec[j] = temp;
            }
            return vec;
        }
        else
        {
            vec
        }
    }
}

impl<M> OptimAlgorithm<M> for StochasticGradientDesc
    where M: Optimizable<Input = Matrix<f64>, Target = Vector<f64>>
{
    fn minimize(mut self: &Self, model: &M, param_start: &Vector<f64>, input: &M::Input, target: &M::Target) ->
    Vector<f64>
    {
        let (m, n) = input.dim();

        let mut param: Vector<f64> = param_start.clone();
        let (param_m, _param_n): (usize, usize) = param.dim();

        let mut cost_prev_iter: f64 = 0.0f64;

        //Sequence (0, 1, 2, ... , m-1)
        let mut permut_index: Vec<usize> = (0..m).collect::<Vec<_>>();

        let mut v_param: Vector<f64> = Vector::zero(param_m);

        for _ in 0..self.iters
        {
            permut_index = Self::shuffle(permut_index);

            let mut cost_iter: f64 = 0.0;

            for i in &permut_index
            {
                let cost: f64 = model.value(&param, &input.get_slice(*i, *i, 0, n-1), &target.get_slice(*i, *i));
                let grad: Vector<f64> = model.gradient(&param, &input.get_slice(*i, *i, 0, n-1), &target.get_slice(*i,
                *i));


                v_param = &(&v_param * &self.beta)  + &(&grad * &(1.0 - self.beta));
                // Update the parameters
                param = &param - &(&v_param * &self.alpha);

                cost_iter += cost;
            }

            // Early stopping
            if (cost_prev_iter - cost_iter).abs() < LEARNING_EPS
            {
                break;
            }
            else
            {
                // Update the latest cost
                cost_prev_iter = cost_iter;
            }
        }
        param
    }
}
