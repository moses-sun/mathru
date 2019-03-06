use std::clone::Clone;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, Neg, Div};
use algebra::abstr::{Semiring, Zero, One, Sign};
use algebra::linear::Vector;
use elementary::{Exponential, Power};
use algebra::abstr::cast::FromPrimitive;
use algebra::abstr::Real;
use std::fmt::Display;
use std::fmt;

/// A matrix is described here
#[derive(Debug, Clone)]
pub struct Matrix<T>
{
    /// Num of rows which the matrix has
    m: usize,
    /// Num of columns which the matrix ha
    n: usize,
    /// Matrix entries
    data:  Vec<T>
}


impl <T> Matrix<T>
    where T: Semiring + Sign
{

    fn gather<'a>(self: &'a Self, i: usize, j: usize, b: usize) -> usize
    {
        (i + (j / b)) % (self.m)
    }

    fn gather_sa<'a>(self: &'a Self, i: usize, j: usize, a: usize) -> usize
    {
        (j + i * self.n - i / a) % self.m
    }

    fn scatter_da<'a>(self: &'a Self, i: usize, j: usize, b: usize) -> usize
    {
        (((i + j / b)) % (self.m) + j * self.m) % (self.n)
    }
}

impl<T> Matrix<T>
    where T: Clone + Zero
{
    /// Returns the transposed matrix
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let mut a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, -2.0, 3.0, -7.0]);
    /// a = a.transpose();
    ///
    /// let a_trans: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 3.0, -2.0, -7.0]);
    ///
    /// assert_eq!(a_trans, a);
    /// ```
    pub fn transpose(self: &Self) -> Self
    {
        let mut trans : Matrix<T> = Matrix::zero(&self.n, &self.m);

        for j in 0..self.n
        {
            for i in 0..self.m
            {
                *trans.get_mut(&j, &i) = self.get(&i, &j).clone()
            }
        }

        trans
    }
}

impl<T> Matrix<T>
    where T: Semiring + Sign
{

    fn gcd(mut m: usize, mut n: usize) -> usize
    {
        while m != 0
        {
            let old_m: usize = m;
            m = n % m;
            n = old_m;
        }
        //n.abs()
        n
    }

    /// Function to transpose a matrix without allocating memory for the transposed matrix
    ///
    /// catanzaro.name/papers/PPoPP-2014.pdf
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Matrix};
	///
	/// let mut uut: Matrix<f64> = Matrix::new(&4, &2, &vec![1.0, 0.0, 3.0, 0.0, 1.0, -7.0, 0.5, 0.25]);
    /// uut = uut.transpose_inplace();
    ///
    /// let refer: Matrix<f64> = Matrix::new(&2, &4, &vec![1.0, 3.0, 1.0, 0.5, 0.0, 0.0, -7.0, 0.25]);
    /// assert_eq!(refer, uut);
    /// ```
    pub fn transpose_inplace<'a>(mut self: Self) -> Matrix<T>
    {

        let gcdiv: usize = Matrix::<T>::gcd(self.m, self.n);

        let a: usize = self.m / gcdiv;
        let b: usize = self.n / gcdiv;

        let bigger: usize = self.m.max(self.n);

        let mut temp : Vec<T> = Vec::with_capacity(bigger);
        unsafe {temp.set_len(bigger)};

        if gcdiv > 1
        {
            for j in 0..self.n
            {
                for i in 0..self.m
                {
                    temp[i] = self.data[self.gather(i, j, b) * self.n + j];
                }

                for i in 0..self.m
                {
                    self.data[i * self.n + j] = temp[i];
                }
            }
        }

        for i in 0..self.m
        {
            for j in 0..self.n
            {
                temp[self.scatter_da(i, j, b)] = self.data[i * self.n + j];
            }

            for j in 0..self.n
            {
                self.data[i * self.n + j] = temp[j];
            }
        }

        for j in 0..self.n
        {
            for i in 0..self.m
            {
                temp[i] =  self.data[self.gather_sa(i, j, a) * self.n + j];
            }

            for i in 0..self.m
            {
                self.data[i * self.n + j] = temp[i];
            }
        }

        let temp_m : usize = self.m;
        self.m = self.n;
        self.n = temp_m;

        return self;
    }

}
//
//impl<T> Matrix<T>
//{
//    /*
//     * inplace backward substitution
//     */
//    fn subst_backward<'a>(self: &'a mut Self)
//        where T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + One + Zero + AddAssign
//
//    {
//        for k in (0..self.n.get_primitive()).rev()
//        {
//            for l in (k..self.n.get_primitive()).rev()
//            {
//
//                let mut sum : T = T::zero();
//
//                for i in (k+1)..self.n.get_primitive()
//                {
//                    sum += self.data[k * self.n.get_primitive() + i].clone() * self.data[i * self.n.get_primitive() +
//                        l].clone();
//                }
//
//                let b : T;
//                if k == l
//                {
//                    b = T::one();
//                }
//                else
//                {
//                    b = T::zero();
//                }
//                let div : T = self.data[k * self.n.get_primitive() + k].clone();
//                self.data[k * self.n.get_primitive() + l] = (b - sum) / div;
//
//            }
//        }
//    }
//}
//
//impl<T> Matrix<T>
//{
//    /**
//     * inplace forward substitution
//     */
//    fn subst_forward<'a>(self: &'a mut Self)
//        where T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + One + Zero + AddAssign
//    {
//
//        for k in Interval::range(Natural::zero(), self.n)
//        {
//            for l in Interval::range(Natural::zero(), k)
//            {
//
//                let mut sum : T = T::zero();
//
//                for i in Interval::range(Natural::zero(), k)
//                {
//                    sum += self.get(&k, &i).clone() * self.get(&i, &self.n).clone();
//                }
//
//                let b : T;
//                if k == l
//                {
//                    b = T::one();
//
//                }
//                else
//                {
//                    b = T::zero();
//                }
//                let div: T = self.get(&k, &k).clone();
//                *self.get_mut(&k, &l) =  (b - sum) / div;
//            }
//        }
//
//    }
//}


