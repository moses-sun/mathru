#[cfg(test)]
mod kutta3
{
    use mathru::algebra::linear::Vector;
    use mathru::analysis::differential_equation::ordinary::{ExplicitODE, Kutta3};

    use super::super::problem::{ExplicitODE1, ExplicitODE2};

    fn compare_epsilon(a: f64, b: f64, epsilon: f64) -> bool
    {
        if (a - b).abs() > epsilon
        {
            println!("|a-b|: {}", (a - b).abs());
            return false;
        }

        return true;
    }

    #[test]
    fn fn1()
    {
        let problem: ExplicitODE1 = ExplicitODE1::default();
        let solver: Kutta3<f64> = Kutta3::new(0.01);

        let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

        let len: usize = y.len();

        let time_span: (f64, f64) = problem.time_span();
        let init_cond: Vector<f64> = problem.init_cond();

        assert!(compare_epsilon(time_span.1, t[len - 1], 0.000000001));
        assert!(compare_epsilon(*init_cond.get(0) * (2.0 * time_span.1).exp(),
                                *y[len - 1].get(0),
                                0.00004));
    }

    #[test]
    fn fn2()
    {
        let problem: ExplicitODE2 = ExplicitODE2::default();
        let solver: Kutta3<f64> = Kutta3::new(0.1);

        let (t, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

        let len: usize = y.len();

        let time_span: (f64, f64) = problem.time_span();

        assert!(compare_epsilon(time_span.1, t[len - 1], 0.00000001));
        assert!(compare_epsilon(time_span.1.tan(), *y[len - 1].get(0), 0.04));
    }
}
