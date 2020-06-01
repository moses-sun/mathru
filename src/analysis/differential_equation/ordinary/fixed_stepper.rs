//! Fixed step size Stepper
use super::explicit_method::ExplicitFixedStepSizeMethod;
use super::implicit_method::ImplicitFixedStepSizeMethod;
use super::{ExplicitODE, ImplicitODE};
use crate::algebra::abstr::Real;
use crate::algebra::linear::Vector;

/// Fixed step size Stepper
pub struct ExplicitFixedStepper<T>
{
    /// Step size
    step_size: T,
}

impl<T> ExplicitFixedStepper<T> where T: Real
{
    /// Creates an instance with the given step size
    ///
    /// # Arguments
    ///
    /// * 'step_size'
    ///
    /// # Panics
    ///
    /// if 'step_size' <= 0.0
    pub fn new(step_size: T) -> ExplicitFixedStepper<T>
    {
        if step_size <= T::zero()
        {
            panic!();
        }
        return ExplicitFixedStepper { step_size };
    }

    pub fn solve<F, M>(self: &Self, prob: &F, method: &M) -> Result<(Vec<T>, Vec<Vector<T>>), ()>
        where F: ExplicitODE<T>,
              M: ExplicitFixedStepSizeMethod<T>
    {
        let t_span = prob.time_span();
        let init = prob.init_cond();
        let t_start = t_span.0;
        let t_stop = t_span.1;

        if t_start > t_stop
        {
            panic!()
        }

        let mut x_n: Vector<T> = init.clone();

        let mut t_n: T = t_start;

        let limit = ((t_stop - t_start) / self.step_size).ceil() + T::one();

        let steps: usize = limit.to_u64() as usize;
        let mut t_vec: Vec<T> = Vec::with_capacity(steps);
        let mut res_vec: Vec<Vector<T>> = Vec::with_capacity(steps);

        for _i in 0..steps
        {
            let h: T = self.step_size.min(t_stop - t_n);

            t_vec.push(t_n);
            res_vec.push(x_n.clone());

            x_n = method.do_step(prob, &t_n, &x_n, &h);

            t_n = t_n + h;
        }

        return Ok((t_vec, res_vec));
    }

    pub fn get_step_size(self: &Self) -> &T
    {
        return &self.step_size;
    }

    pub fn set_step_size(mut self: &mut Self, step_size: T)
    {
        self.step_size = step_size;
    }
}

/// Fixed step size Stepper
pub struct ImplicitFixedStepper<T>
{
    /// Step size
    step_size: T,
}

impl<T> ImplicitFixedStepper<T> where T: Real
{
    /// Creates an instance with the given step size
    ///
    /// # Arguments
    ///
    /// * 'step_size'
    ///
    /// # Panics
    ///
    /// if 'step_size' <= 0.0
    pub fn new(step_size: T) -> ImplicitFixedStepper<T>
    {
        if step_size <= T::zero()
        {
            panic!();
        }
        return ImplicitFixedStepper { step_size };
    }

    pub fn solve<F, M>(self: &Self, prob: &F, method: &M) -> Result<(Vec<T>, Vec<Vector<T>>), ()>
        where F: ImplicitODE<T>,
              M: ImplicitFixedStepSizeMethod<T>
    {
        let t_span = prob.time_span();
        let init = prob.init_cond();
        let t_start = t_span.0;
        let t_stop = t_span.1;

        if t_start > t_stop
        {
            panic!()
        }

        let mut x_n: Vector<T> = init.clone();

        let mut t_n: T = t_start;

        let limit = ((t_stop - t_start) / self.step_size).ceil() + T::one();

        let steps: usize = limit.to_u64() as usize;

        let mut t_vec: Vec<T> = Vec::with_capacity(steps);
        let mut res_vec: Vec<Vector<T>> = Vec::with_capacity(steps);

        for _i in 0..steps
        {
            let h: T = self.step_size.min(t_stop - t_n);

            t_vec.push(t_n);
            res_vec.push(x_n.clone());

            x_n = method.do_step(prob, &t_n, &x_n, &h);

            t_n = t_n + h;
        }

        return Ok((t_vec, res_vec));
    }

    pub fn get_step_size(self: &Self) -> &T
    {
        return &self.step_size;
    }

    pub fn set_step_size(mut self: &mut Self, step_size: T)
    {
        self.step_size = step_size;
    }
}