impl<T> Matrix<T>
{

    /// Calculates the determinant
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, -2.0, 3.0, -7.0]);
    /// let determinant_a: f64 = a.det();
    ///
    /// assert_eq!(-1.0, determinant_a);
    /// ```
    pub fn det<'a>(self: &'a Self) -> T
        where T: MulAssign + Add<Output = T> + Sub<Output = T> +  Mul<Output = T>  + Div<Output = T> + One + Zero  +
        Clone + PartialOrd + Neg<Output = T>
    {
        assert_eq!(self.m, self.n);

        if self.m == 1
        {
            return self.get(&0, &0).clone();
        }

        if self.m == 2
        {
            let a_11: T = self.get(&0, &0).clone();
            let a_12: T = self.get(&0, &1).clone();
            let a_21: T = self.get(&1, &0).clone();
            let a_22: T = self.get(&1, &1).clone();
            return a_11 * a_22 - a_12 * a_21;
        }

        let (_l, u, p): (Matrix<T>, Matrix<T>, Matrix<T>) = self.dec_lu();
        let mut det: T = T::one();

        for i in 0..self.m
        {
			det *= u.get(&i, &i).clone();
        }

        let mut perm: T = -T::one();
        for i in 0..self.m
        {
            if *p.get(&i, &i) != T::one()
            {
                perm *= -T::one();
            }
        }

        (perm) * det
    }
}

impl<T> Matrix<T>
    where T:  Add<Output = T> + Sub<Output = T> +  Mul<Output = T>  + Div<Output = T> + One + Zero + Clone +
    PartialOrd
{
    /// Decomposes the matrix into a upper and a lower matrix
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, -2.0, 3.0, -7.0]);
    /// let l_ref: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 1.0 / 3.0, 1.0]);
    ///
    /// let (l, u, p): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_lu();
    ///
    /// assert_eq!(l_ref, l);
    /// ```
    pub fn dec_lu<'a>(self: &'a Self) -> (Matrix<T>, Matrix<T>, Matrix<T>)
    {
        assert_eq!(self.m, self.n);

        let mut l: Matrix<T> = Matrix::one(&self.m);
        let mut u: Matrix<T> = Matrix::one(&self.n);
        let mut p: Matrix<T> = Matrix::one(&self.m);

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
}

impl<T> Matrix<T>
    where T: Neg<Output = T> + Div<T, Output = T>  + Mul<T, Output = T> + Add<T, Output = T> + Power + Clone +
PartialOrd + One + Zero + Sign + AddAssign + FromPrimitive + Display
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
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, -2.0, 3.0, -7.0]);
    /// let q_ref: Matrix<f64> = Matrix::new(&2, &2, &vec![0.31622776601683794, -0.9486832980505138, 0.9486832980505138, 0.31622776601683794]);
    ///
    /// let (q, r): (Matrix<f64>, Matrix<f64>) = a.dec_qr();
    ///
    /// assert_eq!(q_ref, q);
    /// ```
    pub fn dec_qr<'a>(self: &'a Self) -> (Matrix<T>, Matrix<T>)
    {
        if self.m < self.n
        {
            panic!();
        }
        let mut q: Matrix<T> = Matrix::one(&self.m);
        let mut r: Matrix<T> = self.clone();

        for j in 0..self.n
        {
            for i in (j + 1..self.m).rev()
            {
                let a_jj: T = r.get(&j, &j).clone();
                let a_ij: T = r.get(&i, &j).clone();
                //let k: T = a_jj.sgn();
                let p: T = (a_jj.clone() * a_jj.clone() + a_ij.clone() * a_ij.clone()).pow(&T::from_f64
                (0.5).unwrap());
                if (p != T::zero()) && (a_jj != T::zero()) && (a_ij != T::zero())
                {
                    let c : T = a_jj / p.clone();
                    let s : T = -a_ij / p;
                    let g_ij: Matrix<T> = Matrix::givens(&r.m, &i, &j, &c, &s);

                    r = &g_ij * &r;
                    q = &g_ij * &q;
                }
            }
        }
        q = q.transpose();
        (q, r)
    }
}

impl<T> Matrix<T>
    where T: Clone
{
    fn swap_rows<'a, 'b, 'c>(self: &'a mut Self, i: &'b usize, j: &'c usize)
    {
        for k in 0..self.n
        {
            let temp: T = self.get(&i, &k).clone();
            *(self.get_mut(&i, &k)) = self.get(&j, &k).clone();
            *(self.get_mut(&j, &k)) = temp;
        }
    }
}




impl<T> Matrix<T>
{

    //
    // returns column vector
    pub fn get_column<'a, 'b>(self: &'a Self, i: &'b usize) -> Vector<T>
        where T: Zero + Copy + One
    {
        let mut v: Vector<T> = Vector::zero(&self.m);

        for k in 0..self.m
        {
            *(v.get_mut(&k)) = *(self.get( &k, &i));
        }

        v
    }

    //
    // return row vector
    pub fn get_row<'a, 'b>(self: &'a Self, i: &'b usize) -> Vector<T>
        where T: Zero + One + Copy
    {
        let mut v: Vector<T> = Vector::zero(&self.n);
        v = v.transpose();

        for k in 0..self.n
        {
            *(v.get_mut(&k)) = *(self.get(&i, &k));
        }

        v
    }

    fn set_column(mut self: Self, row: &Vector<T>, i: &usize) -> Matrix<T>
        where T: Zero + One + Copy
    {
        for k in 0..self.m
        {
            *self.get_mut(&k, &i) = *row.get(&k);
        }
        self
    }
}

