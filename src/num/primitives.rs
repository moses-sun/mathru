use algebra::abstr::{Natural, Integer, Real};
use algebra::abstr::{Number, Semiring, Ring, Sign, Abs, Field, Zero, One, Lapack};
use algebra::abstr::cast::{NumCast, FromPrimitive, ToPrimitive, AsPrimitive};
use elementary::{Exponential, Trigonometry, Power, Hyperbolic};
use num::bound::Bound;
use std::mem::size_of;
use std::{u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64};

//#[cfg(feature = "openblas")]
extern crate lapack;
extern crate blas;
extern crate openblas_src;
//use lapack;

macro_rules! number_impl
{
    ($t:ty) =>
    {
        impl Number for $t
        {
//			fn min(self: Self, a: Self) -> Self
//    		{
//    			if self <= a
//				{
//					return self
//				}
//				else
//				{
//					return a
//				}
//            }
//
//    		fn max(self: Self, a: Self) -> Self
//    		{
//    			if self >= a
//				{
//					return self
//				}
//				else
//				{
//					return a
//				}
//    		}
        }
    };
}

number_impl!(u8);
number_impl!(u16);
number_impl!(u32);
number_impl!(u64);
number_impl!(usize);
number_impl!(i8);
number_impl!(i16);
number_impl!(i32);
number_impl!(i64);
number_impl!(isize);
number_impl!(f32);
number_impl!(f64);

macro_rules! semiring_impl
{
    ($t:ty) =>
    {
        impl Semiring for $t
        {
//			fn get_primitive<'a>(self: &'a Self) -> $t
//			{
//				return *self
//			}
        }
    };
}

semiring_impl!(usize);
semiring_impl!(u8);
semiring_impl!(u16);
semiring_impl!(u32);
semiring_impl!(u64);

semiring_impl!(isize);
semiring_impl!(i8);
semiring_impl!(i16);
semiring_impl!(i32);
semiring_impl!(i64);

semiring_impl!(f32);
semiring_impl!(f64);

macro_rules! ring_impl
{
    ($t:ty) =>
    {
        impl Ring for $t
        {

        }
    };
}

ring_impl!(isize);
ring_impl!(i8);
ring_impl!(i16);
ring_impl!(i32);
ring_impl!(i64);

ring_impl!(f32);
ring_impl!(f64);

macro_rules! field_impl
{
    ($t:ty, $pi: expr) =>
    {
        impl Field for $t
        {
			fn epsilon() -> Self
			{
				$pi
			}
        }

        impl Sign for $t
		{
			fn sgn(self: &Self) -> Self
            {
                if *self < 1.0
                {
                    return -1.0;
                }
                else
                {
                    return 1.0;
                }
            }
		}

		impl Abs for $t
		{
			fn abs(self: &Self) -> Self
			{
				(*self).abs()
			}
		}
    };
}

field_impl!(f32, f32::EPSILON);
field_impl!(f64, f64::EPSILON);

macro_rules! zero_impl {
    ($t:ty, $v:expr) =>
    {
        impl Zero for $t
        {
            #[inline]
            fn zero() -> $t
            {
                $v
            }
        }
    };
}

zero_impl!(usize, 0);
zero_impl!(u8, 0);
zero_impl!(u16, 0);
zero_impl!(u32, 0);
zero_impl!(u64, 0);
#[cfg(has_i128)]
zero_impl!(u128, 0);

zero_impl!(isize, 0);
zero_impl!(i8, 0);
zero_impl!(i16, 0);
zero_impl!(i32, 0);
zero_impl!(i64, 0);
#[cfg(has_i128)]
zero_impl!(i128, 0);

zero_impl!(f32, 0.0);
zero_impl!(f64, 0.0);


macro_rules! one_impl {
    ($t:ty, $v:expr) =>
    {
        impl One for $t {
            #[inline]
            fn one() -> $t {
                $v
            }
        }
    };
}

