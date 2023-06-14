use mathru::algebra::linear::matrix::{
    General, Transpose, UnitLowerTriangular, UnitUpperTriangular,
};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

#[test]
fn transpose_randomized() {
    let r = || -> f64 { thread_rng().gen::<f64>() };
    let n = thread_rng().sample(Uniform::from(1..10));

    let mut a = General::zero(n, n);
    for col in 0..n {
        for row in col..n {
            a[[row, col]] = if row == col { 1.0 } else { r() };
        }
    }

    let actual: UnitLowerTriangular<f64> =
        Into::<UnitUpperTriangular<f64>>::into(a.clone()).transpose();

    assert_abs_diff_eq!(a.transpose(), actual.into(), epsilon = 1.0e-4);
}
