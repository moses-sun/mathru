use mathru::algebra::linear::matrix::CholeskyDecomposition;
use mathru::algebra::linear::matrix::Diagonal;

#[test]
fn dec_cholesky() {
    let d = Diagonal::new(&[1.0f64; 1024]);

    let _ = d.dec_cholesky();
}
