use crate::analysis::ode::problem::ExplicitOde1;
use mathru::{
    algebra::linear::Vector,
    analysis::differential_equation::ordinary::{
        solver::explicit::runge_kutta::adaptive::{
            Fehlberg21, ProportionalControl, ProportionalControlBuilder,
        },
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

    let h_0: f64 = 0.2;
    let fac: f64 = 0.9;
    let fac_min: f64 = 0.01;
    let fac_max: f64 = 1.4;
    let n_max: u32 = 30;
    let abs_tol: f64 = 10e-6;
    let rel_tol: f64 = 10e-6;

    let solver: ProportionalControl<f64> =
        ProportionalControl::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);
    let (t, y): (Vec<f64>, Vec<Vector<f64>>) =
        solver.solve(&problem, &Fehlberg21::default()).unwrap();

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

#[test]
fn test_callback() {
    let ode = ExplicitOde1::default();

    let time_end = 1.0;
    let time_early_end = 0.6;
    let problem = ExplicitInitialValueProblemBuilder::new(&ode, 0.0, vector![0.5; 2.0])
        .t_end(time_end)
        .callback(&|t: &f64, _x: &Vector<f64>| if t > &0.6 { false } else { true })
        .build();

    let h_0: f64 = 0.2;
    let fac: f64 = 0.9;
    let fac_min: f64 = 0.01;
    let fac_max: f64 = 1.4;
    let n_max: u32 = 30;
    let abs_tol: f64 = 10e-6;
    let rel_tol: f64 = 10e-6;

    let solver: ProportionalControl<f64> =
        ProportionalControl::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);
    let (t, y): (Vec<f64>, Vec<Vector<f64>>) =
        solver.solve(&problem, &Fehlberg21::default()).unwrap();

    let len: usize = y.len();
    let init_cond: Vector<f64> = problem.init_cond();

    assert_relative_eq!(time_early_end, t[len - 1], epsilon = 0.02);
    assert_relative_eq!(
        init_cond[0] * (2.0 * t[len - 1]).exp(),
        y[len - 1][0],
        epsilon = 0.01
    );
    assert_relative_eq!(
        init_cond[1] * (2.0 * t[len - 1]).exp(),
        y[len - 1][1],
        epsilon = 0.03
    );
}

#[test]
fn initial_step_size() {
    let ode = ExplicitOde1::default();

    let time_end = 1.0;
    let problem = ExplicitInitialValueProblemBuilder::new(&ode, 0.0, vector![0.5; 2.0])
        .t_end(time_end)
        .build();

    let fac: f64 = 0.9;
    let fac_min: f64 = 0.01;
    let fac_max: f64 = 1.4;
    let n_max: u32 = 100;
    let abs_tol: f64 = 10e-6;
    let rel_tol: f64 = 10e-6;

    let solver: ProportionalControl<f64> =
        ProportionalControlBuilder::new(n_max, fac, fac_min, fac_max, abs_tol, rel_tol).build();
    let (t, y): (Vec<f64>, Vec<Vector<f64>>) =
        solver.solve(&problem, &Fehlberg21::default()).unwrap();

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
