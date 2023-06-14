use mathru::algebra::linear::matrix::{General, UpperTriangular};

#[test]
fn from_general() {
    let expected: General<f64> = matrix![1.0, 2.0, 3.0;
                                        0.0, 4.0, 5.0;
                                        0.0, 0.0, 6.0];

    let actual: General<f64> = UpperTriangular::from(expected.clone()).into();

    assert_relative_eq!(actual, expected);
}
