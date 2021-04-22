#[cfg(feature = "lapack")]
pub mod lapack;
#[cfg(feature = "native")]
pub mod native;

pub trait Solve<T>
{
    /// A * x = b
    fn solve(self: &Self, rhs: &T) -> Result<T, ()>;
}
// #[cfg(feature = "lapack")]
// pub use self::lapack::Solve;
// #[cfg(feature = "native")]
// pub use self::native::Solve;
