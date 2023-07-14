use mathru::analysis::integral::gauss_legendre::Adaptive;

#[test]
fn adaptive() {
    let f = |x: f64| -2.0 * x.powi(3) + x.powi(2) + 5.0;

    let adaptive: Adaptive<f64> = Adaptive::new(2, 0.001);
    let integral: f64 = adaptive.integrate(&f, -3.0, 4.0);

    assert_relative_eq!(integral, -22.167, epsilon = 1.0e-3);
}
