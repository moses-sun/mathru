use crate::algebra::{
    abstr::{Field, Scalar},
    linear::matrix::General,
};
use std::ops::Sub;

impl<T> Sub for General<T>
where
    T: Field + Scalar,
{
    type Output = General<T>;

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
    /// use mathru::algebra::linear::matrix::General;
    /// use mathru::matrix;
    ///
    /// let a: General<f64> = matrix![1.0, 0.0;  
    ///                               3.0, -7.0];
    /// let b: General<f64> = matrix![1.0, 0.0;
    ///                              3.0, -7.0];
    ///
    /// let c: General<f64> = a - b;
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert_eq!(self.dim(), rhs.dim());

        let (m, n): (usize, usize) = rhs.dim();

        let mut c: General<T> = self;

        T::xaxpy(
            (m * n) as i32,
            -T::one(),
            &rhs.data[..],
            1,
            &mut c.data[..],
            1,
        );

        c
    }
}

impl<'a, 'b, T> Sub<&'b General<T>> for &'a General<T>
where
    T: Field + Scalar,
{
    type Output = General<T>;

    fn sub(self, rhs: &'b General<T>) -> Self::Output {
        debug_assert_eq!(self.dim(), rhs.dim());

        let (m, n): (usize, usize) = rhs.dim();

        let mut c: General<T> = self.clone();

        T::xaxpy(
            (m * n) as i32,
            -T::one(),
            &rhs.data[..],
            1,
            &mut c.data[..],
            1,
        );

        c
    }
}

impl<'a, 'b, T> Sub<&'b General<T>> for &'a mut General<T>
where
    T: Field + Scalar,
{
    type Output = &'a mut General<T>;

    fn sub(self, rhs: &'b General<T>) -> Self::Output {
        debug_assert_eq!(self.dim(), rhs.dim());

        let (m, n): (usize, usize) = rhs.dim();

        T::xaxpy(
            (m * n) as i32,
            -T::one(),
            &rhs.data[..],
            1,
            &mut self.data[..],
            1,
        );

        self
    }
}

impl<T> Sub<T> for General<T>
where
    T: Field + Scalar,
{
    type Output = General<T>;

    /// Subtracts a scalar from all matrix elements
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::matrix::General;
    /// use mathru::matrix;
    ///
    /// let a: General<f64> = matrix![1.0, 0.0;
    ///                              3.0, -7.0];
    /// let b: General<f64> = a - -4.0;
    /// ```
    fn sub(self, rhs: T) -> Self::Output {
        self.apply_mut(&|x: &T| -> T { *x - rhs })
    }
}

///
/// Subtracts scalar from all matrix elements
impl<'a, 'b, T> Sub<&'b T> for &'a General<T>
where
    T: Field + Scalar,
{
    type Output = General<T>;

    /// Subtracts a scalar value from all matrix elements
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::matrix::General;
    /// use mathru::matrix;
    ///
    /// let a: General<f64> = matrix![1.0, 0.0;
    ///                               3.0, -7.0];
    ///
    /// let b: General<f64> = &a - &-4.0;
    /// ```
    fn sub(self, rhs: &T) -> Self::Output {
        self.apply(&|x: &T| -> T { x.clone() - rhs.clone() })
    }
}

///
/// Subtracts scalar from all matrix elements
impl<'a, 'b, T> Sub<&'b T> for &'a mut General<T>
where
    T: Field + Scalar,
{
    type Output = &'a mut General<T>;

    /// Subtracts a scalar value from all matrix elements
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::matrix::General;
    /// use mathru::matrix;
    ///
    /// let a: General<f64> = matrix![1.0, 0.0;
    ///                               3.0, -7.0];
    ///
    /// let b: General<f64> = &a - &-4.0;
    /// ```
    fn sub(self, rhs: &T) -> Self::Output {
        println!("hier her");
        self.data.iter_mut().for_each(|x: &mut T| *x -= *rhs);
        self
    }
}
