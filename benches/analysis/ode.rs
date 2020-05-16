use criterion::Criterion;


extern crate mathru;

use mathru::analysis::differential_equation::ordinary::{Euler, Heun, Kutta3, RungeKutta4, RungeKuttaFehlberg54, DormandPrince54};
use super::ode_problems::ExplicitODE1;

criterion_group!(
	ode,
	forward_euler,
	heun,
	kutta3,
	rungekutta4,
	rungekuttafehlberg54,
	dormandprince54,
);

fn forward_euler(bench: &mut Criterion)
{
	let problem: ExplicitODE1 = ExplicitODE1::default();
	let solver: Euler<f64> = Euler::new(0.001);

    bench.bench_function("Forward Euler", move |bh| bh.iter(|| solver.solve(&problem).unwrap()));
}

fn heun(bench: &mut Criterion)
{
	let problem: ExplicitODE1 = ExplicitODE1::default();
	let solver: Heun<f64> = Heun::new(0.001);

    bench.bench_function("Heun", move |bh| bh.iter(|| solver.solve(&problem).unwrap()));
}

fn kutta3(bench: &mut Criterion)
{
	let problem: ExplicitODE1 = ExplicitODE1::default();
	let solver: Kutta3<f64> = Kutta3::new(0.001);

    bench.bench_function("Kutta3", move |bh| bh.iter(|| solver.solve(&problem).unwrap()));
}

fn rungekutta4(bench: &mut Criterion)
{
	let problem: ExplicitODE1 = ExplicitODE1::default();
	let solver: RungeKutta4<f64> = RungeKutta4::new(0.001);

    bench.bench_function("Kutta4", move |bh| bh.iter(|| solver.solve(&problem).unwrap()));
}

fn rungekuttafehlberg54(bench: &mut Criterion)
{
	let problem: ExplicitODE1 = ExplicitODE1::default();
	let h_0: f64 = 0.001;
   	let fac: f64 = 0.9;
   	let fac_min: f64 = 0.01;
   	let fac_max: f64 = 2.0;
   	let n_max: u32 = 5000;
    let abs_tol: f64 = 10e-7;
    let rel_tol: f64 = 10e-3;

	let solver: RungeKuttaFehlberg54<f64> = RungeKuttaFehlberg54::new(n_max, h_0, fac, fac_min, fac_max, abs_tol, rel_tol);

    bench.bench_function("RungeKuttaFehlberg54", move |bh| bh.iter(|| solver.solve(&problem).unwrap()));
}

fn dormandprince54(bench: &mut Criterion)
{
	let problem: ExplicitODE1 = ExplicitODE1::default();
	let h_0: f64 = 0.001;
	let abs_tol: f64 = 10e-7;
	let n_max: u32 = 5000;
	let solver: DormandPrince54<f64> = DormandPrince54::new(abs_tol, h_0, n_max);

    bench.bench_function("DormandPrince54", move |bh| bh.iter(|| solver.solve(&problem).unwrap()));
}