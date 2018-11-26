//use std::clone::Clone;
use std::ops::{Add}; //, AddAssign, Mul, MulAssign, Sub, SubAssign, Neg, Div, Rem};

use algebra::abstr::Real;
use algebra::abstr::identity::{Zero, One};
//use algebra::abstr::{Semiring, Ring, Field, Zero, One, Sign, Sqrt};
//use algebra::linear::Vector;
//use analysis::{Exponential, Interval};
use num::{Natural};
//use geometry::Trigonometry;
//use std::fmt::Display;
//use std::fmt;

/// A matrix is described here
#[derive(Debug, Clone)]
pub struct Matrix<T>
{
    /// Num of rows which the matrix has
    m: Natural<usize>,
    /// Num of columns which the matrix ha
    n: Natural<usize>,
    /// Matrix entries
    data:  Vec<T>
}


//
//impl <T> Matrix<T>
//    where T: Semiring<T> + Sign + Rem + Number<T>
//{
//
//    fn gather<'a>(self: &'a Self, i: Natural<usize>, j: Natural<usize>, b: Natural<usize>) -> Natural<usize>
//    {
//        (i + Natural::new(j.get_primitive() / b.get_primitive())).rem(self.m)
//    }
//
//    fn gather_sa<'a>(self: &'a Self, i: Natural<usize>, j: Natural<usize>, a: Natural<usize>) -> Natural<usize>
//    {
//        Natural::new(j.get_primitive() + i.get_primitive()*self.n.get_primitive() - i.get_primitive()/a.get_primitive
//        ()).rem(self.m)
//
//    }
//
//    fn scatter_da<'a>(self: &'a Self, i: Natural<usize>, j: Natural<usize>, b: Natural<usize>) -> Natural<usize>
//    {
//        ((Natural::new(i.get_primitive() + j.get_primitive() / b.get_primitive())).rem(self.m) + j * self.m).rem
//        (self.n)
//    }
//}
//
//impl<T> Matrix<T>
//    where T: Clone + Zero
//{
//    ///
//    pub fn trans(self: &Self) -> Self
//    {
//        let mut trans : Matrix<T> = Matrix::zero(&self.n, &self.m);
//
//        for j in Interval::range(Natural::zero(), self.n)
//        {
//            for i in Interval::range(Natural::zero(), self.m)
//            {
//                *trans.get_mut(&j, &i) = self.get(&i, &j).clone()
//            }
//        }
//
//        trans
//    }
//}
//
//impl<T> Matrix<T>
//    where T: Real//Number<T> + Semiring<T> + Sign + Rem
//{
////
////    /// Function to transpose a matrix
////    ///
////    ///catanzaro.name/papers/PPoPP-2014.pdf
////    pub fn trans_inplace<'a>(self: &'a mut Self)
////    {
////
////    let gcdiv : Natural<usize> = self.m.gcd(&self.n);
////
////    let a : Natural<usize> = Natural::new(self.m.get_primitive()/gcdiv.get_primitive());
////    let b : Natural<usize> = Natural::new(self.n.get_primitive()/gcdiv.get_primitive());
////
////    let bigger : Natural<usize> = self.m.max(self.n);
////
////    let mut temp : Vec<T> = Vec::with_capacity(bigger.get_primitive());
////    unsafe {temp.set_len(bigger.get_primitive())};
////
////    if gcdiv > Natural::one()
////    {
////        for j in 0..self.n.get_primitive()
////        {
////            for i in 0..self.m.get_primitive()
////            {
////                temp[i] = self.data[self.gather(Natural::new(i), Natural::new(j), b).get_primitive() * self.n.get_primitive() + j];
////            }
////
////            for i in 0..self.m.get_primitive()
////            {
////                self.data[i * self.n.get_primitive() + j] = temp[i];
////            }
////        }
////    }
////
////    for i in 0..self.m.get_primitive()
////    {
////        for j in 0..self.n.get_primitive()
////        {
////            temp[self.scatter_da(Natural::new(i), Natural::new(j), b).get_primitive()] = self.data[i * self.n
////                .get_primitive() + j];
////        }
////
////        for j in 0..self.n.get_primitive()
////        {
////            self.data[i * self.n.get_primitive() + j] = temp[j];
////        }
////    }
////
////    for j in 0..self.n.get_primitive()
////    {
////        for i in 0..self.m.get_primitive()
////        {
////            temp[i] =  self.data[self.gather_sa(Natural::new(i), Natural::new(j), a).get_primitive() * self.n.get_primitive() + j];
////        }
////
////        for i in 0..self.m.get_primitive()
////        {
////            self.data[i * self.n.get_primitive() + j] = temp[i];
////        }
////    }
////
////    let temp_m : Natural<usize> = self.m;
////    self.m = self.n;
////    self.n = temp_m;
////
////    }
////}
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
//
//
//impl<T> Matrix<T>
//{
//    pub fn det<'a>(self: &'a Self) -> T
//        where T: MulAssign + Add<Output = T> + Sub<Output = T> +  Mul<Output = T>  + Div<Output = T> + One + Zero  +
//        Clone + PartialOrd
//    {
//        assert_eq!(self.m, self.n);
//        let (l, u, p): (Matrix<T>, Matrix<T>, Matrix<T>) = self.dec_lu();
//        let mut det: T = T::one();
//
//        for i in Interval::range(Natural::zero(), self.m)
//        {
//			det *= l.get(&i, &i).clone() * u.get(&i, &i).clone();
//        }
//        det
//    }
//}
//
//impl<T> Matrix<T>
//    where T:  Add<Output = T> + Sub<Output = T> +  Mul<Output = T>  + Div<Output = T> + One + Zero + Clone + PartialOrd
//{
//    pub fn dec_lu<'a>(self: &'a Self) -> (Matrix<T>, Matrix<T>, Matrix<T>)
//    {
//        assert_eq!(self.m, self.n);
//
//        let mut l: Matrix<T> = Matrix::one(&self.m);
//        let mut u: Matrix<T> = Matrix::one(&self.n);
//        let mut p: Matrix<T> = Matrix::one(&self.m);
//
//        let mut a: Matrix<T> = self.clone();
//
//        for i in Interval::range(Natural::zero(), a.m)
//        {
//            //pivoting
//            let mut min: T = a.get(&i, &i).clone();
//            let mut i_min: Natural<usize> = i;
//            for l in Interval::range((i + Natural::one()), a.m)
//            {
//                if *a.get(&i, &l) < min
//                {
//                    min = a.get(&i, &l).clone();
//                    i_min = l;
//                }
//            }
//            if i != i_min
//            {
//                a.swap_rows(&i, &i_min);
//                p.swap_rows(&i, &i_min);
//            }
//
//            let mut upper_bound: Integer<i64> = a.m.cast();
//            upper_bound  -= Integer::one();
//            for j in Interval::range(i, upper_bound.cast())
//            {
//                let f: T = (*(a.get(&(j + Natural::one()), &i))).clone() / a.get(&i, &i).clone();
//
//                for k in Interval::range(i, a.n) //.get_primitive()
//                {
//                    *(a.get_mut(&(j + Natural::one()), &k)) =  a.get(&(j + Natural::one()), &k).clone() - f.clone() *
//                     a.get(&i, &k).clone();
//                }
//                *(a.get_mut(&(j + Natural::one()), &i)) = f.clone();
//            }
//        }
//
//        for i in 1..a.m.get_primitive()
//        {
//            for j in 0..i
//            {
//                l.data[i * a.n.get_primitive() + j] = a.data[i * a.n.get_primitive() + j].clone();
//            }
//        }
//
//        for i in 0..a.n.get_primitive()
//        {
//            for k in i..a.n.get_primitive()
//            {
//                u.data[i * a.n.get_primitive() + k] = a.data[i * a.n.get_primitive() + k].clone();
//            }
//        }
//
//        (l, u, p)
//    }
//}
//
//impl<T> Matrix<T>
//    where T: Neg<Output = T> + Div<T, Output = T>  + Mul<T, Output = T> + Add<T, Output = T> + Sqrt + Clone +
//PartialOrd + One + Zero + Sign + AddAssign
//{
//    //QR Decomposition with Givens rotations
//    pub fn dec_qr<'a>(self: &'a Self) -> (Matrix<T>, Matrix<T>)
//    {
//        if self.m < self.n
//        {
//            panic!();
//        }
//        let mut q: Matrix<T> = Matrix::one(&self.m);
//        let mut r: Matrix<T> = self.clone();
//
//        for j in Interval::range(Natural::zero(), self.n) //column
//        {
//            for i in Interval::range((j+Natural::one()), self.m) //row
//            {
//                let a_jj : T = r.get(&j, &j).clone();
//                let a_ij : T = r.get(&i, &j).clone();
//                let p : T = a_jj.sign()*((a_jj.clone() * a_jj.clone() + a_ij.clone() * a_ij.clone()).sqrt());
//                if (p != T::zero()) && (a_jj != T::zero()) && (a_ij != T::zero())
//                {
//                    let c : T = a_jj / p.clone();
//                    let s : T = a_ij / p;
//                    let g_ij : Matrix<T> = Matrix::givens(&r.m, &i, &j, &c, &-s);
//                    r = &g_ij * &r;
//                    q = &g_ij * &q;
//                }
//            }
//        }
//        q = q.trans();
//        (q, r)
//    }
//}
//
//impl<T> Matrix<T>
//    where T: Clone
//{
//    fn swap_rows<'a, 'b, 'c>(self: &'a mut Self, i: &'b Natural<usize>, j: &'c Natural<usize>)
//    {
//        for k in Interval::range(Natural::zero(), self.n)
//        {
//            let temp: T = self.get(&i, &k).clone();
//            *(self.get_mut(&i, &k)) = self.get(&j, &k).clone();
//            *(self.get_mut(&j, &k)) = temp;
//        }
//    }
//}
//
//
//

