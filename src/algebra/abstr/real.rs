use super::field::Field;
use elementary::{Exponential, Trigonometry, Power, Hyperbolic};

pub trait Real: Field + Exponential + Trigonometry + Power + Hyperbolic
{

}