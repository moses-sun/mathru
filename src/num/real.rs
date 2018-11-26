use algebra::abstr::identity::Zero;
use std::ops::{Add};
use std::fmt;
use std::fmt::{Display};
use algebra::abstr::{Number};
use algebra::abstr::Real as RealT;

#[macro_export]
macro_rules! Real
{
    ($v:expr) =>
    {
		(Real::new($v));
    };
}


pub type Real32 = Real<f32>;
pub type Real64 = Real<f64>;


#[derive(Debug)]
pub struct Real<T>
{
	num: T
}

impl<T> Real<T>
	//where T: Zero + Add<T, Output = T>
{
	pub fn new(num: T) -> Self
	{
		Real
		{
			num: num,
		}
	}
}

impl<T> PartialEq for Real<T>
    where T: PartialEq
{
    fn eq<'a, 'b>(self: &'a Self, other: &Self) -> bool
    {
        if self.num == other.num
            {
            return true;
        }
        false
    }
}

impl<T> Display for Real<T>
	where T: Display
{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.num)
    }
}

impl<T> Zero for Real<T>
	where T: Zero
{
    // This cannot be an associated constant, because of bignums.
    fn zero() -> Self
	{
		Real
		{
			num: T::zero()
		}
	}
}

impl<T> Add for Real<T>
	where T: Add<T, Output = T>
{
    type Output = Real<T>;

    fn add(self, other: Real<T>) -> Real<T>
    {
        Real
        {
        	num: self.num + other.num,
        }
    }
}

