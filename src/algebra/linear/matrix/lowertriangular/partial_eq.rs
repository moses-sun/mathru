use super::LowerTriangular;

impl<T> PartialEq for LowerTriangular<T>
where
    T: PartialEq,
{
    /// Checks if two matrices are equal
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::matrix::{General, LowerTriangular};
    /// use mathru::matrix;
    ///
    /// let a: LowerTriangular<f64> = LowerTriangular::new(matrix![ 1.0, 0.0;
    ///                                                             3.0, -7.0]);
    /// let b: LowerTriangular<f64> = LowerTriangular::new(matrix![ 1.0, 0.0;
    ///                                                             3.0, -7.0]);
    ///
    /// assert_eq!(true, a == b);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        // TODO implement optimized comparison
        self.matrix.eq(&other.matrix)
    }
}
