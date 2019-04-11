
pub mod gradientdesc;
mod optimizable;
mod optimalgorithm;
mod newton;
mod batchgd;
mod stochasticgd;
//mod adagrad;


pub use self::gradientdesc::{GradientDesc};
pub use self::batchgd::BatchGradientDesc;
pub use self::stochasticgd::StochasticGradientDesc;
pub use self::optimizable::Optimizable;
pub use self::optimalgorithm::OptimAlgorithm;
//pub use self::adagrad::AdaGrad;
