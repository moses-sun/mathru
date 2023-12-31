use crate::algebra::abstr::{Field, Scalar};
use crate::algebra::linear::{matrix::General, vector::vector::Vector};

pub struct MatrixColumnIntoIterator<'a, T> {
    m: &'a General<T>,
    column: usize,
}

impl<'a, T> MatrixColumnIntoIterator<'a, T> {
    pub fn new(m: &'a General<T>) -> MatrixColumnIntoIterator<'a, T> {
        MatrixColumnIntoIterator { m, column: 0 }
    }
}

impl<'a, T> Iterator for MatrixColumnIntoIterator<'a, T>
where
    T: Field + Scalar,
{
    type Item = Vector<T>;

    // just return the str reference
    fn next(&mut self) -> Option<Self::Item> {
        if self.column < self.m.ncols() {
            let column: Vector<T> = self.m.get_column(self.column);
            self.column += 1;

            Some(column)
        } else {
            None
        }
    }
}
