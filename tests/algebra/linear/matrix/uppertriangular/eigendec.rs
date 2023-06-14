use mathru::algebra::linear::matrix::{
    Diagonal, EigenDec, EigenDecomposition, General, UpperTriangular,
};
use mathru::matrix;

#[test]
fn dec_eigen() {
    let a: UpperTriangular<f64> = matrix![  -3.0, 3.0, 6.0;
                                            0.0, -5.0, -6.0;
                                            0.0, 0.0, 4.0]
    .into();

    let eigen: EigenDec<f64> = a.dec_eigen().unwrap();

    let eigen_values_ref = Diagonal::new(&vec![-3.0, -5.0, 4.0]);

    let zero: General<f64> = General::zero(3, 3);

    let (values, vectors) = eigen.pair();
    assert_relative_eq!(eigen_values_ref, values);
    assert_relative_ne!(zero, vectors);

    // for i in 0..3 {
    //     let v_i = vectors.get_column(i);
    //     let lambda_i = values[i];
    //     assert_abs_diff_eq!(&a * &v_i, v_i * lambda_i, epsilon = 1.0e-10);
    // }
}
