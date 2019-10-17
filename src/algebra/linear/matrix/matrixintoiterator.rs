use crate::algebra::abstr::{Real};
use std::iter::Iterator;
use std::vec::IntoIter;

pub struct MatrixIntoIterator<T>
{
    pub iter: IntoIter<T>
}

impl<T> Iterator for MatrixIntoIterator<T>
    where T: Real
{
    type Item = T;

    fn next(self: &mut Self) -> Option<Self::Item>
    {
        return Some(T::zero());
    }
}