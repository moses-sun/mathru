use crate::analysis::differential_equation::ordinary::solver::explicit::runge_kutta::fixed::ExplicitRK;

pub trait ExplicitRKMethod<T> {
    fn tableau<'a>(&'a self) -> &'a ExplicitRK<T>;
}
