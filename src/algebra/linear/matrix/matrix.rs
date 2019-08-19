/// Matrix

use std::clone::Clone;
use crate::algebra::abstr::{Number, Semiring, Zero, One, Sign};
use crate::algebra::linear::Vector;
use crate::algebra::abstr::Real;
use std::fmt::Display;
use std::fmt;

use rand;
use rand::Rng;

use serde::{Deserialize, Serialize};


/// Macro to construct matrices
///
/// ```
/// #[macro_use]
/// extern crate mathru;
/// fn main()
/// {
///     use mathru::algebra::linear::Matrix;
///
///     // Construct a 2x3 matrix of f32
///     let mat: Matrix<f32> = matrix![1.0, 2.0, 3.0; 4.0, 5.0, 6.0];
/// }
/// ```
#[macro_export]
macro_rules! matrix
{
    ($( $( $x: expr ),*);*) =>
    {
        {
            let data_nested_array = [ $( [ $($x),* ] ),* ];
            let rows = data_nested_array.len();
            let cols = data_nested_array[0].len();
            let mut data_array: Vec<_> = Vec::with_capacity(rows * cols);
            for j in 0..cols
            {
                for i in 0..rows
                {
                    data_array.push(data_nested_array[i][j]);
                }
            }
            Matrix::new(rows, cols, data_array)
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matrix<T>
{
    /// Num of rows which the matrix has
    pub(crate) m: usize,
    /// Num of columns which the matrix ha
    pub(crate) n: usize,
    /// Matrix entries
    pub(crate) data:  Vec<T>
}

impl<T> Matrix<T>
    where T: Clone
{
    /// Applies the function f on every element in the matrix
    ///
    pub fn apply(mut self: Matrix<T>, f: &dyn Fn(&T) -> T) -> Matrix<T>
    {
        self.data = self.data.iter().map(f).collect();
        self
    }

    pub fn apply_mut(self: &Matrix<T>, f: &dyn Fn(&T) -> T) -> Matrix<T>
    {
        (self.clone()).apply(f)
    }
}


impl<T> Matrix<T>
    where T: Real
{
    /// Returns the transposed matrix
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let mut a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, -2.0, 3.0, -7.0]);
    /// a = a.transpose();
    ///
    /// let a_trans: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 3.0, -2.0, -7.0]);
    ///
    /// assert_eq!(a_trans, a);
    /// ```
    pub fn transpose(self: &Self) -> Self
    {
        let mut trans : Matrix<T> = Matrix::zero(self.n, self.m);

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
    /// TODO
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Matrix};
	///
	/// let mut uut: Matrix<f64> = Matrix::new(4, 2, vec![1.0, 0.0, 3.0, 0.0, 1.0, -7.0, 0.5, 0.25]);
    /// uut = uut.transpose_inplace();
    ///
    /// let refer: Matrix<f64> = Matrix::new(2, 4, vec![1.0, 3.0, 1.0, 0.5, 0.0, 0.0, -7.0, 0.25]);
    /// assert_eq!(refer, uut);
    /// ```
    pub fn transpose_inplace<'a>(mut self: Self) -> Matrix<T>
    {

        let gcdiv: usize = Matrix::<T>::gcd(self.m, self.n);

        let a: usize = self.m / gcdiv;
        let b: usize = self.n / gcdiv;

        let bigger: usize = self.m.max(self.n);

        let mut temp : Vec<T> = Vec::with_capacity(bigger);
        //Bad
        unsafe {temp.set_len(bigger)};

        if gcdiv > 1
        {
            for j in 0..self.n
            {
                for i in 0..self.m
                {
                    temp[i] = self.data[j * self.m + self.gather(i, j, b)];
                }

                for i in 0..self.m
                {
                    self.data[j * self.m + i] = temp[i];
                }
            }
        }

        for i in 0..self.m
        {
            for j in 0..self.n
            {
                temp[self.scatter_da(i, j, b)] = self.data[j * self.m + i];
            }

            for j in 0..self.n
            {
                self.data[j * self.m + i] = temp[j];
            }
        }

        for j in 0..self.n
        {
            for i in 0..self.m
            {
                temp[i] =  self.data[j * self.m + self.gather_sa(i, j, a)];
            }

            for i in 0..self.m
            {
                self.data[j * self.m + i] = temp[i];
            }
        }

        let temp_m : usize = self.m;
        self.m = self.n;
        self.n = temp_m;

        return self;
    }

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
    where T: Real
{

    /// Calculates the determinant
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, -2.0, 3.0, -7.0]);
    /// let determinant_a: f64 = a.det();
    ///
    /// assert_eq!(-1.0, determinant_a);
    /// ```
    pub fn det<'a>(self: &'a Self) -> T
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

        let mut counter: usize = 0;
        for i in 0..self.m
        {
            if *p.get(&i, &i) != T::one()
            {
                counter += 1;
            }
        }

        let mut perm: T = T::one();
        if counter != 0
        {
            perm = (-T::one()).pow( &T::from_usize(counter - 1).unwrap());
        }

        perm * det
    }


    /// Calculates the trace of a matrix
    ///
    /// # Arguments
    ///
    /// self: square matrix
    ///
    /// # Panics
    ///
    /// if it is not a square matrix
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, -2.0, 3.0, -7.0]);
    /// let tr: f64 = a.trace();
    ///
    /// assert_eq!(-6.0, tr);
    /// ```
    pub fn trace(self: &Self) -> T
        where T: Real
    {
        let (m, n): (usize, usize) = self.dim();
        if m != n
        {
            panic!("matrix is not square");
        }

        let mut sum: T = T::zero();
        for i in 0..m
        {
            sum += self.get(&i, &i).clone();
        }

        return sum;
    }
}




