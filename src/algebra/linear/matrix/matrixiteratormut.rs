use std::slice::IterMut;
use crate::algebra::abstr::Real;

pub struct MatrixIteratorMut<'a, T>
{
    pub iter: IterMut<'a, T>
}

impl<'a, T> Iterator for MatrixIteratorMut<'a, T>
    where T: Real
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item>
    {
        return self.iter.next();
    }
}