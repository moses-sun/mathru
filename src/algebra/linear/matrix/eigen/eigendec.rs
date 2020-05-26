use crate::algebra::linear::{Vector, Matrix};
use serde::{Deserialize, Serialize};
use std::clone::Clone;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EigenDec<T>
{
    value: Vector<T>,
    vector: Matrix<T>
}

impl<T> EigenDec<T>
{
    pub(super) fn new(value: Vector<T>, vector: Matrix<T>) -> EigenDec<T>
    {
        return
        EigenDec
        {
            value: value,
            vector: vector
        };
    }

    pub fn value(self: Self) -> Vector<T>
    {
        return self.value;
    }

    pub fn vector(self: Self) -> Matrix<T>
    {
        return self.vector;
    }

    pub fn pair(self: Self)  -> (Vector<T>, Matrix<T>)
    {
        return (self.value, self.vector);
    }
}