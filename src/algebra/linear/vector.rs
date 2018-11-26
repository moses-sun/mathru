use algebra::linear::Matrix;
//use analysis::Exponential;
use std::ops::{Add, AddAssign, Mul, Sub, Div, Rem};
use algebra::abstr::{Semiring, Field, Ring, Zero, One, Sign, Sqrt};
//use num::{Number, Natural, Real};
use std::fmt::Display;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Vector<T>
{
    data: Matrix<T>
}

impl <T> Vector<T>
    where T: Field<T>
{
    
}

impl<T> Vector<T>
    where T: AddAssign + Mul<T, Output = T> + Zero + One + Clone + Exponential + Div<T, Output = T>
{
    /// x E R^m
    ///||x||_p = (\sum_{i=1}^{m}|x_{i}|^p)^{1/p}
    pub fn p_norm<'a, 'b>(self: &'a Self, p: &'b T) -> T
    {
        let mut a : Self = (*self).clone();
        let (m, n) : (Natural<usize>, Natural<usize>) = a.dim();
        let mut sum : T = T::zero();
        for i in 0..(m.get_primitive()*n.get_primitive())
        {
            let b : T = (*a.get(&Natural::new(i))).clone();
            sum += b.clone().pow((*p).clone());
        }
        let norm: T = sum.pow(T::one()/((*p).clone()));
        norm
    }
}

impl<T> Vector<T>
    where T: Sqrt + Zero + One + Exponential + AddAssign + Add<T, Output = T> + Clone + Copy
{
    pub fn eukl_norm<'a, 'b>(self: &'a Self) -> T
    {
        let mut a : Self = (*self).clone();
        let (m, _n) : (Natural<usize>, Natural<usize>) = a.dim();
        let mut sum : T = T::zero();
        let exponent : T = T::one() + T::one();
        for i  in 0..m.get_primitive()
        {
            sum += a.get(&Natural::new(i)).pow(exponent);
        }
        sum.sqrt()
    }
}

impl <T> Vector<T>
    where T: Clone + Copy
{
    pub fn new_row<'a, 'b>(n: &'a Natural<usize>, data: &'b Vec<T>) -> Self
    {
        assert_eq!(n.get_primitive(), data.len());
        Vector
        {
            data: Matrix::new(&Natural::one(), n, data)
        } 
    }
}

impl<T> Vector<T>
    where T: Clone + Zero
{

    /// transpose
    /// A := A^T
    pub fn trans<'a>(self: &'a Self) -> Self
    {
        Vector {
            data: self.data.trans()
        }
    }

}

impl<T> Vector<T>
    where T: AddAssign + Zero + Mul<T, Output = T> + Clone
{
    /// Dot product
    ///
    /// C = <A, B> = A ° B
    pub fn dotp<'a, 'b>(self: &'a Self, rhs: &'b Self) -> T
    {
        let temp : Vector<T> = self.trans();
        let mut res : Matrix<T> = (&temp.data).mul( &(rhs.data));
        (*res.get(&Natural::zero(), &Natural::zero())).clone()
    }
}

impl<T> Vector<T>
    where T: Zero + Mul<T, Output = T> + Clone + One + Display
{
    /// Dyadic product
    /// \mathbb{R}^m \times \mathbb{R}^n -> \mathbb{R}^{m \times n}
    /// (x, y) |-> x \otimes y
    /// c_{ij} = x_{i}y_{j}
    pub fn dyadp<'a, 'b>(self: &'a Self, rhs: &'b Self) -> Matrix<T>
    {
        let mut self_temp = self.clone();
        let mut rhs_temp = rhs.clone();
        let (x_m, _x_n) : (Natural<usize>, Natural<usize>) = self.dim();
        let (y_m, _y_n) : (Natural<usize>, Natural<usize>) = rhs.dim();
        let mut c : Matrix<T> = Matrix::zero(&x_m, &y_m);

        for i in 0..x_m.get_primitive()
        {
            for j in 0..y_m.get_primitive()
            {
                *c.get_mut(&Natural::new(i), &Natural::new(j)) = (*(self_temp.get(&Natural::new(i)))).clone() * (*
                (rhs_temp
                    .get(&Natural::new(j)))).clone();
            }
        }
        c
    }
}

