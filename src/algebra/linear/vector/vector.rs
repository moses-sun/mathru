//! Vector
//!
//!

use algebra::linear::Matrix;
use elementary::{Exponential, Power};
use std::ops::{Add, AddAssign, Mul, Sub, Div};
use algebra::abstr::{Zero, One};
use algebra::abstr::{Real};
use algebra::abstr::cast::FromPrimitive;
use std::fmt::Display;
use std::fmt;
use serde::{Serialize, Deserialize};

#[cfg(default = "lapack")]
extern crate blas;
extern crate openblas_src;




/// Macro to construct vectores
///
/// ```
/// #[macro_use]
/// extern crate mathru;
/// fn main()
/// {
///     use mathru::algebra::linear::Vector;
///
///     // Construct a 2x3 matrix of f32
///     let mat: Vector<f64> = vector![1.0; 2.0; 3.0];
/// }
/// ```
#[macro_export]
macro_rules! vector
{
    ($( $x: expr ),*) =>
    {
        {
            let data = [ $($x),* ];
            let rows = data.len();
            let data_array: Vec<_> = data.into_iter()
                .cloned()
                .collect();
            Vector::new_row(rows, data_array)
        }
    };

    ($( $x: expr );*) =>
    {
        {
            let data = [ $($x),* ];
            let cols = data.len();
            let data_vec: Vec<_> = data.into_iter()
                .cloned()
                .collect();
            Vector::new_column(cols, data_vec)
        }
    };
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector<T>
{
    data: Matrix<T>
}


impl<T> Vector<T>
    where T: AddAssign + Mul<T, Output = T> + Zero + One + Clone + Exponential + Div<T, Output = T> + Power + PartialOrd
{
    /// Computes the p norm
    ///
    /// # Arguments
    ///
    /// p >= 1.0
    ///
    /// # Panics
    ///
    /// p < 1.0
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 0.0, 3.0, -2.0]);
	/// let p: f64 = 2.0;
	/// let norm_ref: f64 = a.eucl_norm();
    /// let norm: f64 = a.p_norm(&p);
    ///
    /// assert_eq!(norm_ref, norm);
    /// ```
    pub fn p_norm<'a, 'b>(self: &'a Self, p: &'b T) -> T
    {
        assert!(*p >= T::one());

        let a: Self = (*self).clone();
        let (m, n): (usize, usize) = a.dim();
        let mut sum: T = T::zero();
        for i in 0..(m *n)
        {
            let b : T = (*a.get(&i)).clone();
            sum += b.clone().pow(p);
        }
        let norm: T = sum.pow(&(T::one() / p.clone()));
        norm
    }
}

impl<T> Vector<T>
    where T: Power + Zero + One + Exponential + AddAssign + Add<T, Output = T> + Clone + Copy + FromPrimitive +
    Div<T, Output = T> + PartialOrd
{
    /// Computes the euclidean norm
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 0.0, 3.0, -2.0]);
	/// let norm_ref: f64 = 3.7416573867739413;
    /// let norm: f64 = a.eucl_norm();
    ///
    /// assert_eq!(norm_ref, norm);
    /// ```
    pub fn eucl_norm<'a, 'b>(self: &'a Self) -> T
    {
        let exp: T = T::from_f64(2.0).unwrap();

        return self.p_norm(&exp);
    }
}

impl <T> Vector<T>
    where T: Clone + Copy + Zero + One
{

    /// Returns a row vector
    ///
    /// # Panics
    ///
    /// n != data.len()
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_row(4, vec![1.0, 0.0, 3.0, -2.0]);
    ///
    /// ```
    pub fn new_row<'a, 'b>(n: usize, data: Vec<T>) -> Self
    {
        assert_eq!(n, data.len());
        Vector
        {
            data: Matrix::new(1, n, data)
        }
    }

    /// Returns a column vector
    ///
    /// # Panics
    ///
    /// m != data.len()
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 0.0, 3.0, -2.0]);
    ///
    /// ```
    pub fn new_column(m: usize, data: Vec<T>) -> Self
    {
        assert_eq!(m, data.len());
        Vector
        {
            data: Matrix::new(m, 1, data)
        }
    }

    pub fn apply(mut self: Vector<T>, f: &Fn(&T) -> T) -> Self
    {
        self.data = self.data.apply(f);
        self
    }
}

impl<T> Vector<T>
    where T: Real
{

    /// Returns the transposed vector
    ///
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 0.0, 3.0, -2.0]);
    /// let b: Vector<f64> = a.transpose();
    /// ```
    pub fn transpose<'a>(self: &'a Self) -> Self
    {
        Vector {
            data: self.data.transpose()
        }
    }

}

