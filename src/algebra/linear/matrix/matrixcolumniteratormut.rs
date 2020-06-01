use crate::algebra::abstr::{Field, Scalar};
use std::slice::IterMut;

pub struct MatrixColumnIteratorMut<'a, T>
{
    //_phantom: PhantomData<T>
    pub iter: IterMut<'a, T>,
}

impl<'a, T> Iterator for MatrixColumnIteratorMut<'a, T> where T: Field + Scalar
{
    type Item = T;

    // just return the str reference
    fn next(&mut self) -> Option<Self::Item>
    {
        //self.iter.next()
        Some(T::zero())
    }
}
