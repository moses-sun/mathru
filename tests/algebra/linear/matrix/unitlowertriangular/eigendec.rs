use mathru::algebra::linear::matrix::{
    Diagonal, EigenDec, EigenDecomposition, General, UnitLowerTriangular,
};
use mathru::matrix;

#[test]
fn dec_eigen() {
    let a: UnitLowerTriangular<f64> = matrix![  1.0, 0.0, 0.0; 
                                                3.0, 1.0, 0.0;
                                                6.0, -6.0, 1.0]
    .into();

    let eigen: EigenDec<f64> = a.dec_eigen().unwrap();

    let eigen_values_ref = Diagonal::new(&vec![1.0, 1.0, 1.0]);

    assert_relative_eq!(eigen_values_ref, eigen.values());
}
