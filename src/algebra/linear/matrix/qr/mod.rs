#[cfg(feature = "native")]
pub mod native;
#[cfg(feature = "blaslapack")]
pub mod lapack;

pub mod qrdec;
pub use self::qrdec::QRDec;

