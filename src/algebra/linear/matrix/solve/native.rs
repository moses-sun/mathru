use crate::algebra::linear::{Vector, Matrix};
use crate::algebra::abstr::{Field, Scalar};
use crate::algebra::linear::matrix::{Substitute};


pub trait Solve<T>
{
    /// A * x = b
    ///
    ///
    fn solve(self: &Self, rhs: &T) -> Option<T>;
}

impl<T> Solve<Vector<T>> for  Matrix<T>
    where T: Field + Scalar
{
    /// Solves Ax = y
    ///  where A \in R^{m * n}, x \in R^n, y \in R^m
    ///
    ///
    fn solve(self: &Self, rhs: &Vector<T>) -> Option<Vector<T>>
    {
        return self.solve_vector_r(rhs);
    }

}

impl<T> Solve<Matrix<T>> for Matrix<T>
    where T: Field + Scalar
{
    fn solve(self: &Self, rhs: &Matrix<T>) -> Option<Matrix<T>>
    {
        return self.solve_matrix_r(rhs);
    }
}

impl<T> Matrix<T>
    where T: Field + Scalar
{

    fn solve_vector_r(self: &Self, y: &Vector<T>) -> Option<Vector<T>>
    {
        let (l, u, p): (Matrix<T>, Matrix<T>, Matrix<T>) = self.dec_lu().lup();

        let b_hat: Vector<T> = &p * y;

        let y: Vector<T> = l.substitute_forward(b_hat);

        let x: Vector<T> = u.substitute_backward(y);

        return Some(x);
    }

}

impl<T> Matrix<T>
    where T: Field + Scalar
{
    #[cfg(feature = "native")]
    pub fn solve_matrix_r(self: &Self, y: &Matrix<T>) -> Option<Matrix<T>>
    {
        return self.dec_lu().solve(y);
    }
}
