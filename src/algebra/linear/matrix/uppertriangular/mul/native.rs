use std::ops::Mul;

use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{matrix::UpperTriangular, Vector},
};

/// Multiplies matrix by vector.
impl<'a, 'b, T> Mul<&'b Vector<T>> for &'a UpperTriangular<T>
where
    T: Field + Scalar,
{
    type Output = Vector<T>;

    fn mul(self, v: &'b Vector<T>) -> Vector<T> {
        // Implement optimized version
        (&self.matrix).mul(v)
    }
}
