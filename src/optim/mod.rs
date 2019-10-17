mod jacobian;
mod hessian;
mod gaussnewton;
mod gradient;
//mod adagrad;

mod rosenbrock;


pub use self::gradient::Gradient;
//pub use self::stochasticgd::StochasticGradientDesc;
pub use self::jacobian::Jacobian;
pub use self::hessian::Hessian;
pub use self::gaussnewton::GaussNewton;
pub use self::rosenbrock::Rosenbrock;
//pub use self::adagrad::AdaGrad;
