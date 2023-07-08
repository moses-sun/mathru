use mathru::algebra::linear::matrix::Diagonal;

#[test]
fn matrix_owner() {
    let a: Diagonal<f64> = Diagonal::new(&[1.0, -1.0]);

    let b: Diagonal<f64> = Diagonal::new(&[2.0, 3.0]);

    let prod_ref: Diagonal<f64> = Diagonal::new(&[2.0, -3.0]);

    let res: Diagonal<f64> = a * b;

    assert_relative_eq!(prod_ref, res);
}

#[test]
fn matrix_borrow_mut() {
    let mut a: Diagonal<f64> = Diagonal::new(&[1.0, -1.0]);

    let b: Diagonal<f64> = Diagonal::new(&[2.0, 3.0]);

    let prod_ref: Diagonal<f64> = Diagonal::new(&[2.0, -3.0]);

    let _ = &mut a * &b;

    assert_relative_eq!(prod_ref, a);
}

#[test]
fn diagonal_mul_diagonal_borrow() {
    let a: Diagonal<f64> = Diagonal::new(&[1.0, -1.0]);

    let b: Diagonal<f64> = Diagonal::new(&[2.0, 3.0]);

    let prod_ref: Diagonal<f64> = Diagonal::new(&[2.0, -3.0]);

    assert_relative_eq!(prod_ref, &a * &b);
}

#[test]
fn diagonal_mul_scalar_borrow() {
    let a: Diagonal<f64> = Diagonal::new(&[1.0, -1.0]);

    let reference: Diagonal<f64> = Diagonal::new(&[-2.0, 2.0]);

    let res: Diagonal<f64> = &a * &-2.0;

    assert_relative_eq!(reference, res);
}

#[test]
fn diagonal_mul_scalar_borrow_mut() {
    let mut a: Diagonal<f64> = Diagonal::new(&[1.0, -1.0]);

    let reference: Diagonal<f64> = Diagonal::new(&[-2.0, 2.0]);

    let _ = &mut a * &-2.0;

    assert_relative_eq!(reference, a);
}

#[test]
fn diagonal_mul_scalar_own() {
    let a: Diagonal<f64> = Diagonal::new(&[1.0, -1.0]);

    let reference: Diagonal<f64> = Diagonal::new(&[-2.0, 2.0]);

    let res = a * -2.0;

    assert_relative_eq!(reference, res);
}
