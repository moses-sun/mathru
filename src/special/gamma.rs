//! Provides [gamma](https://en.wikipedia.org/wiki/Beta_function) related functions

use crate::algebra::abstr::Real;
use crate::elementary::Power;

/// Provides [gamma](https://en.wikipedia.org/wiki/Beta_function) related functions
pub trait Gamma
{
    /// Gamma function
    ///
    /// ```math
    /// \Gamma(z) = \int_0^\infty t^{z-1} {\mathrm e}^{-t} \mathrm dt
    /// ```
    ///
    /// Fore more information:
    /// <a href="https://en.wikipedia.org/wiki/Gamma_function">https://en.wikipedia.org/wiki/Gamma_function</a>
    ///
    /// # Arguments
    ///
    /// * `self` > 0.0
    ///
    /// # Panics
    ///
    /// *`self` <= 0.0
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::gamma::Gamma;
    ///
    /// let z: f64 = 0.3_f64;
    /// let gamma: f64 = z.gamma();
    /// ```
    fn gamma(self: Self) -> Self;


    /// Log-gamma function
    ///
    /// ln&Gamma;(z)
    //
    /// Fore more information:
    /// <a href="https://en.wikipedia.org/wiki/Gamma_function#The_log-gamma_function">https://en.wikipedia.org/wiki/Gamma_function#The_log-gamma_function</a>
    ///
    /// # Arguments
    ///
    /// * `self`
    ///
    /// ```
    /// use mathru::special::gamma::Gamma;
    ///
    /// let x: f64 = 0.3_f64;
    /// let ln_gamma: f64 = x.ln_gamma();
    /// ```
    fn ln_gamma(self: Self) -> Self;

    /// Digamma function
    ///
    /// ```math
    /// \psi(x)=\frac{d}{dx}\ln\big(\Gamma(x)\big)=\frac{\Gamma'(x)}{\Gamma(x)}
    /// ```
    ///
    /// Fore more information:
    /// <a href="https://en.wikipedia.org/wiki/Digamma_function">https://en.wikipedia.org/wiki/Digamma_function</a>
    ///
    /// # Arguments
    ///
    /// * `self`
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::gamma::Gamma;
    ///
    /// let x: f64 = 0.3_f64;
    /// let digamma: f64 = x.digamma();
    /// ```
    fn digamma(self: Self) -> Self;

    /// Upper incomplete gamma function
    ///
    /// ```math
    /// \Gamma(s,x) = \int_x^{\infty} t^{s-1}\,\mathrm{e}^{-t}\,{\rm d}t
    /// ```
    ///
    /// Fore more information:
    /// <a href="https://en.wikipedia.org/wiki/Incomplete_gamma_function#Upper_incomplete_Gamma_function">https://en.wikipedia.org/wiki/Incomplete_gamma_function#Upper_incomplete_Gamma_function</a>
    ///
    /// # Arguments
    ///
    /// * `self`
    /// * `x`
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::gamma::Gamma;
    ///
    /// let a: f64 = 0.5_f64;
    /// let x: f64 = 0.3_f64;
    ///
    /// let gamma_u: f64 = a.gamma_u(x);
    /// ```
    fn gamma_u(self: Self, x: Self) -> Self;

    /// Upper incomplete regularized gamma function
    ///
    /// ```math
    /// P(s,x)=\frac{\gamma(s,x)}{\Gamma(s)}
    /// ```
    ///
    /// Fore more information:
    /// <a href="https://en.wikipedia
    /// .org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables">https://en
    /// .wikipedia.org/wiki/Incomplete_gamma_function#
    /// Regularized_Gamma_functions_and_Poisson_random_variables</a>
    ///
    /// # Arguments
    ///
    /// * `self`
    /// * `x`
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::gamma::Gamma;
    ///
    /// let a: f64 = 0.5_f64;
    /// let x: f64 = 0.3_f64;
    /// let gamma_ur: f64 = a.gamma_ur(x);
    /// ```
    fn gamma_ur(self: Self, x: Self) -> Self;

