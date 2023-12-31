use mathru::statistics::distrib::{Continuous, Exponential};

#[test]
fn pdf0() {
    let lambda: f64 = 1.0;
    let distrib: Exponential<f64> = Exponential::new(lambda);
    let x: f64 = 1.0;
    let prob: f64 = distrib.pdf(x);

    assert_relative_eq!((-1.0_f64).exp(), prob);
}

#[test]
fn cdf0() {
    let lambda: f64 = 0.5;
    let distrib: Exponential<f64> = Exponential::new(lambda);

    assert_relative_eq!(1.0 - (-1.0_f64).exp(), distrib.cdf(2.0))
}

#[test]
fn quantile() {
    let lambda: f64 = 0.5;
    let distrib: Exponential<f64> = Exponential::new(lambda);

    assert_relative_eq!(1.3862943611198906, distrib.quantile(0.5), epsilon = 1.0e-10)
}

#[test]
fn skewnes() {
    let lambda: f64 = 0.5;
    let distrib: Exponential<f64> = Exponential::new(lambda);

    assert_relative_eq!(2.0, distrib.skewness(), epsilon = 1.0e-10);
}

#[test]
fn median() {
    let lambda: f64 = 0.5;
    let distrib: Exponential<f64> = Exponential::new(lambda);

    assert_relative_eq!((2.0f64).ln() / lambda, distrib.median(), epsilon = 1.0e-10);
}

#[test]
fn entropy() {
    let lambda: f64 = 0.5;
    let distrib: Exponential<f64> = Exponential::new(lambda);

    assert_relative_eq!(1.0 - lambda.ln(), distrib.entropy(), epsilon = 1.0e-10);
}

#[test]
fn mean() {
    let lambda: f64 = 0.5;
    let distrib: Exponential<f64> = Exponential::new(lambda);
    let mean: f64 = distrib.mean();

    assert_eq!(1.0 / lambda, mean);
}

#[test]
fn variance() {
    let lambda: f64 = 0.5;
    let distrib: Exponential<f64> = Exponential::new(lambda);
    let variance: f64 = distrib.variance();

    assert_eq!(1.0 / (lambda * lambda), variance);
}

#[test]
fn random() {
    let lambda: f64 = 0.5;
    let distrib_ref: Exponential<f64> = Exponential::new(lambda);
    let mut data: Vec<f64> = Vec::new();

    for _i in 0..10000 {
        data.push(distrib_ref.random());
    }

    let distrib: Exponential<f64> = Exponential::from_data(&data);

    assert_abs_diff_eq!(1.0 / lambda, distrib.mean(), epsilon = 0.1);
}
