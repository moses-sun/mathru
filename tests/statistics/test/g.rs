#[cfg(test)]
mod gtest
{
    use mathru::statistics::test::{G};

    #[test]
    fn test0()
    {
        let x: Vec<f64> = vec![70.0, 79.0, 3.0, 4.0];
        let y: Vec<f64> = vec![0.54, 0.40, 0.05, 0.01];
        let test: G<f64> = G::test_vector(&x, &y);

        assert_eq!(13.1447992204914, test.g());
        assert_eq!(3, test.df());
        assert_eq!(0.004319866167957542, test.p_value())
    }

    #[test]
    fn test1()
    {
        let x: Vec<f64> = vec![1752.0, 1895.0];
        let y: Vec<f64> = vec![0.5, 0.5];
        let test: G<f64> = G::test_vector(&x, &y);

        assert_eq!(5.608511956526968, test.g());
        assert_eq!(1, test.df());
    }

}