use mathru::algebra::linear::matrix::{Diagonal, EigenDecomposition, General};

#[test]
fn dec_eigen() {
    let a: Diagonal<f64> = Diagonal::new(&[1.0, 2.0, 3.0]);
    let eigen_values_ref = Diagonal::new(&vec![1.0, 2.0, 3.0]);

    let (values, vectors) = a.dec_eigen().unwrap().pair();

    assert_relative_eq!(eigen_values_ref, values);

    assert_relative_eq!(
        &Into::<General<f64>>::into(a) * &vectors,
        &vectors * &Into::<General<f64>>::into(values),
        epsilon = 1.0e-5
    );
}
