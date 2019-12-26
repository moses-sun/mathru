use crate::algebra::linear::{Vector, Matrix};
use crate::algebra::abstr::{Field, Scalar};
use crate::elementary::Power;

impl<T> Matrix<T>
     where T: Field + Scalar + Power
{
    /// Computes the eigenvalues of a real matrix
    ///
    ///
    /// # Arguments
    ///
    ///
    ///
    /// # Return
    ///
    /// Vector with unsorted eigenvalues
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Vector, Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(3, 3, vec![1.0, -3.0, 3.0, 3.0, -5.0, 3.0, 6.0, -6.0, 4.0]);
    /// let eig: Vector<f64> = a.eigenvalue();
    ///
    /// ```
    pub fn eigenvalue(self: &Self) -> Vector<T>
    {
        let (m, n) : (usize, usize) = self.dim();
        assert!(m == n, "Unable to compute the eigen value of a non-square matrix");
        assert!(m != 0, "Unable to compute the eigen vlaue of an empty matrix.");
        self.eigenvalue_r()
    }

    #[cfg(feature = "native")]
    pub fn eigenvalue_r(self: &Self) -> Vector<T>
    {
        let (m, _n): (usize, usize) = self.dim();

        let h: Matrix<T> = self.dec_hessenberg().h();

        let (_u, t): (Matrix<T>, Matrix<T>) = h.clone().francis();

        let mut eig: Vector<T> = Vector::zero(m);

        for i in 0..m
        {
            *eig.get_mut(i) = t.get(i, i).clone();
        }

        return eig;
    }

    #[cfg(feature = "native")]
    fn francis(mut self: Self) -> (Matrix<T>, Matrix<T>)
    {
        let epsilon: T = T::epsilon();

        let (m, n): (usize, usize) = self.dim();

        let mut u: Matrix<T> = Matrix::one(m);

        let mut p: usize = n;
        let mut q: usize;

        while p > 2
        {
            q = p - 1;

            // Bulge generating
            let s: T = *self.get(q - 1,q - 1) + *self.get(p - 1,p -1);
            let t: T = *self.get(q - 1,q - 1) * *self.get(p - 1,p - 1) - *self.get(q - 1,p - 1) * *self
            .get(p - 1,q - 1);

            // compute first 3 elements of first column of M
            let mut x: T = self.get(0,0).pow(&T::from_f64(2.0).unwrap()) + *self.get(0,1) * *self.get(1,0) - s *
            *self.get(0,0) + t;
            let mut y: T = *self.get(1,0) * (*self.get(0,0) + *self.get(1, 1) - s);
            let mut z: T = *self.get(1,0) * *self.get(2,1);

            for k in 0..(p - 2)
            {
                let b: Vector<T> = Vector::new_column(3, vec![x, y, z]);
                let h: Matrix<T> = Matrix::householder(&b, 0);

                //Determine the Householder reflector P with P [x; y; z] = αe1 ;
                {
                    let r: usize = k.max(1);

                    let temp = &h * &self.get_slice(k, k + 2,r - 1, n - 1);
                    self = self.set_slice(&temp, k, r - 1);

                }

                {
                    let h_trans: Matrix<T> = h.transpose();
                    let r: usize = p.min(k + 4);
                    let temp: Matrix<T> = &self.get_slice(0, r - 1, k, k + 2) * &h_trans;
                    self = self.set_slice(&temp, 0, k);

                    let temp1: Matrix<T> = &u.get_slice(0, n-1, k, k + 2) * &h_trans;

                    u = u.set_slice(&temp1, 0, k);
                }

                x  = *self.get(k + 1, k);
                y = *self.get(k + 2, k);
                if k < (p - 3)
                {
                    z = *self.get(k + 3, k);
                }
            }

            // Determine the Givens rotation P with P [x; y]T = αe1 ;
            let (c, s): (T, T) = Matrix::givens_cosine_sine_pair(x, y);
            let g: Matrix<T> = Matrix::givens(2, 0, 1, c, s);

            {
                let temp: Matrix<T> = &g * &self.get_slice(q - 1, p - 1, p - 3, n - 1);
                self = self.set_slice(&temp, q - 1, p - 3);
            }

            {
                let g_trans: Matrix<T> = g.transpose();
                let temp: Matrix<T> = &self.get_slice(0, p - 1, p - 2, p - 1) * &g_trans;
                self = self.set_slice(&temp, 0, p - 2);

                let u_slice = &self.get_slice(0, n - 1, p - 2, p - 1) * & g_trans;
                u = u.set_slice(&u_slice, 0, p - 2);
            }

            // check for convergence
            let m: T = self.get(q - 1,q - 1).abs();
            let n: T = self.get(p - 1,p - 1).abs();
            if self.get(p - 1 ,q - 1).abs() < epsilon.clone() * (m + n)
            {
                *self.get_mut(p - 1, q - 1) = T::zero();
                p = p - 1;
            }
            else
            {
                let k: T = self.get(q - 2,q - 2).abs();
                let l: T = self.get(q - 1,q - 1).abs();
                if self.get(p - 2, q - 2).abs() < epsilon.clone() * (k + l)
                {
                    *self.get_mut(p - 2, q - 2) = T::zero();
                    p = p - 2;
                }
            }
            p = p -1;
        }

        return (u, self);
    }

    #[cfg(feature = "blaslapack")]
    pub fn eigenvalue_r(self: &Self) -> Vector<T>
    {
        let (m, n) : (usize, usize) = self.dim();

        let mut self_data: Vec<T> = self.clone().transpose().data;
        let n_i32: i32 = n as i32;

        let mut info: i32 = 0;

        let mut wr: Vec<T> = vec![T::zero(); n];
        let mut wi: Vec<T> = vec![T::zero(); n];

        let mut temp1 = [T::zero()];
        let mut temp2 = [T::zero()];

        let lwork = T::xgeev_work_size(
            'N' as u8,
            'N' as u8,
            n_i32,
            &mut self_data[..],
            n_i32,
            wr.as_mut_slice(),
            wi.as_mut_slice(),
            &mut temp1,
            n_i32,
            &mut temp2,
            n_i32,
            &mut info,
        );

        let mut work: Vec<T> = vec![T::zero(); lwork as usize];

        T::xgeev(
            'N' as u8,
            'N' as u8,
                    n_i32,
                    &mut self_data[..],
                    n_i32,
                    wr.as_mut_slice(),
                    wi.as_mut_slice(),
                    &mut temp1,
                    1 as i32,
                    &mut temp2,
                    1 as i32,
                    &mut work,
                    lwork,
                    &mut info,
        );

        assert_eq!(0, info);

        return Vector::new_column(m, wr);
    }
}