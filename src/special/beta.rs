use special::gamma::gamma;

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
/// extern crate mathru;
/// use mathru::special::beta;
/// let x: f64 = 0.3_f64;
/// let y: f64 = 0.6_f64;
///
/// let beta: f64 = beta::beta(x, y);
/// ```
pub fn beta<'a>(x: f64, y: f64) -> f64
{
	gamma(x) * gamma(y) / gamma(x + y)
}




/// Incomplete regularized beta function
///
/// B(x; a,b) = &int;<sub>0</sub> <sup>x</sup> t<sup>a-1</sup>(1-t)<sup>b-1</sup> dt
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
/// extern crate mathru;
/// use mathru::special::beta;
/// let a: f64 = 0.3_f64;
/// let b: f64 = 0.6_f64;
/// let x: f64 = 0.2_f64;
///
/// let beta: f64 = beta::beta_inc_reg(x, a, b);
/// ```
/// The code from the following C code was ported to Rust
/// <a href="http://people.sc.fsu.edu/~jburkardt/c_src/asa109/asa109.c">Wikipedia Beta function</a>
pub fn beta_inc_reg(x: f64, a: f64, b: f64) -> f64
{
    let acu: f64 = 0.1E-14;

    /*
    Check the input arguments.
    */
    if a <= 0.0_f64 || b <= 0.0_f64
    {
        panic!();
    }

    if 0.0_f64 > x || x > 1.0_f64
    {
        panic!();
    }

    /*
    Special cases.
    */
    if x == 0.0 || x == 1.0
    {
        return x
    }

    /*
      Change tail if necessary and determine S.
    */
    let mut psq: f64 = a + b;
    let mut cx: f64 = 1.0_f64 - x;
    let xx: f64;
    let pp: f64;
    let qq: f64;
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

    let mut term: f64 = 1.0_f64;
    let mut ai: f64= 1.0_f64;
    let mut value: f64 = 1.0;

    let mut ns: i32 = ( qq + cx * psq ) as i32;

    /*
      Use the Soper reduction formula.
    */
    let mut rx: f64 = xx / cx;
    let mut temp: f64 = qq - ai;
    if ns == 0
    {
        rx = xx;
    }

    loop
    {
        term = term * temp * rx / ( pp + ai );
        value = value + term;;
        temp = term.abs();

        if temp <= acu && temp <= acu * value
        {
            value = value * (pp * xx.ln() + ( qq - 1.0_f64 ) * cx.ln() - (beta(a, b)).ln() ).exp() / pp;

            if indx == 0
            {
                value = 1.0_f64 - value;
            }
            break;
        }

        ai = ai + 1.0;
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
            psq = psq + 1.0_f64;
        }
    }

    if indx == 1
    {
        return 1.0_f64 - value
    }
    else
    {
        return value
    }
}