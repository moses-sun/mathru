use super::problem::{ExplicitODE1, ExplicitODE2};
use mathru::{
    algebra::linear::Vector,
    analysis::differential_equation::ordinary::{Tsitouras, ProportionalControl, ExplicitODE},
};

#[test]
fn fn1()
{
    let h_0: f64 = 0.1;
    let fac: f64 = 0.9;
    let fac_min: f64 = 0.01;
    let fac_max: f64 = 2.0;
    let n_max: u32 = 500;
    let abs_tol: f64 = 10e-6;
    let rel_tol: f64 = 10e-6;

    let problem: ExplicitODE1 = ExplicitODE1::default();
    let solver: ProportionalControl<f64> = ProportionalControl::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);
    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem, &Tsitouras::default()).unwrap();


    let time_span: (f64, f64) = problem.time_span();
    let init_cond: Vector<f64> = problem.init_cond();

    let len: usize = y.len();

    assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.00001);
    assert_relative_eq!(init_cond[0] * (2.0 * time_span.1).exp(), y[len - 1][0], epsilon=0.0002);
}

#[test]
fn fn2()
{
    let h_0: f64 = 0.001;
    let fac: f64 = 0.9;
    let fac_min: f64 = 0.01;
    let fac_max: f64 = 2.0;
    let n_max: u32 = 500;
    let abs_tol: f64 = 10e-7;
    let rel_tol: f64 = 10e-7;

    let problem: ExplicitODE2 = ExplicitODE2::default();
    let solver: ProportionalControl<f64> = ProportionalControl::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);
    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem, &Tsitouras::default()).unwrap();

    let len: usize = y.len();

    let time_span: (f64, f64) = problem.time_span();

    assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.0001);
    assert_relative_eq!(time_span.1.tan(), y[len - 1][0], epsilon=0.0007);
}
