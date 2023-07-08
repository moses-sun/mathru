use mathru::algebra::linear::matrix::{Diagonal, General};
use rand::distributions::OpenClosed01;
use rand::{thread_rng, Rng};

fn generate_matrix() -> Diagonal<f64> {
    let r = || -> f64 { thread_rng().sample(OpenClosed01) };
    matrix![r(), 0.0, 0.0; 0.0, r(), 0.0; 0.0, 0.0, -13.0].into()
}

#[test]
#[should_panic]
fn index_out_of_bounds() {
    let a = generate_matrix();

    let _ = a[[3, 1]];
}

#[test]
#[should_panic]
fn index_write_non_diagonal() {
    let mut a = generate_matrix();

    a[[2, 1]] = 3.0;
}

#[test]
fn index_in_bounds() {
    let a = generate_matrix();

    assert_eq!(a[[2, 2]], -13.0);
}

#[test]
#[should_panic]
fn index_mut_out_of_bounds() {
    let mut a = generate_matrix();

    a[[3, 1]] = 2.0;
}

#[test]
fn index_off_diagonal() {
    let a = generate_matrix();

    assert_eq!(a[[2, 1]], 0.0);
}
