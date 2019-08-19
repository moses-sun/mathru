#[macro_use]
pub mod matrix;
mod eigenvalue;
mod hessenberg;
mod lu;
mod qr;
mod inverse;
mod mul;
mod add;
mod sub;
mod div;
mod cholesky;

pub use self::matrix::Matrix;