#[cfg(test)]
mod bdf
{
    use mathru::{
        algebra::linear::Vector,
        analysis::differential_equation::ordinary::{problem, ImplicitODE, BDF},
    };

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
        let problem: problem::Euler<f64> = problem::Euler::default();
        let solver: BDF<f64> = BDF::new(6, 0.1);

        let (x, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

        assert!(compare_epsilon(0.992199, *y.last().unwrap().get(0), 0.0001));
    }
}
