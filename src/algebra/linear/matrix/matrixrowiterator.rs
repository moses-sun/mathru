use std::slice::Iter;
use crate::algebra::abstr::Real;

pub struct MatrixRowIterator<'a, T>
{
    //_phantom: PhantomData<T>
    pub iter: Iter<'a, T>,
}

impl<'a, T> Iterator for MatrixRowIterator<'a, T>
    where T: Real
{
    type Item = T;

    // just return the str reference
    fn next(&mut self) -> Option<Self::Item> {
            //self.iter.next()
            Some(T::zero())
    }
}