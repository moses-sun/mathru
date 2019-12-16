//! Monoid
use super::operator::{Operator, Addition, Multiplication};
use super::semigroup::Semigroup;
use super::identity::Identity;

/// A Monoid is a triple $`(\mathbb{M}, \circ, e)`$, composed by a set $`\mathbb{M}`$ and a binary inner operation $`\circ`$
/// and the element $`e \in \mathbb{M}`$
/// # Definition
///
/// ```math
/// \circ: \mathbb{M} \times \mathbb{M} \rightarrow \mathbb{M} , (x, y) \mapsto x \circ y
/// ```
/// 1. associativity <br>
/// $`\forall x, y, z \in \mathbb{M}`$: $`x \circ (y \circ z) = (x \circ y) \circ z`$
/// 2. $`e`$ neutral element <br>
/// $`\forall x \in \mathbb{M}`$: $`x \circ e = e \circ x = x`$
pub trait Monoid<O: Operator>: Semigroup<O> + Identity<O>
{
	fn id() ->  Self;
}


impl<T> Monoid<Addition> for T
    where T: Semigroup<Addition> + Identity<Addition>
{
	fn id() -> Self
	{
		return Identity::<Addition>::id();
	}
}

impl<T> Monoid<Multiplication> for T
    where T: Semigroup<Multiplication> + Identity<Multiplication>
{
	fn id() -> Self
	{
		return Identity::<Multiplication>::id();
	}
}

