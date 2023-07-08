use mathru::algebra::linear::matrix::{Diagonal, General};

#[test]
fn add_own() {
    let a: Diagonal<f64> = matrix![ 1.0, 0.0;
                                    0.0, 2.0]
    .into();

    let b: Diagonal<f64> = matrix![ -3.0, 0.0;
                                    0.0, 5.0]
    .into();

    let sum: Diagonal<f64> = matrix![-2.0, 0.0;
                                     0.0, 7.0]
    .into();

    assert_eq!(sum, a + b);
}

#[test]
fn add_borrow() {
    let a: Diagonal<f64> = matrix![ 1.0, 0.0;
                                    0.0, 2.0]
    .into();

    let b: Diagonal<f64> = matrix![ -3.0, 0.0;
                                    0.0, 5.0]
    .into();

    let sum: Diagonal<f64> = matrix![-2.0, 0.0;
                                     0.0, 7.0]
    .into();

    assert_eq!(sum, &a + &b);
}

#[test]
fn add_borrow_mut() {
    let mut a: Diagonal<f64> = matrix![ 1.0, 0.0;
                                        0.0, 2.0]
    .into();

    let b: Diagonal<f64> = matrix![ -3.0, 0.0;
                                    0.0, 5.0]
    .into();

    let sum: Diagonal<f64> = matrix![-2.0, 0.0;
                                     0.0, 7.0]
    .into();

    assert_eq!(&sum, &mut a + &b);
}

#[test]
fn add_own_2() {
    let a: Diagonal<f64> = matrix![ 1.0, 0.0;
                                    0.0, 2.0]
    .into();

    let sum: Diagonal<f64> = matrix![ 5.0, 0.0;
                                    0.0, 6.0]
    .into();

    assert_eq!(sum, a + 4.0);
}

#[test]
fn add_borrow_2() {
    let a: Diagonal<f64> = matrix![ 1.0, 0.0;
                                    0.0, 2.0]
    .into();

    let sum: Diagonal<f64> = matrix![ 5.0, 0.0;
                                    0.0, 6.0]
    .into();

    assert_eq!(sum, &a + &4.0);
}

#[test]
fn add_borrow_mut_2() {
    let mut a: Diagonal<f64> = matrix![ 1.0, 0.0;
                                        0.0, 2.0]
    .into();

    let sum: Diagonal<f64> = matrix![ 5.0, 0.0;
                                    0.0, 6.0]
    .into();

    assert_eq!(&sum, &mut a + &4.0);
}