one_impl!(usize, 1);
one_impl!(u8, 1);
one_impl!(u16, 1);
one_impl!(u32, 1);
one_impl!(u64, 1);
#[cfg(has_i128)]
one_impl!(u128, 1);

one_impl!(isize, 1);
one_impl!(i8, 1);
one_impl!(i16, 1);
one_impl!(i32, 1);
one_impl!(i64, 1);
#[cfg(has_i128)]
one_impl!(i128, 1);

one_impl!(f32, 1.0);
one_impl!(f64, 1.0);

macro_rules! natural_impl
{
	($t:ty) =>
    {
        impl Natural for $t
        {

        }
    }
}

natural_impl!(u8);
natural_impl!(u16);
natural_impl!(u32);
natural_impl!(u64);
natural_impl!(usize);

macro_rules! integer_impl
{
    ($t:ty) =>
    {
        impl Integer for $t
        {

        }
    }
}

integer_impl!(i8);
integer_impl!(i16);
integer_impl!(i32);
integer_impl!(i64);

macro_rules! real_impl
{
    ($t:ty) =>
    {
        impl Real for $t
        {
			/// Returns the smallest integer greater than or equal to a number.
			fn ceil(self: &Self) -> Self
			{
				(*self).ceil()
			}

			/// Returns the largest integer less than or equal to a number.
			fn floor(self: &Self) -> Self
			{
				(*self).floor()
			}
        }
    }
}

real_impl!(f64);
real_impl!(f32);


macro_rules! impl_to_primitive_int_to_int
{
    ($SrcT:ty, $DstT:ty, $slf:expr) => (
        {
            if size_of::<$SrcT>() <= size_of::<$DstT>()
            {
                Some($slf as $DstT)
            }
            else
            {
                let n = $slf as i64;
                let min_value: $DstT = Bound::lower_bound();
                let max_value: $DstT = Bound::upper_bound();
                if min_value as i64 <= n && n <= max_value as i64
                {
                    Some($slf as $DstT)
                }
                else
                {
                	None
               	}
            }
        }
    )
}

macro_rules! impl_to_primitive_int_to_uint
{
    ($SrcT:ty, $DstT:ty, $slf:expr) => (
        {
            let zero: $SrcT = Zero::zero();
            let max_value: $DstT = Bound::upper_bound();
            if zero <= $slf && $slf as u64 <= max_value as u64
           	{
               Some($slf as $DstT)
           	}
           	else
            {
            	None
            }
        }
    )
}

macro_rules! impl_to_primitive_int
{
    ($T:ty) => (
        impl ToPrimitive for $T
        {

            fn to_isize(&self) -> Option<isize>
            {
            	impl_to_primitive_int_to_int!($T, isize, *self)
            }


            fn to_i8(&self) -> Option<i8>
            {
            	impl_to_primitive_int_to_int!($T, i8, *self)
            }


            fn to_i16(&self) -> Option<i16>
            {
            	impl_to_primitive_int_to_int!($T, i16, *self)
            }


            fn to_i32(&self) -> Option<i32>
            {
            	impl_to_primitive_int_to_int!($T, i32, *self)
            }


            fn to_i64(&self) -> Option<i64>
            {
            	impl_to_primitive_int_to_int!($T, i64, *self)
            }


            fn to_usize(&self) -> Option<usize>
            {
            	impl_to_primitive_int_to_uint!($T, usize, *self)
            }


            fn to_u8(&self) -> Option<u8>
            {
            	impl_to_primitive_int_to_uint!($T, u8, *self)
            }


            fn to_u16(&self) -> Option<u16>
            {
            	impl_to_primitive_int_to_uint!($T, u16, *self)
            }


            fn to_u32(&self) -> Option<u32>
            {
            	impl_to_primitive_int_to_uint!($T, u32, *self)
            }


            fn to_u64(&self) -> Option<u64>
            {
            	impl_to_primitive_int_to_uint!($T, u64, *self)
            }


            fn to_f32(&self) -> Option<f32>
            {
            	Some(*self as f32)
            }


            fn to_f64(&self) -> Option<f64>
            {
            	Some(*self as f64)
            }
        }
    )
}

