use crate::algebra::abstr::cast::{ToPrimitive, AsPrimitive};
use crate::elementary::{Exponential, Trigonometry, Power, Hyperbolic};
use std::{u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64};


#[cfg(feature = "blaslapack")]
use crate::algebra::abstr::{Zero, Blas, Lapack};
#[cfg(feature = "blaslapack")]
use lapack;
#[cfg(feature = "blaslapack")]
use blas;

macro_rules! impl_to_primitive_int_to_int
{
    ($SrcT:ty, $DstT:ty, $slf:expr) => (
        {
        	$slf as $DstT
        }
    )
}

macro_rules! impl_to_primitive_int_to_uint
{
    ($SrcT:ty, $DstT:ty, $slf:expr) => (
        {
        	return $slf as $DstT;
        }
    )
}

macro_rules! impl_to_primitive_int
{
    ($T:ty) => (
        impl ToPrimitive for $T
        {

            fn to_i8(&self) -> i8
            {
            	impl_to_primitive_int_to_int!($T, i8, *self)
            }


            fn to_i16(&self) -> i16
            {
            	impl_to_primitive_int_to_int!($T, i16, *self)
            }


            fn to_i32(&self) -> i32
            {
            	impl_to_primitive_int_to_int!($T, i32, *self)
            }


            fn to_i64(&self) -> i64
            {
            	impl_to_primitive_int_to_int!($T, i64, *self)
            }

           	fn to_i128(&self) -> i128
            {
            	impl_to_primitive_int_to_int!($T, i128, *self)
            }

            fn to_u8(&self) -> u8
            {
            	impl_to_primitive_int_to_uint!($T, u8, *self)
            }


            fn to_u16(&self) -> u16
            {
            	impl_to_primitive_int_to_uint!($T, u16, *self)
            }


            fn to_u32(&self) -> u32
            {
            	impl_to_primitive_int_to_uint!($T, u32, *self)
            }


            fn to_u64(&self) -> u64
            {
            	impl_to_primitive_int_to_uint!($T, u64, *self)
            }

            fn to_u128(&self) -> u128
            {
            	impl_to_primitive_int_to_uint!($T, u128, *self)
            }


            fn to_f32(&self) -> f32
            {
            	return *self as f32;
            }


            fn to_f64(&self) -> f64
            {
            	return *self as f64;
            }
        }
    )
}

impl_to_primitive_int!(i8);
impl_to_primitive_int!(i16);
impl_to_primitive_int!(i32);
impl_to_primitive_int!(i64);
impl_to_primitive_int!(i128);

macro_rules! impl_to_primitive_uint_to_int {
    ($DstT:ty, $slf:expr) => (
        {
        	return $slf as $DstT;
        }
    )
}

macro_rules! impl_to_primitive_uint_to_uint {
    ($SrcT:ty, $DstT:ty, $slf:expr) => (
        {
        	return $slf as $DstT;
        }
    )
}

macro_rules! impl_to_primitive_uint
{
    ($T:ty) =>
    (
        impl ToPrimitive for $T
        {

            fn to_i8(&self) -> i8
            {
            	impl_to_primitive_uint_to_int!(i8, *self)
            }


            fn to_i16(&self) -> i16
            {
            	impl_to_primitive_uint_to_int!(i16, *self)
            }


            fn to_i32(&self) -> i32
            {
            	impl_to_primitive_uint_to_int!(i32, *self)
            }


            fn to_i64(&self) -> i64
            {
            	impl_to_primitive_uint_to_int!(i64, *self)
            }

			fn to_i128(&self) -> i128
            {
            	impl_to_primitive_uint_to_int!(i128, *self)
            }

            fn to_u8(&self) -> u8
            {
            	impl_to_primitive_uint_to_uint!($T, u8, *self)
            }

            fn to_u16(&self) -> u16
            {
            	impl_to_primitive_uint_to_uint!($T, u16, *self)
            }

            fn to_u32(&self) -> u32
            {
            	impl_to_primitive_uint_to_uint!($T, u32, *self)
            }

            fn to_u64(&self) -> u64
            {
            	impl_to_primitive_uint_to_uint!($T, u64, *self)
            }

            fn to_u128(&self) -> u128
            {
            	impl_to_primitive_uint_to_uint!($T, u128, *self)
            }

            fn to_f32(&self) -> f32
            {
            	return *self as f32;
            }

            fn to_f64(&self) -> f64
            {
            	return *self as f64;
            }
        }
    )
}

