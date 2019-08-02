//#![feature(test)]
#![allow(unused_macros)]

extern crate mathru;

#[macro_use]
extern crate criterion;

pub mod matrix;

criterion_main!(
    matrix::matrix,
);

