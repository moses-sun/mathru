use crate::algebra::abstr::{AbsDiffEq, Field, Scalar};
use crate::algebra::linear::matrix::Diagonal;
use crate::algebra::linear::matrix::{EigenDec, EigenDecomposition, General, UnitLowerTriangular};
use crate::elementary::Power;

impl<T> EigenDecomposition<T> for UnitLowerTriangular<T>
where
    T: Field + Scalar + Power + AbsDiffEq<Epsilon = T>,
{
    /// Computes the eigen decomposition
    ///
    /// # Arguments
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::matrix::{EigenDec, EigenDecomposition, General, UnitLowerTriangular};
    /// use mathru::matrix;
    ///
    /// let a: UnitLowerTriangular<f64> = matrix![  1.0, 0.0, 0.0;
    ///                                             3.0, 1.0, 0.0;
    ///                                             6.0, -6.0, 1.0].into();
    ///
    /// let eigen: EigenDec<f64> = a.dec_eigen().unwrap();
    /// ```
    fn dec_eigen(&self) -> Result<EigenDec<T>, String> {
        let (m, _): (usize, usize) = self.dim();

        let values: Diagonal<T> = Diagonal::new(&vec![T::one(); m]);
        let vectors: General<T> = General::zero(m, m);

        Ok(EigenDec::new(values, vectors))
    }
}
