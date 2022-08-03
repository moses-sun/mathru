use crate::analysis::ode::problem::TestOde;
use mathru::{
    algebra::linear::Vector,
    analysis::differential_equation::ordinary::solver::implicit::runge_kutta::{
        ImplicitEuler, ImplicitFixedStepSizeMethod,
    },
};

#[test]
fn fn1() {
    // Create an ODE instance
    let x_0 = 0.0;
    let h = 0.1;
    let problem = TestOde {};
    let solver: ImplicitEuler<f64> = ImplicitEuler::default();

    let y_0 = vector![0.0];

    let y: Vector<f64> = solver.do_step(&problem, &x_0, &y_0, &h);

    let y_1_ref = vector![(x_0 + h).tan()];
    assert_relative_eq!(y_1_ref, y, epsilon = 0.01);
}
