use criterion::Criterion;
use mathru::algebra::linear::matrix::General;

criterion_group!(
    bench_general_add_own_general,
    general_add_own_general,
    vec_add_own_vec
);

fn general_add_own_general(bench: &mut Criterion) {
    bench.bench_function("general add own general", move |bh| {
        bh.iter(|| {
            let a: General<f64> = General::new(100, 100, vec![3.0; 10000]);
            let b: General<f64> = General::new(100, 100, vec![3.0; 10000]);
            let _ = a + b;
        });
    });
}

fn vec_add_own_vec(bench: &mut Criterion) {
    bench.bench_function("vec add own vec", move |bh| {
        bh.iter(|| {
            let vec1: Vec<f64> = vec![3.0; 10000];
            let vec2: Vec<f64> = vec![3.0; 10000];
            let _ = vec1
                .into_iter()
                .zip(&vec2)
                .map(|(a, b)| a + b)
                .collect::<Vec<f64>>();
        });
    });
}