impl_to_primitive_int!(isize);
impl_to_primitive_int!(i8);
impl_to_primitive_int!(i16);
impl_to_primitive_int!(i32);
impl_to_primitive_int!(i64);

macro_rules! impl_to_primitive_uint_to_int {
    ($DstT:ty, $slf:expr) => (
        {
            let max_value: $DstT = Bound::upper_bound();
            if $slf as u64 <= max_value as u64
            {
                Some($slf as $DstT)
            }
            else
            {
                None
            }
        }
    )
}

macro_rules! impl_to_primitive_uint_to_uint {
    ($SrcT:ty, $DstT:ty, $slf:expr) => (
        {
            if size_of::<$SrcT>() <= size_of::<$DstT>()
            {
                Some($slf as $DstT)
            }
            else
            {
                let zero: $SrcT = Zero::zero();
                let max_value: $DstT = Bound::upper_bound();
                if zero <= $slf && $slf as u64 <= max_value as u64
                {
                    Some($slf as $DstT)
                }
                else
                {
                    None
                }
            }
        }
    )
}

macro_rules! impl_to_primitive_uint
{
    ($T:ty) => (
        impl ToPrimitive for $T
        {

            fn to_isize(&self) -> Option<isize>
            {
            	impl_to_primitive_uint_to_int!(isize, *self)
            }


            fn to_i8(&self) -> Option<i8>
            {
            	impl_to_primitive_uint_to_int!(i8, *self)
            }


            fn to_i16(&self) -> Option<i16>
            {
            	impl_to_primitive_uint_to_int!(i16, *self)
            }


            fn to_i32(&self) -> Option<i32>
            {
            	impl_to_primitive_uint_to_int!(i32, *self)
            }


            fn to_i64(&self) -> Option<i64>
            {
            	impl_to_primitive_uint_to_int!(i64, *self)
            }


            fn to_usize(&self) -> Option<usize>
            {
                impl_to_primitive_uint_to_uint!($T, usize, *self)
            }


            fn to_u8(&self) -> Option<u8>
            {
            	impl_to_primitive_uint_to_uint!($T, u8, *self)
            }


            fn to_u16(&self) -> Option<u16>
            {
            	impl_to_primitive_uint_to_uint!($T, u16, *self)
            }


            fn to_u32(&self) -> Option<u32>
            {
            	impl_to_primitive_uint_to_uint!($T, u32, *self)
            }


            fn to_u64(&self) -> Option<u64>
            {
            	impl_to_primitive_uint_to_uint!($T, u64, *self)
            }


            fn to_f32(&self) -> Option<f32>
            {
            	Some(*self as f32)
            }


            fn to_f64(&self) -> Option<f64>
            {
            	Some(*self as f64)
            }
        }
    )
}

impl_to_primitive_uint!(usize);
impl_to_primitive_uint!(u8);
impl_to_primitive_uint!(u16);
impl_to_primitive_uint!(u32);
impl_to_primitive_uint!(u64);
//impl_to_primitive_uint!(f32);
//impl_to_primitive_uint!(f64);

macro_rules! impl_to_primitive_float_to_float
{
    ($SrcT:ident, $DstT:ident, $slf:expr) => (
        if size_of::<$SrcT>() <= size_of::<$DstT>()
        {
            Some($slf as $DstT)
        }
        else
        {
            // Make sure the value is in range for the cast.
            // NaN and +-inf are cast as they are.
            let n = $slf as f64;
            let max_value: $DstT = ::std::$DstT::MAX;
            if !n.is_finite() || (-max_value as f64 <= n && n <= max_value as f64)
            {
                Some($slf as $DstT)
            }
            else
            {
                None
            }
        }
    )
}


