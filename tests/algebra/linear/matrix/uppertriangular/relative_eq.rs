use mathru::algebra::linear::matrix::{General, UpperTriangular};

#[test]
#[should_panic]
fn non_matching_content() {
    let a: UpperTriangular<f64> = matrix![1.0, 2.0;
                                          0.0, 2.0]
    .into();
    let b: UpperTriangular<f64> = matrix![2.0, 2.0;
                                          0.0, 2.0]
    .into();

    assert_relative_eq!(a, b);
}

#[test]
#[should_panic]
fn non_matching_dimensions() {
    let a: UpperTriangular<f64> = matrix![1.0, 2.0;
                                          0.0, 2.0]
    .into();

    let b: UpperTriangular<f64> = matrix![-1.0].into();

    assert_relative_eq!(a, b);
}
