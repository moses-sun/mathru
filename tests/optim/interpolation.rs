extern crate mathru;

#[cfg(test)]
mod interpolation
{
    use mathru::analysis::interpolation::polynom;
    use mathru::analysis::interpolation::polynom::Linear;
    use mathru::analysis::interpolation::Interpolation;

    fn compare_vec(a: &Vec<f64>, b: &Vec<f64>, epsilon: f64) -> bool
    {
        for (e_a, e_b) in a.iter().zip(b.iter())
        {
            if (e_a - e_b).abs() > epsilon
            {
                return false;
            }
        }

        return true;
    }

    #[test]
    fn linear_0()
    {
        let mut x: Vec<f64> = vec![-3.0, 2.0, 14.0];
        let y: Vec<f64> = vec![-2.0, 5.0, 7.0];
        let sample: Vec<(f64, f64)> = vec![(-3.0, -2.0), (2.0, 5.0), (14.0, 7.0)];
        let f_hat: Linear = polynom::Linear::interpolate(&sample);

        let y_hat: Vec<f64> = x.iter().map(|x_i: &f64| f_hat.eval(*x_i)).collect();

        assert!(compare_vec(&y_hat, &y, 0.00000001));
    }

    #[test]
    fn linear_1()
    {
        let mut x: Vec<f64> = vec![-3.0, 2.0, 14.0];
        let y: Vec<f64> = vec![-2.0, 5.0, 7.0];
        let sample: Vec<(f64, f64)> = vec![(-3.0, -2.0), (2.0, 5.0), (14.0, 7.0)];
        let f_hat: Linear = polynom::Linear::interpolate(&sample);

        let x_test: Vec<f64> = vec![-3.5, 2.5, 14.5];

        let y_ref: Vec<f64> = vec![-2.7, 5.0833333333, 7.083333333];
        let y_hat: Vec<f64> = x_test.iter().map(|x_i: &f64| f_hat.eval(*x_i)).collect();

        assert!(compare_vec(&y_hat, &y_ref, 0.00000001));
    }
}