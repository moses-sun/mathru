//pub struct AdaGrad {
//    alpha: f64,
//    tau: f64,
//    iters: usize,
//}
//
//impl AdaGrad
//{
//    /// Constructs an instance of AdaGrad
//    ///
//    /// #
//    ///
//    /// # Examples
//    ///
//    /// ```
//    /// use mathru::optim::AdaGrad;
//    ///
//    /// let gd = AdaGrad::new(0.5, 1.0, 100);
//    /// ```
//    pub fn new(alpha: f64, tau: f64, iters: usize) -> AdaGrad
//    {
//        assert!(alpha > 0.0, "The step size must be greater than 0.0");
//        assert!(tau >= 0.0, "The adaptive constant (tau) cannot be negative.");
//        AdaGrad
//        {
//            alpha: alpha,
//            tau: tau,
//            iters: iters,
//        }
//    }
//}
//
//
//impl<M: Optimizable<Inputs = Matrix<f64>, Targets = Matrix<f64>>> OptimAlgorithm<M> for AdaGrad
//{
//    fn optimize(&self,
//                model: &M,
//                start: &[f64],
//                inputs: &M::Inputs,
//                targets: &M::Targets)
//                -> Vec<f64> {
//
//        // Initialize the adaptive scaling
//        let mut ada_s = Vector::zeros(start.len());
//
//        // Initialize the optimal parameters
//        let mut optimizing_val = Vector::new(start.to_vec());
//
//        // Set up the indices for permutation
//        let mut permutation = (0..inputs.rows()).collect::<Vec<_>>();
//        // The cost at the start of each iteration
//        let mut iter_cost: f64 = 0f64;
//
//        for _ in 0..self.iters
//        {
//            // The cost at the end of each stochastic gd pass
//            let mut end_cost = 0f64;
//            // Permute the indices
//            rand_utils::in_place_fisher_yates(&mut permutation);
//            for i in &permutation {
//                // Compute the cost and gradient for this data pair
//                let (cost, mut vec_data) = model.compute_grad(optimizing_val.data(),
//                                                              &inputs.select_rows(&[*i]),
//                                                              &targets.select_rows(&[*i]));
//                // Update the adaptive scaling by adding the gradient squared
//                utils::in_place_vec_bin_op(ada_s.mut_data(), &vec_data, |x, &y| *x += y * y);
//
//                // Compute the change in gradient
//                utils::in_place_vec_bin_op(&mut vec_data, ada_s.data(), |x, &y| {
//                    *x = self.alpha * (*x / (self.tau + (y).sqrt()))
//                });
//                // Update the parameters
//                optimizing_val = &optimizing_val - Vector::new(vec_data);
//                // Set the end cost (this is only used after the last iteration)
//                end_cost += cost;
//            }
//            end_cost /= inputs.rows() as f64;
//
//            // Early stopping
//            if (iter_cost - end_cost).abs() < LEARNING_EPS {
//                break;
//            } else {
//                // Update the cost
//                iter_cost = end_cost;
//            }
//        }
//        optimizing_val.into_vec()
//    }
//}