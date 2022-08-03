use crate::analysis::ode::problem::implicit_ode;
use mathru::{
    algebra::linear::Vector, analysis::differential_equation::ordinary::solver::implicit::BDF,
};

#[test]
fn fn1() {
    let problem = implicit_ode();
    let solver: BDF<f64> = BDF::new(6, 0.001);
    let x_end = problem.t_end().unwrap();
    let (_x, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    assert_relative_eq!(x_end.tan(), y.last().unwrap()[0], epsilon = 0.001);
}
