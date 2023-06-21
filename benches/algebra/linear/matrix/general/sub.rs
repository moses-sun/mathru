use criterion::Criterion;
use mathru::algebra::linear::matrix::General;

criterion_group!(
    sub,
    bench_sub_matrix_own,
    bench_sub_matrix_borrow,
    bench_sub_matrix_mut_borrow,
    bench_sub_scalar_own,
    bench_sub_scalar_mut_borrow,
    bench_sub_scalar_borrow /*, bench_vector_sub_vector, bench_vec_sub_vec*/
);

fn bench_sub_matrix_own(bench: &mut Criterion) {
    bench.bench_function("sub matrix own", move |bh| {
        bh.iter(|| {
            let a: General<f64> = General::new(1000, 1000, vec![3.0; 1000000]);
            let b: General<f64> = General::new(1000, 1000, vec![3.0; 1000000]);
            let _ = a - b;
        });
    });
}

fn bench_sub_matrix_borrow(bench: &mut Criterion) {
    let a: General<f64> = General::new(1000, 1000, vec![3.0; 1000000]);
    let b: General<f64> = General::new(1000, 1000, vec![3.0; 1000000]);

    bench.bench_function("sub matrix borrow", move |bh| {
        bh.iter(|| {
            let _ = &a - &b;
        });
    });
}

fn bench_sub_matrix_mut_borrow(bench: &mut Criterion) {
    let mut a: General<f64> = General::new(1000, 1000, vec![3.0; 1000000]);
    let b: General<f64> = General::new(1000, 1000, vec![3.0; 1000000]);

    bench.bench_function("sub matrix mut borrow", move |bh| {
        bh.iter(|| {
            let _ = &mut a - &b;
        });
    });
}

fn bench_sub_scalar_own(bench: &mut Criterion) {
    bench.bench_function("sub scalar own", move |bh| {
        bh.iter(|| {
            let matrix: General<f64> = General::new(1000, 1000, vec![3.0; 1000000]);
            let _ = matrix - 3.0f64;
        });
    });
}

fn bench_sub_scalar_borrow(bench: &mut Criterion) {
    let matrix: General<f64> = General::new(1000, 1000, vec![3.0; 1000000]);
    bench.bench_function("sub scalar borrow", move |bh| {
        bh.iter(|| {
            let _ = &matrix - &3.0f64;
        });
    });
}

fn bench_sub_scalar_mut_borrow(bench: &mut Criterion) {
    let mut matrix: General<f64> = General::new(1000, 1000, vec![3.0; 1000000]);
    bench.bench_function("sub scalar mut borrow", move |bh| {
        bh.iter(|| {
            let _ = (&mut matrix) - &3.0f64;
        });
    });
}
