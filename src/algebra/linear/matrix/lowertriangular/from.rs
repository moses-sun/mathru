use crate::algebra::linear::matrix::{General, LowerTriangular, UnitLowerTriangular};

impl<T> From<General<T>> for LowerTriangular<T> {
    fn from(matrix: General<T>) -> Self {
        LowerTriangular { matrix }
    }
}

impl<T> From<UnitLowerTriangular<T>> for LowerTriangular<T> {
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
    fn from(ut: UnitLowerTriangular<T>) -> Self {
        LowerTriangular { matrix: ut.matrix }
    }
}
