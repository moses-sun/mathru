use crate::analysis::ode::problem::ExplicitOde1;
use mathru::{
    algebra::linear::vector::Vector,
    analysis::differential_equation::ordinary::{
        solver::explicit::runge_kutta::fixed::{FixedStepper, RungeKutta4},
        ExplicitInitialValueProblemBuilder,
    },
};

#[test]
fn t_end() {
    let ode = ExplicitOde1::default();

    let time_end = 1.0;
    let problem = ExplicitInitialValueProblemBuilder::new(&ode, 0.0, vector![0.5; 2.0])
        .t_end(time_end)
        .build();

    let solver = FixedStepper::new(0.2);
    let (t, y): (Vec<f64>, Vec<Vector<f64>>) =
        solver.solve(&problem, &RungeKutta4::default()).unwrap();

    let len: usize = y.len();
    let init_cond: Vector<f64> = problem.init_cond();

    assert_relative_eq!(time_end, t[len - 1], epsilon = 0.001);
    assert_relative_eq!(
        init_cond[0] * (2.0 * time_end).exp(),
        y[len - 1][0],
        epsilon = 0.01
    );

    assert_relative_eq!(
        init_cond[1] * (2.0 * time_end).exp(),
        y[len - 1][1],
        epsilon = 0.03
    );
}
