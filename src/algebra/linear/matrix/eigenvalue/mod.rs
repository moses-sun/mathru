#[cfg(feature = "native")]
pub mod native;
#[cfg(feature = "blaslapack")]
pub mod lapack;

#[cfg(feature = "native")]
pub use self::native as eigenvalue;
#[cfg(feature = "blaslapack")]
pub use self::lapack as eigenvalue;