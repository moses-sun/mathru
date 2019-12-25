use std::f64::consts::PI;

use crate::algebra::abstr::{Field, Scalar, Lattice, Sign};
use crate::elementary::{Exponential, Trigonometry, Power, Hyperbolic};
use std::ops::Neg;

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
/// The following approximation is implemented
/// https://en.wikipedia.org/wiki/Lanczos_approximation
pub fn gamma<T>(z: T) -> T
    where T: Field + Scalar + Exponential + Trigonometry + Power + Hyperbolic + Neg<Output = T>
{
    if z < T::from_f64(0.5_f64).unwrap()
    {
        return T::pi() / ((T::pi() * z).sin() * gamma (T::one() - z)) //reflection formula
    }

 	let t: T = z + T::from_f64(6.5).unwrap();
    let x: T = T::from_f64(0.99999999999980993).unwrap() +
        T::from_f64(676.5203681218851).unwrap() / z -
        T::from_f64(1259.1392167224028).unwrap() / (z + T::one()) +
        T::from_f64(771.32342877765313).unwrap() / (z + T::from_f64(2.0).unwrap()) -
        T::from_f64(176.61502916214059).unwrap() / (z + T::from_f64(3.0).unwrap()) +
        T::from_f64(12.507343278686905).unwrap() / (z + T::from_f64( 4.0).unwrap()) -
        T::from_f64(0.13857109526572012).unwrap() / (z + T::from_f64(5.0).unwrap()) +
        T::from_f64(9.9843695780195716e-6).unwrap() / (z + T::from_f64(6.0).unwrap()) +
        T::from_f64(1.5056327351493116e-7).unwrap() / (z + T::from_f64(7.0).unwrap());

    return T::from_f64(2.0_f64.sqrt() * PI.sqrt()).unwrap() * t.pow(&(z - T::from_f64(0.5).unwrap())) * (-t).exp() * x
}


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
pub fn ln_gamma<T>(x: T) -> T
    where T: Field + Scalar + Exponential + Trigonometry + Power + Hyperbolic + Neg<Output = T>
{
    // Auxiliary variable when evaluating the `gamma_ln` function
    let gamma_r: T = T::from_f64(10.900511).unwrap();

    // Polynomial coefficients for approximating the `gamma_ln` function
    let gamma_dk: & [T] = &[
    T::from_f64(2.48574089138753565546e-5).unwrap(),
    T::from_f64(1.05142378581721974210).unwrap(),
    T::from_f64(-3.45687097222016235469).unwrap(),
    T::from_f64(4.51227709466894823700).unwrap(),
    T::from_f64(-2.98285225323576655721).unwrap(),
    T::from_f64(1.05639711577126713077).unwrap(),
    T::from_f64(-1.95428773191645869583e-1).unwrap(),
    T::from_f64(1.70970543404441224307e-2).unwrap(),
    T::from_f64(-5.71926117404305781283e-4).unwrap(),
    T::from_f64(4.63399473359905636708e-6).unwrap(),
    T::from_f64(-2.71994908488607703910e-9).unwrap(),
    ];

    if x < T::from_f64(0.5).unwrap()
    {
        let s = gamma_dk
            .iter()
            .enumerate()
            .skip(1)
            .fold(gamma_dk[0], |s, t| s + *t.1 / (T::from_u64(t.0 as u64).unwrap() - x));

        T::pi().ln()
            - (T::pi() * x).sin().ln()
            - s.ln()
            - (T::from_f64(2.0).unwrap() * (T::e() / T::pi()).pow(&T::from_f64(0.5).unwrap())).ln()
            - (T::from_f64(0.5).unwrap() - x) * ((T::from_f64(0.5).unwrap() - x + gamma_r) / T::e()).ln()
    }

    else
    {
        let s = gamma_dk
            .iter()
            .enumerate()
            .skip(1)
            .fold(gamma_dk[0], |s, t| s + *t.1 / (x + T::from_u64(t.0 as u64).unwrap() - T::one()));

        s.ln()
            + (T::from_f64(2.0).unwrap() * (T::e() / T::pi()).pow(&T::from(0.5).unwrap())).ln()
            + (x - T::from_f64(0.5).unwrap()) * ((x - T::from_f64(0.5).unwrap() + gamma_r) / T::e()).ln()
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
pub fn digamma<T>(x: T) -> T
    where T: Field + Scalar + Exponential + Trigonometry + Power + Hyperbolic + Neg<Output = T>
{
  	let c: T = T::from_f64(8.5).unwrap();
 // 	let euler_mascheroni: T = T::from_f64(0.57721566490153286060).unwrap();

	/*
  	Check the input.
	*/
//	let real_neg: bool;
//  	if x <= T::zero()
//  	{
//  	    real_neg = true;
//  	}
//  	else
//    {
//        real_neg = false;
//    }
//
//	/*
//  	Use approximation for small argument.
//	*/
//  	if x <= T::from_f64(0.000001).unwrap()
//  	{
//    	return -euler_mascheroni - T::one() / x + T::from_f64(1.6449340668482264365).unwrap() * x;
//  	}


  	/*
  	Reduce to DIGAMA(X + N).
	*/
  	let mut value : T = T::zero();
  	let mut x2: T = x;
  	//The comparison only compares the real part ot the number
  	while  x2 < c
  	{
    	value = value - T::one() / x2;
    	x2 = x2 + T::one();
  	}
	/*
  	Use Stirling's (actually de Moivre's) expansion.
	*/
  	let mut r: T = T::one() / x2;
  	value = value + x2.ln() - T::from_f64(0.5).unwrap() * r;

  	r = r * r;

  	value = value
    - r * ( T::from_f64(1.0 / 12.0).unwrap()
    - r * ( T::from_f64(1.0 / 120.0).unwrap()
    - r * ( T::from_f64(1.0 / 252.0).unwrap()
    - r * ( T::from_f64(1.0 / 240.0).unwrap()
    - r * ( T::from_f64(1.0 / 132.0).unwrap()
    - r * ( T::from_f64(691.0 / 32760.0).unwrap()
    - r * ( T::from_f64(1.0 / 12.0).unwrap()
    ) ) ) ) ) ) );

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
pub fn gamma_u<T>(a: T, x: T) -> T
    where T: Field + Scalar + Exponential + Trigonometry + Power + Hyperbolic + Neg<Output = T> + Sign
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
pub fn gamma_ur<T>(a: T, x: T) -> T
    where T: Field + Scalar + Exponential + Trigonometry + Power + Hyperbolic + Neg<Output = T> + Sign
{
    T::one() - gamma_lr(a, x)
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
pub fn gamma_l<T>(a: T, x: T) -> T
    where T: Field + Scalar + Exponential + Trigonometry + Power + Hyperbolic + Neg<Output = T> + Sign
{
    gamma_lr(a, x) * gamma(a)
}


/// Lower regularized incomplete gamma function
///
/// Q(a, x) = &Gamma;(a, x) / &Gamma;(a)
///
/// Fore more information:
/// <a href="https://en.wikipedia
/// .org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables">https://en
/// .wikipedia.org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables</a>
///
/// https://people.sc.fsu.edu/~jburkardt/c_src/asa239/asa239.c
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
pub fn gamma_lr<T>(a: T, x: T) -> T
    where T: Field + Scalar + Exponential + Trigonometry + Power + Hyperbolic + Neg<Output = T> + Sign
{
//    if a.is_nan() || x.is_nan()
//    {
//        panic!();
//    }
//
//    if a <= T::zero() || a == INFINITY
//    {
//        panic!();
//    }
    if x <= T::zero() // || x == INFINITY
    {
        panic!();
    }

    let eps: T = T::from_f64(0.000000000000001_f64).unwrap();
    let big: T = T::from_f64(4503599627370496.0_f64).unwrap();
    let big_inv: T = T::from_f64(2.22044604925031308085e-16_f64).unwrap();

    if a == T::zero()
    {
        return T::one()
    }

    if x == T::zero()
    {
        return T::zero()
    }

    let ax: T = a * x.ln() - x - ln_gamma(a);

    if ax < T::from_f64(-709.78271289338399_f64).unwrap()
    {
        if a < x
        {
            return T::one()
        }
        return T::zero()
    }

    if x <= T::one() || x <= a
    {
        let mut r2: T = a;
        let mut c2: T = T::one();
        let mut ans2: T = T::one();
        loop
        {
            r2 += T::one();
            c2 *= x / r2;
            ans2 += c2;

            if c2 / ans2 <= eps
            {
                break;
            }
        }
        return ax.exp() * ans2 / a
    }

    let mut y: T = T::one() - a;
    let mut z: T = x + y + T::one();
    let mut c: T = T::zero();

    let mut p3: T = T::one();
    let mut q3: T = x;
    let mut p2: T = x + T::one();
    let mut q2: T = z * x;
    let mut ans: T = p2 / q2;

    loop
    {
        y += T::one();
        z += T::from_f64(2.0_f64).unwrap();
        c += T::one();
        let yc = y * c;

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

        if q != T::zero()
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

    T::one() - ax.exp() * ans
}