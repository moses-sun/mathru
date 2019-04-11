fn lu_decompose_100x100(bh: &mut criterion::Criterion) {
    let m = DMatrix::<f64>::new_random(100, 100);
    bh.bench_function("lu_decompose_100x100", move |bh| bh.iter(|| test::black_box(LU::new(m.clone()))));
}

fn lu_decompose_500x500(bh: &mut criterion::Criterion) {
    let m = DMatrix::<f64>::new_random(500, 500);
    bh.bench_function("lu_decompose_500x500", move |bh| bh.iter(|| test::black_box(LU::new(m.clone()))));
}