    /// Lower incomplete gamma function
    ///
    /// ```math
    /// \gamma(s,x) = \int_0^x t^{s-1}\,\mathrm{e}^{-t}\,{\rm d}t
    /// ```
    ///
    /// Fore more information:
    /// <a href="https://en.wikipedia
    /// .org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables">https://en
    /// .wikipedia.org/wiki/Incomplete_gamma_function#
    /// Regularized_Gamma_functions_and_Poisson_random_variables</a>
    ///
    /// # Arguments
    ///
    /// * `self`
    /// * `x`
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::gamma::Gamma;
    ///
    /// let a: f64 = 0.5_f64;
    /// let x: f64 = 0.3_f64;
    /// let gamma_l: f64 = a.gamma_l(x);
    /// ```
    fn gamma_l(self: Self, x: Self) -> Self;

    /// Lower regularized incomplete gamma function
    ///
    /// ```math
    /// Q(s,x)=\frac{\Gamma(s,x)}{\Gamma(s)}=1-P(s,x)
    /// ```
    ///
    /// Fore more information:
    /// <a href="https://en.wikipedia
    /// .org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables">https://en
    /// .wikipedia.org/wiki/Incomplete_gamma_function#
    /// Regularized_Gamma_functions_and_Poisson_random_variables</a>
    ///
    /// # Arguments
    ///
    /// * `self`
    /// * `x`
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::special::gamma::Gamma;
    ///
    /// let a: f64 = 0.5_f64;
    /// let x: f64 = 0.3_f64;
    /// let gamma_lr: f64 = a.gamma_lr(x);
    /// ```
    fn gamma_lr(self: Self, x: Self) -> Self;
}

