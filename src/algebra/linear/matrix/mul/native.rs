use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{Matrix, Vector},
};
use std::ops::Mul;

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Field + Scalar,
{
    type Output = Matrix<T>;

    /// Multiplies two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, -18.0, 49.0]);
    /// assert_eq!(res_ref, a * b);
    /// ```
    fn mul(mut self, rhs: Self) -> Self::Output {
        let _ = &mut self * &rhs;
        self
    }
}

impl<'a, 'b, T> Mul<&'b Matrix<T>> for &'a mut Matrix<T>
where
    T: Field + Scalar,
{
    type Output = &'a mut Matrix<T>;
    /// Multiplies two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let mut a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, -18.0, 49.0]);
    /// assert_eq!(res_ref, *(&mut a * &b));
    /// ```
    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
        let (l_rows, l_cols) = self.dim();
        let (r_rows, r_cols): (usize, usize) = rhs.dim();

        assert_eq!(l_cols, r_rows);

        let m = l_rows as usize;
        let n = r_cols as usize;
        let k = l_cols as usize;

        T::xgemm(
            m,
            k,
            n,
            T::one(),
            self.clone().data[..].as_ptr(),
            1,
            m as isize,
            rhs.data[..].as_ptr(),
            1,
            k as isize,
            T::zero(),
            self.data[..].as_mut_ptr(),
            1,
            m as isize,
        );

        self.data.truncate(m * n);
        self.m = l_rows;
        self.n = r_cols;
        self
    }
}

// Multiplies matrix by vector.
impl<T> Mul<Vector<T>> for Matrix<T>
where
    T: Field + Scalar,
{
    type Output = Vector<T>;

    fn mul(self, m: Vector<T>) -> Vector<T> {
        (&self) * (&m)
    }
}

/// Multiplies matrix by vector.
impl<'a, 'b, T> Mul<&'b Vector<T>> for &'a Matrix<T>
where
    T: Field + Scalar,
{
    type Output = Vector<T>;

    fn mul(self, v: &'b Vector<T>) -> Vector<T> {
        let (self_m, self_n): (usize, usize) = self.dim();
        let (v_m, v_n): (usize, usize) = v.dim();

        if self_n != v_m && v_n != 1 {
            panic!("Matrix and Vector dimension do not match");
        }

        let m = self_m;
        let k = self_n;
        let n = v_n;

        let mut prod_data = Vec::with_capacity(m);
        unsafe { prod_data.set_len(m) }

        T::xgemm(
            m,
            k,
            n,
            T::one(),
            self.data[..].as_ptr(),
            1,
            m as isize,
            v.data.data[..].as_ptr(),
            1,
            k as isize,
            T::zero(),
            prod_data.as_mut_ptr(),
            1,
            m as isize,
        );

        Vector::new_column(prod_data)
    }
}

/// Multiplies matrix by scalar
impl<T> Mul<T> for Matrix<T>
where
    T: Field + Scalar,
{
    type Output = Matrix<T>;

    /// Multiplies a matrix with a scalar
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let f: f64 = 7.0;
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![7.0, 0.0, 21.0, -49.0]);
    ///
    /// assert_eq!(res_ref, a * f);
    /// ```
    fn mul(mut self, m: T) -> Matrix<T> {
        let _ = &mut self * &m;
        self
    }
}

// Multiplies matrix by scalar
impl<'a, 'b, T> Mul<&'b T> for &'a Matrix<T>
where
    T: Field + Scalar,
{
    type Output = Matrix<T>;

    /// Multiplies a matrix with a scalar
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![4.0, 0.0, 12.0, -28.0]);
    ///
    /// assert_eq!(res_ref, &a * &4.0);
    /// ```
    fn mul(self, k: &'b T) -> Matrix<T> {
        let (m, n) = self.dim();
        Matrix {
            m,
            n,
            data: self.data.iter().map(&|x: &T| *x * *k).collect::<Vec<T>>(),
        }
    }
}

//
impl<'a, 'b, T> Mul<&'b T> for &'a mut Matrix<T>
where
    T: Field + Scalar,
{
    type Output = &'a mut Matrix<T>;

    /// Multiplies two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = &a * &-4.0;
    /// ```
    fn mul(self, rhs: &'b T) -> Self::Output {
        let _ = self.data.iter_mut().for_each(&|a: &mut T| *a *= *rhs);
        self
    }
}

impl<'a, 'b, T> Mul<&'b Matrix<T>> for &'a Matrix<T>
where
    T: Field + Scalar,
{
    type Output = Matrix<T>;

    /// Multiplies two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let res_ref: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, -18.0, 49.0]);
    /// assert_eq!(res_ref, &a * &b);
    /// ```
    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
        let (self_rows, self_cols) = self.dim();
        let (rhs_rows, rhs_cols) = rhs.dim();

        assert_eq!(self_cols, rhs_rows);

        let m = self_rows as usize;
        let n = rhs_cols as usize;
        let k = self_cols as usize;
        let mut c: Matrix<T> = Matrix::zero(m as usize, n as usize);

        T::xgemm(
            m,
            k,
            n,
            T::one(),
            self.data[..].as_ptr(),
            1,
            m as isize,
            rhs.data[..].as_ptr(),
            1,
            k as isize,
            T::zero(),
            c.data[..].as_mut_ptr(),
            1,
            m as isize,
        );

        return c;
    }
}
