use mathru::algebra::linear::matrix::{Diagonal, EigenDecomposition, General, UpperTriangular};
use mathru::matrix;

#[test]
fn dec_eigen() {
    let a: UpperTriangular<f64> = matrix![  -3.0, 3.0, 6.0;
                                            0.0, -5.0, -6.0;
                                            0.0, 0.0, 4.0]
    .into();

    let eigen_values_ref = Diagonal::new(&vec![-3.0, -5.0, 4.0]);

    let (values, vectors) = a.dec_eigen().unwrap().pair();

    assert_relative_eq!(eigen_values_ref, values);
    assert_relative_eq!(
        &Into::<General<f64>>::into(a) * &vectors,
        &vectors * &Into::<General<f64>>::into(values),
        epsilon = 1.0e-5
    );
}
