use super::matrix::Matrix;
use super::vector::Vector;
use crate::algebra::abstr::{Zero, One};


pub(crate) trait Allocator
{
  	///
    /// Fortran like, column wise
	fn new_matrix<T>(m: usize, n: usize, data: Vec<T>) -> Matrix<T>
	where T: Clone + Copy + Zero + One;

	fn new_vector_row<T>(n: usize, data: Vec<T>) -> Vector<T>
	where T: Clone + Copy + Zero + One;

	fn new_vector_column<T>(m: usize, data: Vec<T>) -> Vector<T>
	where T: Clone + Copy + Zero + One;
}