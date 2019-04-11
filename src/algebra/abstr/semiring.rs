use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::marker::Sized;
use crate::algebra::abstr::Number;

/// Semiring
///
///<a href="https://en.wikipedia.org/wiki/Semiring">https://en.wikipedia.org/wiki/Semiring</a>
pub trait Semiring: Number + Zero + Add<Self, Output = Self> + One + Mul<Self, Output = Self> + AddAssign<Self>
+ MulAssign<Self>
{

}

/// Defines an additive identity element for `Self`.
pub trait Zero: Sized + Add<Self, Output = Self>
{
    /// Returns the additive identity element of `Self`, `0`.
    ///
    /// # Laws
    ///
    /// ```{.text}
    /// a + 0 = a       ∀ a ∈ Self
    /// 0 + a = a       ∀ a ∈ Self
    /// ```
    fn zero() -> Self;
}

/// Defines a multiplicative identity element for `Self`.
pub trait One: Sized + Mul<Self, Output = Self> {
    /// Returns the multiplicative identity element of `Self`, `1`.
    ///
    /// # Laws
    ///
    /// ```{.text}
    /// a * 1 = a       ∀ a ∈ Self
    /// 1 * a = a       ∀ a ∈ Self
    /// ```
    fn one() -> Self;
}
