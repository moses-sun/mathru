use crate::algebra::linear::{Vector, Matrix};
use super::Substitute;
use crate::algebra::abstr::{Field, Scalar};

impl<T> Substitute<Vector<T>> for Matrix<T> where T: Field + Scalar
{
    fn substitute_forward(self: &Self, a: Vector<T>) -> Vector<T>
    {
        let mut b: Vector<T> = a;
        for k in 0..self.n
        {
            for l in 0..k
            {
                *b.get_mut(k) = *b.get(k) - *self.get(k, l) * *b.get(l);
            }
            *b.get_mut(k) = *b.get(k) / *self.get(k, k);
        }

        return b;
    }

    fn substitute_backward(self: &Self, c: Vector<T>) -> Vector<T>
    {
        let mut b: Vector<T> = c;

        for k in (0..self.n).rev()
        {
            for l in (k + 1..self.n).rev()
            {
                *b.get_mut(k) = *b.get(k) - *self.get(k, l) * *b.get(l);
            }
            if *self.get(k, k) != T::zero()
            {
                *b.get_mut(k) = *b.get(k) / *self.get(k, k);
            }
            else
            {
                *b.get_mut(k) = T::one();
            }
        }

        return b;
    }
}

impl<T> Substitute<Matrix<T>> for Matrix<T> where T: Field + Scalar
{
    fn substitute_forward(self: &Self, a: Matrix<T>) -> Matrix<T>
    {
        let mut b: Matrix<T> = a;
        let min: usize = std::cmp::min( self.m, self.n);
        for k in 0..min
        {
            for l in 0..k
            {
                b.set_row( & (b.get_row(k) - (b.get_row(l) * * self.get(k, l))), k);
            }
            b.set_row( & (b.get_row(k) / * self.get(k, k)), k);
        }
        return b;
    }

    fn substitute_backward(self: &Self, a: Matrix<T>) -> Matrix<T>
    {
        let mut b: Matrix<T> = a;
        let min = std::cmp::min(self.m, self.n);

        for k in (0..min).rev()
        {
            for l in (k + 1..min).rev()
            {
                b.set_row(&(b.get_row(k) - (b.get_row(l) * *self.get(k, l))), k);
            }
            b.set_row(&(b.get_row(k) / *self.get(k, k)), k);
        }

        return b;
    }
}