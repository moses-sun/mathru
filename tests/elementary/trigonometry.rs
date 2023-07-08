use mathru::algebra::abstr::{Complex, One};
use mathru::elementary::{Hyperbolic, Trigonometry};

#[test]
fn sin_complex_f64() {
    let a: f64 = 1.0;
    let b: f64 = 2.0;
    let z: Complex<f64> = Complex::new(a, b);
    let real: f64 = (-(-b).exp() * a.sin() - b.exp() * a.sin()) / -2.0;
    let imag: f64 = ((-b).exp() * a.cos() - b.exp() * a.cos()) / -2.0;

    let uut: Complex<f64> = z.sin();

    let refer: Complex<f64> = Complex::new(real, imag);
    assert_eq!(refer, uut);
}

#[test]
fn sin_complex_f32() {
    let a: f32 = 1.0;
    let b: f32 = 2.0;
    let z: Complex<f32> = Complex::new(a, b);
    let real: f32 = (-(-b).exp() * a.sin() - b.exp() * a.sin()) / -2.0;
    let imag: f32 = ((-b).exp() * a.cos() - b.exp() * a.cos()) / -2.0;

    let uut: Complex<f32> = z.sin();

    let refer: Complex<f32> = Complex::new(real, imag);
    assert_eq!(refer, uut);
}

#[test]
fn sin_f32() {
    let uut: f32 = (f32::pi() / 2.0).sin();

    assert_eq!(1.0, uut);
}

#[test]
fn sin_f64() {
    let uut: f64 = (f64::pi() / 2.0).sin();

    assert_eq!(1.0, uut);
}

#[test]
fn arsin_f64() {
    let a = f64::pi() / 2.0;

    assert_eq!(a, a.sin().arcsin());
}

#[test]
fn cos_f64() {
    let uut: f64 = (f64::pi()).cos();

    assert_eq!(-1.0, uut);
}

#[test]
fn arccos_f64() {
    let a = f64::pi() / 2.0;

    assert_eq!(a, a.cos().arccos());
}

#[test]
fn cot_f64() {
    let a = 0.5f64;

    assert_eq!(1.0 / a.tan(), a.cot());
}

#[test]
fn arccot_f64() {
    let a = 0.5f64;

    assert_eq!(a, a.cot().arccot());
}

#[test]
fn sec_f64() {
    let a = 0.5f64;

    assert_eq!(1.0 / a.cos(), a.sec());
}

#[test]
fn arcsec_f64() {
    let a = 0.5f64;

    assert_abs_diff_eq!(a, a.sec().arcsec());
}

#[test]
fn csc_f64() {
    let a = 2.0f64;

    assert_abs_diff_eq!(1.0 / a.sin(), a.csc());
}

#[test]
fn arccsc_f64() {
    let a = 0.5f64;

    assert_abs_diff_eq!(a, a.csc().arccsc());
}

#[test]
fn cos() {
    let a: f64 = 1.0;
    let b: f64 = 2.0;
    let z: Complex<f64> = Complex::new(a, b);
    let real: f64 = ((-b).exp() * a.cos() + b.exp() * (a).cos()) / 2.0;
    let imag: f64 = ((-b).exp() * a.sin() - b.exp() * (a).sin()) / 2.0;

    let uut: Complex<f64> = z.cos();

    let refer: Complex<f64> = Complex::new(real, imag);

    assert_eq!(refer, uut);
}

#[test]
fn tan() {
    let a: f64 = 1.0;
    let b: f64 = 2.0;
    let z: Complex<f64> = Complex::new(a, b);
    let refer: Complex<f64> = z.sin() / z.cos();

    let uut: Complex<f64> = z.tan();

    assert_eq!(refer, uut);
}

#[test]
fn cot() {
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let refer: Complex<f64> = Complex::one() / a.tan();

    assert_eq!(refer, a.cot());
}

#[test]
fn sec() {
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let refer: Complex<f64> = Complex::one() / a.cos();

    assert_eq!(refer, a.sec());
}

#[test]
fn csc() {
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let refer: Complex<f64> = Complex::one() / a.sin();

    assert_eq!(refer, a.csc());
}

#[test]
fn arctan0() {
    let a: Complex<f64> = Complex::new(3.0 * std::f64::consts::PI / 2.0, 0.0);
    let refer: Complex<f64> = Complex::new(1.3616916829711636, 0.0);

    assert_eq!(refer, a.arctan());
}

#[test]
fn arctan1() {
    let a: Complex<f64> = Complex::new(0.0_f64, 2.0_f64);
    let refer: Complex<f64> = Complex::new(
        std::f64::consts::PI / 2.0,
        (4.0_f64 / 5.0_f64).artanh() / 2.0_f64,
    );

    assert_eq!(refer, a.arctan());
}

#[test]
fn arctan2() {
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let refer: Complex<f64> = Complex::new(
        (2.0_f64.atan() + std::f64::consts::PI / 2.0_f64) / 2.0_f64,
        (4.0_f64 / 6.0_f64).atanh() / 2.0_f64,
    );

    assert_eq!(refer, a.arctan());
}

#[test]
fn arccot() {
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let refer: Complex<f64> = (Complex::one() / a).arctan();

    assert_eq!(refer, a.arccot());
}

#[test]
fn arcsin() {
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let refer: Complex<f64> = Complex::new(0.4270785863924768, 1.5285709194809995);

    assert_eq!(refer, a.arcsin());
}

#[test]
fn arccos() {
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let refer: Complex<f64> = Complex::new(std::f64::consts::PI / 2.0_f64, 0.0_f64) - a.arcsin();

    assert_eq!(refer, a.arccos());
}

#[test]
fn arcsec() {
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let refer: Complex<f64> = (Complex::one() / a).arccos();

    assert_eq!(refer, a.arcsec());
}

#[test]
fn arccsc() {
    let a: Complex<f64> = Complex::new(1.0_f64, 2.0_f64);
    let refer: Complex<f64> = (Complex::one() / a).arcsin();

    assert_eq!(refer, a.arccsc());
}

#[test]
fn pi() {
    let c: Complex<f64> = Complex::pi();
    let c_ref = Complex::new(f64::pi(), 0.0);
    assert_eq!(c_ref, c);
}
