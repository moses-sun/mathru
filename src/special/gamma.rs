use std::f64::consts::PI;
use std::f64::consts;
use std::f64::INFINITY;


/// Gamma function
///
/// &Gamma;(z) = &int; <sub>0</sub><sup>&infin;</sup> t<sup>z-1</sup>(1-t)<sup>-x</sup> dx
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Gamma_function">https://en.wikipedia.org/wiki/Gamma_function</a>
///
/// # Arguments
///
/// * `z`
///
/// # Example
///
/// ```
/// extern crate mathru;
/// use mathru::special::gamma;
/// let z: f64 = 0.3_f64;
/// let gamma: f64 = gamma::gamma(z);
/// ```
/// The following approximatino is implemented
/// https://en.wikipedia.org/wiki/Lanczos_approximation
pub fn gamma(z: f64) -> f64
{
    if z < 0.5
    {
        return PI / ((PI*z).sin() * gamma (1.0 - z)) //reflection formula
    }

 	let t: f64 = z + 6.5;
    let x: f64 = 0.99999999999980993 +
        676.5203681218851 / z -
        1259.1392167224028 / (z + 1.0) +
        771.32342877765313 / (z + 2.0) -
        176.61502916214059 / (z + 3.0) +
        12.507343278686905 / (z + 4.0) -
        0.13857109526572012 / (z + 5.0) +
        9.9843695780195716e-6 / (z + 6.0) +
        1.5056327351493116e-7 / (z + 7.0);

    return 2.0_f64.sqrt() * PI.sqrt() * t.powf(z - 0.5) * (-t).exp() * x
}

/// Auxiliary variable when evaluating the `gamma_ln` function
const GAMMA_R: f64 = 10.900511;

/// Polynomial coefficients for approximating the `gamma_ln` function
const GAMMA_DK: &'static [f64] = &[
    2.48574089138753565546e-5,
    1.05142378581721974210,
    -3.45687097222016235469,
    4.51227709466894823700,
    -2.98285225323576655721,
    1.05639711577126713077,
    -1.95428773191645869583e-1,
    1.70970543404441224307e-2,
    -5.71926117404305781283e-4,
    4.63399473359905636708e-6,
    -2.71994908488607703910e-9,
];



/// Log-gamma function
///
/// ln&Gamma;(z)
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Gamma_function#The_log-gamma_function">https://en.wikipedia.org/wiki/Gamma_function#The_log-gamma_function</a>
///
/// # Arguments
///
/// * `z`
///
/// # Example
///
/// ```
/// extern crate mathru;
/// use mathru::special::gamma;
/// let x: f64 = 0.3_f64;
///
/// let ln_gamma: f64 = gamma::ln_gamma(x);
/// ```
pub fn ln_gamma(x: f64) -> f64
 {
    if x < 0.5
    {
        let s = GAMMA_DK
            .iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (t.0 as f64 - x));

        consts::PI.ln()
            - (consts::PI * x).sin().ln()
            - s.ln()
            - (2.0 * (consts::E / consts::PI).sqrt()).ln()
            - (0.5 - x) * ((0.5 - x + GAMMA_R) / consts::E).ln()
    }
    else
    {
        let s = GAMMA_DK
            .iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (x + t.0 as f64 - 1.0));

        s.ln()
            + (2.0 * (consts::E / consts::PI).sqrt()).ln()
            + (x - 0.5) * ((x - 0.5 + GAMMA_R) / consts::E).ln()
    }
}


/// Digamma function
///
/// d/dxln&Gamma;(z)
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Digamma_function">https://en.wikipedia.org/wiki/Digamma_function</a>
///
/// # Arguments
///
/// * `x`
///
/// # Example
///
/// ```
/// extern crate mathru;
/// use mathru::special::gamma;
/// let x: f64 = 0.3_f64;
/// let digamma: f64 = gamma::digamma(x);
/// ```
pub fn digamma(x: f64) -> f64
{
  	let c: f64 = 8.5;
  	let euler_mascheroni: f64 = 0.57721566490153286060;

	/*
  	Check the input.
	*/
  	if x <= 0.0
  	{
   		return 0.0
  	}

	/*
  	Use approximation for small argument.
	*/
  	if x <= 0.000001
  	{
    	return -euler_mascheroni - 1.0 / x + 1.6449340668482264365 * x;
  	}
  	/*
  	Reduce to DIGAMA(X + N).
	*/
  	let mut value : f64 = 0.0;
  	let mut x2: f64= x;
  	while  x2 < c
  	{
    	value = value - 1.0 / x2;
    	x2 = x2 + 1.0;
  	}
	/*
  	Use Stirling's (actually de Moivre's) expansion.
	*/
  	let mut r: f64 = 1.0 / x2;
  	value = value + x2.ln() - 0.5 * r;

  	r = r * r;

  	value = value
    - r * ( 1.0 / 12.0
    - r * ( 1.0 / 120.0
    - r * ( 1.0 / 252.0
    - r * ( 1.0 / 240.0
    - r * ( 1.0 / 132.0 ) ) ) ) );

  	return value;
}


