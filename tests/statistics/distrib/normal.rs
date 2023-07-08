use mathru::statistics::distrib::{Continuous, Distribution, Normal};
use std::f64::consts::{E, PI};

#[test]
fn pdf0() {
    let mean: f64 = 0.0;
    let variance: f64 = 1.0;
    let distrib: Normal<f64> = Normal::new(mean, variance);
    let x: f64 = 0.0;
    let prob: f64 = distrib.pdf(x);

    assert_relative_eq!(0.3989422804014327, prob);
}

#[test]
fn random() {
    let mu_1: f64 = 0.0;
    let variance_1: f64 = 1.0;
    let distrib_1: Normal<f64> = Normal::new(mu_1, variance_1);
    let mut data: Vec<f64> = Vec::new();

    for _i in 0..10000 {
        data.push(distrib_1.random());
    }

    let distrib_2: Normal<f64> = Normal::from_data(&data);

    assert_abs_diff_eq!(mu_1, distrib_2.mean(), epsilon = 0.1);
    assert_abs_diff_eq!(variance_1, distrib_2.variance(), epsilon = 0.1);
}

#[test]
fn cdf0() {
    let mean: f64 = 0.0;
    let variance: f64 = 1.0;
    let distrib: Normal<f64> = Normal::new(mean, variance);

    assert_relative_eq!(0.5, distrib.cdf(0.0))
}

#[test]
fn quantile0() {
    let mean: f64 = 0.0;
    let variance: f64 = 1.0;
    let distrib: Normal<f64> = Normal::new(mean, variance);

    assert_relative_eq!(1.2815515655446006, distrib.quantile(0.9));
}

#[test]
fn quantile1() {
    let mean: f64 = 1.0;
    let variance: f64 = 0.5;
    let distrib: Normal<f64> = Normal::new(mean, variance);

    assert_relative_eq!(1.0, distrib.quantile(0.5));
}

#[test]
fn quantile2() {
    let mean: f64 = 1.0;
    let variance: f64 = 0.25;
    let distrib: Normal<f64> = Normal::new(mean, variance);

    assert_relative_eq!(1.8224268134757358, distrib.quantile(0.95));
}

#[test]
fn quantile3() {
    let mean: f64 = 1.0;
    let variance: f64 = 0.25;
    let distrib: Normal<f64> = Normal::new(mean, variance);

    assert_relative_eq!(-3.1110411080652174, distrib.quantile(0.0000000000000001));
}

#[test]
fn mean() {
    let mean: f64 = 1.0;
    let variance: f64 = 0.5;
    let distrib: Normal<f64> = Normal::new(mean, variance);

    assert_relative_eq!(mean, distrib.mean());
}

#[test]
fn median() {
    let mean: f64 = 1.0;
    let variance: f64 = 0.5;
    let distrib: Normal<f64> = Normal::new(mean, variance);

    assert_relative_eq!(mean, distrib.median());
}

#[test]
fn variance() {
    let mean: f64 = 1.0;
    let variance: f64 = 0.5;
    let distrib: Normal<f64> = Normal::new(mean, variance);

    assert_relative_eq!(variance, distrib.variance());
}

#[test]
fn skewness() {
    let mean: f64 = 1.0;
    let variance: f64 = 0.5;
    let distrib: Normal<f64> = Normal::new(mean, variance);

    assert_relative_eq!(0.0, distrib.skewness());
}

#[test]
fn entropy() {
    let mean: f64 = 1.0;
    let variance: f64 = 0.5;
    let distrib: Normal<f64> = Normal::new(mean, variance);

    assert_relative_eq!(2.0 * PI * E * variance, distrib.entropy());
}
