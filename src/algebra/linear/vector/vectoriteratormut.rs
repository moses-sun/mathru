use crate::algebra::linear::matrix::MatrixIteratorMut;
use crate::algebra::abstr::Real;

pub struct VectorIteratorMut<'a, T>
{
    pub iter: MatrixIteratorMut<'a, T>
}

impl<'a, T> Iterator for VectorIteratorMut<'a, T>
    where T: Real
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>
    {
        return Some(T::zero());
    }
}