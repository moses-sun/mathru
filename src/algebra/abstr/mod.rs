//! Abstract algebra
//!
//! <a href="https://en.wikipedia.org/wiki/Abstract_algebra">https://en.wikipedia.org/wiki/Abstract_algebra</a>

mod scalar;
pub mod cast;
mod field;
mod ring;
mod sign;
//mod semiring;

mod identity;
mod natural;
mod integer;
mod real;
mod complex;
mod magma;
mod monoid;
mod loop_;
mod quasigroup;
mod group;
mod semigroup;
mod operator;
mod abeliangroup;
mod lattice;

pub use self::operator::{Operator, Addition, Multiplication};
pub use self::identity::Identity;
pub use self::monoid::Monoid;
pub use self::magma::Magma;
pub use self::loop_::Loop;
pub use self::quasigroup::Quasigroup;
pub use self::semigroup::Semigroup;
pub use self::abeliangroup::AbelianGroup;
pub use self::lattice::Lattice;

//pub use self::semiring::{Semiring, Zero, One};
pub use self::ring::{Ring, CommutativeRing};
pub use self::sign::{Sign};
pub use self::field::{Field};
pub use self::scalar::{Scalar, ScalarOps, Zero, One};
pub use self::natural::Natural;
pub use self::integer::Integer;
pub use self::real::Real;
#[cfg(feature = "blaslapack")]
pub use self::real::{Blas, Lapack};
pub use self::complex::Complex;