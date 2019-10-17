use crate::algebra::abstr::{Real};
use std::iter::Iterator;
use crate::algebra::linear::matrix::MatrixIntoIterator;

pub struct VectorIntoIterator<T>
{
    pub iter: MatrixIntoIterator<T>
}

impl<T> Iterator for VectorIntoIterator<T>
    where T: Real
{
    type Item = T;

    fn next(self: &mut Self) -> Option<Self::Item>
    {
        return self.iter.next();
    }
}