use crate::algebra::linear::{Vector, Matrix};
use crate::algebra::abstr::{Zero, One, Real};
use std::ops::{Add, AddAssign, Mul};


/// Multiplies matrix by vector.
impl<'a, 'b, T> Mul<&'b Vector<T>> for &'a Matrix<T>
    where T: Copy + Zero + Mul<T, Output = T> + Add<T, Output = T> + One + AddAssign
{
    type Output = Vector<T>;

    fn mul(self, v: &'b Vector<T>) -> Vector<T>
    {
        let (_self_m, self_n): (usize, usize) = self.dim();
        let (v_m, _v_n): (usize, usize) = v.dim();

        if self_n != v_m
        {
            panic!("Matrix and Vector dimension do not match");
        }

        let mut prod_data = Vec::with_capacity(self.m);

        for i in 0..self.m
        {
            let mut row_column_product: T = T::zero();
            for k in 0..self.n
            {
                row_column_product += self.data[k * self.m + i] * *v.get(k);
            }
            prod_data.push(row_column_product);
        }

        Vector::new_column(self.m, prod_data)
    }
}

//Multiplies matrix by scalar
impl<T> Mul<T> for Matrix<T>
    where T: Real
{
    type Output = Matrix<T>;

    /// Multiplies a matrix with a scalar
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let f: f64 = 7.0;
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![7.0, 0.0, 21.0, -49.0]);
    ///
    /// assert_eq!(res_ref, a * f);
    /// ```
    fn mul(self, m: T) -> Matrix<T>
    {
        (&self).mul(&m)
    }
}

// Multiplies matrix by scalar
impl<'a, 'b, T> Mul<&'b T> for &'a Matrix<T>
    where T: Real
{
    type Output = Matrix<T>;

    /// Multiplies a matrix with a scalar
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![4.0, 0.0, 12.0, -28.0]);
    ///
    /// assert_eq!(res_ref, &a * &4.0);
    /// ```
    fn mul(self: Self, m: &'b T) -> Matrix<T>
    {
        return self.clone().mul_scalar(m);
    }
}


// Multiplies matrix by vector.
impl<T> Mul<Vector<T>> for Matrix<T>
    where T: Copy + Zero + Mul<T, Output = T> + Add<T, Output = T> + One + AddAssign
{
    type Output = Vector<T>;

    fn mul(self, m: Vector<T>) -> Vector<T>
    {
        (&self) * (&m)
    }
}


impl <T> Mul<Matrix<T>> for Matrix<T>
    where T: Real
{
    type Output = Matrix<T>;

    /// Multiplies two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, -18.0, 49.0]);
    /// assert_eq!(res_ref, a * b);
    /// ```
    fn mul(self: Self, rhs: Self) -> Self::Output
    {
        (&self).mul(&rhs)
    }
}


impl<'a, 'b, T> Mul<&'b Matrix<T>> for &'a Matrix<T>
    where T: Real
{
    type Output = Matrix<T>;

    /// Multiplies two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, -18.0, 49.0]);
    /// assert_eq!(res_ref, &a * &b);
    /// ```
    fn mul(self: Self, rhs: &'b Matrix<T>) -> Self::Output
	{
     	let (_l_rows, l_cols) = self.dim();
        let (r_rows, _r_cols): (usize, usize) = rhs.dim();
        assert_eq!(l_cols, r_rows);

        return self.mul_r(rhs);
	}
}

impl<'a, 'b, T> Matrix<T>
    where T: Real
{

    #[cfg(feature = "native")]
    fn mul_r(self: &'a Self, rhs: &'b Matrix<T>) -> Matrix<T>
    {
        let (l_rows, l_cols) = self.dim();
        let (_r_rows, r_cols): (usize, usize) = rhs.dim();
        let mut prod: Matrix<T> = Matrix::zero(l_rows, r_cols);

        for i in 0..l_rows
        {
            for j in 0..r_cols
            {
                let mut sum: T = T::zero();
                for k in 0..l_cols
                {
                    sum += self.get(i, k).clone() * rhs.get(k, j).clone();
                }
                *prod.get_mut(i, j) = sum;
            }
        }
        prod
    }

    #[cfg(feature = "blaslapack")]
    fn mul_r(self: &'a Self, rhs: &'b Matrix<T>) -> Matrix<T>
    {
        let (self_rows, self_cols) = self.dim();
        let (_rhs_rows, rhs_cols) = rhs.dim();

        let m = self_rows as i32;
        let n = rhs_cols as i32;
        let k = self_cols as i32;
        let mut c: Matrix<T> = Matrix::zero(m as usize, n as usize);

        T::xgemm('N' as u8, 'N' as u8, m, n, k, T::one(), &self.data[..], m, &rhs.data[..], k, T::zero(), &mut c.data[
        ..],
         m);

        return c;

    }
}



impl<'a, 'b, T> Matrix<T>
    where T: Real
{
    #[cfg(feature = "native")]
    fn mul_scalar(self: Self, m: &'b T) -> Matrix<T>
    {
        self.apply_mut(&|&x| x * *m)
    }

    #[cfg(feature = "blaslapack")]
    fn mul_scalar(mut self: Self, s: &'b T) -> Matrix<T>
    {
        let (rows, cols) = self.dim();

        let m: i32 = rows as i32;
        let n: i32 = cols as i32;
        let k: i32 = n;

        let a: Vec<T>= vec![T::zero(); rows * cols];
        let b: Vec<T>= vec![T::zero(); cols * rows];

        T::xgemm('N' as u8, 'N' as u8, m, n, k, T::zero(), &a, m, &b, k, *s, &mut self.data[..],
         m);

        return self;
    }
}