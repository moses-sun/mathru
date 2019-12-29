#[cfg(feature = "native")]
pub mod native;
#[cfg(feature = "blaslapack")]
pub mod lapack;

#[cfg(feature = "native")]
pub use self::native::LUDec;
#[cfg(feature = "blaslapack")]
pub use self::lapack::LUDec;