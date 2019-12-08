//! Vector
//!
//!

use crate::algebra::linear::Matrix;
use crate::elementary::{Exponential, Power};
use std::ops::{Add, AddAssign, Mul, Sub, Div, Neg};
use crate::algebra::abstr::{Zero, One, Sign};
use crate::algebra::abstr::{Real, Scalar};
use crate::algebra::abstr::cast::FromPrimitive;
use std::fmt::Display;
use std::fmt;
use serde::{Serialize, Deserialize};
use std::iter::IntoIterator;
use super::VectorIntoIterator;
use super::VectorIterator;
use super::VectorIteratorMut;

/// Macro to construct vectors
///
/// ```
/// #[macro_use]
/// extern crate mathru;
/// fn main()
/// {
///     use mathru::algebra::linear::Vector;
///
///     // Construct a column vector of f64
///     let v1: Vector<f64> = vector![1.0; 2.0; 3.0];
///     // Construct a row vector of f32
///     let v2: Vector<f32> = vector![2.0, 3.0, 4.0];
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

impl<T> IntoIterator for Vector<T>
    where T: Real
{
    type Item = T;
    type IntoIter = VectorIntoIterator<T>;

    fn into_iter(self: Self) -> Self::IntoIter
    {
        VectorIntoIterator
        {
            //_phantom: PhantomData::default()//
            iter: self.data.into_iter(),
        }
    }
}

//impl<T> FromIterator for Matrix<T>
//    where T: Real
//{
//    fn from_iter<T>(iter: T) -> Se
//    T: IntoIterator<Item = A>,
//}

impl<T> Vector<T>
{
    pub fn iter(self: &Self) -> VectorIterator<T>
    {
        VectorIterator
        {
            iter: self.data.iter()
        }
    }

    pub fn iter_mut(self: &mut Self) -> VectorIteratorMut<T>
    {
        VectorIteratorMut
        {
            iter: self.data.iter_mut()
        }
    }
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
            let b : T = (*a.get(i)).clone();
            sum += b.clone().pow(p);
        }
        let norm: T = sum.pow(&(T::one() / p.clone()));
        norm
    }
}

impl<T> Neg for Vector<T>
    where T: Real
{
    type Output = Vector<T>;

    fn neg(self: Self) -> Self::Output
    {
        return self.apply(&|&x| { -x});
    }
}

impl<T> Vector<T>
{
    pub fn convert_to_vec(self: Self) -> Vec<T>
    {
        return self.data.convert_to_vec();
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

    pub fn apply(mut self: Vector<T>, f: &dyn Fn(&T) -> T) -> Self
    {
        self.data = self.data.apply(f);
        self
    }
}

impl <T> Vector<T>
    where T: Scalar + Clone + Copy + Zero + One
{
    /// Returns a row vector initialized with random numbers
    ///
    /// # Example
	///
	/// ```
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_row_random(4);
    ///
    /// ```
    pub fn new_row_random(n: usize) -> Self
    {
        Vector
        {
            data: Matrix::new_random(1, n)
        }
    }

    /// Returns a column vector initialized with random numbers
    ///
    /// # Example
	///
	/// ```
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column_random(4);
    ///
    /// ```
    pub fn new_column_random(m: usize) -> Self
    {
        Vector
        {
            data: Matrix::new_random(m, 1)
        }
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
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 0.0, 3.0, -2.0]);
    /// let b: Vector<f64> = a.transpose();
    /// ```
    pub fn transpose(mut self: Self) -> Self
    {
        self.data = self.data.transpose();

        return self;
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
        let (lhs_m, lhs_n) = self.dim();
        let (rhs_m, rhs_n) = rhs.dim();
        assert!(lhs_m != 0);
        assert!(lhs_n == 1);
        assert_eq!(lhs_m,rhs_m);
        assert_eq!(lhs_n, rhs_n);

        let temp : Vector<T> = self.clone().transpose();
        let res : Matrix<T> = (&temp.data).mul( &(rhs.data));

        return (*res.get(0, 0)).clone();
    }


    /// Find the argmax of the vector.
    ///
    /// Returns the index of the largest value in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a = Vector::new_column(4, vec![1.0, 2.0, -3.0, 5.0]);
    /// let idx = a.argmax();
    /// assert_eq!(idx, 3);
    /// ```
    pub fn argmax(self: &Self) -> usize
    {
        let (m, n) = self.dim();

        let mut max_index: usize = 0;
        let mut max = *self.get(max_index);;

        let limit: usize = m.max(n);

        assert!(limit != 0);

        for idx in 0..limit
        {
            let element: T = *self.get(idx);
            if  element > max
            {
                max_index = idx;
                max = element;
            }
        }

        return max_index;
    }

