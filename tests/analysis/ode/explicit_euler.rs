use super::problem::{ExplicitODE1, ExplicitODE2};
use mathru::{
    algebra::linear::Vector,
    analysis::differential_equation::ordinary::{ExplicitEuler, FixedStepper, ExplicitODE},
};

#[test]
fn fn1()
{
    let problem: ExplicitODE1 = ExplicitODE1::default();
    let solver: FixedStepper<f64> = FixedStepper::new(0.00001);
    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem, &mut ExplicitEuler::default()).unwrap();

    let len: usize = y.len();

    let time_span: (f64, f64) = problem.time_span();
    let init_cond: Vector<f64> = problem.init_cond();

    assert_relative_eq!(time_span.0, t[0], epsilon=0.000000001);
    assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.000000001);
    assert_relative_eq!(init_cond, y[0], epsilon=0.002);
    assert_relative_eq!(init_cond[0] * (2.0 * time_span.1).exp(), y[len - 1][0], epsilon=0.002);
}

#[test]
fn fn2()
{
    let problem: ExplicitODE2 = ExplicitODE2::default();
    let solver: FixedStepper<f64> = FixedStepper::new(0.00001);
    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem, &mut ExplicitEuler::default()).unwrap();

    let len: usize = y.len();

    let time_span: (f64, f64) = problem.time_span();

    assert_relative_eq!(time_span.0, t[0], epsilon=0.000000001);
    assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.00000001);
    assert_relative_eq!(time_span.1.tan(), y[len - 1][0], epsilon=0.02);
}
