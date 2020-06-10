use crate::algebra::linear::Matrix;
use serde::{Deserialize, Serialize};
use std::clone::Clone;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HessenbergDec<T>
{
    q: Matrix<T>,
    h: Matrix<T>,
}

impl<T> HessenbergDec<T>
{
    pub(super) fn new(q: Matrix<T>, h: Matrix<T>) -> HessenbergDec<T>
    {
        return HessenbergDec { q, h };
    }

    pub fn q(self: Self) -> Matrix<T>
    {
        return self.q;
    }

    pub fn h(self: Self) -> Matrix<T>
    {
        return self.h;
    }

    pub fn qh(self: Self) -> (Matrix<T>, Matrix<T>)
    {
        return (self.q, self.h);
    }
}
