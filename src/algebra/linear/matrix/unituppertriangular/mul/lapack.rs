use std::ops::Mul;

use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{matrix::UnitUpperTriangular, vector::Vector},
};

impl<'a, 'b, T> Mul<&'b UnitUpperTriangular<T>> for &'a UnitUpperTriangular<T>
where
    T: Field + Scalar,
{
    type Output = UnitUpperTriangular<T>;

    fn mul(self, rhs: &'b UnitUpperTriangular<T>) -> Self::Output {
        let mut this = self.clone();

        let _ = (&mut this).mul(rhs);
        this
    }
}

impl<'a, 'b, T> Mul<&'b UnitUpperTriangular<T>> for &'a mut UnitUpperTriangular<T>
where
    T: Field + Scalar,
{
    type Output = &'a mut UnitUpperTriangular<T>;

    fn mul(self, rhs: &'b UnitUpperTriangular<T>) -> Self::Output {
        debug_assert_eq!(self.dim(), rhs.dim());

        let (m, n): (usize, usize) = rhs.dim();

        T::xtrmm(
            'R',
            'U',
            'N',
            'U',
            m as i32,
            n as i32,
            T::one(),
            &rhs.matrix.data[..],
            m as i32,
            &mut self.matrix.data[..],
            m as i32,
        );

        println!("{:?}", self);

        self
    }
}

impl<T> Mul<UnitUpperTriangular<T>> for UnitUpperTriangular<T>
where
    T: Field + Scalar,
{
    type Output = UnitUpperTriangular<T>;

    fn mul(mut self, rhs: UnitUpperTriangular<T>) -> Self::Output {
        let _ = &mut self * &rhs;
        self
    }
}

/// Multiplies matrix by vector.
impl<'a, 'b, T> Mul<&'b Vector<T>> for &'a UnitUpperTriangular<T>
where
    T: Field + Scalar,
{
    type Output = Vector<T>;

    fn mul(self, v: &'b Vector<T>) -> Vector<T> {
        // Implement optimized version
        (&self.matrix).mul(v)
    }
}