impl<T> Matrix<T>
{

    //
    // returns column vector
//    pub fn get_column<'a, 'b>(self: &'a Self, i: &'b usize) -> Vector<T>
//        where T: Zero + Copy + One
//    {
//        let mut v: Vector<T> = Vector::zero(&self.m);
//
//        for k in 0..self.m
//        {
//            *(v.get_mut(&k)) = *(self.get( &k, &i));
//        }
//
//        v
//    }

    //
    // return row vector
//    pub fn get_row<'a, 'b>(self: &'a Self, i: &'b usize) -> Vector<T>
//        where T: Zero + One + Copy
//    {
//        let mut v: Vector<T> = Vector::zero(&self.n);
//        v = v.trans();
//
//        for k in 0..self.n
//        {
//            *(v.get_mut(&k)) = *(self.get(&i, &k));
//        }
//
//        v
//    }
}
//
//impl<T> Matrix<T>
//{
//    pub fn givens<'a, 'b, 'c, 'd, 'e>(m: &'a Natural<usize>, i: &'b Natural<usize>, j: &'c Natural<usize>, c: &'d T, s: &'e T) -> Self
//        where T: Neg<Output = T> + Zero + One + Clone + PartialOrd
//    {
//        if *i >= *m || *j >= *m
//        {
//            panic!("Index out of bounds");
//        }
//        let mut givens: Matrix<T> = Matrix::one(m);
//        *(givens.get_mut(i,i)) = c.clone();
//        *(givens.get_mut(j,j)) = c.clone();
//        *(givens.get_mut(i,j)) = s.clone();
//        *(givens.get_mut(j,i)) = -s.clone();
//        givens
//    }
//}
//
//
//
//impl<T> Matrix<T>
//    where T: Mul<T, Output = T> + Sub<T, Output = T> + Zero + One + Sqrt + Clone + Copy + Add<T, Output = T>
// + Div<T, Output = T> + Exponential + AddAssign + Display
//{
//    /// calculates the householder matrix from the householder reflector v
//    ///
//    ///v:
//    pub fn householder<'a>(v: &'a Vector<T>) -> Self
//    {
//        let (v_m, v_n) : (Natural<usize>, Natural<usize>) = v.dim();
//        let ident : Matrix<T> = Matrix::one(&v_m);
//
//        let two = T::one() + T::one();
//        let lambda : T = two.sqrt() / (v.eukl_norm());
//        let v_norm : Vector<T> =  v.mul_scalar(&lambda);
//        //println!("v: {}", v_norm);
//        //println!("|v|:  {}", v_norm.eukl_norm());
//        let v_vtrans : Matrix<T> = v_norm.dyadp(&v_norm);
//        //println!("v_vtrans: {}", v_vtrans);
//        let h : Matrix<T> = ident - v_vtrans;
//        h
//    }
//}
//
//impl<T> Matrix<T>
//    where T: AddAssign + Div<T, Output = T> + Exponential + Mul<T, Output = T> + Sub<T, Output = T> + Add<T, Output =
//T> + Zero + One + Sqrt + Clone + Copy  + Display // + Number<T>
//{
//
//    pub fn dec_sv<'a>(self: &'a Self) -> (Self, Self, Self)
//    {
//        Matrix::householder_bidiag(self)
//    }
//
//    ///
//	   /// self is an m times n matrix with m >= n
//	   /// A = UBV^{T}
//	   /// U \in T^{m \times n}
//	   /// B \in T^{n \times n}
//	   /// V \in T^{n \times n}
//	   ///
//    fn householder_bidiag<'a>(self: &'a Self) -> (Self, Self, Self)
//    {
//        let (m, n) : (Natural<usize>, Natural<usize>) = self.dim();
//        if m < n
//        {
//            panic!("Read the API");
//        }
//
//        let mut u: Matrix<T> = Matrix::zero(&m, &n);
//        let mut v: Matrix<T> = Matrix::one(&n);
//        let mut b_i : Matrix<T> = Matrix::zero(&n, &n);
//        let mut a_i : Matrix<T> = self.clone();
//        let two = T::one() + T::one();
//
//        for i in Interval::range(Natural::zero(), n)
//            {
//                let u_x: Vector<T> = a_i.get_column(&i);
//                println!("u_x: {}", u_x);
//                let u_reflector : Vector<T> = Matrix::reflector(&u_x, &i);
//                let u_i : Matrix<T> = Matrix::householder(&u_reflector);
//                a_i = &u_i * &a_i;
//                println!("u_i * a: {}", a_i);
//                u = &u * &u_i;
//
//                if i.get_primitive() < (n.get_primitive()-2)
//                    {
//                        let mut v_x: Vector<T> = a_i.get_row(&i);
//                        let v_x_trans: Vector<T> = v_x.trans();
//                        let v_reflector : Vector<T> = Matrix::reflector(&v_x_trans, &(i + Natural::one()));
//                        let v_i : Matrix<T> = Matrix::householder(&v_reflector);
//
//                        v = &v * &v_i;
//                        a_i = &a_i * &v_i;
//                    }
//            }
//
//        (u, a_i, v)
//    }
//
//    ///
//    ///
//    fn reflector<'a>(x: &Vector<T>, rc_number: &Natural<usize>) -> Vector<T>
//    {
//        let p: T = (T::one() + T::one()).sqrt();
//        let two = T::one() + T::one();
//        let (m, _n): (Natural<usize>, Natural<usize>) = x.dim();
//        let mut y: Vector<T> = Vector::zero(&m);
//        let mut x_temp: Vector<T> = x.clone();
//
//        for i in Interval::range(Natural::zero(), *rc_number)
//        {
//            *y.get_mut(&i) = *(x_temp.get(&i));
//            *(x_temp.get_mut(&i)) = T::zero();
//        }
//
//        let norm_x_temp : T = x_temp.p_norm(&two);
//        *(y.get_mut(&rc_number)) = norm_x_temp;
//
//        let diff : Vector<T> = (&x).sub(&y);
//        let norm_diff : T = (&diff).p_norm(&two);
//        let f = (&p).div(norm_diff);
//        let v : Vector<T> = (&diff).mul_scalar(&f);
//        v
//    }
//}
//
//impl<T> Matrix<T>
//    where T: Zero + Clone + Copy + One
//{
//    pub fn bidiag<'a>(self: &'a Self) -> Self
//    {
//        let mut C : Matrix<T> = Matrix::zero(&self.m, &self.n);
//        for i in 0..self.m.get_primitive()
//            {
//                let A_i : Matrix<T> =  self.clone();
//                let a_i : Vector<T> = A_i.get_column(&Natural::zero());
//                //let beta_i : T  =  a_i.eukl_norm();
//            }
//
//        C
//    }
//}
//
//impl<T> Display for Matrix<T>
//    where T: Display
//{
//    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result
//    {
//        write!(f, "\n").unwrap();
//        for i in Interval::range(Natural::zero(), self.m)
//        {
//            for j in Interval::range(Natural::zero(), self.n)
//            {
//                write!(f, "{} ", self.data[(i * self.n + j).get_primitive()]).unwrap();
//            }
//            write!(f, "\n").unwrap();
//        }
//        write!(f, "\n")
//    }
//}
//
//
//impl<T> Matrix<T>
//{
//    ///x = self_ij
//    pub fn get_mut<'a, 'b, 'c>(self: &'a mut Self, i: &'b usize, j: &'c usize) -> &'a mut T
//    {
//        //print!("m: {}, n: {}, i: {}, j: {}", self.m , self.n, *i, *j);
//        assert!(*i < self.m);
//        assert!(*j < self.n);
//        & mut(self.data[i * self.n + j])
//    }
//}
//
//impl<T> Matrix<T>
//{
//    ///x = self_ij
//    pub fn get<'a, 'b, 'c>(self: &'a Self, i: &'b usize, j: &'c usize) -> &'a T
//    {
//        //print!("m: {}, n: {}, i: {}, j: {}", self.m , self.n, *i, *j);
//        assert!(*i < self.m);
//        assert!(*j < self.n);
//        & self.data[i * self.n + j]
//    }
//}

