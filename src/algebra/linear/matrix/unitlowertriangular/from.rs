use crate::algebra::linear::matrix::{General, UnitLowerTriangular};

impl<T> From<General<T>> for UnitLowerTriangular<T> {
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::matrix::{General, UnitLowerTriangular};
    /// use mathru::matrix;
    ///
    /// let m: UnitLowerTriangular<f64> = matrix![1.0, 0.0;
    ///                                           -2.0, 1.0].into();
    /// ```
    fn from(matrix: General<T>) -> Self {
        UnitLowerTriangular { matrix }
    }
}
