//! Implicit Ordinary Differential Equation

use crate::algebra::{
    abstr::Real,
    linear::{Matrix, Vector},
};

/// Implicit ordinary differential equation
pub trait ImplicitODE<T>
where
    T: Real,
{
    fn ode(&self, t: &T, x: &Vector<T>) -> Vector<T>;

    fn jacobian(&self, t: &T, x: &Vector<T>) -> Matrix<T>;
}