impl<T> Vector<T>
    where T: Real
{
    /// Computes the dot product of two vectors
    ///
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 0.0, 3.0, -2.0]);
	/// let b: Vector<f64> = Vector::new_column(4, vec![-1.0, 2.0, 3.0, 5.0]);
	/// let dotp_ref: f64 = -2.0;
	///
	/// let dotp: f64 = a.dotp(&b);
    ///
    /// assert_eq!(dotp_ref, dotp);
    /// ```
    pub fn dotp<'a, 'b>(self: &'a Self, rhs: &'b Self) -> T
    {
        let temp : Vector<T> = self.transpose();
        let res : Matrix<T> = (&temp.data).mul( &(rhs.data));
        (*res.get(&0, &0)).clone()
    }
}

impl<T> Vector<T>
    where T: Zero + Mul<T, Output = T> + Clone + One + Display
{
    /// Computes the dyadic product of two vectors
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector, Matrix};
	///
	/// let a: Vector<f64> = Vector::new_row(4, vec![1.0, 0.0, 3.0, -2.0]);
	/// let b: Vector<f64> = Vector::new_column(4, vec![-1.0, 2.0, 3.0, 5.0]);
	///
	/// let m: Matrix<f64> = a.dyadp(&b);
    /// ```
    pub fn dyadp<'a, 'b>(self: &'a Self, rhs: &'b Self) -> Matrix<T>
    {
        let (x_m, _x_n) : (usize, usize) = self.dim();
        let (y_m, _y_n) : (usize, usize) = rhs.dim();
        let mut c : Matrix<T> = Matrix::zero(x_m, y_m);

        for i in 0..x_m
        {
            for j in 0..y_m
            {
                *c.get_mut(&i, &j) = self.get(&i).clone() * rhs.get(&j).clone();
            }
        }
        c
    }
}
//
//impl<T> Vector<T>
//    where T: Field<T>
//{
//    pub fn crossp<'a, 'b>(self: &'a Self, rhs: &'b Self) -> Vector<T>
//    {
//        unimplemented!()
//    }
//}




impl<T> Vector<T>
    where T: One + Zero
{
    /// Returns the mutual component
    ///
    /// # Arguments
    ///
    /// # Panics
    ///
    /// if i out of bounds
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let mut a: Vector<f64> = Vector::new_row(4, vec![1.0, 0.0, 3.0, -2.0]);
	///
	/// *a.get_mut(&1) = -4.0;
    /// ```
    pub fn get_mut<'a, 'b>(self: &'a mut Self, i: &'b usize) -> &'a mut T
    {
        let (m, n) : (usize, usize) = self.data.dim();
        assert!(m == 1 || n == 1);

        if m == 1
        {
            //row vector
            assert!(*i < n);
            self.data.get_mut(&0, i)
        }
        else
        {
            //column vector
            assert!(*i < m);
            self.data.get_mut(i, &0)
        }

    }
}

impl<T> Vector<T>
    where T: One + Zero
{
    /// Returns the component
    ///
    /// # Panics
    ///
    /// if i out of bounds
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let mut a: Vector<f64> = Vector::new_row(4, vec![1.0, 0.0, 3.0, -2.0]);
	///
	/// assert_eq!(-2.0, *a.get_mut(&3))
    /// ```
    pub fn get<'a, 'b>(self: &'a Self, i: &'b usize) -> &'a T
    {
        let (m, n) : (usize, usize) = self.data.dim();
        assert!(m == 1 || n == 1);

        if m == 1
        {
            //row vector
            assert!(*i < n);
            self.data.get(&0, i)
        }
        else
        {
            //column vector
            assert!(*i < m);
            self.data.get(i, &0)
        }
    }
}

impl<T> Vector<T>
    where T: Real
{
    pub fn reflector<'a>(self: &Self) -> Vector<T>
    {
        let two = T::one() + T::one();
        let mut x_temp: Vector<T> = self.clone();

        let norm_x: T = self.p_norm(&two);

        *x_temp.get_mut(&0) += self.get(&0).sgn() * norm_x;

        *x_temp.get_mut(&0) /= x_temp.p_norm(&two);

        x_temp
    }
}


impl<T> Vector<T>
    where T: Zero + Clone + Copy
{
    /// Returns the zero vector
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![0.0, 0.0, 0.0, 0.0]);
	/// let b: Vector<f64> = Vector::zero(4);
	///
	/// assert_eq!(a, b)
    /// ```
    pub fn zero(m: usize) -> Self
    {
        Vector
        {
           data: Matrix::zero(m, 1)
        }
    }
}
////
////impl<T> One for Vector<T>
////    where T: One
////{
////    fn one<'a>() -> Self
////    {
////        unimplemented!()
////    }
////}
//


impl<T> Vector<T>
{
    /// Returns the vector dimension
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
	/// let (m, n): (usize, usize) = a.dim();
	/// assert_eq!(4, m);
	/// assert_eq!(1, n);
    /// ```
    pub fn dim(&self) -> (usize, usize)
    {
        self.data.dim()
    }
}

