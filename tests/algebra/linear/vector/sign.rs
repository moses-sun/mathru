use crate::mathru::algebra::abstr::Sign;
use mathru::algebra::linear::vector::Vector;

#[test]
#[should_panic]
fn is_positive() {
    let a: Vector<f64> = Vector::new_column(vec![1.0, -2.0]);

    let _ = a.is_positive();
}

#[test]
#[should_panic]
fn is_negative() {
    let a: Vector<f64> = Vector::new_column(vec![1.0, -2.0]);

    let _ = a.is_negative();
}
