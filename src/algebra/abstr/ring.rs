use algebra::abstr::Semiring;
use std::ops::Neg;

//pub trait Abs
//{
//    fn abs<'a>(self: &'a Self) -> Self;
//}
//
//pub trait Sign
//{
//    fn sign<'a>(self: &'a Self) -> Self;
//}


pub trait Ring: Semiring + Neg<Output = Self>
{

}

//pub trait Sqrt
//{
//    fn sqrt<'a>(self: &'a Self) -> Self;
//}






