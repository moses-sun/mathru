use crate::algebra::{
    abstr::{Field, Scalar},
    linear::Matrix,
};

use super::Transpose;

impl<T> Transpose for Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;
    /// Function to transpose a matrix without allocating memory for the
    /// transposed matrix
    ///
    /// catanzaro.name/papers/PPoPP-2014.pdf
    /// TODO
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    /// use mathru::algebra::linear::matrix::Transpose;
    ///
    /// let mut uut: Matrix<f64> = Matrix::new(4, 2, vec![1.0, 0.0, 3.0, 0.0, 1.0, -7.0, 0.5, 0.25]);
    /// uut = uut.transpose();
    ///
    /// let refer: Matrix<f64> = Matrix::new(2, 4, vec![1.0, 1.0, 0.0, -7.0, 3.0, 0.5, 0.0, 0.25]);
    /// assert_eq!(refer, uut);
    /// ```
    fn transpose(self: Self) -> Matrix<T>
    {
        let (m, n): (usize, usize) = self.dim();
        let mut matrix_t: Matrix<T> = Matrix::zero(n, m);

        for i in 0..m
        {
            for j in 0..n
            {
                *matrix_t.get_mut(j, i) = *self.get(i, j);
            }
        }

        return matrix_t;
    }
}