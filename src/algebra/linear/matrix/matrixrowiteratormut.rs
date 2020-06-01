use crate::algebra::abstr::{Field, Scalar};
use std::slice::IterMut;

pub struct MatrixRowIteratorMut<'a, T>
{
    //_phantom: PhantomData<T>
    pub iter: IterMut<'a, T>,
}

impl<'a, T> Iterator for MatrixRowIteratorMut<'a, T> where T: Field + Scalar
{
    type Item = T;

    // just return the str reference
    fn next(&mut self) -> Option<Self::Item>
    {
        //self.iter.next()
        Some(T::zero())
    }
}