impl_to_primitive_uint!(u8);
impl_to_primitive_uint!(u16);
impl_to_primitive_uint!(u32);
impl_to_primitive_uint!(u64);
impl_to_primitive_uint!(u128);
//impl_to_primitive_uint!(f32);
//impl_to_primitive_uint!(f64);

macro_rules! impl_to_primitive_float_to_float
{
    ($SrcT:ident, $DstT:ident, $slf:expr) => (
    	return $slf as $DstT;
    )
}


macro_rules! impl_to_primitive_float
{
    ($T:ident) =>
    (
        impl ToPrimitive for $T
        {

            fn to_i8(&self) -> i8
            {
            	return *self as i8;
            }


            fn to_i16(&self) -> i16
            {
            	return *self as i16;
            }


            fn to_i32(&self) -> i32
            {
            	return *self as i32;
            }


            fn to_i64(&self) -> i64
            {
            	return *self as i64;
            }

			fn to_i128(&self) -> i128
            {
            	return *self as i128;
            }

            fn to_u8(&self) -> u8
            {
            	return *self as u8;
            }


            fn to_u16(&self) -> u16
            {
            	return *self as u16;
            }

            fn to_u32(&self) -> u32
            {
            	return *self as u32;
            }

            fn to_u64(&self) -> u64
            {
            	return *self as u64;
            }

			fn to_u128(&self) -> u128
            {
            	return *self as u128;
            }

            fn to_f32(&self) -> f32
            {
            	impl_to_primitive_float_to_float!($T, f32, *self)
            }

            fn to_f64(&self) -> f64
            {
            	impl_to_primitive_float_to_float!($T, f64, *self)
            }
        }
    )
}

