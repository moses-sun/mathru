use algebra::abstr::{Number, Natural, Integer, Real};
use algebra::abstr::identity::{Zero, One};
use algebra::abstr::cast::{NumCast, FromPrimitive, ToPrimitive};
use num::bound::Bound;
use std::mem::size_of;
use std::{u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64};

macro_rules! number_impl
{
    ($t:ty) =>
    {
        impl Number for $t
        {
			fn min(self: Self, a: Self) -> Self
    		{
    			if self <= a
				{
					return self
				}
				else
				{
					return a
				}
            }

    		fn max(self: Self, a: Self) -> Self
    		{
    			if self >= a
				{
					return self
				}
				else
				{
					return a
				}
    		}
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
number_impl!(f32);
number_impl!(f64);

macro_rules! zero_impl {
    ($t:ty, $v:expr) => {
        impl Zero for $t {
            #[inline]
            fn zero() -> $t {
                $v
            }
//            #[inline]
//            fn is_zero(&self) -> bool {
//                *self == $v
//            }
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
    ($t:ty, $v:expr) => {
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