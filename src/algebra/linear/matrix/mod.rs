#[macro_use]
pub mod matrix;
mod matrixintoiterator;
mod matrixiterator;
mod matrixiteratormut;
mod matrixrowiterator;
mod matrixrowiteratormut;
mod matrixcolumniterator;
mod matrixcolumniteratormut;

mod eigen;
pub use self::eigen::EigenDec;

mod hessenberg;
pub use self::hessenberg::HessenbergDec;

mod lu;
pub use self::lu::LUDec;

mod qr;
pub use self::qr::QRDec;

mod inverse;
mod mul;
mod add;
mod sub;
mod div;

mod cholesky;
pub use self::cholesky::CholeskyDec;


mod solve;
mod substitute;


pub use self::substitute::Substitute;
pub use self::solve::Solve;
pub use self::inverse::Inverse;
pub use self::matrix::Matrix;
pub use self::matrixintoiterator::MatrixIntoIterator;
pub use self::matrixiterator::MatrixIterator;
pub use self::matrixiteratormut::MatrixIteratorMut;
pub use self::matrixrowiterator::MatrixRowIterator;
pub use self::matrixrowiteratormut::MatrixRowIteratorMut;
pub use self::matrixcolumniterator::MatrixColumnIterator;
pub use self::matrixcolumniteratormut::MatrixColumnIteratorMut;