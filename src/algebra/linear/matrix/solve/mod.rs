#[cfg(feature = "blaslapack")]
pub mod lapack;
#[cfg(feature = "native")]
pub mod native;

pub trait Solve<T>
{
    /// A * x = b
    fn solve(self: &Self, rhs: &T) -> Result<T, ()>;
}
// #[cfg(feature = "blaslapack")]
// pub use self::lapack::Solve;
// #[cfg(feature = "native")]
// pub use self::native::Solve;