#[cfg(feature = "native")]
impl<T> Matrix<T>
    where T: Clone
{
    pub(super) fn swap_rows<'a, 'b, 'c>(self: &'a mut Self, i: &'b usize, j: &'c usize)
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
    where T: Real
{
    //
    // returns column vector
    pub fn get_column<'a, 'b>(self: &'a Self, i: &'b usize) -> Vector<T>
    {
        let mut v: Vector<T> = Vector::zero(self.m);

        for k in 0..self.m
        {
            *(v.get_mut(&k)) = *(self.get( &k, &i));
        }

        v
    }

    ///
    /// return row vector
    ///
    /// i: row
    pub fn get_row<'a, 'b>(self: &'a Self, i: &'b usize) -> Vector<T>
    {
        let mut v: Vector<T> = Vector::zero(self.n);
        v = v.transpose();

        for k in 0..self.n
        {
            *(v.get_mut(&k)) = *(self.get(&i, &k));
        }

        v
    }

    /// set column
    pub fn set_column(mut self: Self, column: &Vector<T>, i: &usize) -> Matrix<T>
    {
        let (m, _n) = column.dim();
        if m != self.m
        {
            panic!("Dimensions do not match");
        }

        for k in 0..self.m
        {
            *self.get_mut(&k, &i) = *column.get(&k);
        }
        self
    }

    /// set row
    ///
    /// # Arguments
    /// * 'row'
    /// * 'i'
    ///
    /// # Panics
    ///
    ///
    pub fn set_row(mut self: Self, row: &Vector<T>, i: &usize) -> Matrix<T>
    {
        let (_m, n): (usize, usize) = row.dim();
        if n != self.n
        {
            panic!("Dimensions do not match");
        }

        for k in 0..self.n
        {
            *self.get_mut(&i, &k) = *row.get(&k);
        }
        self
    }


}

impl<T> Matrix<T>
    where T: Real
{

    ///
    ///
    pub fn givens<'a, 'b, 'c, 'd, 'e>(m: &'a usize, i: &'b usize, j: &'c usize, c: &'d T, s: &'e T) -> Self
    {
        if *i >= *m || *j >= *m
        {
            panic!("Index out of bounds");
        }
        let mut givens: Matrix<T> = Matrix::one(*m);
        *(givens.get_mut(i,i)) = c.clone();
        *(givens.get_mut(j,j)) = *c;
        *(givens.get_mut(i,j)) = s.clone();
        *(givens.get_mut(j,i)) = -*s;
        givens
    }

    #[cfg(feature = "native")]
    /// function [c,s] = Givens(a,b)
    /// Givens rotation computation
    /// Determines cosine-sine pair (c,s) so that [c s;-s c]'*[a;b] = [r;0]
    /// GVL4: Algorithm 5.1.3
    pub(super) fn givens_cosine_sine_pair(a: T,b: T) -> (T, T)
    {
        let exponent: T = T::from_f64(2.0).unwrap();
        let exponent_sqrt: T = T::from_f64(0.5).unwrap();

        let c: T;
        let s: T;

        if b == T::zero()
        {
            c = T::one();
            s = T::zero();
        }
        else
        {
            if b.abs() > a.abs()
            {
                let tau: T = -a / b;
                s = T::one() / (T::one() + tau.pow(&exponent)).pow(&exponent_sqrt);
                c = s * tau;
            }
            else
            {
                let tau: T = -b / a;
			    c = T::one() / (T::one() + tau.pow(&exponent)).pow(&exponent_sqrt);
                s = c * tau;
            }
        }

        return (c, s);
    }
}