macro_rules! impl_to_primitive_float
{
    ($T:ident) => (
        impl ToPrimitive for $T
        {

            fn to_isize(&self) -> Option<isize>
            {
            	Some(*self as isize)
            }


            fn to_i8(&self) -> Option<i8>
            {
            	Some(*self as i8)
            }


            fn to_i16(&self) -> Option<i16>
            {
            	Some(*self as i16)
            }


            fn to_i32(&self) -> Option<i32>
            {
            	Some(*self as i32)
            }


            fn to_i64(&self) -> Option<i64>
            {
            	Some(*self as i64)
            }


            fn to_usize(&self) -> Option<usize>
            {
            	Some(*self as usize)
            }


            fn to_u8(&self) -> Option<u8>
            {
            	Some(*self as u8)
            }


            fn to_u16(&self) -> Option<u16>
            {
            	Some(*self as u16)
            }


            fn to_u32(&self) -> Option<u32>
            {
            	Some(*self as u32)
            }


            fn to_u64(&self) -> Option<u64>
            {
            	Some(*self as u64)
            }


            fn to_f32(&self) -> Option<f32>
            {
            	impl_to_primitive_float_to_float!($T, f32, *self)
            }


            fn to_f64(&self) -> Option<f64>
            {
            	impl_to_primitive_float_to_float!($T, f64, *self)
            }
        }
    )
}

impl_to_primitive_float!(f32);
impl_to_primitive_float!(f64);

macro_rules! impl_num_cast
{
    ($T:ty, $conv:ident) => (
        impl NumCast for $T
        {
            fn from<N: ToPrimitive>(n: N) -> Option<$T>
            {
                // `$conv` could be generated using `concat_idents!`, but that
                // macro seems to be broken at the moment
                n.$conv()
            }
        }
    )
}

impl_num_cast!(u8, to_u8);
impl_num_cast!(u16, to_u16);
impl_num_cast!(u32, to_u32);
impl_num_cast!(u64, to_u64);
impl_num_cast!(usize, to_usize);
impl_num_cast!(i8, to_i8);
impl_num_cast!(i16, to_i16);
impl_num_cast!(i32, to_i32);
impl_num_cast!(i64, to_i64);
impl_num_cast!(isize, to_isize);
impl_num_cast!(f32, to_f32);
impl_num_cast!(f64, to_f64);


macro_rules! impl_from_primitive
{
    ($T:ty, $to_ty:ident) => (
        impl FromPrimitive for $T
        {

            fn from_i8(n: i8) -> Option<$T>
            {
            	n.$to_ty()
            }


            fn from_i16(n: i16) -> Option<$T>
            {
            	n.$to_ty()
            }


            fn from_i32(n: i32) -> Option<$T>
            {
            	n.$to_ty()
            }


            fn from_i64(n: i64) -> Option<$T>
            {
            	n.$to_ty()
            }

            fn from_u8(n: u8) -> Option<$T>
            {
            	n.$to_ty()
            }


            fn from_u16(n: u16) -> Option<$T>
            {
            	n.$to_ty()
            }


            fn from_u32(n: u32) -> Option<$T>
            {
            	n.$to_ty()
            }


            fn from_u64(n: u64) -> Option<$T>
            {
            	n.$to_ty()
            }


            fn from_f32(n: f32) -> Option<$T>
            {
            	n.$to_ty()
            }


            fn from_f64(n: f64) -> Option<$T>
            {
            	n.$to_ty()
            }
        }
    )
}

impl_from_primitive!(isize, to_isize);
impl_from_primitive!(i8, to_i8);
impl_from_primitive!(i16, to_i16);
impl_from_primitive!(i32, to_i32);
impl_from_primitive!(i64, to_i64);
impl_from_primitive!(usize, to_usize);
impl_from_primitive!(u8, to_u8);
impl_from_primitive!(u16, to_u16);
impl_from_primitive!(u32, to_u32);
impl_from_primitive!(u64, to_u64);
impl_from_primitive!(f32, to_f32);
impl_from_primitive!(f64, to_f64);


macro_rules! bound_impl {
    ($t:ty, $min:expr, $max:expr) =>
    {
        impl Bound for $t
        {
            fn lower_bound() -> $t { $min }

            fn upper_bound() -> $t { $max }
        }
    }
}

