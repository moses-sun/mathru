use std::ops::Mul;

use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{matrix::UnitLowerTriangular, vector::Vector},
};

impl<'a, 'b, T> Mul<&'b UnitLowerTriangular<T>> for &'a UnitLowerTriangular<T>
where
    T: Field + Scalar,
{
    type Output = UnitLowerTriangular<T>;

    fn mul(self, rhs: &'b UnitLowerTriangular<T>) -> Self::Output {
        // Implement optimized version
        UnitLowerTriangular {
            matrix: (&self.matrix).mul(&rhs.matrix),
        }
    }
}

impl<'a, 'b, T> Mul<&'b UnitLowerTriangular<T>> for &'a mut UnitLowerTriangular<T>
where
    T: Field + Scalar,
{
    type Output = &'a mut UnitLowerTriangular<T>;

    fn mul(self, rhs: &'b UnitLowerTriangular<T>) -> Self::Output {
        // Implement optimized version

        let _ = (&mut self.matrix).mul(&rhs.matrix);
        self
    }
}

impl<T> Mul<UnitLowerTriangular<T>> for UnitLowerTriangular<T>
where
    T: Field + Scalar,
{
    type Output = UnitLowerTriangular<T>;

    fn mul(mut self, rhs: UnitLowerTriangular<T>) -> Self::Output {
        let _ = &mut self * &rhs;
        self
    }
}

/// Multiplies matrix by vector.
impl<'a, 'b, T> Mul<&'b Vector<T>> for &'a UnitLowerTriangular<T>
where
    T: Field + Scalar,
{
    type Output = Vector<T>;

    fn mul(self, v: &'b Vector<T>) -> Vector<T> {
        // Implement optimized version
        (&self.matrix).mul(v)
    }
}
