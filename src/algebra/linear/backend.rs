use super::allocator::Allocator;
use super::{Vector, Matrix};
use crate::algebra::abstr::{Zero, One};


struct Native
{

}




struct Lapack
{

}

//impl Allocator for Native
//{
//  	///
//    /// Fortran like, column wise
//	fn new_matrix<T>(m: usize, n: usize, data: Vec<T>) -> Matrix<T>
//		where T: Clone + Copy + Zero + One
//	{
//		assert_eq!(m * n, data.len());
//        Matrix{m: m, n: n, data: data}
//	}
//
//	fn new_vector_row<T>(n: usize, data: Vec<T>) -> Vector<T>
//		where T: Clone + Copy + Zero + One
//	{
//        assert_eq!(n, data.len());
//        Vector
//        {
//            data: Matrix::new(1, n, data)
//        }
//	}
//
//	fn new_vector_column<T>(m: usize, data: Vec<T>) -> Vector<T>
//		where T: Clone + Copy + Zero + One
//	{
//        assert_eq!(m, data.len());
//        Vector
//        {
//            data: Matrix::new(m, 1, data)
//        }
//	}
//}


/*pub fn native() -> Native
{

}*/

