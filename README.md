# mathru

[![crate](https://img.shields.io/crates/v/mathru.svg)](https://crates.io/crates/mathru)
[![documentation](https://docs.rs/mathru/badge.svg)](https://docs.rs/mathru)
![minimum rustc 1.44.0](https://img.shields.io/badge/rustc-1.44.0-green.svg)
![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![pipeline status](https://gitlab.com/matthiaseiholzer/mathru/badges/master/pipeline.svg)](https://gitlab.com/matthiaseiholzer/mathru/-/commits/master)
------------
Mathru is a numeric library containing algorithms for linear algebra, analysis and statistics written in pure Rust with BLAS/LAPACK support.

## Features
The following features are implemented in this create:

* [Linear algebra](https://matthiaseiholzer.gitlab.io/mathru/documentation/algebra/)
    * [Vector](https://matthiaseiholzer.gitlab.io/mathru/documentation/algebra/vector/)
    * [Matrix](https://matthiaseiholzer.gitlab.io/mathru/documentation/algebra/matrix/)
        * Basic matrix operations(+,-,*)
        * Transposition
        * [LU decomposition](https://matthiaseiholzer.gitlab.io/mathru/documentation/algebra/matrix/#lu-with-partial-pivoting)
        * [QR decomposition](https://matthiaseiholzer.gitlab.io/mathru/documentation/algebra/matrix/#qr)
        * [Hessenberg decomposition](https://matthiaseiholzer.gitlab.io/mathru/documentation/algebra/matrix/#hessenberg)
        * [Cholesky decomposition](https://matthiaseiholzer.gitlab.io/mathru/documentation/algebra/matrix/#cholesky)
        * Eigen decomposition
        * Singular value decomposition
        * Inverse
        * Pseudo inverse
        * Determinant
        * Trace
        * [Solve linear system](https://matthiaseiholzer.gitlab.io/mathru/documentation/algebra/matrix/#linear-system-resolution)

* [Ordinary differential equation (ODE)](https://matthiaseiholzer.gitlab.io/mathru/documentation/analysis/differentialeq/)
    * [Explicit methods](https://matthiaseiholzer.gitlab.io/mathru/documentation/analysis/differentialeq/explicit)
        * Heun's method
        * Euler method
        * Midpoint method
        * Ralston's method
        * Kutta 3rd order
        * Runge-Kutta 4th order
        * Runge-Kutta-Felhberg 4(5)
        * Dormand-Prince 4(5)
        * Cash-Karp 4(5)
        * Tsitouras 4(5)
        * Bogacki-Shampine 2(3)
        * Adams-Bashforth
    * Automatic step size control with starting step size
    * [Implicit methods](https://matthiaseiholzer.gitlab.io/mathru/documentation/analysis/differentialeq/implicit)
        * Implicit Euler
        * Backward differentiation formula (BDF)

* [Optimization](https://matthiaseiholzer.gitlab.io/mathru/documentation/optimization)
    * Gauss-Newton algorithm
    * Gradient descent
    * Newton method
    * Levenberg-Marquardt algorithm
    * Conjugate gradient method

* [Statistics](https://matthiaseiholzer.gitlab.io/mathru/documentation/statistics)
    * probability distribution
        * Bernoulli
        * Beta
        * Binomial
        * Exponential
        * Gamma
        * Chi-squared
        * Normal
        * Poisson
        * Raised cosine
        * Student-t
        * Uniform
    * test
        * Chi-squared
        * G
        * Student-t

* elementary functions
    * trigonometric functions
    * hyperbolic functions
    * exponential functions

* special functions
    * gamma functions
    * beta functions
    * hypergeometrical functions

## Usage

Add this to your `Cargo.toml` for the native Rust implementation:

```toml
[dependencies.mathru]
version = "^0.6"
```
Add the following lines to 'Cargo.toml' if the openblas library should be used:

```toml
[dependencies.mathru]
version = "^0.6"
default-features = false
features = "openblas"
```

One of the following implementations for linear algebra can be activated as a feature:
- native: Native Rust implementation(activated by default)
- [openblas](https://www.openblas.net): Optimized BLAS library
- [netlib](https://www.netlib.org): Collection of mathematical software, papers, and databases
- [intel-mkl](https://software.intel.com/content/www/us/en/develop/tools/math-kernel-library.html): Intel Math Kernel Library
- [accelerate](https://developer.apple.com/documentation/accelerate) Make large-scale mathematical computations and image calculations, optimized for high performance and low-energy consumption.(macOS only)


Then import the modules and it is ready to be used.

```rust
use mathru as mr;
```

## Documentation

See [project page](https://matthiaseiholzer.gitlab.io/mathru) for more information and examples.
The API is documented on [docs.rs](https://docs.rs/mathru).

## License

Licensed under

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Any contribution is welcome!
