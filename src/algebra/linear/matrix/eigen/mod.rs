#[cfg(feature = "native")]
pub mod native;
#[cfg(feature = "blaslapack")]
pub mod lapack;

pub mod eigendec;
pub use self::eigendec::EigenDec;

