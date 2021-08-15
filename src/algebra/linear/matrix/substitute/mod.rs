#[cfg(feature = "lapack")]
pub mod lapack;
#[cfg(feature = "native")]
pub mod native;

pub trait Substitute<T>
{
    fn substitute_forward(self: &Self, b: T) -> Result<T, ()>;

    fn substitute_backward(self: &Self, b: T) -> Result<T, ()>;
}