bound_impl!(usize, usize::MIN, usize::MAX);
bound_impl!(u8,    u8::MIN,    u8::MAX);
bound_impl!(u16,   u16::MIN,   u16::MAX);
bound_impl!(u32,   u32::MIN,   u32::MAX);
bound_impl!(u64,   u64::MIN,   u64::MAX);

bound_impl!(isize, isize::MIN, isize::MAX);
bound_impl!(i8,    i8::MIN,    i8::MAX);
bound_impl!(i16,   i16::MIN,   i16::MAX);
bound_impl!(i32,   i32::MIN,   i32::MAX);
bound_impl!(i64,   i64::MIN,   i64::MAX);

bound_impl!(f32, f32::MIN, f32::MAX);
bound_impl!(f64, f64::MIN, f64::MAX);


macro_rules! trigonometry_impl
{
    ($t:ty, $pi: expr) =>
    {
    	impl Trigonometry for $t
		{
			/// Returns the mathematic constant PI
			fn pi() -> Self
			{
				$pi
			}

			/// Sinus
			fn sin(self: &Self) -> Self
			{
				(*self).sin()
			}

			/// Cosinus
			fn cos(self: &Self) -> Self
			{
				(*self).cos()
			}

			///Tangens
			fn tan(self: &Self) -> Self
			{
				(*self).tan()
			}


			//
			fn cot(self: &Self) -> Self
			{
				1.0 / self.tan()
			}

			/// Secant
			///
			/// # Panics
			///
			/// self = n pi + pi/2 n \in Z
			///
			///
			fn sec(self: &Self) -> Self
			{
				1.0 / self.cos()
			}

			fn csc(self: &Self) -> Self
			{
				1.0 / self.sin()
			}

			/// Inverse sine function
			///
			/// # Arguemnts
			///
			/// -1.0 <= x <= 1.0
			///
			/// # Panics
			///
			/// |x| > 1.0
			///
			fn arcsin(self: &Self) -> Self
			{
				if self.abs() > 1.0
				{
					panic!();
				}

				self.asin()
			}

			/// Inverse cosine function
			///
			/// # Arguemnts
			///
			/// -1.0 <= x <= 1.0
			///
			/// # Panics
			///
			/// |x| > 1.0
			///
			fn arccos(self: &Self) -> Self
			{
				if self.abs() > 1.0
				{
					panic!();
				}

				self.acos()
			}

			/// Computes the arctangent of a number
    		///
			fn arctan(self: &Self) -> Self
			{
				self.atan()
			}

			/// Computes the arctangent
			fn arctan2(self: &Self, other: &Self) -> Self
			{
				self.atan2(*other)
			}


			fn arccot(self: &Self) -> Self
			{
				if *self == 0.0
				{
					return 0.0
				}

				if *self > 0.0
				{
					return (1.0 / self).atan()
				}
				else
				{
					return (1.0 / self).atan()
				}
			}

			fn arcsec(self: &Self) -> Self
			{
				(1.0 / self).acos()
			}

			fn arccsc(self: &Self) -> Self
			{
				(1.0 / self).asin()
			}
		}
	}
}


trigonometry_impl!(f32, std::f32::consts::PI);
trigonometry_impl!(f64, std::f64::consts::PI);

macro_rules! exponential_impl
{
    ($t:ty, $e: expr) =>
    {
    	impl Exponential for $t
		{
			///
			fn e() -> Self
			{
				$e
			}
			///Exponential function
			fn exp(self: &Self) -> Self
			{
				(*self).exp()
			}

			///Logiarithm function
			fn ln(self: &Self) -> Self
			{
				(*self).ln()
			}
		}
	}
}

exponential_impl!(f32, f32::consts::E);
exponential_impl!(f64, f64::consts::E);

macro_rules! power_impl
{
    ($t:ty) =>
    {
    	impl Power for $t
		{
			fn pow(self: &Self, exp: &Self) -> Self
			{
				self.powf(*exp)
			}

			fn root(self: &Self, root: &Self) -> Self
			{
				self.powf(1.0 / *root)
			}
		}
	}
}

