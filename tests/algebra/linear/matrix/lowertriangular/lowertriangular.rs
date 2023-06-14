use mathru::algebra::linear::matrix::{General, LowerTriangular};

#[test]
fn from_general() {
    let expected: General<f64> = matrix![2.0, 0.0, 0.0;
                                        4.0, 3.0, 0.0;
                                        7.0, 8.0, 5.0];

    let actual: General<f64> = LowerTriangular::from(expected.clone()).into();

    assert_relative_eq!(actual, expected);
}