impl_to_primitive_float!(f32);
impl_to_primitive_float!(f64);


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
				return self.powf(*exp);
			}

			fn root(self: &Self, root: &Self) -> Self
			{
				return self.powf(1.0 / *root);
			}

			fn sqrt(self: &Self) -> Self
			{
				return self.powf(0.5);
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


#[cfg(feature = "blaslapack")]
macro_rules! lapack_impl(
    ($T: ty, $xgehrd: path, $xorghr: path, $xgeev: path, $xgetrf: path, $xgeqrf: path, $xorgqr: path, $xgetri: path, $xpotrf: path,
    $xgetrs: path)
    => (
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
                let mut work = [<$T>::zero()];
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
                let mut work = [<$T>::zero() ];
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
                let mut work = [ <$T>::zero() ];
                let lwork = -1 as i32;

                unsafe { $xgeev(jobvl, jobvr, n, a, lda, wr, wi, vl, ldvl, vr, ldvr, &mut work, lwork, info) };
                work[0] as i32
			}

			//LU decomposition
			fn xgetrf(m: i32, n: i32, a: &mut [Self], lda: i32, ipiv: &mut [i32], info: &mut i32)
			{
                unsafe { $xgetrf(m, n, a, lda, ipiv, info)}
			}

			fn xgeqrf(m: i32, n: i32, a: &mut [Self], lda: i32, tau: &mut [Self], work: &mut [Self], lwork: i32,
			info: &mut i32)
			{
				unsafe { $xgeqrf(m, n, a, lda, tau, work, lwork, info) };
			}

  			fn xgeqrf_work_size(m: i32, n: i32, a: &mut [Self], lda: i32, tau: &mut [Self], info: &mut i32) -> i32
			{
				let mut work = [<$T>::zero() ];
                let lwork = -1 as i32;

                unsafe { $xgeqrf(m, n, a, lda, tau, &mut work, lwork, info) };
                work[0] as i32
			}

			fn xorgqr(m: i32, n: i32, k: i32, a: &mut [Self], lda: i32, tau: &mut [Self], work: &mut [Self], lwork:
			i32,
			info: &mut i32)
			{
				unsafe { $xorgqr(m, n, k, a, lda, tau, work, lwork, info) };
			}

  			fn xorgqr_work_size(m: i32, n: i32, k: i32, a: &mut [Self], lda: i32, tau: &mut [Self], info: &mut i32) ->
  			 i32
			{
				let mut work = [<$T>::zero() ];
                let lwork = -1 as i32;

                unsafe { $xorgqr(m, n, k, a, lda, tau, &mut work, lwork, info) };
                work[0] as i32
			}

			//
			fn xgetri(n: i32, a: &mut [Self], lda: i32, ipiv: &mut [i32], work: &mut [Self], lwork: i32, info: &mut i32)
			{
                unsafe { $xgetri(n, a, lda, ipiv, work, lwork, info)}
			}

			fn xgetri_work_size(n: i32, a: &mut [Self], lda: i32, ipiv: &mut [i32], info: &mut i32) -> i32
			{
				let mut work = [ <$T>::zero() ];
                let lwork = -1 as i32;
				unsafe { $xgetri(n, a, lda, ipiv, &mut work, lwork, info) };

				work[0] as i32
			}

			//cholsky
			fn xpotrf(uplo: char, n: i32, a: &mut [Self], lda: i32, info: &mut i32)
			{
				unsafe
				{
					$xpotrf(uplo as u8, n, a, lda, info);
				}
			}

			// solve
			fn xgetrs(n: i32, nrhs: i32, a: &mut [Self], lda: i32, ipiv: &mut [i32], b: &mut [Self], ldb: i32, info: &mut i32)
			{
				unsafe
				{
					$xgetrs('N' as u8, n, nrhs, a, lda, ipiv, b, ldb, info);
				}
			}


      	}
    )
);

#[cfg(feature = "blaslapack")]
macro_rules! blas_impl(
    ($T: ty, $xgemm: path, $xtrsm: path)
    => (
        impl Blas for $T
       	{
       		fn xgemm(transa: u8, transb: u8, m: i32, n: i32, k: i32, alpha: Self,
    		a: &[Self],
    		lda: i32,
    		b: &[Self],
    		ldb: i32,
    		beta: Self,
    		c: &mut [Self],
    		ldc: i32 )
			{
				unsafe { $xgemm(transa, transb, m, n , k, alpha, a, lda, b, ldb, beta, c, ldc)}
			}

			fn xtrsm(side: char, uplo: char, transa: char, diag: char, m: i32, n: i32, alpha: Self, a: &[Self], lda: i32, b: &mut
			[Self], ldb: i32)
			{
				unsafe
				{
					$xtrsm(side as u8, uplo as u8, transa as u8, diag as u8, m, n, alpha, a, lda, b, ldb);
				}
			}
		}
	)
);

#[cfg(feature = "blaslapack")]
lapack_impl!(f32, lapack::sgehrd, lapack::sorghr, lapack::sgeev, lapack::sgetrf, lapack::sgeqrf, lapack::sorgqr,
lapack::sgetri, lapack::spotrf, lapack::sgetrs);
#[cfg(feature = "blaslapack")]
lapack_impl!(f64, lapack::dgehrd, lapack::dorghr, lapack::dgeev, lapack::dgetrf, lapack::dgeqrf, lapack::dorgqr,
lapack::dgetri, lapack::dpotrf, lapack::dgetrs);

#[cfg(feature = "blaslapack")]
blas_impl!(f32, blas::sgemm, blas::strsm);
#[cfg(feature = "blaslapack")]
blas_impl!(f64, blas::dgemm, blas::dtrsm);