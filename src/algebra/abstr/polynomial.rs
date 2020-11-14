//! Polynomial
use std::fmt::{Result, Formatter, Display};
use crate::algebra::abstr::{Real, AbsDiffEq};
use crate::algebra::{
    abstr::{Field, Scalar},
};
use std::ops::{Add, Mul, Div, Sub, Neg};
use crate::algebra::abstr::Zero;
use crate::abs_diff_eq;


/// Polynomial expression
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Polynomial<T> {
    coef: Vec<T>,
}

impl<T> Polynomial<T>
{
    /// Creates a new polynomial
    ///
    /// # Arguments
    ///
    /// * `coef:  Coefficients
    ///
    /// # Panics
    ///
    /// If the coef is an empty vector
    ///
    /// # Example
    ///
    /// ```math
    /// (1 + 2x + 3x^2)
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::new(vec![1.0, 2.0, 3.0]);
    /// ```
    pub fn new(coef: Vec<T>) -> Polynomial<T>
    {
        if coef.len() == 0
        {
            panic!()
        }

        Polynomial
        {
            coef
        }
    }
}

impl<T> Display for Polynomial<T>
    where T: Display + Real
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        let mut only_zero_terms: bool = true;

        for (i, a_i) in self.coef.iter().enumerate()
        {
            if a_i.abs_diff_ne(&T::zero(), T::default_epsilon())
            {
                if i > 1
                {
                    if only_zero_terms
                    {
                        write!(f, "{}x^{}", a_i, i)?;
                    }
                    else
                    {
                        write!(f, " + {}x^{}", a_i, i)?;
                    }
                } else {
                    if i == 0
                    {
                        write!(f, "{}", a_i)?;
                    }
                    else {
                        if only_zero_terms
                        {
                            write!(f, "{}x", a_i)?;
                        } else {
                            write!(f, " + {}x", a_i)?
                        }
                    }
                }

                only_zero_terms = false
            }
        }

        Ok(())
    }
}

impl<T> Polynomial<T>
    where T: Field + Scalar
{
    /// Evaluate the polynomial with horner's rule
    ///
    /// # Argument
    ///
    /// * `x`: The polynomial is evaluated at this value
    ///
    /// # Example
    ///
    /// ```math
    /// (1 + 2x + 3x^2) = p(x) \\\\
    /// p(2) = 17
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::new(vec![1.0, 2.0, 3.0]);
    ///
    /// assert_eq!(17.0, a.eval(2.0));
    /// ```
    pub fn eval(self: &Self, x: T) -> T
    {
        let mut s: T = T::zero();

        for v in self.coef.iter().rev()
        {
            s = *v + (x * s);
        }
        s
    }
}


