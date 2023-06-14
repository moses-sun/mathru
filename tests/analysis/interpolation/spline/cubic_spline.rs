use mathru::algebra::linear::Vector;
use mathru::analysis::interpolation::spline::{CubicSpline, CubicSplineConstraint};

#[test]
fn solve_thomas() {
    let a = vector![1.0; 1.0; 1.0; 1.0];
    let b = vector![-2.0; -2.0; -2.0; -2.0; -2.0];
    let c = vector![1.0; 1.0; 1.0; 1.0];

    let y = vector![ -1.0; -1.0; -1.0; -1.0; -1.0];

    let x = CubicSpline::solve_thomas(&a, &b, c, y).unwrap();

    let x_ref = vector![2.5; 4.0; 4.5; 4.0; 2.5];

    assert_relative_eq!(x, x_ref, epsilon = 10e-14);
}

#[test]
fn interpolate_natural_2() {
    let x = vec![-2.0, 0.0];
    let y = vec![6.0, 2.0];

    let cubic_spline = CubicSpline::interpolate(&x, &y, CubicSplineConstraint::Natural);

    assert_eq!(6.0, cubic_spline.eval(-2.0));
    assert_eq!(4.0, cubic_spline.eval(-1.0));
    assert_eq!(2.0, cubic_spline.eval(0.0));
}

#[test]
fn interpolate_natural_3() {
    let x = vec![-2.0, 0.0, 2.0];
    let y = vec![6.0, 2.0, 6.0];

    let cubic_spline = CubicSpline::interpolate(&x, &y, CubicSplineConstraint::Natural);

    assert_eq!(6.0, cubic_spline.eval(-2.0));
    assert_eq!(3.25, cubic_spline.eval(-1.0));
    assert_eq!(2.0, cubic_spline.eval(0.0));
    assert_eq!(3.25, cubic_spline.eval(1.0));
    assert_eq!(6.0, cubic_spline.eval(2.0));
}

#[test]
fn interpolate_natural_4() {
    let x = vec![-2.0, 0.0, 1.0, 2.0];
    let y = vec![6.0, 2.0, 4.0, 5.0];

    let cubic_spline = CubicSpline::interpolate(&x, &y, CubicSplineConstraint::Natural);

    assert_relative_eq!(6.0, cubic_spline.eval(-2.0));
    assert_relative_eq!(2.891304347826087, cubic_spline.eval(-1.0));
    assert_relative_eq!(2.0, cubic_spline.eval(0.0));
    assert_relative_eq!(4.0, cubic_spline.eval(1.0));
    assert_relative_eq!(5.0, cubic_spline.eval(2.0));
}

#[test]
fn interpolate_natural_5() {
    let x = vec![-2.0, 0.0, 1.0, 2.0, 4.0];
    let y = vec![6.0, 2.0, 4.0, 5.0, -2.0];

    let cubic_spline = CubicSpline::interpolate(&x, &y, CubicSplineConstraint::Natural);

    assert_relative_eq!(6.0, cubic_spline.eval(-2.0));
    assert_relative_eq!(2.9375, cubic_spline.eval(-1.0));
    assert_relative_eq!(2.0, cubic_spline.eval(0.0));
    assert_relative_eq!(4.0, cubic_spline.eval(1.0));
    assert_relative_eq!(5.0, cubic_spline.eval(2.0), epsilon = 0.00000000000001);
    assert_relative_eq!(-2.0, cubic_spline.eval(4.0), epsilon = 0.00000000000001);
}

#[test]
fn interpolate_periodic_2() {
    let x = vec![-2.0, 0.0];
    let y = vec![6.0, 2.0];

    let periodic_spline = CubicSpline::interpolate(&x, &y, CubicSplineConstraint::Periodic);

    assert_relative_eq!(6.0, periodic_spline.eval(-2.0));
    assert_relative_eq!(2.0, periodic_spline.eval(0.0), epsilon = 0.00000000000001);
}

#[test]
fn interpolate_periodic_3() {
    let x = vec![-2.0, 0.0, 1.0];
    let y = vec![6.0, 2.0, 4.0];

    let periodic_spline = CubicSpline::interpolate(&x, &y, CubicSplineConstraint::Periodic);

    assert_relative_eq!(6.0, periodic_spline.eval(-2.0));
    assert_relative_eq!(4.0, periodic_spline.eval(-1.0));
    assert_relative_eq!(2.0, periodic_spline.eval(0.0), epsilon = 0.00000000001);
    assert_relative_eq!(3.0, periodic_spline.eval(0.5));
    assert_relative_eq!(4.0, periodic_spline.eval(1.0));
}

#[test]
fn interpolate_periodic_5() {
    let x = vec![-2.0, 0.0, 1.0, 2.0, 4.0];
    let y = vec![6.0, 2.0, 4.0, 5.0, -2.0];

    let periodic_spline = CubicSpline::interpolate(&x, &y, CubicSplineConstraint::Periodic);

    assert_relative_eq!(6.0, periodic_spline.eval(-2.0));
    assert_relative_eq!(2.0, periodic_spline.eval(0.0), epsilon = 0.00000000000001);
    assert_relative_eq!(
        2.34375,
        periodic_spline.eval(3.0),
        epsilon = 0.00000000000001
    );
    assert_relative_eq!(4.0, periodic_spline.eval(1.0));
    assert_relative_eq!(5.0, periodic_spline.eval(2.0), epsilon = 0.00000000000001);
    assert_relative_eq!(-2.0, periodic_spline.eval(4.0), epsilon = 0.00000000000001);
}