    /// Find the argmin of the vector.
    ///
    /// Returns the index of the smallest value in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a = Vector::new_column(4, vec![1.0, -2.0, -6.0, 75.0]);
    /// let b = a.argmin();
    /// assert_eq!(b, 2);
    /// ```
    pub fn argmin(&self) -> usize
    {
        let (m, n) = self.dim();

        let mut min_index: usize = 0;
        let mut min: T = *self.get(min_index);;

        let limit: usize = m.max(n);

        assert!(limit != 0);

        for idx in 0..limit
        {
            let element: T = *self.get(idx);
            if  element < min
            {
                min_index = idx;
                min = element;
            }
        }

        return min_index;
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
                *c.get_mut(i, j) = self.get(i).clone() * rhs.get(j).clone();
            }
        }
        c
    }
}

impl<T> Vector<T>
    //where T: One + Zero
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
	/// use mathru::algebra::linear::{Vector};
	///
	/// let mut a: Vector<f64> = Vector::new_row(4, vec![1.0, 0.0, 3.0, -2.0]);
	///
	/// *a.get_mut(1) = -4.0;
    /// ```
    pub fn get_mut<'a>(self: &'a mut Self, i: usize) -> &'a mut T
    {
        let (m, n) : (usize, usize) = self.data.dim();
        assert!(m == 1 || n == 1);

        if m == 1
        {
            //row vector
            assert!(i < n);
            self.data.get_mut(0, i)
        }
        else
        {
            //column vector
            assert!(i < m);
            self.data.get_mut(i, 0)
        }

    }
}

impl<T> Vector<T>
    //where T: One + Zero
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
	/// use mathru::algebra::linear::{Vector};
	///
	/// let mut a: Vector<f64> = Vector::new_row(4, vec![1.0, 0.0, 3.0, -2.0]);
	///
	/// assert_eq!(-2.0, *a.get_mut(3))
    /// ```
    pub fn get<'a>(self: &'a Self, i: usize) -> &'a T
    {
        let (m, n) : (usize, usize) = self.data.dim();
        assert!(m == 1 || n == 1);

        if m == 1
        {
            //row vector
            assert!(i < n);
            self.data.get(0, i)
        }
        else
        {
            //column vector
            assert!(i < m);
            self.data.get(i, 0)
        }
    }
}

