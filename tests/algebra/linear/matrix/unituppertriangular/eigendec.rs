use mathru::algebra::linear::matrix::{
    Diagonal, EigenDec, EigenDecomposition, General, Transpose, UnitUpperTriangular,
};
use mathru::matrix;

#[test]
fn dec_eigen() {
    let a: UnitUpperTriangular<f64> = matrix![  1.0, 0.0, 0.0; 
                                                3.0, 1.0, 0.0;
                                                2.0, -6.0, 1.0]
    .transpose()
    .into();

    let eigen: EigenDec<f64> = a.dec_eigen().unwrap();

    let (values, vectors) = eigen.pair();

    let eigen_values_ref = Diagonal::new(&vec![1.0, 1.0, 1.0]);
    assert_relative_eq!(eigen_values_ref, values);

    assert_relative_eq!(
        &Into::<General<f64>>::into(a) * &vectors,
        &vectors * &Into::<General<f64>>::into(values),
        epsilon = 1.0e-5
    );
}

#[test]
fn dec_eigen_2() {
    let a: UnitUpperTriangular<f64> = matrix![  1.0, 0.0, 0.0; 
                                                3.0, 1.0, 0.0;
                                                0.0, -6.0, 1.0]
    .transpose()
    .into();

    let eigen: EigenDec<f64> = a.dec_eigen().unwrap();

    let (values, vectors) = eigen.pair();

    let eigen_values_ref = Diagonal::new(&vec![1.0, 1.0, 1.0]);
    assert_relative_eq!(eigen_values_ref, values);

    assert_relative_eq!(
        &Into::<General<f64>>::into(a) * &vectors,
        &vectors * &Into::<General<f64>>::into(values),
        epsilon = 1.0e-5
    );
}

#[test]
fn dec_eigen_3() {
    let a: UnitUpperTriangular<f64> = matrix![  1.0, 0.0, 0.0; 
                                                3.0, 1.0, 0.0;
                                                0.0, 0.0, 1.0]
    .transpose()
    .into();

    let eigen: EigenDec<f64> = a.dec_eigen().unwrap();

    let (values, vectors) = eigen.pair();

    let eigen_values_ref = Diagonal::new(&vec![1.0, 1.0, 1.0]);
    assert_relative_eq!(eigen_values_ref, values);

    assert_relative_eq!(
        &Into::<General<f64>>::into(a) * &vectors,
        &vectors * &Into::<General<f64>>::into(values),
        epsilon = 1.0e-5
    );
}

#[test]
fn dec_eigen_4() {
    let a: UnitUpperTriangular<f64> = matrix![  1.0, 0.0, 0.0; 
                                                0.0, 1.0, 0.0;
                                                0.0, 0.0, 1.0]
    .into();

    let eigen: EigenDec<f64> = a.dec_eigen().unwrap();

    let (values, vectors) = eigen.pair();

    let eigen_values_ref = Diagonal::new(&vec![1.0, 1.0, 1.0]);
    assert_relative_eq!(eigen_values_ref, values);

    assert_relative_eq!(
        &Into::<General<f64>>::into(a) * &vectors,
        &vectors * &Into::<General<f64>>::into(values),
        epsilon = 1.0e-5
    );
}
