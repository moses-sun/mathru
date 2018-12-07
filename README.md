# Rumath

[![crate](https://img.shields.io/crates/v/mathru.svg)](https://crates.io/crates/mathru)
[![documentation](https://docs.rs/mathru/badge.svg)](https://docs.rs/mathru)
![minimum rustc 1.30.1](https://img.shields.io/badge/rustc-1.30.1-green.svg)

------------
A simple mathematics library written in Rust

## Features
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
        - trigonometric function
            - sin()     - arcsin()
            - cos()     - arccos()
            - tan()     - arctan()
            - cot()     - arccot()
            - sec()     - arcsec()
            - csc()     - arccsc()

        - hyperbolic functions
            - sinh()    - arsinh()
            - cosh()    - arcosh()
            - tanh()    - artanh()
            - coth()    - arcoth()
            - sech()    - arsech()
            - csch()    - arcsch()

        - exponential
            - exp()     - ln()

        implemented for f32, f64, Complex<f32>, Complex<f64>

## Usage
-----

Add this to your `Cargo.toml`:

```toml
[dependencies]
mathru = "0.0.4"
```

and this to your crate root:

```rust
extern crate mathru;
```

## Contributions
-------------

Any contribution is welcome!