/// Upper incomplete gamma function
///
/// &gamma;(a, x) = &int;<sub>0</sub><sup>x</sup>t<sup>a-1</sup>e<sup>-t</sub>dt
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Incomplete_gamma_function#Upper_incomplete_Gamma_function">https://en.wikipedia.org/wiki/Incomplete_gamma_function#Upper_incomplete_Gamma_function</a>
///
/// # Arguments
///
/// * `a`
/// * `x`
///
/// # Example
///
/// ```
/// extern crate mathru;
/// use mathru::special::gamma;
///
/// let a: f64 = 0.5_f64;
/// let x: f64 = 0.3_f64;
///
/// let gamma_u: f64 = gamma::gamma_u(a, x);
/// ```
pub fn gamma_u(a: f64, x: f64) -> f64
{
    gamma_ur(a, x) * gamma(a)
}

/// Upper incomplete regularized gamma function
///
/// P(a, x) = &gamma;(a,x) / &Gamma;(a)
///
/// Fore more information:
/// <a href="https://en.wikipedia
/// .org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables">https://en
/// .wikipedia.org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables</a>
///
/// # Arguments
///
/// * `a`
/// * `x`
///
/// # Example
///
/// ```
/// extern crate mathru;
/// use mathru::special::gamma;
///
/// let a: f64 = 0.5_f64;
/// let x: f64 = 0.3_f64;
///
/// let gamma_u: f64 = gamma::gamma_ur(a, x);
/// ```
pub fn gamma_ur(a: f64, x: f64) -> f64
{
    1.0 - gamma_lr(a, x)
}

/// Lower incomplete gamma function
///
/// &Gamma;(a, x) = &int;<sub>x</sub><sup>&infin;</sup>t<sup>a-1</sup>e<sup>-t</sub>dt
///
/// Fore more information:
/// <a href="https://en.wikipedia
/// .org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables">https://en
/// .wikipedia.org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables</a>
///
/// # Arguments
///
/// * `a`
/// * `x`
///
/// # Example
///
/// ```
/// extern crate mathru;
/// use mathru::special::gamma;
///
/// let a: f64 = 0.5_f64;
/// let x: f64 = 0.3_f64;
///
/// let gamma_u: f64 = gamma::gamma_ur(a, x);
/// ```
//pub fn gamma_l(a: f64, x: f64) -> f64
//{
//    gamma_l(a, x) * gamma(a)
//}

/// Lower regularized incomplete gamma function
///
/// Q(a, x) = &Gamma;(a, x) / &Gamma;(a)
///
/// Fore more information:
/// <a href="https://en.wikipedia
/// .org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables">https://en
/// .wikipedia.org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables</a>
///
/// # Arguments
///
/// * `a`
/// * `x`
///
/// # Example
///
/// ```
/// extern crate mathru;
/// use mathru::special::gamma;
///
/// let a: f64 = 0.5_f64;
/// let x: f64 = 0.3_f64;
///
/// let gamma_u: f64 = gamma::gamma_ur(a, x);
/// ```
pub fn gamma_lr(a: f64, x: f64) -> f64
{
    if a.is_nan() || x.is_nan()
    {
        panic!();
    }

    if a <= 0.0 || a == INFINITY
    {
        panic!();
    }
    if x <= 0.0 || x == INFINITY
    {
        panic!();
    }

    let eps: f64 = 0.000000000000001_f64;
    let big: f64 = 4503599627370496.0_f64;
    let big_inv: f64 = 2.22044604925031308085e-16_f64;

    if a == 0.0_f64
    {
        return 1.0_f64
    }

    if x == 0.0_f64
    {
        return 0.0_f64;
    }

    let ax = a * x.ln() - x - ln_gamma(a);

    if ax < -709.78271289338399_f64
    {
        if a < x
        {
            return 1.0_f64
        }
        return 0.0_f64
    }

    if x <= 1.0_f64 || x <= a
    {
        let mut r2: f64 = a;
        let mut c2: f64 = 1.0_f64;
        let mut ans2: f64= 1.0_f64;
        loop
        {
            r2 += 1.0_f64;
            c2 *= x / r2;
            ans2 += c2;

            if c2 / ans2 <= eps
            {
                break;
            }
        }
        return ax.exp() * ans2 / a
    }

    let mut y: f64 = 1.0 - a;
    let mut z: f64 = x + y + 1.0;
    let mut c: u32 = 0;

    let mut p3: f64 = 1.0_f64;
    let mut q3: f64 = x;
    let mut p2: f64 = x + 1.0_f64;
    let mut q2: f64 = z * x;
    let mut ans: f64 = p2 / q2;

    loop
    {
        y += 1.0_f64;
        z += 2.0_f64;
        c += 1;
        let yc = y * f64::from(c);

        let p = p2 * z - p3 * yc;
        let q = q2 * z - q3 * yc;

        p3 = p2;
        p2 = p;
        q3 = q2;
        q2 = q;

        if p.abs() > big
        {
            p3 *= big_inv;
            p2 *= big_inv;
            q3 *= big_inv;
            q2 *= big_inv;
        }

        if q != 0.0_f64
        {
            let nextans = p / q;
            let error = ((ans - nextans) / nextans).abs();
            ans = nextans;

            if error <= eps
            {
                break;
            }
        }
    }

    1.0_f64 - ax.exp() * ans
}