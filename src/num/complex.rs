use algebra::abstr::identity::{Zero, One};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};
use std::fmt;
use std::fmt::{Display};


#[macro_export]
macro_rules! Complex
{
    ($r:expr, $i:expr) =>
    {
		(Complex::new($r, $i));
    };
}



pub type Complex32 = Complex<f32>;
pub type Complex64 = Complex<f64>;


#[derive(Debug)]
pub struct Complex<T>
{
	real: T,
	imag: T,
}

impl<T> Complex<T>
{
	pub fn new(real: T, imag: T) -> Complex<T>
	{
		Complex
		{
			real: real,
			imag: imag
		}
	}
}

//impl<T> Complex for Complex<T>
//{
//
//
//}
//
//impl<T> Number for Complex<T>
//{
//
//}

impl<T> PartialEq for Complex<T>
    where T: PartialEq
{
    fn eq<'a, 'b>(self: &'a Self, rhs: &Self) -> bool
    {
        if self.real == rhs.real && self.imag == rhs.imag
        {
            return true;
        }
        false
    }
}

impl<T> Display for Complex<T>
	where T: Display
{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl<T> Zero for Complex<T>
	where T: Zero
{
	 fn zero() -> Self
	 {
	 	Complex
		{
			real: T::zero(),
			imag: T::zero(),
		}
	 }
}

impl<T> Add for Complex<T>
	where T: Add<T, Output = T>
{
    type Output = Complex<T>;

    fn add(self, rhs: Complex<T>) -> Complex<T>
    {
        Complex
        {
        	real: self.real + rhs.real,
        	imag: self.imag + rhs.imag,
        }
    }
}


impl<T> One for Complex<T>
    where T: One + Zero + Sub<T, Output = T> + Copy
{
    fn one() -> Self
    {
        Complex
            {
                real: T::one(),
                imag: T::zero()
            }
    }
}

impl<T> Mul for Complex<T>
    where T: Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Clone + Copy
{
    type Output = Complex<T>;

    fn mul(self: Self, other: Self) -> Self::Output
    {
        Complex
       	{
        	real : self.real * other.real - self.imag * other.imag,
            imag : self.imag * other.real + self.real * other.imag
       	}
    }
}

impl<'a, 'b, T> Mul<&'b Complex<T>> for &'a Complex<T>
    where T: Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Clone + Copy
{
    type Output = Complex<T>;

    fn mul(self: Self, rhs: &'b Complex<T>) -> Self::Output
    {
        Complex
       	{
        	real: self.real * rhs.real - self.imag * rhs.imag,
           	imag: self.imag * rhs.real + self.real * rhs.imag
       	}
    }
}

impl<T> Sub for Complex<T>
    where T: Sub<T, Output = T> + Clone + Copy
{
    type Output = Complex<T>;

    fn sub(self: Self, rhs: Self) -> Self::Output
    {
        &self - &rhs
    }
}

impl<'a, 'b, T> Sub<&'b Complex<T>> for &'a Complex<T>
    where T: Sub<T, Output = T> + Clone + Copy
{
    type Output = Complex<T>;

    fn sub(self: Self, rhs: &'b Complex<T>) -> Self::Output {
        Complex
            {
                real: self.real - rhs.real,
                imag: self.imag - rhs.imag
            }
    }
}

impl<T> SubAssign for Complex<T>
    where T: SubAssign + Clone + Copy
{
    fn sub_assign<'a>(self: &'a mut Self, other: Self)
    {
        self.real -= other.real;
        self.imag -= other.imag;
    }
}
