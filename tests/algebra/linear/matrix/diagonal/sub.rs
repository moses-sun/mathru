use mathru::algebra::linear::matrix::{Diagonal, General};

#[test]
fn sub_own() {
    let a: Diagonal<f64> = matrix![ 1.0, 0.0;
                                    0.0, 2.0]
    .into();

    let b: Diagonal<f64> = matrix![ -3.0, 0.0;
                                    0.0, 5.0]
    .into();

    let sum: Diagonal<f64> = matrix![4.0, 0.0;
                                     0.0, -3.0]
    .into();

    assert_eq!(sum, a - b);
}

#[test]
fn sub_borrow() {
    let a: Diagonal<f64> = matrix![ 1.0, 0.0;
                                    0.0, 2.0]
    .into();

    let b: Diagonal<f64> = matrix![ -3.0, 0.0;
                                    0.0, 5.0]
    .into();

    let sum: Diagonal<f64> = matrix![4.0, 0.0;
                                     0.0, -3.0]
    .into();

    assert_eq!(sum, &a - &b);
}

#[test]
fn sub_borrow_mut() {
    let mut a: Diagonal<f64> = matrix![ 1.0, 0.0;
                                        0.0, 2.0]
    .into();

    let b: Diagonal<f64> = matrix![ -3.0, 0.0;
                                    0.0, 5.0]
    .into();

    let sum: Diagonal<f64> = matrix![4.0, 0.0;
                                     0.0, -3.0]
    .into();

    assert_eq!(&sum, &mut a - &b);
}

#[test]
fn sub_own_2() {
    let a: Diagonal<f64> = matrix![ 1.0, 0.0;
                                        0.0, 2.0]
    .into();

    let diff: Diagonal<f64> = matrix![ -3.0, 0.0;
                                    0.0, -2.0]
    .into();

    assert_eq!(diff, a - 4.0);
}

#[test]
fn sub_borrow_2() {
    let a: Diagonal<f64> = matrix![1.0, 0.0;
                                   0.0, 2.0]
    .into();

    let diff: Diagonal<f64> = matrix![-3.0, 0.0;
                                      0.0, -2.0]
    .into();

    assert_eq!(diff, &a - &4.0);
}

#[test]
fn sub_borrow_mut_2() {
    print!("sub borrow mut 2");
    let mut a: Diagonal<f64> = matrix![1.0, 0.0;
                                       0.0, 2.0]
    .into();

    let diff: Diagonal<f64> = matrix![-3.0, 0.0;
                                      0.0, -2.0]
    .into();

    assert_eq!(&diff, &mut a - &4.0);
}