impl<T> Vector<T>
    where T: Real
{
    /// Returns a slice of the vector
    ///
    /// # Arugments
    ///
    /// 0 <= s < m \
    /// 0 <= e < m \
    ///
    /// s: start \
    /// e: end \
    ///
    /// # Panics
    ///
    /// iff s and e are out of bounds
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Vector};
    ///
    /// let mut a: Vector<f64> = Vector::new_column(4, vec![1.0, -2.0, 3.0, -7.0]);
    /// a = a.get_slice(1, 2);
    ///
    /// let a_ref: Vector<f64> = Vector::new_column(2, vec![-2.0, 3.0]);
    ///
    /// assert_eq!(a_ref, a);
    /// ```
    pub fn get_slice(self: &Self, s: usize, e: usize) -> Vector<T>
    {
        let (m, n): (usize, usize) = self.dim();
        if m == 1
        {
            println!("n: {}, s: {}, e: {}", n, s, e);
            assert!(s < n);
            assert!(e < n);
        }
        else
        {
            assert!(s < m);
            assert!(e < m);
        }

        let mut slice: Vector<T> = Vector::zero((e - s + 1));

        for r in s..(e + 1)
        {
            *slice.get_mut(&(r - s)) = *self.get(&r)
        }

        return slice;
    }

    /// Overwrite a slice of the vector with the given values
    ///
    /// # Arugments
    ///
    /// 0 <= s < m \
    ///
    /// s: start \
    ///
    /// # Example
    ///
    /// ```
    /// extern crate mathru;
    /// use mathru::algebra::linear::{Vector};
    ///
    /// let mut a: Vector<f64> = Vector::new_column(4, vec![1.0, -2.0, 3.0, -7.0]);
    /// let b: Vector<f64> = Vector::new_column(2, vec![-5.0, 4.0]);
    /// a.set_slice(&b, 1);
    ///
    /// let a_ref: Vector<f64> = Vector::new_column(4, vec![1.0, -5.0, 4.0, -7.0]);
    ///
    /// assert_eq!(a_ref, a);
    /// ```
    pub fn set_slice<'a>(self: &mut Self, rhs: &Self, s: usize)
    {
        let (m, _n): (usize, usize) = self.dim();
        let (s_m, _s_n): (usize, usize) = rhs.dim();
        assert!(s + s_m <= m);

        for r in s..(s + s_m)
        {
            *self.get_mut(&r) = *rhs.get(&(r - s));
        }
    }
}

//impl<T> Vector<T>
//{
//    pub fn get_mut<'a, 'b>(self: &'a mut Self, i: &'b usize) -> &'a mut T
//    {
//        assert!(*i < self.m);
//        & mut(self.data[i * self.n + j])
//    }
//}
//
//impl<T> Matrix<T>
//{
//    ///x = self_ij
//    pub fn get<'a, 'b, 'c>(self: &'a Self, i: &'b usize) -> &'a T
//    {
//        assert!(*i < self.m);
//
//        & self.data[i * self.n + j]
//    }
//}


impl<T> PartialEq for Vector<T>
    where T: Real
{
    /// Compares if two vectors are equal
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![0.0, 0.0, 0.0, 0.0]);
	/// let b: Vector<f64> = Vector::zero(4);
	///
	/// assert_eq!(true, a.eq(&b))
    /// ```
    fn eq<'a, 'b>(self: &'a Self, other: &'b Self) -> bool
    {
        if self.data == other.data
        {
            return true;
        }
        false
    }
}

impl<T> Display for Vector<T>
    where T: Display
{
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result
    {
        self.data.fmt(f)
    }
}


impl <T> Add for Vector<T>
    where T: Add<T, Output = T> + Mul<T, Output = T> + Zero + Clone + Copy
{
    type Output = Vector<T>;

    /// Adds two vectors
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
	/// let b: Vector<f64> = Vector::new_column(4, vec![3.0, -4.0, 5.0, 4.0]);
	/// let res_ref: Vector<f64> = Vector::new_column(4, vec![4.0, -2.0, 8.0, 8.0]);
	///
	/// assert_eq!(res_ref, a + b)
    /// ```
    fn add(self: Self, rhs: Self) -> Self::Output
    {
        (&self).add(&rhs)
    }
}

