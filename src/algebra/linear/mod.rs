//! Linear algebra
//!

pub use self::vector::Vector;
#[macro_use]
pub use self::matrix::Matrix;

pub mod vector;
#[macro_use]
pub mod matrix;

pub mod backend;
pub mod allocator;

