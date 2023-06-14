use crate::mathru::algebra::linear::matrix::Determinant;
use mathru::algebra::linear::matrix::{General, LowerTriangular};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

#[test]
fn determinant_randomized() {
    for _i in 0..100 {
        let r = || -> f64 { thread_rng().gen::<f64>() };
        let n = thread_rng().sample(Uniform::from(1..10));

        let mut a = General::zero(n, n);
        let mut expected = 1.0f64;
        for col in 0..n {
            for row in 0..=col {
                let random = r();
                if row == col {
                    expected *= random;
                }
                a[[row, col]] = random;
            }
        }

        let actual = LowerTriangular::from(a.clone()).det();

        assert_abs_diff_eq!(expected, actual, epsilon = 1.0e-8);
    }
}
