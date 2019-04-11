//! # mathru
//!
//! A crate that provides  mathematics functions implemented entirely in Rust.
//!
//!
//! ## Usage
//!
//! The library usage is described well in the API documentation - including example code.
//!
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! mathru = "0.1.0"
//! ```
//!
//! And then import the library using:
//! ```rust
//! extern crate mathru;
//! ```
//!
//! Then import the modules and it is ready to be used:
//!
//!``` rust
//! extern crate mathru;
//! use mathru::algebra::linear::{Matrix};
//! // Compute the LU decomposition of a 2x2 matrix
//! let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, -2.0, 3.0, -7.0]);
//! let l_ref: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 1.0 / 3.0, 1.0]);
//!
//! let (l, u, p): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu();
//!
//! assert_eq!(l_ref, l);
//!```

#[cfg(feature = "blaslapack")]
extern crate blas;
#[cfg(feature = "blaslapack")]
extern crate blas_src;
#[cfg(feature = "blaslapack")]
extern crate lapack;

#[macro_use]
extern crate serde;

#[macro_use]
pub mod algebra;
pub mod elementary;
#[macro_use]
pub mod num;
pub mod special;
pub mod stats;
//pub mod optim;



