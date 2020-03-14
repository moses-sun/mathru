use crate::algebra::linear::{Vector, Matrix};
use crate::algebra::abstr::Real;
use super::implicit_method::{ImplicitFixedStepSizeMethod};
use super::ImplicitODE;
use crate::analysis::NewtonRaphson;
use crate::analysis::ode::fixed_stepper::ImplicitFixedStepper;
use crate::analysis::{Function, Jacobian};
use std::marker::PhantomData;

/// Solves an ordinary differential equation using Euler's method.
///
/// <a href="https://en.wikipedia.org/wiki/Backward_Euler_method">https://en.wikipedia.org/wiki/Backward_Euler_method</a>
pub struct ImplicitEuler<T>
{
    stepper: ImplicitFixedStepper<T>,

    root_finder: NewtonRaphson<T>,
}

impl<T> ImplicitEuler<T>
    where T: Real
{
    /// Creates a backward Euler instance
    ///
    pub fn new(step_size: T) -> ImplicitEuler<T>
    {
        return ImplicitEuler
        {
            stepper: ImplicitFixedStepper::new(step_size),
            root_finder: NewtonRaphson::new(100, T::from_f64(1.0e-6))
        };
    }

    pub fn solve<F>(self: &Self, prob: &F) -> Result<(Vec<T>, Vec<Vector<T>>), ()>
        where F: ImplicitODE<T>
    {
        return self.stepper.solve(prob, self);
    }

    pub fn get_step_size(self: &Self) -> &T
    {
        return self.stepper.get_step_size();
    }

    pub fn set_step_size(self: &mut Self, step_size: T)
    {
        self.stepper.set_step_size(step_size)
    }
}

impl<T> ImplicitFixedStepSizeMethod<T> for ImplicitEuler<T>
    where T: Real
{
    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> Vector<T>
        where F: ImplicitODE<T>
    {
        let ie_helper = ImplicitEulerHelper::new(prob, t_n, x_n, h);
        //println!("x_n = {}", x_n);
        let x_n = self.root_finder.find_root(&ie_helper, x_n).unwrap();

        //println!("x_n+1 = {}", x_n);
        return x_n;
    }

    /// Euler's method is a first order method
    fn order(self: &Self) -> u8
    {
        return 1;
    }
}

struct ImplicitEulerHelper<'a, T, F>
    where T: Real, F: ImplicitODE<T>
{
    function: &'a F,
    t: &'a T,
    x: &'a Vector<T>,
    h: &'a T,
    phantom: PhantomData<T>
}


impl<'a, T, F> ImplicitEulerHelper<'a, T, F>
    where T: Real, F: ImplicitODE<T> + Jacobian<T>
{
    pub fn new(function: &'a F, t: &'a T, x: &'a Vector<T>, h: &'a T) -> ImplicitEulerHelper<'a, T, F>
    {
        return ImplicitEulerHelper
        {
            function: function,
            t: t,
            x: x,
            h: h,
            phantom: PhantomData,
        }
    }
}



impl<'a, T, F> Function<Vector<T>> for ImplicitEulerHelper<'a, T, F>
    where T: Real, F: ImplicitODE<T>
{
    type Codomain = Vector<T>;

    ///$`x(t_{n+1}) = y(t_n) + hf(t_{n+1}, x(t_{n+1})`$
    ///
    /// g(z) = y(t_n) + hf(t_{n+1}, z) - z)`$
    fn eval(self: &Self, z: &Vector<T>) -> Vector<T>
    {
        let t_n1 = *self.t + *self.h;
        //println!("y_n: {}, x:{}", self.x, z);
        let result = &(self.x + &(&self.function.func(&t_n1, z) * self.h)) - z;

        //println!("g(x): {}", result);
        return result;
    }
}

impl<'a, T, F> Jacobian<T> for ImplicitEulerHelper<'a, T, F>
    where T: Real, F: ImplicitODE<T> + Jacobian<T>
{
    ///
    /// $` \frac{\partial g(z)}{\partial z} = h \frac{\partial f(t_{n+1}, z)}{\partial z} - I`$
    fn jacobian(self: &Self, z: &Vector<T>) -> Matrix<T>
    {
        let (m, _n): (usize, usize) = z.dim();
        let jacobian =  self.function.jacobian(z) * *self.h - Matrix::one(m);

        //println!("J_g(x) = {}", jacobian);

        return jacobian;
    }
}