impl<T> Matrix<T>
{
    pub fn givens<'a, 'b, 'c, 'd, 'e>(m: &'a usize, i: &'b usize, j: &'c usize, c: &'d T, s: &'e T) -> Self
        where T: Neg<Output = T> + Zero + One + Clone + PartialOrd
    {
        if *i >= *m || *j >= *m
        {
            panic!("Index out of bounds");
        }
        let mut givens: Matrix<T> = Matrix::one(m);
        *(givens.get_mut(i,i)) = c.clone();
        *(givens.get_mut(j,j)) = c.clone();
        *(givens.get_mut(i,j)) = s.clone();
        *(givens.get_mut(j,i)) = -s.clone();
        givens
    }
}



impl<T> Matrix<T>
    where T: Mul<T, Output = T> + Sub<T, Output = T> + Zero + One + Clone + Copy + Add<T, Output = T>
 + Div<T, Output = T> + Exponential + AddAssign + Display + Power + FromPrimitive
{
    /// calculates the householder matrix from the householder reflector v
    ///
    ///v:
//    pub fn householder<'a>(v: &'a Vector<T>, k: usize) -> Self
//    {
//        let (v_m, v_n): (usize, usize) = v.dim();
//
//        let d: Vector<T> = v.get_slice(k, v_n -1);
//        let alpha: T = d.p_norm(&T::one());
//        if d.get(&0) >= 0.0
//        {
//            alpha = -alpha;
//        }
//
//        if alpha == 0
//        {
//            let h: Matrix<T> = Matrix::one(&v_n);
//            return h;
//        }
//
//    }

    fn householder_1(v: & Vector<T>) -> Self
    {
        let (v_m, _v_n): (usize, usize) = v.dim();
        let ident : Matrix<T> = Matrix::one(&v_m);

        let two: T = T::from_f64(2.0).unwrap();
        let v_dyadp: Matrix<T> = v.dyadp(&v);
        let h : Matrix<T> = ident - v_dyadp.mul_scalar(&two);
        h
    }
}

