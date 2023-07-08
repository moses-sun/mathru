use mathru::analysis::integral::gauss_kronrod::GaussKronrod;

#[test]
fn gauss_legendre_1() {
    let gl: GaussKronrod<f64> = GaussKronrod::new();
    let f = |x| x;

    let integral: f64 = gl.integrate(f, 2.0, 4.0);

    assert_relative_eq!(integral, 6.0, epsilon = 1.0e-1);
}
