# mathru

[![crate](https://img.shields.io/crates/v/mathru.svg)](https://crates.io/crates/mathru)
[![documentation](https://docs.rs/mathru/badge.svg)](https://docs.rs/mathru)
![minimum rustc 1.38.0](https://img.shields.io/badge/rustc-1.38.0-green.svg)
![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
------------
mathru is a numeric library containing algorithms for linear algebra, analysis and statistics written in pure Rust with BLAS/LAPACK support.


## Features
    - Linear algebra
        - Vector
        - Matrix
            - Basic matrix operations(+,-,*)
            - Transposition
            - LU decomposition (native/lapack)
            - QR decomposition (native/lapack)
            - Hessenberg decomposition (native/lapack)
            - Singular value decomposition
            - Inverse (native/lapack)
            - Pseudo inverse (native/lapack)
            - Determinant (native/lapack)
            - Trace
            - Eigenvalue (native/lapack)

    - Ordinary differential equation (ODE)
        - Heun's method
        - Euler method
        - Midpoint method
        - Ralston's method
        - Runge-Kutta 4th order
        - Runge-Kutta-Felhberg 4(5)
        - Dormand-Prince 4(5)
        
    - Optimization
        - Gauss-Newton algorithm
        - Gradient descent
        - Newton method
        - Levenberg-Marquardt algorithm
        - Conjugate gradient method

    - Statistics
        - probability distribution
            - Bernoulli
            - Beta
            - Binomial
            - Chisquared
            - Exponential
            - Gamma
            - Chi-squared
            - Multinomial
            - Normal
            - Poisson
            - Raised cosine
            - Student-t
            - Uniform
        - test
            - Chi-squared 
            - G
            - Student-t

    - elementary functions
        - trigonometric functions
        - hyperbolic functions
        - exponential functions

    - special functions
        - gamma functions
        - beta functions
        - hypergeometrical functions
## Usage

Add this to your `Cargo.toml` for the native Rust implementation:

```toml
[dependencies.mathru]
version = "0.5"
```
Add the following lines to 'Cargo.toml' if the blas/lapack backend should be used:

```toml
[dependencies.mathru]
version = "0.5"
default-features = false
features = ["blaslapack"]
```

Then import the modules and it is ready to be used:

``` rust
use mathru::algebra::linear::{Matrix};

// Compute the LU decomposition of a 2x2 matrix
let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, -2.0, 3.0, -7.0]);
let l_ref: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 1.0 / 3.0, 1.0]);

let (l, u, p): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu();

assert_eq!(l_ref, l);
```

Solve an ODE:

```
//
// explicit ODE
// x' = 1 + x^2
fn f(_t: &f64, x: &Vector<f64>) -> Vector<f64>
{
	let result  = vector![1.0] + x.clone().apply(&|e: &f64| -> f64 {return e * e;}) ;

	return result;
}

let h_0: f64 = 0.0001;
let e_max: f64 = 0.000001;
let n_max: u32 = 500;

let init: Vector<f64> = vector![0.0];
let solver: Dopri5<f64> = Dopri5::new(h_0, e_max, n_max);

let (t, y): (Vector<f64>, Matrix<f64>) = solver.solve(f, init, 0.0, 1.4);

let (m, _n): (usize, usize) = y.dim();

assert!(compare_real(&1.40, &t.get(&(m-1)), 0.0001));
assert!(compare_real(&1.4_f64.tan(), &y.get(&(m-1), &0), 0.0001));

```
## Contributions

Any contribution is welcome!