macro_rules! impl_gamma
{
    ($t: ty, $PI: expr, $E: expr) =>
    {
        impl Gamma for $t
        {
            fn gamma(self: Self) -> Self
            {
                if self == 0.0
                {
                panic!("The gamma function is undefined for z == 0.0")
                }

                if self < 0.5
                {
                    return $PI / (($PI * self).sin() * (1.0 - self).gamma()); //reflection formula
                }

                let t: $t = self + 6.5;
                let x: $t = 0.99999999999980993 + 676.5203681218851 / self
                    - 1259.1392167224028 / (self + 1.0)
                    + 771.32342877765313 / (self + 2.0)
                    - 176.61502916214059 / (self + 3.0)
                    + 12.507343278686905 / (self + 4.0)
                    - 0.13857109526572012 / (self + 5.0)
                    + 9.9843695780195716e-6 / (self + 6.0)
                    + 1.5056327351493116e-7 / (self + 7.0);

                return 2.0.sqrt() * $PI.sqrt()
                    * t.pow((self - 0.5))
                    * (-t).exp()
                    * x;
            }

            fn ln_gamma(self: Self) -> Self
            {
                // Auxiliary variable when evaluating the `gamma_ln` function
                let gamma_r: Self = 10.900511;

                // Polynomial coefficients for approximating the `gamma_ln` function
                let gamma_dk: &[$t] = &[2.48574089138753565546e-5,
                           1.05142378581721974210,
                           -3.45687097222016235469,
                           4.51227709466894823700,
                           -2.98285225323576655721,
                           1.05639711577126713077,
                           -1.95428773191645869583e-1,
                           1.70970543404441224307e-2,
                           -5.71926117404305781283e-4,
                           4.63399473359905636708e-6,
                           -2.71994908488607703910e-9];

                let x: Self = self;

                if x < 0.5
                {
                    let s = gamma_dk.iter()
                        .enumerate()
                        .skip(1)
                        .fold(gamma_dk[0], |s, t| s + *t.1 / ((t.0 as u64) as $t - x));

                    $PI.ln()
                    - ($PI * x).sin().ln()
                    - s.ln()
                    - (2.0 * ($E / $PI).pow(0.5)).ln()
                    - (0.5 - x) * ((0.5 - x + gamma_r) / $E).ln()
                }
                else
                {
                    let s = gamma_dk.iter()
                        .enumerate()
                        .skip(1)
                        .fold(gamma_dk[0], |s, t| {
                            s + *t.1 / (x + (t.0 as u64) as $t - 1.0)
                        });

                    s.ln()
                    + (2.0 * ($E / $PI).pow(0.5)).ln()
                    + (x - 0.5) * ((x - 0.5 + gamma_r) / $E).ln()
                }
            }

            fn digamma(self: Self) -> Self
            {
                let c: Self = 8.5;
                let mut value: Self = 0.0;
                let mut x2: Self = self;
                //The comparison only compares the real part ot the number
                while x2 < c
                {
                    value = value - 1.0 / x2;
                    x2 = x2 + 1.0;
                }
                /*
                  Use Stirling's (actually de Moivre's) expansion.
                */
                let mut r: Self = 1.0 / x2;
                value = value + x2.ln() - 0.5 * r;

                r = r * r;

                value = value
                        - r
                          * (1.0 / 12.0
                             - r
                               * (1.0 / 120.0
                                  - r
                                    * (1.0 / 252.0
                                       - r
                                         * (1.0 / 240.0
                                            - r
                                              * (1.0 / 132.0
                                                 - r
                                                   * (691.0 / 32760.0
                                                      - r * (1.0 / 12.0)))))));

                return value;
            }

            fn gamma_u(self: Self, x: Self) -> Self
            {
                return self.gamma_ur(x) * self.gamma();
            }

            fn gamma_ur(self: Self, x: Self) -> Self
            {
                return 1.0 - self.gamma_lr(x);
            }

            fn gamma_l(self: Self, x: Self) -> Self
            {
                return self.gamma_lr(x)  * self.gamma();
            }

            fn gamma_lr(self: Self, x: Self) -> Self
            {

                if x <= 0.0
                {
                    panic!("Lower regularized gamma function is not defined for `x` <= 0.0");
                }

                let eps: Self = 0.000000000000001;
                let big: Self = 4503599627370496.0;
                let big_inv: Self = 2.22044604925031308085e-16;

                if self == 0.0
                {
                    return 1.0;
                }

                if x == 0.0
                {
                    return 0.0;
                }

                let ax: Self = self * x.ln() - x - self.ln_gamma();

                if ax < -709.78271289338399
                {
                    if self < x
                    {
                        return 1.0;
                    }
                    return 0.0;
                }

                if x <= 1.0 || x <= self
                {
                    let mut r2: Self = self;
                    let mut c2: Self = 1.0;
                    let mut ans2: Self = 1.0;
                    loop
                    {
                        r2 += 1.0;
                        c2 *= x / r2;
                        ans2 += c2;

                        if c2 / ans2 <= eps
                        {
                            break;
                        }
                    }
                    return ax.exp() * ans2 / self;
                }

                let mut y: Self = 1.0 - self;
                let mut z: Self = x + y + 1.0;
                let mut c: Self = 0.0;

                let mut p3: Self = 1.0;
                let mut q3: Self = x;
                let mut p2: Self = x + 1.0;
                let mut q2: Self = z * x;
                let mut ans: Self = p2 / q2;

                loop
                {
                    y += 1.0;
                    z += 2.0;
                    c += 1.0;
                    let yc: Self = y * c;

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

                    if q != 0.0
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

                1.0 - ax.exp() * ans
            }
        }
    };
}

impl_gamma!(f64, std::f64::consts::PI, std::f64::consts::E);
impl_gamma!(f32, std::f32::consts::PI, std::f32::consts::E);


/// Gamma function
///
/// ```math
/// \Gamma(z) = \int_0^\infty t^{z-1} {\mathrm e}^{-t} \mathrm dt
/// ```
///
/// Fore more information:
/// <a href="https://en.wikipedia.org/wiki/Gamma_function">https://en.wikipedia.org/wiki/Gamma_function</a>
///
/// # Arguments
///
/// * `z`
///
/// # Panics
///
/// *`z` == 0.0
///
/// # Example
///
/// ```
/// use mathru::special::gamma;
///
/// let z: f64 = 0.3_f64;
/// let gamma: f64 = gamma::gamma(z);
/// ```
/// The following approximation is implemented
/// https://en.wikipedia.org/wiki/Lanczos_approximation
pub fn gamma<T>(z: T) -> T
    where T: Real + Gamma
{
    return z.gamma();
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
/// use mathru::special::gamma;
///
/// let x: f64 = 0.3_f64;
/// let ln_gamma: f64 = gamma::ln_gamma(x);
/// ```
pub fn ln_gamma<T>(x: T) -> T
    where T: Gamma
{
    return x.gamma();
}

/// Digamma function
///
/// ```math
/// \psi(x)=\frac{d}{dx}\ln\big(\Gamma(x)\big)=\frac{\Gamma'(x)}{\Gamma(x)}
/// ```
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
/// use mathru::special::gamma;
///
/// let x: f64 = 0.3_f64;
/// let digamma: f64 = gamma::digamma(x);
/// ```
pub fn digamma<T>(x: T) -> T
    where T: Gamma
{
    return x.digamma();
}

/// Upper incomplete gamma function
///
/// ```math
/// \Gamma(s,x) = \int_x^{\infty} t^{s-1}\,\mathrm{e}^{-t}\,{\rm d}t
/// ```
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
/// use mathru::special::gamma;
///
/// let a: f64 = 0.5_f64;
/// let x: f64 = 0.3_f64;
///
/// let gamma_u: f64 = gamma::gamma_u(a, x);
/// ```
pub fn gamma_u<T>(a: T, x: T) -> T
    where T: Real + Gamma
{
    return a.gamma_u(x);
}

/// Upper incomplete regularized gamma function
///
/// ```math
/// P(s,x)=\frac{\gamma(s,x)}{\Gamma(s)}
/// ```
///
/// Fore more information:
/// <a href="https://en.wikipedia
/// .org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables">https://en
/// .wikipedia.org/wiki/Incomplete_gamma_function#
/// Regularized_Gamma_functions_and_Poisson_random_variables</a>
///
/// # Arguments
///
/// * `a`
/// * `x`
///
/// # Example
///
/// ```
/// use mathru::special::gamma;
///
/// let a: f64 = 0.5_f64;
/// let x: f64 = 0.3_f64;
/// let gamma_u: f64 = gamma::gamma_ur(a, x);
/// ```
pub fn gamma_ur<T>(a: T, x: T) -> T
    where T: Real + Gamma
{
    return a.gamma_ur(x);
}

/// Lower incomplete gamma function
///
/// ```math
/// \gamma(s,x) = \int_0^x t^{s-1}\,\mathrm{e}^{-t}\,{\rm d}t
/// ```
///
/// Fore more information:
/// <a href="https://en.wikipedia
/// .org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables">https://en
/// .wikipedia.org/wiki/Incomplete_gamma_function#
/// Regularized_Gamma_functions_and_Poisson_random_variables</a>
///
/// # Arguments
///
/// * `a`
/// * `x`
///
/// # Example
///
/// ```
/// use mathru::special::gamma;
///
/// let a: f64 = 0.5_f64;
/// let x: f64 = 0.3_f64;
/// let gamma_l: f64 = gamma::gamma_l(a, x);
/// ```
pub fn gamma_l<T>(a: T, x: T) -> T
    where T: Real + Gamma
{
    return a.gamma_l(x);
}

/// Lower regularized incomplete gamma function
///
/// ```math
/// Q(s,x)=\frac{\Gamma(s,x)}{\Gamma(s)}=1-P(s,x)
/// ```
///
/// Fore more information:
/// <a href="https://en.wikipedia
/// .org/wiki/Incomplete_gamma_function#Regularized_Gamma_functions_and_Poisson_random_variables">https://en
/// .wikipedia.org/wiki/Incomplete_gamma_function#
/// Regularized_Gamma_functions_and_Poisson_random_variables</a>
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
/// use mathru::special::gamma;
///
/// let a: f64 = 0.5_f64;
/// let x: f64 = 0.3_f64;
/// let gamma_lr: f64 = gamma::gamma_lr(a, x);
/// ```
pub fn gamma_lr<T>(a: T, x: T) -> T
    where T: Real + Gamma
{
    return a.gamma_lr(x);
}
