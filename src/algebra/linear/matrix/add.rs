use crate::algebra::linear::{Matrix};
use crate::algebra::abstr::{Scalar};
use std::ops::{Add};

impl<T> Add<Self> for Matrix<T>
    where T: Scalar
{
    type Output = Matrix<T>;

    /// Adds two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    ///
    /// assert_eq!(b, Matrix::zero(2, 2) + a);
    /// ```
    fn add(self: Self, rhs: Self) -> Self::Output
    {
        (&self).add(&rhs)
    }
}


///
///Adds two matrices
///
impl<'a, 'b, T> Add<&'b Matrix<T>> for &'a Matrix<T>
    where T: Scalar
{
    type Output = Matrix<T>;

    /// Adds two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    ///
    /// assert_eq!(b, &Matrix::zero(2, 2) + &a);
    /// ```
    fn add(self: Self, rhs: &'b Matrix<T>) -> Self::Output
    {
        assert_eq!(self.dim(), rhs.dim());
        return self.add_r(rhs);
    }
}

impl<'a, 'b, T> Matrix<T>
    where T: Scalar
{
    #[cfg(feature = "native")]
    fn add_r(self: &Self, rhs: &'b Matrix<T>) -> Matrix<T>
    {
        let (m, n) = self.dim();
        Matrix
            {
            m: m,
            n: n,
            data: self.data.iter().zip(rhs.data.iter()).map(|(x, y)| *x + *y).collect::<Vec<T>>()
        }
    }

    #[cfg(feature = "blaslapack")]
    fn add_r(self: &Self, rhs: &'b Matrix<T>) -> Matrix<T>
    {
        let mut c: Matrix<T> = rhs.clone();
        let (b_m, b_n): (usize, usize) = rhs.dim();

        let a: Matrix<T> = Matrix::one(b_m);
        let m: i32 = b_m as i32;
        let n: i32 = b_n as i32;
        let k: i32 = b_m as i32;

        T::xgemm('N' as u8, 'N' as u8, m, n, k, T::one(), &a.data[..], m, &self.data[..], k, T::one(), &mut c.data[..],
         m);

        return c;
    }
}

impl<T> Matrix<T>
    where T: Scalar
{
    pub fn add_func(self: &Self, rhs: &Matrix<T>) -> Matrix<T>
    {
        let (m, n) = self.dim();
        Matrix
            {
            m: m,
            n: n,
            data: self.data.iter().zip(rhs.data.iter()).map(|(x, y)| *x + *y).collect::<Vec<T>>()
        }
    }
}

///
/// Add scalar to matrix
///
impl<'a, 'b, T> Add<&'b T> for &'a Matrix<T>
    where T: Scalar
{
    type Output = Matrix<T>;

    /// Add a scalar to the matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![-3.0, -4.0, -1.0, -11.0]);
    ///
    /// assert_eq!(b, &a + &-4.0);
    /// ```
    fn add(self: Self, rhs: &T) -> Self::Output
    {
        return self.apply(&|x: &T| -> T {x.clone() + rhs.clone()});
    }
}

///
/// Add scalar to matrix
///
impl<T> Add<T> for Matrix<T>
    where T: Scalar
{
    type Output = Matrix<T>;

    /// Add a scalar to the matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![-3.0, -4.0, -1.0, -11.0]);
    ///
    /// assert_eq!(b, a + -4.0);
    /// ```
    fn add(self: Self, rhs: T) -> Self::Output
    {
        return (&self).add(&rhs);
    }
}

