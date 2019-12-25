//! Cast module

//Copied from https://github.com/rust-num/num-traits/blob/master/src/cast.rs

/// A generic trait for converting a value to a number.
pub trait ToPrimitive
{
	/// Converts the value of `self` to an `i8`.
	fn to_i8(&self) -> Option<i8>
	{
		self.to_i64().and_then(|x| x.to_i8())
	}

	/// Converts the value of `self` to an `i16`.
	fn to_i16(&self) -> Option<i16>
	{
		self.to_i64().and_then(|x| x.to_i16())
	}

	/// Converts the value of `self` to an `i32`.
	fn to_i32(&self) -> Option<i32>
	{
		self.to_i64().and_then(|x| x.to_i32())
	}

	/// Converts the value of `self` to an `i64`.
	fn to_i64(&self) -> Option<i64>;

	/// Converts the value of `self` to an `i128`.
	fn to_i128(&self) -> Option<i128>;

	/// Converts the value of `self` to an `u8`.
	fn to_u8(&self) -> Option<u8>
	{
		self.to_u64().and_then(|x| x.to_u8())
	}

	/// Converts the value of `self` to an `u16`.
	fn to_u16(&self) -> Option<u16>
	{
		self.to_u64().and_then(|x| x.to_u16())
	}

	/// Converts the value of `self` to an `u32`.
	fn to_u32(&self) -> Option<u32>
	{
		self.to_u64().and_then(|x : u64 | x.to_u32())
	}

	/// Converts the value of `self` to an `u64`.
	fn to_u64(&self) -> Option<u64>;

	/// Converts the value of `self` to an `u128`.
	fn to_u128(&self) -> Option<u128>;

	/// Converts the value of `self` to an `f32`.
	fn to_f32(&self) -> Option<f32>
	{
		self.to_f64().and_then(|x| x.to_f32())
	}

	/// Converts the value of `self` to an `f64`.
	fn to_f64(&self) -> Option<f64>;
//	{
//		self.to_f64().and_then(|x: f64| x.to_f64())
//	}
}

/// A generic trait for converting a number to a value.
pub trait FromPrimitive: Sized
{
	/// Convert an `i8` to return an optional value of this type. If the
	/// type cannot be represented by this value, the `None` is returned.
	fn from_i8(n: i8) -> Option<Self>
	{
		FromPrimitive::from_i64(n as i64)
	}

	/// Convert an `i16` to return an optional value of this type. If the
	/// type cannot be represented by this value, the `None` is returned.
	fn from_i16(n: i16) -> Option<Self>
	{
		FromPrimitive::from_i64(n as i64)
	}

	/// Convert an `i32` to return an optional value of this type. If the
	/// type cannot be represented by this value, the `None` is returned.
	fn from_i32(n: i32) -> Option<Self>
	{
		FromPrimitive::from_i64(n as i64)
	}

	/// Convert an `i64` to return an optional value of this type. If the
	/// type cannot be represented by this value, the `None` is returned.
	fn from_i64(n: i64) -> Option<Self>;

	/// Convert an `i128` to return an optional value of this type. If the
	/// type cannot be represented by this value, the `None` is returned.
	fn from_i128(n: i128) -> Option<Self>;

	/// Convert an `u8` to return an optional value of this type. If the
	/// type cannot be represented by this value, the `None` is returned.
	fn from_u8(n: u8) -> Option<Self>
	{
		FromPrimitive::from_u64(n as u64)
	}

	/// Convert an `u16` to return an optional value of this type. If the
	/// type cannot be represented by this value, the `None` is returned.
	
	fn from_u16(n: u16) -> Option<Self>
	{
		FromPrimitive::from_u64(n as u64)
	}

	/// Convert an `u32` to return an optional value of this type. If the
	/// type cannot be represented by this value, the `None` is returned.
	fn from_u32(n: u32) -> Option<Self>
	{
		FromPrimitive::from_u64(n as u64)
	}

	/// Convert an `u64` to return an optional value of this type. If the
	/// type cannot be represented by this value, the `None` is returned.
	fn from_u64(n: u64) -> Option<Self>;

	/// Convert an `u128` to return an optional value of this type. If the
	/// type cannot be represented by this value, the `None` is returned.
	fn from_u128(n: u128) -> Option<Self>;

	/// Convert a `f32` to return an optional value of this type. If the
	/// type cannot be represented by this value, the `None` is returned.
	fn from_f32(n: f32) -> Option<Self>
	{
		FromPrimitive::from_f64(n as f64)
	}

	/// Convert a `f64` to return an optional value of this type. If the
	/// type cannot be represented by this value, the `None` is returned.
	fn from_f64(n: f64) -> Option<Self>;
//	{
//		FromPrimitive::from_f64(n as f64)
//	}
}

/// Cast from one machine scalar to another.
///
pub fn cast<T: NumCast, U: NumCast>(n: T) -> Option<U>
{
	NumCast::from(n)
}

/// An interface for casting between machine scalars.
pub trait NumCast: Sized + ToPrimitive
{
	/// Creates a number from another value that can be converted into
	/// a primitive via the `ToPrimitive` trait.
	fn from<T: ToPrimitive>(n: T) -> Option<Self>;
}

pub trait AsPrimitive<T>: 'static + Copy
where
    T: 'static + Copy,
{
    /// Convert a value to another, using the `as` operator.
    fn as_(self) -> T;
}

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

            fn from_i128(n: i128) -> Option<$T>
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

			fn from_u128(n: u128) -> Option<$T>
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

macro_rules! impl_num_cast
{
    ($T:ty, $conv:ident) => (
        impl NumCast for $T
        {
            fn from<N: ToPrimitive>(n: N) -> Option<$T>
            {
                n.$conv()
            }
        }
    )
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
