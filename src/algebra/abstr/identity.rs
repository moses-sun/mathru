//use core::num::Wrapping;
use std::ops::{Add, Mul};

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
    ///
    /// # Purity
    ///
    /// This function should return the same result at all times regardless of
    /// external mutable state, for example values stored in TLS or in
    /// `static mut`s.
    // This cannot be an associated constant, because of bignums.
    fn zero() -> Self;
}


//impl<T: Zero> Zero for Wrapping<T>
//where
//    Wrapping<T>: Add<Output = Wrapping<T>>,
//{
//    fn is_zero(&self) -> bool {
//        self.0.is_zero()
//    }
//    fn zero() -> Self {
//        Wrapping(T::zero())
//    }
//}

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
    ///
    /// # Purity
    ///
    /// This function should return the same result at all times regardless of
    /// external mutable state, for example values stored in TLS or in
    /// `static mut`s.
    // This cannot be an associated constant, because of bignums.
    fn one() -> Self;
}