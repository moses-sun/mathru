use criterion::Criterion;
use mathru::algebra::linear::matrix::{CholeskyDecomposition, Diagonal, General};

criterion_group!(bench_cholesky, cholesky_dec_1024_1024,);

fn cholesky_dec_1024_1024(bench: &mut Criterion) {
    let a: General<f64> = Diagonal::new(&[1.0; 1024]).into();

    bench.bench_function("cholesky dec 1024 1024", move |bh| {
        bh.iter(|| {
            let _ = a.dec_cholesky();
        })
    });
}
