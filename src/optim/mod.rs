mod jacobian;
mod hessian;
mod gaussnewton;
mod gradient;
mod newton;


pub use self::gradient::Gradient;
pub use self::jacobian::Jacobian;
pub use self::hessian::Hessian;
pub use self::gaussnewton::GaussNewton;
pub use self::newton::Newton;
