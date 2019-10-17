use std::slice::Iter;
use crate::algebra::abstr::Real;

pub struct MatrixIterator<'a, T>
{
    pub iter: Iter<'a, T>,
}

impl<'a, T> Iterator for MatrixIterator<'a, T>
    where T: Real
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        return self.iter.next();
    }
}