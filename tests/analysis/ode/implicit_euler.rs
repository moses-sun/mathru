#[cfg(test)]
mod implicit_euler
{
    use mathru::{
        algebra::linear::Vector,
        analysis::differential_equation::ordinary::{problem, ImplicitEuler},
    };

    fn compare_epsilon(a: f64, b: f64, epsilon: f64) -> bool
    {
        if (a - b).abs() > epsilon
        {
            println!("a: {}, b:{} |a-b|: {}", a, b, (a - b).abs());
            return false;
        }

        return true;
    }

    #[test]
    fn fn1()
    {
        // Create an ODE instance
        let problem: problem::Euler<f64> = problem::Euler::default();
        let solver: ImplicitEuler<f64> = ImplicitEuler::new(0.0001);

        let (_x, y): (Vec<f64>, Vec<Vector<f64>>) = solver.solve(&problem).unwrap();

        assert!(compare_epsilon(0.9852, *y.last().unwrap().get(0), 0.0001));
    }
}
