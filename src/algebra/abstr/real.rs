use super::field::Field;
use crate::elementary::{Exponential, Trigonometry, Power, Hyperbolic};

#[cfg(feature = "native")]
/// Real number
///
///<a href="https://en.wikipedia.org/wiki/Real_number">https://en.wikipedia.org/wiki/Real_number</a>
pub trait Real: Field + Exponential + Trigonometry + Power + Hyperbolic
{
	/// Returns the smallest integer greater than or equal to a number.
	fn ceil(self: &Self) -> Self;

	/// Returns the largest integer less than or equal to a number.
	fn floor(self: &Self) -> Self;

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

#[cfg(feature = "blaslapack")]
/// Real number
///
///<a href="https://en.wikipedia.org/wiki/Real_number">https://en.wikipedia.org/wiki/Real_number</a>
pub trait Real: Field + Exponential + Trigonometry + Power + Hyperbolic + Lapack + Blas
{
	/// Returns the smallest integer greater than or equal to a number.
	fn ceil(self: &Self) -> Self;

	/// Returns the largest integer less than or equal to a number.
	fn floor(self: &Self) -> Self;

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

#[cfg(feature = "blaslapack")]
pub trait Lapack: Sized
{
	fn xgehrd(n: i32, ilo: i32, ihi: i32, a: &mut [Self], lda: i32, tau: &mut [Self], work: &mut [Self], lwork: i32,
	info: &mut i32);

  	fn xgehrd_work_size(n: i32, ilo: i32, ihi: i32, a: &mut [Self], lda: i32, tau: &mut [Self], info: &mut i32) -> i32;

    fn xorghr(
        n: i32,
        ilo: i32,
        ihi: i32,
        a: &mut [Self],
        lda: i32,
        tau: &[Self],
        work: &mut [Self],
        lwork: i32,
        info: &mut i32,
    );

    fn xorghr_work_size(n: i32, ilo: i32, ihi: i32, a: &mut [Self], lda: i32, tau: &[Self], info: &mut i32) -> i32;

  	fn xgeev(jobvl: u8, jobvr: u8, n: i32, a: &mut [Self], lda: i32,
                     wr: &mut [Self], wi: &mut [Self],
                     vl: &mut [Self], ldvl: i32, vr: &mut [Self], ldvr: i32,
                     work: &mut [Self], lwork: i32, info: &mut i32);



  	fn xgeev_work_size(jobvl: u8, jobvr: u8, n: i32, a: &mut [Self], lda: i32,
                               wr: &mut [Self], wi: &mut [Self], vl: &mut [Self], ldvl: i32,
                               vr: &mut [Self], ldvr: i32, info: &mut i32) -> i32;


	fn xgetrf(m: i32, n: i32, a: &mut [Self], lda: i32, ipiv: &mut [i32], info: &mut i32);


	fn xgeqrf(m: i32, n: i32, a: &mut [Self], lda: i32, tau: &mut [Self], work: &mut [Self], lwork: i32, info: &mut
	i32);

  	fn xgeqrf_work_size(m: i32, n: i32, a: &mut [Self], lda: i32, tau: &mut [Self], info: &mut i32) -> i32;

	fn xorgqr(m: i32, n: i32, k: i32, a: &mut [Self], lda: i32, tau: &mut [Self], work: &mut [Self], lwork: i32, info:
	&mut
	i32);

  	fn xorgqr_work_size(m: i32, n: i32, k: i32, a: &mut [Self], lda: i32, tau: &mut [Self], info: &mut i32) -> i32;


  	fn xgetri(n: i32, a: &mut [Self], lda: i32, ipiv: &mut [i32], work: &mut [Self], lwork: i32, info: &mut i32);

	fn xgetri_work_size(n: i32, a: &mut [Self], lda: i32, ipiv: &mut [i32], info: &mut i32) -> i32;

	fn xpotrf(uplo: char, n: i32, a: &mut [Self], lda: i32, info: &mut i32);
}


#[cfg(feature = "blaslapack")]
pub trait Blas: Sized
{
	fn xgemm(transa: u8, transb: u8, m: i32, n: i32, k: i32, alpha: Self,
    a: &[Self],
    lda: i32,
    b: &[Self],
    ldb: i32,
    beta: Self,
    c: &mut [Self],
    ldc: i32 ) ;

}