use super::ExplicitODE;
use crate::algebra::linear::{Vector};
///
pub trait ExplicitFixedStepSizeMethod<T>
{
    ///
    ///
    fn do_step<F>(self: &Self, prob: &F, t_n: &T, x_n: &Vector<T>, h: &T) -> (Vector<T>)
        where F: ExplicitODE<T>;

}