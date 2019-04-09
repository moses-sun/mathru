use algebra::linear::{Vector, Matrix};
use algebra::abstr::{Zero, One, Real};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, Neg, Div};

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
                row_column_product += self.data[i * self.n + k] * *v.get( &k);
            }
            prod_data.push(row_column_product);
        }

        Vector::new_column(self.m, prod_data)
    }
}

impl<T> Matrix<T>
    where T: Mul<T, Output = T> + Zero + Clone
{
    #[deprecated(since = "0.1.1")]
    pub fn mul_scalar<'a, 'b>(self: &'a Self, rhs: &'b T) -> Matrix<T>
    {
        let (rows, cols): (usize, usize) = self.dim();

        let mut prod: Matrix<T> = Matrix::zero(rows, cols);

        for i in 0..rows
        {
            for j in 0..cols
            {
                *prod.get_mut(&i, &j) = self.get(&i, &j).clone() * (*rhs).clone();
            }
        }
        prod
    }


}

//Multiplies matrix by scalar
impl<T> Mul<T> for Matrix<T>
    where T: Copy + Zero + Mul<T, Output = T> + Add<T, Output = T>
{
    type Output = Matrix<T>;

    /// Multiplies a matrix with a scalar
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
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
    where T: AddAssign + Zero + Mul<T, Output = T> + Clone
{
    type Output = Matrix<T>;

    /// Multiplies two matrices
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
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

impl <'a, 'b, T> Mul<&'b Matrix<T>> for &'a Matrix<T>
    where T: Mul<T, Output = T> + AddAssign + Zero + Clone
{
    type Output = Matrix<T>;

    /// Multiplies two matrices
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, -18.0, 49.0]);
    /// assert_eq!(res_ref, &a * &b);
    /// ```
    fn mul(self: Self, rhs: &'b Matrix<T>) -> Self::Output
    {
        let (l_rows, l_cols) = self.dim();
        let (r_rows, r_cols): (usize, usize) = rhs.dim();
        assert_eq!(l_cols, r_rows);

        let mut prod: Matrix<T> = Matrix::zero(l_rows, r_cols);

        for i in 0..l_rows
        {
            for j in 0..r_cols
            {
                let mut sum: T = T::zero();
                for k in 0..l_cols
                {
                    sum += self.get(&i, &k).clone() * rhs.get(&k, &j).clone();
                }
                *prod.get_mut(&i, &j) = sum;
            }
        }
        prod
    }
}

// Multiplies matrix by scalar
impl<'a, 'b, T> Mul<&'b T> for &'a Matrix<T>
    where T: Copy + Zero + Mul<T, Output = T> + Add<T, Output = T>
{
    type Output = Matrix<T>;

    #[cfg(feature = "native")]
    fn mul(self, m: &'b T) -> Matrix<T>
    {
        (self.clone().apply(&|&x| x * *m))
    }

    #[cfg(feature = "blaslapack")]
    fn mul(self, m: &'b T) -> Matrix<T>
    {
        (self.clone().apply(&|&x| x * *m))
    }
}