//
//
//impl<T> Matrix<T>
//    where T: Mul<T, Output = T> + Zero + Clone
//{
//    pub fn mul_scalar<'a, 'b>(self: &'a Self, rhs: &'b T) -> Matrix<T>
//    {
//        let (rows, cols): (Natural<usize>, Natural<usize>) = self.dim();
//
//        let mut prod: Matrix<T> = Matrix::zero(&rows, &cols);
//
//        for i in Interval::range(Natural::zero(), rows) //Zeile
//        {
//            for j in Interval::range(Natural::zero(), cols) //Spalte
//            {
//                *prod.get_mut(&i, &j) = self.get(&i, &j).clone() * (*rhs).clone();
//            }
//        }
//        prod
//
//    }
//}
//
//
//impl <T> Mul for Matrix<T>
//    where T: AddAssign + Zero + Mul<T, Output = T> + Clone
//{
//    type Output = Matrix<T>;
//
//    fn mul(self: Self, rhs: Self) -> Self::Output
//    {
//        (&self).mul(&rhs)
//    }
//}
//
//impl <'a, 'b, T> Mul<&'b Matrix<T>> for &'a Matrix<T>
//    where T: Mul<T, Output = T> + AddAssign + Zero + Clone
//{
//    type Output = Matrix<T>;
//
//    /// prod = self * rhs
//    fn mul(self: Self, rhs: &'b Matrix<T>) -> Self::Output
//    {
//        let (l_rows, l_cols) = self.dim();
//        let (r_rows, r_cols): (Natural<usize>, Natural<usize>) = rhs.dim();
//        assert_eq!(l_cols, r_rows);
//
//        let mut prod: Matrix<T> = Matrix::zero(&l_rows, &r_cols);
//
//        for i in Interval::range(Natural::zero(), l_rows) //Zeile
//        {
//            for j in Interval::range(Natural::zero(), r_cols) //Spalte
//            {
//                let mut sum: T = T::zero();
//                for k in Interval::range(Natural::zero(), l_cols)
//                {
//                    sum += self.get(&i, &k).clone() * rhs.get(&k, &j).clone();
//                }
//                *prod.get_mut(&i, &j) = sum;
//            }
//        }
//        prod
//    }
//}
//
//

