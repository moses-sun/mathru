#[cfg(feature = "blaslapack")]
pub mod lapack;
#[cfg(feature = "native")]
pub mod native;

pub mod eigendec;
pub use self::eigendec::EigenDec;
