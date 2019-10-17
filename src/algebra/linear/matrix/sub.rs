use crate::algebra::linear::{Matrix};
use crate::algebra::abstr::{Real};
use std::ops::{Sub};

impl <T> Sub for Matrix<T>
    where T: Real
{
    type Output = Matrix<T>;

    /// Subtracts two matrices
    ///
    /// A = (a_{ij}) \in T^{m \times n}
    /// B = (b_{ij}) \in T^{m \times n}
    /// A - B = ( a_{ij} - b_{ij} )
    ///
    /// # Arguments
    ///
    /// rhs:
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    ///
    /// assert_eq!(Matrix::zero(2, 2), a - b);
    /// ```
    fn sub(self: Self, rhs: Self) -> Self::Output
    {
        (&self).sub(&rhs)
    }
}


impl<'a, 'b, T> Sub<&'b Matrix<T>> for &'a Matrix<T>
    where T: Real
{
    type Output = Matrix<T>;

    fn sub(self: Self, rhs: &'b Matrix<T>) -> Self::Output
    {
        assert_eq!(self.dim(), rhs.dim());
        return self.sub_r(rhs);
    }
}

impl<'a, 'b, T> Matrix<T>
    where T: Real
{
    #[cfg(feature = "native")]
    fn sub_r(self: &Self, rhs: &'b Matrix<T>) -> Matrix<T>
    {
        let (m, n) = rhs.dim();
        Matrix
        {
            m: m,
            n: n,
            data: self.data.iter().zip(rhs.data.iter()).map(|(x, y)| *x - *y).collect::<Vec<T>>()
        }
    }

    #[cfg(feature = "blaslapack")]
    fn sub_r(self: &Self, rhs: &'b Matrix<T>) -> Matrix<T>
    {
        let mut c: Matrix<T> = rhs.clone();
        let (b_m, b_n): (usize, usize) = rhs.dim();

        let a: Matrix<T> = Matrix::one(b_m);
        let m: i32 = b_m as i32;
        let n: i32 = b_n as i32;
        let k: i32 = b_m as i32;


        T::xgemm('N' as u8, 'N' as u8, m, n, k, T::one(), &a.data[..], m, &self.data[..], k, -T::one(), &mut c.data[..],
         m);

        return c;
    }
}


///
/// Subtracts scalar from all matrix elements
///
impl<'a, 'b, T> Sub<&'b T> for &'a Matrix<T>
    where T: Real
{
    type Output = Matrix<T>;

    /// Subtracts a scalar value from all matrix elements
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![5.0, 4.0, 7.0, -3.0]);
    ///
    /// assert_eq!(b, &a - &-4.0);
    /// ```
    fn sub(self: Self, rhs: &T) -> Self::Output
    {
        return self.apply(&|x: &T| -> T {x.clone() - rhs.clone()});
    }
}

impl<T> Sub<T> for Matrix<T>
    where T: Real
{
    type Output = Matrix<T>;

    /// Subtracts a scalar from all matrix elements
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![5.0, 4.0, 7.0, -3.0]);
    ///
    /// assert_eq!(b, a - -4.0);
    /// ```
    fn sub(self: Self, rhs: T) -> Self::Output
    {
        return (&self).sub(&rhs);
    }
}