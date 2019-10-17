use crate::algebra::linear::{Matrix};
use crate::algebra::abstr::{Real};

#[cfg(feature = "blaslapack")]
use crate::algebra::abstr::{Zero};

impl<T> Matrix<T>
    where T: Real
{
    /// QR Decomposition with Givens rotations
    ///
    /// A = QR \
    /// Q is an orthogonal matrix \
    /// R is an upper triangular matrix \
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, -2.0, 3.0, -7.0]);
    ///
    /// let (q, r): (Matrix<f64>, Matrix<f64>) = a.dec_qr();
    ///
    /// ```
    pub fn dec_qr<'a>(self: &'a Self) -> (Matrix<T>, Matrix<T>)
    {
        self.dec_qr_r()
    }

    #[cfg(feature = "native")]
    fn dec_qr_r<'a>(self: &'a Self) -> (Matrix<T>, Matrix<T>)
    {
        let mut q: Matrix<T> = Matrix::one(self.m);
        let mut r: Matrix<T> = self.clone();

        for j in 0..self.n
        {
            for i in (j + 1..self.m).rev()
            {
                let a_jj: T = r.get(j, j).clone();
                let a_ij: T = r.get(i, j).clone();
                //let k: T = a_jj.sgn();
                let p: T = (a_jj.clone() * a_jj.clone() + a_ij.clone() * a_ij.clone()).pow(&T::from_f64
                (0.5).unwrap());
                if (p != T::zero()) && (a_jj != T::zero()) && (a_ij != T::zero())
                {
                    let c : T = a_jj / p.clone();
                    let s : T = -a_ij / p;
                    let g_ij: Matrix<T> = Matrix::givens(r.m, i, j, c, s);

                    r = &g_ij * &r;
                    q = &g_ij * &q;
                }
            }
        }
        q = q.transpose();
        (q, r)
    }

    #[cfg(feature = "blaslapack")]
    fn dec_qr_r<'a>(self: &'a Self) -> (Matrix<T>, Matrix<T>)
    {
        let (m, n) : (usize, usize) = self.dim();

        //lapack(fortran) uses column major order
        let mut self_data = self.clone().data;

        let m_i32: i32 = m as i32;
        let n_i32: i32 = n as i32;
        let m_n_min: usize = m.min(n);

        let mut tau: Vec<T> = vec![Zero::zero(); m_n_min];

        let mut info: i32 = 0;

        let lwork: i32 = T::xgeqrf_work_size(m_i32, n_i32, & mut self_data[..], m_i32, &mut tau[..], &mut info);

        assert_eq!(0, info);

       	let mut work: Vec<T> = vec![T::zero(); lwork as usize];

        T::xgeqrf(
            m_i32,
            n_i32,
            &mut self_data[..],
            m_i32,
            tau.as_mut(),
            &mut work,
            lwork,
            &mut info,
        );

        assert_eq!(0, info);
        let a: Matrix<T> = Matrix::new(m, n, self_data.clone());
        let r: Matrix<T> = a.r();

        let lwork = T::xorgqr_work_size(
            m_i32,
            m_n_min as i32,
            tau.len() as i32,
            &mut self_data[..],
            m_i32,
            &mut tau[..],
            &mut info,
        );

        assert_eq!(0, info);

        let mut work = vec![T::zero(); lwork as usize];

        T::xorgqr(
            m_i32,
            m_n_min as i32,
            tau.len() as i32,
            &mut self_data[..],
            m_i32,
            &mut tau[..],
            &mut work,
            lwork,
            &mut info,
        );
        assert_eq!(0, info);

        let q: Matrix<T> = Matrix::new(m, n, self_data);

        return (q, r);
    }

    #[cfg(feature = "blaslapack")]
    fn r(mut self: Self) -> Self
    {
        for i in 1..self.m
        {
            for k in 0..(i.min(self.n))
            {
                *self.get_mut(i, k) = T::zero();
            }
        }

        self
    }
}