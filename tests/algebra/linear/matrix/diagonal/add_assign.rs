use mathru::algebra::linear::matrix::Diagonal;
use mathru::algebra::linear::matrix::General;

#[test]
fn diagonal() {
    let mut a: Diagonal<f32> = matrix![1.0, 0.0;
                                       0.0, -1.0]
    .into();

    let b: Diagonal<f32> = matrix![6.0, 0.0;
                                   0.0, -4.0]
    .into();

    let sum_ref: Diagonal<f32> = matrix![7.0, 0.0;
                                         0.0, -5.0]
    .into();

    a += b;

    assert_relative_eq!(sum_ref, a);
}

#[test]
fn scalar() {
    let mut a: Diagonal<f32> = matrix![1.0, 0.0;
                                       0.0, -1.0]
    .into();

    let sum: Diagonal<f32> = matrix![6.0, 0.0;
                                     0.0, 4.0]
    .into();
    a += 5.0f32;

    assert_relative_eq!(sum, a);
}