//
//Adds two matrices
//
//impl<T> Add for Matrix<T>
//    where T: Real + Add<T, Output = T> + Zero + Clone
//{
//    type Output = Matrix<T>;
//
//    fn add(self: Self, rhs: Self) -> Self::Output
//    {
//        (&self).add(&rhs)
//    }
//}


//
//Adds two matrices
//
//impl <'a, 'b, T> Add<&'b Matrix<T>> for &'a Matrix<T>
//    where T: Real + Add<T, Output = T> + Zero + Clone + One
//{
//    type Output = Matrix<T>;
//
//    fn add(self: Self, rhs: &'b Matrix<T>) -> Self::Output
//    {
//        assert_eq!(self.dim(), rhs.dim());
//
//        let mut sum: Matrix<T> = Matrix::zero(&self.m, &self.n);
//
//        for i in 0..sum.m
//        {
//            for j in 0..sum.n
//            {
//                *sum.get_mut(&i, &j) = self.get(&i, &j).clone() + rhs.get(&i, &j).clone();
//            }
//        }
//        sum
//    }
//}

/////
/////Subs two matrices
/////
//impl <T> Sub for Matrix<T>
//    where T: Sub<Output = T> + Zero + Clone + Copy
//{
//    type Output = Matrix<T>;
//
//    fn sub(self: Self, rhs: Self) -> Self::Output
//    {
//        (&self).sub(&rhs)
//    }
//}
//
/////
/////Subs two matrices
/////
/////
//impl<'a, 'b, T> Sub<&'b Matrix<T>> for &'a Matrix<T>
//    where T: Sub<T, Output = T> + Zero + Clone + Copy
//{
//    type Output = Matrix<T>;
//
//    fn sub(self: Self, rhs: &'b Matrix<T>) -> Self::Output
//    {
//        assert_eq!(self.dim(), rhs.dim());
//        let (m, n) : (Natural<usize>, Natural<usize>) = self.dim();
//
//        let mut diff: Matrix<T> = Matrix::zero(&m, &n);
//
//        for i in Interval::range(Natural::zero(), m)
//        {
//            for j in Interval::range(Natural::zero(), n)
//            {
//                *diff.get_mut(&i, &j) = self.get(&i, &j).clone() - rhs.get(&i, &j).clone();
//            }
//        }
//        diff
//    }
//}
//
//
//impl<T> PartialEq for Matrix<T>
//    where T: PartialEq
//{
//    fn eq<'a, 'b>(self: &'a Self, other: &'b Self) -> bool
//    {
//
//        if self.dim() != other.dim()
//        {
//            return false;
//        }
//
//        if self.data == other.data
//        {
//            return true;
//        }
//
//        false
//    }
//}
//
//impl<T> Matrix<T>
//    where T: Clone + Copy
//{
//    pub fn new<'a, 'b, 'c>(m: &'a Natural<usize>, n: &'b Natural<usize>, data: &'c Vec<T>) -> Self
//    {
//        assert_eq!((*m * *n).get_primitive(), data.len());
//        Matrix{m: *m, n: *n, data: data.clone()}
//    }
//}
//
//}
//
//impl<T> Matrix<T>
//    where T: Real + Zero + Clone + One
//{
//    pub fn zero<'a, 'b>(m: &'a usize, n: &'b usize) -> Self
//    {
//        Matrix {
//            m: *m,
//            n: *n,
//            data: vec![T::zero(); *m * *n],
//        }
//    }
//}
//
//
//impl<T> Matrix<T>
//    where T: Real + Clone + One + Zero
//{
//    pub fn one<'a>(size: &'a usize) -> Self
//    {
//        let mut data = vec![T::zero(); *size * *size];
//
//        //for i in Interval::range(size.zero(), *size)
//        for i in 0..*size
//        {
//            data[i * *size + i] = T::one();
//        }
//
//        Matrix
//        {
//            m: *size,
//            n: *size,
//            data: data,
//        }
//    }
//}
//
//
//
//impl<T> Matrix<T>
//    where T: Real
//{
//    pub fn dim(&self) -> (usize, usize)
//    {
//        (self.m, self.n)
//    }
//}

