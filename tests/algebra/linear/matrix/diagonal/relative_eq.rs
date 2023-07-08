use mathru::algebra::linear::matrix::Diagonal;

#[test]
#[should_panic]
fn non_matching_content() {
    let a: Diagonal<f64> = Diagonal::new(&vec![1.0, 2.0]);
    let b: Diagonal<f64> = Diagonal::new(&vec![2.0, 1.0]);

    assert_relative_eq!(a, b);
}

#[test]
#[should_panic]
fn non_matching_dimensions() {
    let a: Diagonal<f64> = Diagonal::new(&vec![1.0, 2.0]);
    let b: Diagonal<f64> = Diagonal::new(&vec![-1.0]);

    assert_relative_eq!(a, b);
}
