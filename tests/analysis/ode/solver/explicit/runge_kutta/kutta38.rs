use super::super::super::problem::explicit_ode;
use mathru::{
    algebra::linear::Vector,
    analysis::differential_equation::ordinary::{
        solver::explicit::runge_kutta::fixed::{ExplicitRKMethod, Kutta38},
        ExplicitInitialValueProblem,
    },
    elementary::Trigonometry,
};

#[test]
fn test_1() {
    let problem: ExplicitInitialValueProblem<f64> = explicit_ode();

    let rk = Kutta38::<f64>::default();
    let t_0 = 0.9;
    let h = 0.1;
    let t_1 = t_0 + h;

    let x_0 = vector![t_0.tan()];

    let x_1 = rk.tableau().do_step(&problem.ode(), &t_0, &x_0, &h);

    let x_1_ref = vector![t_1.tan()];

    assert_relative_eq!(x_1, x_1_ref, epsilon = 0.0000005);
}
