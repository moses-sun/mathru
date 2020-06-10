use crate::algebra::abstr::{Field, Scalar};
use std::slice::Iter;

pub struct MatrixRowIterator<'a, T>
{
    //_phantom: PhantomData<T>
    pub iter: Iter<'a, T>,
}

impl<'a, T> Iterator for MatrixRowIterator<'a, T> where T: Field + Scalar
{
    type Item = T;

    // just return the str reference
    fn next(&mut self) -> Option<Self::Item>
    {
        //self.iter.next()
        Some(T::zero())
    }
}
