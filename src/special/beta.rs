//! Beta functions
use crate::algebra::abstr::Real;
use crate::special::gamma::gamma;

/// Beta function
///
/// B(x,y) = &int; t<sup>x-1</sup>(1-t)<sup>y-1</sup> dt
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Beta_function">Wikipedia Beta function</a>
///
/// # Arguments
///
/// * `x` > 0.0
/// * `y` > 0.0
///
/// # Panics
///
/// Panics if the parameter conditions are not fulfilled.
///
/// # Example
///
/// ```
/// use mathru::special::beta;
///
/// let x: f64 = 0.3_f64;
/// let y: f64 = 0.6_f64;
///
/// let beta: f64 = beta::beta(x, y);
/// ```
pub fn beta<T>(x: T, y: T) -> T
    where T: Real
{
    gamma(x) * gamma(y) / gamma(x + y)
}

/// Incomplete regularized beta function
///
/// B(x; a,b) = &int;<sub>0</sub> <sup>x</sup>
/// t<sup>a-1</sup>(1-t)<sup>b-1</sup> dt
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Beta_function#Incomplete_beta_function">Wikipedia Beta function</a>
///
/// # Arguments
///
/// * `x` > 0.0
/// * `y` > 0.0
///
/// # Panics
///
/// Panics if the parameter conditions are not fulfilled.
///
/// # Example
///
/// ```
/// use mathru::special::beta;
///
/// let a: f64 = 0.3_f64;
/// let b: f64 = 0.6_f64;
/// let x: f64 = 0.2_f64;
///
/// let beta: f64 = beta::beta_inc_reg(x, a, b);
/// ```
/// The code from the following C code was ported to Rust
/// <a href="http://people.sc.fsu.edu/~jburkardt/c_src/asa109/asa109.c">C implementation</a>
pub fn beta_inc_reg<T>(x: T, a: T, b: T) -> T
    where T: Real
{
    let acu: T = T::from_f64(0.1E-14);

    /*
    Check the input arguments.
    */
    if a <= T::zero() || b <= T::zero()
    {
        panic!();
    }

    if T::zero() > x || x > T::one()
    {
        panic!();
    }

    /*
    Special cases.
    */
    if x == T::zero() || x == T::one()
    {
        return x;
    }

    /*
      Change tail if necessary and determine S.
    */
    let mut psq: T = a + b;
    let mut cx: T = T::one() - x;
    let xx: T;
    let pp: T;
    let qq: T;
    let indx: u32;

    if a < psq * x
    {
        xx = cx;
        cx = x;
        pp = b;
        qq = a;
        indx = 1;
    }
    else
    {
        xx = x;
        pp = a;
        qq = b;
        indx = 0;
    }

    let mut term: T = T::one();
    let mut ai: T = T::one();
    let mut value: T = T::one();

    let mut ns: i32 = (qq + cx * psq).to_i32();

    /*
      Use the Soper reduction formula.
    */
    let mut rx: T = xx / cx;
    let mut temp: T = qq - ai;
    if ns == 0
    {
        rx = xx;
    }

    loop
    {
        term = term * temp * rx / (pp + ai);
        value = value + term;
        temp = term.abs();

        if temp <= acu && temp <= acu * value
        {
            value =
                value * (pp * xx.ln() + (qq - T::one()) * cx.ln() - (beta(a, b)).ln()).exp() / pp;

            if indx == 0
            {
                value = T::one() - value;
            }
            break;
        }

        ai = ai + T::one();
        ns = ns - 1;

        if 0 <= ns
        {
            temp = qq - ai;
            if ns == 0
            {
                rx = xx;
            }
        }
        else
        {
            temp = psq;
            psq = psq + T::one();
        }
    }

    if indx == 1
    {
        return T::one() - value;
    }
    else
    {
        return value;
    }
}
