//! Abstract algebra
//!
//! <a href="https://en.wikipedia.org/wiki/Abstract_algebra">https://en.wikipedia.org/wiki/Abstract_algebra</a>

mod scalar;
pub mod cast;
mod field;
mod ring;
mod semiring;


mod natural;
mod integer;
mod real;
mod complex;

pub use self::semiring::{Semiring, Zero, One};
pub use self::ring::{Ring};
pub use self::field::{Field, Sign, Abs};
pub use self::scalar::Scalar;
pub use self::natural::Natural;
pub use self::integer::Integer;
pub use self::real::Real;
#[cfg(feature = "blaslapack")]
pub use self::real::{Blas, Lapack};
pub use self::complex::Complex;