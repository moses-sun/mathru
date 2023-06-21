use std::ops::Mul;

use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{matrix::LowerTriangular, vector::Vector},
};

/// Multiplies matrix by vector.
impl<'a, 'b, T> Mul<&'b Vector<T>> for &'a LowerTriangular<T>
where
    T: Field + Scalar,
{
    type Output = Vector<T>;

    fn mul(self, v: &'b Vector<T>) -> Vector<T> {
        // Implement optimized version
        (&self.matrix).mul(v)
    }
}
