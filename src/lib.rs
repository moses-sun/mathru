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
//! mathru = "0.3"
//! ```
//!
//!
//! Then import the modules and it is ready to be used:
//!
//!``` rust
//! # #[macro_use]
//! # extern crate mathru;
//! # fn main()
//! # {
//! use mathru::algebra::linear::{Vector, Matrix};
//! // Compute the LU decomposition of a 2x2 matrix
//! let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 2.0, -3.0, -7.0]);
//! let b: Vector<f64> = vector![1.0; 3.0];
//!
//! let (l, u, p): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu();
//!
//! let b_hat = &p * &b;
//!
//! let y = u.subst_backward_vector(b_hat);
//!
//! let x = p * l.subst_forward_vector(y);
//!
//! println!("{}", x);
//! # }
//!```

#[cfg(feature = "blaslapack")]
extern crate blas;
#[cfg(feature = "blaslapack")]
extern crate blas_src;
#[cfg(feature = "blaslapack")]
extern crate lapack;

//extern crate serde;

#[macro_use]
pub mod algebra;
pub mod elementary;
pub mod num;
pub mod special;
pub mod stats;
pub mod analysis;
pub mod optim;



