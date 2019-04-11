
use criterion::Criterion;
use criterion::black_box;


extern crate mathru;

use mathru::algebra::linear::Matrix;

criterion_group!(
	matrix,
//	mat100_add_mat100,
//	mat200_add_mat200,
//	mat500_add_mat500,
//	mat100_mul_mat100,
//	mat200_mul_mat200,
//	mat500_mul_mat500,
	dec_lu_100x100,
	dec_lu_200x200,
	dec_lu_500x500,
);

fn mat100_add_mat100(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new_random(100, 100);
    let b: Matrix<f64> = Matrix::new_random(100, 100);

    bench.bench_function("mat100_add_mat100", move |bh| bh.iter(|| &a + &b));
}

fn mat200_add_mat200(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new_random(200,200);
    let b: Matrix<f64> = Matrix::new_random(200, 200);

    bench.bench_function("mat200_add_mat200", move |bh| bh.iter(|| &a + &b));
}

fn mat500_add_mat500(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new_random(500, 500);
    let b: Matrix<f64> = Matrix::new_random(500, 500);

    bench.bench_function("mat500_add_mat500", move |bh| bh.iter(|| &a + &b));
}

fn mat100_mul_mat100(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new_random(100, 100);
    let b: Matrix<f64> = Matrix::new_random(100, 100);

    bench.bench_function("mat100_mul_mat100", move |bh| bh.iter(|| &a * &b));
}

fn mat200_mul_mat200(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new_random(200, 200);
    let b: Matrix<f64> = Matrix::new_random(200, 200);

    bench.bench_function("mat200_mul_mat200", move |bh| bh.iter(|| &a * &b));
}

fn mat500_mul_mat500(bench: &mut Criterion)
{
    let a: Matrix<f64> = Matrix::new_random(500, 500);
    let b: Matrix<f64> = Matrix::new_random(500, 500);

    bench.bench_function("mat500_mul_mat500", move |bh| bh.iter(|| &a * &b));
}

fn dec_lu_100x100(bench: &mut Criterion)
{
    let m : Matrix<f64> = Matrix::<f64>::new_random(100, 100);
    bench.bench_function("dec_lu_100x100", move |bench| bench.iter(|| black_box(m.dec_lu())));
}

fn dec_lu_200x200(bench: &mut Criterion)
{
    let m : Matrix<f64> = Matrix::<f64>::new_random(200, 200);
    bench.bench_function("dec_lu_200x200", move |bench| bench.iter(|| black_box(m.dec_lu())));
}

fn dec_lu_500x500(bench: &mut Criterion)
{
    let m: Matrix<f64> = Matrix::new_random(500, 500);
    bench.bench_function("dec_lu_500x500", move |bench| bench.iter(|| black_box(m.dec_lu())));
}
