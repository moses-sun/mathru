extern crate mathru;

#[cfg(test)]
mod chisquaredtest_test
{
    use mathru::stats::test::{Test, ChiSquared};

    #[test]
    fn test0()
    {
        let x: Vec<f64> = vec![10.0, 20.0, 30.0];
        let y: Vec<f64> = vec![6.0, 9.0, 17.0];
        let test: ChiSquared = ChiSquared::test_vector(&x, &y);

        assert_eq!(0.27157465150403504, test.value());
        assert_eq!(2, test.df());
        assert_eq!(0.8730282833800732, test.p_value());
    }

}