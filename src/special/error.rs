use crate::algebra::abstr::Real;

/// Error Function
pub fn erf<T>(x: T) -> T
    where T: Real
{
    let x_squared: T = x * x;
    let a: T = T::from_f64(0.140012);
    let b: T =
        -x_squared * (T::from_f64(4.0) / T::pi() + a * x_squared) / (T::one() + a * x_squared);
    let error: T = x.sign() * (T::one() - b.exp()).sqrt();

    return error;
}

pub fn inv_erf<T>(x: T) -> T
    where T: Real
{
    return T::zero();
}