impl<T> Matrix<T>
    where T: Real
{
    /// Computes the singular value decomposition
    ///
    /// M = U * S * V*
    ///
    /// # Return
    ///
    /// (u, s, v)
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&4, &4, &vec![4.0, 1.0, -2.0, 2.0, 1.0, 2.0, 0.0, -2.0, 0.0, 3.0, -2.0, 2.0,
    /// 2.0, 1.0, -2.0, -1.0]);
    ///
    /// let (u, s, v): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_sv();
    /// ```
    pub fn dec_sv<'a>(self: &'a Self) -> (Self, Self, Self)
    {
        let (mut u, mut b, mut v): (Matrix<T>, Matrix<T>, Matrix<T>) = self.householder_bidiag();

        let (_m, n): (usize, usize) = b.dim();
        let max_iterations: usize = 500 * n * n;


        let mut u_k: Matrix<T> = Matrix::one(&n);

        for _k in 0.. max_iterations
        {
            let (u_ks, b_k, v_k): (Matrix<T>, Matrix<T>, Matrix<T>) = Matrix::msweep(u_k, b, v);
            u_k = u_ks;
            b = b_k;
            v = v_k;
        }

        u = u * u_k.transpose();

        let (b_m, _b_n): (usize, usize) = b.dim();

        // check that all singular values are positive
        for l in 0..b_m
        {
            if b.get(&l, &l) < &T::zero()
            {
                *b.get_mut(&l, &l) = - *b.get(&l, &l);
                let mut column_l: Vector<T> = u.get_column(&l);
                column_l = column_l.mul_scalar(&-T::one());
                u = u.set_column(&column_l, &l);
            }
        }

        // null all values beneath the diagonal
        for l in 0..b_m
        {
            for k in 0..b_m
            {
                if k != l
                {
                    *b.get_mut(&k, &l) = T::zero();
                }
            }
        }

        // sort singular values in descending order
        return (u, b, v)
    }


//    pub fn dec_sv<'a>(self: &'a Self) -> (Self, Self, Self)
//    {
//        let (mut u, mut b, mut v): (Matrix<T>, Matrix<T>, Matrix<T>) = self.householder_bidiag();
//
//        let TOLERANCE: T = T::from_f64(100.0*0.0001).unwrap();
//        let (m, n): (usize, usize) = b.dim();
//        let max_iterations: usize = 500 * n * n;
//
//        let mut d: Vector<T> = Vector::zero(&n);
//        let mut e: Vector<T> = Vector::zero(&(n-1));
//
//        for i in 0..n
//        {
//            *d.get_mut(&i) = *b.get(&i, &i);
//        }
//
//        for i in 0..n-1
//        {
//            *e.get_mut(&i) = *b.get(&i, &(i + 1));
//        }
//
//        let THRESH: T = T::from_f64(1.9e-19_f64).unwrap();
//
//        let mut i_upper: usize = n - 2;
//        let mut i_lower: usize = 0;
//
//        for k in 0..max
//        {
//
//            for i in (0..i_upper + 1).rev()
//            {
//                if e.get(&i).abs() > THRESH
//                {
//                    i_upper = i;
//                    break;
//                }
//            }
//
//            for i in i_lower..(i_upper + 1)
//            {
//                if e.get(&i).abs() > THRESH
//                {
//                    i_lower = i;
//                    break;
//                }
//            }
//
//            if ((i_upper == i_lower) &&  (e.get(&i_upper).abs() <= THRESH)) || (i_upper < i_lower)
//            {
//                println!("sort");
//                //sort
//                let singular: Matrix<T> = Matrix::new_diag(&m, &n, &d);
//                return (u, singular, v);
//            }
//
//            let e_slice: Vector<T> = e.get_slice(i_lower, i_upper).transpose();
//            let d_slice: Vector<T> = d.get_slice(i_lower, i_upper + 1).transpose();
//
//            let u_slice: Matrix<T> = u.get_slice(i_lower, i_upper + 1, i_lower, i_upper + 1);
//            let v_slice: Matrix<T> = v.get_slice(i_lower, i_upper + 1, i_lower, i_upper + 1);
//
//            let (u_slice_updated, d_slice_updated, e_slice_updated, v_slice_updated): (Matrix<T>, Vector<T>,
//            Vector<T>, Matrix<T>) =
//            Matrix::vsweep(u_slice,
//            d_slice,
//            e_slice,
//             v_slice);
//            u = u.set_slice(&u_slice_updated, i_lower, i_lower);
//            v = v.set_slice( &v_slice_updated, i_lower, i_lower);
//
//            d.set_slice(&d_slice_updated.transpose(), i_lower);
//            e.set_slice(&e_slice_updated.transpose(), i_lower);
//
//        }
//
//        let singular: Matrix<T> = Matrix::new_diag(&m, &n, &d);
//        return (u, singular, v)
//    }
//
//    pub fn vsweep(mut u: Matrix<T>, mut d: Vector<T>, mut e: Vector<T>, mut v: Matrix<T>) -> (Matrix<T>, Vector<T>,
//    Vector<T>, Matrix<T>)
//    {
//        let (m, n): (usize, usize) = d.dim();
//
//        let mut c_old: T = T::one();
//        let mut s_old: T = T::one();
//        let (mut c, mut s, mut r): (T, T, T) = (T::one(), T::one(), T::one());
//
//        //println!("m {}, n{}", m, n);
//
//
//        for i in 0..n-1
//        {
//            let mut q : Matrix<T> = Matrix::one(&n);
//
//            let (c_n, s_n, r_n) = Matrix::rot(c * *d.get(&i), *e.get(&i));
//
//            *q.get_mut(&i, &i) = c_n;
//            *q.get_mut(&i, &(i +1)) = s_n;
//            *q.get_mut(&(i + 1), &i) = -s_n;
//            *q.get_mut(&(i + 1), &(i + 1)) = c_n;
//
//		    u = &q * &u;
//
//            c = c_n;
//            s = s_n;
//            r = r_n;
//            if i != 0
//            {
//                *e.get_mut(&(i-1)) = s_old * r;
//            }
//            let (c_old_n, s_old_n, r_n) = Matrix::rot(c_old * r, *d.get(&(i + 1)) * s);
//
//            *q.get_mut(&i, &i) = c_old_n;
//            *q.get_mut(&i, &(i +1)) = s_old_n;
//            *q.get_mut(&(i + 1), &i) = -s_old_n;
//            *q.get_mut(&(i + 1), &(i + 1)) = c_old_n;
//
//            v = &v * &q.transpose();
//
//            c_old = c_old_n;
//            s_old = s_old_n;
//            *d.get_mut(&i) = r_n;
//        }
//
//        let h: T = *d.get(&(n - 1)) * c;
//        *e.get_mut(&(n-2)) = h * s;
//        *d.get_mut(&(n-1)) = h * c;
//
//        return (u, d, e, v);
//    }

    pub fn msweep(mut u: Matrix<T>, mut b: Matrix<T>, mut v: Matrix<T>) -> (Matrix<T>, Matrix<T>, Matrix<T>)
    {
        let (_m, n): (usize, usize) = b.dim();

        for k in 0..n-1
        {
            let mut q : Matrix<T> = Matrix::one(&n);

            // Construct matrix Q and multiply on the right by Q'.
            // Q annihilates both B(k-1,k+1) and B(k,k+1)
            // but makes B(k+1,k) non-zero.
            let (c_r, s_r, _r_r): (T, T, T) = Matrix::rot(*b.get(&k,&k), *b.get(&k,&(k + 1)));

            *q.get_mut(&k, &k) = c_r;
            *q.get_mut(&k, &(k +1)) = s_r;
            *q.get_mut(&(k + 1), &k) = -s_r;
            *q.get_mut(&(k + 1), &(k + 1)) = c_r;

            b = &b * &q.transpose();
            v = &v * &q.transpose();

            //B(find(abs(B)<1.e-13))=0;

            // Construct matrix Q and multiply on the left by Q.
            // Q annihilates B(k+1,k) but makes B(k,k+1) and
            // B(k,k+2) non-zero.
            let (c_l, s_l, _r_l): (T, T, T) = Matrix::rot(*b.get(&k,&k), *b.get(&(k + 1),&k));

            *q.get_mut(&k, &k) = c_l;
            *q.get_mut(&k, &(k +1)) = s_l;
            *q.get_mut(&(k + 1), &k) = -s_l;
            *q.get_mut(&(k + 1), &(k + 1)) = c_l;

            b = &q * &b;
		    u = &q * &u;
            //B(find(abs(B)<1.e-13))=0;
        }

        return (u, b, v);
    }


    pub fn rot(f: T, g: T) -> (T, T, T)
    {
        if f == T::zero()
        {
            return (T::zero(), T::one(), g);
        }
        else
        {
            let expo: T = T::from_f64(2.0).unwrap();
            let sqrt: T = T::from_f64(0.5).unwrap();
            if f.abs() > g.abs()
            {
                let t: T = g / f;
                let t1: T = (T::one() + t.pow(& expo)).pow(& sqrt);

                return (T::one() / t1, t / t1, f * t1);
            }
            else
            {
                let t: T = f / g;
                let t1: T = (T::one() + t.pow(& expo)).pow(& sqrt);

                return (t / t1, T::one() / t1, g * t1);
            }
        }
    }


    pub fn zero_diagonal(self: &mut Self) -> bool
    {
        let (_m, n): (usize, usize) = self.dim();
        let mut diagonal_zero: bool = false;

        for i in 0..n
        {
            if *self.get(&i, &i) == T::zero()
            {
                diagonal_zero = true;
                *self.get_mut(&i, &(i + 1)) = T::zero()
            }
        }

        return diagonal_zero
    }

    ///
    /// self is an m times n matrix with m >= n
    /// A = UBV^{T}
    /// U \in T^{m \times n}
    /// B \in T^{n \times n}
    /// V \in T^{n \times n}
    ///
    pub fn householder_bidiag<'a>(self: &'a Self) -> (Self, Self, Self)
    {
        let (m, n) : (usize, usize) = self.dim();
        if m < n
        {
            panic!("Read the API");
        }

        let mut u: Matrix<T> = Matrix::one(&m);
        let mut v: Matrix<T> = Matrix::one(&n);
        //let mut b_i : Matrix<T> = Matrix::zero(&n, &n);
        let mut a_i : Matrix<T> = self.clone();


        for i in 0..n-1
        {
            // eliminate non-zeros below the diagonal
            // Keep the product U*B unchanged
            let u_x: Vector<T> = a_i.clone().get_column(&i);
            let u_slice: Vector<T> = u_x.get_slice(i, m - 1);

            let u_reflector : Vector<T> = Matrix::reflector(&u_slice);
            let u_i : Matrix<T> = Matrix::householder_1(&u_reflector);

            let a_i_slice = &u_i * &a_i.clone().get_slice(i, m - 1, i, n - 1);
            a_i = a_i.set_slice(&a_i_slice, i, i);
            let mut u_mi: Matrix<T> = Matrix::one(&m);
            u_mi = u_mi.set_slice(&u_i, i, i);


            u = &u * &u_mi;

            //eliminate non-zeros to the right of the
            //superdiagonal by working with the transpose
            // Keep the product B*V' unchanged
            //B_T = B';
            if i < (n - 1)
            {
                let v_x: Vector<T> = a_i.get_row(&i);
                let v_x_trans: Vector<T> = v_x.transpose();
                let v_x_trans_slice: Vector<T> = v_x_trans.get_slice(i+1, n - 1);

                let v_reflector : Vector<T> = Matrix::reflector(&v_x_trans_slice);
                let v_i : Matrix<T> = Matrix::householder_1(&v_reflector);
                let mut v_ni: Matrix<T> = Matrix::one(&n);
                v_ni = v_ni.set_slice(&v_i, i+1, i+1);
                //let a_i_slice = &a_i.clone().get_slice(i+1, m - 1, i+1, n - 1) * &v_i;
                //a_i = a_i.set_slice(&a_i_slice, i+1, i+1);
                a_i = &a_i * &v_ni;

                v = &v * &v_ni;
            }

        }

        //Null all elements beneath the diagonal, and superdiagonal
        for i in 0..m
        {
            for k in 0..n
            {
                if k != i && k != (i + 1)
                {
                    *a_i.get_mut(&i, &k)  = T::zero();
                }
            }
        }
        (u, a_i, v)

    }

    /// norm(x) = 1
    ///
    pub fn reflector<'a>(x: &Vector<T>) -> Vector<T>
    {
        let two = T::one() + T::one();
        let mut x_temp: Vector<T> = x.clone();

        let norm_x: T = x.p_norm(&two);

        *x_temp.get_mut(&0) += x.get(&0).sgn() * norm_x;

        let f: T = T::one() / x_temp.p_norm(&two);
        x_temp = x_temp.mul_scalar(&f);

        x_temp
    }



}
//    pub fn golub_kahan_step<'a>(self: &'a Self) -> (Self, Self, Self)
//    {
//
//        let mut b: Matrix<T> = self.clone();
//        let (b_m, b_n): (usize, usize) = b.dim();
//        let mut u: Matrix<T> = Matrix::one(&b_n);
//        let mut v: Matrix<T> = Matrix::one(&b_n);
//
//
//        //let t: Matrix<T> = &b.transpose() * &b;
//
//        //(t_{11} - \lambda)(t_{22} - \lambda) -(t_{21}*t_{12})
//        //
//        //\lambda^2 - \lambda(t_{11} + t_{22}) + t_{11}t_{22}) - t_{21}t_{12} = 0
//        //
//        // f = 1
//        // g = -(t_{11} + t_{22})
//        // h = t_{11}t_{22} - t_{21}t_{12}
//        //
//        // \lambda = \frac{-g +/- (g^2 - 4*fh)^{0.5}}{2*f)
//
//
//        let d_m: T = *b.get(&(b_n - 2), &(b_n - 2));
//        let f_m1: T = *b.get(&(b_n - 3), &(b_n - 2));
//        let f_m: T = *b.get(&(b_n - 2), &(b_n - 1));
//        let d_n: T = *b.get(&(b_n - 1), &(b_n - 1));
//        let t_11: T = d_m * d_m + f_m1 * f_m1;
//        let t_12: T = d_m * f_m;
//        let t_21: T = d_m * f_m;
//        let t_22: T = d_n * d_n + f_m * f_m;
//
//        let g: T = -(t_11 + t_22);
//        let h: T = (t_11 * t_22)  - (t_21 * t_12);
//        let k: T = (g.pow(&T::from_f64(2.0_f64).unwrap()) - T::from_f64(4.0).unwrap() * h).pow(&T::from_f64(0.5_f64)
//        .unwrap());
//        let eigv_1: T = (-g + k) / T::from_f64(2.0_f64).unwrap();
//        let eigv_2: T = (-g - k) / T::from_f64(2.0_f64).unwrap();
//
//        let eigv: T;
//        if (eigv_1 - t_22).abs() < (eigv_2 - t_22).abs()
//        {
//            eigv = eigv_1;
//        }
//        else
//        {
//            eigv = eigv_2;
//        }
//
//        let mut y: T = t_11 - eigv;
//        let mut z: T = t_12;
//
//        for i in 0..(b_n - 1)
//        {
//            //y*s + z + c = 0
//            //s = \sin \theta
//            //c = \cos \theta
//            //\tan \theta = -z / y
//
//            let theta_1: T = (-z).arctan2(&y);
//            let c_1: T = theta_1.cos();
//            let s_1: T = theta_1.sin();
//
//            let g_1: Matrix<T> = Matrix::givens(&b_n, &i, &(i + 1), &c_1, &s_1);
//            v = v * g_1.clone();
//            b = b * g_1;
//
//            y = *b.get(&i, &i);
//            z = *b.get(&(i + 1), &i);
//
//            let theta_2: T = (-z).arctan2(&y);
//            let c_2: T = theta_2.cos();
//            let s_2: T = -theta_2.sin();
//
//            let g_2: Matrix<T> = Matrix::givens(&b_n, &i, &(i + 1), &c_2, &s_2);
//            b = g_2.clone() * b;
//            u = g_2 * u;
//
//            if i < b_n - 2
//            {
//                y = *b.get(&i, &(i + 1));
//                z = *b.get(&i, &(i + 2));
//            }
//
//            //println!("b_i: {}", b);
//        }
//
//        return (u, b, v)
//    }
//
//    fn nullen(mut self: Self) -> Self
//    {
//        let epsilon: T = T::from_f64(std::f64::EPSILON).unwrap(); //It shoud be a multiple of the machine epsilon
//        let (_m, n): (usize, usize) = self.dim();
//
//        for i in 0..(n - 1)
//        {
//            let b_ii: T = *self.get(&i, &i);
//            let b_iip: T = *self.get(&i, &(i + 1));
//            let b_ipip: T = *self.get(&(i + 1), &(i + 1));
//
//            if b_iip.abs() <= T::from_f64(0.000001).unwrap() //T::from_f64(1000.0).unwrap() * epsilon * (b_ii.abs() +
//            // b_ipip
//            // .abs())
//            {
//                *self.get_mut(&i, &(i + 1)) = T::zero()
//            }
//        }
//        return self
//    }
//
//    //
//    //  q = 2
//    //  a_11 a_12 a_13 a_14     x 0 0 0
//    //  a_21 a_22 a_23 a_24     0 x x 0
//    //  a_31 a_32 a_33 a_34 =   0 0 x 0 |
//    //  a_41 a_42 a_43 a_44     0 0 0 x | q
//    //								_ _
//    //  x != 0						q
//    pub fn find_nonzero_superdiagonal(self: &Self) -> usize
//    {
//        let (m, n): (usize, usize) = self.dim();
//
//        let mut q: usize = 0;
//
//        // a_{n-0}{n} does not have a superdiagnoal
//        for i in 1..n
//        {
//            q = i;
//            if *self.get(&(n - i - 1), &(n - i)) != T::zero()
//            {
//                break;
//            }
//        }
//
//        return q;
//    }
//
//     //
//    //  p = 3
//    //  a_11 a_12 a_13 a_14     x x x x
//    //  a_21 a_22 a_23 a_24     x x 0 x |
//    //  a_31 a_32 a_33 a_34 =   x x x 0 |
//    //  a_41 a_42 a_43 a_44     x x x x | p
//    //							  _ _ _
//    //  x != 0						p
//    pub fn find_zero_superdiagonal(self: &Self) -> usize
//    {
//        let (m, n): (usize, usize) = self.dim();
//
//        let mut p: usize = 0;
//
//        // a_{n-0}{n} does not have a superdiagnoal
//        for i in 1..n
//        {
//            p = i;
//            if *self.get(&(n - i - 1), &(n - i)) == T::zero()
//            {
//                break;
//            }
//        }
//
//        return p;
//    }
//}

