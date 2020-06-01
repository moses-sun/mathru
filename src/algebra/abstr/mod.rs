//! Abstract algebra
//!
//! <a href="https://en.wikipedia.org/wiki/Abstract_algebra">https://en.wikipedia.org/wiki/Abstract_algebra</a>

pub mod cast;
mod field;
mod ring;
mod scalar;
mod sign;
//mod semiring;

mod abeliangroup;
mod complex;
mod group;
mod identity;
mod integer;
mod lattice;
mod loop_;
mod magma;
mod monoid;
mod natural;
mod operator;
mod quasigroup;
mod real;
mod semigroup;

pub use self::abeliangroup::{AbelianGroup, AbelianGroupAdd, AbelianGroupMul};
pub use self::group::{Group, GroupAdd, GroupMul};
pub use self::identity::Identity;
pub use self::lattice::Lattice;
pub use self::loop_::Loop;
pub use self::magma::{Magma, MagmaAdd, MagmaMul};
pub use self::monoid::{Monoid, MonoidAdd, MonoidMul, One, Zero};
pub use self::operator::{Addition, Multiplication, Operator};
pub use self::quasigroup::Quasigroup;
pub use self::semigroup::{Semigroup, SemigroupAdd, SemigroupMul};

//pub use self::semiring::{Semiring, Zero, One};
pub use self::complex::Complex;
pub use self::field::Field;
pub use self::integer::Integer;
pub use self::natural::Natural;
pub use self::real::Real;
pub use self::ring::{CommutativeRing, Ring};
pub use self::scalar::Scalar;
#[cfg(feature = "blaslapack")]
pub use self::scalar::{Blas, Lapack};
pub use self::sign::Sign;
