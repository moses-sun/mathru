use crate::mathru::algebra::abstr::Real;

#[test]
fn infinity() {
    assert_eq!(f32::INFINITY, f32::infinity());
}

#[test]
fn net_infinity() {
    assert_eq!(f32::NEG_INFINITY, f32::neg_infinity());
}