impl<T> Matrix<T>
    where T: Real
{
    /// Returns a slice of the matrix
    ///
    /// # Arugments
    ///
    /// 0 <= row_s < m \
    /// 0 <= row_e < m \
    /// 0 <= column_s < n \
    /// 0 <= column_e <= n \
    ///
    /// row_s: start row \
    /// row_e: end row \
    /// column_s: start column \
    /// column_e: end column \
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let mut a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, -2.0, 3.0, -7.0]);
    /// a = a.get_slice(0, 0, 1, 1);
    ///
    /// let a_ref: Matrix<f64> = Matrix::new(&1, &1, &vec![-2.0]);
    ///
    /// assert_eq!(a_ref, a);
    /// ```
    pub fn get_slice(self: &Self, row_s: usize, row_e: usize, column_s: usize, column_e: usize) -> Matrix<T>
    {
        assert!(row_s < self.m);
        assert!(row_e < self.m);
        assert!(column_s < self.n);
        assert!(column_e < self.n);

        let mut slice: Matrix<T> = Matrix::zero(&(row_e - row_s + 1), &(column_e - column_s + 1));

        for r in row_s..(row_e + 1)
        {
            for c in column_s..(column_e + 1)
            {
                  *slice.get_mut(&(r - row_s), &(c - column_s)) = *self.get(&r, &c)
            }
        }
        return slice;
    }

    /// Replaces parts of the matrix with the given values
    ///
    /// # Arugments
    ///
    /// 0 <= row < m \
    /// 0 <= column < n
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let mut a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(&1, &2, &vec![2.0, -1.0]);
    /// a = a.set_slice(&b, 0, 0);
    ///
    /// let a_updated: Matrix<f64> = Matrix::new(&2, &2, &vec![2.0, -1.0, 3.0, -7.0]);
    ///
    /// assert_eq!(a_updated, a);
    /// ```
    pub fn set_slice(mut self: Self, slice: &Self, row: usize, column: usize) -> Matrix<T>
    {
        let (s_m, s_n): (usize, usize) = slice.dim();
        let (m, n): (usize, usize) = self.dim();
        assert!(row + s_m <= m);
        assert!(column + s_n <= n);

        for r in row..(row + s_m)
        {
            for c in column..(column + s_n)
            {
                *self.get_mut(&r, &c) = *slice.get(&(r - row), &(c - column));
            }
        }
        self
    }
}


