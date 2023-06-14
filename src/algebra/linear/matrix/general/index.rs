use crate::algebra::linear::matrix::General;
use std::ops::{Index, IndexMut};

impl<T> Index<[usize; 2]> for General<T> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        assert!(index[0] < self.m);
        assert!(index[1] < self.n);

        &self.data[index[1] * self.m + index[0]]
    }
}

impl<T> IndexMut<[usize; 2]> for General<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        assert!(index[0] < self.m);
        assert!(index[1] < self.n);

        &mut self.data[index[1] * self.m + index[0]]
    }
}
