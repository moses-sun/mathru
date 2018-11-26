use algebra::abstr::Number;
use algebra::abstr::identity::{Zero, One};
use algebra::abstr::Natural as NaturalT;
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign, Rem};
use std::marker::Copy;
use algebra::abstr::cast::{NumCast, ToPrimitive};
use algebra::abstr::cast;
use std::cmp::{Ordering, PartialOrd};

#[macro_export]
macro_rules! Natural
{
    ($v:expr) =>
    {
		(Natural::new($v));
    };
}



pub type Natural8 = Natural<u8>;
pub type Natural16 = Natural<u16>;
pub type Natural32 = Natural<u32>;
pub type Natural64 = Natural<u64>;

#[derive(Debug, Copy, Clone)]
pub struct Natural<T>
{
	num: T
}

impl<T> NaturalT for Natural<T>
	where T: NaturalT + ToPrimitive + NumCast + Display + PartialEq + Mul<T, Output = T> + Zero + One + Copy
{

}

impl<T> Number for Natural<T>
	where T: Number + NumCast + ToPrimitive + NaturalT + Display + PartialEq + Copy + Zero + One
{
 fn min(self: Self, a: Self) -> Self
    {
        Natural
        {
        	num: self.num.min(a.num)
        }
    }

    fn max(self: Self, a: Self) -> Self
    {
        Natural
        {
            num: self.num.max(a.num)
        }
    }
}

impl<T> Natural<T>
	where T: NaturalT + Copy + AddAssign + MulAssign
{
	pub fn new(num: T) -> Natural<T>
	{
		Natural
		{
			num: num
		}
	}

	pub fn factorial<'a>(self: &'a Self) -> Self
    {
        if *self == Natural::one() || *self == Natural::zero()
        {
       		return Natural::one();
        }

        let mut fact : T = T::one();
        let mut counter : T = T::one();

        while &counter != &self.num
        {
            counter += T::one();
            fact *= counter;
        }
        Natural::new(fact)

    }
}

impl<T> Natural<T>
    where T: Rem<Output = T> + NaturalT + Copy + Clone
{
    //
    ///Greates common divisor
    ///
    pub fn gcd<'a, 'b>(self: Self, rhs: Self) -> Self
    {
        if rhs == Self::zero()
        {
            return self;
        }
        else
        {
            return Natural::gcd(rhs, (self.rem(rhs)));
        }
    }
}

impl<T> Rem for Natural<T>
    where T: Rem<Output = T>
{
    type Output = Self;
    fn rem(self: Self, rhs: Self) -> Self::Output
    {
        Natural
        {
            num: self.num.rem(rhs.num)
        }
    }
}

impl<T> PartialEq for Natural<T>
    where T: NaturalT
{
    fn eq<'a, 'b>(self: &'a Self, rhs: &Self) -> bool
    {
        if self.num == rhs.num
        {
            return true;
        }
        false
    }
}

impl<T> PartialOrd for Natural<T>
    where T: PartialOrd + NaturalT
{
    fn partial_cmp(self: &Self, rhs: &Self) -> Option<Ordering>
    {
        self.num.partial_cmp(&rhs.num)
    }
}

impl<T> Display for Natural<T>
	where T: Display
{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.num)
    }
}

impl<T> Zero for Natural<T>
	where T: Zero
{
    // This cannot be an associated constant, because of bignums.
    fn zero() -> Self
	{
		Natural
		{
			num: T::zero()
		}
	}
}

impl<T> Add for Natural<T>
	where T: Add<T, Output = T>
{
    type Output = Natural<T>;

    fn add(self, other: Natural<T>) -> Natural<T>
    {
        Natural
        {
        	num: self.num + other.num,
        }
    }
}

impl<'a, 'b, T> Add<&'b Natural<T>> for &'a Natural<T>
    where T: Add<T, Output = T> + Copy
{
    type Output = Natural<T>;

    fn add(self: Self, rhs: &'b Natural<T>) -> Self::Output
    {
        Natural {
            num: self.num.add(rhs.num)
        }
    }
}

impl<T> AddAssign for Natural<T>
    where T: AddAssign + Clone + Copy
{
    fn add_assign(self: &mut Self, other: Self)
    {
        self.num += other.num;
    }
}

impl<T> Mul for Natural<T>
    where T: Mul<T, Output = T> + Clone + Copy
{
    type Output = Self;

    fn mul(self: Self, rhs: Self) -> Self::Output
    {
        (&self).mul(&rhs)
    }
}

impl<'a, 'b, T> Mul<&'b Natural<T>> for &'a Natural<T>
    where T: Mul<T, Output = T> + Clone + Copy
{
    type Output = Natural<T>;

    fn mul(self: Self, rhs: &'b Natural<T>) -> Self::Output
    {
        Natural
        {
            num: self.num * rhs.num
        }
    }
}


impl<T> MulAssign for Natural<T>
    where T: MulAssign
{
    fn mul_assign(self: &mut Self, other: Self)
    {
        self.num *= other.num;
    }
}

impl<T> ToPrimitive for Natural<T>
    where T: NumCast + Copy + Clone
{
    fn to_i64(self: &Self) -> Option<i64>
    {
        //self.cast()
        let v: T = self.num;
        cast::cast(v)
    }

    fn to_u64(self: &Self) -> Option<u64>
    {
        //self.cast()
        let v: T = self.num;
        cast::cast(v)
    }
}

impl<T> One for Natural<T>
    where T: One + Copy
{
    fn one() -> Self
    {
        Natural
        {
       		num: T::one()
        }
    }
}

