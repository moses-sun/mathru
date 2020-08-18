/// Exponential function and its inverse
///
///<a href="https://en.wikipedia.org/wiki/Exponential_function">https://en.wikipedia.org/wiki/Exponential_function</a>
pub trait Exponential
{
    /// Euler's number
    fn e() -> Self;

    ///Exponential function
    fn exp(self: Self) -> Self;

    /// Natural logiarithm function
    fn ln(self: Self) -> Self;
}