impl<T> Display for Matrix<T>
    where T: Display
{
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "\n").unwrap();
        for i in 0..self.m
        {
            for j in 0..self.n
            {
                write!(f, "{} ", self.data[(i * self.n + j)]).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "\n")
    }
}


impl<T> Matrix<T>
{
    /// Returns the mutual element a_ij from the matrix
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let mut a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    /// *a.get_mut(&1, &0) = -8.0;
    ///
    /// let a_updated: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, -8.0, -7.0]);
    /// assert_eq!(a_updated, a);
    /// ```
    pub fn get_mut<'a, 'b, 'c>(self: &'a mut Self, i: &'b usize, j: &'c usize) -> &'a mut T
    {
        assert!(*i < self.m);
        assert!(*j < self.n);
        & mut(self.data[i * self.n + j])
    }


    /// Returns the element a_ij from the matrix
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    /// let a_ref: f64 = 3.0;
    /// let element: f64 = *a.get(&1, &0);
    ///
    /// assert_eq!(a_ref, element);
    /// ```
    pub fn get<'a, 'b, 'c>(self: &'a Self, i: &'b usize, j: &'c usize) -> &'a T
    {
        assert!(*i < self.m);
        assert!(*j < self.n);

        & self.data[i * self.n + j]
    }
}