impl<T> Vector<T>
    where T: Field<T>
{
    pub fn crossp<'a, 'b>(self: &'a Self, rhs: &'b Self) -> Vector<T>
    {
        unimplemented!()
    }
}

impl<T> Vector<T>
    where T: One + Clone + Copy
{
    ///x E R^m
    pub fn new_column<'a, 'b>(m: &'a Natural<usize>, data: &'b Vec<T>) -> Self
    {
        assert_eq!(m.get_primitive(), data.len());
        Vector
        {
            data: Matrix::new(m, &Natural::one(), data)
        }
    }
}


impl<T> Vector<T>
    where T: One + Zero
{/// x = A_i
    pub fn get_mut<'a, 'b>(self: &'a mut Self, i: &'b Natural<usize>) -> &'a mut T
    {
        let (m, n) : (Natural<usize>, Natural<usize>) = self.data.dim();
        assert!(m == Natural::one() || n == Natural::one());

        if m == Natural::one()
        {
            //row vector
            assert!(*i < n);
            self.data.get_mut(&Natural::zero(), i)
        }
        else
        {
            //column vector
            assert!(*i < m);
            self.data.get_mut(i, &Natural::zero())
        }

    }
}

impl<T> Vector<T>
    where T: One + Zero
{/// x = A_i
    pub fn get<'a, 'b>(self: &'a Self, i: &'b Natural<usize>) -> &'a T
    {
        let (m, n) : (Natural<usize>, Natural<usize>) = self.data.dim();
        assert!(m == Natural::one() || n == Natural::one());

        if m == Natural::one()
        {
            //row vector
            assert!(*i < n);
            self.data.get(&Natural::zero(), i)
        }
        else
        {
            //column vector
            assert!(*i < m);
            self.data.get(i, &Natural::zero())
        }

    }
}

//impl<T> Semiring<T> for Vector<T>
//    where T: Semiring<T>
//{
//    fn pow<'a, 'b>(self: &'a Self, p: &'b Self) -> Self
//    {
//        unimplemented!()
//    }
//
//    fn get_primitive<'a>(self: &'a Self) -> T
//    {
//        unimplemented!()
//    }
//}
//
//
impl<T> Vector<T>
    where T: Zero + Clone + Copy
{
    pub fn zero<'a>(m: &'a Natural<usize>) -> Self
    {
        Vector
        {
           data: Matrix::zero(m, &Natural::one())
        }
    }
}
//
//impl<T> One for Vector<T>
//    where T: One
//{
//    fn one<'a>() -> Self
//    {
//        unimplemented!()
//    }
//}

///Self E R^m
///dim := m
impl<T> Vector<T>
{
    pub fn dim(&self) -> (Natural<usize>, Natural<usize>)
    {
        self.data.dim()
    }
}

impl<T> PartialEq for Vector<T>
    where T: Field<T> + PartialEq
{
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


//
//Adds two vectors
// f: T^k x T^k -> T^k
// f: self, rhs |-> self + rhs
impl <T> Add for Vector<T>
    where T: Add<T, Output = T> + Mul<T, Output = T> + Zero + Clone + Copy
{
    type Output = Vector<T>;

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
    pub fn mul_scalar<'a, 'b>(self: &'a Self, rhs: &'b T) -> Vector<T>
    {
        Vector
        {
            data: self.data.mul_scalar(rhs)
        }
    }
}

////
//// a, b, c E R^n
//// c = a*b^T
//impl<T> Mul<Vector<T>> for Vector<T>
//    where T: Field<T> + Mul<T, Output = T>
//{
//    type Output = Matrix<T>;
//    fn mul(self: Self, rhs: Vector<T>) -> Self::Output
//    {
//        &(self.data) * &(rhs.data)
//   }
//}



//impl<T>  Mul<T> for Vector<T>
//  where T: Field<T> + Mul<Output = T>
//{
//    type Output = Vector<T>;
//
//    fn mul(self: Self, rhs: T) -> Self::Output
//    {
//        &self * &rhs
//    }
//}
//
//impl<'a, 'b, T> Mul<&'b T> for &'a Vector<T>
//  where T: Field<T> + Mul<Output = T>
//{
//    type Output = Vector<T>;
//
//    fn mul(self: Self, rhs: &'b T) -> Self::Output
//    {
//        Vector
//        {
//            data: self.data * (*rhs)
//        }
//    }
//}
