use algebra::linear::{Vector, Matrix};
use algebra::abstr::Zero;
use algebra::abstr::Real;

impl<T> Matrix<T>
    where T: Real
{
    /// Decomposes the matrix into a upper and a lower matrix
    ///
    /// PA = LU
    ///
    /// # Arguments
    ///
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, -2.0, 3.0, -7.0]);
    ///
    /// let (l, u, p): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu();
    ///
    /// ```
    pub fn dec_lu<'a>(self: &'a Self) -> (Matrix<T>, Matrix<T>, Matrix<T>)
    {
        let (m, n): (usize, usize) = self.dim();
        assert_eq!(m, n);
        self.dec_lu_r()
    }

    #[cfg(feature = "native")]
    fn dec_lu_r<'a>(self: &'a Self) -> (Matrix<T>, Matrix<T>, Matrix<T>)
    {
        let (m, n): (usize, usize) = self.dim();

        let mut l: Matrix<T> = Matrix::one(self.m);
        let mut u: Matrix<T> = Matrix::one(self.n);
        let mut p: Matrix<T> = Matrix::one(self.m);

        let mut a: Matrix<T> = self.clone();

        for i in 0..a.m
        {
            //pivoting
            let mut max: T = a.get(&i, &i).clone();
            let mut i_max: usize = i;

            for l in i + 1..a.m
            {
                if *a.get(&l, &i) > max
                {
                    max = a.get(&l, &i).clone();
                    i_max = l;
                }
            }

            if i != i_max
            {
                a.swap_rows(&i, &i_max);
                p.swap_rows(&i, &i_max);
            }


            for j in (i + 1)..a.n
            {
                let f: T;
                if a.get(&i, &i).clone() != T::zero()
                {
                    f = (*(a.get(&(j), &i))).clone() / a.get(&i, &i).clone();
                }
                else
                {
                    f = (*(a.get(&(j), &i))).clone();
                }

                for k in (i + 1)..a.n
                {
                    *(a.get_mut(&(j), &k)) =  a.get(&(j), &k).clone() - f.clone() * a.get(&i, &k).clone();
                }
                *(a.get_mut(&(j), &i)) = f.clone();
            }
        }

        for i in 1..a.n
        {
            for j in 0..i
            {
                l.data[i * a.n + j] = a.data[i * a.n + j].clone();
            }
        }

        for i in 0..a.n
        {
            for k in i..a.n
            {
                u.data[i * a.n + k] = a.data[i * a.n + k].clone();
            }
        }

        (l, u, p)
    }

    #[cfg(feature = "lapack_be")]
    fn dec_lu_r<'a>(self: &'a Self) -> (Matrix<T>, Matrix<T>, Matrix<T>)
    {

        let mut p: Matrix<T> = Matrix::one(self.m);

        let (m, n): (usize, usize) = self.dim();
        let m_i32: i32 = m as i32;
        let n_i32: i32 = n as i32;

        let dim_min: i32= m_i32.min(n_i32);
        let mut ipiv: Vec<i32> = vec![Zero::zero(); dim_min as usize];

        let mut info: i32 = 0;

        let mut self_data = self.transpose().data;

        T::xgetrf(
            m_i32,
            n_i32,
            self_data.as_mut_slice(),
            m_i32,
            ipiv.as_mut_slice(),
            &mut info,
        );

        assert_eq!(0, info);

        let mat: Matrix<T> = Matrix::new(m, n, self_data).transpose_inplace();
        let l: Matrix<T> = Matrix::l(&mat, m, n);
        let u: Matrix<T> = Matrix::u(&mat, m, n);
        let p: Matrix<T> = Matrix::p(ipiv);

        (l, u, p)
    }

    #[cfg(feature = "lapack_be")]
    fn l(mat: &Matrix<T>, m: usize, n: usize) -> Self
    {
        let mut res: Matrix<T> = mat.clone();

        //fill upper triangle with zero
        for i in 0..m
        {
            for k in i..n
            {
                *res.get_mut(&i, &k) = T::zero();
            }
        }

        //set diagonal to 1
        for i in 0..m
        {
             *res.get_mut(&i, &i) = T::one();
        }

        res
    }

    #[cfg(feature = "lapack_be")]
    fn u(mat: &Matrix<T>, m: usize, n: usize) -> Self
    {
        let mut res: Matrix<T> = mat.clone();

        //fill lower triangle with zero
        for i in 0..m
        {
            for k in 0..i
            {
                *res.get_mut(&i, &k) = T::zero();
            }
        }

        res
    }

    #[cfg(feature = "lapack_be")]
    /// transforms a sequence of permutations to a permutation matrix
    fn p(per: Vec<i32>) -> Self
    {
        let length = per.len();

        let mut perm: Vec<usize> = vec![0; length];

        for i in 0..length
        {
            perm[i] = i;
        }

        for i in 0..length-1
        {
            let temp = perm[(per[i] -1) as usize];
            perm[(per[i] -1) as usize] = perm[i];
            perm[i] = temp;
        }

        let mut p: Matrix<T> = Matrix::zero(length, length);

        for i in 0..length
        {
            let k = perm[i];
            *p.get_mut(&i, &k) = T::one();
        }

        p
    }

}