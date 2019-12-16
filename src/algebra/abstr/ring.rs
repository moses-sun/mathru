use crate::algebra::abstr::Semiring;
use std::ops::Neg;

/// Ring
///
///<a href="https://en.wikipedia.org/wiki/Ring_(mathematics)">https://en.wikipedia.org/wiki/Ring_(mathematics)</a>
pub trait Ring: Semiring + Neg<Output = Self>
{

}









