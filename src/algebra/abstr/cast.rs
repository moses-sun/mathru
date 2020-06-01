//! Cast module

//Copied from https://github.com/rust-num/num-traits/blob/master/src/cast.rs

/// A generic trait for converting a value to a number.
pub trait ToPrimitive
{
    /// Converts the value of `self` to an `i8`.
    fn to_i8(&self) -> i8
    {
        self.to_i64().to_i8()
    }

    /// Converts the value of `self` to an `i16`.
    fn to_i16(&self) -> i16
    {
        self.to_i64().to_i16()
    }

    /// Converts the value of `self` to an `i32`.
    fn to_i32(&self) -> i32
    {
        self.to_i64().to_i32()
    }

    /// Converts the value of `self` to an `i64`.
    fn to_i64(&self) -> i64;

    /// Converts the value of `self` to an `i128`.
    fn to_i128(&self) -> i128;

    /// Converts the value of `self` to an `u8`.
    fn to_u8(&self) -> u8
    {
        self.to_u64().to_u8()
    }

    /// Converts the value of `self` to an `u16`.
    fn to_u16(&self) -> u16
    {
        self.to_u64().to_u16()
    }

    /// Converts the value of `self` to an `u32`.
    fn to_u32(&self) -> u32
    {
        self.to_u64().to_u32()
    }

    /// Converts the value of `self` to an `u64`.
    fn to_u64(&self) -> u64;

    /// Converts the value of `self` to an `u128`.
    fn to_u128(&self) -> u128;

    /// Converts the value of `self` to an `f32`.
    fn to_f32(&self) -> f32
    {
        self.to_f64().to_f32()
    }

    /// Converts the value of `self` to an `f64`.
    fn to_f64(&self) -> f64;
    //	{
    //		self.to_f64().and_then(|x: f64| x.to_f64())
    //	}
}

/// A generic trait for converting a number to a value.
pub trait FromPrimitive: Sized
{
    /// Convert an `i8` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    fn from_i8(n: i8) -> Self
    {
        FromPrimitive::from_i64(n as i64)
    }

    /// Convert an `i16` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    fn from_i16(n: i16) -> Self
    {
        FromPrimitive::from_i64(n as i64)
    }

    /// Convert an `i32` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    fn from_i32(n: i32) -> Self
    {
        FromPrimitive::from_i64(n as i64)
    }

    /// Convert an `i64` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    fn from_i64(n: i64) -> Self;

    /// Convert an `i128` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    fn from_i128(n: i128) -> Self;

    /// Convert an `u8` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    fn from_u8(n: u8) -> Self
    {
        FromPrimitive::from_u64(n as u64)
    }

    /// Convert an `u16` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.

    fn from_u16(n: u16) -> Self
    {
        FromPrimitive::from_u64(n as u64)
    }

    /// Convert an `u32` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    fn from_u32(n: u32) -> Self
    {
        FromPrimitive::from_u64(n as u64)
    }

    /// Convert an `u64` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    fn from_u64(n: u64) -> Self;

    /// Convert an `u128` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    fn from_u128(n: u128) -> Self;

    /// Convert a `f32` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    fn from_f32(n: f32) -> Self
    {
        FromPrimitive::from_f64(n as f64)
    }

    /// Convert a `f64` to return an optional value of this type. If the
    /// type cannot be represented by this value, the `None` is returned.
    fn from_f64(n: f64) -> Self;
    //	{
    //		FromPrimitive::from_f64(n as f64)
    //	}
}

/// Cast from one machine scalar to another.
pub fn cast<T: NumCast, U: NumCast>(n: T) -> U
{
    NumCast::from(n)
}

/// An interface for casting between machine scalars.
pub trait NumCast: Sized + ToPrimitive
{
    /// Creates a number from another value that can be converted into
    /// a primitive via the `ToPrimitive` trait.
    fn from<T: ToPrimitive>(n: T) -> Self;
}

pub trait AsPrimitive<T>: 'static + Copy
    where T: 'static + Copy
{
    /// Convert a value to another, using the `as` operator.
    fn as_(self) -> T;
}

macro_rules! impl_from_primitive {
    ($T:ty, $to_ty:ident) => {
        impl FromPrimitive for $T
        {
            fn from_i8(n: i8) -> $T
            {
                n.$to_ty()
            }

            fn from_i16(n: i16) -> $T
            {
                n.$to_ty()
            }

            fn from_i32(n: i32) -> $T
            {
                n.$to_ty()
            }

            fn from_i64(n: i64) -> $T
            {
                n.$to_ty()
            }

            fn from_i128(n: i128) -> $T
            {
                n.$to_ty()
            }

            fn from_u8(n: u8) -> $T
            {
                n.$to_ty()
            }

            fn from_u16(n: u16) -> $T
            {
                n.$to_ty()
            }

            fn from_u32(n: u32) -> $T
            {
                n.$to_ty()
            }

            fn from_u64(n: u64) -> $T
            {
                n.$to_ty()
            }

            fn from_u128(n: u128) -> $T
            {
                n.$to_ty()
            }

            fn from_f32(n: f32) -> $T
            {
                n.$to_ty()
            }

            fn from_f64(n: f64) -> $T
            {
                n.$to_ty()
            }
        }
    };
}

impl_from_primitive!(i8, to_i8);
impl_from_primitive!(i16, to_i16);
impl_from_primitive!(i32, to_i32);
impl_from_primitive!(i64, to_i64);
impl_from_primitive!(i128, to_i128);
impl_from_primitive!(u8, to_u8);
impl_from_primitive!(u16, to_u16);
impl_from_primitive!(u32, to_u32);
impl_from_primitive!(u64, to_u64);
impl_from_primitive!(u128, to_u128);
impl_from_primitive!(f32, to_f32);
impl_from_primitive!(f64, to_f64);

macro_rules! impl_num_cast {
    ($T:ty, $conv:ident) => {
        impl NumCast for $T
        {
            fn from<N: ToPrimitive>(n: N) -> $T
            {
                n.$conv()
            }
        }
    };
}

impl_num_cast!(u8, to_u8);
impl_num_cast!(u16, to_u16);
impl_num_cast!(u32, to_u32);
impl_num_cast!(u64, to_u64);
impl_num_cast!(u128, to_u128);
impl_num_cast!(i8, to_i8);
impl_num_cast!(i16, to_i16);
impl_num_cast!(i32, to_i32);
impl_num_cast!(i64, to_i64);
impl_num_cast!(i128, to_i128);
impl_num_cast!(f32, to_f32);
impl_num_cast!(f64, to_f64);
