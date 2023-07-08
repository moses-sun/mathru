use mathru::algebra::linear::matrix::{Diagonal, General};

#[test]
fn apply_mut() {
    let mut a: Diagonal<f64> = matrix![1.0, 0.0;
                                       0.0, 2.0]
    .into();
    let reference: Diagonal<f64> = matrix![2.0, 0.0;
                            0.0, 3.0]
    .into();

    a.mut_apply(&|e| *e + 1.0);

    assert_eq!(a, reference);
}
