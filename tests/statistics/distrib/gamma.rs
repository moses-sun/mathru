use mathru::{
    special::gamma::{digamma, Gamma as g},
    statistics::distrib::{Continuous, Gamma},
};
use std::f64;

#[test]
#[should_panic]
fn pdf_lower_0() {
    let p: f64 = 2.0;
    let b: f64 = 5.0;
    let x: f64 = 0.0 - f64::EPSILON;

    let distrib: Gamma<f64> = Gamma::new(p, b);

    distrib.pdf(x);
}

#[test]
#[should_panic]
fn pdf0() {
    let p: f64 = 2.0;
    let b: f64 = 2.0;
    let x: f64 = 0.0;

    let distrib: Gamma<f64> = Gamma::new(p, b);

    distrib.pdf(x);
}

#[test]
fn pdf1() {
    let p: f64 = 2.0;
    let b: f64 = 2.0;
    let x: f64 = 1.0;

    let distrib: Gamma<f64> = Gamma::new(p, b);

    distrib.pdf(x);
}

#[test]
fn pdf2() {
    let p: f64 = 2.0;
    let b: f64 = 2.0;
    let x: f64 = 0.4;

    let distrib: Gamma<f64> = Gamma::new(p, b);

    assert_eq!(0.7189263425875543, distrib.pdf(x));
}

#[test]
#[should_panic]
fn cdf_lower_0() {
    let p: f64 = 2.0;
    let q: f64 = 5.0;
    let x: f64 = 0.0 - f64::EPSILON;

    let distrib: Gamma<f64> = Gamma::new(p, q);

    distrib.cdf(x);
}

#[test]
fn cdf0() {
    let p: f64 = 2.0;
    let q: f64 = 5.0;
    let x: f64 = 0.0;

    let distrib: Gamma<f64> = Gamma::new(p, q);

    assert_eq!(0.0_f64, distrib.cdf(x));
}

#[test]
fn cdf1() {
    let p: f64 = 2.0;
    let q: f64 = 5.0;
    let x: f64 = 1.0;

    let distrib: Gamma<f64> = Gamma::new(p, q);

    assert_eq!(0.9595723180054873_f64, distrib.cdf(x));
}

#[test]
fn cdf2() {
    let p: f64 = 2.0_f64;
    let q: f64 = 5.0_f64;
    let x: f64 = 0.3_f64;

    let distrib: Gamma<f64> = Gamma::new(p, q);

    assert_eq!(0.44217459962892536_f64, distrib.cdf(x));
}

#[test]
fn skewness() {
    let alpha: f64 = 2.0_f64;
    let beta: f64 = 5.0_f64;

    let distrib: Gamma<f64> = Gamma::new(alpha, beta);

    assert_eq!(distrib.skewness(), 2.0 / alpha.sqrt());
}

#[test]
#[should_panic]
fn quantile() {
    let alpha: f64 = 2.0_f64;
    let beta: f64 = 5.0_f64;

    let distrib: Gamma<f64> = Gamma::new(alpha, beta);
    let _ = distrib.quantile(0.1);
}

#[test]
fn mean() {
    let alpha: f64 = 2.0_f64;
    let beta: f64 = 5.0_f64;

    let distrib: Gamma<f64> = Gamma::new(alpha, beta);

    let mean: f64 = distrib.mean();

    assert_eq!(alpha / beta, mean);
}

#[test]
fn variance() {
    let alpha: f64 = 2.0_f64;
    let beta: f64 = 5.0_f64;

    let distrib: Gamma<f64> = Gamma::new(alpha, beta);
    let variance: f64 = distrib.variance();

    assert_eq!(alpha / (beta * beta), variance);
}

#[test]
#[should_panic]
fn median() {
    let alpha: f64 = 2.0_f64;
    let beta: f64 = 5.0_f64;

    let distrib: Gamma<f64> = Gamma::new(alpha, beta);
    let _ = distrib.median();
}

#[test]
fn entropy() {
    let alpha: f64 = 2.0_f64;
    let beta: f64 = 5.0_f64;

    let distrib: Gamma<f64> = Gamma::new(alpha, beta);
    let entropy: f64 = distrib.entropy();
    assert_eq!(
        alpha - beta.ln() + alpha.gamma().ln() + (1.0 - alpha) * digamma(alpha),
        entropy
    );
}
