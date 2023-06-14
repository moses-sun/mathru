use crate::algebra::{
    abstr::{Field, Scalar},
    linear::General,
};
use std::ops::Add;

impl<T> Add<General<T>> for General<T>
where
    T: Field + Scalar,
{
    type Output = General<T>;

    /// Adds two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: General<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: General<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let c = a + b;
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.dim(), rhs.dim());

        let (m, n): (usize, usize) = rhs.dim();

        let mut c: General<T> = rhs;

        T::xaxpy(
            (m * n) as i32,
            T::one(),
            &self.data[..],
            1,
            &mut c.data[..],
            1,
        );

        return c;
    }
}

///
///Adds two matrices
impl<'a, 'b, T> Add<&'b General<T>> for &'a General<T>
where
    T: Field + Scalar,
{
    type Output = General<T>;

    /// Adds two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: General<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: General<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// ```
    fn add(self, rhs: &'b General<T>) -> Self::Output {
        assert_eq!(self.dim(), rhs.dim());

        let (m, n): (usize, usize) = rhs.dim();

        let mut c: General<T> = rhs.clone();

        T::xaxpy(
            (m * n) as i32,
            T::one(),
            &self.data[..],
            1,
            &mut c.data[..],
            1,
        );

        return c;
    }
}

///
///Adds two matrices
impl<'a, 'b, T> Add<&'b General<T>> for &'a mut General<T>
where
    T: Field + Scalar,
{
    type Output = &'a mut General<T>;

    /// Adds two matrices
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let mut a: General<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: General<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let _ = &mut a + & b;
    /// ```
    fn add(self, rhs: &'b General<T>) -> Self::Output {
        assert_eq!(self.dim(), rhs.dim());

        let (m, n): (usize, usize) = rhs.dim();

        T::xaxpy(
            (m * n) as i32,
            T::one(),
            &rhs.data[..],
            1,
            &mut self.data[..],
            1,
        );

        return self;
    }
}

///
/// Add scalar to matrix
impl<T> Add<T> for General<T>
where
    T: Field + Scalar,
{
    type Output = General<T>;

    /// Add a scalar to the matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: General<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: General<f64> = Matrix::new(2, 2, vec![-3.0, -4.0, -1.0, -11.0]);
    ///
    /// ```
    fn add(mut self, rhs: T) -> Self::Output {
        let _ = &mut self + &rhs;
        self
    }
}

///
/// Add scalar to matrix
impl<'a, 'b, T> Add<&'b T> for &'a General<T>
where
    T: Field + Scalar,
{
    type Output = General<T>;

    /// Add a scalar to the matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: General<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: General<f64> = Matrix::new(2, 2, vec![-3.0, -4.0, -1.0, -11.0]);
    /// ```
    fn add(self, rhs: &T) -> Self::Output {
        let mut a: General<T> = self.clone();
        let _ = &mut a + rhs;
        a
    }
}

///
/// Add scalar to matrix
impl<'a, 'b, T> Add<&'b T> for &'a mut General<T>
where
    T: Field + Scalar,
{
    type Output = &'a mut General<T>;

    /// Add a scalar to the matrix
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    ///
    /// let a: General<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: General<f64> = Matrix::new(2, 2, vec![-3.0, -4.0, -1.0, -11.0]);
    /// ```
    fn add(self, rhs: &T) -> Self::Output {
        self.data.iter_mut().for_each(|x: &mut T| *x += *rhs);
        self
    }
}
