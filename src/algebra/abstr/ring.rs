use super::operator::{Operator, Addition, Multiplication};
use super::monoid::{Monoid};
use super::abeliangroup::AbelianGroup;

/// Ring
///
/// # Definition
///
/// 1. $`R`$ is an abelian group under addition, meaning that:
/// - $`(a + b) + c = a + (b + c), \forall a, b, c \in \mathbb{R} `$  (that is, + is associative).
/// - $`a + b = b + a , \forall a, b \in \mathbb{R}`$  (that is, + is commutative).
/// - There is an element 0 in $`\mathbb{R}`$ such that $`a + 0 = a, \forall a \in \mathbb{R} `$  (that is, 0 is the additive identity).
/// - For each a in $`\mathbb{R}`$ there exists −a in $`\mathbb{R}`$ such that $`a + (−a) = 0`$  (that is, −a is the additive inverse of a).
/// 2. $`\mathbb{R}`$ is a monoid under multiplication, meaning that:
/// - $`(a * b) * c = a * (b * c), \forall a, b, c \in \mathbb{R}`$  (that is, * is associative).
/// - There is an element 1 in R such that $`a · 1 = a \wedge 1 · a = a, \forall a \in \mathbb{R}`$ (that is, 1 is the multiplicative
/// identity)
///
/// 3. Multiplication is distributive with respect to addition, meaning that:
/// - $`a * (b + c) = (a * b) + (a * c), \forall a, b, c \in \mathbb{R}`$ (left distributivity).
/// - $`(b + c) * a = (b * a) + (c * a), \forall a, b, c \in \mathbb{R}`$  (right distributivity).
///
/// <a href="https://en.wikipedia.org/wiki/Ring_(mathematics)">https://en.wikipedia.org/wiki/Ring_(mathematics)</a>
pub trait Ring<A: Operator = Addition, M: Operator = Multiplication>: AbelianGroup<A> + Monoid<M>
{

}

macro_rules! impl_ring
{
    ($($t:ty),*) =>
    {
        $(
        impl Ring for $t
        {

        }
        )*
    };
}

impl_ring!(i8, i16, i32, i64, i128, f32, f64);


pub trait CommutativeRing<A: Operator = Addition, M: Operator = Multiplication>: Ring<A, M>
{

}

macro_rules! impl_commutative_ring
{
    ($($t:ty),*) =>
    {
        $(
        impl CommutativeRing for $t
        {

        }
        )*
    };
}

impl_commutative_ring!(i8, i16, i32, i64, i128, f32, f64);