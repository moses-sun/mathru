//! Algorithms for minimizing objective functions.

mod conjugategradient;
mod gaussnewton;
mod gradient;
mod levenbergmarquardt;
mod newton;
mod optim;
mod optimresult;

pub use self::optim::Optim;

/// Conjugate Gradient method
pub use self::conjugategradient::ConjugateGradient;
/// Gauss-Newton method
pub use self::gaussnewton::GaussNewton;
/// Gradient mehtod
pub use self::gradient::Gradient;
/// Levenberg Marquardt method
pub use self::levenbergmarquardt::LevenbergMarquardt;
/// Newton's method
pub use self::newton::Newton;

//pub use self::nonlinearcg::NonlinearConjugateGradient;
pub use self::optimresult::OptimResult;
