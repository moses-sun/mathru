use mathru::algebra::linear::matrix::General;
use mathru::algebra::linear::matrix::LowerTriangular;
use mathru::algebra::linear::vector::Vector;

#[test]
fn mul_owner() {
    let a: LowerTriangular<f64> = matrix![1.0, 0.0;
                                          3.0, 4.0]
    .into();

    let b: LowerTriangular<f64> = matrix![5.0, 0.0;
                                          6.0, 9.0]
    .into();

    let prod_ref: LowerTriangular<f64> = matrix![5.0, 0.0;
                                                 39.0, 36.0]
    .into();

    let res: LowerTriangular<f64> = a * b;

    assert_relative_eq!(prod_ref, res);
}

#[test]
fn mul_borrow() {
    let a: LowerTriangular<f64> = matrix![1.0, 0.0;
                                          3.0, 4.0]
    .into();

    let b: LowerTriangular<f64> = matrix![5.0, 0.0;
                                          6.0, 9.0]
    .into();

    let prod_ref: LowerTriangular<f64> = matrix![5.0, 0.0;
                                                39.0, 36.0]
    .into();

    let res: LowerTriangular<f64> = &a * &b;

    assert_relative_eq!(prod_ref, res);
}

#[test]
fn mul_borrow_mut() {
    let mut a: LowerTriangular<f64> = matrix![1.0, 0.0;
                                              3.0, 4.0]
    .into();

    let b: LowerTriangular<f64> = matrix![5.0, 0.0;
                                          6.0, 9.0]
    .into();

    let prod_ref: LowerTriangular<f64> = matrix![5.0, 0.0;
                                                39.0, 36.0]
    .into();

    let _ = &mut a * &b;

    assert_relative_eq!(prod_ref, a);
}

#[test]
fn mul_vector_borrow() {
    let m: LowerTriangular<f32> = matrix![   1.0, 2.0;
                                    3.0, 4.0]
    .into();

    let v: Vector<f32> = vector![   2.0;
                                    4.0];

    let prod_ref: Vector<f32> = vector![    10.0;
                                            22.0];

    assert_relative_eq!(prod_ref, &m * &v);
}
