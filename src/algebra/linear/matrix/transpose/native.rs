use crate::algebra::{
    abstr::{Field, Scalar},
    linear::Matrix,
};

use super::Transpose;

impl<T> Transpose for Matrix<T>
    where T: Field + Scalar
{
    type Output = Matrix<T>;
    /// Function to transpose a matrix without allocating memory for the
    /// transposed matrix
    ///
    /// catanzaro.name/papers/PPoPP-2014.pdf
    /// TODO
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Matrix;
    /// use mathru::algebra::linear::matrix::Transpose;
    ///
    /// let mut uut: Matrix<f64> = Matrix::new(4, 2, vec![1.0, 0.0, 3.0, 0.0, 1.0, -7.0, 0.5, 0.25]);
    /// uut = uut.transpose();
    ///
    /// let refer: Matrix<f64> = Matrix::new(2, 4, vec![1.0, 1.0, 0.0, -7.0, 3.0, 0.5, 0.0, 0.25]);
    /// assert_eq!(refer, uut);
    /// ```
    fn transpose(self: Self) -> Matrix<T>
    {
        let (m, n): (usize, usize) = self.dim();
        let mut matrix_t: Matrix<T> = Matrix::zero(n, m);

        for i in 0..m
        {
            for j in 0..n
            {
                *matrix_t.get_mut(j, i) = *self.get(i, j);
            }
        }

        return matrix_t;
    }
}
// impl<T> Matrix<T> where T: Field + Scalar
// {
//     pub fn gcd(mut m: usize, mut n: usize) -> usize
//     {
//         while m != 0
//         {
//             let old_m: usize = m;
//             m = n % m;
//             n = old_m;
//         }
//         n
//     }
//     /// Function to transpose a matrix without allocating memory for the
//     /// transposed matrix
//     ///
//     /// catanzaro.name/papers/PPoPP-2014.pdf
//     /// TODO
//     /// # Example
//     ///
//     /// ```
//     /// use mathru::algebra::linear::Matrix;
//     ///
//     /// let mut uut: Matrix<f64> = Matrix::new(4, 2, vec![1.0, 0.0, 3.0, 0.0, 1.0, -7.0, 0.5, 0.25]);
//     /// uut = uut.transpose();
//     ///
//     /// let refer: Matrix<f64> = Matrix::new(2, 4, vec![1.0, 1.0, 0.0, -7.0, 3.0, 0.5, 0.0, 0.25]);
//     /// assert_eq!(refer, uut);
//     /// ```
//     pub fn transpose(self: Self) -> Matrix<T>
//     {
//         let (m, n) = self.dim();
//         let mut matrix_t: Matrix<T> = Matrix::zero(n, m);
//
//         for i in 0..m
//         {
//             for j in 0..n
//             {
//                 *matrix_t.get_mut(j, i) = *self.get(i, j);
//             }
//         }
//
//         return matrix_t;
//     }
// //    pub fn transpose_r(mut self: Self) -> Matrix<T>
// //    {
// //        let gcdiv: usize = Matrix::<T>::gcd(self.m, self.n);
// //
// //        let a: usize = self.m / gcdiv;
// //        let b: usize = self.n / gcdiv;
// //
// //        let bigger: usize = self.m.max(self.n);
// //
// //        let mut temp : Vec<T> = Vec::with_capacity(bigger);
// //        //Bad
// //        unsafe {temp.set_len(bigger)};
// //
// //        if gcdiv > 1
// //        {
// //            for j in 0..self.n
// //            {
// //                for i in 0..self.m
// //                {
// //                    temp[i]  = *self.get(self.gather(i, j, b), j)
// //                    //temp[i] = self.data[self.gather(i, j, b) * self.n + j];
// //                }
// //
// //                for i in 0..self.m
// //                {
// //                    *self.get_mut(i, j) = temp[i];
// //                    //self.data[i * self.n + j] = temp[i];
// //                }
// //            }
// //        }
// //
// //        for i in 0..self.m
// //        {
// //            for j in 0..self.n
// //            {
// //                temp[self.scatter_da(i, j, b)] = *self.get(i, j);
// //                //temp[self.scatter_da(i, j, b)] = self.data[i * self.n + j];
// //            }
// //
// //            for j in 0..self.n
// //            {
// //                *self.get_mut(i, j) = temp[j];
// //                //self.data[i * self.n + j] = temp[j];
// //            }
// //        }
// //
// //        for j in 0..self.n
// //        {
// //            for i in 0..self.m
// //            {
// //                temp[i] = *self.get(self.gather_sa(i, j, a), j);
// //                //temp[i] =  self.data[self.gather_sa(i, j, a) * self.n + j];
// //            }
// //
// //            for i in 0..self.m
// //            {
// //                *self.get_mut(i, j) = temp[i];
// //                //self.data[i * self.n + j] = temp[i];
// //            }
// //        }
// //
// //        let temp_m : usize = self.m;
// //        self.m = self.n;
// //        self.n = temp_m;
// //
// //        return self;
// //    }
// //
// //    fn gather<'a>(self: &'a Self, i: usize, j: usize, b: usize) -> usize
// //    {
// //        return (i + (j / b)) % self.m;
// //    }
// //
// //    fn gather_sa<'a>(self: &'a Self, i: usize, j: usize, a: usize) -> usize
// //    {
// //        return (j + i * self.n - i / a) % self.m;
// //    }
// //
// //    fn scatter_da<'a>(self: &'a Self, i: usize, j: usize, b: usize) -> usize
// //    {
// //        return (((i + j / b) % self.m) + j * self.m) % self.n;
// //    }
// }