impl<T> Vector<T>
    where T: Real
{
    pub fn reflector(self: &Self) -> Vector<T>
    {
        let two = T::one() + T::one();
        let mut x_temp: Vector<T> = self.clone();

        let norm_x: T = self.p_norm(&two);

        *x_temp.get_mut(0) += self.get(0).sgn() * norm_x;
        let x_temp_norm: T = x_temp.p_norm(&two);
        *x_temp.get_mut(0) /= x_temp_norm;

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

impl<T> Vector<T>
{
    /// Returns the vector dimension
    ///
    /// # Example
	///
	/// ```
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
            assert!(s < n);
            assert!(e < n);
        }
        else
        {
            assert!(s < m);
            assert!(e < m);
        }

        let mut slice: Vector<T> = Vector::zero(e - s + 1);

        for r in s..(e + 1)
        {
            *slice.get_mut(r - s) = *self.get(r)
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
            *self.get_mut(r) = *rhs.get(r - s);
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


impl<T> PartialEq<Self> for Vector<T>
    where T: Scalar
{
    /// Compares if two vectors are equal
    ///
    /// # Example
	///
	/// ```
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


impl<T> Add<Self> for Vector<T>
    where T: Real
{
    type Output = Vector<T>;

    /// Adds two vectors
    ///
    /// # Example
	///
	/// ```
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

impl<T> Add<T> for Vector<T>
    where T: Real
{
    type Output = Vector<T>;

    /// Adds a scalar to the vector
    ///
    /// # Example
	///
	/// ```
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
	/// let res_ref: Vector<f64> = Vector::new_column(4, vec![-4.0, -3.0, -2.0, -1.0]);
	///
	/// assert_eq!(res_ref, a + -5.0)
    /// ```
    fn add(mut self: Self, rhs: T) -> Self::Output
    {
        self.data = (&self.data).add(&rhs);
        return self;
    }
}

impl<'a, T> Add<&T> for &'a Vector<T>
    where T: Real
{
    type Output = Vector<T>;

    /// Adds a scalar to the vector
    ///
    /// # Example
	///
	/// ```
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
	/// let res_ref: Vector<f64> = Vector::new_column(4, vec![-4.0, -3.0, -2.0, -1.0]);
	///
	/// assert_eq!(res_ref, a + -5.0)
    /// ```
    fn add(self: Self, rhs: &T) -> Self::Output
    {
        Vector
        {
            data : (&self.data).add(rhs)
        }
    }
}


impl<T> Sub<T> for Vector<T>
    where T: Real
{
    type Output = Vector<T>;

    /// Subtracts a scalar value from all vector elements
    ///
    /// # Example
	///
	/// ```
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
	/// let res_ref: Vector<f64> = Vector::new_column(4, vec![6.0, 7.0, 8.0, 9.0]);
	///
	/// assert_eq!(res_ref, a - -5.0)
    /// ```
    fn sub(mut self: Self, rhs: T) -> Self::Output
    {
        self.data = (&self.data).sub(&rhs);
        return self;
    }
}

impl<'a, T> Sub<&T> for &'a Vector<T>
    where T: Real
{
    type Output = Vector<T>;

    /// Subtract a scalar from vector elements
    ///
    /// # Example
	///
	/// ```
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
	/// let res_ref: Vector<f64> = Vector::new_column(4, vec![-4.0, -3.0, -2.0, -1.0]);
	///
	/// assert_eq!(res_ref, a - 5.0)
    /// ```
    fn sub(self: Self, rhs: &T) -> Self::Output
    {
        Vector
        {
            data : (&self.data).sub(rhs)
        }
    }
}

impl<T> Mul<T> for Vector<T>
    where T: Real
{
    type Output = Vector<T>;

    /// Multiplies the vector elements with a scalar value
    ///
    /// # Example
	///
	/// ```
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
	/// let res_ref: Vector<f64> = Vector::new_column(4, vec![-5.0, -10.0, -15.0, -20.0]);
	///
	/// assert_eq!(res_ref, a * -5.0)
    /// ```
    fn mul(self: Self, rhs: T) -> Self::Output
    {
        Vector
        {
            data: &self.data * (&rhs)
        }
    }
}

impl<'a, T> Mul<&T> for &'a Vector<T>
    where T: Real
{
    type Output = Vector<T>;

    /// Multiplies the vector elements with the scalar value
    ///
    /// # Example
	///
	/// ```
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
	/// let res_ref: Vector<f64> = Vector::new_column(4, vec![5.0, 10.0, 15.0, 20.0]);
	///
	/// assert_eq!(res_ref, a * 5.0)
    /// ```
    fn mul(self: Self, rhs: &T) -> Self::Output
    {
        Vector
        {
            data : (&self.data).mul(rhs)
        }
    }
}

impl<T> Div<T> for Vector<T>
    where T: Real
{
    type Output = Vector<T>;

    /// Divides the vector elements with scalar values
    ///
    /// # Example
	///
	/// ```
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![-5.0, -10.0, -15.0, -20.0]);
	/// let res_ref: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
	///
	/// assert_eq!(res_ref, a / -5.0)
    /// ```
    fn div(self: Self, rhs: T) -> Self::Output
    {
        Vector
        {
            data: &self.data / (&rhs)
        }
    }
}

impl<'a, T> Div<&T> for &'a Vector<T>
    where T: Real
{
    type Output = Vector<T>;

    /// Divides the elements of a vector with the scalar value
    ///
    /// # Example
	///
	/// ```
	/// use mathru::algebra::linear::{Vector};
	///
	/// let a: Vector<f64> = Vector::new_column(4, vec![5.0, 10.0, 15.0, 20.0]);
	/// let res_ref: Vector<f64> = Vector::new_column(4, vec![1.0, 2.0, 3.0, 4.0]);
	///
	/// assert_eq!(res_ref, a / 5.0)
    /// ```
    fn div(self: Self, rhs: &T) -> Self::Output
    {
        Vector
        {
            data : (&self.data).div(rhs)
        }
    }
}

//c = a + b, a,b,c E T^m
impl<'a, 'b, T> Add<&'b Vector<T>> for &'a Vector<T>
    where T: Real
{
    type Output = Vector<T>;

    /// Adds two vectors
    ///
    /// # Example
	///
	/// ```
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
    where T: Real
{
    type Output = Vector<T>;

    /// Subtracts two vectors
    ///
    /// # Example
	///
	/// ```
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

impl <'a, 'b, T> Sub<&'b Vector<T>> for &'a Vector<T>
    where T: Real
{
    type Output = Vector<T>;

    /// Subtracts two vectors
    ///
    /// # Example
	///
	/// ```
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
                sum = sum + *self.data.get(0, k) * *rhs.get(k, i);
            }
            res.push(sum.clone());
        }

        Vector::new_row(n, res)
    }
}


impl<T> Sign for Vector<T>
    where T: Real
{
	fn sgn(self: &Self) -> Self
    {
        return (self.clone()).apply(&|x: &T| x.sgn() );
    }
}


impl<T> Vector<T>
    where T: Real
{
    pub fn compare_neighbourhood(self: &Self, b: &Self, epsilon: T) -> bool
    {
        let (self_m, self_n): (usize, usize) = self.dim();
        let (b_m, b_n): (usize, usize) = b.dim();

        if self_m != b_m || self_n != b_n
        {
            println!("dimension mismatch");
            return false;
        }

        for i in 0..self_m
        {
            if (*self.get(i) - *b.get(i)).abs() > epsilon
            {
                println!("a: {}, b: {} a-b: {}", self, b, self - b);
                return false;
            }
        }

        return true;
    }
}