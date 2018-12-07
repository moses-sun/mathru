use super::field::Field;
use elementary::{Exponential, Trigonometry, Power, Hyperbolic};


/// Real number
///
///<a href="https://en.wikipedia.org/wiki/Real_number">https://en.wikipedia.org/wiki/Real_number</a>
pub trait Real: Field + Exponential + Trigonometry + Power + Hyperbolic
{

}