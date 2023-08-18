use crate::mathru::algebra::abstr::Sign;
use mathru::algebra::abstr::cast::{FromPrimitive, NumCast};
use mathru::algebra::abstr::{AbsDiffEq, Complex};
use mathru::algebra::abstr::{One, Zero};

#[test]
fn arg0() {
    let a: Complex<f64> = Complex::new(-3.5, -6.0);
    let angle: Complex<f64> = Complex::new(-6.0_f64.atan2(-3.5), 0.0_f64);
    assert_eq!(angle, a.arg());
}

#[test]
fn arg1() {
    let phi: Complex<f64> = Complex::new(0.0, 0.0_f64);
    let a: Complex<f64> = Complex::new(0.5, 0.0);
    assert_eq!(phi, a.arg());
}

#[test]
fn add() {
    let a: Complex<f32> = Complex::new(-3.5, 6.5);
    let b: Complex<f32> = Complex::new(2.0, -3.0);
    let sum: Complex<f32> = Complex::new(-1.5, 3.5);
    assert_eq!(sum, a + b);
}

#[test]
fn mul() {
    let a: Complex<f32> = Complex::new(-3.5, 6.5);
    let b: Complex<f32> = Complex::new(2.0, -3.0);
    let prod: Complex<f32> = Complex::new(-3.5 * 2.0 - 6.5 * -3.0, 6.5 * 2.0 + -3.5 * -3.0);
    assert_eq!(prod, a * b);
}

#[test]
fn mulassign() {
    let mut a: Complex<f32> = Complex::new(-3.5, 6.5);
    let b: Complex<f32> = Complex::new(2.0, -3.0);
    let prod: Complex<f32> = a * b;
    a *= b;
    assert_eq!(prod, a);
}

#[test]
fn zero() {
    let reference: Complex<f32> = Complex::new(0.0, 0.0);
    assert_eq!(reference, Complex::zero());
}

#[test]
fn one() {
    let reference: Complex<f64> = Complex::new(1.0, 0.0);
    assert_eq!(reference, Complex::one());
}

#[test]
fn conj() {
    let a_real: f32 = -3.5;
    let a_imag: f32 = 6.5;
    let a: Complex<f32> = Complex::new(a_real, a_imag);
    let conj: Complex<f32> = Complex::new(a_real, -a_imag);
    assert_eq!(conj, a.conj());
}

#[test]
fn div0() {
    let a_real: f32 = -3.5;
    let a_imag: f32 = 6.5;
    let b_real: f32 = 2.0;
    let b_imag: f32 = -3.0;
    let a: Complex<f32> = Complex::new(a_real, a_imag);
    let b: Complex<f32> = Complex::new(b_real, b_imag);
    let prod: Complex<f32> = Complex::new(
        (a_real * b_real + a_imag * b_imag) / (b_real * b_real + b_imag * b_imag),
        (a_imag * b_real - a_real * b_imag) / (b_real * b_real + b_imag * b_imag),
    );

    assert_eq!(prod, a / b);
}

#[test]
fn div1() {
    let a_real: f32 = -3.5;
    let a_imag: f32 = 6.5;
    let b_real: f32 = 0.0;
    let b_imag: f32 = 3.0;
    let a: Complex<f32> = Complex::new(a_real, a_imag);
    let b: Complex<f32> = Complex::new(b_real, b_imag);
    let prod: Complex<f32> = Complex::new(
        (a_real * b_real + a_imag * b_imag) / (b_real * b_real + b_imag * b_imag),
        (a_imag * b_real - a_real * b_imag) / (b_real * b_real + b_imag * b_imag),
    );

    assert_eq!(prod, a / b);
}

#[test]
fn sub() {
    let a: Complex<f32> = Complex::new(-3.5, 6.5);
    let b: Complex<f32> = Complex::new(2.0, -3.0);
    let diff: Complex<f32> = Complex::new(-5.5, 9.5);
    assert_eq!(diff, a - b);
}

#[test]
fn neg0() {
    let a_real: f32 = 1.0;
    let a_imag: f32 = 2.0;
    let uut: Complex<f32> = Complex::new(a_real, a_imag);
    assert_eq!(Complex::new(-a_real, -a_imag), -uut);
}

#[test]
fn from_f64() {
    let real = 3.14f64;
    let imag = 0.0f64;
    let a: Complex<f64> = Complex::from_f64(real);

    assert_eq!(real, a.re);
    assert_eq!(imag, a.im)
}

#[test]
fn from_u8() {
    let real = 3;
    let imag = 0.0f64;
    let a: Complex<f64> = Complex::from_u8(real);

    assert_eq!(real as f64, a.re);
    assert_eq!(imag, a.im)
}

