use criterion::Criterion;
use mathru::algebra::linear::matrix::General;

criterion_group!(
    bench_general_add_borrow_general,
    general_add_borrow_general,
    vec_add_borrow_vec
);

fn general_add_borrow_general(bench: &mut Criterion) {
    bench.bench_function("general add borrow general", move |bh| {
        let vec1: General<f64> = General::new(100, 100, vec![3.0; 10000]);
        let vec2: General<f64> = General::new(100, 100, vec![3.0; 10000]);
        bh.iter(|| {
            let _: General<f64> = &vec1 + &vec2;
        });
    });
}

fn vec_add_borrow_vec(bench: &mut Criterion) {
    bench.bench_function("vec add borrow vec", move |bh| {
        let vec1: Vec<f64> = vec![3.0; 10000];
        let vec2: Vec<f64> = vec![3.0; 10000];
        bh.iter(|| {
            let _ = vec1
                .iter()
                .zip(&vec2)
                .map(|(a, b)| a + b)
                .collect::<Vec<f64>>();
        });
    });
}
