use mathru::statistics::test::{Test, T};

// #[test]
// fn test_independence_identical_means() {
//     let mean: f64 = 5.0;
//     let variance: f64 = 10.0;
//     let sample_size: u32 = 500;
//     let rv1: Vec<f64> = Normal::new(mean, variance).random_sequence(sample_size);
//     let rv2: Vec<f64> = Normal::new(mean, variance).random_sequence(sample_size);

//     let measure1: T<f64> = T::test_independence_equal_variance(&rv1, &rv2);
//     let measure2: T<f64> = T::test_independence_unequal_variance(&rv1, &rv2);

//     println!("{}", measure1.value());

//     assert!((measure1.value().abs() - 0.268).abs() < 0.001);
//     assert!((measure2.value().abs() - 0.268).abs() < 0.001);
// }

// #[test]
// fn test_independence_equal_mean_unequal_variance() {
//     let mean: f64 = 5.0;
//     let variance1: f64 = 10.0;
//     let variance2: f64 = 20.0;
//     let sample_size: u32 = 500;
//     let rv1: Vec<f64> = Normal::new(mean, variance1).random_sequence(sample_size);
//     let rv2: Vec<f64> = Normal::new(mean, variance2).random_sequence(sample_size);

//     let measure1: T<f64> = T::test_independence_equal_variance(&rv1, &rv2);
//     let measure2: T<f64> = T::test_independence_unequal_variance(&rv1, &rv2);

//     assert!((measure1.value().abs() - 0.465).abs() < 0.001);
//     assert!((measure2.value().abs() - 0.465).abs() < 0.001);
// }

// #[test]
// fn test_independence_unequal_sample_sizes() {
//     let mean: f64 = 5.0;
//     let variance: f64 = 20.0;
//     let sample_size1: u32 = 500;
//     let sample_size2: u32 = 100;
//     let distrib1 = Normal::new(mean, variance);
//     let distrib2 = Normal::new(mean, variance);
//     let rv1: Vec<f64> = distrib1.random_sequence(sample_size1);
//     let rv2: Vec<f64> = distrib2.random_sequence(sample_size2);

//     let measure1: T<f64> = T::test_independence_equal_variance(&rv1, &rv2);
//     let measure2: T<f64> = T::test_independence_unequal_variance(&rv1, &rv2);

//     assert!((measure1.value().abs() - 0.9988).abs() < 0.001);
//     assert!((measure2.value().abs() - 0.6971).abs() < 0.001);
// }

#[test]
fn test_independence_equal_variance() {
    let rv1: Vec<f64> = vec![1.0, 4.0, 5.0, 2.0, 1.0];
    let rv2: Vec<f64> = vec![1.0, 4.0, 5.0, 3.0, 4.0];

    let t_measure: T<f64> = T::test_independence_equal_variance(&rv1, &rv2);

    assert_relative_eq!(t_measure.value().abs(), 0.7559, epsilon = 0.0001);
}

#[test]
fn test_independence_unequal_variance() {
    let rv1: Vec<f64> = vec![1.0, 4.0, 5.0, 2.0, 1.0];
    let rv2: Vec<f64> = vec![1.0, 4.0, 5.0, 3.0, 4.0];

    let measure: T<f64> = T::test_independence_unequal_variance(&rv1, &rv2);

    assert_relative_eq!(measure.value().abs(), 0.756, epsilon = 0.001);
    assert_relative_eq!(measure.p_value(), 0.472, epsilon = 0.001);
}
