use crate::algebra::linear::{Vector, Matrix};
use crate::algebra::abstr::Field;

//use super::Function;

pub enum Method{

    RK4,
}


pub trait Solver<T>
where T: Field,
{
    /**
     * Solve the ordinary diffential equation, returning the results of the calculation.
     *
     */
    fn solve<F>(self: &Self, function: F, initial_cond: Vector<T>, t_start: T, t_end: T) -> (Vector<T>, Matrix<T>)
    where
        F: Fn(&T, &Vector<T>) -> Vector<T>;

}