impl<T> Matrix<T>
    where T: Mul<T, Output = T> + Zero + Clone
{
    /// Multiplies a matrix with a scalar
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    /// let f: f64 = 7.0;
    /// let res_ref: Matrix<f64> = Matrix::new(&2, &2, &vec![7.0, 0.0, 21.0, -49.0]);
    ///
    /// assert_eq!(res_ref, a.mul_scalar(&f));
    /// ```
    pub fn mul_scalar<'a, 'b>(self: &'a Self, rhs: &'b T) -> Matrix<T>
    {
        let (rows, cols): (usize, usize) = self.dim();

        let mut prod: Matrix<T> = Matrix::zero(&rows, &cols);

        for i in 0..rows
        {
            for j in 0..cols
            {
                *prod.get_mut(&i, &j) = self.get(&i, &j).clone() * (*rhs).clone();
            }
        }
        prod
    }
}


impl <T> Mul for Matrix<T>
    where T: AddAssign + Zero + Mul<T, Output = T> + Clone
{
    type Output = Matrix<T>;

    /// Multiplies two matrices
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    /// let res_ref: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, -18.0, 49.0]);
    /// assert_eq!(res_ref, a * b);
    /// ```
    fn mul(self: Self, rhs: Self) -> Self::Output
    {
        (&self).mul(&rhs)
    }
}

impl <'a, 'b, T> Mul<&'b Matrix<T>> for &'a Matrix<T>
    where T: Mul<T, Output = T> + AddAssign + Zero + Clone
{
    type Output = Matrix<T>;

    /// Multiplies two matrices
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    /// let res_ref: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, -18.0, 49.0]);
    /// assert_eq!(res_ref, &a * &b);
    /// ```
    fn mul(self: Self, rhs: &'b Matrix<T>) -> Self::Output
    {
        let (l_rows, l_cols) = self.dim();
        let (r_rows, r_cols): (usize, usize) = rhs.dim();
        assert_eq!(l_cols, r_rows);

        let mut prod: Matrix<T> = Matrix::zero(&l_rows, &r_cols);

        for i in 0..l_rows
        {
            for j in 0..r_cols
            {
                let mut sum: T = T::zero();
                for k in 0..l_cols
                {
                    sum += self.get(&i, &k).clone() * rhs.get(&k, &j).clone();
                }
                *prod.get_mut(&i, &j) = sum;
            }
        }
        prod
    }
}


impl <T> Add for Matrix<T>
    where T: Semiring
{
    type Output = Matrix<T>;

    /// Adds two matrices
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    ///
    /// assert_eq!(b, Matrix::zero(&2, &2) + a);
    /// ```
    fn add(self: Self, rhs: Self) -> Self::Output
    {
        (&self).add(&rhs)
    }
}


///
///Adds two matrices
///
impl <'a, 'b, T> Add<&'b Matrix<T>> for &'a Matrix<T>
    where T: Add<T, Output = T> + Zero + Clone
{
    type Output = Matrix<T>;

    /// Adds two matrices
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    ///
    /// assert_eq!(b, &Matrix::zero(&2, &2) + &a);
    /// ```
    fn add(self: Self, rhs: &'b Matrix<T>) -> Self::Output
    {
        assert_eq!(self.dim(), rhs.dim());

        let mut sum: Matrix<T> = Matrix::zero(&self.m, &self.n);

        for i in 0..sum.m
        {
            for j in 0..sum.n
            {
                *sum.get_mut(&i, &j) = self.get(&i, &j).clone() + rhs.get(&i, &j).clone();
            }
        }
        sum
    }
}


impl <T> Sub for Matrix<T>
    where T: Sub<Output = T> + Zero + Clone + Copy
{
    type Output = Matrix<T>;

    /// Subtracts two matrices
    ///
    /// A = (a_{ij}) \in T^{m \times n}
    /// B = (b_{ij}) \in T^{m \times n}
    /// A - B = ( a_{ij} - b_{ij} )
    ///
    /// # Arguments
    ///
    /// rhs:
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    ///
    /// assert_eq!(Matrix::zero(&2, &2), a - b);
    /// ```
    fn sub(self: Self, rhs: Self) -> Self::Output
    {
        (&self).sub(&rhs)
    }
}


impl<'a, 'b, T> Sub<&'b Matrix<T>> for &'a Matrix<T>
    where T: Sub<T, Output = T> + Zero + Clone + Copy
{
    type Output = Matrix<T>;

    fn sub(self: Self, rhs: &'b Matrix<T>) -> Self::Output
    {
        assert_eq!(self.dim(), rhs.dim());
        let (m, n) : (usize, usize) = self.dim();

        let mut diff: Matrix<T> = Matrix::zero(&m, &n);

        for i in 0..m//Interval::range(Natural::zero(), m)
        {
            for j in 0..n //Interval::range(Natural::zero(), n)
            {
                *diff.get_mut(&i, &j) = self.get(&i, &j).clone() - rhs.get(&i, &j).clone();
            }
        }
        diff
    }
}


impl<T> PartialEq for Matrix<T>
    where T: PartialEq
{
    /// Checks if two matrices are equal
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    ///
    /// assert_eq!(true, a==b);
    /// ```
    fn eq<'a, 'b>(self: &'a Self, other: &'b Self) -> bool
    {

        if self.dim() != other.dim()
        {
            return false;
        }

        if self.data == other.data
        {
            return true;
        }

        false
    }
}

impl<T> Matrix<T>
    where T: Clone + Copy + Zero + One
{
    pub fn new<'a, 'b, 'c>(m: &'a usize, n: &'b usize, data: &'c Vec<T>) -> Self
    {
        assert_eq!(*m * *n, data.len());
        Matrix{m: *m, n: *n, data: data.clone()}
    }

//    fn new_diag<'a, 'b, 'c>(m: &'a usize, n: &'b usize, data: &'c Vector<T>) -> Self
//    {
//        let (size_m, _size_n): (usize, usize) = data.dim();
//        assert!(size_m <= *m);
//        assert!(size_m <= *n);
//
//        let mut diag: Matrix<T> = Matrix::zero(&m, &n);
//
//        for i in 0..size_m
//        {
//            *diag.get_mut(&i, &i) = *data.get(&i);
//        }
//
//        return diag;
//    }
}



impl<T> Matrix<T>
    where T: Zero + Clone
{
    /// Returns the zero matrix(additive neutral element)
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = &a + &Matrix::zero(&2, &2);
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn zero<'a, 'b>(m: &'a usize, n: &'b usize) -> Self
    {
        Matrix {
            m: *m,
            n: *n,
            data: vec![T::zero(); *m * *n],
        }
    }
}


impl<T> Matrix<T>
    where T: One + Zero + Clone
{
    /// Returns the eye matrix(multiplicative neutral element)
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = &a * &Matrix::one(&2);
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn one<'a>(size: &'a usize) -> Self
    {
        let mut data = vec![T::zero(); *size * *size];

        for i in 0..*size //Interval::range(Natural::zero(), *size)
        {
            data[i * *size + i] = T::one();
        }

        Matrix
        {
            m: *size,
            n: *size,
            data: data,
        }
    }
}


impl<T> Matrix<T>
{
    /// Returns the matrix dimension
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&4, &2, &vec![1.0, 0.0, 3.0, 0.0, 1.0, -7.0, 0.5, 0.25]);
    /// let (m, n) = a.dim();
    ///
    /// assert_eq!(4, m);
    /// assert_eq!(2, n);
    /// ```
    pub fn dim(&self) -> (usize, usize)
    {
        (self.m, self.n)
    }
}

