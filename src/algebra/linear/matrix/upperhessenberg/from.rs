use crate::algebra::linear::matrix::{General, UpperHessenberg};

impl<T> From<General<T>> for UpperHessenberg<T> {
    ///
    /// # Example
    /// ```
    /// use mathru::algebra::linear::matrix::{General, UpperHessenberg};
    /// use mathru::matrix;
    ///
    /// let m: UpperHessenberg<f64> = matrix![1.0, -2.0, 5.0;
    ///                                       2.0, -3.0, 3.0;
    ///                                       0.0, 1.0, -1.5].into();
    /// ```
    fn from(matrix: General<T>) -> Self {
        UpperHessenberg { matrix }
    }
}
