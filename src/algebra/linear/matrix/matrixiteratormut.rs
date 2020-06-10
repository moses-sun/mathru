use crate::algebra::abstr::{Field, Scalar};
use std::slice::IterMut;

pub struct MatrixIteratorMut<'a, T>
{
    pub iter: IterMut<'a, T>,
}

impl<'a, T> Iterator for MatrixIteratorMut<'a, T> where T: Field + Scalar
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item>
    {
        return self.iter.next();
    }
}
