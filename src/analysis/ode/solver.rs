use crate::algebra::linear::{Vector};
use crate::algebra::abstr::Field;


/// Trait which is implemented by every ode algorithm
pub trait Solver<T>
    where T: Field,
{
    ///Solve the ordinary diffential equation, returning the results of the calculation.
    fn solve<F>(self: &Self, function: F, init_cond: Vector<T>, t_span: (T, T)) -> Result<(Vec<T>, Vec<Vector<T>>),
     ()>
        where F: Fn(&T, &Vector<T>) -> Vector<T>;

}