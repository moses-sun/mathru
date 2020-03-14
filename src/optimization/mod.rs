//! Optimization
//!
//! This module provides functions for minimizing objective functions. It includes solvers
//! for nonlinear problems, nonlinear least-squares.
mod gaussnewton;
mod gradient;
mod newton;
mod levenbergmarquardt;
mod conjugategradient;
mod optimresult;
mod nonlinearcg;

/// Gradient mehtod
pub use self::gradient::Gradient;
/// Gauss-Newton method
pub use self::gaussnewton::GaussNewton;
/// Newton's method
pub use self::newton::Newton;
/// Levenberg Marquardt method
pub use self::levenbergmarquardt::LevenbergMarquardt;
/// Conjugate Gradient method
pub use self::conjugategradient::ConjugateGradient;

//pub use self::nonlinearcg::NonlinearConjugateGradient;
pub use self::optimresult::OptimResult;