//c = a + b, a,b,c E T^m
impl<'a, 'b, T> Add<&'b Vector<T>> for &'a Vector<T>
    where T: Add<T, Output = T> + Zero + Clone + Copy
{
    type Output = Vector<T>;

    /// Adds two vectors
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
	/// let b: Vector<f64> = Vector::new_column(4, vec![3.0, -4.0, 5.0, 4.0]);
	/// let res_ref: Vector<f64> = Vector::new_column(4, vec![4.0, -2.0, 8.0, 8.0]);
	///
	/// assert_eq!(res_ref, &a + &b)
    /// ```
    fn add(self: Self, rhs: &'b Vector<T>) -> Self::Output
    {
        Vector
        {
            data: (&self.data).add(&rhs.data)
        }
    }
}

//c = a - b , a,b,c E T^m
impl <T> Sub for Vector<T>
    where T: Sub<Output = T> + Zero + Clone + Copy
{
    type Output = Vector<T>;

    /// Subtracts two vectors
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
	/// let b: Vector<f64> = Vector::new_column(4, vec![3.0, -4.0, 5.0, 4.0]);
	/// let res_ref: Vector<f64> = Vector::new_column(4, vec![-2.0, 6.0, -2.0, 0.0]);
	///
	/// assert_eq!(res_ref, a - b)
    /// ```
    fn sub(self: Self, rhs: Vector<T>) -> Self::Output
    {
        (&self).sub(&rhs)
    }
}

//c = a - b , a,b,c E T^m
impl <'a, 'b, T> Sub<&'b Vector<T>> for &'a Vector<T>
    where T: Sub<T, Output = T> + Zero + Clone + Copy
{
    type Output = Vector<T>;

    /// Subtracts two vectors
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
	/// let b: Vector<f64> = Vector::new_column(4, vec![3.0, -4.0, 5.0, 4.0]);
	/// let res_ref: Vector<f64> = Vector::new_column(4, vec![-2.0, 6.0, -2.0, 0.0]);
	///
	/// assert_eq!(res_ref, &a - &b)
    /// ```
    fn sub(self: Self, rhs: &'b Vector<T>) -> Self::Output
    {
        Vector
        {
            data : (&self.data).sub(&rhs.data)
        }
    }
}

impl<T> Vector<T>
    where T: Mul<T, Output = T> + Zero + Clone
{
    #[deprecated(since = "0.1.1")]
    /// Multiplies the vector with a scalar
    ///
    /// # Example
	///
	/// ```
	/// extern crate mathru;
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, -2.0, 3.0, 4.0]);
	/// let res_ref: Vector<f64> = Vector::new_column(4, vec![-2.0, 4.0, -6.0, -8.0]);
	///
	/// assert_eq!(res_ref, a.mul_scalar(&-2.0));
    /// ```
    pub fn mul_scalar<'a, 'b>(self: &'a Self, rhs: &'b T) -> Vector<T>
    {
        Vector
        {
            data: self.data.mul_scalar(rhs)
        }
    }
}

//
// a, b, c E R^n
// c = a*b^T
//impl<T> Mul<Vector<T>> for Vector<T>
//    where T: Field
//{
//    type Output = Matrix<T>;
//    fn mul(self: Self, rhs: Vector<T>) -> Self::Output
//    {
//        &(self.data) * &(rhs.data)
//   }
//}



impl<T>  Mul<T> for Vector<T>
  where T: Copy + Zero + Mul<T, Output = T> + Add<T, Output = T>
{
    type Output = Vector<T>;

    fn mul(self: Self, rhs: T) -> Self::Output
    {
        &self * &rhs
    }
}

impl<'a, 'b, T> Mul<&'b T> for &'a Vector<T>
    where T: Copy + Zero + Mul<T, Output = T> + Add<T, Output = T>
{
    type Output = Vector<T>;

    fn mul(self: Self, rhs: &'b T) -> Self::Output
    {
        Vector
        {
            data: &self.data * (rhs)
        }
    }
}


impl<T>  Mul<Matrix<T>> for Vector<T>
  where T: Copy + Zero + Mul<T, Output = T> + Add<T, Output = T> + One + Display
{
    type Output = Vector<T>;

    fn mul(self: Self, rhs: Matrix<T>) -> Self::Output
    {
        &self * &rhs
    }
}

impl<'a, 'b, T> Mul<&'b Matrix<T>> for &'a Vector<T>
    where T: Copy + Zero + Mul<T, Output = T> + Add<T, Output = T> + One + Display
{
    type Output = Vector<T>;

    fn mul(self: Self, rhs: &'b Matrix<T>) -> Self::Output
    {
        let (rhs_m, _rhs_n): (usize, usize) = rhs.dim();
        let (_m, n): (usize, usize) = self.dim();

        if n != rhs_m
        {
            panic!("Vector and matrix dimension do not match");
        }

        let mut res: Vec<T> = Vec::with_capacity(n);

        for i in 0..n
        {
            let mut sum: T = T::zero();
            for k in 0..n
            {
                sum = sum + *self.data.get(&0, &k) * *rhs.get(&k, &i);
            }
            res.push(sum.clone());
        }

        Vector::new_row(n, res)
    }
}