impl<T> Matrix<T>
    where T: Real
{
    /// Returns the householder matrix
    ///
    /// # Arguments
    ///
    /// v: Column vector
    /// k: index 0 <= k < m
    ///
    /// # Panics
    ///
    /// if index out of bounds
    pub fn householder(v: & Vector<T>, k: usize) -> Self
    {
        let (v_m, v_n): (usize, usize) = v.dim();
        if k >= v_m
        {
            panic!("Index k out of bounds");
        }

        if v_m == 0
        {
            panic!();
        }

        if v_m == 1
        {
            return Matrix::one(v_m);
        }

        let d: Vector<T> = v.get_slice(k, v_m -1);

        let norm: T = T::from_f64(2.0).unwrap();

        let alpha: T;

        let d_0: T = *d.get(&0);

        if  d_0 >= T::zero()
        {
            alpha = -d.p_norm(&norm);
        }
        else
        {
            alpha = d.p_norm(&norm);
        }

        if alpha == T::zero()
        {
            let h: Matrix<T> = Matrix::one(v_n);
            return h;
        }

        let (d_m, _d_n) = d.dim();

        let mut v: Vector<T> = Vector::zero(d_m);

        * v.get_mut(&0) = (T::from_f64(0.5).unwrap() * (T::one() - d_0 / alpha)).pow(&T::from_f64(0.5).unwrap());
        let p: T = -alpha * *v.get(&0);


        if d_m - 1 >= 1
        {
            let temp: Vector<T> = d.get_slice(1, d_m - 1).apply(&|e: &T| -> T {*e / (T::from_f64(2.0).unwrap() * p)});
            v.set_slice(&temp, 1);
        }

        let mut w: Vector<T> = Vector::zero(v_m);

        w.set_slice(&v, k);


        let ident : Matrix<T> = Matrix::one(v_m);

        let two: T = T::from_f64(2.0).unwrap();
        let w_dyadp: Matrix<T> = w.dyadp(&w);
        let h : Matrix<T> = &ident - &(&w_dyadp * &two);

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
    /// let a: Matrix<f64> = Matrix::new(4, 4, vec![4.0, 1.0, -2.0, 2.0, 1.0, 2.0, 0.0, -2.0, 0.0, 3.0, -2.0, 2.0,
    /// 2.0, 1.0, -2.0, -1.0]);
    ///
    /// let (u, s, v): (Matrix<f64>, Matrix<f64>, Matrix<f64>) = a.dec_sv();
    /// ```
    pub fn dec_sv<'a>(self: &'a Self) -> (Self, Self, Self)
    {
        let (mut u, mut b, mut v): (Matrix<T>, Matrix<T>, Matrix<T>) = self.householder_bidiag();

        let (_m, n): (usize, usize) = b.dim();
        let max_iterations: usize = 500 * n * n;


        let mut u_k: Matrix<T> = Matrix::one(n);

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
                column_l = &column_l * &-T::one();
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

    fn msweep(mut u: Matrix<T>, mut b: Matrix<T>, mut v: Matrix<T>) -> (Matrix<T>, Matrix<T>, Matrix<T>)
    {
        let (_m, n): (usize, usize) = b.dim();

        for k in 0..n-1
        {
            let mut q : Matrix<T> = Matrix::one(n);

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

        let mut u: Matrix<T> = Matrix::one(m);
        let mut v: Matrix<T> = Matrix::one(n);
        //let mut b_i : Matrix<T> = Matrix::zero(&n, &n);
        let mut a_i : Matrix<T> = self.clone();


        for i in 0..n-1
        {
            // eliminate non-zeros below the diagonal
            // Keep the product U*B unchanged
            let u_x: Vector<T> = a_i.clone().get_column(&i);
            let u_slice: Vector<T> = u_x.get_slice(i, m - 1);

            let u_i: Matrix<T> = Matrix::householder(&u_slice, 0);

            let a_i_slice = &u_i * &a_i.clone().get_slice(i, m - 1, i, n - 1);
            a_i = a_i.set_slice(&a_i_slice, i, i);
            let mut u_mi: Matrix<T> = Matrix::one(m);
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

                let v_i: Matrix<T> = Matrix::householder(&v_x_trans_slice, 0);

                let mut v_ni: Matrix<T> = Matrix::one(n);
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
    /// # #[macro_use]
    /// # extern crate mathru;
    /// # fn main()
    /// # {
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let mut a: Matrix<f64> = matrix![1.0, -2.0; 3.0, -7.0];
    /// a = a.get_slice(0, 0, 1, 1);
    ///
    /// let a_ref: Matrix<f64> = Matrix::new(1, 1, vec![-2.0]);
    ///
    /// assert_eq!(a_ref, a);
    /// # }
    /// ```
    pub fn get_slice(self: &Self, row_s: usize, row_e: usize, column_s: usize, column_e: usize) -> Matrix<T>
    {
        assert!(row_s < self.m);
        assert!(row_e < self.m);
        assert!(column_s < self.n);
        assert!(column_e < self.n);

        let mut slice: Matrix<T> = Matrix::zero(row_e - row_s + 1, column_e - column_s + 1);

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
    /// # #[macro_use]
    /// # extern crate mathru;
    /// # fn main()
    /// # {
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let mut a: Matrix<f64> = matrix![   1.0, 0.0;
    ///                                     3.0, -7.0];
    ///
    /// let b: Matrix<f64> = matrix![2.0, -1.0];
    /// a = a.set_slice(&b, 0, 0);
    ///
    /// let a_updated: Matrix<f64> = matrix![   2.0, -1.0;
    ///                                         3.0, -7.0];
    ///
    /// assert_eq!(a_updated, a);
    /// # }
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
                write!(f, "{} ", self.get(&i, &j)).unwrap();
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
    /// # #[macro_use]
    /// # extern crate mathru;
    /// # fn main()
    /// # {
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let mut a: Matrix<f64> = matrix![1.0, 0.0; 3.0, -7.0];
    /// *a.get_mut(&1, &0) = -8.0;
    ///
    /// let a_updated: Matrix<f64> = matrix![1.0, 0.0; -8.0, -7.0];
    /// assert_eq!(a_updated, a);
    /// # }
    /// ```
    pub fn get_mut<'a, 'b, 'c>(self: &'a mut Self, i: &'b usize, j: &'c usize) -> &'a mut T
    {
        assert!(*i < self.m);
        assert!(*j < self.n);
        & mut(self.data[j * self.m + i])
    }


    /// Returns the element a_ij from the matrix
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate mathru;
    /// # fn main()
    /// # {
    /// use mathru::algebra::linear::{Matrix};
    ///
    /// let a: Matrix<f64> = matrix![   1.0, 0.0;
    ///                                 3.0, -7.0];
    ///
    /// let a_ref: f64 = 3.0;
    /// let element: f64 = *a.get(&1, &0);
    ///
    /// assert_eq!(a_ref, element);
    /// # }
    /// ```
    pub fn get(self: &Self, i: &usize, j: &usize) -> & T
    {
        assert!(*i < self.m);
        assert!(*j < self.n);

        return & self.data[j * self.m + i];
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
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
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

    ///
    /// Fortran like, column wise
    pub fn new<'a, 'b>(m: usize, n: usize, data: Vec<T>) -> Self
    {
        assert_eq!(m * n, data.len());
        Matrix{m: m, n: n, data: data}
    }
}

impl<T> Matrix<T>
    where T: Number + Clone + Copy + Zero + One
{
    pub fn new_random(m: usize, n: usize) -> Matrix<T>
    {
        let mut rng = rand::thread_rng();
        let data: Vec<T> = vec![T::from_f64(rng.gen()).unwrap() ; m * n];
        Matrix::new(m, n, data)
    }
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
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = &a + &Matrix::zero(2, 2);
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn zero(m: usize, n: usize) -> Self
    {
        Matrix {
            m: m,
            n: n,
            data: vec![T::zero(); m * n],
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
    /// let a: Matrix<f64> = Matrix::new(2, 2, vec![1.0, 0.0, 3.0, -7.0]);
    /// let b: Matrix<f64> = &a * &Matrix::one(2);
    ///
    /// assert_eq!(a, b);
    /// ```
    pub fn one(size: usize) -> Self
    {
        let mut data = vec![T::zero(); size * size];

        for i in 0..size
        {
            data[i * size + i] = T::one();
        }

        Matrix
        {
            m: size,
            n: size,
            data: data,
        }
    }

    pub fn ones(m: usize, n: usize) -> Self
    {
        Matrix
        {
            m: m,
            n: n,
            data: vec![T::one(); m * n]
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
    /// let a: Matrix<f64> = Matrix::new(4, 2, vec![1.0, 0.0, 3.0, 0.0, 1.0, -7.0, 0.5, 0.25]);
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