#[test]
fn from_u16() {
    let real = 3;
    let imag = 0.0f64;
    let a: Complex<f64> = Complex::from_u16(real);

    assert_eq!(real as f64, a.re);
    assert_eq!(imag, a.im)
}

#[test]
fn from_u32() {
    let real = 3;
    let imag = 0.0f64;
    let a: Complex<f64> = Complex::from_u32(real);

    assert_eq!(real as f64, a.re);
    assert_eq!(imag, a.im)
}

#[test]
fn from_u64() {
    let real = 3;
    let imag = 0.0f64;
    let a: Complex<f64> = Complex::from_u64(real);

    assert_eq!(real as f64, a.re);
    assert_eq!(imag, a.im)
}

#[test]
fn from_u128() {
    let real = 3;
    let imag = 0.0f64;
    let a: Complex<f64> = Complex::from_u128(real);

    assert_eq!(real as f64, a.re);
    assert_eq!(imag, a.im)
}

#[test]
fn from_i8() {
    let real = 3;
    let imag = 0.0f64;
    let a: Complex<f64> = Complex::from_i8(real);

    assert_eq!(real as f64, a.re);
    assert_eq!(imag, a.im)
}

#[test]
fn from_i16() {
    let real = 3;
    let imag = 0.0f64;
    let a: Complex<f64> = Complex::from_i16(real);

    assert_eq!(real as f64, a.re);
    assert_eq!(imag, a.im)
}

#[test]
fn from_i32() {
    let real = 3;
    let imag = 0.0f64;
    let a: Complex<f64> = Complex::from_i32(real);

    assert_eq!(real as f64, a.re);
    assert_eq!(imag, a.im)
}

#[test]
fn from_i64() {
    let real = 3;
    let imag = 0.0f64;
    let a: Complex<f64> = Complex::from_i64(real);

    assert_eq!(real as f64, a.re);
    assert_eq!(imag, a.im)
}

#[test]
fn from_i128() {
    let real = 3;
    let imag = 0.0f64;
    let a: Complex<f64> = Complex::from_i128(real);

    assert_eq!(real as f64, a.re);
    assert_eq!(imag, a.im)
}

#[test]
#[should_panic]
fn sign() {
    let a: Complex<f64> = Complex::from_i128(-3);

    let _ = a.sign();
}

#[test]
#[should_panic]
fn is_positive() {
    let a: Complex<f64> = Complex::from_i128(-3);

    let _ = a.is_positive();
}

#[test]
#[should_panic]
fn is_negative() {
    let a: Complex<f64> = Complex::from_i128(-3);

    let _ = a.is_negative();
}

#[test]
fn add_borrow() {
    let a: Complex<f64> = Complex::new(1.0, -1.0);
    let b: Complex<f64> = Complex::new(-2.0, 2.0);

    assert_abs_diff_eq!(Complex::new(-1.0, 1.0), &a + &b);
}

#[test]
fn from() {
    let a: Complex<f64> = <Complex<f64> as NumCast>::from(1.0f64);
    assert_eq!(Complex::one(), a);
}

#[test]
fn div_assign() {
    let a_r = 1.0;
    let a_i = -1.0;
    let b_r = -2.0;
    let b_i = 2.0;
    let g = b_r * b_r + b_i * b_i;

    let mut a: Complex<f64> = Complex::new(a_r, a_i);
    let b: Complex<f64> = Complex::new(b_r, b_i);

    a /= b;

    assert_abs_diff_eq!(
        Complex::new((a_r * b_r + a_i * b_i) / g, (-a_r * b_i + a_i * b_r) / g),
        a
    );
}

#[test]
fn abs_diff_eq_smaller() {
    let a: Complex<f64> = Complex::zero();
    let tol = <Complex<f64> as AbsDiffEq>::default_epsilon();
    assert_abs_diff_eq!(a, a - tol);
}

#[test]
fn abs_diff_eq_bigger() {
    let a: Complex<f64> = Complex::zero();
    let tol = <Complex<f64> as AbsDiffEq>::default_epsilon();
    assert_abs_diff_eq!(a, a + tol);
}

#[test]
fn abs_diff_eq() {
    let a: Complex<f64> = Complex::new(-3.5, 0.0);
    let b: Complex<f64> = Complex::new(-3.5, -7.347880794884119e-16);
    let tol = Complex::new(f64::EPSILON, 4.0 * f64::EPSILON);
    assert_abs_diff_eq!(a, b, epsilon = tol);
}
