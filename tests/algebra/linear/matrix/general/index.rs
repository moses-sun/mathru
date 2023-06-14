use mathru::algebra::linear::matrix::General;
use rand::distributions::OpenClosed01;
use rand::{thread_rng, Rng};

fn generate_matrix() -> General<f64> {
    let r = || -> f64 { thread_rng().sample(OpenClosed01) };
    matrix![r(), r(), r(); r(), r(), r(); r(), -13.0 , r()]
}

#[test]
#[should_panic]
fn index_out_of_bounds() {
    let a = generate_matrix();

    let _ = a[[3, 1]];
}

#[test]
fn index_in_bounds() {
    let a = generate_matrix();

    assert_eq!(a[[2, 1]], -13.0);
}

#[test]
#[should_panic]
fn index_mut_out_of_bounds() {
    let mut a = generate_matrix();

    a[[3, 1]] = 2.0;
}

#[test]
fn index_mut_in_bounds() {
    let mut a = generate_matrix();

    a[[2, 1]] = 2.0;

    assert_eq!(a[[2, 1]], 2.0);
}
