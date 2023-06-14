use mathru::algebra::linear::matrix::{General, UnitLowerTriangular};

#[test]
fn from_general() {
    let expected: General<f64> = matrix![1.0, 0.0, 0.0;
                                        4.0, 1.0, 0.0;
                                        7.0, 8.0, 1.0];

    let actual: General<f64> = UnitLowerTriangular::from(expected.clone()).into();

    assert_relative_eq!(actual, expected);
}
