use crate::algebra::abstr::{Field, Scalar, Zero};
use crate::algebra::linear::{Vector};
use crate::vector;
use std::slice::IterMut;

pub struct MatrixRowIteratorMut<'a, T>
{
    iter: IterMut<'a, T>,
    vec: Vector<T>,
}

impl<'a, T> MatrixRowIteratorMut<'a, T>
    where T: Zero
{
    pub fn new(iter: IterMut<'a, T>) -> MatrixRowIteratorMut<'a, T>
    {
        return MatrixRowIteratorMut{iter: iter, vec: vector![T::zero()]};
    }
}

impl<'a, T> Iterator for MatrixRowIteratorMut<'a, T> where T: Field + Scalar
{
    type Item = &'a mut Vector<T>;

    fn next(&'a mut self) -> Option<Self::Item>
    {
        Some(mut &'a self.vec)
    }
}
