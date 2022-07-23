use crate::analysis::differential_equation::ordinary::solver::explicit::runge_kutta::adaptive::ExplicitRKEmbedded;

pub trait ExplicitRKEmbeddedMethod<T> {
    fn tableau<'a>(&'a self) -> &'a ExplicitRKEmbedded<T>;
}