impl<'a, 'b, T> Add<&'b Polynomial<T>> for &'a Polynomial<T>
    where T: Field + Scalar
{
    type Output = Polynomial<T>;

    /// Adds two polynomials
    ///
    /// # Example
    ///
    /// ```math
    /// (1 + 2x + 3x^2) + (1 + 2x) = 2 + 4x + 3x^2
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::new(vec![1.0, 2.0, 3.0]);
    /// let b: Polynomial<f64> = Polynomial::new(vec![1.0, 2.0]);
    /// let c: Polynomial<f64> = Polynomial::new(vec![2.0, 4.0, 3.0]);
    ///
    /// assert_eq!(c, &a + &b);
    /// ```
    fn add(self: Self, rhs: &'b Polynomial<T>) -> Self::Output
    {
        let mut sum = if self.coef.len() > rhs.coef.len()
        {
            self.coef.clone()
        }
        else {
            rhs.coef.clone()
        };

        for (i, (a_i, b_i)) in self.coef.iter().zip(rhs.coef.iter()).enumerate()
        {
            sum[i] = *a_i + *b_i
        }

        return Polynomial::new(sum)
    }
}

impl<'a, 'b, T> Sub<&'b Polynomial<T>> for &'a Polynomial<T>
    where T: Field + Scalar
{
    type Output = Polynomial<T>;

    /// Subtracts two polynomials
    ///
    /// # Example
    ///
    /// ```math
    /// (1 + 2x + 3x^2) - (1 + 2x) = 3x^2
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::new(vec![1.0, 2.0, 3.0]);
    /// let b: Polynomial<f64> = Polynomial::new(vec![1.0, 2.0]);
    /// let c: Polynomial<f64> = Polynomial::new(vec![0.0, 0.0, 3.0]);
    ///
    /// assert_eq!(c, &a - &b);
    /// ```
    fn sub(self: Self, rhs: &'b Polynomial<T>) -> Self::Output
    {
        let mut sum = if self.coef.len() > rhs.coef.len()
        {
            self.coef.clone()
        }
        else {
            rhs.coef.clone()
        };

        for (i, (a_i, b_i)) in self.coef.iter().zip(rhs.coef.iter()).enumerate()
        {
            sum[i] = *a_i - *b_i
        }

        return Polynomial::new(sum)
    }
}

impl<T> Zero for Polynomial<T>
    where T: Zero
{
    fn zero() -> Polynomial<T>
    {
        return Polynomial::new(vec![T::zero()])
    }
}

impl<'a, T> Neg for &'a Polynomial<T>
    where T: Field
{
    type Output = Polynomial<T>;

    /// Negates the polynomial
    ///
    /// # Example
    ///
    /// ```math
    /// -(1 + 2x) = -1 - 2x
    /// ```
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::new(vec![1.0, 2.0]);
    ///
    /// assert_eq!(Polynomial::new(vec![-1.0, -2.0]), -a);
    /// ```
    ///
    fn neg(self) -> Self::Output
    {
        Polynomial::new(
            self.coef.clone().into_iter().map(|x| -x).collect::<Vec<T>>()
        )
    }
}

impl<T> Neg for Polynomial<T>
    where T: Field
{
    type Output = Polynomial<T>;

    /// Returns the negative of a polynomial
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::new(vec![1.0, 2.0, 3.0]);
    ///
    /// assert_eq!(Polynomial::new(vec![-1.0, -2.0, -3.0]), -a)
    /// ```
    fn neg(self) -> Self::Output
    {
        Polynomial::new(
            self.coef.into_iter().map(|x| -x).collect::<Vec<T>>()
        )
    }
}

impl<T> Polynomial<T>
{
    /// Returns the degree of the polynomial
    ///
    /// # Example
    ///
    /// ```math
    /// deg(1 + 2x + 3x^2) =2
    /// ```
   ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::new(vec![1.0, 2.0, 3.0]);
    ///
    /// assert_eq!(2, a.degree())
    /// ```
    pub fn degree(&self) -> usize
    {
        return self.coef.len() - 1
    }
}



impl<'a, 'b, T> Mul<&'b Polynomial<T>> for &'a Polynomial<T>
    where T: Field + Scalar
{
    type Output = Polynomial<T>;

    /// Multiplies two polynomials
    ///
    /// # Arguments
    ///
    /// * `self': Factor
    /// * `rhs?: Factor
    ///
    /// # Example
    ///
    /// ```math
    /// (1 + 2x + 3x^2)(1 + x) = (1 + 3x + 5x^2 + 3x^3)
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::new(vec![1.0, 2.0, 3.0]);
    /// let b: Polynomial<f64> = Polynomial::new(vec![1.0, 1.0]);
    /// let c: Polynomial<f64> = Polynomial::new(vec![1.0, 3.0, 5.0, 3.0]);
    ///
    /// assert_eq!(c, &a * &b)
    /// ```
    fn mul(self: Self, rhs: &'b Polynomial<T>) -> Self::Output
    {

        let deg_lhs = self.degree();
        let deg_rhs = rhs.degree();

        let deg_res =  deg_lhs + deg_rhs;
        let mut res: Vec<T> = vec![T::zero(); deg_res + 1];

        for (i, v_i) in self.coef.iter().enumerate()
        {
            for (j, v_j) in rhs.coef.iter().enumerate()
            {
                res[i + j] += *v_i * *v_j
            }
        }

        return Polynomial::new(res);
    }
}


impl<'a, 'b, T> Div<&'b Polynomial<T>> for &'a Polynomial<T>
    where T: Field + Scalar
{
    type Output = (Polynomial<T>, Polynomial<T>);

    /// Dividedes two polynomials
    ///
    /// # Example
    ///
    /// ```math
    /// (3x^3 + 5x^2 + 3x + 1) / (3x^2 + 2x + 1) = (x + 1)
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    /// use crate::mathru::algebra::abstr::Zero;
    ///
    /// let a: Polynomial<f64> = Polynomial::new(vec![1.0, 2.0, 3.0]);
    /// let b: Polynomial<f64> = Polynomial::new(vec![1.0, 1.0]);
    /// let c: Polynomial<f64> = Polynomial::new(vec![1.0, 3.0, 5.0, 3.0]);
    ///
    /// assert_eq!(b, (&c / &a).0);
    /// assert_eq!(Polynomial::zero(), (&c / &a).1)
    /// ```
    fn div(self: Self, rhs: &'b Polynomial<T>) -> Self::Output
    {
        if rhs.degree() > self.degree()
        {
            return (Polynomial::zero(), self.clone());
        }

        let mut remainder: Vec<T> = self.coef.clone();
        let quotient_degree: usize = self.degree() - rhs.degree();
        let mut quotient = vec![T::zero(); quotient_degree + 1];

        for i in (0..(quotient_degree + 1)).rev()
        {
            let q: T = remainder[rhs.degree()  + i] / rhs.coef[rhs.degree()];

            quotient[i] = q;

            for (k, v_k) in rhs.coef.iter().enumerate()
            {
                remainder[k + i] -= *v_k * q;
            }
        }

        return (Polynomial::new(quotient), Polynomial::new(Polynomial::reduce_coef(remainder)));
    }
}

impl<T> Polynomial<T>
    where T: Zero + AbsDiffEq
{
    fn reduce_coef(mut coef: Vec<T>) -> Vec<T>
    {
        let len = coef.len();
        for i in (1..len).rev()
        {
            let v = &coef[i];
            if abs_diff_eq!(*v, T::zero())
            {
                coef.pop();
            }
            else {
                break;
            }
        }

        return coef;
    }

    pub fn reduce(self: Self) -> Self
    {
       return Polynomial::new(Polynomial::reduce_coef(self.coef));
    }
}

impl<T> Polynomial<T>
    where T: Field + Scalar
{
    /// Differentiate polynomial
    ///
    /// # Example
    ///
    /// ```math
    /// p(x) = 3x^2 + 5x^2 + 3x + 1
    /// ```
    ///
    /// ```math
    /// \frac{\partial p(x)}{\partial x} = 9x^2 + 10x + 3
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let c: Polynomial<f64> = Polynomial::new(vec![1.0, 3.0, 5.0, 3.0]);
    /// let c_s: Polynomial<f64> = Polynomial::new(vec![3.0, 10.0, 9.0]);
    ///
    /// assert_eq!(c_s, c.differentiate());
    /// ```
    pub fn differentiate(self: &Self) -> Polynomial<T>
    {
        if self.degree() == 0
        {
            return Polynomial::new(vec![T::zero()]);
        }

        let mut coef_diff = Vec::with_capacity(self.degree());
        for (i, a_i) in self.coef.iter().skip(1).enumerate()
        {
            coef_diff.push(T::from_f64((i + 1) as f64) * *a_i);
        }

        return Polynomial::new(Polynomial::reduce_coef(coef_diff));
    }

    /// Integrate polynomial
    ///
    /// # Example
    ///
    /// ```math
    /// p(x) = 1 + 2x + 3x^2
    /// ```
    ///
    /// ```math
    /// \int p(x) dx = c + x + x^2 + x^3
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let c: Polynomial<f64> = Polynomial::new(vec![1.0, 2.0, 3.0]);
    /// let c_s: Polynomial<f64> = Polynomial::new(vec![0.0, 1.0, 1.0, 1.0]);
    ///
    /// assert_eq!(c_s, c.integrate());
    /// ```
    pub fn integrate(self: &Self) -> Polynomial<T>
    {
        let mut coef_int = Vec::with_capacity(self.degree() + 1);
        coef_int.push(T::zero());

        for (i, a_i) in self.coef.iter().enumerate()
        {
            coef_int.push(*a_i / T::from_f64((i + 1) as f64));
        }

        return Polynomial::new(Polynomial::reduce_coef(coef_int));
    }
}
