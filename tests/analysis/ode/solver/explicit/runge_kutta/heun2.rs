use crate::analysis::ode::problem::explicit_ode;
use mathru::{
    algebra::linear::vector::Vector,
    analysis::differential_equation::ordinary::solver::explicit::runge_kutta::fixed::{
        ExplicitRKMethod, Heun2,
    },
};

#[test]
fn test_1() {
    let problem = explicit_ode();

    let rk = Heun2::<f64>::default();
    let t_0 = 0.9f64;
    let h = 0.1;
    let t_1 = t_0 + h;

    let x_0 = vector![t_0.tan()];

    let x_1 = rk.tableau().do_step(problem.ode(), &t_0, &x_0, &h);

    let x_1_ref = vector![t_1.tan()];

    assert_relative_eq!(x_1, x_1_ref, epsilon = 0.003);
}
