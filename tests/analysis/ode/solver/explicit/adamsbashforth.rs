use super::super::super::problem::{explicit_ode, explicit_ode1};
use mathru::{
    algebra::linear::vector::Vector,
    analysis::differential_equation::ordinary::solver::explicit::AdamsBashforth,
};

#[test]
fn fn1_1_step() {
    let problem = explicit_ode1();
    let solver: AdamsBashforth<f64> = AdamsBashforth::new(1, 0.001);

    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let len: usize = y.len();

    let time_end = problem.t_end().unwrap();
    let init_cond: Vector<f64> = problem.init_cond();

    assert_relative_eq!(time_end, t[len - 1], epsilon = 0.000000001);
    assert_relative_eq!(
        init_cond[0] * (2.0 * time_end).exp(),
        y[len - 1][0],
        epsilon = 0.2
    );
}

#[test]
fn fn1_2_steps() {
    let problem = explicit_ode1();
    let solver: AdamsBashforth<f64> = AdamsBashforth::new(2, 0.001);

    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let len: usize = y.len();

    let time_end = problem.t_end().unwrap();
    let init_cond: Vector<f64> = problem.init_cond();

    assert_relative_eq!(time_end, t[len - 1], epsilon = 0.000000001);
    assert_relative_eq!(
        init_cond[0] * (2.0 * time_end).exp(),
        y[len - 1][0],
        epsilon = 0.0003
    );
}

#[test]
fn fn1_3_steps() {
    let problem = explicit_ode1();
    let solver: AdamsBashforth<f64> = AdamsBashforth::new(3, 0.001);

    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let len: usize = y.len();

    let time_end = problem.t_end().unwrap();
    let init_cond: Vector<f64> = problem.init_cond();

    assert_relative_eq!(time_end, t[len - 1], epsilon = 0.000000001);
    assert_relative_eq!(
        init_cond[0] * (2.0 * time_end).exp(),
        y[len - 1][0],
        epsilon = 0.00006
    );
}

#[test]
fn fn1_4_steps() {
    let problem = explicit_ode1();
    let solver: AdamsBashforth<f64> = AdamsBashforth::new(4, 0.001);

    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let len: usize = y.len();

    let time_end = problem.t_end().unwrap();
    let init_cond: Vector<f64> = problem.init_cond();

    assert_relative_eq!(time_end, t[len - 1], epsilon = 0.000000001);
    assert_relative_eq!(
        init_cond[0] * (2.0 * time_end).exp(),
        y[len - 1][0],
        epsilon = 0.000055
    );
}

#[test]
fn fn1_5_steps() {
    let problem = explicit_ode1();
    let solver: AdamsBashforth<f64> = AdamsBashforth::new(5, 0.001);

    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let len: usize = y.len();

    let time_end = problem.t_end().unwrap();
    let init_cond: Vector<f64> = problem.init_cond();

    assert_relative_eq!(time_end, t[len - 1], epsilon = 0.000000001);
    assert_relative_eq!(
        init_cond[0] * (2.0 * time_end).exp(),
        y[len - 1][0],
        epsilon = 0.000055
    );
}

#[test]
fn fn2_1_step() {
    let problem = explicit_ode();
    let solver: AdamsBashforth<f64> = AdamsBashforth::new(1, 0.001);

    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let len: usize = y.len();

    let time_end = problem.t_end().unwrap();

    assert_relative_eq!(time_end, t[len - 1], epsilon = 0.00000001);
    assert_relative_eq!(time_end.tan(), y[len - 1][0], epsilon = 0.07);
}
