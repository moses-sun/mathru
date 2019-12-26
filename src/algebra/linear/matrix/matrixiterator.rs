use std::slice::Iter;
use crate::algebra::abstr::{Field, Scalar};

pub struct MatrixIterator<'a, T>
{
    pub iter: Iter<'a, T>,
}

impl<'a, T> Iterator for MatrixIterator<'a, T>
    where T: Field + Scalar
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        return self.iter.next();
    }
}