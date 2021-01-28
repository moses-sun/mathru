use crate::algebra::{
    abstr::{Field, Scalar},
    linear::{matrix::Substitute, Matrix, Vector},
};

use super::Solve;

impl<T> Solve<Vector<T>> for Matrix<T> where T: Field + Scalar
{
    /// Solves Ax = y
    ///  where A \in R^{m * n}, x \in R^n, y \in R^m
    fn solve(self: &Self, rhs: &Vector<T>) -> Result<Vector<T>, ()>
    {
        let (l, u, p): (Matrix<T>, Matrix<T>, Matrix<T>) = self.dec_lu().unwrap().lup();

        let b_hat: Vector<T> = &p * rhs;

        let y: Vector<T> = l.substitute_forward(b_hat);

        let x: Vector<T> = u.substitute_backward(y);

        return Ok(x);
    }
}

impl<T> Solve<Matrix<T>> for Matrix<T> where T: Field + Scalar
{
    fn solve(self: &Self, rhs: &Matrix<T>) -> Result<Matrix<T>, ()>
    {
        return self.dec_lu()?.solve(rhs);
    }
}
