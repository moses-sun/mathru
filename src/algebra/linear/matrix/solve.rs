use crate::algebra::linear::{Vector, Matrix};
use crate::algebra::abstr::{Real};

#[cfg(feature = "blaslapack")]
use crate::algebra::abstr::{Zero};

impl<T> Matrix<T>
    where T: Real
{
    /// Solves Ax = y
    ///  where A \in R^{m * n}, x \in R^n, y \in R^m
    ///
    ///
    pub fn solve_vector(self: &Self, y: &Vector<T>) -> Vector<T>
    {
        return self.solve_vector_r(y);
    }

    #[cfg(feature = "blaslapack")]
    pub fn solve_vector_r(self: &Self, y: &Vector<T>) -> Vector<T>
    {
        let (m, n): (usize, usize) = self.dim();
        let m_i32: i32 = m as i32;
        let n_i32: i32 = n as i32;

        let (y_m, y_n): (usize, usize) = y.dim();
        let y_n_i32: i32 = y_n as i32;
        let y_m_i32: i32 = y_m as i32;

        let dim_min: i32= m_i32.min(n_i32);
        let mut ipiv: Vec<i32> = vec![Zero::zero(); dim_min as usize];

        let mut info: i32 = 0;

        let mut self_data: Vec<T> = self.clone().data;
        let mut y_data: Vec<T> = y.clone().get_data().data;

        T::xgetrf(
            m_i32,
            n_i32,
            self_data.as_mut_slice(),
            m_i32,
            ipiv.as_mut_slice(),
            &mut info,
        );

        assert!(info >= 0);

        T::xgetrs(
            m_i32,
            1,
            self_data.as_mut_slice(),
            n_i32,
            ipiv.as_mut_slice(),
            y_data.as_mut_slice(),
            y_m_i32,
            &mut info
        );

        assert_eq!(info, 0);

        return Vector::new_column(y_m, y_data);
    }

    #[cfg(feature = "native")]
    pub fn solve_vector_r(self: &Self, y: &Vector<T>) -> Vector<T>
    {
        let (l, u, p): (Matrix<T>, Matrix<T>, Matrix<T>) = self.dec_lu();

        let b_hat: Vector<T> = p * y.clone();

        let y: Vector<T> = l.subst_forward_vector(b_hat);

        let x: Vector<T> = u.subst_backward_vector(y);

        return x;
    }

//    /// Solves Ax = Y
//    ///  where A \in R^{m * n}, x \in R^n, y \in R^{m, k}
//    ///
//    ///
//    pub fn solve_matrix(self: &Self, y: &Matrix<T>) -> Vector<T>
//    {
//        return self.solve_matrix_r(y);
//    }
//
//    #[cfg(feature = "blaslapack")]
//    pub fn solve_matrix_r(self: &Self, y: &Matrix<T>) -> Vector<T>
//    {
//        let (m, n): (usize, usize) = self.dim();
//        let m_i32: i32 = m as i32;
//        let n_i32: i32 = n as i32;
//
//        let (y_m, y_n): (usize, usize) = y.dim();
//        let y_n_i32: i32 = y_n as i32;
//
//        let dim_min: i32= m_i32.min(n_i32);
//        let mut ipiv: Vec<i32> = vec![Zero::zero(); dim_min as usize];
//
//        let mut info: i32 = 0;
//
//        let mut self_data: Vec<T> = self.clone().data;
//        let mut y_data: Vec<T> = y.clone().get_data().data;
//
//        T::xgetrf(
//            m_i32,
//            n_i32,
//            self_data.as_mut_slice(),
//            m_i32,
//            ipiv.as_mut_slice(),
//            &mut info,
//        );
//
//        assert!(info >= 0);
//
//        T::xgetrs(
//            n_i32,
//            1,
//            self_data.as_mut_slice(),
//            m_i32,
//            ipiv.as_mut_slice(),
//            y_data.as_mut_slice(),
//            y_n_i32,
//            &mut info
//        );
//
//        assert_eq!(info, 0);
//
//        return Vector::new_column(y_m, y_data);
//    }
//
//    #[cfg(feature = "native")]
//    pub fn solve_matrix_r(self: &Self, y: &Matrix<T>) -> Matrix<T>
//    {
//        let (l, u, p): (Matrix<T>, Matrix<T>, Matrix<T>) = self.dec_lu();
//
//        let b_hat: Matrix<T> = p * y.clone();
//
//        let y: Matrix<T> = l.subst_forward_matrix(b_hat);
//
//        let x: Matrix<T> = u.subst_backward_matrix(y);
//
//        return x;
//    }
}