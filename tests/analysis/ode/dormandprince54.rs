use super::problem::{ExplicitODE1, ExplicitODE2};
use mathru::{
    algebra::linear::Vector,
    analysis::differential_equation::ordinary::{DormandPrince54, ExplicitODE},
};

#[test]
fn fn1()
{
    let h_0: f64 = 0.1;
    let n_max: u32 = 500;
    let abs_tol: f64 = 0.0000001;

    let solver: DormandPrince54<f64> = DormandPrince54::new(abs_tol, h_0, n_max);
    let problem: ExplicitODE1 = ExplicitODE1::default();

    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let time_span: (f64, f64) = problem.time_span();
    let init_cond: Vector<f64> = problem.init_cond();

    let len: usize = y.len();

    assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.00001);
    assert_relative_eq!(*init_cond.get(0) * (2.0 * time_span.1).exp(), *y[len - 1].get(0), epsilon=0.0002);
}

#[test]
fn fn2()
{
    let problem: ExplicitODE2 = ExplicitODE2::default();

    let h_0: f64 = 0.001;
    let n_max: u32 = 300;
    let abs_tol: f64 = 0.00000001;

    let solver: DormandPrince54<f64> = DormandPrince54::new(abs_tol, h_0, n_max);

    let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

    let len: usize = y.len();

    let time_span: (f64, f64) = problem.time_span();

    assert_relative_eq!(time_span.1, t[len - 1], epsilon=0.0001);
    assert_relative_eq!(time_span.1.tan(), *y[len - 1].get(0), epsilon=0.0007);
}
