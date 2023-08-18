use std::ops::Mul;

use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{matrix::UpperTriangular, vector::Vector},
};

impl<'a, 'b, T> Mul<&'b UpperTriangular<T>> for &'a UpperTriangular<T>
where
    T: Field + Scalar,
{
    type Output = UpperTriangular<T>;

    fn mul(self, rhs: &'b UpperTriangular<T>) -> Self::Output {
        let mut this = self.clone();

        let _ = (&mut this).mul(rhs);
        this
    }
}

impl<'a, 'b, T> Mul<&'b UpperTriangular<T>> for &'a mut UpperTriangular<T>
where
    T: Field + Scalar,
{
    type Output = &'a mut UpperTriangular<T>;

    fn mul(self, rhs: &'b UpperTriangular<T>) -> Self::Output {
        debug_assert_eq!(self.dim(), rhs.dim());

        let (m, n): (usize, usize) = rhs.dim();

        T::xtrmm(
            'R',
            'U',
            'N',
            'N',
            m as i32,
            n as i32,
            T::one(),
            &rhs.matrix.data[..],
            m as i32,
            &mut self.matrix.data[..],
            m as i32,
        );

        self
    }
}

impl<T> Mul<UpperTriangular<T>> for UpperTriangular<T>
where
    T: Field + Scalar,
{
    type Output = UpperTriangular<T>;

    fn mul(mut self, rhs: UpperTriangular<T>) -> Self::Output {
        let _ = &mut self * &rhs;
        self
    }
}

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
