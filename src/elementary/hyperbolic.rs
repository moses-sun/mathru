/// Hyperbolic functions
///
///<a href="https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions">https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions</a>
pub trait Hyperbolic
{
    /// Hyperbolic sine
    fn sinh(self: Self) -> Self;

    /// Hyperbolic cosine
    fn cosh(self: Self) -> Self;

    /// Hyperbolic tangens
    fn tanh(self: Self) -> Self;

    /// Hyperbolic cotangens
    fn coth(self: Self) -> Self;

    /// Hyperbolic secant
    fn sech(self: Self) -> Self;

    /// Hyperbolic cosecant
    fn csch(self: Self) -> Self;

    /// Inverse hyperbolic  sine
    fn arsinh(self: Self) -> Self;

    /// Inverse hyperbolic cosine
    fn arcosh(self: Self) -> Self;

    /// Inverse hyperbolic tangens
    fn artanh(self: Self) -> Self;

    /// Inverse hyperbolic cosecant
    fn arcoth(self: Self) -> Self;

    /// Inverse hyperbolic secant
    fn arsech(self: Self) -> Self;

    /// Inverse hyperbolic cosecant
    fn arcsch(self: Self) -> Self;
}
