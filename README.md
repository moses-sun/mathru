# mathru

[![crate](https://img.shields.io/crates/v/mathru.svg)](https://crates.io/crates/mathru)
[![documentation](https://docs.rs/mathru/badge.svg)](https://docs.rs/mathru)
![minimum rustc 1.32.0](https://img.shields.io/badge/rustc-1.32.0-green.svg)

------------
A simple mathematics library written in Rust and supports BLAS/LAPACK.

## Implementation

This project is implemented entirely in Rust.

## Features
    - linear algebra
        - Vector
        - Matrix
            - Basic matrix operations(+,-,*)
            - Transposition
            - LU decomposition (native/lapack)
            - QR decomposition (native/lapack)
            - Hessenberg decomposition (native/lapack)
            - Singular value decomposition
            - Inverse matrix (native/lapack)
            - Determinant (native/lapack)
            - Trace
            - Eigenvalue (native/lapack)

    - complex numbers
        - Complex<f32>
        - Complex<f64>

    - special functions
        - gamma functions
        - beta functions
    - statistics
        - distributions
            - normal distribution
            - gamma distribution
            - binomial distribution
            - poisson distribution
            - exponential distribution
            - chi squared distribution
            - beta distribution
            - bernoulli distribution

    - elementary functions
        - trigonometric functions
        - hyperbolic functions
        - exponential

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mathru = "0.1.2"
```

And then import the library using:
```rust
extern crate mathru;
```

Then import the modules and it is ready to be used:

``` rust
extern crate mathru;
use mathru::algebra::linear::{Matrix};

// Compute the LU decomposition of a 2x2 matrix
let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, -2.0, 3.0, -7.0]);
let l_ref: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 1.0 / 3.0, 1.0]);

let (l, u, p): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu();

assert_eq!(l_ref, l);
```


## Contributions

Any contribution is welcome!
