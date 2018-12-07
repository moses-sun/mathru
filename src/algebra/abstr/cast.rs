
//use std::num::Wrapping;
//use numeric::Bound;

/// A generic trait for converting a value to a number.
pub trait ToPrimitive
{
	/// Converts the value of `self` to an `isize`.
	fn to_isize(&self) -> Option<isize>
	{
		self.to_i64().and_then(|x| x.to_isize())
	}

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

	/// Converts the value of `self` to a `usize`.
	fn to_usize(&self) -> Option<usize>
	{
		self.to_u64().and_then(|x| x.to_usize())
	}

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

	/// Converts the value of `self` to an `f32`.
	fn to_f32(&self) -> Option<f32>
	{
		self.to_f64().and_then(|x| x.to_f32())
	}

	/// Converts the value of `self` to an `f64`.
	fn to_f64(&self) -> Option<f64>
	{
		self.to_f64().and_then(|x: f64| x.to_f64())
	}
}



/// A generic trait for converting a number to a value.
pub trait FromPrimitive: Sized
{
	/// Convert an `isize` to return an optional value of this type. If the
	/// value cannot be represented by this value, the `None` is returned.
	fn from_isize(n: isize) -> Option<Self>
	{
		FromPrimitive::from_i64(n as i64)
	}

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

	/// Convert a `usize` to return an optional value of this type. If the
	/// type cannot be represented by this value, the `None` is returned.
	fn from_usize(n: usize) -> Option<Self>
	{
		FromPrimitive::from_u64(n as u64)
	}

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

	/// Convert a `f32` to return an optional value of this type. If the
	/// type cannot be represented by this value, the `None` is returned.
	fn from_f32(n: f32) -> Option<Self>
	{
		FromPrimitive::from_f64(n as f64)
	}

	/// Convert a `f64` to return an optional value of this type. If the
	/// type cannot be represented by this value, the `None` is returned.
	fn from_f64(n: f64) -> Option<Self>
	{
		FromPrimitive::from_f64(n as f64)
	}
}



/// Cast from one machine scalar to another.
///
/// # Examples
///
///
/// # use num_traits as num;
/// let twenty: f32 = num::cast(0x14).unwrap();
/// assert_eq!(twenty, 20f32);
///
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



