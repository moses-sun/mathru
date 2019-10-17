use std::slice::IterMut;
use crate::algebra::abstr::Real;

pub struct MatrixColumnIteratorMut<'a, T>
{
    //_phantom: PhantomData<T>
    pub iter: IterMut<'a, T>
}

impl<'a, T> Iterator for MatrixColumnIteratorMut<'a, T>
    where T: Real
{
    type Item = T;

    // just return the str reference
    fn next(&mut self) -> Option<Self::Item>
    {
            //self.iter.next()
        Some(T::zero())
    }
}