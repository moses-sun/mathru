use mathru::algebra::linear::matrix::General;
use mathru::algebra::linear::matrix::UnitUpperTriangular;
use mathru::algebra::linear::vector::Vector;

#[test]
fn mul_owner() {
    let a: UnitUpperTriangular<f64> = matrix![1.0, 3.0;
                                              0.0, 1.0]
    .into();

    let b: UnitUpperTriangular<f64> = matrix![1.0, 6.0;
                                              0.0, 1.0]
    .into();

    let prod_ref: UnitUpperTriangular<f64> = matrix![1.0, 9.0;
                                                     0.0, 1.0]
    .into();

    let res: UnitUpperTriangular<f64> = a * b;

    assert_relative_eq!(prod_ref, res);
}

#[test]
fn mul_borrow() {
    let a: UnitUpperTriangular<f64> = matrix![1.0, 3.0;
                                              0.0, 1.0]
    .into();

    let b: UnitUpperTriangular<f64> = matrix![1.0, 6.0;
                                              0.0, 1.0]
    .into();

    let prod_ref: UnitUpperTriangular<f64> = matrix![1.0, 9.0;
                                                     0.0, 1.0]
    .into();

    let res: UnitUpperTriangular<f64> = &a * &b;

    assert_relative_eq!(prod_ref, res);
}

#[test]
fn mul_borrow_mut() {
    let mut a: UnitUpperTriangular<f64> = matrix![1.0, 3.0;
                                                  0.0, 1.0]
    .into();

    let b: UnitUpperTriangular<f64> = matrix![1.0, 6.0;
                                              0.0, 1.0]
    .into();

    let prod_ref: UnitUpperTriangular<f64> = matrix![1.0, 9.0;
                                                     0.0, 1.0]
    .into();

    let _ = &mut a * &b;

    assert_relative_eq!(prod_ref, a);
}

#[test]
fn mul_vector_borrow() {
    let m: UnitUpperTriangular<f32> = matrix![1.0, 2.0;
                                              3.0, 4.0]
    .into();

    let v: Vector<f32> = vector![2.0;
                                 4.0];

    let prod_ref: Vector<f32> = vector![10.0;
                                        22.0];

    assert_relative_eq!(prod_ref, &m * &v);
}
