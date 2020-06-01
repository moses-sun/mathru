//#![feature(test)]
#![allow(unused_macros)]
#[macro_use]
extern crate mathru;

#[macro_use]
extern crate criterion;

mod algebra;
mod analysis;

criterion_main!(algebra::linear::matrix::matrix, analysis::ode::ode,);
