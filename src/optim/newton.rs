//use self::mathru::algebra::linear::{Vector};
//use self::mathru::numeric::{Natural, Real, Bound};
//use self::mathru::algebra::abstr::{Abs, Zero};
//use super::gradient::Lossfunction;
//
//pub struct Newton
//{
//    alpha: Real<f64>,
//    iters: Natural<u64>
//}
//
//
//impl Newton
//{
//    pub fn new<'a>(alpha: &'a Real<f64>, iters: &'a Natural<u64>) -> Newton
//    {
//        Newton
//        {
//            alpha : *alpha,
//            iters : *iters
//        }
//    }
//
//    pub fn minimize(self: &Self, model: &Lossfunction, beta_0: &Vector<Real<f64>>) -> Vector<Real<f64>>
//    {
//        let LEARNING_EPSILON: Real<f64> = Real::new(1.0e-20);
//        let mut cost_prev: Real<f64> = Real::upper_bound();
//
//        //initial parameter values
//		let mut beta_j: Vector<Real<f64>> = beta_0.clone();
//
//        for j in Interval::range(Natural::zero(), self.iters)
//        {
//            let cost : Real<f64> = model.value(&beta_j);
//            let gradient: Vector<Real<f64>> = model.gradient(&beta_j);
//
//            if (cost_prev - cost).abs() < LEARNING_EPSILON
//            {
//                break;
//            }
//            else
//            {
//                beta_j = beta_j - gradient.mul_scalar(&self.alpha);
//            }
//        }
//
//        return beta_j;
//    }
//
//    pub fn maximize(self: &Self, model: &Lossfunction, beta_0: &Vector<Real<f64>>) -> Vector<Real<f64>>
//    {
//        let LEARNING_EPSILON: Real<f64> = Real::new(1.0e-20);
//        let mut cost_prev: Real<f64> = Real::upper_bound();
//
//        //initial parameter values
//		let mut beta_j: Vector<Real<f64>> = beta_0.clone();
//
//        for j in Interval::range(Natural::zero(), self.iters)
//        {
//            let cost : Real<f64> = model.value(&beta_j);
//            let gradient: Vector<Real<f64>> = model.gradient(&beta_j);
//
//            if (cost_prev - cost).abs() < LEARNING_EPSILON
//            {
//                break;
//            }
//            else
//            {
//                beta_j = beta_j - gradient.mul_scalar(&self.alpha);
//            }
//        }
//
//        return beta_j;
//    }
//}