power_impl!(f32);
power_impl!(f64);

macro_rules! hyperbolic_impl
{
    ($t:ty) =>
    {
    	impl Hyperbolic for $t
		{
			/// Hyperbolic sine
			fn sinh(self: &Self) -> Self
			{
				(*self).sinh()
			}

			/// Hyperbolic cosine
			fn cosh(self: &Self) -> Self
			{
				(*self).cosh()
			}

			/// Hyperbolic tangens
			///
    		/// # Arguments
    		///
   			/// * `self` :
    		///
    		/// # Example
    		///
    		/// ```
    		/// extern crate mathru;
   			/// use mathru::elementary::Hyperbolic;
    		///
    		/// let x: f64 = 0.0_f64;
			///
			/// let f: f64 = x.tanh();
			/// let g: f64 = 0.0;
			/// let abs_difference: f64 = (f - g).abs();
			///
			/// assert!(abs_difference < 1.0e-10);
    		/// ```
			fn tanh(self: &Self) -> Self
			{
				(*self).tanh()
			}

			/// Hyperbolic cotangens
			///
    		/// # Arguments
    		///
   			/// * `self` : != 0.0
    		///
    		/// # Panic
    		///
    		/// iff self == 0.0
    		///
    		/// # Example
    		///
    		/// ```
    		/// extern crate mathru;
   			/// use mathru::elementary::Hyperbolic;
    		///
    		/// let x: f64 = 1.0_f64;
			///
			/// let f: f64 = x.coth();
			/// let g: f64 = x.cosh() / x.sinh();
			/// let abs_difference: f64 = (f - g).abs();
			///
			/// assert!(abs_difference < 1.0e-10);
    		/// ```
			fn coth(self: &Self) -> Self
			{
				if *self == 0.0
				{
					panic!();
				}

				self.cosh() / self.sinh()
			}

			/// Hyperbolic secant
			///
    		/// # Arguments
    		///
   			/// * `self` :
    		///
    		/// # Example
    		///
    		/// ```
    		/// extern crate mathru;
   			/// use mathru::elementary::Hyperbolic;
    		///
    		/// let x: f64 = 0.0_f64;
			///
			/// let f: f64 = x.sech();
			/// let g: f64 = 1.0;
			/// let abs_difference: f64 = (f - g).abs();
			///
			/// assert!(abs_difference < 1.0e-10);
    		/// ```
			fn sech(self: &Self) -> Self
			{
				1.0 / self.cosh()
			}

			/// Hyperbolic cosecant
			///
    		/// # Arguments
    		///
   			/// * `self` : != 0.0
    		///
    		/// # Panics
    		///
    		/// if  self == 0
    		///
    		/// # Example
    		///
    		///
    		/// ```
    		/// extern crate mathru;
   			/// use mathru::elementary::Hyperbolic;
    		///
    		/// let x: f64 = 1.0_f64;
			///
			/// let f: f64 = x.csch();
			/// let g: f64 = 1.0 / x.sinh();
			/// let abs_difference: f64 = (f - g).abs();
			///
			/// assert!(abs_difference < 1.0e-10);
    		/// ```
			fn csch(self: &Self) -> Self
			{
				if *self == 0.0
				{
					panic!();
				}
				1.0 / self.sinh()
			}

			/// Hyperbolic inverse sine
			fn arsinh(self: &Self) -> Self
			{
				(*self).asinh()
			}

			/// Hyperbolic inverse cosine
			fn arcosh(self: &Self) -> Self
			{
				(*self).acosh()
			}

			/// Hyperbolic inverse tangens
			fn artanh(self: &Self) -> Self
			{
				if -1.0 >= *self || *self >= 1.0
				{
					panic!();
				}

				self.atanh()
			}

			/// Hyperbolic inverse cotan
			///
    		/// # Arguments
    		///
   			/// * `self`  -1.0 > self, self > 1.0
    		///
    		/// # Panics
    		///
    		/// if  -1.0 <= self && self <= 1.0
    		///
   			/// # Example
    		///
    		/// ```
    		/// extern crate mathru;
   			/// use mathru::algebra::abstr::{Field};
    		/// use mathru::elementary::{Exponential, Hyperbolic};
    		///
    		/// let x: f64 = 2.0_f64;
			/// let f: f64 = x.arcoth();
			/// let g: f64 = ((x + 1.0) / ( x - 1.0)).ln() / 2.0;
			/// let abs_difference: f64 = (f - g).abs();
			///
			/// assert!(abs_difference < 1.0e-10);
    		/// ```
			fn arcoth(self: &Self) -> Self
			{
				if -1.0 <= *self && *self <= 1.0
				{
					panic!();
				}

				((*self + 1.0) / (*self  - 1.0)).ln() / 2.0
			}

			/// Hyperbolic inverse secant
			///
    		/// # Arguments
    		///
   			/// * `self`  0.0 < self <= 1.0
    		///
    		/// # Panics
    		///
    		/// if  0.0 >= self || self > 1.0
    		///
    		/// # Example
    		///
    		/// ```
    		/// extern crate mathru;
    		/// use mathru::elementary::{Exponential, Hyperbolic};
    		///
    		/// let x: f64 = 0.5_f64;
			/// let f: f64 = x.arsech();
			/// let g: f64 = (1.0 / x).arcosh();
			/// let abs_difference: f64 = (f - g).abs();
			///
			/// assert!(abs_difference < 1.0e-10);
   			/// ```
			fn arsech(self: &Self) -> Self
			{
				if 0.0 >= *self || *self > 1.0
				{
					panic!();
				}

				(1.0 / self).arcosh()
			}

			/// Hyperbolic inverse cosecant
			///
    		/// # Arguments
    		///
   			/// * `self`  <> 0.0
    		///
    		/// # Panics
    		///
    		/// iff self = 0.0
    		///
    		/// # Example
    		///
    		/// ```
    		/// extern crate mathru;
   			/// use mathru::algebra::abstr::{Field};
    		/// use mathru::elementary::{Exponential, Hyperbolic};
    		///
    		/// let x: f64 = 2.0_f64;
			/// let f: f64 = x.arcsch();
			/// let g: f64 = (1.0 / x).arsinh();
			/// let abs_difference: f64 = (f - g).abs();
			///
			/// assert!(abs_difference < 1.0e-10);
    		/// ```
			fn arcsch(self: &Self) -> Self
			{
				(1.0 / self).arsinh()
			}
		}
	}
}

