use mathru::{
    algebra::linear::vector::Vector,
    analysis::differential_equation::ordinary::solver::implicit::runge_kutta::{
        ImplicitEuler, ImplicitFixedStepper,
    },
};

use crate::analysis::ode::problem::implicit_ode;

#[test]
fn solve() {
    let step_size: f64 = 0.0001;

    let problem = implicit_ode();
    let solver: ImplicitFixedStepper<f64> = ImplicitFixedStepper::new(step_size);

    // Solve the ODE
    let (_t, x): (Vec<f64>, Vec<Vector<f64>>) =
        solver.solve(&problem, &ImplicitEuler::default()).unwrap();

    let x_1_ref = vector![(problem.t_end().unwrap()).tan()];
    assert_relative_eq!(x_1_ref, x.last().unwrap(), epsilon = 0.01);
}

#[test]
fn set_get_step_size() {
    let step_size: f64 = 0.1;

    let mut solver: ImplicitFixedStepper<f64> = ImplicitFixedStepper::new(0.2);

    solver.set_step_size(step_size);

    assert_eq!(step_size, *solver.get_step_size());
}
