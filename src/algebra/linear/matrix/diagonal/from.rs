use crate::algebra::linear::matrix::{Diagonal, General};

impl<T> From<General<T>> for Diagonal<T> {
    /// Construct a Diagonal view of the matrix 'matrix'. Entries of 'matrix' below and above the diagonal are ignored.
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::matrix::{General, Diagonal};
    /// use mathru::matrix;
    ///
    /// let m: Diagonal<f64> = matrix![1.0, 0.0;
    ///                                0.0, 1.0].into();
    /// ```
    fn from(matrix: General<T>) -> Self {
        Diagonal { matrix }
    }
}
