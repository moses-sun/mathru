use super::super::super::problem::explicit_ode;
use mathru::{
    algebra::linear::Vector,
    analysis::differential_equation::ordinary::{
        solver::explicit::runge_kutta::adaptive::{DormandPrince54, ExplicitRKEmbeddedMethod},
        ExplicitInitialValueProblem,
    },
    elementary::Trigonometry,
};

#[test]
fn test_1() {
    let problem: ExplicitInitialValueProblem<f64> = explicit_ode();

    let rk = DormandPrince54::<f64>::default();
    let t_0 = 0.9;
    let h = 0.1;
    let t_1 = t_0 + h;

    let x_0 = vector![t_0.tan()];

    let (x_1, x_1_s) = rk.tableau().do_step(&problem.ode(), &t_0, &x_0, &h);

    let x_1_ref = vector![t_1.tan()];

    assert_relative_eq!(x_1, x_1_ref, epsilon = 0.000002);
    assert_relative_eq!(x_1_s, x_1_ref, epsilon = 0.000003);
}