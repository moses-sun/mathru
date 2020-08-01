#[cfg(feature = "blaslapack")]
pub mod lapack;
#[cfg(feature = "native")]
pub mod native;

pub trait Transpose
{
    type Output;
    fn transpose(self: Self) -> Self::Output;
}