hyperbolic_impl!(f32);
hyperbolic_impl!(f64);

macro_rules! impl_as_primitive
{
    (@ $T: ty => $(#[$cfg:meta])* impl $U: ty ) =>
    {
        $(#[$cfg])*
        impl AsPrimitive<$U> for $T
        {
            #[inline]
            fn as_(self) -> $U
            {
            	self as $U
            }
        }
    };

    (@ $T: ty => { $( $U: ty ),* } ) =>
    {
    	$(
        	impl_as_primitive!(@ $T => impl $U);
    	)*
    };
    ($T: ty => { $( $U: ty ),* } ) =>
    {
        impl_as_primitive!(@ $T => { $( $U ),* });
        impl_as_primitive!(@ $T => { u8, u16, u32, u64, usize });
        impl_as_primitive!(@ $T => #[cfg(has_i128)] impl u128);
        impl_as_primitive!(@ $T => { i8, i16, i32, i64, isize });
        impl_as_primitive!(@ $T => #[cfg(has_i128)] impl i128);
    };
}

impl_as_primitive!(u8 => { char, f32, f64 });
impl_as_primitive!(i8 => { f32, f64 });
impl_as_primitive!(u16 => { f32, f64 });
impl_as_primitive!(i16 => { f32, f64 });
impl_as_primitive!(u32 => { f32, f64 });
impl_as_primitive!(i32 => { f32, f64 });
impl_as_primitive!(u64 => { f32, f64 });
impl_as_primitive!(i64 => { f32, f64 });
#[cfg(has_i128)]
impl_as_primitive!(u128 => { f32, f64 });
#[cfg(has_i128)]
impl_as_primitive!(i128 => { f32, f64 });
impl_as_primitive!(usize => { f32, f64 });
impl_as_primitive!(isize => { f32, f64 });
impl_as_primitive!(f32 => { f32, f64 });
impl_as_primitive!(f64 => { f32, f64 });
impl_as_primitive!(char => { char });
impl_as_primitive!(bool => {});



macro_rules! lapack_impl(
    ($T: ty, $xgehrd: path, $xorghr: path, $xgeev: path, $xgetrf: path) => (
        impl Lapack for $T
       	{

       		//Hessenberg
       		fn xgehrd(n: i32, ilo: i32, ihi: i32, a: &mut [Self], lda: i32,
                      tau: &mut [Self], work: &mut [Self], lwork: i32, info: &mut i32)
           	{
                unsafe { $xgehrd(n, ilo, ihi, a, lda, tau, work, lwork, info) }
			}

            fn xgehrd_work_size(n: i32, ilo: i32, ihi: i32, a: &mut [Self], lda: i32,
                                tau: &mut [Self], info: &mut i32) -> i32
            {
                let mut work = [Zero::zero()];
                let lwork = -1 as i32;

                unsafe { $xgehrd(n, ilo, ihi, a, lda, tau, &mut work, lwork, info) };

                work[0] as i32
            }

         	fn xorghr(n: i32, ilo: i32, ihi: i32, a: &mut [Self], lda: i32, tau: &[Self],
                      work: &mut [Self], lwork: i32, info: &mut i32)
          	{
                unsafe { $xorghr(n, ilo, ihi, a, lda, tau, work, lwork, info) }
            }

            fn xorghr_work_size(n: i32, ilo: i32, ihi: i32, a: &mut [Self], lda: i32,
                                tau: &[Self], info: &mut i32) -> i32 {
                let mut work = [ Zero::zero() ];
                let lwork = -1 as i32;

                unsafe { $xorghr(n, ilo, ihi, a, lda, tau, &mut work, lwork, info) };

                work[0] as i32
            }

            fn xgeev(jobvl: u8, jobvr: u8, n: i32, a: &mut [Self], lda: i32,
                     wr: &mut [Self], wi: &mut [Self],
                     vl: &mut [Self], ldvl: i32, vr: &mut [Self], ldvr: i32,
                     work: &mut [Self], lwork: i32, info: &mut i32)
          	{
                unsafe { $xgeev(jobvl, jobvr, n, a, lda, wr, wi, vl, ldvl, vr, ldvr, work, lwork, info) }
            }


            fn xgeev_work_size(jobvl: u8, jobvr: u8, n: i32, a: &mut [Self], lda: i32,
                               wr: &mut [Self], wi: &mut [Self], vl: &mut [Self], ldvl: i32,
                               vr: &mut [Self], ldvr: i32, info: &mut i32) -> i32
          	{
                let mut work = [ Zero::zero() ];
                let lwork = -1 as i32;

                unsafe { $xgeev(jobvl, jobvr, n, a, lda, wr, wi, vl, ldvl, vr, ldvr, &mut work, lwork, info) };
                work[0] as i32
			}

			fn xgetrf(m: i32, n: i32, a: &mut [Self], lda: i32, ipiv: &mut [i32], info: &mut i32)
			{
                unsafe { $xgetrf(m, n, a, lda, ipiv, info)}
			}
        }
    )
);


lapack_impl!(f32, lapack::sgehrd, lapack::sorghr, lapack::sgeev, lapack::sgetrf);
lapack_impl!(f64, lapack::dgehrd, lapack::dorghr, lapack::dgeev, lapack::dgetrf);
//hessenberg_scalar_impl!(Complex<f32>, lapack::cgehrd);
//hessenberg_scalar_impl!(Complex<f64>, lapack::zgehrd);