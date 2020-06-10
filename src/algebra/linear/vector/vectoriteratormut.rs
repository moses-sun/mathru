use crate::algebra::{
    abstr::{Field, Scalar},
    linear::matrix::MatrixIteratorMut,
};

pub struct VectorIteratorMut<'a, T>
{
    pub iter: MatrixIteratorMut<'a, T>,
}

impl<'a, T> Iterator for VectorIteratorMut<'a, T> where T: Field + Scalar
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>
    {
        return Some(T::zero());
    }
}
