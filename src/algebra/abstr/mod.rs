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
mod abs_diff_eq;
mod relative_eq;

pub use self::{
    abeliangroup::{AbelianGroup, AbelianGroupAdd, AbelianGroupMul},
    group::{Group, GroupAdd, GroupMul},
    identity::Identity,
    lattice::Lattice,
    loop_::Loop,
    magma::{Magma, MagmaAdd, MagmaMul},
    monoid::{Monoid, MonoidAdd, MonoidMul, One, Zero},
    operator::{Addition, Multiplication, Operator},
    quasigroup::Quasigroup,
    semigroup::{Semigroup, SemigroupAdd, SemigroupMul},
    abs_diff_eq::{AbsDiffEq, AbsDiff},
    relative_eq::{RelativeEq, Relative},
};

#[cfg(feature = "blaslapack")]
pub use self::scalar::{Blas, Lapack};
pub use self::{
    complex::Complex,
    field::Field,
    integer::Integer,
    natural::Natural,
    real::Real,
    ring::{CommutativeRing, Ring},
    scalar::Scalar,
    sign::Sign,
};