impl<T> Matrix<T>
     where T:  Add<Output = T> + Sub<Output = T> +  Mul<Output = T>  + Div<Output = T> + One + Zero + Clone +
    PartialOrd + AddAssign + Copy
{


    /// Inverse Matrix
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&2, &2, &vec![1.0, 0.0, 3.0, -7.0]);
    /// let b_inv: Matrix<f64> = a.inv();
    ///
    /// ```
    pub fn inv<'a>(self: &'a Self) -> Matrix<T>
    {
        let (mut l, mut u, p) : (Matrix<T>, Matrix<T>, Matrix<T>) = self.dec_lu();

        l.subst_forward();
        u.subst_backward();
        return &(&u * &l) * &p;
    }

     /*
     * inplace backward substitution
     */
    fn subst_backward<'a>(self: &'a mut Self)
    {
        for k in (0..self.n).rev()
        {
            for l in (k..self.n).rev()
            {

                let mut sum : T = T::zero();

                for i in (k+1)..self.n
                {
                    sum += self.data[k * self.n + i] * self.data[i * self.n + l];
                }

                let b : T;
                if k == l
                {
                    b = T::one();

                }
                else
                {
                    b = T::zero();
                }
                let div : T = self.data[k * self.n + k];
                self.data[k * self.n + l] = (b - sum) / div;

            }
        }
    }

    /**
     * inplace forward substitution
     */
    fn subst_forward<'a>(self: &'a mut Self)
    {

        for k in 0..self.n
        {
            for l in 0..k
            {

                let mut sum : T = T::zero();

                for i in 0..k
                {
                    sum += self.data[k * self.n + i] * self.data[i * self.n + l];
                }

                let b : T;
                if k == l
                {
                    b = T::one();

                }
                else
                {
                    b = T::zero();
                }
                let div : T = self.data[k * self.n + k];
                self.data[k * self.n + l] = (b - sum) / div;

            }
        }

    }
}

impl<T> Matrix<T>
     where T: Real
{
    /// Decomposes self in to the M
    ///
    /// q * h * q^T = self
    ///
    /// # Arguments
    ///
    /// # Return
    ///
    /// (q, h)
    ///
    /// q:
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(&3, &3, &vec![1.0, 5.0, 3.0, 1.0, 0.0, -7.0, 3.0, 8.0, 9.0]);
    /// let (q, h): (Matrix<f64>, Matrix<f64>) = a.dec_hessenberg();
    ///
    /// ```
    pub fn dec_hessenberg(self: &Self) -> (Matrix<T>, Matrix<T>)
    {
        let (m, n) : (usize, usize) = self.dim();
        if m < n
        {
            panic!("Read the API");
        }

        let mut q: Matrix<T> = Matrix::one(&m);
        let mut h: Matrix<T> = self.clone();

        for i in 0..n-1
        {
            // eliminate non-zeros below the lower subdiagonal
            let u_x: Vector<T> = h.clone().get_column(&i);
            let u_slice: Vector<T> = u_x.get_slice(i+1, m - 1);

            let u_reflector : Vector<T> = Matrix::reflector(&u_slice);
            let u_i : Matrix<T> = Matrix::householder_1(&u_reflector);

            let h_slice = &u_i * &h.clone().get_slice(i+1, m - 1, i, n - 1);
            h = h.set_slice(&h_slice, i+1, i);

            let h_trans =  h.clone().get_slice(i, m - 1, i+1, n - 1);

            let h_slice_r = &h_trans * &u_i;
            h = h.set_slice(&h_slice_r, i, i+1);

            let mut q_mi: Matrix<T> = Matrix::one(&m);
            q_mi = q_mi.set_slice(&u_i, i+1, i+1);

            q = &q_mi * &q;

        }

        return (q.transpose_inplace(), h);
    }
}

