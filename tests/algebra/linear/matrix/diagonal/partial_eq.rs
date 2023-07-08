use mathru::algebra::linear::matrix::Diagonal;
use mathru::algebra::linear::matrix::General;

#[test]
#[should_panic]
fn partial_eq_non_matching_content() {
    let a: Diagonal<f64> = matrix![1.0, 0.0;
                                   0.0, 2.0]
    .into();
    let b: Diagonal<f64> = matrix![2.0, 0.0;
                                   0.0, 2.0]
    .into();

    assert_eq!(a, b);
}

#[test]
#[should_panic]
fn partial_eq_non_matching_dimensions() {
    let a: Diagonal<f64> = matrix![1.0, 2.0].into();
    let b: Diagonal<f64> = matrix![-1.0].into();

    assert_eq!(a, b);
}

#[test]
fn equal() {
    let a: Diagonal<f64> = matrix![1.0, 0.0;
                                   0.0, 2.0]
    .into();
    let b: Diagonal<f64> = matrix![1.0, 0.0;
                                   0.0, 2.0]
    .into();

    assert_eq!(a, b);
}
